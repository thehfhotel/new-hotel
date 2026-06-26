'use client'

import { useState, useEffect, useRef } from 'react'
import { X, Loader2, Ticket, Calendar, User, Printer } from 'lucide-react'
import { useBranchFetch } from '@/lib/use-branch-fetch'
import { useAuth } from '@/contexts/AuthContext'

/**
 * Track G7 permission key required to open the issue modal. Centralised
 * so the backend route gate (`require_permission("coupon.issue")`) and
 * the UI gate stay in sync — change one, change both.
 */
const ISSUE_PERMISSION = 'coupon.issue'

/**
 * Issue Coupon modal — Track G5.
 *
 * Mirrors iHOTEL's `FrmCuponMain` flow: the receptionist issues a
 * food/breakfast coupon entitlement (or a stand-alone promo voucher)
 * to a guest. POSTs `/api/coupons` and refreshes the caller via
 * `onSuccess`.
 *
 * Validation client-side mirrors the service layer:
 *   - value ≥ 0 (zero is fine — legacy `HT_Cupon` rows are
 *     entitlement-only with no monetary value)
 *
 * The backend re-validates against the canonical row so the modal's
 * checks are advisory only.
 */
interface IssueCouponModalProps {
  isOpen: boolean
  onClose: () => void
  /** Optional canonical customer id the coupon is bound to. */
  customerId?: number
  /** Optional legacy `Cin_no` the coupon attaches to (folio context). */
  forCinNo?: string
  /** Defaults `0` so legacy-style entitlement coupons land without UI fuss. */
  defaultValue?: number
  onSuccess: (issuedCode: string) => void
}

function formatCurrency(amount: number): string {
  return new Intl.NumberFormat('th-TH', {
    style: 'currency',
    currency: 'THB',
    minimumFractionDigits: 2,
  }).format(amount)
}

export default function IssueCouponModal({
  isOpen,
  onClose,
  customerId,
  forCinNo,
  defaultValue = 0,
  onSuccess,
}: IssueCouponModalProps) {
  const branchFetch = useBranchFetch()
  const { hasPermission } = useAuth()

  // Track G7 — UI gate. The backend `require_permission` middleware is
  // the authoritative check (it rejects with 403 regardless of what the
  // UI does); hiding the modal here just avoids the misleading
  // "Issue coupon" button glimpse for users who lack the permission.
  const canIssue = hasPermission(ISSUE_PERMISSION)

  const [value, setValue] = useState<string>('')
  const [expiresAt, setExpiresAt] = useState<string>('')
  const [issuedBy, setIssuedBy] = useState<string>('')
  const [issuedCode, setIssuedCode] = useState<string | null>(null)
  const [isSubmitting, setIsSubmitting] = useState(false)
  const [error, setError] = useState<string | null>(null)

  const prevIsOpenRef = useRef(false)

  useEffect(() => {
    if (isOpen && !prevIsOpenRef.current) {
      // Defaults on re-open: face value from props (typically 0 for
      // food coupons), no expiry, empty operator (server falls back
      // to `Admin` mirroring the legacy `cupon_by` default).
      setValue(defaultValue.toFixed(2))
      setExpiresAt('')
      setIssuedBy('')
      setIssuedCode(null)
      setError(null)
    }
    prevIsOpenRef.current = isOpen
  }, [isOpen, defaultValue])

  const handleClose = () => {
    setError(null)
    onClose()
  }

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault()

    const numValue = parseFloat(value)
    if (isNaN(numValue) || numValue < 0) {
      setError('กรุณาระบุมูลค่าคูปอง (ต้องไม่น้อยกว่า 0)')
      return
    }

    setIsSubmitting(true)
    setError(null)

    try {
      const res = await branchFetch(`/api/coupons`, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({
          customerId: customerId ?? undefined,
          value: numValue,
          expiresAt: expiresAt || undefined,
          issuedBy: issuedBy || undefined,
          forCinNo: forCinNo ?? undefined,
        }),
      })

      const data = await res.json()

      if (!res.ok || !data.success) {
        throw new Error(data.error || data.message || 'เกิดข้อผิดพลาดในการออกคูปอง')
      }

      const code = data.code ?? ''
      setIssuedCode(code)
      onSuccess(code)
    } catch (err) {
      setError(err instanceof Error ? err.message : 'เกิดข้อผิดพลาดในการออกคูปอง')
    } finally {
      setIsSubmitting(false)
    }
  }

  const handlePrint = () => {
    if (typeof window !== 'undefined') {
      window.print()
    }
  }

  if (!isOpen) return null
  // Hide the modal entirely for users without the permission. The
  // host component should also hide the "Issue coupon" launcher
  // button — this is the defensive secondary gate.
  if (!canIssue) return null

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
        <div className="flex items-center justify-between p-4 border-b border-gray-200 bg-emerald-500/10">
          <div className="flex items-center gap-2">
            <Ticket size={20} className="text-emerald-600" />
            <div>
              <h2 className="text-xl font-bold text-gray-900">ออกคูปอง</h2>
              <p className="text-sm text-gray-500">Issue Coupon</p>
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

        {/* Success state — show the issued coupon code + print button */}
        {issuedCode ? (
          <div className="p-6 space-y-4">
            <div className="text-center space-y-2">
              <p className="text-sm text-gray-500">ออกคูปองเรียบร้อย</p>
              <p className="text-3xl font-mono font-bold text-emerald-700 tracking-wider">
                {issuedCode}
              </p>
              {parseFloat(value) > 0 ? (
                <p className="text-lg text-gray-700">
                  มูลค่า {formatCurrency(parseFloat(value))}
                </p>
              ) : null}
              {expiresAt ? (
                <p className="text-sm text-gray-500">หมดอายุ {expiresAt}</p>
              ) : null}
            </div>
            <div className="flex gap-3 pt-2">
              <button
                type="button"
                onClick={handleClose}
                className="flex-1 px-4 py-2 border border-gray-300 text-gray-700 rounded-lg hover:bg-gray-100"
              >
                เสร็จสิ้น
              </button>
              <button
                type="button"
                onClick={handlePrint}
                className="flex-1 px-4 py-2 bg-emerald-600 text-white rounded-lg hover:bg-emerald-700 flex items-center justify-center gap-2"
              >
                <Printer size={18} />
                พิมพ์คูปอง
              </button>
            </div>
          </div>
        ) : (
          <form onSubmit={handleSubmit} className="p-4 space-y-4">
            {error ? (
              <div className="p-3 bg-red-50 border border-red-200 rounded-lg text-red-600 text-sm">
                {error}
              </div>
            ) : null}

            {forCinNo ? (
              <div className="p-3 bg-gray-50 border border-gray-200 rounded-lg text-sm">
                <p className="text-gray-500">ผูกกับการเข้าพัก</p>
                <p className="font-mono font-medium text-gray-800">{forCinNo}</p>
              </div>
            ) : null}

            <div>
              <label className="block text-sm font-medium text-gray-700 mb-1">
                <Ticket size={16} className="inline mr-1" />
                มูลค่า (บาท)
              </label>
              <input
                type="number"
                value={value}
                onChange={(e) => setValue(e.target.value)}
                placeholder="0.00"
                min="0"
                step="0.01"
                className="w-full px-3 py-2 bg-gray-100 border border-gray-300 text-gray-800 rounded-lg focus:ring-2 focus:ring-emerald-500 focus:border-emerald-500 text-lg font-medium"
              />
              <p className="mt-1 text-xs text-gray-500">
                ใส่ 0 สำหรับคูปองอาหาร / Set 0 for food/breakfast coupons
              </p>
            </div>

            <div>
              <label className="block text-sm font-medium text-gray-700 mb-1">
                <Calendar size={16} className="inline mr-1" />
                วันหมดอายุ (ถ้ามี)
              </label>
              <input
                type="date"
                value={expiresAt}
                onChange={(e) => setExpiresAt(e.target.value)}
                className="w-full px-3 py-2 bg-gray-100 border border-gray-300 text-gray-800 rounded-lg focus:ring-2 focus:ring-emerald-500 focus:border-emerald-500"
              />
            </div>

            <div>
              <label className="block text-sm font-medium text-gray-700 mb-1">
                <User size={16} className="inline mr-1" />
                ผู้ออก
              </label>
              <input
                type="text"
                value={issuedBy}
                onChange={(e) => setIssuedBy(e.target.value)}
                placeholder="ชื่อพนักงาน (ค่าเริ่มต้น: Admin)"
                maxLength={50}
                className="w-full px-3 py-2 bg-gray-100 border border-gray-300 text-gray-800 rounded-lg focus:ring-2 focus:ring-emerald-500 focus:border-emerald-500"
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
                disabled={isSubmitting}
                className="flex-1 px-4 py-2 bg-emerald-600 text-white rounded-lg hover:bg-emerald-700 disabled:opacity-50 disabled:cursor-not-allowed flex items-center justify-center gap-2"
              >
                {isSubmitting ? (
                  <>
                    <Loader2 className="animate-spin" size={18} />
                    กำลังออกคูปอง...
                  </>
                ) : (
                  'ยืนยันการออกคูปอง'
                )}
              </button>
            </div>
          </form>
        )}
      </div>
    </div>
  )
}
