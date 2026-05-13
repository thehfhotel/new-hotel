'use client'

import { useState, useEffect, useRef } from 'react'
import { X, Loader2, DollarSign, FileText, Undo2 } from 'lucide-react'
import { useBranchFetch } from '@/lib/use-branch-fetch'

/**
 * Refund modal — Track G2 / T4 CRIT-1.
 *
 * Mirrors `PaymentModal`'s shape but the receptionist enters a refund
 * (positive magnitude — the backend negates) against an existing
 * payment row. POSTs `/api/new/payments/:id/refund` and refreshes the
 * caller via `onSuccess`.
 *
 * Validation client-side mirrors the service layer:
 *   - amount > 0
 *   - amount ≤ remaining refundable balance (original - already_refunded)
 * The backend re-validates against the canonical row so the modal's
 * checks are advisory only.
 */
interface RefundPaymentModalProps {
  isOpen: boolean
  onClose: () => void
  paymentId: number
  /** Original payment amount in baht. Drives the "default to full refund" preset. */
  originalAmount: number
  /** Sum of magnitudes already refunded against this payment (baht). */
  alreadyRefunded: number
  /** Tender method label for the original payment — informational only. */
  originalMethodLabel?: string
  onSuccess: () => void
}

function formatCurrency(amount: number): string {
  return new Intl.NumberFormat('th-TH', {
    style: 'currency',
    currency: 'THB',
    minimumFractionDigits: 2,
  }).format(amount)
}

export default function RefundPaymentModal({
  isOpen,
  onClose,
  paymentId,
  originalAmount,
  alreadyRefunded,
  originalMethodLabel,
  onSuccess,
}: RefundPaymentModalProps) {
  const branchFetch = useBranchFetch()

  const remainingRefundable = Math.max(originalAmount - alreadyRefunded, 0)

  const [amount, setAmount] = useState<string>('')
  const [reason, setReason] = useState('')
  const [isSubmitting, setIsSubmitting] = useState(false)
  const [error, setError] = useState<string | null>(null)

  const prevIsOpenRef = useRef(false)

  useEffect(() => {
    if (isOpen && !prevIsOpenRef.current) {
      // Default to a full refund of whatever is still refundable. The
      // receptionist can dial it back for a partial refund.
      setAmount(remainingRefundable > 0 ? remainingRefundable.toFixed(2) : '')
      setReason('')
      setError(null)
    }
    prevIsOpenRef.current = isOpen
  }, [isOpen, remainingRefundable])

  const handleClose = () => {
    setError(null)
    onClose()
  }

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault()

    const numAmount = parseFloat(amount)
    if (isNaN(numAmount) || numAmount <= 0) {
      setError('กรุณาระบุจำนวนเงินที่จะคืน')
      return
    }
    // 0.005 baht slack mirrors the service layer's tolerance.
    if (numAmount - 0.005 > remainingRefundable) {
      setError(
        `จำนวนเงินที่คืนต้องไม่เกินยอดคงเหลือที่คืนได้ (${formatCurrency(remainingRefundable)})`,
      )
      return
    }

    setIsSubmitting(true)
    setError(null)

    try {
      const res = await branchFetch(`/api/new/payments/${paymentId}/refund`, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({
          amount: numAmount,
          reason: reason || undefined,
        }),
      })

      const data = await res.json()

      if (!res.ok || !data.success) {
        throw new Error(data.error || data.message || 'เกิดข้อผิดพลาดในการคืนเงิน')
      }

      onSuccess()
      handleClose()
    } catch (err) {
      setError(err instanceof Error ? err.message : 'เกิดข้อผิดพลาดในการคืนเงิน')
    } finally {
      setIsSubmitting(false)
    }
  }

  if (!isOpen) return null

  return (
    <div
      className="fixed inset-0 bg-black/30 flex items-center justify-center z-50"
      onClick={handleClose}
    >
      <div
        className="bg-white border border-gray-200 rounded-xl shadow-2xl w-full max-w-md mx-4 max-h-[90vh] overflow-y-auto"
        onClick={(e) => e.stopPropagation()}
      >
        {/* Header */}
        <div className="flex items-center justify-between p-4 border-b border-gray-200 bg-amber-500/10">
          <div className="flex items-center gap-2">
            <Undo2 size={20} className="text-amber-600" />
            <div>
              <h2 className="text-xl font-bold text-gray-900">คืนเงิน</h2>
              <p className="text-sm text-gray-500">Refund Payment #{paymentId}</p>
            </div>
          </div>
          <button
            onClick={handleClose}
            className="p-2 hover:bg-gray-100 rounded-lg"
            aria-label="ปิด"
          >
            <X size={20} />
          </button>
        </div>

        {/* Summary */}
        <div className="p-4 bg-gray-100 border-b border-gray-200">
          <div className="grid grid-cols-3 gap-4 text-center">
            <div>
              <p className="text-sm text-gray-500">ยอดเดิม</p>
              <p className="font-semibold text-gray-800">
                {formatCurrency(originalAmount)}
              </p>
              {originalMethodLabel ? (
                <p className="text-xs text-gray-500 mt-0.5">{originalMethodLabel}</p>
              ) : null}
            </div>
            <div>
              <p className="text-sm text-gray-500">คืนแล้ว</p>
              <p className="font-semibold text-amber-600">
                {formatCurrency(alreadyRefunded)}
              </p>
            </div>
            <div>
              <p className="text-sm text-gray-500">คืนได้อีก</p>
              <p
                className={`font-semibold ${
                  remainingRefundable > 0 ? 'text-emerald-600' : 'text-gray-500'
                }`}
              >
                {formatCurrency(remainingRefundable)}
              </p>
            </div>
          </div>
        </div>

        {/* Form */}
        <form onSubmit={handleSubmit} className="p-4 space-y-4">
          {error ? (
            <div className="p-3 bg-red-50 border border-red-200 rounded-lg text-red-600 text-sm">
              {error}
            </div>
          ) : null}

          <div>
            <label className="block text-sm font-medium text-gray-700 mb-1">
              <DollarSign size={16} className="inline mr-1" />
              จำนวนเงินที่คืน (บาท) <span className="text-red-500">*</span>
            </label>
            <input
              type="number"
              value={amount}
              onChange={(e) => setAmount(e.target.value)}
              placeholder="0.00"
              min="0"
              step="0.01"
              required
              disabled={remainingRefundable <= 0}
              aria-describedby="refund-max-hint"
              className="w-full px-3 py-2 bg-gray-100 border border-gray-300 text-gray-800 rounded-lg focus:ring-2 focus:ring-amber-500 focus:border-amber-500 text-lg font-medium disabled:opacity-50"
            />
            {/*
              `max` is enforced client-side by our submit handler (with the
              same 0.005 baht slack the service uses) so the friendly
              Thai-language error message renders instead of being swallowed
              by HTML5 validity reporting.
            */}
            <span id="refund-max-hint" className="sr-only">
              สูงสุด {formatCurrency(remainingRefundable)}
            </span>
            {remainingRefundable > 0 ? (
              <button
                type="button"
                onClick={() => setAmount(remainingRefundable.toFixed(2))}
                className="mt-1 text-sm text-amber-600 hover:text-amber-700"
              >
                คืนเต็มจำนวน ({formatCurrency(remainingRefundable)})
              </button>
            ) : (
              <p className="mt-1 text-sm text-gray-500">
                ไม่มียอดคงเหลือให้คืน
              </p>
            )}
          </div>

          <div>
            <label className="block text-sm font-medium text-gray-700 mb-1">
              <FileText size={16} className="inline mr-1" />
              เหตุผล
            </label>
            <textarea
              value={reason}
              onChange={(e) => setReason(e.target.value)}
              placeholder="เช่น ลูกค้าร้องเรียน, ปรับราคาห้องผิด..."
              rows={2}
              maxLength={500}
              className="w-full px-3 py-2 bg-gray-100 border border-gray-300 text-gray-800 rounded-lg focus:ring-2 focus:ring-amber-500 focus:border-amber-500 resize-none"
            />
          </div>

          <div className="flex gap-3 pt-2">
            <button
              type="button"
              onClick={handleClose}
              className="flex-1 px-4 py-2 border border-gray-300 text-gray-700 rounded-lg hover:bg-gray-100"
              disabled={isSubmitting}
            >
              ยกเลิก
            </button>
            <button
              type="submit"
              disabled={isSubmitting || remainingRefundable <= 0}
              className="flex-1 px-4 py-2 bg-amber-600 text-white rounded-lg hover:bg-amber-700 disabled:opacity-50 disabled:cursor-not-allowed flex items-center justify-center gap-2"
            >
              {isSubmitting ? (
                <>
                  <Loader2 className="animate-spin" size={18} />
                  กำลังคืนเงิน...
                </>
              ) : (
                'ยืนยันการคืนเงิน'
              )}
            </button>
          </div>
        </form>
      </div>
    </div>
  )
}
