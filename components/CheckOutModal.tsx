'use client'

import { useEffect, useState } from 'react'
import { X, AlertCircle, Loader2, LogOut } from 'lucide-react'
import { useBranchFetch } from '@/lib/use-branch-fetch'

/**
 * Check-out confirmation modal.
 *
 * Looks up the active check-in for `roomId` (filter:
 * `?roomId=X&status=active`), then submits PUT
 * `/api/checkins/:cinId/checkout`. The PUT triggers the `CheckOut`
 * writeback recipe which mirrors `HT_CheckIn_Ds.Cin_Room_Status='Check-Out'`,
 * `HT_Room_Status.room_status='Check Out'`, etc to legacy MSSQL.
 *
 * Payment is handled separately via the payment flow — this modal does NOT
 * record a payment.
 */

interface RoomLite {
  id: number
  roomNo: string
}

interface ActiveCheckIn {
  id: number
  cinNo: string
  customerName?: string | null
  checkinTime?: string | null
  expectedCheckout?: string | null
}

interface CheckOutModalProps {
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

export default function CheckOutModal({ room, onClose, onSuccess }: CheckOutModalProps) {
  const branchFetch = useBranchFetch()
  const [loading, setLoading] = useState(true)
  const [submitting, setSubmitting] = useState(false)
  const [error, setError] = useState<string | null>(null)
  const [activeCheckin, setActiveCheckin] = useState<ActiveCheckIn | null>(null)

  // Find the currently active check-in for this room.
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
        setActiveCheckin(list[0])
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

  const submit = async () => {
    if (!activeCheckin) return
    setSubmitting(true)
    setError(null)
    try {
      const res = await branchFetch(
        `/api/checkins/${activeCheckin.id}/checkout`,
        {
          method: 'PUT',
          headers: { 'Content-Type': 'application/json' },
          body: JSON.stringify({}),
        },
      )
      const data = await res.json()
      if (!res.ok || !data.success) {
        throw new Error(data.message || 'เช็คเอ้าท์ไม่สำเร็จ')
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
            เช็คเอ้าท์ ห้อง {room.roomNo}
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
              <div className="bg-sky-50 border border-sky-200 rounded p-3 space-y-2 text-sm">
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
                  <span>{formatThaiDate(activeCheckin.checkinTime)}</span>
                </div>
                <div className="flex justify-between">
                  <span className="text-gray-600">วันที่จะออก:</span>
                  <span>{formatThaiDate(activeCheckin.expectedCheckout)}</span>
                </div>
              </div>
              <div className="text-xs text-gray-500">
                หมายเหตุ: การชำระเงินทำผ่านเมนู Payment แยก เช็คเอ้าท์นี้จะไม่บันทึกยอดเงิน
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
            disabled={submitting || loading || !activeCheckin}
            className="px-4 py-2 text-sm font-medium text-white bg-sky-600 rounded hover:bg-sky-700 disabled:opacity-50 flex items-center"
          >
            {submitting ? (
              <Loader2 size={14} className="mr-2 animate-spin" />
            ) : (
              <LogOut size={14} className="mr-2" />
            )}
            ยืนยันเช็คเอ้าท์
          </button>
        </div>
      </div>
    </div>
  )
}
