'use client'

import { useCallback, useEffect, useMemo, useRef, useState } from 'react'
import { AlertTriangle, CheckCircle2, Clock, Loader2, Send } from 'lucide-react'
import { useDeskSignals } from './use-desk-signals'
import {
  actorName,
  canDeskAck,
  canDeskDone,
  formatSignalTime,
  isGuestAccountability,
  signalLabel,
  type RoomSignal,
} from './signal-lib'

/**
 * ขอเช็คห้อง on the checkout + settle surface (ADR 0008, CONTEXT.md
 * §Housekeeping "Room-check").
 *
 * The room-check is the checkout coordinator and the most urgent signal in the
 * system, because the guest is standing at the counter. This panel is the
 * desk's whole side of it:
 *
 *   idle → [ขอเช็คห้อง] → รอผลตรวจ → เคลียร์ (settle now)
 *                                  ↘ มีของหาย / มีของเสียหาย (act first)
 *
 * ADDITIVE ONLY (improvement invariant). The button is MANUAL — CONTEXT.md
 * rules out auto-firing on checkout open, because many checkouts settle
 * without an inspection — it appends below the existing modal body so no
 * existing checkout step moves or disappears, and it never blocks the confirm
 * button: an unresolved problem is rendered loudly, but whether to settle
 * anyway stays reception's judgment (iHOTEL never gated the settle either).
 *
 * ── Reading the answer ─────────────────────────────────────────────────────
 * `GET /api/housekeeping/signals` returns `status IN (open, acked)` by
 * contract, so the moment a maid answers, the check leaves the list:
 *
 * - A **problems** answer is still fully visible, because the answer inserts
 *   one standing `maid_to_desk` child per problem (`parentId` → the check) and
 *   those children stay open until the desk completes them. Those children are
 *   what this panel shows in red, and they are shown whether or not this tab
 *   ever saw the parent — a guest-accountability signal reaching the settle
 *   screen is the point of the whole feature.
 * - A **เคลียร์** answer leaves nothing behind in `signals`. It is read from
 *   the read's SECOND list, `answeredRoomChecks`: today's newest answered
 *   `room_check` per room, `outcome` included. That is server state, so it
 *   survives a reload — which is what closed the v1 gap this file used to
 *   document ("เคลียร์ invisible after desk tab reload"). Cancelled checks are
 *   excluded from that list by contract, so it cannot manufacture a green.
 *
 * TWO safety rules sit on top of the server list, because "the newest answer
 * today" is not the same question as "the answer to the check this desk is
 * waiting on":
 *
 * - A LIVE `room_check` for this room always wins. While one is open or acked
 *   the panel shows รอผลตรวจ, never the older answer underneath it.
 * - A brand-new ขอเช็คห้อง, or a desk CANCEL, raises a per-room floor on which
 *   answer ids may still be displayed (`ANSWER_FLOOR`). Otherwise a desk that
 *   asked again and cancelled would drop straight back onto this morning's
 *   green — a false clear on the exact screen where money changes hands.
 *
 * ── Older backend (field absent) ───────────────────────────────────────────
 * `answeredRoomChecks === null` means the payload had no such field: this
 * bundle is talking to a backend that predates it. The panel then behaves
 * EXACTLY as it did before — เคลียร์ inferred from the transition this tab
 * watched (a tracked check left the open/acked list and spawned no problem
 * children), remembered module-level for the life of the tab, forgotten on
 * reload. That path fails toward asking again, never toward a false green.
 */

/** Observed เคลียร์ answers, keyed by room id, for the life of the tab.
 *  The FALLBACK display path (older backend); with `answeredRoomChecks`
 *  present the server list is authoritative and this is only recorded, not
 *  read. Module-level on purpose: a receptionist fires ขอเช็คห้อง, CLOSES the
 *  modal while the maid walks up, and reopens it — which unmounts and remounts
 *  this component. React state would forget across that; this does not. */
const OBSERVED_CLEAR = new Map<number, number>()

/** Per room, the lowest answered-check id this panel may still display.
 *  Raised when the desk asks again or cancels, so a superseded answer can
 *  never paint over a fresh request. Module-level for the same
 *  close-and-reopen reason as `OBSERVED_CLEAR`. */
const ANSWER_FLOOR = new Map<number, number>()

function raiseAnswerFloor(roomId: number, minVisibleId: number) {
  const current = ANSWER_FLOOR.get(roomId) ?? 0
  if (minVisibleId > current) ANSWER_FLOOR.set(roomId, minVisibleId)
}

export const REQUEST_LABEL = 'ขอเช็คห้อง'
export const PENDING_TEXT = 'รอผลตรวจห้องจากแม่บ้าน'
export const CLEAR_TEXT = 'แม่บ้านตอบว่า เคลียร์ — ชำระเงินและเช็คเอาท์ได้'
export const PROBLEMS_TEXT = 'ต้องจัดการก่อนชำระเงิน'
export const SECTION_TITLE = 'ตรวจห้องก่อนเช็คเอาท์'

function SmallButton({
  label,
  onClick,
  busy,
  tone,
}: {
  label: string
  onClick: () => void
  busy?: boolean
  tone: 'plain' | 'primary'
}) {
  const base =
    'inline-flex items-center gap-1.5 rounded px-2.5 py-1 text-xs font-medium disabled:opacity-50'
  const skin =
    tone === 'primary'
      ? 'bg-sky-600 text-white hover:bg-sky-700'
      : 'border border-gray-300 bg-white text-gray-700 hover:bg-gray-50'
  return (
    <button type="button" onClick={onClick} disabled={busy} className={`${base} ${skin}`}>
      {busy && <Loader2 size={12} className="animate-spin" />}
      {label}
    </button>
  )
}

export default function RoomCheckPanel({
  roomId,
  roomNo,
}: {
  roomId: number
  roomNo: string
}) {
  // Its own subscription: the checkout modal runs no useLiveRefresh of its
  // own, and the guest is waiting, so the safety poll is tighter than the
  // board's 60s.
  const client = useDeskSignals({ pollMs: 15000 })
  const {
    signals,
    answeredRoomChecks,
    enabled,
    loading,
    error,
    busySignalId,
    sendingRoomId,
    send,
    act,
  } = client

  const [clearedSignalId, setClearedSignalId] = useState<number | null>(
    () => OBSERVED_CLEAR.get(roomId) ?? null,
  )
  const [answerFloor, setAnswerFloor] = useState<number>(() => ANSWER_FLOOR.get(roomId) ?? 0)

  /** Raise the floor in the module map AND in render state, together. */
  const bumpAnswerFloor = useCallback(
    (minVisibleId: number) => {
      raiseAnswerFloor(roomId, minVisibleId)
      setAnswerFloor((current) => (minVisibleId > current ? minVisibleId : current))
    },
    [roomId],
  )

  const roomSignals = useMemo(
    () => signals.filter((s) => s.roomId === roomId),
    [signals, roomId],
  )

  const pendingCheck: RoomSignal | undefined = useMemo(
    () => roomSignals.find((s) => s.type === 'room_check'),
    [roomSignals],
  )

  /** Standing guest-accountability signals on this room. Shown regardless of
   *  whether they came from a room-check answer — the desk must know before it
   *  settles either way. */
  const problems = useMemo(
    () => roomSignals.filter((s) => s.direction === 'maid_to_desk' && isGuestAccountability(s)),
    [roomSignals],
  )

  /** Today's answer for THIS room, once the floor has had its say. `null` both
   *  when the backend sent no field and when it sent nothing for this room. */
  const answeredCheck: RoomSignal | null = useMemo(() => {
    if (!answeredRoomChecks) return null
    return (
      answeredRoomChecks.find((s) => s.roomId === roomId && s.signalId >= answerFloor) ?? null
    )
  }, [answeredRoomChecks, roomId, answerFloor])

  // Which check this panel is watching, so its disappearance can be read.
  const trackedRef = useRef<number | null>(null)

  useEffect(() => {
    if (loading) return
    if (pendingCheck) {
      trackedRef.current = pendingCheck.signalId
      return
    }
    const tracked = trackedRef.current
    if (tracked == null) return
    trackedRef.current = null
    // Children of THIS check mean the answer was "problems"; the red block
    // below already renders them, so nothing green may be recorded.
    if (problems.some((p) => p.parentId === tracked)) return
    OBSERVED_CLEAR.set(roomId, tracked)
    setClearedSignalId(tracked)
  }, [loading, pendingCheck, problems, roomId])

  const request = useCallback(async () => {
    OBSERVED_CLEAR.delete(roomId)
    setClearedSignalId(null)
    // Asking again retires whatever was answered before this tap — including
    // an answer still sitting in `answeredRoomChecks`. Raise the floor BEFORE
    // the write so the old green cannot flash while the send is in flight.
    if (answeredCheck) bumpAnswerFloor(answeredCheck.signalId + 1)
    const created = await send(roomId, 'room_check')
    // The new check's own id is the exact floor: only ITS answer counts now.
    if (created) bumpAnswerFloor(created.signalId)
  }, [send, roomId, answeredCheck, bumpAnswerFloor])

  const cancel = useCallback(
    async (signalId: number) => {
      // Drop the tracking BEFORE the write: a cancelled check disappears from
      // the list exactly like an answered one, and must not read as เคลียร์.
      trackedRef.current = null
      // Same for the server list: a cancelled check is never IN it, so without
      // this the panel would fall back onto an earlier answer for this room.
      bumpAnswerFloor(signalId)
      await act(signalId, 'cancel')
    },
    [act, bumpAnswerFloor],
  )

  // Aggregate ("ทั้งหมด") view: a signal is always about one room at one
  // property and the endpoints require a real ?branch=. Nothing to offer.
  if (!enabled) return null

  // Server list present ⇒ it is authoritative. Absent ⇒ the module-memory
  // inference this file used before the field existed.
  const answerKnown = answeredRoomChecks != null
  const clearAnswered = answerKnown ? answeredCheck?.outcome === 'clear' : clearedSignalId != null
  const showClear = clearAnswered && !pendingCheck && problems.length === 0

  // A `problems` answer with every child already completed still must not read
  // as "nothing to see" on the settle screen; the children themselves arrive
  // through the ordinary signals list and render below the heading.
  const showProblems =
    problems.length > 0 || (!pendingCheck && answeredCheck?.outcome === 'problems')

  return (
    <div className="border-t border-gray-200 pt-3 space-y-2">
      <div className="text-xs font-semibold text-gray-500">{SECTION_TITLE}</div>

      {showProblems && (
        <div className="rounded border border-red-300 bg-red-50 p-3 space-y-2">
          <div className="flex items-center gap-2 text-sm font-bold text-red-700">
            <AlertTriangle size={16} className="shrink-0" />
            {PROBLEMS_TEXT}
          </div>
          {problems.map((problem) => (
            <div
              key={problem.signalId}
              className="flex flex-wrap items-center gap-2 text-sm text-red-800"
            >
              <span className="font-semibold">{signalLabel(problem.type)}</span>
              <span className="text-xs text-red-600">
                {actorName(problem.createdBy)}
                {formatSignalTime(problem.createdAt) && ` · ${formatSignalTime(problem.createdAt)}`}
              </span>
              <span className="ml-auto flex items-center gap-2">
                {canDeskAck(problem) && (
                  <SmallButton
                    label="รับทราบ"
                    tone="plain"
                    busy={busySignalId === problem.signalId}
                    onClick={() => act(problem.signalId, 'ack')}
                  />
                )}
                {canDeskDone(problem) && (
                  <SmallButton
                    label="เสร็จสิ้น"
                    tone="primary"
                    busy={busySignalId === problem.signalId}
                    onClick={() => act(problem.signalId, 'done')}
                  />
                )}
              </span>
            </div>
          ))}
        </div>
      )}

      {pendingCheck ? (
        <div className="flex flex-wrap items-center gap-2 rounded border border-amber-300 bg-amber-50 p-3 text-sm text-amber-800">
          <Clock size={16} className="shrink-0" />
          <span className="font-medium">{PENDING_TEXT}</span>
          <span className="text-xs">
            ห้อง {roomNo}
            {formatSignalTime(pendingCheck.createdAt) &&
              ` · ส่งเมื่อ ${formatSignalTime(pendingCheck.createdAt)}`}
            {pendingCheck.status === 'acked' &&
              actorName(pendingCheck.ackedBy) &&
              ` · รับแล้ว โดย ${actorName(pendingCheck.ackedBy)}`}
          </span>
          <span className="ml-auto">
            <SmallButton
              label="ยกเลิก"
              tone="plain"
              busy={busySignalId === pendingCheck.signalId}
              onClick={() => cancel(pendingCheck.signalId)}
            />
          </span>
        </div>
      ) : (
        <>
          {showClear && (
            <div className="flex items-center gap-2 rounded border border-green-300 bg-green-50 p-3 text-sm font-medium text-green-700">
              <CheckCircle2 size={16} className="shrink-0" />
              {CLEAR_TEXT}
            </div>
          )}
          <button
            type="button"
            onClick={request}
            disabled={sendingRoomId === roomId}
            className="inline-flex items-center gap-2 rounded border border-sky-300 bg-white px-3 py-2 text-sm font-medium text-sky-700 hover:bg-sky-50 disabled:opacity-50"
          >
            {sendingRoomId === roomId ? (
              <Loader2 size={15} className="animate-spin" />
            ) : (
              <Send size={15} />
            )}
            {REQUEST_LABEL}
          </button>
        </>
      )}

      {error && <div className="text-xs text-red-600">{error}</div>}
    </div>
  )
}
