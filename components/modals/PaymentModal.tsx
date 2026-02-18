'use client'

import { useState, useEffect } from 'react'
import { X, Loader2, CreditCard, FileText, DollarSign, Banknote, QrCode } from 'lucide-react'

interface PaymentModalProps {
  isOpen: boolean
  onClose: () => void
  checkinId: number
  totalAmount: number
  totalPaid: number
  onSuccess: () => void
}

type PaymentMethod = 'cash' | 'credit' | 'transfer' | 'qr'

const paymentMethods: { value: PaymentMethod; label: string; icon: React.ReactNode }[] = [
  { value: 'cash', label: 'เงินสด / Cash', icon: <Banknote size={18} /> },
  { value: 'credit', label: 'บัตรเครดิต / Credit Card', icon: <CreditCard size={18} /> },
  { value: 'transfer', label: 'โอนเงิน / Transfer', icon: <DollarSign size={18} /> },
  { value: 'qr', label: 'QR Code', icon: <QrCode size={18} /> },
]

// Format currency
function formatCurrency(amount: number): string {
  return new Intl.NumberFormat('th-TH', {
    style: 'currency',
    currency: 'THB',
    minimumFractionDigits: 0,
  }).format(amount)
}

export default function PaymentModal({
  isOpen,
  onClose,
  checkinId,
  totalAmount,
  totalPaid,
  onSuccess,
}: PaymentModalProps) {
  const balance = totalAmount - totalPaid

  const [amount, setAmount] = useState<string>('')
  const [paymentMethod, setPaymentMethod] = useState<PaymentMethod>('cash')
  const [reference, setReference] = useState('')
  const [notes, setNotes] = useState('')
  const [isSubmitting, setIsSubmitting] = useState(false)
  const [error, setError] = useState<string | null>(null)

  // Reset form and pre-fill amount when modal opens
  useEffect(() => {
    if (isOpen) {
      setAmount(balance > 0 ? balance.toString() : '')
      setPaymentMethod('cash')
      setReference('')
      setNotes('')
      setError(null)
    }
  }, [isOpen, balance])

  const handleClose = () => {
    setError(null)
    onClose()
  }

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault()

    const numAmount = parseFloat(amount)
    if (isNaN(numAmount) || numAmount <= 0) {
      setError('กรุณาระบุจำนวนเงินที่ถูกต้อง')
      return
    }

    setIsSubmitting(true)
    setError(null)

    try {
      const res = await fetch(`/api/new/checkins/${checkinId}/payments`, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({
          amount: numAmount,
          method: paymentMethod,
          reference: reference || undefined,
          notes: notes || undefined,
        }),
      })

      const data = await res.json()

      if (!res.ok || !data.success) {
        throw new Error(data.error || data.message || 'เกิดข้อผิดพลาดในการบันทึกการชำระเงิน')
      }

      onSuccess()
      handleClose()
    } catch (err) {
      setError(err instanceof Error ? err.message : 'เกิดข้อผิดพลาดในการบันทึกการชำระเงิน')
    } finally {
      setIsSubmitting(false)
    }
  }

  if (!isOpen) return null

  const showReferenceField = paymentMethod === 'credit' || paymentMethod === 'transfer'

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
        <div className="flex items-center justify-between p-4 border-b border-gray-200 bg-red-500/10">
          <div>
            <h2 className="text-xl font-bold text-gray-900">บันทึกการชำระเงิน</h2>
            <p className="text-sm text-gray-500">Record Payment</p>
          </div>
          <button
            onClick={handleClose}
            className="p-2 hover:bg-gray-100 rounded-lg"
          >
            <X size={20} />
          </button>
        </div>

        {/* Summary */}
        <div className="p-4 bg-gray-100 border-b border-gray-200">
          <div className="grid grid-cols-3 gap-4 text-center">
            <div>
              <p className="text-sm text-gray-500">ยอดรวม</p>
              <p className="font-semibold text-gray-800">{formatCurrency(totalAmount)}</p>
            </div>
            <div>
              <p className="text-sm text-gray-500">ชำระแล้ว</p>
              <p className="font-semibold text-red-600">{formatCurrency(totalPaid)}</p>
            </div>
            <div>
              <p className="text-sm text-gray-500">คงเหลือ</p>
              <p className={`font-semibold ${balance > 0 ? 'text-amber-400' : 'text-red-600'}`}>
                {formatCurrency(balance)}
              </p>
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

          {/* Amount */}
          <div>
            <label className="block text-sm font-medium text-gray-700 mb-1">
              <DollarSign size={16} className="inline mr-1" />
              จำนวนเงิน (บาท) <span className="text-red-500">*</span>
            </label>
            <input
              type="number"
              value={amount}
              onChange={(e) => setAmount(e.target.value)}
              placeholder="0"
              min="0"
              step="0.01"
              required
              className="w-full px-3 py-2 bg-gray-100 border border-gray-300 text-gray-800 rounded-lg focus:ring-2 focus:ring-red-500 focus:border-red-500 text-lg font-medium"
            />
            {balance > 0 && (
              <button
                type="button"
                onClick={() => setAmount(balance.toString())}
                className="mt-1 text-sm text-red-600 hover:text-red-300"
              >
                ใช้ยอดคงเหลือ ({formatCurrency(balance)})
              </button>
            )}
          </div>

          {/* Payment Method */}
          <div>
            <label className="block text-sm font-medium text-gray-700 mb-2">
              <CreditCard size={16} className="inline mr-1" />
              วิธีการชำระเงิน
            </label>
            <div className="grid grid-cols-2 gap-2">
              {paymentMethods.map((method) => (
                <button
                  key={method.value}
                  type="button"
                  onClick={() => setPaymentMethod(method.value)}
                  className={`flex items-center gap-2 px-3 py-2 border rounded-lg text-sm ${
                    paymentMethod === method.value
                      ? 'border-red-500 bg-red-500/10 text-red-600'
                      : 'border-gray-300 hover:bg-gray-100'
                  }`}
                >
                  {method.icon}
                  <span>{method.label.split(' / ')[0]}</span>
                </button>
              ))}
            </div>
          </div>

          {/* Reference (for card/transfer) */}
          {showReferenceField && (
            <div>
              <label className="block text-sm font-medium text-gray-700 mb-1">
                {paymentMethod === 'credit' ? 'เลขบัตร 4 หลักสุดท้าย' : 'เลขอ้างอิง'}
              </label>
              <input
                type="text"
                value={reference}
                onChange={(e) => setReference(e.target.value)}
                placeholder={paymentMethod === 'credit' ? '1234' : 'เลขอ้างอิงการโอน'}
                maxLength={100}
                className="w-full px-3 py-2 bg-gray-100 border border-gray-300 text-gray-800 rounded-lg focus:ring-2 focus:ring-red-500 focus:border-red-500"
              />
            </div>
          )}

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
              rows={2}
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
                'บันทึกการชำระเงิน'
              )}
            </button>
          </div>
        </form>
      </div>
    </div>
  )
}
