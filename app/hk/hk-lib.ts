// Shared helpers for the maid-facing housekeeping surface (/hk).
//
// The /hk API deliberately lives UNDER /hk (`/hk/api/*`, rewritten by
// next.config.js to the backend's /api/hk/*) so ONE path-scoped Cloudflare
// Access application covers both the pages and their API calls — see the
// rewrite comment in next.config.js. Everything here is plain TypeScript so
// the pure helpers are unit-testable without a DOM.

import { HK_STATUS_LABELS } from '@/lib/v2/status'
import {
  ITEM_PROBLEMS,
  REPORT_ITEMS,
  REPORT_MAX_PHOTOS,
  REPORT_MAX_PHOTOS_TOTAL,
  REPORT_MIN_PHOTOS,
  REPORT_ZONES,
  reportItemLabel,
  RETURN_REASONS,
  ROOM_STATUS_CODES,
  TICK_STATES,
  type ReportStatus,
  type ReturnReason,
  type RoomStatusCode,
  type TickState,
} from './report-vocab'
import {
  DESK_SIGNALS,
  MAID_SIGNALS,
  ROOM_CHECK_PROBLEMS,
  type RoomCheckOutcome,
  type RoomSignal,
  type SignalDirection,
  type SignalType,
} from './signal-vocab'

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
  /** `HK_MARK_DIRTY_ENABLED` — hides the "แจ้งห้องไม่สะอาด" button when false.
   * For a reception-only (viewer) identity the server always sends `false`. */
  markDirtyEnabled: boolean
  /**
   * `true` ⇒ this identity holds the `housekeeping` grant and may FILE reports
   * (cleaning progress, mark-dirty, แจ้งขาดผ้า). `false` ⇒ a `reception`-only
   * identity: a read-only viewer of the same screens.
   *
   * Optional on the type (not on the wire — a current backend always sends it)
   * for deploy skew, and `canReport()` below owns what an absent value means.
   * Do not read this field directly; read it through that helper.
   */
  canReport?: boolean
  /**
   * Set exactly when `branches` is empty; `null` otherwise (and always while
   * location enforcement is off). Always PRESENT — branch on the value, not on
   * the key.
   */
  branchesUnavailableReason: HkBranchesUnavailableReason | null
}

/**
 * May this identity FILE reports, or is it a read-only viewer? PURE — the ONE
 * place the backend-skew rule for `HkMe.canReport` is written down.
 *
 * An ABSENT field means `true`, and the asymmetry is deliberate: `/hk` only
 * ever admitted maids before the `reception` viewer grant existed, so a bundle
 * talking to an older backend that omits the field is, by construction, talking
 * to one where every admitted identity could report. Defaulting to `false`
 * instead would strip the buttons from every maid on the floor for the length
 * of a rollback — a much worse failure than briefly offering a button the
 * server would refuse.
 *
 * `null`/`undefined` (the `/me` call has not answered, or failed) reads the
 * same way for the same reason. This is UX only: the server is the enforcement
 * — a reception-only identity's POST is refused with 403 whatever the UI shows.
 */
export function canReport(me: Pick<HkMe, 'canReport'> | null | undefined): boolean {
  return me?.canReport !== false
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
  /**
   * @deprecated Superseded by `linenShortageOpen` (owner request 2026-09-01).
   *
   * `true` ⇒ at least one linen shortage was reported for this room TODAY
   * (Thai day, the same day scope as `cleaning`). Its meaning is EXACTLY what
   * it always was and the backend still sends it unchanged — it is kept on the
   * wire purely so an older bundle mid-deploy keeps rendering the chip it knows
   * how to render. New code reads `linenShortageOpen`; this field survives only
   * as `linenShortageTag`'s skew fallback.
   */
  linenShortageToday?: boolean
  /**
   * `true` ⇒ this room has at least one OPEN linen-shortage report — filed and
   * not yet marked restocked, of ANY age. This is what ขาดผ้า now MEANS: a
   * shortage is somebody's outstanding work until a maid taps เติมผ้าแล้ว, and
   * completion (not the day rolling over) is what clears it — the same
   * "visible until done" convention the room signals follow (ADR 0008).
   *
   * Sent on the list AND the detail room so the two screens can never disagree.
   * Optional for the same bundle/backend-skew reason as `occupancy`: an older
   * backend omits it, and `linenShortageTag` then falls back to the day-scoped
   * `linenShortageToday` rather than claiming a room is clear of shortages.
   */
  linenShortageOpen?: boolean
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
  /**
   * Today's linen-shortage TOTALS for this room, one row per kind, in
   * `LINEN_KINDS` order; `[]` when nothing was reported today. The detail of
   * the tag `room.linenShortageToday` raises — so a maid who taps into a
   * flagged room sees WHAT is missing, not just THAT something is.
   *
   * Optional for backend skew (an older backend omits it ⇒ no totals line).
   * See `HkLinenShortageTotal` for why `kind` is a plain `string` here.
   */
  linenShortages?: HkLinenShortageTotal[]
  /**
   * The OPEN linen-shortage totals for this room — one row per kind, in the
   * backend's `VALID_LINEN_KINDS` order, `[]` when nothing is outstanding.
   * Same shape as `linenShortages` above, a different QUESTION: that one is
   * "what was reported today" (a record), this one is "what is still owed to
   * this room" (the work). They disagree routinely — a shortage filed
   * yesterday and never restocked is open but not today's, and one filed and
   * restocked this morning is today's but not open.
   *
   * Optional for backend skew, and the fallback is deliberately NOTHING: an
   * older backend omits it and the room screen renders the old day-scoped
   * totals LINE instead of the task card. Guessing the open set from today's
   * totals would put an เติมผ้าแล้ว button in front of a maid for reports the
   * server has no endpoint to resolve.
   */
  linenShortagesOpen?: HkLinenShortageTotal[]
}

// ---------------------------------------------------------------------------
// แจ้งขาดผ้า — linen shortage (`POST /hk/api/rooms/{roomId}/linen-shortage`)
// ---------------------------------------------------------------------------
//
// The maid finds a room short of linen while she is standing in it; the linen
// room needs to know WHAT and HOW MANY, and nothing else. The whole vocabulary
// lives here — wire codes AND their Thai labels — so the room screen never
// spells a `kind` twice and the display order is a single fact rather than a
// coincidence between a list and a switch.

/**
 * The reportable linen kinds, in DISPLAY ORDER (bed linen largest-first, then
 * towels largest to smallest — the order a maid walks a room in). `kind` is the
 * wire code the backend accepts; the Thai label never crosses the wire.
 *
 * This list is the ONLY place a kind is spelled: the wire type (`LinenKind`),
 * the form record (`LinenCounts`), the stepper rows, the request body's order
 * and the totals line all derive from it, so adding a kind here is the whole
 * change — matched, on the wire, by the backend's own allowlist.
 */
export const LINEN_KINDS = [
  { kind: 'bed_sheet', label: 'ผ้าปูที่นอน' },
  { kind: 'pillowcase', label: 'ปลอกหมอน' },
  { kind: 'duvet_cover', label: 'ปลอกผ้านวม' },
  { kind: 'bath_towel', label: 'ผ้าเช็ดตัว' },
  { kind: 'face_towel', label: 'ผ้าเช็ดหน้า' },
  { kind: 'foot_towel', label: 'ผ้าเช็ดเท้า' },
] as const

/** Exactly the `kind` codes `LINEN_KINDS` carries — derived, never re-typed. */
export type LinenKind = (typeof LINEN_KINDS)[number]['kind']

/** Every kind's current count on the form. `0` means "not reported", which is
 * why it is a full record rather than a sparse map: the steppers need a number
 * to render for every row, and only `linenShortageItems` decides what ships. */
export type LinenCounts = Record<LinenKind, number>

/** One row of the request body. Only counts ABOVE zero become items. */
export interface LinenShortageItem {
  kind: LinenKind
  qty: number
}

/**
 * One row of `HkRoomDetail.linenShortages` — today's total for one kind.
 *
 * `kind` is a plain `string`, NOT `LinenKind`, on purpose: this direction is
 * SERVER→client, and a newer backend that ships a sixth kind before this bundle
 * knows about it must render as a readable row, not crash a maid's screen or be
 * silently dropped. `linenKindLabel` falls back to the raw code for exactly
 * that case. The client→server direction stays strictly typed
 * (`LinenShortageItem`), because there we choose what to send.
 */
export interface HkLinenShortageTotal {
  kind: string
  qty: number
}

/** Stepper floor. A row at zero is simply not part of the report. */
export const LINEN_MIN_QTY = 0

/**
 * Stepper ceiling. Nothing about a single room justifies more than this, and a
 * runaway stepper (a stuck thumb on a phone in a pocket) must not turn into a
 * 400-pillowcase errand for the linen room.
 */
export const LINEN_MAX_QTY = 20

/** A fresh, all-zero form. PURE. */
export function emptyLinenCounts(): LinenCounts {
  return Object.fromEntries(LINEN_KINDS.map(({ kind }) => [kind, 0])) as LinenCounts
}

/**
 * Hold a stepper value inside the contract (an integer in
 * `LINEN_MIN_QTY..LINEN_MAX_QTY`). One place, so the − and + buttons cannot
 * disagree with each other or with the server's own bounds. PURE.
 */
export function clampLinenQty(qty: number): number {
  if (!Number.isFinite(qty)) return LINEN_MIN_QTY
  return Math.min(LINEN_MAX_QTY, Math.max(LINEN_MIN_QTY, Math.trunc(qty)))
}

/**
 * The request body's `items` — every kind whose count is ABOVE zero, in
 * `LINEN_KINDS` display order so the report reads the same way the form did.
 * Zero-count rows are DROPPED, not sent as `qty: 0`: the wire says what is
 * missing, never what is fine. PURE.
 */
export function linenShortageItems(counts: LinenCounts): LinenShortageItem[] {
  return LINEN_KINDS.filter(({ kind }) => counts[kind] > 0).map(({ kind }) => ({
    kind,
    qty: counts[kind],
  }))
}

/**
 * `POST /hk/api/rooms/{roomId}/linen-shortage`. The client branches on
 * `success` ALONE — a 200 carrying `success: false` is a failure, and the maid
 * must be told to retry rather than shown a green banner for a report that
 * never landed. `reported` (how many rows the server recorded) is not rendered.
 */
export interface HkLinenShortageResponse {
  success: boolean
  roomId: number
  reported: number
}

/**
 * `POST /hk/api/rooms/{roomId}/linen-shortage/resolve` — เติมผ้าแล้ว.
 *
 * ROOM-LEVEL by design: one tap closes EVERY open report row for the room,
 * whatever kind and whatever day it was filed. A maid restocks a room in one
 * trip, so a per-kind tap would be busywork that leaves half a room "still
 * short" because she forgot the third button.
 *
 * `resolved` is how many rows the server closed, and **zero is a success**:
 * two maids tapping the same room seconds apart, or a retry after a dropped
 * response, must both land on "done", never on an error the second maid has no
 * way to act on.
 */
export interface HkLinenResolveResponse {
  success: boolean
  roomId: number
  resolved: number
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

const LINEN_RESOLVE_ERROR = 'บันทึกไม่สำเร็จ กรุณาลองใหม่'

/**
 * เติมผ้าแล้ว — `POST /hk/api/rooms/{roomId}/linen-shortage/resolve`.
 *
 * NO BODY: the room in the path is the whole request. There is nothing to
 * choose (it resolves every open row for the room) and nothing to stamp (the
 * resolver's identity comes from the verified Cloudflare Access assertion,
 * server-side, exactly as the report's does) — so a body could only ever be a
 * way for a client bug to say something the server would have to refuse.
 *
 * Branches on `success` ALONE, like the report and the signals: a 200 carrying
 * `success: false` is a resolve that did NOT land, and a green banner over it
 * sends a maid away believing a room is stocked. A `resolved: 0` success is
 * still a success — see `HkLinenResolveResponse`. Maid-only; a viewer's POST
 * is refused with 403 (which `hkFetch` turns into the standing Thai message)
 * regardless of what the UI offered.
 */
export async function resolveHkLinenShortage(
  branch: Branch | null,
  roomId: number
): Promise<HkLinenResolveResponse> {
  const res = await hkFetch(`/rooms/${roomId}/linen-shortage/resolve`, branch, {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
  })
  const body: HkLinenResolveResponse | null = res.ok ? await res.json().catch(() => null) : null
  if (!res.ok || !body?.success) throw new Error(LINEN_RESOLVE_ERROR)
  return body
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
 * The ขาดผ้า tag for a room, or `null` when there is nothing to say.
 *
 * Shared by BOTH `/hk` screens so a room cannot be flagged on one and plain on
 * the other. Sky-toned to match the แจ้งขาดผ้า button that files the report —
 * it is the same subject, and deliberately neither the red of a dirty room nor
 * the emerald of a finished one: a linen shortage is a DIFFERENT kind of fact,
 * not a third cleanliness severity.
 *
 * It therefore renders ALONGSIDE every cleaning state, เสร็จแล้ว included —
 * that pairing is the whole point. A room that is finished but still short of
 * linen is not finished, and must not read as finished-and-forgotten.
 *
 * Driven by `linenShortageOpen` (outstanding work, any age), falling back to
 * the deprecated day-scoped `linenShortageToday` ONLY when the open flag is
 * absent — an older backend mid-deploy. The `??` ordering is the whole skew
 * rule: an explicit `false` from a NEW backend means "restocked, nothing to
 * say" and must not be second-guessed by today's record, while an ABSENT flag
 * means the backend cannot answer the new question and the old answer is
 * better than none.
 *
 * `undefined` on both and `false` render nothing: silence is not evidence of a
 * shortage. PURE.
 */
export function linenShortageTag(
  room: Pick<HkRoom, 'linenShortageOpen' | 'linenShortageToday'>
): { label: string; className: string } | null {
  return (room.linenShortageOpen ?? room.linenShortageToday) === true
    ? { label: 'ขาดผ้า', className: 'bg-sky-50 text-sky-800 border-sky-300' }
    : null
}

/**
 * Does this room carry OUTSTANDING linen work? STRICT — no fallback to the
 * day-scoped flag, unlike the chip above.
 *
 * The asymmetry is deliberate and is the whole skew contract for this feature.
 * The chip is decoration on a room that is on screen anyway, so a stale-ish
 * answer beats none. The task panel below is a WORK QUEUE with a completion
 * button behind each row, and an older backend that omits `linenShortageOpen`
 * also has no resolve endpoint — so building a queue out of today's reports
 * would offer a maid a button that 404s. Absent field ⇒ no queue, exactly the
 * behaviour that shipped before. PURE.
 */
export function hasOpenLinenShortage(room: Pick<HkRoom, 'linenShortageOpen'>): boolean {
  return room.linenShortageOpen === true
}

/**
 * The rooms with open linen shortages, in room-number order — the maid's
 * ขาดผ้า queue on the list screen.
 *
 * Sorted the same numeric-aware way `groupRoomsByFloor` sorts within a floor,
 * so the queue reads as a walking order (104, 203, 301) rather than the order
 * the server happened to return. PURE.
 */
export function openLinenRooms(rooms: HkRoom[]): HkRoom[] {
  return rooms
    .filter(hasOpenLinenShortage)
    .sort((a, b) => a.roomNo.localeCompare(b.roomNo, undefined, { numeric: true }))
}

/** The count beside the panel heading ("3 ห้อง"). Its own helper so the panel
 * and any future summary spell the unit once. PURE. */
export function openLinenCountLabel(count: number): string {
  return `${count} ห้อง`
}

/** The room screen's open-shortage card heading. A CONSTANT rather than a
 * literal in the page, because the day-scoped line it replaces
 * ("วันนี้แจ้งขาดผ้า: …") is one word away from it and the two must never be
 * confused: this one is what is STILL owed, that one is what was reported
 * today. */
export const LINEN_OPEN_CARD_TITLE = 'ขาดผ้าค้างอยู่'

/**
 * The open totals as renderable rows — Thai label resolved once, quantity as
 * delivered, in the order DELIVERED (the server already orders them like
 * `LINEN_KINDS`; re-sorting here would invent a second opinion about an order
 * that is already agreed).
 *
 * `null`/`undefined`/`[]` all give `[]`, so the card's "is there anything to
 * show" test is one `length` check. Unknown codes keep their raw code as the
 * label via `linenKindLabel` — a readable row beats a dropped one. PURE.
 */
export function openLinenRows(
  shortages: HkLinenShortageTotal[] | null | undefined
): Array<{ kind: string; label: string; qty: number }> {
  if (!shortages) return []
  return shortages.map(({ kind, qty }) => ({ kind, label: linenKindLabel(kind), qty }))
}

/**
 * Confirm copy for เติมผ้าแล้ว.
 *
 * Earns its second tap for the same reason แจ้งห้องไม่สะอาด does, from the
 * other direction: one tap clears this room's ขาดผ้า flag for EVERYONE —
 * reception's viewer included — and the maid cannot un-resolve it from her
 * phone. The room is NAMED, because a maid in a corridor of near-identical
 * doors needs to confirm the room, not just the intent. PURE.
 */
export function linenResolveConfirmMessage(roomNo: string): string {
  return `ยืนยันว่าเติมผ้าให้ ห้อง ${roomNo} แล้ว?`
}

/** Thai label for a wire `kind` code, falling back to the code itself for a
 * kind a newer backend knows and this bundle does not — a readable row beats a
 * dropped one. PURE. */
export function linenKindLabel(kind: string): string {
  return LINEN_KINDS.find((k) => k.kind === kind)?.label ?? kind
}

/**
 * The one-line TODAY'S-RECORD summary under the แจ้งขาดผ้า button
 * ("วันนี้แจ้งขาดผ้า: ปลอกหมอน 2, ผ้าเช็ดตัว 1"), or `null` when nothing was
 * reported today (and for an older backend that sends no totals at all).
 *
 * Day-scoped and unchanged. It is what the room screen still renders when the
 * backend sends no `linenShortagesOpen` — the OPEN card
 * (`LINEN_OPEN_CARD_TITLE` + `openLinenRows`) is what supersedes it once the
 * backend can answer the outstanding-work question.
 *
 * Rows are rendered in the order DELIVERED — the server already orders them
 * like `LINEN_KINDS`, and re-sorting here would invent a second opinion about
 * an order that is already agreed. PURE. */
export function linenShortageSummary(
  shortages: HkLinenShortageTotal[] | null | undefined
): string | null {
  if (!shortages || shortages.length === 0) return null
  const parts = shortages.map(({ kind, qty }) => `${linenKindLabel(kind)} ${qty}`)
  return `วันนี้แจ้งขาดผ้า: ${parts.join(', ')}`
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

/**
 * Group rooms by floor for the list screen; floorless rooms go last.
 *
 * GENERIC over the row type on purpose: the Report HK day overview
 * (`HkReportRoom`) groups by exactly the same rule and must read as the same
 * screen, and a second copy of this function is how the two lists start
 * disagreeing about where floor 3 ends. The constraint is the only two fields
 * the grouping actually reads.
 */
export function groupRoomsByFloor<T extends { roomNo: string; floor: number | null }>(
  rooms: T[]
): Array<{
  floor: number | null
  rooms: T[]
}> {
  const byFloor = new Map<number | null, T[]>()
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

// ---------------------------------------------------------------------------
// Room signals — ADR 0008 (`docs/adr/0008-room-signals-not-chat.md`)
// ---------------------------------------------------------------------------
//
// A room signal is a CANNED, room-scoped notice between reception and the
// maids — deliberately not a chat: there is no free-text field anywhere in
// this feature, and adding one is the change ADR 0008 exists to refuse. The
// vocabulary itself lives in `signal-vocab.ts` (shared with the reception v2
// surfaces and mirrored by the backend's allowlists); this section owns only
// the /hk client's side of it — the endpoints, and the pure helpers the two
// /hk screens render from.
//
// The vocabulary is RE-EXPORTED here so a /hk page has exactly one import for
// everything about signals, the same way `LINEN_KINDS` sits beside the linen
// helpers above. `signal-vocab.ts` stays the ONE place a type is spelled.

export {
  DESK_SIGNALS,
  MAID_SIGNALS,
  ROOM_CHECK_PROBLEMS,
  signalLabel,
} from './signal-vocab'
export type {
  DeskSignalType,
  MaidSignalType,
  RoomCheckOutcome,
  RoomSignal,
  SignalDirection,
  SignalDoneSource,
  SignalStatus,
  SignalType,
} from './signal-vocab'

/** The one signal type whose completion is an ANSWER, never a bare tap
 * (CONTEXT.md §Housekeeping "Room-check"): the guest is at the counter and the
 * desk needs เคลียร์ / มีของหาย / มีของเสียหาย, not "done". */
export const ROOM_CHECK_TYPE = 'room_check'

/** Exactly the two problems a ขอเช็คห้อง answer may carry. */
export type RoomCheckProblem = (typeof ROOM_CHECK_PROBLEMS)[number]['type']

/**
 * Which side of the conversation this identity is on — derived from the SAME
 * `/me` `canReport` flag that already gates the reporting surface, because the
 * two facts are one fact: a `housekeeping` grant is a maid (sends maid→desk,
 * acts on desk→maid), a `reception` viewer is the desk (the mirror image).
 * There is no third role on `/hk`. PURE.
 */
export type HkSignalRole = 'maid' | 'reception'

export function signalRole(canReportFlag: boolean): HkSignalRole {
  return canReportFlag ? 'maid' : 'reception'
}

/** The direction this role SENDS. PURE. */
export function sentDirection(role: HkSignalRole): SignalDirection {
  return role === 'maid' ? 'maid_to_desk' : 'desk_to_maid'
}

/** The direction this role ACTS ON (acks / completes / answers) — the other
 * side's. "Nobody acts on their own direction's signals except cancel-own-
 * while-open" is the whole rule, and it is written here once. PURE. */
export function actingDirection(role: HkSignalRole): SignalDirection {
  return role === 'maid' ? 'desk_to_maid' : 'maid_to_desk'
}

/** The canned types this role may send, in vocabulary display order. PURE. */
export function sendableSignals(
  role: HkSignalRole
): readonly { type: SignalType; label: string }[] {
  return role === 'maid' ? MAID_SIGNALS : DESK_SIGNALS
}

/** `open` and `acked` — the two statuses the list endpoint returns and the
 * only ones that are still somebody's work. `done`/`cancelled` are terminal
 * and simply leave the screen. PURE. */
export function isLiveSignal(signal: RoomSignal): boolean {
  return signal.status === 'open' || signal.status === 'acked'
}

/** Drop terminal signals, keep the rest. Applied to EVERY list we hold — an
 * SSE event carrying `done` is how a signal disappears from the other role's
 * screen, so a client that kept it would leave a completed room looking busy
 * forever. PURE. */
export function liveSignals(signals: RoomSignal[]): RoomSignal[] {
  return signals.filter(isLiveSignal)
}

/** This room's live signals, oldest first (the order they must be worked).
 * PURE. */
export function signalsForRoom(signals: RoomSignal[], roomId: number): RoomSignal[] {
  return liveSignals(signals)
    .filter((s) => s.roomId === roomId)
    .sort((a, b) => a.createdAt.localeCompare(b.createdAt) || a.signalId - b.signalId)
}

/** How many live signals this room carries — the number the list chip shows.
 * PURE. */
export function openSignalCount(signals: RoomSignal[], roomId: number): number {
  return signalsForRoom(signals, roomId).length
}

/**
 * Live-signal counts for EVERY room in one pass, keyed by `roomId`.
 *
 * The room list renders ~58 cards and would otherwise call `openSignalCount`
 * once per card (a full scan each). One map, built once per render, keeps the
 * list cheap on the phone it actually runs on. PURE.
 */
export function signalCountsByRoom(signals: RoomSignal[]): Map<number, number> {
  const counts = new Map<number, number>()
  for (const signal of liveSignals(signals)) {
    counts.set(signal.roomId, (counts.get(signal.roomId) ?? 0) + 1)
  }
  return counts
}

/**
 * The room's signal chip for the chip row, or `null` when the room has none.
 *
 * DELIBERATELY the only SOLID chip in that row. Everything else there is a
 * pale state badge — clean/dirty, today's progress, ขาดผ้า (sky), the movement
 * tags (orange/violet) — because they describe what a room IS. This one
 * describes work somebody is WAITING on, and at a glance down a two-column
 * grid of near-identical cards it has to be the thing that jumps out. Solid
 * indigo also keeps it clear of every hue already spoken for: red = dirty,
 * emerald = done, amber = in progress, sky = ขาดผ้า, violet/orange = today's
 * movement. PURE.
 */
export function roomSignalChip(count: number): { label: string; className: string } | null {
  if (count <= 0) return null
  return {
    label: `แจ้ง ${count}`,
    className: 'bg-indigo-600 text-white border-indigo-700',
  }
}

/** Where a signal came from, in the reader's own terms. A maid's screen must
 * distinguish "the desk is asking me" from "I told the desk" at a glance —
 * they carry completely different buttons. PURE. */
export function signalOriginLabel(signal: RoomSignal, role: HkSignalRole): string {
  const incoming = signal.direction === actingDirection(role)
  if (role === 'maid') return incoming ? 'จากแผนกต้อนรับ' : 'ส่งถึงแผนกต้อนรับ'
  return incoming ? 'จากแม่บ้าน' : 'ส่งถึงแม่บ้าน'
}

/** Status line for one signal: who has it, or that nobody has yet. PURE. */
export function signalStatusLabel(signal: RoomSignal): string {
  if (signal.status !== 'acked') return 'รอรับเรื่อง'
  const who = signal.ackedBy?.name || signal.ackedBy?.badge
  return who ? `รับเรื่องแล้ว โดย ${who}` : 'รับเรื่องแล้ว'
}

/** Who filed it — name when we have one, badge otherwise (never blank). PURE. */
export function signalActorLabel(
  actor: { badge: string; name: string | null } | null | undefined
): string {
  if (!actor) return ''
  return actor.name || actor.badge
}

/** May this role ack / complete / answer this signal? Only the OTHER side's
 * live signals. PURE — the UI gate; the server enforces the same rule. */
export function canActOnSignal(signal: RoomSignal, role: HkSignalRole): boolean {
  return isLiveSignal(signal) && signal.direction === actingDirection(role)
}

/** May this role cancel it? Own side, still `open` — the single exception to
 * "nobody acts on their own direction". Once the other side has ACKED it, they
 * are already walking to the room; cancelling out from under them is exactly
 * the corridor confusion this feature removes. PURE. */
export function canCancelSignal(signal: RoomSignal, role: HkSignalRole): boolean {
  return signal.status === 'open' && signal.direction === sentDirection(role)
}

/** ขอเช็คห้อง — the signal that may NOT be completed by a bare tap. PURE. */
export function isRoomCheck(signal: RoomSignal): boolean {
  return signal.type === ROOM_CHECK_TYPE
}

/**
 * Is this an arriving signal this role should be ALERTED about (the sound
 * cue)? A brand-new `open` signal pointed AT this role. An ack/done echo of
 * one already on screen, and anything this role sent itself, must stay silent:
 * a cue that fires for one's own taps is a cue people mute. PURE.
 */
export function isIncomingSignal(signal: RoomSignal, role: HkSignalRole): boolean {
  return signal.status === 'open' && signal.direction === actingDirection(role)
}

/**
 * Upsert one signal into a held list — the ONE reducer both the SSE stream and
 * every action response go through, so a signal can never appear twice or
 * linger after it is finished. Terminal signals (`done`/`cancelled`) are
 * REMOVED rather than stored; everything else replaces its predecessor by
 * `signalId`, and the result stays in createdAt order. PURE.
 */
export function mergeSignal(signals: RoomSignal[], incoming: RoomSignal): RoomSignal[] {
  const without = signals.filter((s) => s.signalId !== incoming.signalId)
  if (!isLiveSignal(incoming)) return without
  return [...without, incoming].sort(
    (a, b) => a.createdAt.localeCompare(b.createdAt) || a.signalId - b.signalId
  )
}

/** Several at once (a ขอเช็คห้อง answer spawns one child per problem, in one
 * transaction — they must appear together or not at all). PURE. */
export function mergeSignals(signals: RoomSignal[], incoming: RoomSignal[]): RoomSignal[] {
  return incoming.reduce(mergeSignal, signals)
}

// --- wire shapes -----------------------------------------------------------

/** `GET /hk/api/signals` — this branch's open+acked signals. */
export interface HkSignalsResponse {
  success: boolean
  signals: RoomSignal[]
}

/** `POST /hk/api/rooms/{id}/signals` and `…/signals/{id}/ack|done|cancel`. */
export interface HkSignalResponse {
  success: boolean
  signal: RoomSignal
}

/** `POST /hk/api/signals/{id}/answer` — the answer, plus the standing
 * maid→desk children a `problems` answer spawns (one per problem, same
 * transaction). `spawned` is `[]` for a `clear` answer. */
export interface HkSignalAnswerResponse {
  success: boolean
  signal: RoomSignal
  spawned: RoomSignal[]
}

/** The body of an answer. `clear` carries NO `problems` key — the wire says
 * what was found, never what was not. */
export type RoomCheckAnswer =
  | { outcome: Extract<RoomCheckOutcome, 'clear'> }
  | { outcome: Extract<RoomCheckOutcome, 'problems'>; problems: RoomCheckProblem[] }

/** The three bare-tap transitions. `answer` is deliberately NOT one of them:
 * ขอเช็คห้อง has its own endpoint and its own helper. */
export type HkSignalAction = 'ack' | 'done' | 'cancel'

const SIGNALS_READ_ERROR = 'ไม่สามารถดึงรายการแจ้งได้'
const SIGNAL_WRITE_ERROR = 'ส่งไม่สำเร็จ กรุณาลองใหม่'

/** POST JSON through `hkFetch`, and treat a 200 carrying `success: false` as a
 * failure — the same rule the linen report follows, for the same reason: a
 * green banner over a signal that never landed leaves a guest waiting at a
 * counter for nothing. */
async function postSignalJson<T extends { success: boolean }>(
  path: string,
  branch: Branch | null,
  body: unknown
): Promise<T> {
  const res = await hkFetch(path, branch, {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify(body),
  })
  const parsed: T | null = res.ok ? await res.json().catch(() => null) : null
  if (!res.ok || !parsed?.success) throw new Error(SIGNAL_WRITE_ERROR)
  return parsed
}

/**
 * This branch's live signals. Returns `[]` — never `undefined` — for a backend
 * that answers without a `signals` array, so a page can render the list
 * unconditionally.
 */
export async function fetchHkSignals(branch: Branch | null): Promise<RoomSignal[]> {
  const res = await hkFetch('/signals', branch)
  if (!res.ok) throw new Error(SIGNALS_READ_ERROR)
  const body: HkSignalsResponse | null = await res.json().catch(() => null)
  if (!body?.success) throw new Error(SIGNALS_READ_ERROR)
  return Array.isArray(body.signals) ? liveSignals(body.signals) : []
}

/** Send one canned signal about one room. The DIRECTION is derived server-side
 * from the caller's role — never sent from here, so a client bug cannot file a
 * desk signal in a maid's name. */
export async function sendHkSignal(
  branch: Branch | null,
  roomId: number,
  type: SignalType
): Promise<RoomSignal> {
  const body = await postSignalJson<HkSignalResponse>(`/rooms/${roomId}/signals`, branch, {
    type,
  })
  return body.signal
}

/** รับทราบ / เสร็จสิ้น / ยกเลิก. `done` on a ขอเช็คห้อง is refused by the
 * server (400) — use `answerHkRoomCheck`; the UI never offers the bare tap. */
export async function actOnHkSignal(
  branch: Branch | null,
  signalId: number,
  action: HkSignalAction
): Promise<RoomSignal> {
  const body = await postSignalJson<HkSignalResponse>(
    `/signals/${signalId}/${action}`,
    branch,
    {}
  )
  return body.signal
}

/**
 * Answer a ขอเช็คห้อง — เคลียร์, or one/both problems. The answer COMPLETES the
 * check and, for a `problems` answer, the server spawns one standing maid→desk
 * signal per problem in the SAME transaction; both come back here so the
 * screen can show the desk exactly what it now owes the guest.
 */
export async function answerHkRoomCheck(
  branch: Branch | null,
  signalId: number,
  answer: RoomCheckAnswer
): Promise<{ signal: RoomSignal; spawned: RoomSignal[] }> {
  const body = await postSignalJson<HkSignalAnswerResponse>(
    `/signals/${signalId}/answer`,
    branch,
    answer
  )
  return { signal: body.signal, spawned: Array.isArray(body.spawned) ? body.spawned : [] }
}

// ---------------------------------------------------------------------------
// Signal sound (localStorage) — the mute the cue respects
// ---------------------------------------------------------------------------
//
// Same idiom as the branch above: one key, read defensively, and a storage
// failure degrades to the safe default rather than throwing. The default is
// UNMUTED — a maid holding a silent phone in a corridor is the reason the cue
// exists at all — and the toggle lives on the room list header.

const SOUND_MUTED_KEY = 'hk.signalSoundMuted'

/** `true` ⇒ the cue stays silent. Unknown/unavailable storage reads as `false`
 * (audible), never as muted. */
export function readSignalSoundMuted(): boolean {
  try {
    return localStorage.getItem(SOUND_MUTED_KEY) === '1'
  } catch {
    return false
  }
}

export function storeSignalSoundMuted(muted: boolean): void {
  try {
    localStorage.setItem(SOUND_MUTED_KEY, muted ? '1' : '0')
  } catch {
    /* nothing to do — the toggle simply forgets across reloads */
  }
}

// ---------------------------------------------------------------------------
// Report HK — the daily room report (CONTEXT.md §Housekeeping "Room report")
// ---------------------------------------------------------------------------
//
// One maid's per-room daily attestation, digitizing the owner's paper
// `Report HK.xlsx`: the room's status code (VC/CO/OO/SO — prefilled here from
// facts we already hold, stored as SHE reported it), the 22-item equipment
// checklist as PHOTO-BACKED TICKS, and the photos those ticks name. Reception
// countersigns it with photos of her own, or returns it with a canned reason;
// a returned report is superseded by a FRESH submission that references it
// (append-only history — nothing is ever edited in place).
//
// v2 — THE CAPTURE ZONE STEPPER (owner, 2026-09-02: "1 picture for each tick",
// "fast and easy for a maid working against the clock and physically"). v1's
// exception-only checklist shipped that morning and was superseded the same
// day. What replaced it:
//
//   * `REPORT_ZONES` is the shooting order — เตียง → โต๊ะและมินิบาร์ → ห้องน้ำ →
//     ทั่วไป — and every item belongs to exactly ONE zone. One camera tap each.
//   * The instant a zone's photo is captured, that zone's items are PRE-TICKED
//     ครบ against it. She touches an item only to cycle it ครบ → หาย → ชำรุด
//     (with a quantity), which is why a perfect room is four shots and a
//     handful of taps and never twenty-two decisions.
//   * EVERY tick names the photo that backs it. One photo may back several
//     ticks (the bed shot covers the bed linen); a problem tick may take its
//     own close-up, which the server ALLOWS but does not enforce — the UI
//     drives that, not a 400.
//   * Photos are KEPT FOREVER (owner decision 2026-09-02). There is no purge
//     job here and none is coming.
//
// LEGACY. A report filed before this pair deployed has no tick rows: the server
// answers `ticks: []` and keeps `items` — v1's exceptions array — populated
// from the problem ticks so bundles that predate this one still render. That is
// why `reportItemRows` survives below while the rest of v1's FORM helpers (the
// exception draft, its toggle/stepper, `reportItemsConsistent`) do not: the
// exception-only form is gone, but the reports it filed are permanent.
//
// The vocabulary itself lives in `report-vocab.ts` (mirrored by the backend's
// allowlists) and is RE-EXPORTED here, exactly as the signal vocabulary is
// above, so a page has ONE import for everything about reports. No free text
// anywhere: the return reasons are the whole rejection vocabulary (ADR 0008's
// canned-only discipline, carried over).

export {
  ITEM_PROBLEMS,
  REPORT_ITEMS,
  REPORT_MAX_PHOTOS,
  REPORT_MAX_PHOTOS_TOTAL,
  REPORT_MIN_PHOTOS,
  REPORT_ZONES,
  reportItemLabel,
  RETURN_REASONS,
  ROOM_STATUS_CODES,
  TICK_STATES,
} from './report-vocab'
export type {
  ItemProblem,
  ReportItemCode,
  ReportStatus,
  ReportZone,
  ReturnReason,
  RoomStatusCode,
  TickState,
} from './report-vocab'

// --- wire shapes -----------------------------------------------------------
//
// EVERY field a backend adds after this bundle ships is optional here, and
// several that a current backend always sends are optional too — same skew rule
// the room fields above follow. This surface is opened from a LINE tile whose
// WebView caches a bundle for hours; a screen that throws because one key was
// renamed strands a maid mid-round.

/** Who filed or verified something — name when we have one, badge always. */
export interface HkReportActor {
  badge: string
  name: string | null
}

/**
 * One equipment exception — v1's shape, still carried by `items` on every
 * report so a cached bundle that predates the ticks keeps rendering. `item`
 * and `problem` are plain `string`s in the SERVER→client direction for the same
 * reason `HkLinenShortageTotal.kind` is: a newer backend that knows a 23rd item
 * must render as a readable row here, not crash the screen (`reportItemLabel`
 * falls back to the raw code).
 */
export interface HkReportItemException {
  item: string
  problem: string
  qty: number
}

/**
 * ONE TICK as the server tells it. Loose `string`s again in this direction, and
 * every field but `item` optional: a v1 report has no ticks at all, and a
 * newer backend must be able to add a fourth state without blanking a screen.
 *
 * `photoId` is the photo that BACKS this tick. It is null only on the v1 rows
 * migration 092 backfilled, which had no photo to name.
 */
export interface HkReportTick {
  item: string
  state: string
  qty?: number | null
  photoId?: number | null
}

/** One stored photo with the metadata the verify view groups by. Both sides'
 *  photos arrive in one array — `side` says whose it is. */
export interface HkReportPhotoRef {
  photoId: number
  side: string
  zone?: string | null
  bytes?: number | null
}

/** `GET /hk/api/report-photos/{photoId}/meta` — what the client needs to decide
 *  whether a photo id it stored in a draft is still usable after a reload. */
export interface HkReportPhotoMeta {
  photoId: number
  side: string
  zone?: string | null
  bytes?: number | null
  attached?: boolean
  uploadedAt?: string
}

/** The summary DTO carried by each row of the day overview: the full report
 *  MINUS the tick/photo arrays, PLUS photo COUNTS and the problem count. */
export interface HkReportSummary {
  reportId: number
  roomId: number
  roomNo?: string
  date?: string
  status: ReportStatus
  roomStatus?: string
  /** DERIVED server-side from the ticks (true iff every tick is ok); kept on
   *  the wire for v1 readers. */
  allItemsOk?: boolean
  /** How many ticks are หาย/ชำรุด — the overview's red count. */
  problemCount?: number
  returnReason?: string | null
  parentReportId?: number | null
  submittedBy?: HkReportActor
  submittedAt?: string
  verifiedBy?: HkReportActor | null
  verifiedAt?: string | null
  photoCounts?: { maid: number; reception: number }
}

/** `GET /hk/api/reports/{reportId}` — the summary plus everything the two
 *  detail screens render from. `ticks` + `photos` are v2's evidence; `items`
 *  and the two id arrays stay for v1 readers and legacy reports. */
export interface HkReport extends HkReportSummary {
  ticks?: HkReportTick[]
  photos?: HkReportPhotoRef[]
  items?: HkReportItemException[]
  maidPhotoIds?: number[]
  receptionPhotoIds?: number[]
}

/** One row of the day overview: every active room of the branch, with its
 *  LATEST report for that date (or `null` — the ยังไม่ส่ง case). */
export interface HkReportRoom {
  roomId: number
  roomNo: string
  floor: number | null
  building: string | null
  report: HkReportSummary | null
}

/** `GET /hk/api/reports[?date=YYYY-MM-DD]`. `date` is echoed by the server —
 *  Bangkok's today when the client did not ask for one. */
export interface HkReportsResponse {
  success: boolean
  date: string
  rooms: HkReportRoom[]
}

/** Every write endpoint answers with the report it just wrote. */
export interface HkReportResponse {
  success: boolean
  report: HkReport
}

/** `POST /hk/api/report-photos` (multipart). `bytes` is what the server
 *  actually stored — the number the upload indicator counts. */
export interface HkReportPhotoResponse {
  success: boolean
  photoId: number
  bytes?: number
}

/** What one landed upload gives the form back. */
export interface HkReportPhotoUpload {
  photoId: number
  bytes: number | null
}

/** ONE TICK on the way OUT. Strictly typed, unlike `HkReportTick`: here we
 *  choose what to send, and the server refuses anything else. `qty` is present
 *  on problems ONLY — an `ok` tick carrying a quantity is a 400. */
export interface HkReportTickSubmission {
  item: string
  state: TickState
  qty?: number
  photoId: number
}

/** The body of `POST /hk/api/rooms/{roomId}/report`. Maid-only, v2 shape:
 *  exactly the 22 tick rows, each photo-backed, plus any photo that backs no
 *  tick (an extra shot of the same zone — evidence, not decoration). */
export interface HkReportSubmission {
  roomStatus: RoomStatusCode
  ticks: HkReportTickSubmission[]
  /** Uploaded photos that no tick names. Omitted when there are none. */
  extraPhotoIds?: number[]
  /** Omitted for today, which is the only case this bundle has a screen for. */
  date?: string
  /** Set ONLY when this submission fixes a RETURNED report. Append-only
   *  history: the fix is a new row that points at the one it supersedes. */
  parentReportId?: number
}

// --- labels ----------------------------------------------------------------

const ROOM_STATUS_LABELS: Record<string, string> = Object.fromEntries(
  ROOM_STATUS_CODES.map(({ code, label }) => [code, label])
)
const ITEM_PROBLEM_LABELS: Record<string, string> = Object.fromEntries(
  ITEM_PROBLEMS.map(({ problem, label }) => [problem, label])
)
const RETURN_REASON_LABELS: Record<string, string> = Object.fromEntries(
  RETURN_REASONS.map(({ reason, label }) => [reason, label])
)
const TICK_STATE_LABELS: Record<string, string> = Object.fromEntries(
  TICK_STATES.map(({ state, label }) => [state, label])
)
const ZONE_LABELS: Record<string, string> = Object.fromEntries(
  REPORT_ZONES.map(({ zone, label }) => [zone, label])
)
/** item code → the ONE zone it is shot in. Built from the vocabulary so the
 *  two can never disagree. */
const ITEM_ZONES: Record<string, string> = Object.fromEntries(
  REPORT_ZONES.flatMap(({ zone, items }) => items.map((item) => [item, zone]))
)

/** Thai label for a VC/CO/OO/SO code; the raw code for one this bundle
 *  predates — a readable row beats a dropped one. PURE. */
export function roomStatusLabel(code: string | null | undefined): string {
  if (!code) return ''
  return ROOM_STATUS_LABELS[code] ?? code
}

/** หาย / ชำรุด, falling back to the raw code. PURE. */
export function itemProblemLabel(problem: string | null | undefined): string {
  if (!problem) return ''
  return ITEM_PROBLEM_LABELS[problem] ?? problem
}

/** ครบ / หาย / ชำรุด, falling back to the raw code — a fourth state from a
 *  newer backend renders as itself rather than as a blank cell. PURE. */
export function tickStateLabel(state: string | null | undefined): string {
  if (!state) return ''
  return TICK_STATE_LABELS[state] ?? state
}

/** เตียง / โต๊ะและมินิบาร์ / ห้องน้ำ / ทั่วไป, falling back to the raw code.
 *  PURE. */
export function reportZoneLabel(zone: string | null | undefined): string {
  if (!zone) return ''
  return ZONE_LABELS[zone] ?? zone
}

/** The zone an item is shot in, or null for a code this bundle predates —
 *  which the verify view then groups under อื่น ๆ rather than dropping. PURE. */
export function reportItemZone(item: string): string | null {
  return ITEM_ZONES[item] ?? null
}

/** The item codes of one zone, in the paper form's order. PURE. */
export function reportZoneItems(zone: string): readonly string[] {
  return REPORT_ZONES.find((z) => z.zone === zone)?.items ?? []
}

/** The canned rejection's Thai label, falling back to the raw code. PURE. */
export function returnReasonLabel(reason: string | null | undefined): string {
  if (!reason) return ''
  return RETURN_REASON_LABELS[reason] ?? reason
}

/**
 * The report's date as a Thai reading ("2 ก.ย. 2569"), or the raw string when
 * it cannot be parsed. The wire value is a plain calendar DAY (`YYYY-MM-DD`,
 * Bangkok) rather than an instant, so there is no timezone conversion to get
 * wrong here — and an unparseable value renders as itself rather than as
 * "Invalid Date". PURE.
 */
export function reportDateLabel(date: string | null | undefined): string {
  if (!date) return ''
  const parsed = new Date(`${date}T00:00:00`)
  if (Number.isNaN(parsed.getTime())) return date
  return parsed.toLocaleDateString('th-TH', { day: 'numeric', month: 'short', year: 'numeric' })
}
// --- state, chips and ordering ---------------------------------------------

/** Where a room stands on the day overview. `unsent` is the absence of a
 *  report, which is why this is a separate type from `ReportStatus`. */
export type HkReportState = 'unsent' | 'submitted' | 'verified' | 'returned'

/** The state of one overview row. A report whose `status` this bundle does not
 *  know reads as `submitted` — "somebody has filed something" is the safest
 *  thing to say about an unknown lifecycle value, and it never invites a maid
 *  to file a duplicate. PURE. */
export function reportState(report: HkReportSummary | null | undefined): HkReportState {
  if (!report) return 'unsent'
  if (report.status === 'verified') return 'verified'
  if (report.status === 'returned') return 'returned'
  return 'submitted'
}

/**
 * The overview's state chip. Four states, four Thai phrases, and the RETURN
 * REASON is part of the returned chip's label — a maid scanning her queue has
 * to know WHY a room came back without opening it, and the reason is one of
 * exactly three canned strings.
 *
 * Palette follows the /hk conventions already in this file rather than
 * inventing a fifth vocabulary: gray = nothing yet, amber = in progress
 * (matches กำลังทำความสะอาด), emerald = finished, red = needs work again. PURE.
 */
export function reportStateChip(report: HkReportSummary | null | undefined): {
  state: HkReportState
  label: string
  className: string
} {
  const state = reportState(report)
  switch (state) {
    case 'submitted':
      return {
        state,
        label: 'ส่งแล้ว รอตรวจ',
        className: 'bg-amber-100 text-amber-800 border-amber-300',
      }
    case 'verified':
      return {
        state,
        label: 'ตรวจแล้ว',
        className: 'bg-emerald-100 text-emerald-800 border-emerald-300',
      }
    case 'returned': {
      const reason = returnReasonLabel(report?.returnReason)
      return {
        state,
        label: reason ? `ส่งกลับแก้ไข: ${reason}` : 'ส่งกลับแก้ไข',
        className: 'bg-red-100 text-red-800 border-red-300',
      }
    }
    default:
      return {
        state,
        label: 'ยังไม่ส่ง',
        className: 'bg-gray-100 text-gray-600 border-gray-300',
      }
  }
}

/**
 * Each role's queue order — the SAME rooms, sorted by whose move it is.
 *
 * A maid's queue leads with `returned` (a room she has to walk back to, and
 * the only state that means her work was rejected), then the rooms she has not
 * reported at all; what she has already filed sinks. Reception's leads with
 * `submitted`, which is the only state she can act on at all, then `returned`
 * (she sent it back and is waiting on it), then `unsent`, with `verified` — a
 * finished room for both roles — last in both orders.
 */
export const REPORT_STATE_ORDER: Record<HkSignalRole, readonly HkReportState[]> = {
  maid: ['returned', 'unsent', 'submitted', 'verified'],
  reception: ['submitted', 'returned', 'unsent', 'verified'],
}

/** This row's rank in `role`'s queue — lower sorts first. PURE. */
export function reportRoomPriority(row: HkReportRoom, role: HkSignalRole): number {
  const order = REPORT_STATE_ORDER[role] ?? REPORT_STATE_ORDER.maid
  const index = order.indexOf(reportState(row.report))
  return index === -1 ? order.length : index
}

/**
 * The rooms in `role`'s working order, room number breaking ties the same
 * numeric-aware way every other /hk list sorts (104, 203, 301 — a walking
 * order, not the order the server happened to return). Does not mutate the
 * input. PURE.
 */
export function sortReportRooms(rooms: HkReportRoom[], role: HkSignalRole): HkReportRoom[] {
  return [...rooms].sort(
    (a, b) =>
      reportRoomPriority(a, role) - reportRoomPriority(b, role) ||
      a.roomNo.localeCompare(b.roomNo, undefined, { numeric: true })
  )
}

/** How many rooms sit in each state — the overview's summary bar. PURE. */
export function reportStateCounts(rooms: HkReportRoom[]): Record<HkReportState, number> {
  const counts: Record<HkReportState, number> = {
    unsent: 0,
    submitted: 0,
    verified: 0,
    returned: 0,
  }
  for (const row of rooms) counts[reportState(row.report)] += 1
  return counts
}

// --- room-status prefill ---------------------------------------------------

/**
 * The room-status code the form OPENS on, derived from facts we already hold
 * about the room. The maid may tap any of the four; what she leaves selected is
 * what the backend stores — this only decides which button starts pressed, so a
 * wrong guess costs one tap, never a wrong record.
 *
 * THE MAPPING (in order; the first matching rule wins):
 *
 *   | room facts                                    | code | label            |
 *   |-----------------------------------------------|------|------------------|
 *   | `occupancy === 'occupied'`                    | `so` | พักต่อ            |
 *   | not occupied AND `expectedDeparture === true` | `co` | เช็คเอาท์         |
 *   | anything else (skew included)                 | `vc` | ทำความสะอาดแล้ว   |
 *   | — never derived —                             | `oo` | รอซ่อม            |
 *
 * Why occupancy is read FIRST, ahead of a same-day departure: a guest still in
 * the room when the maid is standing in it is a พักต่อ from her point of view
 * whatever the booking says — a departure the guest has not made yet is not a
 * checkout. Once they have actually gone, `occupancy` flips to vacant and rule
 * 2 turns the same room into CO.
 *
 * Why `roomClean` is deliberately NOT read: the report is filed AFTER the work,
 * so the room's pre-work cleanliness would prefill the wrong answer on exactly
 * the rooms a maid spends her morning on. VC is the resting default instead.
 *
 * Why OO is never derived: nothing on `HkRoom` says "out of order" — แจ้งซ่อม
 * lives in the housekeeping ops app and never reaches this surface — so OO is
 * a judgement the maid makes and taps. Guessing it from a proxy would be
 * inventing a maintenance record. PURE.
 */
export function prefillRoomStatus(
  room: Pick<HkRoom, 'occupancy' | 'expectedDeparture'> | null | undefined
): RoomStatusCode {
  if (room?.occupancy === 'occupied') return 'so'
  if (room?.expectedDeparture === true) return 'co'
  return 'vc'
}

// --- the checklist, as photo-backed ticks ----------------------------------
//
// The maid's working state is TWO plain records, both JSON-serializable on
// purpose (the whole draft is written to sessionStorage after every tap — see
// "the draft" below, and the phone that locks mid-room this exists for):
//
//   `HkTickDraft`  item code → { state, qty, photo }
//   `HkLocalPhoto` one captured shot, from the shutter to its `photoId`
//
// Ticks bind to a photo's LOCAL key, never to its `photoId`: the tick exists
// the instant the shutter closes, and the id arrives seconds later over a
// corridor's worth of wifi. `reportTicksSubmission` is the one place that
// resolves the two, and it refuses to build a body while any binding is still
// unresolved.

/** Quantity floor. A problem tick means at least one is wrong, so the stepper's
 *  bottom is 1 — dropping to zero is done by cycling the tick back to ครบ, not
 *  by stepping into a `qty: 0` the wire has no meaning for. */
export const REPORT_MIN_QTY = 1

/** Quantity ceiling. Mirrors the server's 1..=99 and, like the linen stepper's
 *  own ceiling, keeps a stuck thumb in a pocket from turning into a 400-towel
 *  accusation against a guest. */
export const REPORT_MAX_QTY = 99

/** Hold a stepper value inside the contract. PURE. */
export function clampReportQty(qty: number): number {
  if (!Number.isFinite(qty)) return REPORT_MIN_QTY
  return Math.min(REPORT_MAX_QTY, Math.max(REPORT_MIN_QTY, Math.trunc(qty)))
}

/** One item's tick while the form is open. `photo` is a LOCAL photo key
 *  (`HkLocalPhoto.key`), null while the tick is waiting to be re-backed after
 *  its photo was removed — a state the review step shows and the submit
 *  refuses on. */
export interface HkTickEntry {
  state: TickState
  qty: number | null
  photo: string | null
}

/** The form's whole checklist: item code → its tick. Sparse until a zone is
 *  captured — an item with no entry has not been attested at all, which is
 *  NOT the same as ครบ and is why the body builder never invents one. */
export type HkTickDraft = Record<string, HkTickEntry>

/** Where one captured photo stands. `key` is the client's own identity for the
 *  shot and is what ticks bind to; `photoId` is the server's, and is null until
 *  the upload lands. */
export type HkUploadStatus = 'queued' | 'uploading' | 'uploaded' | 'failed'

export interface HkLocalPhoto {
  key: string
  zone: string
  photoId: number | null
  bytes: number | null
  attempts: number
  status: HkUploadStatus
  /** When the last attempt failed (epoch ms) — the backoff clock. */
  failedAt: number | null
}

/** The client's identity for the n-th shot of a report. Sequential rather than
 *  random so a restored draft and its object URLs line up, and so a test can
 *  name a photo. PURE. */
export function localPhotoKey(seq: number): string {
  return `photo-${seq}`
}

/** A freshly captured shot, before anything has been uploaded. PURE. */
export function newLocalPhoto(key: string, zone: string): HkLocalPhoto {
  return { key, zone, photoId: null, bytes: null, attempts: 0, status: 'queued', failedAt: null }
}

/** One zone's shots, in capture order. PURE. */
export function zonePhotos(photos: HkLocalPhoto[], zone: string): HkLocalPhoto[] {
  return photos.filter((photo) => photo.zone === zone)
}

/** One photo by its local key. PURE. */
export function findLocalPhoto(photos: HkLocalPhoto[], key: string | null): HkLocalPhoto | null {
  if (!key) return null
  return photos.find((photo) => photo.key === key) ?? null
}

/**
 * "รูปที่ 1/2" — which of its zone's shots backs this tick. The chip that makes
 * rebinding discoverable without a menu: a zone with one photo says 1/1 and is
 * not worth tapping; a zone with two says which one vouches for this item.
 * Empty string when the tick has no photo at all. PURE.
 */
export function photoChipLabel(photos: HkLocalPhoto[], key: string | null): string {
  const photo = findLocalPhoto(photos, key)
  if (!photo) return ''
  const siblings = zonePhotos(photos, photo.zone)
  const index = siblings.findIndex((p) => p.key === photo.key)
  if (index === -1) return ''
  return `รูปที่ ${index + 1}/${siblings.length}`
}

/**
 * A zone's photo has landed: PRE-TICK every item of that zone ครบ against it.
 *
 * The whole speed of the flow is here. Two rules keep it from undoing her work:
 * an item she has already decided (any entry with a photo) is left ALONE, and
 * an item whose photo she removed (`photo: null`) is RE-BOUND to this one —
 * a retake repairs the ticks it orphaned instead of making her tick them again.
 * PURE.
 */
export function applyZoneCapture(
  draft: HkTickDraft,
  zone: string,
  photoKey: string
): HkTickDraft {
  const next = { ...draft }
  for (const item of reportZoneItems(zone)) {
    const current = next[item]
    if (!current) {
      next[item] = { state: 'ok', qty: null, photo: photoKey }
    } else if (current.photo === null) {
      next[item] = { ...current, photo: photoKey }
    }
  }
  return next
}

/**
 * One tap on an item cycles it ครบ → หาย → ชำรุด → ครบ.
 *
 * A problem starts at `REPORT_MIN_QTY` and KEEPS its quantity across
 * หาย → ชำรุด (two towels gone and two towels torn is the same two towels she
 * already counted); returning to ครบ drops it, because the wire has no meaning
 * for a quantity of nothing wrong. An item with no entry — a zone she has not
 * shot yet — is not cyclable at all. PURE.
 */
export function cycleTickState(draft: HkTickDraft, item: string): HkTickDraft {
  const current = draft[item]
  if (!current) return draft
  const order = TICK_STATES.map(({ state }) => state)
  const index = order.indexOf(current.state as TickState)
  const state = order[(index === -1 ? 0 : index + 1) % order.length]
  return {
    ...draft,
    [item]: {
      ...current,
      state,
      qty: state === 'ok' ? null : clampReportQty(current.qty ?? REPORT_MIN_QTY),
    },
  }
}

/** Step a PROBLEM tick's quantity, clamped in the reducer rather than only on
 *  the buttons' `disabled` — a bound that exists only as an attribute is one
 *  double-tap away from not existing. An ok tick has no quantity to step. PURE. */
export function stepTickQty(draft: HkTickDraft, item: string, delta: number): HkTickDraft {
  const current = draft[item]
  if (!current || current.state === 'ok') return draft
  return {
    ...draft,
    [item]: { ...current, qty: clampReportQty((current.qty ?? REPORT_MIN_QTY) + delta) },
  }
}

/** Point one tick at a different photo — the close-up she just took, or another
 *  of the zone's shots. An item with no entry is left alone. PURE. */
export function bindTickPhoto(
  draft: HkTickDraft,
  item: string,
  photoKey: string | null
): HkTickDraft {
  const current = draft[item]
  if (!current) return draft
  return { ...draft, [item]: { ...current, photo: photoKey } }
}

/**
 * Point one tick at the NEXT shot of its own zone — the "รูปที่ 1/2" chip's
 * whole behaviour. A tap, not a menu: a zone almost always has one or two
 * photos, and a picker for a two-way choice is a picker a gloved thumb misses.
 * Wraps around, and leaves a tick alone when its zone has nothing else to offer.
 * PURE.
 */
export function cycleTickPhoto(
  draft: HkTickDraft,
  item: string,
  photos: HkLocalPhoto[]
): HkTickDraft {
  const current = draft[item]
  if (!current) return draft
  const zone = reportItemZone(item)
  if (!zone) return draft
  const siblings = zonePhotos(photos, zone)
  if (siblings.length === 0) return draft
  const index = siblings.findIndex((photo) => photo.key === current.photo)
  const next = siblings[(index + 1) % siblings.length]
  return { ...draft, [item]: { ...current, photo: next.key } }
}

/**
 * A photo is going away: UNBIND every tick it backs rather than deleting them.
 *
 * The tick is her judgement about the room and survives the picture; what it
 * loses is its evidence, which the review step then shows as "ต้องถ่ายรูปใหม่"
 * and the submit refuses on. Silently dropping the ticks instead would make a
 * removed photo quietly un-attest five items. PURE.
 */
export function unbindPhotoTicks(draft: HkTickDraft, photoKey: string): HkTickDraft {
  const next: HkTickDraft = {}
  for (const [item, entry] of Object.entries(draft)) {
    next[item] = entry.photo === photoKey ? { ...entry, photo: null } : entry
  }
  return next
}

/** The item codes one photo backs, in the paper form's order — the caption of
 *  the full-screen viewer, and what makes a removal's cost visible. PURE. */
export function ticksBackedBy(draft: HkTickDraft, photoKey: string): string[] {
  return REPORT_ITEMS.map(({ item }) => item).filter((item) => draft[item]?.photo === photoKey)
}

/** How many of a report's ticks are problems. PURE. */
export function draftProblemCount(draft: HkTickDraft): number {
  return Object.values(draft).filter((entry) => entry.state !== 'ok').length
}

/** One zone's standing, for the stepper's dots and the review step. PURE. */
export interface HkZoneProgress {
  zone: string
  label: string
  index: number
  photoCount: number
  itemCount: number
  /** Items with a tick that is actually photo-backed. */
  backedCount: number
  problemCount: number
  /** Ticks whose photo was removed — evidence owed. */
  unbackedCount: number
  /** Every item of the zone attested AND backed. */
  done: boolean
}

/** Every zone's standing, in shooting order. PURE. */
export function reportZoneProgress(
  draft: HkTickDraft,
  photos: HkLocalPhoto[]
): HkZoneProgress[] {
  return REPORT_ZONES.map(({ zone, label, items }, index) => {
    let backedCount = 0
    let problemCount = 0
    let unbackedCount = 0
    for (const item of items) {
      const entry = draft[item]
      if (!entry) continue
      if (entry.state !== 'ok') problemCount += 1
      if (entry.photo) backedCount += 1
      else unbackedCount += 1
    }
    return {
      zone,
      label,
      index,
      photoCount: zonePhotos(photos, zone).length,
      itemCount: items.length,
      backedCount,
      problemCount,
      unbackedCount,
      done: backedCount === items.length,
    }
  })
}

/** One tick on its way to the wire, still allowed to be unbacked — this is
 *  what the REVIEW step renders and what `reportTicksSubmission` refuses on. */
export interface HkReportTickDraft {
  item: string
  label: string
  zone: string
  state: TickState
  qty: number | null
  photo: string | null
  photoId: number | null
}

/**
 * The 22 ticks as the review step reads them: vocabulary order (so the report
 * reads like the paper form however it was tapped), each resolved to the
 * `photoId` of the local photo it names.
 *
 * Items with NO entry are omitted rather than invented as ครบ: a zone she never
 * shot has not been attested, and a body that fabricated "fine" rows would be
 * the one bug on this surface that could cost a guest a charge nobody can
 * explain. `reportTicksSubmission` then refuses a short list. PURE.
 */
export function buildReportTicks(
  draft: HkTickDraft,
  photos: HkLocalPhoto[]
): HkReportTickDraft[] {
  const rows: HkReportTickDraft[] = []
  for (const { item, label } of REPORT_ITEMS) {
    const entry = draft[item]
    if (!entry) continue
    rows.push({
      item,
      label,
      zone: reportItemZone(item) ?? '',
      state: entry.state,
      qty: entry.state === 'ok' ? null : clampReportQty(entry.qty ?? REPORT_MIN_QTY),
      photo: entry.photo,
      photoId: findLocalPhoto(photos, entry.photo)?.photoId ?? null,
    })
  }
  return rows
}

/**
 * The submit body's `ticks`, or NULL when the draft is not fileable.
 *
 * Null — rather than a partial array — because there is exactly one caller and
 * the alternative is a body assembled from `photoId: null`s that the server
 * would refuse after she has walked out of the room. Requires all 22 items,
 * each once, each photo-backed; emits `qty` on problems ONLY. PURE.
 */
export function reportTicksSubmission(
  ticks: HkReportTickDraft[]
): HkReportTickSubmission[] | null {
  if (ticks.length !== REPORT_ITEMS.length) return null
  const seen = new Set<string>()
  const body: HkReportTickSubmission[] = []
  for (const tick of ticks) {
    if (seen.has(tick.item)) return null
    seen.add(tick.item)
    if (tick.photoId === null) return null
    if (tick.state === 'ok') {
      body.push({ item: tick.item, state: 'ok', photoId: tick.photoId })
    } else {
      const qty = clampReportQty(tick.qty ?? REPORT_MIN_QTY)
      body.push({ item: tick.item, state: tick.state, qty, photoId: tick.photoId })
    }
  }
  return body
}

/** Uploaded photos no tick names — an extra shot of a zone, which is evidence
 *  and travels with the report rather than being thrown away. PURE. */
export function reportExtraPhotoIds(
  ticks: HkReportTickDraft[],
  photos: HkLocalPhoto[]
): number[] {
  const named = new Set(ticks.map((tick) => tick.photoId).filter((id): id is number => id !== null))
  const extras: number[] = []
  for (const photo of photos) {
    if (photo.photoId === null || named.has(photo.photoId)) continue
    if (!extras.includes(photo.photoId)) extras.push(photo.photoId)
  }
  return extras
}

/** Every DISTINCT photo the submission would attach (ticks ∪ extras) — the
 *  number the server bounds. PURE. */
export function reportPhotoIds(
  ticks: HkReportTickDraft[],
  extraPhotoIds: number[] = []
): number[] {
  const ids: number[] = []
  for (const id of [...ticks.map((tick) => tick.photoId), ...extraPhotoIds]) {
    if (id !== null && id !== undefined && !ids.includes(id)) ids.push(id)
  }
  return ids
}

/** Rebuild a tick draft from a report — the RETURNED path, where her previous
 *  answers come back so she fixes what was wrong instead of re-deciding
 *  twenty-two rows in a corridor. Photos are DELIBERATELY not carried over
 *  (v1's rule, kept): reception rejected the evidence, and re-sending it is not
 *  a fix, so every tick comes back unbacked and the stepper starts at zone 1.
 *
 *  A legacy v1 report has no ticks — its `items` exceptions are read instead,
 *  and everything it did not name stays UNTICKED (v1 never attested those
 *  individually, and inventing ครบ for them here would be putting words in her
 *  mouth). PURE. */
export function tickDraftFromReport(
  report: Pick<HkReport, 'ticks' | 'items'> | null | undefined
): HkTickDraft {
  const draft: HkTickDraft = {}
  const known = new Set<string>(REPORT_ITEMS.map(({ item }) => item))
  const states = new Set<string>(TICK_STATES.map(({ state }) => state))
  for (const tick of report?.ticks ?? []) {
    if (!known.has(tick.item) || !states.has(tick.state)) continue
    const state = tick.state as TickState
    draft[tick.item] = {
      state,
      qty: state === 'ok' ? null : clampReportQty(tick.qty ?? REPORT_MIN_QTY),
      photo: null,
    }
  }
  if (Object.keys(draft).length > 0) return draft
  for (const { item, problem, qty } of report?.items ?? []) {
    if (!known.has(item) || !states.has(problem)) continue
    draft[item] = { state: problem as TickState, qty: clampReportQty(qty), photo: null }
  }
  return draft
}

// --- the upload queue ------------------------------------------------------
//
// Uploads run BEHIND the maid, never in front of her: the shutter closes, the
// thumbnail appears, the ticks land, and the bytes go up whenever the corridor
// lets them. Everything here is a PURE reducer over `HkLocalPhoto[]` — the
// page owns the timers and the blobs, this owns the rules — because retry
// arithmetic that only exists inside an effect is arithmetic no test can see.

/** How many times one photo is retried before it needs a deliberate tap.
 *  Five attempts spans ~31s of backoff; past that the wifi is not coming back
 *  on its own and a silent loop just eats her battery. */
export const REPORT_UPLOAD_MAX_ATTEMPTS = 5

/** Exponential backoff for attempt `attempts` (1-based), capped. PURE. */
export function uploadBackoffMs(attempts: number): number {
  const n = Math.max(1, Math.trunc(attempts))
  return Math.min(30_000, 1000 * 2 ** (n - 1))
}

/** What the queue does next. `at`/`now` are passed in rather than read from the
 *  clock so the reducer stays pure. */
export type HkUploadAction =
  | { type: 'add'; key: string; zone: string }
  | { type: 'start'; key: string }
  | { type: 'uploaded'; key: string; photoId: number; bytes?: number | null }
  | { type: 'failed'; key: string; at: number }
  | { type: 'resume' }
  | { type: 'remove'; key: string }

/**
 * The queue reducer. Never mutates; an action naming a key it does not hold is
 * a no-op rather than a throw (a settling upload can outlive the photo the
 * maid just removed, and that must not take the screen down with it). PURE.
 */
export function reduceUploadQueue(
  photos: HkLocalPhoto[],
  action: HkUploadAction
): HkLocalPhoto[] {
  switch (action.type) {
    case 'add':
      return photos.some((p) => p.key === action.key)
        ? photos
        : [...photos, newLocalPhoto(action.key, action.zone)]
    case 'start':
      return photos.map((p) =>
        p.key === action.key
          ? { ...p, status: 'uploading', attempts: p.attempts + 1, failedAt: null }
          : p
      )
    case 'uploaded':
      return photos.map((p) =>
        p.key === action.key
          ? {
              ...p,
              status: 'uploaded',
              photoId: action.photoId,
              bytes: action.bytes ?? p.bytes,
              failedAt: null,
            }
          : p
      )
    case 'failed':
      return photos.map((p) =>
        p.key === action.key ? { ...p, status: 'failed', failedAt: action.at } : p
      )
    // The deliberate tap: every stalled photo goes back into the queue with its
    // five attempts restored, eligible IMMEDIATELY rather than after another
    // backoff. "Resumable on next tap" is the whole recovery story for a maid
    // who has walked out of the dead spot, and making her wait 16 more seconds
    // for the retry she just asked for is not that.
    case 'resume':
      return photos.map((p) =>
        p.status === 'failed' ? { ...p, status: 'queued', attempts: 0, failedAt: null } : p
      )
    case 'remove':
      return photos.filter((p) => p.key !== action.key)
    default:
      return photos
  }
}

/**
 * The next photo to send, or null. One at a time — a corridor has one radio,
 * and four parallel 1600px JPEGs is how the whole queue stalls together. A
 * failed photo waits out its backoff; one that has spent all its attempts waits
 * for a `resume`. PURE.
 */
export function nextUploadPhoto(photos: HkLocalPhoto[], now: number): HkLocalPhoto | null {
  if (photos.some((p) => p.status === 'uploading')) return null
  for (const photo of photos) {
    if (photo.status === 'queued') return photo
    if (
      photo.status === 'failed' &&
      photo.attempts < REPORT_UPLOAD_MAX_ATTEMPTS &&
      now - (photo.failedAt ?? 0) >= uploadBackoffMs(photo.attempts)
    ) {
      return photo
    }
  }
  return null
}

/** How long until the queue has something to do again, or null when it is
 *  finished or waiting on a tap. The page's one timer. PURE. */
export function nextUploadWakeMs(photos: HkLocalPhoto[], now: number): number | null {
  if (photos.some((p) => p.status === 'uploading')) return null
  let soonest: number | null = null
  for (const photo of photos) {
    if (photo.status !== 'failed' || photo.attempts >= REPORT_UPLOAD_MAX_ATTEMPTS) continue
    const due = (photo.failedAt ?? 0) + uploadBackoffMs(photo.attempts)
    const wait = Math.max(0, due - now)
    if (soonest === null || wait < soonest) soonest = wait
  }
  return soonest
}

/** The queue at a glance. PURE. */
export function uploadCounts(photos: HkLocalPhoto[]): {
  total: number
  uploaded: number
  pending: number
  stuck: number
} {
  const uploaded = photos.filter((p) => p.status === 'uploaded').length
  const stuck = photos.filter(
    (p) => p.status === 'failed' && p.attempts >= REPORT_UPLOAD_MAX_ATTEMPTS
  ).length
  return { total: photos.length, uploaded, pending: photos.length - uploaded, stuck }
}

/** "อัปโหลดแล้ว 3/5" — the persistent indicator, and the number the blocked
 *  submit quotes back at her so "why can't I send" is never a mystery. PURE. */
export function uploadProgressLabel(photos: HkLocalPhoto[]): string {
  const { uploaded, total } = uploadCounts(photos)
  return `อัปโหลดแล้ว ${uploaded}/${total}`
}

/** Every captured photo has an id. PURE. */
export function uploadsSettled(photos: HkLocalPhoto[]): boolean {
  return photos.every((photo) => photo.photoId !== null)
}

// --- the draft -------------------------------------------------------------
//
// A phone locks mid-room, a LINE WebView is evicted, a maid takes a call: the
// half-filled room must still be there. Every tap writes the draft to
// sessionStorage; the next open reads it back and RECONCILES the photo ids it
// remembers against the server (a photo attached to something else, or gone,
// cannot back a tick any more).
//
// sessionStorage, not local: a draft is about the room she is standing in, and
// one that survived until tomorrow would file yesterday's evidence.

/** Everything the stepper needs to come back exactly as she left it. */
export interface HkReportDraft {
  roomStatus: RoomStatusCode | null
  step: number
  ticks: HkTickDraft
  photos: HkLocalPhoto[]
  /** The local-key counter, so a restored draft never re-issues a key. */
  seq: number
}

/**
 * One key per branch + room + day. The branch is in the key on purpose even
 * though the contract names room+date: room ids are per-property, so 7 is a
 * different room at each hotel, and §A1's rule is that a wrong-hotel anything
 * is a bug we design out rather than one we hope not to hit. PURE.
 */
export function reportDraftKey(branch: Branch | null, roomId: number, date: string): string {
  return `hk.reportDraft.${branch ?? 'none'}.${roomId}.${date}`
}

/** Read a stored draft, or null. Same defensive idiom as the branch storage:
 *  a storage failure, a truncated write or a shape from another bundle degrades
 *  to "no draft" rather than to a screen that throws. */
export function readReportDraft(
  branch: Branch | null,
  roomId: number,
  date: string
): HkReportDraft | null {
  try {
    const raw = sessionStorage.getItem(reportDraftKey(branch, roomId, date))
    if (!raw) return null
    const parsed = JSON.parse(raw) as Partial<HkReportDraft> | null
    if (!parsed || typeof parsed !== 'object') return null
    if (!Array.isArray(parsed.photos) || !parsed.ticks || typeof parsed.ticks !== 'object') {
      return null
    }
    return {
      roomStatus: parsed.roomStatus ?? null,
      step: typeof parsed.step === 'number' ? parsed.step : 0,
      ticks: parsed.ticks as HkTickDraft,
      photos: parsed.photos as HkLocalPhoto[],
      seq: typeof parsed.seq === 'number' ? parsed.seq : parsed.photos.length,
    }
  } catch {
    return null
  }
}

/** Persist the draft. Failure is silent — a full quota must cost her the
 *  RESUME, never the tap she just made. */
export function writeReportDraft(
  branch: Branch | null,
  roomId: number,
  date: string,
  draft: HkReportDraft
): void {
  try {
    sessionStorage.setItem(reportDraftKey(branch, roomId, date), JSON.stringify(draft))
  } catch {
    /* nothing to do — the form simply will not survive a reload */
  }
}

/** Drop the draft — a landed submit, and nothing else. */
export function clearReportDraft(branch: Branch | null, roomId: number, date: string): void {
  try {
    sessionStorage.removeItem(reportDraftKey(branch, roomId, date))
  } catch {
    /* nothing to do */
  }
}

/**
 * Reconcile a restored draft against the photo ids the server still says are
 * hers and UNATTACHED.
 *
 * Two kinds of photo cannot survive a reload and both are dropped here: one
 * that never got an id (its bytes lived only in a page that is gone), and one
 * the server no longer offers (attached to a submission, deleted, or
 * unverifiable). Their ticks are UNBOUND, never deleted — she keeps every
 * judgement she made and owes only the pictures. PURE; the meta lookups happen
 * in the page, which hands the confirmed ids in.
 */
export function reconcileReportDraft(
  draft: HkReportDraft,
  usablePhotoIds: number[]
): HkReportDraft {
  const usable = new Set(usablePhotoIds)
  const kept: HkLocalPhoto[] = []
  const dropped: string[] = []
  for (const photo of draft.photos) {
    if (photo.photoId !== null && usable.has(photo.photoId)) kept.push({ ...photo })
    else dropped.push(photo.key)
  }
  let ticks = draft.ticks
  for (const key of dropped) ticks = unbindPhotoTicks(ticks, key)
  return { ...draft, ticks, photos: kept }
}

// --- validation ------------------------------------------------------------

/** 1..=4 — reception's own evidence, unchanged from v1. PURE. */
export function reportPhotoCountValid(count: number): boolean {
  return count >= REPORT_MIN_PHOTOS && count <= REPORT_MAX_PHOTOS
}

/** The report's photo floor: one per capture zone. Derived from the vocabulary
 *  rather than typed as a 4, so adding a fifth zone moves the floor with it. */
export const REPORT_MIN_PHOTOS_TOTAL = REPORT_ZONES.length

/** The whole report's distinct-photo bound, mirrored from the server so she is
 *  told here rather than after the upload. PURE. */
export function reportPhotoTotalValid(count: number): boolean {
  return count >= REPORT_MIN_PHOTOS_TOTAL && count <= REPORT_MAX_PHOTOS_TOTAL
}

/** Every one of the 22 items ticked, each exactly once, each photo-backed.
 *  PURE. */
export function reportTicksComplete(ticks: HkReportTickDraft[]): boolean {
  return reportTicksSubmission(ticks) !== null
}

/**
 * May the maid's ส่งรายงาน button fire? Every rule the server would refuse on,
 * checked here so she is never told "no" after walking away from the room:
 * a room status from the vocabulary, all 22 ticks photo-backed, and a distinct
 * photo count inside 4..24. PURE.
 */
export function canSubmitReport(draft: {
  roomStatus: string | null
  ticks: HkReportTickDraft[]
  extraPhotoIds?: number[]
}): boolean {
  if (!draft.roomStatus) return false
  if (!ROOM_STATUS_CODES.some(({ code }) => code === draft.roomStatus)) return false
  if (!reportTicksComplete(draft.ticks)) return false
  return reportPhotoTotalValid(reportPhotoIds(draft.ticks, draft.extraPhotoIds ?? []).length)
}

/** A verify needs reception's OWN photos — the two-sided evidence IS the
 *  feature (CONTEXT.md §Housekeeping). PURE. */
export function canVerifyReport(photoIds: number[]): boolean {
  return reportPhotoCountValid(photoIds.length)
}

/** A return needs a reason and NO photos: it is a rejection, not a walk-up.
 *  PURE. */
export function canReturnReport(reason: string | null): boolean {
  return Boolean(reason) && RETURN_REASONS.some(({ reason: r }) => r === reason)
}

/** Who may FILE a report / who may VERIFY one. Two helpers rather than one
 *  negation, because they are two different rules that happen to be opposites
 *  today: a maid never verifies — including one who also holds the reception
 *  grant, which `canReport` already resolves to the maid side. UX only; the
 *  server enforces both. PURE. */
export function canFileReport(role: HkSignalRole): boolean {
  return role === 'maid'
}

export function canVerifyReports(role: HkSignalRole): boolean {
  return role === 'reception'
}

// --- reading a filed report ------------------------------------------------
//
// Two shapes reach these: a v2 report with `ticks` + `photos`, and a legacy v1
// one with neither, whose only record of what was wrong is the `items`
// exceptions array. Both must render — a report is permanent, and the bundle
// that reads it will outlive the bundle that wrote it.

/** One tick as a screen renders it. PURE-built by `reportTickRows`. */
export interface HkReportTickRow {
  key: string
  item: string
  label: string
  zone: string
  zoneLabel: string
  state: string
  stateLabel: string
  qty: number | null
  photoId: number | null
  problem: boolean
}

/** The report's ticks as renderable rows, in the paper form's order — the
 *  server already orders them, but a screen must not depend on that to group
 *  by zone. Unknown item codes keep their raw code and land in the อื่น ๆ
 *  group rather than being dropped. PURE. */
export function reportTickRows(
  ticks: HkReportTick[] | null | undefined
): HkReportTickRow[] {
  const order: string[] = REPORT_ITEMS.map(({ item }) => item)
  return [...(ticks ?? [])]
    .sort((a, b) => {
      const ai = order.indexOf(a.item)
      const bi = order.indexOf(b.item)
      return (ai === -1 ? order.length : ai) - (bi === -1 ? order.length : bi)
    })
    .map((tick) => {
      const zone = reportItemZone(tick.item)
      return {
        key: tick.item,
        item: tick.item,
        label: reportItemLabel(tick.item),
        zone: zone ?? '',
        zoneLabel: zone ? reportZoneLabel(zone) : 'อื่น ๆ',
        state: tick.state,
        stateLabel: tickStateLabel(tick.state),
        qty: tick.qty ?? null,
        photoId: tick.photoId ?? null,
        problem: tick.state !== 'ok',
      }
    })
}

/** The exceptions as renderable rows — v1's renderer, kept because a legacy
 *  report has nothing else. Order as delivered (the server already orders them;
 *  re-sorting would invent a second opinion about an order that is already
 *  agreed). PURE. */
export function reportItemRows(
  items: HkReportItemException[] | null | undefined
): Array<{ key: string; item: string; problem: string; qty: number; label: string; problemLabel: string }> {
  return (items ?? []).map(({ item, problem, qty }) => ({
    key: `${item}:${problem}`,
    item,
    problem,
    qty,
    label: reportItemLabel(item),
    problemLabel: itemProblemLabel(problem),
  }))
}

/** Ticks grouped into the capture zones, in shooting order, with an อื่น ๆ
 *  bucket for codes this bundle predates. Empty groups are dropped. PURE. */
export function reportTicksByZone(
  ticks: HkReportTick[] | null | undefined
): Array<{ zone: string; label: string; ticks: HkReportTickRow[] }> {
  const rows = reportTickRows(ticks)
  const groups: Array<{ zone: string; label: string; ticks: HkReportTickRow[] }> =
    REPORT_ZONES.map(({ zone, label }) => ({
      zone: zone as string,
      label: label as string,
      ticks: rows.filter((row) => row.zone === zone),
    }))
  const others = rows.filter((row) => !row.zone)
  if (others.length > 0) groups.push({ zone: '', label: 'อื่น ๆ', ticks: others })
  return groups.filter((group) => group.ticks.length > 0)
}

/**
 * ONE SIDE's photos grouped by capture zone, each carrying the ticks it backs —
 * the reception verify view's whole layout, computed once here rather than with
 * three nested `filter`s in JSX.
 *
 * A photo with no zone (a v1 photo, or a close-up uploaded before the zone
 * field existed) groups under อื่น ๆ; a photo backing nothing still appears,
 * because an extra shot of a zone is evidence reception is entitled to see.
 * PURE.
 */
export function reportPhotoGroups(
  photos: HkReportPhotoRef[] | null | undefined,
  ticks: HkReportTick[] | null | undefined,
  side: string
): Array<{
  zone: string
  label: string
  photos: Array<{ photoId: number; bytes: number | null; ticks: HkReportTickRow[] }>
}> {
  const rows = reportTickRows(ticks)
  const mine = (photos ?? []).filter((photo) => photo.side === side)
  const build = (zone: string) =>
    mine
      .filter((photo) => (photo.zone ?? '') === zone)
      .map((photo) => ({
        photoId: photo.photoId,
        bytes: photo.bytes ?? null,
        ticks: rows.filter((row) => row.photoId === photo.photoId),
      }))
  const groups: Array<{
    zone: string
    label: string
    photos: Array<{ photoId: number; bytes: number | null; ticks: HkReportTickRow[] }>
  }> = REPORT_ZONES.map(({ zone, label }) => ({
    zone: zone as string,
    label: label as string,
    photos: build(zone),
  }))
  const known = new Set<string>(REPORT_ZONES.map(({ zone }) => zone))
  const others = mine
    .filter((photo) => !known.has(photo.zone ?? ''))
    .map((photo) => ({
      photoId: photo.photoId,
      bytes: photo.bytes ?? null,
      ticks: rows.filter((row) => row.photoId === photo.photoId),
    }))
  if (others.length > 0) groups.push({ zone: '', label: 'อื่น ๆ', photos: others })
  return groups.filter((group) => group.photos.length > 0)
}

/** One side's photo ids, in delivered order — the fallback the read-only
 *  galleries use when a report predates `photos` and carries only the two id
 *  arrays. PURE. */
export function reportSidePhotoIds(report: HkReport | null | undefined, side: string): number[] {
  const fromPhotos = (report?.photos ?? [])
    .filter((photo) => photo.side === side)
    .map((photo) => photo.photoId)
  if (fromPhotos.length > 0) return fromPhotos
  return (side === 'reception' ? report?.receptionPhotoIds : report?.maidPhotoIds) ?? []
}

/**
 * How many things are wrong in this report. The server's derived
 * `problemCount` when it sent one, the ticks when it did not, and v1's
 * exceptions for a legacy report — in that order, so a cached bundle reading a
 * new report and a new bundle reading an old one both get a number. PURE.
 */
export function reportProblemCount(
  report: (HkReportSummary & Partial<HkReport>) | null | undefined
): number {
  if (!report) return 0
  if (typeof report.problemCount === 'number') return report.problemCount
  if (report.ticks?.length) return report.ticks.filter((tick) => tick.state !== 'ok').length
  return report.items?.length ?? 0
}

/**
 * Is this report's room fine? The TICKS decide when there are any, then the
 * rows, and the flag only when there is nothing else — v1's rule kept
 * verbatim, because showing "ครบทุกรายการ" over a list of missing items is the
 * one failure on this surface that could cost a guest a charge nobody can
 * explain. PURE.
 */
export function reportAllOk(report: HkReport | null | undefined): boolean {
  if (!report) return false
  if (report.ticks?.length) return report.ticks.every((tick) => tick.state === 'ok')
  if ((report.items?.length ?? 0) > 0) return false
  return report.allItemsOk !== false
}

// --- photos ----------------------------------------------------------------

/** The longest edge a maid's photo is uploaded at. A phone camera's 12MP JPEG
 *  is ~4MB of nothing useful for "is this room clean"; 1600px is legible on
 *  reception's screen and uploads over hotel wifi in a corridor. */
export const REPORT_PHOTO_MAX_PX = 1600

/** JPEG quality for the downscale. High enough that a stain is still a stain. */
export const REPORT_PHOTO_QUALITY = 0.82

/** The server's own cap, mirrored so an oversized upload is refused HERE with
 *  Thai copy rather than as a bare 413 after a slow upload. */
export const REPORT_PHOTO_MAX_BYTES = 5 * 1024 * 1024

/**
 * The dimensions a photo is drawn at: unchanged when it already fits, scaled
 * by its LONGEST edge otherwise (aspect ratio preserved, both edges at least
 * 1px). Nonsense input (a zero-height image, a NaN from a broken decoder)
 * returns `{0, 0}`, which the caller reads as "don't touch this file". PURE —
 * this is the whole testable part of the downscale.
 */
export function downscaleDimensions(
  width: number,
  height: number,
  max: number = REPORT_PHOTO_MAX_PX
): { width: number; height: number } {
  if (!Number.isFinite(width) || !Number.isFinite(height) || width <= 0 || height <= 0) {
    return { width: 0, height: 0 }
  }
  const longest = Math.max(width, height)
  if (longest <= max) return { width: Math.round(width), height: Math.round(height) }
  const scale = max / longest
  return {
    width: Math.max(1, Math.round(width * scale)),
    height: Math.max(1, Math.round(height * scale)),
  }
}

/**
 * Downscale a captured photo to a ≤`REPORT_PHOTO_MAX_PX` JPEG before it goes
 * up. DOM-dependent by nature (canvas), so the arithmetic lives in
 * `downscaleDimensions` above, which is what the tests own.
 *
 * EVERY failure path returns the ORIGINAL file rather than throwing: no
 * `createImageBitmap` (an ancient WebView, or jsdom), no 2D context, a decoder
 * that gives up, a `toBlob` that hands back nothing. A maid standing in a room
 * she has just cleaned must be able to file her report; a 4MB upload is a slow
 * report, while a thrown error is no report at all — and the server's 5MB cap
 * is still the backstop underneath.
 */
export async function downscalePhoto(
  file: File | Blob,
  max: number = REPORT_PHOTO_MAX_PX
): Promise<Blob> {
  // Checked FIRST, before anything is decoded: this is the one call that is
  // simply absent outside a real browser, and bailing here keeps the fallback
  // silent instead of noisy.
  if (typeof createImageBitmap !== 'function') return file
  try {
    const bitmap = await createImageBitmap(file)
    const { width, height } = downscaleDimensions(bitmap.width, bitmap.height, max)
    if (width === 0 || height === 0) {
      bitmap.close?.()
      return file
    }
    const canvas = document.createElement('canvas')
    canvas.width = width
    canvas.height = height
    const ctx = canvas.getContext('2d')
    if (!ctx) {
      bitmap.close?.()
      return file
    }
    ctx.drawImage(bitmap, 0, 0, width, height)
    bitmap.close?.()
    const blob = await new Promise<Blob | null>((resolve) => {
      canvas.toBlob(resolve, 'image/jpeg', REPORT_PHOTO_QUALITY)
    })
    return blob && blob.size > 0 ? blob : file
  } catch {
    return file
  }
}

/**
 * `<img src>` for one stored photo. Branch-scoped like every other /hk call —
 * `hkFetch` cannot append the query string for us here, because the browser
 * issues this request itself.
 *
 * Returns `''` for a missing branch, and the caller renders nothing: a
 * wrong-hotel image URL is the same class of bug as a wrong-hotel report, and
 * an empty `src` is a bug that shows rather than one that misleads. PURE.
 */
export function hkReportPhotoUrl(photoId: number, branch: Branch | null): string {
  if (!branch) return ''
  return `${HK_API_BASE}/report-photos/${photoId}?branch=${encodeURIComponent(branch)}`
}

// --- fetch helpers ---------------------------------------------------------

const REPORT_READ_ERROR = 'ไม่สามารถดึงรายงานได้ กรุณาลองใหม่'
const REPORT_WRITE_ERROR = 'บันทึกไม่สำเร็จ กรุณาลองใหม่'
const REPORT_CONFLICT_ERROR = 'ห้องนี้ส่งรายงานของวันนี้ไปแล้ว'
const PHOTO_UPLOAD_ERROR = 'อัปโหลดรูปไม่สำเร็จ กรุณาลองใหม่'
const PHOTO_TOO_LARGE_ERROR = 'รูปใหญ่เกินไป กรุณาถ่ายใหม่'

/** Success copy, as constants: the overview and the report screen both show
 *  them, and two spellings of "ส่งรายงานแล้ว" is how a maid learns to distrust
 *  the banner. */
export const REPORT_SUBMITTED_NOTICE = 'ส่งรายงานแล้ว'
export const REPORT_VERIFIED_NOTICE = 'บันทึกแล้ว: ตรวจแล้ว'
export const REPORT_RETURNED_NOTICE = 'ส่งกลับให้แม่บ้านแก้ไขแล้ว'

/** POST JSON through `hkFetch`, treating a 200 that carries `success: false`
 *  as a failure — the standing /hk rule (a green banner over a write that never
 *  landed is worse than an error). 409 gets its own copy: "already reported" is
 *  not a retry, it is an answer. */
async function postReportJson<T extends { success: boolean }>(
  path: string,
  branch: Branch | null,
  body: unknown
): Promise<T> {
  const res = await hkFetch(path, branch, {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify(body),
  })
  if (res.status === 409) throw new Error(REPORT_CONFLICT_ERROR)
  const parsed: T | null = res.ok ? await res.json().catch(() => null) : null
  if (!res.ok || !parsed?.success) throw new Error(REPORT_WRITE_ERROR)
  return parsed
}

/**
 * The day overview: every active room of the branch with its LATEST report for
 * that date. `date` is omitted for today — there is no date picker, and the
 * server's Bangkok "today" is the one answer both roles must be looking at.
 *
 * Returns the server's echoed `date` so the screen renders the day it actually
 * got, never the day it assumed.
 */
export async function fetchHkReports(
  branch: Branch | null,
  date?: string
): Promise<{ date: string; rooms: HkReportRoom[] }> {
  const path = date ? `/reports?date=${encodeURIComponent(date)}` : '/reports'
  const res = await hkFetch(path, branch)
  if (!res.ok) throw new Error(REPORT_READ_ERROR)
  const body: HkReportsResponse | null = await res.json().catch(() => null)
  if (!body?.success) throw new Error(REPORT_READ_ERROR)
  return {
    date: typeof body.date === 'string' ? body.date : '',
    rooms: Array.isArray(body.rooms) ? body.rooms : [],
  }
}

/** One report in full — ticks, photos with their metadata, and the v1 arrays,
 *  none of which the overview's summary DTO carries. */
export async function fetchHkReport(
  branch: Branch | null,
  reportId: number
): Promise<HkReport> {
  const res = await hkFetch(`/reports/${reportId}`, branch)
  if (!res.ok) throw new Error(REPORT_READ_ERROR)
  const body: { success: boolean; report: HkReport } | null = await res.json().catch(() => null)
  if (!body?.success || !body.report) throw new Error(REPORT_READ_ERROR)
  return body.report
}

/**
 * Everything the per-room report screen needs about the DAY: the room's row
 * from the overview and, when it has a report, that report in full.
 *
 * Composed here rather than in the page because there is no "latest report for
 * this room" endpoint — the day list IS that index — and a screen that
 * open-coded the two-step would be one refactor away from rendering a summary
 * DTO's absent `ticks` as "nothing was wrong".
 */
export async function fetchHkRoomReport(
  branch: Branch | null,
  roomId: number,
  date?: string
): Promise<{ date: string; room: HkReportRoom | null; report: HkReport | null }> {
  const { date: day, rooms } = await fetchHkReports(branch, date)
  const room = rooms.find((r) => r.roomId === roomId) ?? null
  const summary = room?.report ?? null
  const report = summary ? await fetchHkReport(branch, summary.reportId) : null
  return { date: day, room, report }
}

/**
 * Upload ONE photo and get its id (and stored size) back. Multipart, field name
 * `photo`, with the capture ZONE as a second text field when the caller knows
 * it — informational on the server, and what lets the verify view group a
 * report's evidence the way it was shot.
 *
 * NO `Content-Type` header — the browser must set it itself so the multipart
 * boundary matches the body. Setting it by hand is the classic way to make
 * every upload fail with a 400 that looks like a server bug.
 *
 * Photos are uploaded as they are taken and ATTACHED later by the submit body,
 * so an abandoned form leaves unattached rows behind. The DELETE below is what
 * a maid's own remove/retake uses; anything she abandons outright stays, and
 * that is deliberate — photos are kept forever (owner decision 2026-09-02).
 */
export async function uploadHkReportPhoto(
  branch: Branch | null,
  photo: Blob,
  options: { zone?: string; filename?: string } = {}
): Promise<HkReportPhotoUpload> {
  if (photo.size > REPORT_PHOTO_MAX_BYTES) throw new Error(PHOTO_TOO_LARGE_ERROR)
  const form = new FormData()
  form.append('photo', photo, options.filename ?? 'photo.jpg')
  if (options.zone) form.append('zone', options.zone)
  const res = await hkFetch('/report-photos', branch, { method: 'POST', body: form })
  if (res.status === 413) throw new Error(PHOTO_TOO_LARGE_ERROR)
  const body: HkReportPhotoResponse | null = res.ok ? await res.json().catch(() => null) : null
  if (!res.ok || !body?.success || typeof body.photoId !== 'number') {
    throw new Error(PHOTO_UPLOAD_ERROR)
  }
  return { photoId: body.photoId, bytes: typeof body.bytes === 'number' ? body.bytes : null }
}

/**
 * Remove one of MY still-unattached photos — the retake primitive.
 *
 * Uploader-only and only while unattached, both enforced server-side; a photo
 * that is already part of a filed report answers 400 and this resolves FALSE
 * rather than throwing. The caller has already dropped it from the form by the
 * time this settles: a maid in a corridor must never wait on a round trip to
 * retake a picture, and an orphaned row is the cheap side of that trade.
 */
export async function deleteHkReportPhoto(
  branch: Branch | null,
  photoId: number
): Promise<boolean> {
  const res = await hkFetch(`/report-photos/${photoId}`, branch, { method: 'DELETE' })
  if (!res.ok) return false
  const body: { success?: boolean } | null = await res.json().catch(() => null)
  return body?.success === true
}

/**
 * One photo's metadata — what a restored draft asks before it trusts a
 * `photoId` it remembers from before the reload.
 *
 * Resolves NULL for every answer that is not a clean one (404, a 400 from
 * another uploader's id, a body without a photo, a network failure). The caller
 * reads null as "cannot be used" and unbinds the ticks it backed: dropping a
 * picture she must retake now is kinder than a submit refused after she has
 * left the room.
 */
export async function fetchHkReportPhotoMeta(
  branch: Branch | null,
  photoId: number
): Promise<HkReportPhotoMeta | null> {
  try {
    const res = await hkFetch(`/report-photos/${photoId}/meta`, branch)
    if (!res.ok) return null
    const body: { success?: boolean; photo?: HkReportPhotoMeta } | null = await res
      .json()
      .catch(() => null)
    if (!body?.success || !body.photo) return null
    return body.photo
  } catch {
    return null
  }
}

/** File a room's daily report. Maid-only; a viewer's POST is refused with 403
 *  whatever the UI offered. */
export async function submitHkReport(
  branch: Branch | null,
  roomId: number,
  submission: HkReportSubmission
): Promise<HkReport> {
  const body = await postReportJson<HkReportResponse>(
    `/rooms/${roomId}/report`,
    branch,
    submission
  )
  return body.report
}

/** Reception's countersignature — submitted → verified, with her OWN photos. */
export async function verifyHkReport(
  branch: Branch | null,
  reportId: number,
  photoIds: number[]
): Promise<HkReport> {
  const body = await postReportJson<HkReportResponse>(`/reports/${reportId}/verify`, branch, {
    photoIds,
  })
  return body.report
}

/** Reception's rejection — submitted → returned, canned reason, no photos. */
export async function returnHkReport(
  branch: Branch | null,
  reportId: number,
  reason: ReturnReason
): Promise<HkReport> {
  const body = await postReportJson<HkReportResponse>(`/reports/${reportId}/return`, branch, {
    reason,
  })
  return body.report
}

// --- the cross-screen success banner ---------------------------------------
//
// A landed submit/verify/return sends the user BACK to the day overview, and
// the confirmation has to survive that navigation. Same defensive idiom as the
// branch storage above — one key, read in a try/catch, a storage failure
// degrades to "no banner" rather than throwing. sessionStorage, not local: a
// banner is about the tap that just happened, and must not reappear tomorrow.

const REPORT_NOTICE_KEY = 'hk.reportNotice'

export function stashHkReportNotice(message: string): void {
  try {
    sessionStorage.setItem(REPORT_NOTICE_KEY, message)
  } catch {
    /* nothing to do — the destination simply renders no banner */
  }
}

/** Read the pending banner and CLEAR it in one move: it is a one-shot, and a
 *  banner that survives a reload is a banner about nothing. */
export function takeHkReportNotice(): string | null {
  try {
    const stored = sessionStorage.getItem(REPORT_NOTICE_KEY)
    if (stored) sessionStorage.removeItem(REPORT_NOTICE_KEY)
    return stored || null
  } catch {
    return null
  }
}
