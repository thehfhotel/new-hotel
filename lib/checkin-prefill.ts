'use client'

/**
 * Single-use prefill hand-off for the check-in forms.
 *
 * The Thai-ID card reader (`app/card-reader/page.tsx`) runs as its own page
 * (the read itself happens over the per-PC Tauri middleware). When the
 * receptionist clicks "use this data" we stash the parsed card fields in
 * `sessionStorage` and send them to the rooms screen; the next check-in modal
 * to mount (`CheckInModal` / `QuickCheckInModal`) consumes them to prefill the
 * guest fields, then clears the slot so a stale card can't leak into an
 * unrelated check-in.
 *
 * sessionStorage (not localStorage) is deliberate: the hand-off is transient
 * and scoped to the tab, mirroring the throwaway nature of a card scan.
 */

const KEY = 'checkin-prefill'

export interface CheckInPrefill {
  firstName?: string
  lastName?: string
  idCard?: string
  /** ISO nationality hint (e.g. "ไทย"); not all forms use it. */
  nationality?: string
}

/** Stash a pending prefill (overwrites any previous one). */
export function setCheckInPrefill(data: CheckInPrefill): void {
  if (typeof window === 'undefined') return
  try {
    window.sessionStorage.setItem(KEY, JSON.stringify(data))
  } catch {
    // sessionStorage can throw in private mode / quota — prefill is a
    // convenience, never block the flow.
  }
}

/**
 * Read AND clear the pending prefill. Returns `null` when none is set.
 * Single-use by design: the second caller (or a reload) sees nothing.
 */
export function consumeCheckInPrefill(): CheckInPrefill | null {
  if (typeof window === 'undefined') return null
  try {
    const raw = window.sessionStorage.getItem(KEY)
    if (!raw) return null
    window.sessionStorage.removeItem(KEY)
    const parsed = JSON.parse(raw) as CheckInPrefill
    return parsed && typeof parsed === 'object' ? parsed : null
  } catch {
    return null
  }
}
