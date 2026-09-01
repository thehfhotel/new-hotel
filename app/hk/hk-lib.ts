// Shared helpers for the maid-facing housekeeping surface (/hk).
//
// The /hk API deliberately lives UNDER /hk (`/hk/api/*`, rewritten by
// next.config.js to the backend's /api/hk/*) so ONE path-scoped Cloudflare
// Access application covers both the pages and their API calls — see the
// rewrite comment in next.config.js. Everything here is plain TypeScript so
// the pure helpers are unit-testable without a DOM.

import { HK_STATUS_LABELS } from '@/lib/v2/status'
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
