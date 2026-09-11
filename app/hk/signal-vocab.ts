// Room-signal vocabulary — the ONE place a signal type is spelled (ADR 0008,
// CONTEXT.md §Housekeeping). Canned-only by decision: there is deliberately no
// free-text field anywhere in this feature. Imported by BOTH the maid /hk
// surface and the reception v2 surfaces; the backend's allowlists mirror these
// codes (hotel-backend/src/routes/hk.rs — keep in lock-step).

/** Desk→maid signal types, in display order. */
export const DESK_SIGNALS = [
  { type: 'room_check', label: 'ขอเช็คห้อง' },
  { type: 'priority_clean', label: 'ทำห้องนี้ก่อน' },
  { type: 'deliver_linen', label: 'แขกขอผ้าเพิ่ม' },
  { type: 'skip_room', label: 'งดทำห้องนี้' },
  { type: 'checked_out', label: 'แขกเช็คเอาท์แล้ว' },
] as const

/** Maid→desk signal types, in display order. */
export const MAID_SIGNALS = [
  { type: 'guest_in_room', label: 'ลูกค้ายังอยู่ในห้อง' },
  { type: 'found_belongings', label: 'พบของลืมในห้อง' },
  { type: 'item_missing', label: 'มีของหาย' },
  { type: 'item_damaged', label: 'มีของเสียหาย' },
] as const

export type DeskSignalType = (typeof DESK_SIGNALS)[number]['type']
export type MaidSignalType = (typeof MAID_SIGNALS)[number]['type']
export type SignalType = DeskSignalType | MaidSignalType

/** The two problems a ขอเช็คห้อง answer may carry (≥1 when not เคลียร์). */
export const ROOM_CHECK_PROBLEMS = [
  { type: 'item_missing', label: 'มีของหาย' },
  { type: 'item_damaged', label: 'มีของเสียหาย' },
] as const

export type SignalStatus = 'open' | 'acked' | 'done' | 'cancelled'
export type SignalDirection = 'desk_to_maid' | 'maid_to_desk'
export type RoomCheckOutcome = 'clear' | 'problems'
export type SignalDoneSource = 'tap' | 'clean_report' | 'room_check_answer'

/** One signal as every endpoint serializes it (camelCase on the wire). */
export interface RoomSignal {
  signalId: number
  roomId: number
  roomNo: string
  direction: SignalDirection
  /** `string`, not `SignalType`: server→client must render a type this bundle
   *  predates rather than crash — `signalLabel` falls back to the raw code. */
  type: string
  status: SignalStatus
  outcome?: RoomCheckOutcome | null
  parentId?: number | null
  createdBy: { badge: string; name: string | null }
  createdAt: string
  ackedBy?: { badge: string; name: string | null } | null
  ackedAt?: string | null
  doneBy?: { badge: string; name: string | null } | null
  doneAt?: string | null
  doneSource?: SignalDoneSource | null
}

const ALL_LABELS: Record<string, string> = Object.fromEntries(
  [...DESK_SIGNALS, ...MAID_SIGNALS].map(({ type, label }) => [type, label])
)

/** Thai label for a signal type; the raw code for one this bundle predates. PURE. */
export function signalLabel(type: string): string {
  return ALL_LABELS[type] ?? type
}
