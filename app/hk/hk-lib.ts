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

export type CleaningStatus = 'started' | 'done'

export interface HkMe {
  success: boolean
  badge: string
  displayName: string | null
}

export interface HkCleaningProgress {
  status: CleaningStatus
  badge: string
  name: string | null
  at: string
}

export interface HkRoom {
  roomId: number
  roomNo: string
  floor: number | null
  building: string | null
  roomClean: boolean
  cleaning: HkCleaningProgress | null
  openReports: number
}

export interface HkCleaningEvent {
  eventId: number
  status: CleaningStatus
  badge: string
  name: string | null
  at: string
}

export interface HkBrokenReport {
  reportId: number
  description: string
  badge: string
  name: string | null
  status: 'open' | 'acknowledged' | 'resolved'
  hasPhoto: boolean
  at: string
}

export interface HkRoomDetail {
  success: boolean
  room: HkRoom
  events: HkCleaningEvent[]
  reports: HkBrokenReport[]
}

// ---------------------------------------------------------------------------
// Fetch helper
// ---------------------------------------------------------------------------

/** Base path of the maid API as seen from the browser (Access-scoped). */
export const HK_API_BASE = '/hk/api'

/**
 * fetch() against the /hk API. Throws with a Thai message on 401/403 (the
 * fail-closed identity states) so callers can surface one consistent error.
 */
export async function hkFetch(path: string, init?: RequestInit): Promise<Response> {
  const response = await fetch(`${HK_API_BASE}${path}`, init)
  if (response.status === 401) {
    throw new Error('ไม่สามารถยืนยันตัวตนได้ กรุณาเปิดหน้านี้ผ่านเมนูพนักงานอีกครั้ง')
  }
  if (response.status === 403) {
    throw new Error('บัญชีของคุณไม่มีสิทธิ์ใช้งานระบบแม่บ้าน')
  }
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

// ---------------------------------------------------------------------------
// Photo downscale (browser-only)
// ---------------------------------------------------------------------------

/** Longest edge after downscale — keeps uploads well under the body limit. */
const MAX_PHOTO_EDGE = 1600

/**
 * Downscale a picked photo to a JPEG data payload for the broken-item POST.
 * Returns base64 (no data: prefix) + mime. Falls back to the original bytes
 * when the image APIs are unavailable (very old WebViews).
 */
export async function preparePhoto(
  file: File
): Promise<{ photoBase64: string; photoMime: string }> {
  try {
    const bitmap = await createImageBitmap(file)
    const scale = Math.min(1, MAX_PHOTO_EDGE / Math.max(bitmap.width, bitmap.height))
    const width = Math.max(1, Math.round(bitmap.width * scale))
    const height = Math.max(1, Math.round(bitmap.height * scale))
    const canvas = document.createElement('canvas')
    canvas.width = width
    canvas.height = height
    const ctx = canvas.getContext('2d')
    if (!ctx) throw new Error('no 2d context')
    ctx.drawImage(bitmap, 0, 0, width, height)
    bitmap.close()
    const dataUrl = canvas.toDataURL('image/jpeg', 0.8)
    const base64 = dataUrl.split(',')[1] ?? ''
    if (!base64) throw new Error('empty canvas encode')
    return { photoBase64: base64, photoMime: 'image/jpeg' }
  } catch {
    // Fallback: ship the original file (server caps at 5 MB decoded).
    const buffer = await file.arrayBuffer()
    let binary = ''
    const bytes = new Uint8Array(buffer)
    const chunk = 0x8000
    for (let i = 0; i < bytes.length; i += chunk) {
      binary += String.fromCharCode(...bytes.subarray(i, i + chunk))
    }
    return {
      photoBase64: btoa(binary),
      photoMime: file.type || 'image/jpeg',
    }
  }
}
