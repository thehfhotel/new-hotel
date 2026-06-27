'use client'

import { useEffect, useRef, useState } from 'react'
import {
  AlertCircle,
  ArrowRightLeft,
  CheckCircle2,
  Loader2,
  Printer,
  X,
} from 'lucide-react'
import { useBranchFetch } from '@/lib/use-branch-fetch'
import { printRoomChange } from '@/lib/print'
import RoomChangeSlip from '@/components/documents/RoomChangeSlip'

/**
 * Change-room modal — Track G4 / T4 HIGH-3
 * (`docs/coexistence/audit-2026-05-13.md`).
 *
 * Looks up the active check-in for `roomId` (the current room the
 * guest is in), lists the currently-available rooms the receptionist
 * can move the guest TO, then submits
 * `POST /api/checkins/:cinId/change-room` with
 * `{ fromRoomId, toRoomId, reason? }`.
 *
 * The route delegates to `CheckInService::change_room` which validates
 * the move, recomputes the rate against the destination room (Task #54
 * item 2), INSERTs the audit row in `ht_room_changes`, UPDATEs the
 * `ht_checkin_rooms` junction, and emits the legacy-mirror writeback.
 *
 * On success the modal switches to a confirmation step offering a printable
 * room-change slip (Task #54 item 5,
 * `GET /api/checkins/:id/room-change-receipt`). Modelled on
 * `ExtendStayModal.tsx` for visual consistency.
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

interface AvailableRoom {
  id: number
  roomNo: string
  typeName?: string | null
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

/** Local record of the just-completed move (for the confirmation view). */
interface ChangeDone {
  rcId: number
  toRoomNo: string
}

interface ChangeRoomModalProps {
  /** The room the guest is currently in (the "from" room). */
  room: RoomLite
  onClose: () => void
  onSuccess: () => void
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

export default function ChangeRoomModal({
  room,
  onClose,
  onSuccess,
}: ChangeRoomModalProps) {
  const branchFetch = useBranchFetch()
  const [loading, setLoading] = useState(true)
  const [submitting, setSubmitting] = useState(false)
  const [error, setError] = useState<string | null>(null)
  const [activeCheckin, setActiveCheckin] = useState<ActiveCheckIn | null>(
    null,
  )
  const [availableRooms, setAvailableRooms] = useState<AvailableRoom[]>([])
  const [selectedToRoomId, setSelectedToRoomId] = useState<number | null>(
    null,
  )
  const [reason, setReason] = useState<string>('')
  // After a successful move: the confirmation/print step.
  const [done, setDone] = useState<ChangeDone | null>(null)
  const [printing, setPrinting] = useState(false)
  const [receipt, setReceipt] = useState<RoomChangeReceipt | null>(null)
  const slipRef = useRef<HTMLDivElement>(null)

  // Find the currently active check-in for this room + the list of
  // rooms the receptionist can move TO. We run both in parallel so
  // the modal opens snappily.
  useEffect(() => {
    let cancelled = false
    const load = async () => {
      try {
        const [checkinRes, roomsRes] = await Promise.all([
          branchFetch(
            `/api/checkins?roomId=${room.id}&status=active&limit=1`,
          ),
          branchFetch('/api/rooms?status=available&limit=200'),
        ])
        const [checkinData, roomsData] = await Promise.all([
          checkinRes.json(),
          roomsRes.json(),
        ])
        if (cancelled) return

        if (!checkinRes.ok || !checkinData.success) {
          setError(checkinData.message || 'ไม่สามารถค้นหาการเช็คอินได้')
          return
        }
        const list = (checkinData.data || []) as ActiveCheckIn[]
        if (list.length === 0) {
          setError('ห้องนี้ไม่มีผู้เข้าพักอยู่')
          return
        }
        setActiveCheckin(list[0])

        if (roomsRes.ok && roomsData.success) {
          const rooms = ((roomsData.data || []) as AvailableRoom[]).filter(
            (r) => r.id !== room.id, // never let receptionist pick the same room
          )
          setAvailableRooms(rooms)
        }
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
  }, [room.id, branchFetch])

  // Print once the receipt has been fetched + the slip has rendered.
  useEffect(() => {
    if (receipt) {
      printRoomChange(slipRef.current)
    }
  }, [receipt])

  const submit = async () => {
    if (!activeCheckin) return
    if (!selectedToRoomId) {
      setError('กรุณาเลือกห้องที่จะย้ายไป')
      return
    }
    if (selectedToRoomId === room.id) {
      setError('ห้องปลายทางต้องไม่ใช่ห้องเดิม')
      return
    }
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
            fromRoomId: room.id,
            toRoomId: selectedToRoomId,
            reason: trimmedReason ? trimmedReason : undefined,
          }),
        },
      )
      const data = await res.json()
      if (!res.ok || !data.success) {
        throw new Error(data.message || 'เปลี่ยนห้องไม่สำเร็จ')
      }
      // Refresh the parent grid immediately, then switch to the
      // confirmation/print step (modal stays open until the receptionist
      // closes it or prints the slip).
      onSuccess()
      const toRoom = availableRooms.find((r) => r.id === selectedToRoomId)
      setDone({
        rcId: data.rcId,
        toRoomNo: toRoom?.roomNo ?? String(selectedToRoomId),
      })
    } catch (err) {
      setError(err instanceof Error ? err.message : 'เกิดข้อผิดพลาด')
    } finally {
      setSubmitting(false)
    }
  }

  const printSlip = async () => {
    if (!activeCheckin || !done) return
    setPrinting(true)
    setError(null)
    try {
      const res = await branchFetch(
        `/api/checkins/${activeCheckin.id}/room-change-receipt?rcId=${done.rcId}`,
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
    >
      <div
        className="bg-white rounded-lg shadow-xl w-full max-w-md"
        onClick={(e) => e.stopPropagation()}
      >
        <div className="flex items-center justify-between p-4 border-b border-gray-200">
          <h2 className="text-lg font-bold text-gray-900">
            เปลี่ยนห้องพัก ห้อง {room.roomNo}
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
          {done ? (
            <div className="bg-emerald-50 border border-emerald-200 rounded p-3 space-y-2 text-sm">
              <div className="flex items-center text-emerald-700 font-medium">
                <CheckCircle2 size={18} className="mr-2" />
                เปลี่ยนห้องสำเร็จ
              </div>
              <div className="flex justify-between">
                <span className="text-gray-600">ย้ายจากห้อง:</span>
                <span className="font-medium">{room.roomNo}</span>
              </div>
              <div className="flex justify-between">
                <span className="text-gray-600">ไปยังห้อง:</span>
                <span className="font-medium">{done.toRoomNo}</span>
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
                  <span className="font-mono font-medium">
                    {activeCheckin.cinNo}
                  </span>
                </div>
                <div className="flex justify-between">
                  <span className="text-gray-600">ลูกค้า:</span>
                  <span className="font-medium">
                    {activeCheckin.customerName || '-'}
                  </span>
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

              <div className="space-y-2">
                <label
                  htmlFor="change-room-to"
                  className="block text-sm font-medium text-gray-700"
                >
                  ห้องที่จะย้ายไป
                </label>
                <select
                  id="change-room-to"
                  value={selectedToRoomId ?? ''}
                  onChange={(e) =>
                    setSelectedToRoomId(
                      e.target.value ? Number(e.target.value) : null,
                    )
                  }
                  className="w-full px-3 py-2 border border-gray-300 rounded text-sm focus:outline-none focus:ring-2 focus:ring-amber-500"
                >
                  <option value="">-- เลือกห้องว่าง --</option>
                  {availableRooms.map((r) => (
                    <option key={r.id} value={r.id}>
                      {r.roomNo}
                      {r.typeName ? ` (${r.typeName})` : ''}
                    </option>
                  ))}
                </select>
                {availableRooms.length === 0 && (
                  <p className="text-xs text-amber-700">
                    ไม่พบห้องว่างในขณะนี้
                  </p>
                )}
              </div>

              <div className="space-y-2">
                <label
                  htmlFor="change-room-reason"
                  className="block text-sm font-medium text-gray-700"
                >
                  เหตุผล (ไม่จำเป็น)
                </label>
                <textarea
                  id="change-room-reason"
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
          {done ? (
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
                disabled={
                  submitting || loading || !activeCheckin || !selectedToRoomId
                }
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
