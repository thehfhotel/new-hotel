'use client'

import { useEffect, useMemo, useState } from 'react'
import { AlertCircle, Coffee, Loader2, Search, X } from 'lucide-react'
import { useBranchFetch } from '@/lib/use-branch-fetch'

/**
 * POS sale modal — Track G6 (MVP).
 *
 * Lets the cashier search the F3 product catalog, pick a line, set
 * quantity (and optionally override the unit price), then POST it
 * against the open folio at `POST /api/checkins/:id/pos-sale`.
 *
 * The backend service decrements `ht_products.prod_current_stock`
 * and emits the legacy-mirror writeback. The modal lists the
 * folio's existing sales (read from
 * `GET /api/checkins/:id/pos-sales`) so the cashier can see the
 * running tab in reverse-chronological order, mirroring the iHOTEL
 * POS screen.
 *
 * Modelled on `ChangeRoomModal.tsx` (Track G4) for visual + UX
 * consistency.
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

interface SaleEntry {
  saleId: number
  productId: number
  productLegacyNo: string
  productName: string
  productUnit: string | null
  qty: number
  unitPriceBaht: number
  totalBaht: number
  note: string | null
  status: string
  source: string
  soldAt: string
  legacyId: number | null
}

interface RoomLite {
  id: number
  roomNo: string
}

interface ActiveCheckInLite {
  id: number
  cinNo: string
  customerName?: string | null
}

interface PosSaleModalProps {
  /** The room whose active folio is being charged. The modal resolves
   *  the check-in id internally via `GET /api/checkins?roomId=…`,
   *  matching the `ChangeRoomModal` pattern so receptionists always
   *  see the same surface area. */
  room: RoomLite
  onClose: () => void
  /** Called after every successful sale + after the modal closes,
   *  so the parent screen can refresh totals. */
  onSuccess?: () => void
}

function formatThb(amount: number): string {
  return amount.toLocaleString('th-TH', {
    style: 'currency',
    currency: 'THB',
    minimumFractionDigits: 2,
  })
}

function formatTime(iso: string): string {
  try {
    const d = new Date(iso)
    return d.toLocaleTimeString('th-TH', {
      hour: '2-digit',
      minute: '2-digit',
    })
  } catch {
    return iso
  }
}

export default function PosSaleModal({
  room,
  onClose,
  onSuccess,
}: PosSaleModalProps) {
  const branchFetch = useBranchFetch()
  const [products, setProducts] = useState<ProductLite[]>([])
  const [sales, setSales] = useState<SaleEntry[]>([])
  const [grandTotal, setGrandTotal] = useState<number>(0)
  const [activeCheckin, setActiveCheckin] = useState<ActiveCheckInLite | null>(
    null,
  )
  const [loading, setLoading] = useState(true)
  const [submitting, setSubmitting] = useState(false)
  const [error, setError] = useState<string | null>(null)
  const checkInId = activeCheckin?.id ?? null
  const folioLabel = activeCheckin
    ? `Room ${room.roomNo} · ${activeCheckin.cinNo}${
        activeCheckin.customerName ? ` · ${activeCheckin.customerName}` : ''
      }`
    : `Room ${room.roomNo}`

  // Form state — single line per submission. Cashiers ring up one
  // line at a time; multi-line receipts are deferred (CHANGELOG).
  const [search, setSearch] = useState<string>('')
  const [selectedProductId, setSelectedProductId] = useState<number | null>(
    null,
  )
  const [qty, setQty] = useState<string>('1')
  const [unitPriceOverride, setUnitPriceOverride] = useState<string>('')
  const [note, setNote] = useState<string>('')

  // -----------------------------------------------------------------
  // Initial load:
  //   1. Resolve the active check-in for this room (we don't accept
  //      cin_id directly — parity with ChangeRoomModal).
  //   2. Fetch the product catalog + the folio's existing sales.
  // -----------------------------------------------------------------
  useEffect(() => {
    let cancelled = false
    async function load() {
      setLoading(true)
      setError(null)
      try {
        const cinRes = await branchFetch(
          `/api/checkins?roomId=${room.id}&status=active&limit=1`,
        )
        if (cancelled) return
        if (!cinRes.ok) {
          throw new Error(`Failed to look up check-in (HTTP ${cinRes.status})`)
        }
        const cinBody = await cinRes.json()
        const cinList = (cinBody.data ?? []) as ActiveCheckInLite[]
        if (cinList.length === 0) {
          throw new Error('No active check-in for this room.')
        }
        const cin = cinList[0]
        setActiveCheckin(cin)

        const [productsRes, salesRes] = await Promise.all([
          branchFetch('/api/products?active_only=true&limit=200'),
          branchFetch(`/api/checkins/${cin.id}/pos-sales`),
        ])
        if (cancelled) return
        if (!productsRes.ok) {
          throw new Error(
            `Failed to load products (HTTP ${productsRes.status})`,
          )
        }
        if (!salesRes.ok) {
          throw new Error(`Failed to load sales (HTTP ${salesRes.status})`)
        }
        const productsBody = await productsRes.json()
        const salesBody = await salesRes.json()
        setProducts(productsBody.data ?? [])
        setSales(salesBody.data ?? [])
        setGrandTotal(salesBody.grandTotalBaht ?? 0)
      } catch (err) {
        if (cancelled) return
        setError(err instanceof Error ? err.message : 'Failed to load POS data')
      } finally {
        if (!cancelled) setLoading(false)
      }
    }
    load()
    return () => {
      cancelled = true
    }
  }, [branchFetch, room.id])

  // -----------------------------------------------------------------
  // Derived filtered product list — case-insensitive contains on
  // name or legacy_no.
  // -----------------------------------------------------------------
  const filteredProducts = useMemo(() => {
    const term = search.trim().toLowerCase()
    if (!term) return products
    return products.filter(
      (p) =>
        p.name.toLowerCase().includes(term) ||
        p.legacyNo.toLowerCase().includes(term),
    )
  }, [products, search])

  const selectedProduct = useMemo(
    () => products.find((p) => p.id === selectedProductId) ?? null,
    [products, selectedProductId],
  )

  // -----------------------------------------------------------------
  // Submit one sale line. Validation is server-side authoritative;
  // we still pre-validate the form so the cashier sees errors before
  // the round-trip.
  // -----------------------------------------------------------------
  async function handleSubmit(event: React.FormEvent) {
    event.preventDefault()
    setError(null)

    if (!checkInId) {
      setError('No active check-in resolved for this room.')
      return
    }
    if (!selectedProductId) {
      setError('Please pick a product before adding to the folio.')
      return
    }
    const qtyNum = parseFloat(qty)
    if (!Number.isFinite(qtyNum) || qtyNum <= 0) {
      setError('Quantity must be a positive number.')
      return
    }
    let overrideNum: number | null = null
    if (unitPriceOverride.trim() !== '') {
      const parsed = parseFloat(unitPriceOverride)
      if (!Number.isFinite(parsed) || parsed < 0) {
        setError('Unit price override must be a non-negative number.')
        return
      }
      overrideNum = parsed
    }

    setSubmitting(true)
    try {
      const res = await branchFetch(
        `/api/checkins/${checkInId}/pos-sale`,
        {
          method: 'POST',
          headers: { 'Content-Type': 'application/json' },
          body: JSON.stringify({
            productId: selectedProductId,
            qty: qtyNum,
            unitPriceOverride: overrideNum,
            note: note.trim() === '' ? null : note.trim(),
          }),
        },
      )
      if (!res.ok) {
        const body = await res.json().catch(() => null)
        const detail =
          body && typeof body.error === 'string'
            ? body.error
            : `HTTP ${res.status}`
        throw new Error(detail)
      }
      // Refresh the folio's sales list so the new line + new total
      // are reflected before the next sale.
      const salesRes = await branchFetch(
        `/api/checkins/${checkInId}/pos-sales`,
      )
      if (salesRes.ok) {
        const body = await salesRes.json()
        setSales(body.data ?? [])
        setGrandTotal(body.grandTotalBaht ?? 0)
      }
      // Reset form for the next line.
      setSelectedProductId(null)
      setQty('1')
      setUnitPriceOverride('')
      setNote('')
      setSearch('')
      onSuccess?.()
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Failed to record sale.')
    } finally {
      setSubmitting(false)
    }
  }

  return (
    <div className="fixed inset-0 z-50 flex items-center justify-center bg-black/40 p-4">
      <div className="w-full max-w-3xl rounded-lg bg-white shadow-xl">
        {/* Header */}
        <div className="flex items-center justify-between border-b px-6 py-4">
          <div className="flex items-center gap-3">
            <Coffee className="h-5 w-5 text-amber-600" />
            <div>
              <h2 className="text-lg font-semibold text-gray-900">
                Add charge to folio
              </h2>
              {folioLabel && (
                <p className="text-sm text-gray-500">{folioLabel}</p>
              )}
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
        <div className="grid grid-cols-1 gap-6 px-6 py-5 md:grid-cols-2">
          {/* Left: product picker + form */}
          <form onSubmit={handleSubmit} className="space-y-4">
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
              <div className="max-h-56 overflow-y-auto rounded-md border border-gray-200">
                {filteredProducts.length === 0 ? (
                  <p className="px-3 py-3 text-sm text-gray-500">
                    No matching products. Add one in Inventory → Products.
                  </p>
                ) : (
                  <ul className="divide-y divide-gray-100">
                    {filteredProducts.map((product) => {
                      const selected = product.id === selectedProductId
                      return (
                        <li key={product.id}>
                          <button
                            type="button"
                            onClick={() => setSelectedProductId(product.id)}
                            className={`flex w-full items-center justify-between px-3 py-2 text-left text-sm hover:bg-amber-50 ${
                              selected ? 'bg-amber-100' : ''
                            }`}
                          >
                            <span>
                              <span className="font-medium text-gray-900">
                                {product.name}
                              </span>
                              <span className="ml-2 text-xs text-gray-500">
                                #{product.legacyNo} · stock {product.currentStock}
                              </span>
                            </span>
                            <span className="ml-2 text-sm text-gray-700">
                              {formatThb(product.price)}
                            </span>
                          </button>
                        </li>
                      )
                    })}
                  </ul>
                )}
              </div>
            )}

            <div className="grid grid-cols-2 gap-3">
              <div>
                <label className="mb-1 block text-sm font-medium text-gray-700">
                  Quantity
                </label>
                <input
                  type="number"
                  min="0"
                  step="0.01"
                  value={qty}
                  onChange={(e) => setQty(e.target.value)}
                  className="w-full rounded-md border border-gray-300 px-3 py-2 text-sm focus:border-amber-500 focus:outline-none focus:ring-1 focus:ring-amber-500"
                />
              </div>
              <div>
                <label className="mb-1 block text-sm font-medium text-gray-700">
                  Unit price override
                  <span className="ml-1 text-xs font-normal text-gray-400">
                    (optional)
                  </span>
                </label>
                <input
                  type="number"
                  min="0"
                  step="0.01"
                  value={unitPriceOverride}
                  onChange={(e) => setUnitPriceOverride(e.target.value)}
                  placeholder={
                    selectedProduct ? selectedProduct.price.toFixed(2) : ''
                  }
                  className="w-full rounded-md border border-gray-300 px-3 py-2 text-sm focus:border-amber-500 focus:outline-none focus:ring-1 focus:ring-amber-500"
                />
              </div>
            </div>

            <div>
              <label className="mb-1 block text-sm font-medium text-gray-700">
                Note <span className="text-xs font-normal text-gray-400">(optional)</span>
              </label>
              <input
                type="text"
                value={note}
                onChange={(e) => setNote(e.target.value)}
                placeholder="minibar, laundry, ..."
                className="w-full rounded-md border border-gray-300 px-3 py-2 text-sm focus:border-amber-500 focus:outline-none focus:ring-1 focus:ring-amber-500"
              />
            </div>

            {error && (
              <div className="flex items-start gap-2 rounded-md border border-red-200 bg-red-50 px-3 py-2 text-sm text-red-700">
                <AlertCircle className="mt-0.5 h-4 w-4 flex-shrink-0" />
                <span>{error}</span>
              </div>
            )}

            <button
              type="submit"
              disabled={submitting || !selectedProductId}
              className="flex w-full items-center justify-center gap-2 rounded-md bg-amber-600 px-4 py-2 text-sm font-medium text-white hover:bg-amber-700 disabled:cursor-not-allowed disabled:bg-amber-300"
            >
              {submitting ? (
                <>
                  <Loader2 className="h-4 w-4 animate-spin" /> Recording…
                </>
              ) : (
                <>Add to folio</>
              )}
            </button>
          </form>

          {/* Right: running tab */}
          <div>
            <div className="mb-2 flex items-center justify-between">
              <h3 className="text-sm font-semibold text-gray-800">
                Running tab
              </h3>
              <span className="text-sm font-semibold text-gray-900">
                Total: {formatThb(grandTotal)}
              </span>
            </div>

            <div className="max-h-80 overflow-y-auto rounded-md border border-gray-200">
              {sales.length === 0 ? (
                <p className="px-3 py-3 text-sm text-gray-500">
                  No POS charges on this folio yet.
                </p>
              ) : (
                <ul className="divide-y divide-gray-100">
                  {sales.map((sale) => (
                    <li
                      key={sale.saleId}
                      className="flex items-center justify-between px-3 py-2 text-sm"
                    >
                      <span>
                        <span className="font-medium text-gray-900">
                          {sale.productName}
                        </span>
                        <span className="ml-2 text-xs text-gray-500">
                          {sale.qty} × {formatThb(sale.unitPriceBaht)}
                          {sale.source === 'legacy' ? ' · iHOTEL' : ''}
                        </span>
                        {sale.note && (
                          <span className="ml-2 text-xs italic text-gray-400">
                            “{sale.note}”
                          </span>
                        )}
                      </span>
                      <span className="text-sm text-gray-700">
                        {formatThb(sale.totalBaht)}
                        <span className="ml-2 text-xs text-gray-400">
                          {formatTime(sale.soldAt)}
                        </span>
                      </span>
                    </li>
                  ))}
                </ul>
              )}
            </div>
          </div>
        </div>

        {/* Footer */}
        <div className="flex items-center justify-end gap-2 border-t bg-gray-50 px-6 py-3">
          <button
            type="button"
            onClick={onClose}
            className="rounded-md border border-gray-300 bg-white px-4 py-2 text-sm font-medium text-gray-700 hover:bg-gray-100"
          >
            Close
          </button>
        </div>
      </div>
    </div>
  )
}
