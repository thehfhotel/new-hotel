'use client'

import { useEffect, useState } from 'react'
import { X, AlertCircle, Loader2, CalendarPlus } from 'lucide-react'
import { useBranchFetch } from '@/lib/use-branch-fetch'

/**
 * Extend-stay modal — Track G1 / T4 HIGH-2.
 *
 * Looks up the active check-in for `roomId` (filter:
 * `?roomId=X&status=active`), then submits
 * PUT `/api/checkins/:cinId/extend` with `{ newCheckoutDate, reason? }`.
 *
 * The route validates that `newCheckoutDate > existingExpectedCheckout`
 * (shortening a stay is a separate refund / partial-stay flow, Track G2)
 * and on success delegates to the existing `CheckInService::extend`. The
 * writeback recipe (`extend_stay.rs`) then mirrors the change to legacy
 * MSSQL (`HT_Book_Date` / `HT_CheckIn_Ds` / `HT_Room_Status`).
 */

interface RoomLite {
  id: number
  roomNo: string
}

interface ActiveCheckIn {
  id: number
  cinNo: string
  customerName?: string | null
  checkInTime?: string | null
  expectedCheckout?: string | null
}

interface ExtendStayModalProps {
  room: RoomLite
  onClose: () => void
  onSuccess: () => void
}

function formatThaiDate(s: string | null | undefined): string {
  if (!s) return '-'
  try {
    const d = new Date(s)
    return d.toLocaleDateString('th-TH', { day: '2-digit', month: 'short', year: 'numeric' })
  } catch {
    return s
  }
}

/** Convert an ISO date / date-time string to a `YYYY-MM-DD` date input value. */
function toDateInputValue(s: string | null | undefined): string {
  if (!s) return ''
  // `s` is either `YYYY-MM-DD` or `YYYY-MM-DDTHH:mm:ss(.sss)Z`. Slice the
  // date portion — `new Date(s)` would shift to local TZ on display, which
  // we never want for a date-only field.
  return s.slice(0, 10)
}

/** Add one day to a `YYYY-MM-DD` string. Returns `''` if input is empty. */
function addOneDay(yyyymmdd: string): string {
  if (!yyyymmdd) return ''
  const [y, m, d] = yyyymmdd.split('-').map(Number)
  if (!y || !m || !d) return ''
  // Construct UTC to avoid local-TZ shifting at month/year boundaries.
  const next = new Date(Date.UTC(y, m - 1, d + 1))
  return next.toISOString().slice(0, 10)
}

export default function ExtendStayModal({ room, onClose, onSuccess }: ExtendStayModalProps) {
  const branchFetch = useBranchFetch()
  const [loading, setLoading] = useState(true)
  const [submitting, setSubmitting] = useState(false)
  const [error, setError] = useState<string | null>(null)
  const [activeCheckin, setActiveCheckin] = useState<ActiveCheckIn | null>(null)
  const [newCheckoutDate, setNewCheckoutDate] = useState<string>('')
  const [reason, setReason] = useState<string>('')

  // Find the currently active check-in for this room (same pattern as
  // CheckOutModal — keeps the "act on an occupied room" surface uniform).
  useEffect(() => {
    let cancelled = false
    const find = async () => {
      try {
        const res = await branchFetch(
          `/api/checkins?roomId=${room.id}&status=active&limit=1`,
        )
        const data = await res.json()
        if (cancelled) return
        if (!res.ok || !data.success) {
          setError(data.message || 'ไม่สามารถค้นหาการเช็คอินได้')
          return
        }
        const list = (data.data || []) as ActiveCheckIn[]
        if (list.length === 0) {
          setError('ห้องนี้ไม่มีผู้เข้าพักอยู่')
          return
        }
        const active = list[0]
        setActiveCheckin(active)
        // Pre-fill the date picker with `existing + 1 day` so the common
        // one-more-night request is a single click.
        setNewCheckoutDate(addOneDay(toDateInputValue(active.expectedCheckout)))
      } catch (err) {
        if (!cancelled) {
          setError(err instanceof Error ? err.message : 'เกิดข้อผิดพลาด')
        }
      } finally {
        if (!cancelled) setLoading(false)
      }
    }
    find()
    return () => { cancelled = true }
  }, [room.id, branchFetch])

  const existingExpectedDate = toDateInputValue(activeCheckin?.expectedCheckout)
  const minSelectableDate = addOneDay(existingExpectedDate)
  const isDateValid =
    !!newCheckoutDate &&
    !!existingExpectedDate &&
    newCheckoutDate > existingExpectedDate

  const submit = async () => {
    if (!activeCheckin) return
    if (!isDateValid) {
      setError('กรุณาเลือกวันที่ออกใหม่ที่ช้ากว่าวันที่ออกเดิม')
      return
    }
    setSubmitting(true)
    setError(null)
    try {
      const trimmedReason = reason.trim()
      const res = await branchFetch(
        `/api/checkins/${activeCheckin.id}/extend`,
        {
          method: 'PUT',
          headers: { 'Content-Type': 'application/json' },
          body: JSON.stringify({
            newCheckoutDate,
            reason: trimmedReason ? trimmedReason : undefined,
          }),
        },
      )
      const data = await res.json()
      if (!res.ok || !data.success) {
        throw new Error(data.message || 'ขยายเวลาเข้าพักไม่สำเร็จ')
      }
      onSuccess()
      onClose()
    } catch (err) {
      setError(err instanceof Error ? err.message : 'เกิดข้อผิดพลาด')
    } finally {
      setSubmitting(false)
    }
  }

  return (
    <div
      className="fixed inset-0 bg-black/50 flex items-center justify-center z-50 p-4"
      onClick={onClose}
    >
      <div
        className="bg-white rounded-lg shadow-xl w-full max-w-md"
        onClick={(e) => e.stopPropagation()}
      >
        <div className="flex items-center justify-between p-4 border-b border-gray-200">
          <h2 className="text-lg font-bold text-gray-900">
            ขยายเวลาเข้าพัก ห้อง {room.roomNo}
          </h2>
          <button
            onClick={onClose}
            className="p-1 hover:bg-gray-100 rounded"
            aria-label="Close"
          >
            <X size={20} />
          </button>
        </div>

        <div className="p-4 space-y-4">
          {loading ? (
            <div className="flex items-center justify-center py-8 text-gray-500">
              <Loader2 size={20} className="animate-spin mr-2" />
              กำลังค้นหาการเช็คอิน...
            </div>
          ) : activeCheckin ? (
            <>
              <div className="bg-emerald-50 border border-emerald-200 rounded p-3 space-y-2 text-sm">
                <div className="flex justify-between">
                  <span className="text-gray-600">เลขที่เช็คอิน:</span>
                  <span className="font-mono font-medium">{activeCheckin.cinNo}</span>
                </div>
                <div className="flex justify-between">
                  <span className="text-gray-600">ลูกค้า:</span>
                  <span className="font-medium">{activeCheckin.customerName || '-'}</span>
                </div>
                <div className="flex justify-between">
                  <span className="text-gray-600">วันเข้าพัก:</span>
                  <span>{formatThaiDate(activeCheckin.checkInTime)}</span>
                </div>
                <div className="flex justify-between">
                  <span className="text-gray-600">วันที่จะออกเดิม:</span>
                  <span>{formatThaiDate(activeCheckin.expectedCheckout)}</span>
                </div>
              </div>

              <div className="space-y-2">
                <label
                  htmlFor="extend-new-date"
                  className="block text-sm font-medium text-gray-700"
                >
                  วันที่ออกใหม่
                </label>
                <input
                  id="extend-new-date"
                  type="date"
                  value={newCheckoutDate}
                  min={minSelectableDate || undefined}
                  onChange={(e) => setNewCheckoutDate(e.target.value)}
                  className="w-full px-3 py-2 border border-gray-300 rounded text-sm focus:outline-none focus:ring-2 focus:ring-emerald-500"
                />
                {newCheckoutDate && existingExpectedDate && !isDateValid && (
                  <p className="text-xs text-red-600">
                    วันที่ออกใหม่ต้องช้ากว่า {formatThaiDate(activeCheckin.expectedCheckout)}
                  </p>
                )}
              </div>

              <div className="space-y-2">
                <label
                  htmlFor="extend-reason"
                  className="block text-sm font-medium text-gray-700"
                >
                  เหตุผล (ไม่จำเป็น)
                </label>
                <textarea
                  id="extend-reason"
                  value={reason}
                  onChange={(e) => setReason(e.target.value)}
                  rows={2}
                  placeholder="เช่น ลูกค้าขอพักเพิ่มอีก 1 คืน"
                  className="w-full px-3 py-2 border border-gray-300 rounded text-sm focus:outline-none focus:ring-2 focus:ring-emerald-500"
                />
              </div>

              <div className="text-xs text-gray-500">
                หมายเหตุ: การชำระเงินส่วนต่างทำผ่านเมนู Payment แยก ขั้นตอนนี้จะส่งคำสั่งปรับวันที่ออกเท่านั้น
              </div>
            </>
          ) : null}

          {error && (
            <div className="flex items-start p-3 bg-red-50 border border-red-200 rounded text-sm text-red-700">
              <AlertCircle size={16} className="mr-2 shrink-0 mt-0.5" />
              {error}
            </div>
          )}
        </div>

        <div className="flex justify-end gap-2 p-4 border-t border-gray-200">
          <button
            onClick={onClose}
            disabled={submitting}
            className="px-4 py-2 text-sm font-medium text-gray-700 bg-white border border-gray-300 rounded hover:bg-gray-50 disabled:opacity-50"
          >
            ยกเลิก
          </button>
          <button
            onClick={submit}
            disabled={submitting || loading || !activeCheckin || !isDateValid}
            className="px-4 py-2 text-sm font-medium text-white bg-emerald-600 rounded hover:bg-emerald-700 disabled:opacity-50 flex items-center"
          >
            {submitting ? (
              <Loader2 size={14} className="mr-2 animate-spin" />
            ) : (
              <CalendarPlus size={14} className="mr-2" />
            )}
            ยืนยันขยายเวลา
          </button>
        </div>
      </div>
    </div>
  )
}
