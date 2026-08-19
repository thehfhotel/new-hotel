// Shared helpers for the maid-facing housekeeping surface (/hk).
//
// The /hk API deliberately lives UNDER /hk (`/hk/api/*`, rewritten by
// next.config.js to the backend's /api/hk/*) so ONE path-scoped Cloudflare
// Access application covers both the pages and their API calls — see the
// rewrite comment in next.config.js. Everything here is plain TypeScript so
// the pure helpers are unit-testable without a DOM.

import { HK_STATUS_LABELS } from '@/lib/v2/status'

// ---------------------------------------------------------------------------
// API types (mirror hotel-backend/src/routes/hk.rs)
// ---------------------------------------------------------------------------

/** The two properties `/hk` may ever serve. Exactly the `?branch=` spelling
 * the backend's `HkPolicy` accepts — never `"all"` (§A2: `write_pool(Some(All))`
 * silently returns the primary pool, which would re-open the wrong-hotel bug
 * this whole surface exists to close). */
export type Branch = 'hfhotel' | 'hfville'

export type CleaningStatus = 'started' | 'done' | 'dirty'

/** Whether a guest currently occupies the room, independent of cleanliness —
 * "ว่าง + ไม่สะอาด" (guest just left) is a normal, expected combination, not a
 * contradiction to resolve. */
export type Occupancy = 'occupied' | 'vacant'

/** One selectable property on the branch picker, served by `GET /api/hk/me`
 * so `HK_BRANCHES` has ONE source of truth (no `NEXT_PUBLIC_*` coupling). */
export interface HkBranchOption {
  id: Branch
  labelTh: string
}

/**
 * Why `branches` is empty, when it is (`HkMe.branchesUnavailableReason`).
 * Mirrors `REASON_NO_LOCATION` / `REASON_LOOKUP_UNAVAILABLE` in
 * `hotel-backend/src/routes/hk.rs`. The two are NOT interchangeable:
 * `no_location` needs an admin, `lookup_unavailable` needs a retry.
 */
export type HkBranchesUnavailableReason = 'no_location' | 'lookup_unavailable'

export interface HkMe {
  success: boolean
  badge: string
  displayName: string | null
  /**
   * In `HK_BRANCHES` order. Length 1 ⇒ the client auto-selects, no picker.
   *
   * With `HK_LOCATION_ENFORCEMENT_ENABLED` on this is the allowlist
   * INTERSECTED with the employee's own HF ID location, so it is normally
   * length 1 — and CAN be EMPTY, which `branchesUnavailableReason` explains.
   * An empty list is never a reason to fall back to a default branch.
   */
  branches: HkBranchOption[]
  /** `HK_MARK_DIRTY_ENABLED` — hides the "แจ้งห้องไม่สะอาด" button when false. */
  markDirtyEnabled: boolean
  /**
   * Set exactly when `branches` is empty; `null` otherwise (and always while
   * location enforcement is off). Always PRESENT — branch on the value, not on
   * the key.
   */
  branchesUnavailableReason: HkBranchesUnavailableReason | null
}

export interface HkCleaningProgress {
  status: CleaningStatus
  badge: string
  name: string | null
  /** A PG `timestamptz` — a real instant. Render with `timeLabel` below
   * (plain local-time formatting), never with a `timeZone: 'UTC'` override —
   * that hack is reserved for naive LEGACY MSSQL datetimes, which never
   * reach `/hk`. */
  at: string
}

export interface HkRoom {
  roomId: number
  roomNo: string
  floor: number | null
  building: string | null
  /**
   * Cleanliness as **iHOTEL** reports it (CR-1) — `true` = clean.
   *
   * The server merges legacy `HT_Rooms.Room_Clean` over the canonical PMS
   * value so the maid and reception read the same room the same way. When the
   * legacy read cannot answer, this is the PMS mirror instead and the
   * response's `legacyStatusStale` says so. Nothing about the divergence
   * itself reaches the client — the maid gets ONE value to act on.
   */
  roomClean: boolean
  cleaning: HkCleaningProgress | null
  /**
   * Guest occupancy, independent of cleanliness. Always sent by a new backend;
   * optional here (TS only, not on the wire) so an older bundle talking to a
   * newer backend — or a newer bundle mid-deploy against an older backend —
   * degrades to `undefined`, which `occupancyIndicator` renders as nothing
   * rather than a guess.
   */
  occupancy?: Occupancy
  /**
   * Canonical-side day-scoped movement facts — NOT part of the iHOTEL merge
   * (CR-1) and NOT covered by `legacyStatusStale`: these describe today's
   * bookings, not room cleanliness/occupancy. `expectedArrival` is a booking
   * starting today not yet checked in; `expectedDeparture` is a guest due out
   * today, overstays included. A room can be BOTH (back-to-back). Optional
   * here for the same bundle/backend-skew reason as `occupancy`.
   */
  expectedArrival?: boolean
  expectedDeparture?: boolean
}

/** `GET /hk/api/rooms`. */
export interface HkRoomsResponse {
  success: boolean
  data: HkRoom[]
  /**
   * `true` ⇒ iHOTEL could not be reached and every `roomClean` above is the
   * PMS mirror. Render `LEGACY_STATUS_STALE_NOTE` — never an error state: a
   * stale list is workable, a blank one strands a maid on a stairwell.
   *
   * Optional on the type (not on the wire) so an older cached bundle talking
   * to a newer backend, or the reverse, degrades to "no note" rather than
   * `undefined` leaking into the UI.
   */
  legacyStatusStale?: boolean
}

export interface HkCleaningEvent {
  eventId: number
  status: CleaningStatus
  badge: string
  name: string | null
  at: string
}

export interface HkRoomDetail {
  success: boolean
  room: HkRoom
  events: HkCleaningEvent[]
  /** Same meaning as `HkRoomsResponse.legacyStatusStale`. Present on both so
   * the two screens can never tell the maid different stories. */
  legacyStatusStale?: boolean
}

// ---------------------------------------------------------------------------
// Branch selection (localStorage)
// ---------------------------------------------------------------------------
//
// Mirrors the estate precedent the same maids already rely on from the same
// LINE Role Menu — `~/HF/housekeeping/src/client/shared/property.ts`: asked
// once on first run, kept in localStorage, switchable from the header; a null
// value is the signal to render the picker, NEVER a reason to default to one
// branch and file a maid's report against the wrong hotel. Different origins
// (hotel.thehfhotel.org vs housekeeping.thehfhotel.org) mean localStorage
// itself can't be shared — only the pattern is copied.

const BRANCH_STORAGE_KEY = 'hk.branch'
const KNOWN_BRANCHES: readonly Branch[] = ['hfhotel', 'hfville']

function isBranch(value: unknown): value is Branch {
  return typeof value === 'string' && (KNOWN_BRANCHES as readonly string[]).includes(value)
}

/** The maid's stored branch, or `null` if never chosen, cleared, unknown, or
 * storage is unavailable (private browsing). Never guesses. */
export function readStoredBranch(): Branch | null {
  try {
    const stored = localStorage.getItem(BRANCH_STORAGE_KEY)
    return isBranch(stored) ? stored : null
  } catch {
    // Private mode / storage disabled: the picker just reappears every load
    // rather than crashing or silently defaulting.
    return null
  }
}

export function storeBranch(branch: Branch): void {
  try {
    localStorage.setItem(BRANCH_STORAGE_KEY, branch)
  } catch {
    /* nothing to do — the picker simply reappears next load */
  }
}

/**
 * The branch a page should use on first render, BEFORE any tap (§A3). The
 * rules are evaluated in this order, and the ORDER IS THE POINT:
 *  1. a stored value that is no longer in `branches` ⇒ `null`. Discard it and
 *     re-ask — even when only one branch is left. This must be checked FIRST:
 *     if the single-branch auto-select ran first, a Ville maid whose branch was
 *     removed by a rollback (`HK_BRANCHES=hfhotel,hfville` → `hfhotel`) would be
 *     silently moved to HF Hotel with no picker and no notice, and her next
 *     report on ห้อง 203 would file against the OTHER hotel's room 203 — the
 *     exact wrong-hotel bug this surface exists to close.
 *  2. a still-valid stored choice ⇒ that choice.
 *  3. nothing stored AND exactly one configured branch ⇒ auto-select it, no
 *     picker rendered — this is the shipping state (HF Hotel only), so maids
 *     who never chose see zero new UI.
 *  4. anything else ⇒ `null` — the caller must render the picker and block,
 *     never fall back to a default.
 */
export function resolveInitialBranch(
  branches: HkBranchOption[],
  stored: Branch | null
): Branch | null {
  if (stored) {
    // Rule 1 before rule 3: a stale stored branch always re-asks.
    return branches.some((b) => b.id === stored) ? stored : null
  }
  if (branches.length === 1) return branches[0].id
  return null
}

// ---------------------------------------------------------------------------
// Fetch helpers
// ---------------------------------------------------------------------------

/** Base path of the maid API as seen from the browser (Access-scoped). */
export const HK_API_BASE = '/hk/api'

/**
 * Base URL of the Housekeeping ops app (แจ้งซ่อม / เบิกของ). Build-time
 * `NEXT_PUBLIC_*` because the /hk pages are client components — same idiom as
 * `NEXT_PUBLIC_CARD_READER_URL`. Trailing slash trimmed so path joins are safe.
 */
export const HOUSEKEEPING_URL = (
  process.env.NEXT_PUBLIC_HOUSEKEEPING_URL || 'https://housekeeping.thehfhotel.org'
).replace(/\/+$/, '')

const UNAUTHENTICATED_MESSAGE =
  'ไม่สามารถยืนยันตัวตนได้ กรุณาเปิดหน้านี้ผ่านเมนูพนักงานอีกครั้ง'
const FORBIDDEN_MESSAGE = 'บัญชีของคุณไม่มีสิทธิ์ใช้งานระบบแม่บ้าน'
const BRANCH_ERROR_MESSAGE = 'สาขาที่เลือกไม่ถูกต้องหรือยังไม่รองรับ กรุณาเลือกสาขาใหม่อีกครั้ง'
const NO_BRANCH_MESSAGE = 'ยังไม่ได้เลือกสาขา กรุณาเลือกสาขาก่อนใช้งาน'
const LOOKUP_UNAVAILABLE_MESSAGE =
  'ระบบตรวจสอบสาขาพนักงานขัดข้องชั่วคราว กรุณาลองใหม่อีกครั้ง หากยังไม่ได้ ให้ติดต่อผู้ดูแลระบบ'

/**
 * The actionable Thai copy for an empty `branches` list. PURE — unit-tested.
 *
 * `no_location` deliberately covers TWO server-side cases (no location on file
 * for this employee, and a real location this deployment does not serve yet),
 * because the maid's action is the same for both and neither is fixed by
 * retrying. `lookup_unavailable` is the one that IS worth retrying, so its copy
 * says so instead of sending her to an admin over a transient blip.
 *
 * `null`/unknown returns the `no_location` copy: an unrecognised reason from a
 * newer backend must still produce a real, actionable message rather than a
 * blank panel.
 */
export function branchesUnavailableMessage(
  reason: HkBranchesUnavailableReason | null | undefined
): string {
  return reason === 'lookup_unavailable'
    ? LOOKUP_UNAVAILABLE_MESSAGE
    : 'ยังไม่ได้กำหนดสาขาของพนักงาน หรือสาขาของคุณยังไม่เปิดใช้งาน — กรุณาติดต่อผู้ดูแลระบบ'
}

/**
 * fetch() against `GET /api/hk/me` — the ONE `/hk` endpoint that takes NO
 * `?branch=`. It is what tells the client which branches exist, so requiring
 * one here would be circular (§A2). Every other `/hk` call goes through
 * `hkFetch`, which refuses to fire without a branch.
 */
export async function hkFetchMe(init?: RequestInit): Promise<Response> {
  const response = await fetch(`${HK_API_BASE}/me`, init)
  if (response.status === 401) throw new Error(UNAUTHENTICATED_MESSAGE)
  if (response.status === 403) throw new Error(FORBIDDEN_MESSAGE)
  return response
}

/**
 * fetch() against a branch-scoped `/hk` API path. Throws WITHOUT issuing a
 * request when `branch` is `null` — a bug in the caller must never become a
 * wrong-hotel request, only a visible, loud error. Appends `?branch=` or
 * `&branch=` depending on whether `path` already carries a query string.
 * 401/403 keep their existing Thai messages; 400 (branch missing/invalid/
 * disabled, per §A2's required-branch rule) gets its own distinct message.
 */
export async function hkFetch(
  path: string,
  branch: Branch | null,
  init?: RequestInit
): Promise<Response> {
  if (branch === null) {
    throw new Error(NO_BRANCH_MESSAGE)
  }
  const separator = path.includes('?') ? '&' : '?'
  const url = `${HK_API_BASE}${path}${separator}branch=${encodeURIComponent(branch)}`
  const response = await fetch(url, init)
  if (response.status === 401) throw new Error(UNAUTHENTICATED_MESSAGE)
  if (response.status === 403) throw new Error(FORBIDDEN_MESSAGE)
  if (response.status === 400) throw new Error(BRANCH_ERROR_MESSAGE)
  // 503 = the employee-location lookup could not answer (HF ID unreachable or
  // unconfigured). Distinct from 403 on purpose: this one IS worth retrying,
  // so the maid is told to retry rather than that she lacks permission.
  // Unreachable while `HK_LOCATION_ENFORCEMENT_ENABLED` is off.
  if (response.status === 503) throw new Error(LOOKUP_UNAVAILABLE_MESSAGE)
  return response
}

// ---------------------------------------------------------------------------
// Pure display helpers (unit-tested in __tests__/components/hk-lib.test.ts)
// ---------------------------------------------------------------------------

/** Thai label + badge colors for a room's maid-reported progress today. */
export function progressLabel(status: CleaningStatus | null | undefined): {
  label: string
  className: string
} {
  switch (status) {
    case 'started':
      return {
        label: 'กำลังทำความสะอาด',
        className: 'bg-amber-100 text-amber-800 border-amber-300',
      }
    case 'done':
      return {
        label: 'เสร็จแล้ว',
        className: 'bg-emerald-100 text-emerald-800 border-emerald-300',
      }
    case 'dirty':
      return {
        label: 'แจ้งห้องไม่สะอาด',
        className: 'bg-red-100 text-red-800 border-red-300',
      }
    default:
      return {
        label: 'ยังไม่เริ่ม',
        className: 'bg-gray-100 text-gray-600 border-gray-300',
      }
  }
}

/**
 * Explicit clean/dirty chip for a room, from the merged (iHOTEL-wins)
 * `HkRoom.roomClean` — owner feedback (wave-5): "I don't see status from
 * iHOTEL at แม่บ้าน". Before this, a clean room showed NOTHING and a dirty
 * one only a small dot; every room now gets a labelled chip.
 *
 * Uses the EXACT Thai words `lib/v2/status.ts`'s `HK_STATUS_LABELS` gives
 * reception (`.clean` / `.dirty`) — same fact, same word, for both audiences.
 * This is the PRIMARY chip; `progressLabel` (today's maid-reported progress,
 * e.g. "ยังไม่เริ่ม") stays a SECONDARY chip alongside it — dirty +
 * ยังไม่เริ่ม is the ordinary morning state, not a contradiction to resolve.
 */
export function roomCleanChip(roomClean: boolean): { label: string; className: string } {
  return roomClean
    ? { label: HK_STATUS_LABELS.clean, className: 'bg-green-100 text-green-800 border-green-300' }
    : { label: HK_STATUS_LABELS.dirty, className: 'bg-red-100 text-red-800 border-red-300' }
}

/**
 * Header-slot occupancy indicator (มีแขกพัก / ว่าง) — answers "can I enter this
 * room", which is a DIFFERENT question from the clean/dirty chips ("what work
 * is left"). `undefined` (an older backend during deploy skew, before this
 * field existed) renders as `null` — nothing shown, never a guessed value.
 */
export function occupancyIndicator(
  occupancy: Occupancy | null | undefined
): { label: string; className: string } | null {
  if (occupancy === 'occupied') return { label: 'มีแขกพัก', className: 'text-sky-700' }
  if (occupancy === 'vacant') return { label: 'ว่าง', className: 'text-gray-400' }
  return null
}

/**
 * Day-scoped movement tags (แขกออกวันนี้ / แขกเข้าวันนี้) for the tag row
 * between the header and the chip row. Canonical-side booking facts — a
 * DIFFERENT axis from occupancy (right-now) and the clean/dirty chips (what
 * work). Departure is listed first (the day's chronology: someone must leave
 * before the back-to-back arrival can occupy the room). `[]` when both flags
 * are absent or false — old-backend skew and an ordinary room both render
 * nothing, on purpose: there is no third "unknown" tag to show.
 */
export function movementTags(
  room: Pick<HkRoom, 'expectedArrival' | 'expectedDeparture'>
): Array<{ key: 'departure' | 'arrival'; label: string; className: string }> {
  const tags: Array<{ key: 'departure' | 'arrival'; label: string; className: string }> = []
  if (room.expectedDeparture === true) {
    tags.push({
      key: 'departure',
      label: 'แขกออกวันนี้',
      className: 'bg-orange-50 text-orange-700 border-orange-300',
    })
  }
  if (room.expectedArrival === true) {
    tags.push({
      key: 'arrival',
      label: 'แขกเข้าวันนี้',
      className: 'bg-violet-50 text-violet-700 border-violet-300',
    })
  }
  return tags
}

/**
 * Count of rooms whose merged `roomClean` is false — the number a maid
 * actually plans her round by. Surfaced in the list's summary bar alongside
 * เสร็จแล้ว/กำลังทำ/ทั้งหมด. PURE — unit-tested.
 */
export function countRoomsNeedingClean(rooms: HkRoom[]): number {
  return rooms.filter((r) => !r.roomClean).length
}

/**
 * Shown when `legacyStatusStale` is true (CR-1 rule 2): both room-status
 * columns on screen — cleanliness AND occupancy — are the PMS mirror because
 * iHOTEL could not be reached.
 *
 * Deliberately says three things, in the order a maid needs them: WHAT she is
 * looking at (PMS's own status), WHY (iHOTEL unreachable), and the ONE
 * consequence that changes her behaviour (it may disagree with reception's
 * screen, so ask before assuming). It is a NOTICE, not an error — the list
 * beneath it is fully usable and every button still works.
 *
 * Lives here, not on the server, for the same reason `branchesUnavailableMessage`
 * does: the server sends a machine-readable flag and the client owns the Thai.
 */
export const LEGACY_STATUS_STALE_NOTE =
  'ขณะนี้เชื่อมต่อระบบ iHOTEL ไม่ได้ สถานะห้อง (ความสะอาดและการเข้าพัก) ที่แสดงมาจากระบบ PMS ซึ่งอาจไม่ตรงกับหน้าจอแผนกต้อนรับ'

/**
 * The Thai note for a room list / room detail response, or `null` when there
 * is nothing to say. PURE — unit-tested.
 *
 * `undefined` (an older backend that does not send the field) returns `null`:
 * silence must never be read as "stale", or every maid gets a permanent
 * warning banner during a rollback and learns to ignore it.
 */
export function legacyStatusNote(stale: boolean | null | undefined): string | null {
  return stale === true ? LEGACY_STATUS_STALE_NOTE : null
}

/**
 * Confirm copy for แจ้งห้องไม่สะอาด (mark-dirty).
 *
 * The button used to fire on a SINGLE tap, and a mis-tap is not free: it flips
 * the room dirty in iHOTEL, which puts a real room back on reception's board
 * and sends someone to look at it. So the destructive action gets a second
 * tap — and the prompt NAMES THE ROOM, because a maid holding a phone in a
 * corridor of near-identical doors needs to confirm the room, not just the
 * intent. เสร็จแล้ว / เริ่มทำความสะอาด stay single-tap: they are the normal
 * flow, and a confirm on every tap is a confirm nobody reads.
 */
export function markDirtyConfirmMessage(roomNo: string): string {
  return `ยืนยันแจ้งว่า ห้อง ${roomNo} ยังไม่สะอาด?`
}

/** Group rooms by floor for the list screen; floorless rooms go last. */
export function groupRoomsByFloor(rooms: HkRoom[]): Array<{
  floor: number | null
  rooms: HkRoom[]
}> {
  const byFloor = new Map<number | null, HkRoom[]>()
  for (const room of rooms) {
    const key = room.floor ?? null
    const bucket = byFloor.get(key)
    if (bucket) {
      bucket.push(room)
    } else {
      byFloor.set(key, [room])
    }
  }
  const floors = [...byFloor.keys()].sort((a, b) => {
    if (a === null) return 1
    if (b === null) return -1
    return a - b
  })
  return floors.map((floor) => ({
    floor,
    rooms: (byFloor.get(floor) ?? []).sort((a, b) =>
      a.roomNo.localeCompare(b.roomNo, undefined, { numeric: true })
    ),
  }))
}

/** hh:mm (Thai locale) for an ISO timestamp — PG timestamptz values are real
 * instants, so normal local-time rendering applies (the UTC-display rule in
 * CLAUDE.md is for naive LEGACY MSSQL datetimes, which never reach /hk). */
export function timeLabel(iso: string): string {
  const date = new Date(iso)
  if (Number.isNaN(date.getTime())) return ''
  return date.toLocaleTimeString('th-TH', { hour: '2-digit', minute: '2-digit' })
}
