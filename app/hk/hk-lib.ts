// Shared helpers for the maid-facing housekeeping surface (/hk).
//
// The /hk API deliberately lives UNDER /hk (`/hk/api/*`, rewritten by
// next.config.js to the backend's /api/hk/*) so ONE path-scoped Cloudflare
// Access application covers both the pages and their API calls — see the
// rewrite comment in next.config.js. Everything here is plain TypeScript so
// the pure helpers are unit-testable without a DOM.

// ---------------------------------------------------------------------------
// API types (mirror hotel-backend/src/routes/hk.rs)
// ---------------------------------------------------------------------------

/** The two properties `/hk` may ever serve. Exactly the `?branch=` spelling
 * the backend's `HkPolicy` accepts — never `"all"` (§A2: `write_pool(Some(All))`
 * silently returns the primary pool, which would re-open the wrong-hotel bug
 * this whole surface exists to close). */
export type Branch = 'hfhotel' | 'hfville'

export type CleaningStatus = 'started' | 'done' | 'dirty'

/** One selectable property on the branch picker, served by `GET /api/hk/me`
 * so `HK_BRANCHES` has ONE source of truth (no `NEXT_PUBLIC_*` coupling). */
export interface HkBranchOption {
  id: Branch
  labelTh: string
}

export interface HkMe {
  success: boolean
  badge: string
  displayName: string | null
  /** In `HK_BRANCHES` order. Length 1 ⇒ the client auto-selects, no picker. */
  branches: HkBranchOption[]
  /** `HK_MARK_DIRTY_ENABLED` — hides the "แจ้งห้องไม่สะอาด" button when false. */
  markDirtyEnabled: boolean
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
  roomClean: boolean
  cleaning: HkCleaningProgress | null
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
 * The branch a page should use on first render, BEFORE any tap (§A3):
 *  1. exactly one configured branch ⇒ auto-select it, no picker rendered —
 *     this is the shipping state (HF Hotel only) so maids see zero new UI.
 *  2. more than one AND a still-valid stored choice ⇒ that choice.
 *  3. more than one AND no stored choice, or a stored value that is no
 *     longer in `branches` ⇒ `null` — the caller must render the picker and
 *     block, never fall back to a default.
 */
export function resolveInitialBranch(
  branches: HkBranchOption[],
  stored: Branch | null
): Branch | null {
  if (branches.length === 1) return branches[0].id
  if (stored && branches.some((b) => b.id === stored)) return stored
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
