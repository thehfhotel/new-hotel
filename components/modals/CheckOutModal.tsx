'use client'

import { useState, useEffect } from 'react'
import { X, Loader2, User, Calendar, FileText, CreditCard } from 'lucide-react'
import { useBranchFetch } from '@/lib/use-branch-fetch'
import { useBranch } from '@/contexts/BranchContext'

interface CheckInDetails {
  id: number
  cinNo: string
  customerId: number
  customerName: string | null
  roomId: number
  roomNo: string | null
  checkInTime: string | null
  expectedCheckout: string | null
  ratePerNight: number | null
  status: string
}

interface CheckOutModalProps {
  isOpen: boolean
  onClose: () => void
  checkIn: CheckInDetails | null
  onSuccess: () => void
}

type PaymentMethod = 'cash' | 'credit' | 'transfer'

const paymentMethods: { value: PaymentMethod; label: string }[] = [
  { value: 'cash', label: 'เงินสด / Cash' },
  { value: 'credit', label: 'บัตรเครดิต / Credit Card' },
  { value: 'transfer', label: 'โอนเงิน / Transfer' },
]

// Convert Gregorian year to Buddhist Era
function toBuddhistYear(date: Date): string {
  const year = date.getFullYear() + 543
  const month = String(date.getMonth() + 1).padStart(2, '0')
  const day = String(date.getDate()).padStart(2, '0')
  return `${day}/${month}/${year}`
}

// Format date for display (Thai format with Buddhist Era)
function formatThaiDate(dateStr: string | null): string {
  if (!dateStr) return '-'
  const date = new Date(dateStr)
  const thaiMonths = [
    'ม.ค.', 'ก.พ.', 'มี.ค.', 'เม.ย.', 'พ.ค.', 'มิ.ย.',
    'ก.ค.', 'ส.ค.', 'ก.ย.', 'ต.ค.', 'พ.ย.', 'ธ.ค.'
  ]
  const day = date.getDate()
  const month = thaiMonths[date.getMonth()]
  const year = date.getFullYear() + 543
  return `${day} ${month} ${year}`
}

// Calculate nights between two dates
function calculateNights(checkInStr: string | null, checkOutStr: string | null): number {
  if (!checkInStr) return 0
  const checkIn = new Date(checkInStr)
  const checkOut = checkOutStr ? new Date(checkOutStr) : new Date()
  const diffTime = checkOut.getTime() - checkIn.getTime()
  const diffDays = Math.ceil(diffTime / (1000 * 60 * 60 * 24))
  return Math.max(1, diffDays) // Minimum 1 night
}

// Format currency
function formatCurrency(amount: number): string {
  return new Intl.NumberFormat('th-TH', {
    style: 'currency',
    currency: 'THB',
    minimumFractionDigits: 0,
  }).format(amount)
}

export default function CheckOutModal({
  isOpen,
  onClose,
  checkIn,
  onSuccess,
}: CheckOutModalProps) {
  const branchFetch = useBranchFetch()
  const { checkoutServerTotalEnabled } = useBranch()

  const [paymentMethod, setPaymentMethod] = useState<PaymentMethod>('cash')
  const [notes, setNotes] = useState('')
  const [isSubmitting, setIsSubmitting] = useState(false)
  const [error, setError] = useState<string | null>(null)
  // Spike Phase 2 (ship-dark): backend-computed checkout total. Populated only
  // when CHECKOUT_SERVER_TOTAL_ENABLED is on; otherwise the legacy client-side
  // nights × rate is used, so live charges are unchanged.
  const [quote, setQuote] = useState<{ nights: number; ratePerNight: number; roomTotal: number } | null>(null)

  // Totals: the server quote when the flag is on + loaded, else the client calc.
  const clientNights = checkIn ? calculateNights(checkIn.checkInTime, null) : 0
  const clientRate = checkIn?.ratePerNight || 0
  const useServerTotal = checkoutServerTotalEnabled && quote != null
  const nights = useServerTotal ? quote!.nights : clientNights
  const ratePerNight = useServerTotal ? quote!.ratePerNight : clientRate
  const totalAmount = useServerTotal ? quote!.roomTotal : clientNights * clientRate

  // Reset form when modal opens
  useEffect(() => {
    if (isOpen) {
      setPaymentMethod('cash')
      setNotes('')
      setError(null)
    }
  }, [isOpen])

  // Fetch the server-authoritative quote when enabled (ship-dark Phase 2). When
  // off, the quote stays null and the client computation above is used.
  useEffect(() => {
    if (!isOpen || !checkoutServerTotalEnabled || !checkIn) {
      setQuote(null)
      return
    }
    let alive = true
    branchFetch(`/api/checkins/${checkIn.id}/checkout-quote`)
      .then((r) => (r.ok ? r.json() : null))
      .then((d) => {
        if (alive && d?.success) {
          setQuote({ nights: d.nights, ratePerNight: d.ratePerNight, roomTotal: d.roomTotal })
        }
      })
      .catch(() => {})
    return () => {
      alive = false
    }
  }, [isOpen, checkoutServerTotalEnabled, checkIn, branchFetch])

  const handleClose = () => {
    setError(null)
    onClose()
  }

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault()

    if (!checkIn) return

    setIsSubmitting(true)
    setError(null)

    try {
      const res = await branchFetch(`/api/checkins/${checkIn.id}/checkout`, {
        method: 'PUT',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({
          totalAmount: totalAmount || undefined,
          paymentStatus: paymentMethod,
          notes: notes || undefined,
        }),
      })

      const data = await res.json()

      if (!res.ok || !data.success) {
        throw new Error(data.error || data.message || 'เกิดข้อผิดพลาดในการเช็คเอาท์')
      }

      onSuccess()
      handleClose()
    } catch (err) {
      setError(err instanceof Error ? err.message : 'เกิดข้อผิดพลาดในการเช็คเอาท์')
    } finally {
      setIsSubmitting(false)
    }
  }

  if (!isOpen || !checkIn) return null

  return (
    <div
      className="fixed inset-0 bg-black/30 flex items-center justify-center z-50"
      onClick={handleClose}
    >
      <div
        className="bg-white border border-gray-200 rounded-xl shadow-2xl w-full max-w-lg mx-4 max-h-[90vh] overflow-y-auto"
        onClick={(e) => e.stopPropagation()}
      >
        {/* Header */}
        <div className="flex items-center justify-between p-4 border-b border-gray-200 bg-red-500/10">
          <div>
            <h2 className="text-xl font-bold text-gray-900">เช็คเอาท์</h2>
            <p className="text-sm text-gray-500">
              ห้อง {checkIn.roomNo} - {checkIn.cinNo}
            </p>
          </div>
          <button
            onClick={handleClose}
            className="p-2 hover:bg-gray-100 rounded-lg transition-colors"
          >
            <X size={20} />
          </button>
        </div>

        {/* Summary */}
        <div className="p-4 bg-gray-100 border-b border-gray-200 space-y-3">
          {/* Guest Name */}
          <div className="flex items-center gap-3">
            <div className="w-10 h-10 bg-red-500/10 rounded-full flex items-center justify-center">
              <User size={20} className="text-red-600" />
            </div>
            <div>
              <p className="text-sm text-gray-500">ชื่อผู้เข้าพัก</p>
              <p className="font-medium text-gray-800">
                {checkIn.customerName || 'ไม่ระบุ'}
              </p>
            </div>
          </div>

          {/* Check-in Date */}
          <div className="flex items-center gap-3">
            <div className="w-10 h-10 bg-red-500/10 rounded-full flex items-center justify-center">
              <Calendar size={20} className="text-red-600" />
            </div>
            <div>
              <p className="text-sm text-gray-500">วันที่เช็คอิน</p>
              <p className="font-medium text-gray-800">
                {formatThaiDate(checkIn.checkInTime)}
              </p>
            </div>
          </div>

          {/* Checkout Date (Today) */}
          <div className="flex items-center gap-3">
            <div className="w-10 h-10 bg-red-500/10 rounded-full flex items-center justify-center">
              <Calendar size={20} className="text-red-600" />
            </div>
            <div>
              <p className="text-sm text-gray-500">วันที่เช็คเอาท์ (วันนี้)</p>
              <p className="font-medium text-gray-800">
                {toBuddhistYear(new Date())}
              </p>
            </div>
          </div>

          {/* Summary Box */}
          <div className="bg-white rounded-lg border border-gray-300 p-4 mt-4">
            <div className="flex justify-between items-center mb-2">
              <span className="text-gray-500">จำนวนคืน</span>
              <span className="font-medium">{nights} คืน</span>
            </div>
            <div className="flex justify-between items-center mb-2">
              <span className="text-gray-500">ราคาต่อคืน</span>
              <span className="font-medium">
                {ratePerNight > 0 ? formatCurrency(ratePerNight) : 'ไม่ระบุ'}
              </span>
            </div>
            <div className="flex justify-between items-center pt-2 border-t border-gray-300">
              <span className="text-lg font-semibold text-gray-800">รวมทั้งหมด</span>
              <span className="text-lg font-bold text-red-600">
                {totalAmount > 0 ? formatCurrency(totalAmount) : 'ไม่ระบุ'}
              </span>
            </div>
          </div>
        </div>

        {/* Form */}
        <form onSubmit={handleSubmit} className="p-4 space-y-4">
          {/* Error Message */}
          {error && (
            <div className="p-3 bg-red-50 border border-red-200 rounded-lg text-red-600 text-sm">
              {error}
            </div>
          )}

          {/* Payment Method */}
          <div>
            <label className="block text-sm font-medium text-gray-700 mb-1">
              <CreditCard size={16} className="inline mr-1" />
              วิธีการชำระเงิน
            </label>
            <select
              value={paymentMethod}
              onChange={(e) => setPaymentMethod(e.target.value as PaymentMethod)}
              className="w-full px-3 py-2 bg-gray-100 border border-gray-300 text-gray-800 rounded-lg focus:ring-2 focus:ring-red-500 focus:border-red-500"
            >
              {paymentMethods.map((method) => (
                <option key={method.value} value={method.value}>
                  {method.label}
                </option>
              ))}
            </select>
          </div>

          {/* Notes */}
          <div>
            <label className="block text-sm font-medium text-gray-700 mb-1">
              <FileText size={16} className="inline mr-1" />
              หมายเหตุ
            </label>
            <textarea
              value={notes}
              onChange={(e) => setNotes(e.target.value)}
              placeholder="หมายเหตุเพิ่มเติม..."
              rows={3}
              className="w-full px-3 py-2 bg-gray-100 border border-gray-300 text-gray-800 rounded-lg focus:ring-2 focus:ring-red-500 focus:border-red-500 resize-none"
            />
          </div>

          {/* Submit Buttons */}
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
              disabled={isSubmitting}
              className="flex-1 px-4 py-2 bg-red-600 text-white rounded-lg hover:bg-red-700 disabled:opacity-50 disabled:cursor-not-allowed flex items-center justify-center gap-2"
            >
              {isSubmitting ? (
                <>
                  <Loader2 className="animate-spin" size={18} />
                  กำลังบันทึก...
                </>
              ) : (
                'ยืนยันเช็คเอาท์'
              )}
            </button>
          </div>
        </form>
      </div>
    </div>
  )
}
