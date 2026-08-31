'use client'

import { useEffect, useState } from 'react'
import { X, AlertCircle, Loader2, LogOut, CreditCard } from 'lucide-react'
import { useBranchFetch } from '@/lib/use-branch-fetch'
import RoomCheckPanel from '@/components/v2/signals/RoomCheckPanel'

/**
 * Check-out & settle modal (M1 task #37).
 *
 * Looks up the active check-in for `roomId` (filter:
 * `?roomId=X&status=active`), shows the server-authoritative folio
 * (`GET /api/checkins/:id/checkout-quote` — room / products / paid / balance),
 * then on confirm:
 *   1. captures any outstanding balance as a payment via the payment flow
 *      (`POST /api/checkins/:id/payments`, the same path PaymentModal uses), and
 *   2. completes the check-out (`PUT /api/checkins/:id/checkout`).
 *
 * The checkout PUT body stays `{}` on purpose: `CHECKOUT_SERVER_TOTAL_ENABLED`
 * ships dark and we must not change the command/writeback total when it is off.
 * Money is captured through the payment row above, not the checkout total. The
 * folio is DISPLAYED regardless of the flag (read-only quote endpoint).
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

// Server-authoritative folio from /api/checkins/:id/checkout-quote.
interface Folio {
  nights: number
  ratePerNight: number
  roomTotal: number
  productTotal: number
  vatPercent: number
  vat: number
  deposit: number
  netTotal: number
  payTotal: number
  balance: number
}

// One room of the stay, from GET /api/checkins/:id/rooms (#75 per-room checkout).
interface RoomLine {
  crId: number
  roomId: number
  roomNo: string
  status: 'active' | 'checked_out'
  nights: number
  ratePerNight: number
  roomSubtotal: number
  deposit: number
}

interface CheckOutModalProps {
  room: RoomLite
  onClose: () => void
  onSuccess: () => void
  /**
   * Opt-in: append the ขอเช็คห้อง room-signal panel (ADR 0008) below the
   * existing modal body.
   *
   * OPT-IN, not always-on, and deliberately so. This one modal is mounted by
   * three surfaces — the v2 rooms board, the v1 dashboard, and the v1 rooms
   * page — and only the v2 desk surface is in the room-signals build. Without
   * the flag the component is not even mounted, so the v1 screens issue no
   * signal read, open no event stream, and render no new control.
   */
  roomCheck?: boolean
}

type PaymentMethod = 'cash' | 'credit' | 'transfer' | 'qr'

const paymentMethods: { value: PaymentMethod; label: string }[] = [
  { value: 'cash', label: 'เงินสด / Cash' },
  { value: 'credit', label: 'บัตรเครดิต / Credit Card' },
  { value: 'transfer', label: 'โอนเงิน / Transfer' },
  { value: 'qr', label: 'QR Code' },
]

function formatThaiDate(s: string | null | undefined): string {
  if (!s) return '-'
  try {
    const d = new Date(s)
    return d.toLocaleDateString('th-TH', { day: '2-digit', month: 'short', year: 'numeric' })
  } catch {
    return s
  }
}

function formatCurrency(amount: number): string {
  return new Intl.NumberFormat('th-TH', {
    style: 'currency',
    currency: 'THB',
    minimumFractionDigits: 0,
  }).format(amount)
}

export default function CheckOutModal({
  room,
  onClose,
  onSuccess,
  roomCheck = false,
}: CheckOutModalProps) {
  const branchFetch = useBranchFetch()
  const [loading, setLoading] = useState(true)
  const [submitting, setSubmitting] = useState(false)
  const [error, setError] = useState<string | null>(null)
  const [activeCheckin, setActiveCheckin] = useState<ActiveCheckIn | null>(null)
  const [folio, setFolio] = useState<Folio | null>(null)
  const [paymentMethod, setPaymentMethod] = useState<PaymentMethod>('cash')
  // Guards against a double-charge if the payment POST succeeds but the
  // checkout PUT fails and the receptionist retries: we never re-record the
  // balance once it has been captured in this modal session.
  const [paymentCaptured, setPaymentCaptured] = useState(false)
  // #75 per-room checkout: all rooms of the stay + the cr_ids selected to
  // release. Seeded with the clicked room once the room list loads.
  const [rooms, setRooms] = useState<RoomLine[]>([])
  const [selectedCrIds, setSelectedCrIds] = useState<Set<number>>(new Set())

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

  // Once the active check-in is known, pull the server folio for display +
  // the outstanding-balance amount we settle on checkout. Read-only quote, so
  // it is fetched regardless of CHECKOUT_SERVER_TOTAL_ENABLED.
  useEffect(() => {
    if (!activeCheckin) return
    let cancelled = false
    branchFetch(`/api/checkins/${activeCheckin.id}/checkout-quote`)
      .then((r) => (r.ok ? r.json() : null))
      .then((d) => {
        if (cancelled || !d?.success) return
        setFolio({
          nights: d.nights,
          ratePerNight: d.ratePerNight,
          roomTotal: d.roomTotal,
          productTotal: d.productTotal,
          vatPercent: d.vatPercent,
          vat: d.vat,
          deposit: d.deposit,
          netTotal: d.netTotal,
          payTotal: d.payTotal,
          balance: d.balance,
        })
      })
      .catch(() => {})
    return () => { cancelled = true }
  }, [activeCheckin, branchFetch])

  // #75 — load all rooms of the stay so a multi-room guest can release one,
  // some, or all rooms. Seed the selection with the room the user clicked.
  useEffect(() => {
    if (!activeCheckin) return
    let cancelled = false
    branchFetch(`/api/checkins/${activeCheckin.id}/rooms`)
      .then((r) => (r.ok ? r.json() : null))
      .then((d) => {
        if (cancelled || !d?.success) return
        const list = (d.data || []) as RoomLine[]
        setRooms(list)
        const clicked = list.find((r) => r.roomId === room.id && r.status === 'active')
        setSelectedCrIds(new Set(clicked ? [clicked.crId] : []))
      })
      .catch(() => {})
    return () => { cancelled = true }
  }, [activeCheckin, room.id, branchFetch])

  const activeRooms = rooms.filter((r) => r.status === 'active')
  // Multi-room is keyed on the TOTAL room count, not just active rooms: a stay
  // partially checked out down to one active room is still a multi-room stay, so
  // its last room must go through the per-room (crIds) path — not the legacy
  // whole-stay {} body. (The backend defends this too, but keep the client correct.)
  const isMultiRoom = rooms.length > 1
  const allActiveSelected =
    activeRooms.length > 0 && activeRooms.every((r) => selectedCrIds.has(r.crId))
  // Checking out every active room completes the stay → settle the whole-stay
  // balance now. A partial subset leaves the stay in-house, so the balance
  // settles when the LAST room checks out (avoids over-collecting the folio).
  const finalCheckout = !isMultiRoom || allActiveSelected
  const balanceDue =
    finalCheckout && folio && folio.balance > 0.005 ? folio.balance : 0
  const toggleCr = (crId: number) =>
    setSelectedCrIds((prev) => {
      const next = new Set(prev)
      if (next.has(crId)) next.delete(crId)
      else next.add(crId)
      return next
    })
  const toggleAll = () =>
    setSelectedCrIds(
      allActiveSelected ? new Set() : new Set(activeRooms.map((r) => r.crId)),
    )

  const submit = async () => {
    if (!activeCheckin) return
    if (isMultiRoom && selectedCrIds.size === 0) return
    setSubmitting(true)
    setError(null)
    try {
      // 1) Capture any outstanding balance as a payment first (reuse the #36
      //    payment path). Skipped when nothing is due, the folio failed to load
      //    (then checkout behaves like the legacy money-less flow), or the
      //    balance was already captured on a prior attempt in this session.
      if (balanceDue > 0 && !paymentCaptured) {
        const payRes = await branchFetch(
          `/api/checkins/${activeCheckin.id}/payments`,
          {
            method: 'POST',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify({
              amount: balanceDue,
              method: paymentMethod,
              notes: 'เช็คเอ้าท์',
            }),
          },
        )
        const payData = await payRes.json()
        if (!payRes.ok || !payData.success) {
          throw new Error(
            payData.error || payData.message || 'บันทึกการชำระเงินไม่สำเร็จ',
          )
        }
        setPaymentCaptured(true)
      }

      // 2) Complete the check-out. Multi-room stays send the selected
      //    `crIds` (per-room/partial checkout, #75); single-room stays send
      //    `{}` (the legacy whole-stay path — byte-identical to before).
      const res = await branchFetch(
        `/api/checkins/${activeCheckin.id}/checkout`,
        {
          method: 'PUT',
          headers: { 'Content-Type': 'application/json' },
          body: JSON.stringify(
            isMultiRoom ? { crIds: Array.from(selectedCrIds) } : {},
          ),
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
        className="bg-white rounded-lg shadow-xl w-full max-w-md max-h-[90vh] overflow-y-auto"
        onClick={(e) => e.stopPropagation()}
      >
        <div className="flex items-center justify-between p-4 border-b border-gray-200">
          <h2 className="text-lg font-bold text-gray-900">
            {rooms.length > 1
              ? `เช็คเอาท์ — ${activeCheckin?.customerName || activeCheckin?.cinNo || ''}`
              : `เช็คเอ้าท์ ห้อง ${room.roomNo}`}
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
                  <span>{formatThaiDate(activeCheckin.checkInTime)}</span>
                </div>
                <div className="flex justify-between">
                  <span className="text-gray-600">วันที่จะออก:</span>
                  <span>{formatThaiDate(activeCheckin.expectedCheckout)}</span>
                </div>
              </div>

              {/* #75 per-room checkout — pick which rooms to release. Only
                  shown for multi-room stays; single-room behaves as before. */}
              {rooms.length > 1 && (
                <div className="border border-gray-200 rounded">
                  <div className="flex items-center justify-between px-3 py-2 border-b border-gray-200 bg-gray-50">
                    <span className="text-sm font-semibold text-gray-800">เลือกห้องที่จะเช็คเอาท์</span>
                    <label className="flex items-center gap-1.5 text-xs text-gray-600 cursor-pointer">
                      <input type="checkbox" checked={allActiveSelected} onChange={toggleAll} />
                      เลือกทั้งหมด
                    </label>
                  </div>
                  <div className="divide-y divide-gray-100">
                    {rooms.map((r) => {
                      const out = r.status === 'checked_out'
                      return (
                        <label
                          key={r.crId}
                          className={`flex items-center gap-2 px-3 py-2 text-sm ${out ? 'opacity-50' : 'cursor-pointer hover:bg-sky-50'}`}
                        >
                          <input
                            type="checkbox"
                            disabled={out}
                            checked={!out && selectedCrIds.has(r.crId)}
                            onChange={() => toggleCr(r.crId)}
                          />
                          <span className="font-medium">ห้อง {r.roomNo}</span>
                          <span className="text-gray-500">· {r.nights} คืน</span>
                          <span className="ml-auto font-medium">{formatCurrency(r.roomSubtotal)}</span>
                          {out && <span className="text-xs text-green-600">เช็คเอาท์แล้ว</span>}
                        </label>
                      )
                    })}
                  </div>
                  {!finalCheckout && (
                    <div className="px-3 py-2 text-xs text-amber-700 bg-amber-50 border-t border-amber-200">
                      เช็คเอาท์เฉพาะห้องที่เลือก — ห้องที่เหลือยังเข้าพักอยู่ ยอดคงเหลือทั้งบิลจะเรียกเก็บเมื่อเช็คเอาท์ห้องสุดท้าย
                    </div>
                  )}
                </div>
              )}

              {/* Folio breakdown (server-authoritative; display regardless of
                  the CHECKOUT_SERVER_TOTAL_ENABLED flag). */}
              {folio && (
                <div className="border border-gray-200 rounded p-3 space-y-2 text-sm">
                  <div className="flex justify-between">
                    <span className="text-gray-600">ค่าห้อง ({folio.nights} คืน)</span>
                    <span className="font-medium">{formatCurrency(folio.roomTotal)}</span>
                  </div>
                  {folio.productTotal > 0 && (
                    <div className="flex justify-between">
                      <span className="text-gray-600">ค่าสินค้า / บริการ</span>
                      <span className="font-medium">{formatCurrency(folio.productTotal)}</span>
                    </div>
                  )}
                  <div className="flex justify-between pt-2 border-t border-gray-200">
                    <span className="font-semibold text-gray-800">รวมทั้งหมด</span>
                    <span className="font-bold">{formatCurrency(folio.netTotal)}</span>
                  </div>
                  {folio.payTotal > 0 && (
                    <div className="flex justify-between">
                      <span className="text-gray-600">ชำระแล้ว</span>
                      <span className="font-medium">−{formatCurrency(folio.payTotal)}</span>
                    </div>
                  )}
                  <div className="flex justify-between pt-2 border-t border-gray-200">
                    <span className="font-semibold text-gray-800">คงเหลือ</span>
                    <span className={`font-bold ${balanceDue > 0 ? 'text-red-600' : 'text-green-600'}`}>
                      {formatCurrency(folio.balance)}
                    </span>
                  </div>
                  {folio.deposit > 0 && (
                    // Deposit (มัดจำ) is HELD, not credited against the room
                    // bill — it is returned separately via คืนเงินมัดจำ (#49),
                    // matching iHOTEL. Shown here for visibility only; it does
                    // NOT reduce the balance above.
                    <div className="flex justify-between pt-2 border-t border-dashed border-gray-200 text-[13px]">
                      <span className="text-gray-500">มัดจำที่รับไว้ (คืนแยกต่างหาก)</span>
                      <span className="font-medium text-gray-700">{formatCurrency(folio.deposit)}</span>
                    </div>
                  )}
                </div>
              )}

              {/* Tender select — only meaningful when there is a balance to
                  settle. The selection drives the payment recorded on confirm. */}
              {balanceDue > 0 && (
                <div>
                  <label className="block text-sm font-medium text-gray-700 mb-1">
                    <CreditCard size={16} className="inline mr-1" />
                    วิธีการชำระเงิน (ยอดคงเหลือ {formatCurrency(balanceDue)})
                  </label>
                  <select
                    value={paymentMethod}
                    onChange={(e) => setPaymentMethod(e.target.value as PaymentMethod)}
                    className="w-full px-3 py-2 bg-gray-50 border border-gray-300 text-gray-800 rounded focus:ring-2 focus:ring-sky-500 focus:border-sky-500"
                  >
                    {paymentMethods.map((m) => (
                      <option key={m.value} value={m.value}>
                        {m.label}
                      </option>
                    ))}
                  </select>
                </div>
              )}

              <div className="text-xs text-gray-500">
                {balanceDue > 0
                  ? 'ยอดคงเหลือจะถูกบันทึกเป็นการชำระเงินก่อนเช็คเอ้าท์'
                  : 'ไม่มียอดค้างชำระ เช็คเอ้าท์ได้ทันที'}
              </div>

              {/* ADR 0008 room-check — APPENDED at the end of the body so no
                  existing checkout step moves, and adjacent to the confirm
                  button, which is where an unresolved ของหาย/ของเสียหาย has
                  to be seen. Manual by contract: it never fires on open. */}
              {roomCheck && <RoomCheckPanel roomId={room.id} roomNo={room.roomNo} />}
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
            disabled={
              submitting ||
              loading ||
              !activeCheckin ||
              (isMultiRoom && selectedCrIds.size === 0)
            }
            className="px-4 py-2 text-sm font-medium text-white bg-sky-600 rounded hover:bg-sky-700 disabled:opacity-50 flex items-center"
          >
            {submitting ? (
              <Loader2 size={14} className="mr-2 animate-spin" />
            ) : (
              <LogOut size={14} className="mr-2" />
            )}
            {balanceDue > 0
              ? 'ชำระเงินและเช็คเอ้าท์'
              : isMultiRoom && !allActiveSelected
                ? `เช็คเอาท์ห้องที่เลือก (${selectedCrIds.size})`
                : 'ยืนยันเช็คเอ้าท์'}
          </button>
        </div>
      </div>
    </div>
  )
}
