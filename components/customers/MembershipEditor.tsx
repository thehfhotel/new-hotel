'use client'

import { useEffect, useState } from 'react'
import { BadgeCheck, Loader2, Save, X } from 'lucide-react'

interface MembershipEditorProps {
  customerId: number
  /** Current link, from the customer record (`membershipId`). */
  membershipId: string | null | undefined
  /**
   * Persist the link. Called with the trimmed id, or `null` to clear.
   * The parent wires this to `PUT /api/customers/{id}/membership`
   * (branch-aware). Saving is INDEPENDENT of the main customer save —
   * membership is a dedicated endpoint so a stale edit form can never
   * clobber a freshly-scanned link.
   */
  onSave: (customerId: number, membershipId: string | null) => Promise<void>
}

/**
 * Desk affordance for the loyalty membership link (migration 086): staff
 * type or scan the membership id from the guest's member QR at check-in.
 * Renders inside the customer edit form (edit mode only) with its own
 * save/clear actions.
 */
export default function MembershipEditor({
  customerId,
  membershipId,
  onSave,
}: MembershipEditorProps) {
  const [value, setValue] = useState(membershipId ?? '')
  const [saving, setSaving] = useState(false)
  const [error, setError] = useState<string | null>(null)
  const [savedFlash, setSavedFlash] = useState(false)

  // Re-sync when the modal is reused for another customer.
  useEffect(() => {
    setValue(membershipId ?? '')
    setError(null)
    setSavedFlash(false)
  }, [customerId, membershipId])

  const persist = async (next: string | null) => {
    setSaving(true)
    setError(null)
    setSavedFlash(false)
    try {
      await onSave(customerId, next)
      setValue(next ?? '')
      setSavedFlash(true)
    } catch (err) {
      setError(
        err instanceof Error ? err.message : 'ไม่สามารถบันทึกรหัสสมาชิกได้'
      )
    } finally {
      setSaving(false)
    }
  }

  const trimmed = value.trim()
  const hasExisting = Boolean(membershipId)

  return (
    <div className="border border-gray-200 rounded-lg p-3 bg-gray-50">
      <label className="flex items-center gap-2 text-sm font-medium text-gray-700 mb-1">
        <BadgeCheck className="w-4 h-4" />
        รหัสสมาชิก Loyalty
      </label>
      <div className="flex items-center gap-2">
        <input
          type="text"
          name="membershipId"
          value={value}
          onChange={(e) => setValue(e.target.value)}
          placeholder="พิมพ์หรือสแกนจาก QR สมาชิก"
          maxLength={64}
          className="flex-1 px-3 py-2 bg-gray-100 border border-gray-300 text-gray-800 rounded-lg focus:ring-2 focus:ring-red-500 focus:border-red-500 outline-hidden transition-colors font-mono"
        />
        <button
          type="button"
          onClick={() => persist(trimmed === '' ? null : trimmed)}
          disabled={saving || (!hasExisting && trimmed === '')}
          className="flex items-center gap-1 px-3 py-2 bg-red-600 text-white text-sm rounded-lg hover:bg-red-700 disabled:opacity-50 transition-colors"
        >
          {saving ? (
            <Loader2 className="w-4 h-4 animate-spin" />
          ) : (
            <Save className="w-4 h-4" />
          )}
          บันทึก
        </button>
        {hasExisting && (
          <button
            type="button"
            onClick={() => persist(null)}
            disabled={saving}
            aria-label="ยกเลิกการเชื่อมสมาชิก"
            className="flex items-center gap-1 px-3 py-2 text-red-600 text-sm hover:bg-red-500/10 rounded-lg disabled:opacity-50 transition-colors"
          >
            <X className="w-4 h-4" />
            ล้าง
          </button>
        )}
      </div>
      {error && <p className="mt-1 text-sm text-red-600">{error}</p>}
      {savedFlash && !error && (
        <p className="mt-1 text-sm text-green-600">บันทึกแล้ว</p>
      )}
      <p className="mt-1 text-xs text-gray-500">
        บันทึกแยกจากปุ่มบันทึกหลัก — มีผลทันที ใช้เชื่อมแขกกับแอปสะสมคะแนน
      </p>
    </div>
  )
}
