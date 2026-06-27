'use client'

import { useEffect, useMemo, useState } from 'react'
import { AlertCircle, Loader2, Plus, Printer, Search, ShoppingBag, Trash2, X } from 'lucide-react'
import { useBranchFetch } from '@/lib/use-branch-fetch'
import { useBranch } from '@/contexts/BranchContext'
import { hotelInfoForBranch } from '@/lib/hotel-info'
import StandaloneReceiptTemplate from '@/components/documents/StandaloneReceiptTemplate'
import type { StandaloneReceiptData } from '@/types/invoice'

/**
 * Task #45 — walk-up (roomless) POS modal.
 *
 * Unlike `PosSaleModal` (which requires a `room` prop + resolves the active
 * folio), this modal rings up a sale with NO check-in: the cashier builds a
 * cart of product lines, optionally captures customer / tax details, and
 * POSTs to `/api/new/pos/walkup-sale`. On success it renders a printable
 * standalone receipt.
 */

interface ProductLite {
  id: number
  legacyNo: string
  name: string
  unit: string | null
  price: number
  currentStock: number
  active: boolean
}

interface CartLine {
  productId: number
  legacyNo: string
  name: string
  unit: string | null
  qty: number
  unitPrice: number
  discount: number
}

interface WalkupPosModalProps {
  onClose: () => void
  /** Called after a successful sale so the parent can refresh stock. */
  onSuccess?: () => void
}

const PAYMENT_METHODS = [
  { value: 'cash', label: 'เงินสด / Cash' },
  { value: 'credit', label: 'บัตรเครดิต / Credit' },
  { value: 'transfer', label: 'โอน / Transfer' },
  { value: 'qr', label: 'QR / PromptPay' },
]

function formatThb(amount: number): string {
  return amount.toLocaleString('th-TH', {
    style: 'currency',
    currency: 'THB',
    minimumFractionDigits: 2,
  })
}

export default function WalkupPosModal({ onClose, onSuccess }: WalkupPosModalProps) {
  const branchFetch = useBranchFetch()
  const { branch } = useBranch()
  const hotelInfo = hotelInfoForBranch(branch)

  const [products, setProducts] = useState<ProductLite[]>([])
  const [loading, setLoading] = useState(true)
  const [submitting, setSubmitting] = useState(false)
  const [error, setError] = useState<string | null>(null)

  const [search, setSearch] = useState('')
  const [cart, setCart] = useState<CartLine[]>([])
  const [customerName, setCustomerName] = useState('')
  const [customerTaxId, setCustomerTaxId] = useState('')
  const [customerTel, setCustomerTel] = useState('')
  const [paymentMethod, setPaymentMethod] = useState('cash')
  const [vatPercent, setVatPercent] = useState('0')

  const [receipt, setReceipt] = useState<StandaloneReceiptData | null>(null)

  useEffect(() => {
    let cancelled = false
    async function load() {
      setLoading(true)
      setError(null)
      try {
        const res = await branchFetch('/api/products?active_only=true&limit=200')
        if (cancelled) return
        if (!res.ok) {
          throw new Error(`Failed to load products (HTTP ${res.status})`)
        }
        const body = await res.json()
        setProducts(body.data ?? [])
      } catch (err) {
        if (cancelled) return
        setError(err instanceof Error ? err.message : 'Failed to load products')
      } finally {
        if (!cancelled) setLoading(false)
      }
    }
    load()
    return () => {
      cancelled = true
    }
  }, [branchFetch])

  const filteredProducts = useMemo(() => {
    const term = search.trim().toLowerCase()
    if (!term) return products
    return products.filter(
      (p) =>
        p.name.toLowerCase().includes(term) ||
        p.legacyNo.toLowerCase().includes(term),
    )
  }, [products, search])

  function addToCart(product: ProductLite) {
    setCart((prev) => {
      const existing = prev.find((l) => l.productId === product.id)
      if (existing) {
        return prev.map((l) =>
          l.productId === product.id ? { ...l, qty: l.qty + 1 } : l,
        )
      }
      return [
        ...prev,
        {
          productId: product.id,
          legacyNo: product.legacyNo,
          name: product.name,
          unit: product.unit,
          qty: 1,
          unitPrice: product.price,
          discount: 0,
        },
      ]
    })
  }

  function updateLine(productId: number, patch: Partial<CartLine>) {
    setCart((prev) =>
      prev.map((l) => (l.productId === productId ? { ...l, ...patch } : l)),
    )
  }

  function removeLine(productId: number) {
    setCart((prev) => prev.filter((l) => l.productId !== productId))
  }

  const subtotal = useMemo(
    () => cart.reduce((sum, l) => sum + l.qty * l.unitPrice, 0),
    [cart],
  )
  const discountTotal = useMemo(
    () => cart.reduce((sum, l) => sum + (l.discount || 0), 0),
    [cart],
  )
  const grandTotal = Math.max(subtotal - discountTotal, 0)

  async function handleSubmit() {
    setError(null)
    if (cart.length === 0) {
      setError('Add at least one product to the cart.')
      return
    }
    for (const line of cart) {
      if (!Number.isFinite(line.qty) || line.qty <= 0) {
        setError(`Quantity for "${line.name}" must be positive.`)
        return
      }
    }

    setSubmitting(true)
    try {
      const res = await branchFetch('/api/new/pos/walkup-sale', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({
          lines: cart.map((l) => ({
            productId: l.productId,
            qty: l.qty,
            unitPriceOverride: l.unitPrice,
            discount: l.discount || 0,
          })),
          customerName: customerName.trim() || undefined,
          customerTel: customerTel.trim() || undefined,
          taxId: customerTaxId.trim() || undefined,
          vatPercent: parseInt(vatPercent, 10) || 0,
          paymentMethod,
        }),
      })
      if (!res.ok) {
        const body = await res.json().catch(() => null)
        const detail =
          body && typeof body.error === 'string' ? body.error : `HTTP ${res.status}`
        throw new Error(detail)
      }
      const body = await res.json()
      // Build the printable receipt from the server response (the
      // authoritative totals + VAT split).
      setReceipt({
        receiptId: body.receiptId,
        receiptNumber: undefined,
        customerName: customerName.trim() || undefined,
        customerTel: customerTel.trim() || undefined,
        customerTaxId: customerTaxId.trim() || undefined,
        lines: (body.lines ?? []).map(
          (l: {
            productLegacyNo: string
            productName: string
            unitName: string
            qty: number
            unitPriceBaht: number
            discountBaht: number
            totalBaht: number
          }) => ({
            productNo: l.productLegacyNo,
            name: l.productName,
            unit: l.unitName,
            qty: l.qty,
            unitPrice: l.unitPriceBaht,
            discount: l.discountBaht,
            total: l.totalBaht,
          }),
        ),
        subtotal: body.subtotalBaht ?? subtotal,
        discount: body.discountBaht ?? discountTotal,
        beforeVat: body.beforeVatBaht,
        vatAmount: body.vatBaht ?? 0,
        vatPercent: body.vatPercent ?? 0,
        grandTotal: body.totalBaht ?? grandTotal,
        paymentMethod: body.paymentMethod ?? paymentMethod,
        paidAmount: body.paidBaht ?? body.totalBaht ?? grandTotal,
        createdAt: body.soldAt ?? new Date().toISOString(),
      })
      onSuccess?.()
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Failed to record sale.')
    } finally {
      setSubmitting(false)
    }
  }

  // --- Receipt view (post-sale) ---------------------------------------
  if (receipt) {
    return (
      <div className="fixed inset-0 z-50 flex items-center justify-center bg-black/40 p-4">
        <div className="flex max-h-[90vh] w-full max-w-3xl flex-col rounded-lg bg-white shadow-xl">
          <div className="no-print flex items-center justify-between border-b px-6 py-4">
            <h2 className="text-lg font-semibold text-gray-900">Sale complete</h2>
            <div className="flex items-center gap-2">
              <button
                type="button"
                onClick={() => window.print()}
                className="flex items-center gap-2 rounded-md bg-blue-600 px-4 py-2 text-sm font-medium text-white hover:bg-blue-700"
              >
                <Printer className="h-4 w-4" /> Print
              </button>
              <button
                type="button"
                onClick={onClose}
                className="rounded-md border border-gray-300 bg-white px-4 py-2 text-sm font-medium text-gray-700 hover:bg-gray-100"
              >
                Done
              </button>
            </div>
          </div>
          <div className="overflow-y-auto">
            <StandaloneReceiptTemplate receiptData={receipt} hotelInfo={hotelInfo} />
          </div>
        </div>
      </div>
    )
  }

  // --- Sale entry view -------------------------------------------------
  return (
    <div className="fixed inset-0 z-50 flex items-center justify-center bg-black/40 p-4">
      <div className="flex max-h-[90vh] w-full max-w-4xl flex-col rounded-lg bg-white shadow-xl">
        {/* Header */}
        <div className="flex items-center justify-between border-b px-6 py-4">
          <div className="flex items-center gap-3">
            <ShoppingBag className="h-5 w-5 text-amber-600" />
            <div>
              <h2 className="text-lg font-semibold text-gray-900">Walk-up sale</h2>
              <p className="text-sm text-gray-500">ขายสินค้า (ไม่ผูกห้อง) — standalone receipt</p>
            </div>
          </div>
          <button
            type="button"
            onClick={onClose}
            className="rounded-full p-1 text-gray-400 hover:bg-gray-100 hover:text-gray-700"
            aria-label="Close"
          >
            <X className="h-5 w-5" />
          </button>
        </div>

        {/* Body */}
        <div className="grid grid-cols-1 gap-6 overflow-y-auto px-6 py-5 md:grid-cols-2">
          {/* Left: product picker */}
          <div className="space-y-4">
            <div>
              <label className="mb-1 block text-sm font-medium text-gray-700">
                Search products
              </label>
              <div className="relative">
                <Search className="absolute left-3 top-2.5 h-4 w-4 text-gray-400" />
                <input
                  type="text"
                  value={search}
                  onChange={(e) => setSearch(e.target.value)}
                  placeholder="Coca-Cola, P001, …"
                  className="w-full rounded-md border border-gray-300 py-2 pl-9 pr-3 text-sm focus:border-amber-500 focus:outline-none focus:ring-1 focus:ring-amber-500"
                />
              </div>
            </div>

            {loading ? (
              <div className="flex items-center justify-center py-6 text-gray-400">
                <Loader2 className="h-5 w-5 animate-spin" />
              </div>
            ) : (
              <div className="max-h-72 overflow-y-auto rounded-md border border-gray-200">
                {filteredProducts.length === 0 ? (
                  <p className="px-3 py-3 text-sm text-gray-500">
                    No matching products. Add one in Inventory → Products.
                  </p>
                ) : (
                  <ul className="divide-y divide-gray-100">
                    {filteredProducts.map((product) => (
                      <li key={product.id}>
                        <button
                          type="button"
                          onClick={() => addToCart(product)}
                          className="flex w-full items-center justify-between px-3 py-2 text-left text-sm hover:bg-amber-50"
                        >
                          <span>
                            <span className="font-medium text-gray-900">{product.name}</span>
                            <span className="ml-2 text-xs text-gray-500">
                              #{product.legacyNo} · stock {product.currentStock}
                            </span>
                          </span>
                          <span className="ml-2 flex items-center gap-2 text-sm text-gray-700">
                            {formatThb(product.price)}
                            <Plus className="h-4 w-4 text-amber-600" />
                          </span>
                        </button>
                      </li>
                    ))}
                  </ul>
                )}
              </div>
            )}

            {/* Customer (optional) */}
            <div className="space-y-3 rounded-md border border-gray-200 p-3">
              <p className="text-xs font-semibold uppercase text-gray-500">
                Customer (optional)
              </p>
              <input
                type="text"
                value={customerName}
                onChange={(e) => setCustomerName(e.target.value)}
                placeholder="Customer name"
                className="w-full rounded-md border border-gray-300 px-3 py-2 text-sm focus:border-amber-500 focus:outline-none focus:ring-1 focus:ring-amber-500"
              />
              <div className="grid grid-cols-2 gap-2">
                <input
                  type="text"
                  value={customerTaxId}
                  onChange={(e) => setCustomerTaxId(e.target.value)}
                  placeholder="Tax ID"
                  className="w-full rounded-md border border-gray-300 px-3 py-2 text-sm focus:border-amber-500 focus:outline-none focus:ring-1 focus:ring-amber-500"
                />
                <input
                  type="text"
                  value={customerTel}
                  onChange={(e) => setCustomerTel(e.target.value)}
                  placeholder="Phone"
                  className="w-full rounded-md border border-gray-300 px-3 py-2 text-sm focus:border-amber-500 focus:outline-none focus:ring-1 focus:ring-amber-500"
                />
              </div>
            </div>
          </div>

          {/* Right: cart */}
          <div className="flex flex-col">
            <h3 className="mb-2 text-sm font-semibold text-gray-800">Cart</h3>
            <div className="flex-1 overflow-y-auto rounded-md border border-gray-200">
              {cart.length === 0 ? (
                <p className="px-3 py-3 text-sm text-gray-500">
                  Tap a product to add it to the cart.
                </p>
              ) : (
                <ul className="divide-y divide-gray-100">
                  {cart.map((line) => (
                    <li key={line.productId} className="px-3 py-2 text-sm">
                      <div className="flex items-center justify-between">
                        <span className="font-medium text-gray-900">{line.name}</span>
                        <button
                          type="button"
                          onClick={() => removeLine(line.productId)}
                          className="text-gray-400 hover:text-red-600"
                          aria-label={`Remove ${line.name}`}
                        >
                          <Trash2 className="h-4 w-4" />
                        </button>
                      </div>
                      <div className="mt-1 grid grid-cols-3 gap-2">
                        <label className="text-xs text-gray-500">
                          Qty
                          <input
                            type="number"
                            min="0"
                            step="0.01"
                            value={line.qty}
                            onChange={(e) =>
                              updateLine(line.productId, {
                                qty: parseFloat(e.target.value) || 0,
                              })
                            }
                            className="mt-0.5 w-full rounded border border-gray-300 px-2 py-1 text-sm"
                          />
                        </label>
                        <label className="text-xs text-gray-500">
                          Price
                          <input
                            type="number"
                            min="0"
                            step="0.01"
                            value={line.unitPrice}
                            onChange={(e) =>
                              updateLine(line.productId, {
                                unitPrice: parseFloat(e.target.value) || 0,
                              })
                            }
                            className="mt-0.5 w-full rounded border border-gray-300 px-2 py-1 text-sm"
                          />
                        </label>
                        <label className="text-xs text-gray-500">
                          Disc.
                          <input
                            type="number"
                            min="0"
                            step="0.01"
                            value={line.discount}
                            onChange={(e) =>
                              updateLine(line.productId, {
                                discount: parseFloat(e.target.value) || 0,
                              })
                            }
                            className="mt-0.5 w-full rounded border border-gray-300 px-2 py-1 text-sm"
                          />
                        </label>
                      </div>
                      <div className="mt-1 text-right text-xs text-gray-600">
                        {formatThb(Math.max(line.qty * line.unitPrice - line.discount, 0))}
                      </div>
                    </li>
                  ))}
                </ul>
              )}
            </div>

            {/* Totals + payment */}
            <div className="mt-3 space-y-2 text-sm">
              <div className="flex justify-between text-gray-600">
                <span>Subtotal</span>
                <span>{formatThb(subtotal)}</span>
              </div>
              {discountTotal > 0 && (
                <div className="flex justify-between text-green-600">
                  <span>Discount</span>
                  <span>-{formatThb(discountTotal)}</span>
                </div>
              )}
              <div className="flex justify-between text-base font-semibold text-gray-900">
                <span>Total</span>
                <span>{formatThb(grandTotal)}</span>
              </div>
              <div className="grid grid-cols-2 gap-2">
                <label className="text-xs text-gray-500">
                  Payment
                  <select
                    value={paymentMethod}
                    onChange={(e) => setPaymentMethod(e.target.value)}
                    className="mt-0.5 w-full rounded border border-gray-300 px-2 py-1.5 text-sm"
                  >
                    {PAYMENT_METHODS.map((m) => (
                      <option key={m.value} value={m.value}>
                        {m.label}
                      </option>
                    ))}
                  </select>
                </label>
                <label className="text-xs text-gray-500">
                  VAT %
                  <select
                    value={vatPercent}
                    onChange={(e) => setVatPercent(e.target.value)}
                    className="mt-0.5 w-full rounded border border-gray-300 px-2 py-1.5 text-sm"
                  >
                    <option value="0">0%</option>
                    <option value="7">7%</option>
                  </select>
                </label>
              </div>
            </div>
          </div>
        </div>

        {/* Error + footer */}
        {error && (
          <div className="mx-6 mb-2 flex items-start gap-2 rounded-md border border-red-200 bg-red-50 px-3 py-2 text-sm text-red-700">
            <AlertCircle className="mt-0.5 h-4 w-4 flex-shrink-0" />
            <span>{error}</span>
          </div>
        )}
        <div className="flex items-center justify-end gap-2 border-t bg-gray-50 px-6 py-3">
          <button
            type="button"
            onClick={onClose}
            className="rounded-md border border-gray-300 bg-white px-4 py-2 text-sm font-medium text-gray-700 hover:bg-gray-100"
          >
            Cancel
          </button>
          <button
            type="button"
            onClick={handleSubmit}
            disabled={submitting || cart.length === 0}
            className="flex items-center justify-center gap-2 rounded-md bg-amber-600 px-4 py-2 text-sm font-medium text-white hover:bg-amber-700 disabled:cursor-not-allowed disabled:bg-amber-300"
          >
            {submitting ? (
              <>
                <Loader2 className="h-4 w-4 animate-spin" /> Recording…
              </>
            ) : (
              <>Complete sale</>
            )}
          </button>
        </div>
      </div>
    </div>
  )
}
