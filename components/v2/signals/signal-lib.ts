/**
 * Room signals on the RECEPTION DESK surface — pure helpers (ADR 0008,
 * CONTEXT.md §Housekeeping).
 *
 * The vocabulary itself lives in ONE place, `app/hk/signal-vocab.ts`, and is
 * shared verbatim with the maid `/hk` surface and mirrored by the backend
 * allowlists. Nothing here re-spells a signal type or a Thai label; this file
 * only holds what is specific to the desk: which endpoints the desk calls,
 * which actions the desk is allowed to offer, and how a signal is ordered and
 * tinted on a reception screen.
 *
 * Canned-only by decision — there is deliberately no free-text field anywhere
 * in this feature, so there is nothing here that composes one.
 */

import {
  DESK_SIGNALS,
  signalLabel,
  type RoomSignal,
  type SignalStatus,
} from '@/app/hk/signal-vocab'

export { DESK_SIGNALS, signalLabel }
export type { RoomSignal }

// ---------------------------------------------------------------------------
// Endpoints — the DESK half of the contract (hotel-backend/src/routes/
// housekeeping.rs, behind the existing reception auth). The maid half lives at
// /api/hk/* and is never called from here.
// ---------------------------------------------------------------------------

/** Open + acked signals for one branch. `?branch=` is appended by branchFetch. */
export const DESK_SIGNALS_ENDPOINT = '/api/housekeeping/signals'

/** Send one desk→maid signal about one room. */
export function deskSendEndpoint(roomId: number): string {
  return `/api/housekeeping/rooms/${roomId}/signals`
}

/** The three lifecycle taps the desk may make. `answer` is deliberately absent:
 *  a ขอเช็คห้อง is completed by the MAID answering (เคลียร์ / problems), never
 *  by a desk tap — the desk `done` on a room_check is a 400 by contract. */
export type DeskSignalAction = 'ack' | 'done' | 'cancel'

export function deskActionEndpoint(signalId: number, action: DeskSignalAction): string {
  return `/api/housekeeping/signals/${signalId}/${action}`
}

/**
 * Live event names the desk subscribes to for signal changes — the four
 * variants the backend actually publishes on the reception relay
 * (`outbox::event::ROOM_SIGNAL_EVENT_NAMES` in
 * `hotel-backend/src/outbox/event.rs` — keep in lock-step). The maid stream's
 * `hk_signal` name does not appear on this relay. Every one of these merely
 * schedules the same debounced refetch, so an extra name would cost nothing —
 * but a MISSING name silently downgrades that transition to the safety poll,
 * which is why the list mirrors the backend constant exactly.
 */
export const SIGNAL_LIVE_EVENTS = [
  'RoomSignalRaised',
  'RoomSignalAcked',
  'RoomSignalCompleted',
  'RoomSignalCancelled',
] as const

// ---------------------------------------------------------------------------
// Display vocabulary
// ---------------------------------------------------------------------------

/** Thai status wording. `done`/`cancelled` never reach the desk list (the read
 *  is `status IN (open, acked)`) but are spelled here so a late-arriving DTO
 *  from an action response renders as words, not as a raw code. */
export const SIGNAL_STATUS_LABELS: Record<SignalStatus, string> = {
  open: 'รอรับ',
  acked: 'รับแล้ว',
  done: 'เสร็จสิ้น',
  cancelled: 'ยกเลิกแล้ว',
}

export function signalStatusLabel(status: string): string {
  return SIGNAL_STATUS_LABELS[status as SignalStatus] ?? status
}

/**
 * The v2 tint class for one signal.
 *
 * Signals are NOT room state, so this is not the iHOTEL room-state palette
 * (CONTEXT.md §UX "Status colors" governs the hue that MEANS vacant/occupied/
 * dirty/reserved/maintenance, and nothing here claims one of those meanings).
 * Two of the existing operational tones are borrowed for their established
 * reception meaning:
 *
 * - `out` (DeepSkyBlue, iHOTEL's "ยังไม่ได้ Check-Out") for ขอเช็คห้อง and
 *   แขกเช็คเอาท์แล้ว — both are the checkout axis.
 * - `fix` (the alarm tone) for the two guest-accountability signals, มีของหาย
 *   and มีของเสียหาย: money-adjacent, and the desk must see them before it
 *   settles.
 *
 * Everything else is `arr` while it waits and neutral `mut` once somebody has
 * put their name on it — an acked signal is somebody's job now, not the room
 * board's alarm.
 */
export function signalTone(signal: Pick<RoomSignal, 'type' | 'status'>): string {
  if (signal.type === 'item_missing' || signal.type === 'item_damaged') return 'fix'
  if (signal.type === 'room_check' || signal.type === 'checked_out') return 'out'
  return signal.status === 'acked' ? 'mut' : 'arr'
}

/** `name ?? badge` — the CF IdP forwards only `apps`+`badge` today, so `name`
 *  is usually null. Same rule the cleaning board uses for its reporter line. */
export function actorName(actor?: { badge: string; name: string | null } | null): string {
  if (!actor) return ''
  return actor.name ?? actor.badge
}

/**
 * Format a signal instant for display.
 *
 * `createdAt` / `ackedAt` are real PG `timestamptz` values, so the browser's
 * NORMAL local conversion is exactly right — deliberately no `timeZone`
 * override. (`'UTC'` belongs only to naive datetimes mirrored out of legacy
 * MSSQL; a pinned `'Asia/Bangkok'` is the same mistake in the other
 * direction.) Matches `formatEventTime` on the cleaning board.
 */
export function formatSignalTime(at: string | null | undefined): string {
  if (!at) return ''
  const instant = new Date(at)
  if (Number.isNaN(instant.getTime())) return ''
  return instant.toLocaleTimeString('th-TH', { hour: '2-digit', minute: '2-digit' })
}

// ---------------------------------------------------------------------------
// What the DESK may act on
// ---------------------------------------------------------------------------
//
// "Nobody acts on their own direction's signals except cancel-own-while-open."
// The desk is the creator side of `desk_to_maid`, so it may only cancel those,
// and it is the audience for `maid_to_desk`, so it acks and completes those.
// The backend enforces all of this; these predicates keep the UI from ever
// offering a tap that is going to come back a 400.

export function canDeskAck(signal: RoomSignal): boolean {
  return signal.direction === 'maid_to_desk' && signal.status === 'open'
}

export function canDeskDone(signal: RoomSignal): boolean {
  return (
    signal.direction === 'maid_to_desk' &&
    (signal.status === 'open' || signal.status === 'acked')
  )
}

export function canDeskCancel(signal: RoomSignal): boolean {
  return signal.direction === 'desk_to_maid' && signal.status === 'open'
}

/** The two maid→desk types that mean "this room's guest may owe for something".
 *  Kept as a predicate rather than a second list so the codes stay spelled in
 *  `signal-vocab.ts` alone. */
export function isGuestAccountability(signal: Pick<RoomSignal, 'type'>): boolean {
  return signal.type === 'item_missing' || signal.type === 'item_damaged'
}

// ---------------------------------------------------------------------------
// Ordering and grouping
// ---------------------------------------------------------------------------

/** Lower sorts first. ขอเช็คห้อง leads because the guest is standing at the
 *  counter; the guest-accountability pair follows because money depends on it. */
function priority(type: string): number {
  if (type === 'room_check') return 0
  if (type === 'item_missing' || type === 'item_damaged') return 1
  if (type === 'guest_in_room') return 2
  return 3
}

/** Urgent first, then longest-waiting first — a reception screen is read for
 *  "what needs me now", not for a chronology. Returns a new array. */
export function sortSignals(signals: readonly RoomSignal[]): RoomSignal[] {
  return [...signals].sort((a, b) => {
    const byPriority = priority(a.type) - priority(b.type)
    if (byPriority !== 0) return byPriority
    const byAge = a.createdAt.localeCompare(b.createdAt)
    if (byAge !== 0) return byAge
    return a.signalId - b.signalId
  })
}

/** Signals keyed by room number, each list already in `sortSignals` order.
 *  Keyed on `roomNo` (not `roomId`) because the cleaning board's room list
 *  carries only the number. */
export function signalsByRoomNo(signals: readonly RoomSignal[]): Map<string, RoomSignal[]> {
  const byRoom = new Map<string, RoomSignal[]>()
  for (const signal of sortSignals(signals)) {
    const list = byRoom.get(signal.roomNo)
    if (list) list.push(signal)
    else byRoom.set(signal.roomNo, [signal])
  }
  return byRoom
}
