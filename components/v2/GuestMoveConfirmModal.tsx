'use client'

import { useEffect, useRef, useState } from 'react'
import {
  AlertCircle,
  AlertTriangle,
  ArrowRightLeft,
  CheckCircle2,
  Loader2,
  Printer,
  X,
} from 'lucide-react'
import { useBranchFetch } from '@/lib/use-branch-fetch'
import { printRoomChange } from '@/lib/print'
import RoomChangeSlip from '@/components/documents/RoomChangeSlip'
import type { SpatialRoom } from '@/lib/v2/spatial-grid'

/**
 * Guest-move drag confirm dialog (#225, ADR 0003).
 *
 * Opened when an occupied tile is dropped on an eligible target on the spatial
 * room grid. This is an ADDITIVE ALTERNATE to `ChangeRoomModal` (the
 * RoomActionSheet "เปลี่ยนห้อง" path, which stays untouched): same backend
 * contract — `POST /api/checkins/:cinId/change-room` with
 * `{ fromRoomId, toRoomId, reason? }` — and the same room-change slip print
 * pipeline (`GET /api/checkins/:id/room-change-receipt?rcId=` +
 * `printRoomChange`).
 *
 * Improvement invariant (CONTEXT.md): every value reception could edit is on
 * this one confirm screen before commit — from/to room, guest, dates, price
 * implication when the room type differs, and the free-text reason.
 */

interface ActiveCheckIn {
  id: number
  cinNo: string
  customerName?: string | null
  checkInTime?: string | null
  expectedCheckout?: string | null
}

/** Shape returned by `GET /api/checkins/:id/room-change-receipt`. */
interface RoomChangeReceipt {
  success: boolean
  rcId: number
  cinNo: string
  customerName?: string | null
  fromRoomNo: string
  toRoomNo: string
  roomBeforePrice: number
  toPrice?: string | null
  reason?: string | null
  changedAt?: string | null
  changedBy?: string | null
}

function formatThaiDate(s: string | null | undefined): string {
  if (!s) return '-'
  try {
    const d = new Date(s)
    return d.toLocaleDateString('th-TH', {
      day: '2-digit',
      month: 'short',
      year: 'numeric',
    })
  } catch {
    return s
  }
}

function formatPrice(p: number | null | undefined): string {
  if (p == null) return '-'
  return `${p.toLocaleString('th-TH')} บาท`
}

export default function GuestMoveConfirmModal({
  fromRoom,
  toRoom,
  onClose,
  onSuccess,
}: {
  /** The occupied room the tile was dragged FROM. */
  fromRoom: SpatialRoom
  /** The eligible room the tile was dropped ON. */
  toRoom: SpatialRoom
  onClose: () => void
  onSuccess: () => void
}) {
  const branchFetch = useBranchFetch()
  const [loading, setLoading] = useState(true)
  const [submitting, setSubmitting] = useState(false)
  const [error, setError] = useState<string | null>(null)
  const [activeCheckin, setActiveCheckin] = useState<ActiveCheckIn | null>(null)
  const [reason, setReason] = useState('')
  const [rcId, setRcId] = useState<number | null>(null)
  const [printing, setPrinting] = useState(false)
  const [receipt, setReceipt] = useState<RoomChangeReceipt | null>(null)
  const slipRef = useRef<HTMLDivElement>(null)

  const targetDirty = toRoom.isClean === false
  const typeDiffers =
    fromRoom.roomTypeId != null &&
    toRoom.roomTypeId != null &&
    fromRoom.roomTypeId !== toRoom.roomTypeId

  // Look up the active check-in for the source room (guest name + folio id).
  useEffect(() => {
    let cancelled = false
    const load = async () => {
      try {
        const res = await branchFetch(
          `/api/checkins?roomId=${fromRoom.id}&status=active&limit=1`,
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
    load()
    return () => {
      cancelled = true
    }
  }, [fromRoom.id, branchFetch])

  // Print once the receipt has been fetched + the slip has rendered.
  useEffect(() => {
    if (receipt) {
      printRoomChange(slipRef.current)
    }
  }, [receipt])

  const submit = async () => {
    if (!activeCheckin) return
    setSubmitting(true)
    setError(null)
    try {
      const trimmedReason = reason.trim()
      const res = await branchFetch(
        `/api/checkins/${activeCheckin.id}/change-room`,
        {
          method: 'POST',
          headers: { 'Content-Type': 'application/json' },
          body: JSON.stringify({
            fromRoomId: fromRoom.id,
            toRoomId: toRoom.id,
            reason: trimmedReason ? trimmedReason : undefined,
          }),
        },
      )
      const data = await res.json()
      if (!res.ok || !data.success) {
        throw new Error(data.message || 'เปลี่ยนห้องไม่สำเร็จ')
      }
      onSuccess()
      setRcId(data.rcId)
    } catch (err) {
      setError(err instanceof Error ? err.message : 'เกิดข้อผิดพลาด')
    } finally {
      setSubmitting(false)
    }
  }

  const printSlip = async () => {
    if (!activeCheckin || rcId == null) return
    setPrinting(true)
    setError(null)
    try {
      const res = await branchFetch(
        `/api/checkins/${activeCheckin.id}/room-change-receipt?rcId=${rcId}`,
      )
      const data = await res.json()
      if (!res.ok || !data.success) {
        throw new Error(data.message || 'ไม่สามารถพิมพ์ใบเปลี่ยนห้องได้')
      }
      setReceipt(data as RoomChangeReceipt)
    } catch (err) {
      setError(err instanceof Error ? err.message : 'เกิดข้อผิดพลาด')
    } finally {
      setPrinting(false)
    }
  }

  return (
    <div
      className="fixed inset-0 bg-black/50 flex items-center justify-center z-50 p-4"
      onClick={onClose}
      role="dialog"
      aria-modal="true"
    >
      <div
        className="bg-white rounded-lg shadow-xl w-full max-w-md"
        onClick={(e) => e.stopPropagation()}
      >
        <div className="flex items-center justify-between p-4 border-b border-gray-200">
          <h2 className="text-lg font-bold text-gray-900">
            ยืนยันการย้ายห้อง {fromRoom.roomNo} → {toRoom.roomNo}
          </h2>
          <button
            onClick={onClose}
            className="p-1 hover:bg-gray-100 rounded"
            aria-label="ปิด"
          >
            <X size={20} />
          </button>
        </div>

        <div className="p-4 space-y-4">
          {rcId != null ? (
            <div className="bg-emerald-50 border border-emerald-200 rounded p-3 space-y-2 text-sm">
              <div className="flex items-center text-emerald-700 font-medium">
                <CheckCircle2 size={18} className="mr-2" />
                เปลี่ยนห้องสำเร็จ
              </div>
              <div className="flex justify-between">
                <span className="text-gray-600">ย้ายจากห้อง:</span>
                <span className="font-medium">{fromRoom.roomNo}</span>
              </div>
              <div className="flex justify-between">
                <span className="text-gray-600">ไปยังห้อง:</span>
                <span className="font-medium">{toRoom.roomNo}</span>
              </div>
              <div className="text-xs text-gray-500">
                พิมพ์ใบแจ้งเปลี่ยนห้องให้ลูกค้า หรือกดเสร็จสิ้นเพื่อปิด
              </div>
            </div>
          ) : loading ? (
            <div className="flex items-center justify-center py-8 text-gray-500">
              <Loader2 size={20} className="animate-spin mr-2" />
              กำลังค้นหาข้อมูล...
            </div>
          ) : activeCheckin ? (
            <>
              <div className="bg-amber-50 border border-amber-200 rounded p-3 space-y-2 text-sm">
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
                  <span className="text-gray-600">วันที่จะออก:</span>
                  <span>{formatThaiDate(activeCheckin.expectedCheckout)}</span>
                </div>
                <div className="flex justify-between">
                  <span className="text-gray-600">ย้ายจากห้อง:</span>
                  <span className="font-medium">
                    {fromRoom.roomNo}
                    {fromRoom.roomTypeName ? ` (${fromRoom.roomTypeName})` : ''}
                  </span>
                </div>
                <div className="flex justify-between">
                  <span className="text-gray-600">ไปยังห้อง:</span>
                  <span className="font-medium">
                    {toRoom.roomNo}
                    {toRoom.roomTypeName ? ` (${toRoom.roomTypeName})` : ''}
                  </span>
                </div>
              </div>

              {typeDiffers && (
                <div className="bg-blue-50 border border-blue-200 rounded p-3 text-sm text-blue-800 space-y-1">
                  <div className="font-medium">ประเภทห้องไม่เหมือนเดิม — ราคาจะเปลี่ยน</div>
                  <div className="flex justify-between">
                    <span>ราคาห้องเดิม:</span>
                    <span>{formatPrice(fromRoom.priceWeekday)}</span>
                  </div>
                  <div className="flex justify-between">
                    <span>ราคาห้องใหม่:</span>
                    <span className="font-medium">{formatPrice(toRoom.priceWeekday)}</span>
                  </div>
                  <div className="text-xs text-blue-700">
                    ราคาห้องจะถูกคำนวณใหม่ตามห้องปลายทาง
                  </div>
                </div>
              )}

              {targetDirty && (
                <div className="flex items-start p-3 bg-amber-50 border border-amber-300 rounded text-sm text-amber-800">
                  <AlertTriangle size={16} className="mr-2 shrink-0 mt-0.5" />
                  ห้อง {toRoom.roomNo} ยังรอทำความสะอาด — ตรวจสอบก่อนย้ายผู้เข้าพัก
                </div>
              )}

              <div className="space-y-2">
                <label
                  htmlFor="guest-move-reason"
                  className="block text-sm font-medium text-gray-700"
                >
                  เหตุผล (ไม่จำเป็น)
                </label>
                <textarea
                  id="guest-move-reason"
                  value={reason}
                  onChange={(e) => setReason(e.target.value)}
                  rows={2}
                  placeholder="เช่น ลูกค้าบ่นเรื่องเสียงดัง / ห้องเสียต้องซ่อม"
                  className="w-full px-3 py-2 border border-gray-300 rounded text-sm focus:outline-none focus:ring-2 focus:ring-amber-500"
                />
              </div>

              <div className="text-xs text-gray-500">
                หมายเหตุ:
                ข้อมูลการเปลี่ยนห้องจะถูกบันทึกในประวัติการเข้าพักของลูกค้า
                ราคาห้องจะถูกคำนวณใหม่ตามห้องปลายทาง
                และจะส่งไปยังระบบเก่าโดยอัตโนมัติ
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
          {rcId != null ? (
            <>
              <button
                onClick={printSlip}
                disabled={printing}
                className="px-4 py-2 text-sm font-medium text-gray-700 bg-white border border-gray-300 rounded hover:bg-gray-50 disabled:opacity-50 flex items-center"
              >
                {printing ? (
                  <Loader2 size={14} className="mr-2 animate-spin" />
                ) : (
                  <Printer size={14} className="mr-2" />
                )}
                พิมพ์ใบเปลี่ยนห้อง
              </button>
              <button
                onClick={onClose}
                className="px-4 py-2 text-sm font-medium text-white bg-amber-600 rounded hover:bg-amber-700"
              >
                เสร็จสิ้น
              </button>
            </>
          ) : (
            <>
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
                className="px-4 py-2 text-sm font-medium text-white bg-amber-600 rounded hover:bg-amber-700 disabled:opacity-50 flex items-center"
              >
                {submitting ? (
                  <Loader2 size={14} className="mr-2 animate-spin" />
                ) : (
                  <ArrowRightLeft size={14} className="mr-2" />
                )}
                ยืนยันเปลี่ยนห้อง
              </button>
            </>
          )}
        </div>
      </div>

      {/* Off-screen printable slip — only present once the receipt loads. The
          @media print rules in RoomChangeSlip reposition + reveal it. */}
      {receipt && (
        <div style={{ position: 'absolute', left: '-9999px', top: 0 }} aria-hidden>
          <RoomChangeSlip
            ref={slipRef}
            cinNo={receipt.cinNo}
            customerName={receipt.customerName}
            fromRoomNo={receipt.fromRoomNo}
            toRoomNo={receipt.toRoomNo}
            roomBeforePrice={receipt.roomBeforePrice}
            toPrice={receipt.toPrice}
            reason={receipt.reason}
            changedAt={receipt.changedAt}
            changedBy={receipt.changedBy}
          />
        </div>
      )}
    </div>
  )
}
