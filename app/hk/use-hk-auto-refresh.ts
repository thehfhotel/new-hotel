'use client'

// Keeping the maid's screen honest over time (wave-5 R2 follow-up).
//
// THE BUG THIS CLOSES. `/hk` and `/hk/rooms/[id]` fetched ONCE — on mount, and
// (on the room screen) again after the maid's own POST. Nothing else ever
// refetched. These pages are opened from a LINE rich-menu tile, and the LINE
// in-app WebView keeps a page alive for hours: a maid opens the list at the
// start of her round, and the rooms reception flips in iHOTEL for the rest of
// the morning NEVER appear. CR-1 merged iHOTEL's truth into the response, but
// nothing on the client ever asked for that response a second time — so from
// the owner's chair the status simply "doesn't sync". Manual pull-to-refresh
// is not an answer: nobody taps a refresh button they have no reason to think
// is needed.
//
// WHY NOT SSE. The reception board (`app/v2/housekeeping/page.tsx`) gets live
// updates from `useLiveRefresh`, which opens `EventSource('/api/events')`.
// That stream sits behind the cookie-session auth middleware, OUTSIDE the
// path-scoped `/hk` Cloudflare Access application that covers this surface
// (see the rewrite comment in next.config.js, and the AuthGuard exemption in
// cbf759a) — a maid's EventSource on it would just 401 forever. Giving `/hk`
// live push means a new `/hk/api/events` endpoint plus Access config; that is
// a real piece of work, not this fix. Polling a already-cheap JSON endpoint is
// the honest minimum that makes the screen true.
//
// WHY THESE NUMBERS.
//  * 60s matches `SAFETY_POLL_MS` on the reception board, so the two screens
//    converge on the same room within the same minute. There it is a belt
//    behind SSE braces; here it is the ONLY mechanism, which argues for
//    keeping it rather than stretching it.
//  * We poll ONLY while the page is actually visible, and refetch immediately
//    when it becomes visible again. That is what keeps it cheap on a phone: a
//    handset in an apron pocket issues zero requests, and the moment the maid
//    looks at it she gets a fresh answer instead of a minute-old one. The
//    interval is really only there for the case where she is staring at the
//    list while reception flips a room.
//  * Each poll costs one `/api/hk/rooms` — which does one small read against
//    legacy (`SELECT Room_no, Room_Clean FROM HT_Rooms`, ~58 rows). A handful
//    of maids at one request per visible minute is far below what a single
//    reception board already does to the same box.

import { useEffect, useRef } from 'react'

/** Visible-only poll cadence. Matches the reception board's safety poll so the
 *  maid and the receptionist converge on a room within the same minute. */
export const HK_POLL_MS = 60_000

/**
 * Collapses bursts of triggers into one request. `visibilitychange` and
 * `focus` both fire when a WebView comes back to the foreground, and an
 * interval tick can land in the same instant — without this, one return to the
 * app would issue three identical fetches.
 */
export const HK_MIN_REFETCH_GAP_MS = 5_000

/**
 * Refetch `refresh` while the page is visible: every `HK_POLL_MS`, and
 * immediately whenever the page becomes visible or regains focus.
 *
 * Deliberately does NOT fire on mount — every caller already does its own
 * first load, and a duplicate opening request is pure cost on a phone.
 *
 * `enabled` is the caller's gate for "not right now": the room screen passes
 * `false` while the maid's own POST is in flight, so a background refetch can
 * never race her report and paint the pre-write answer over it.
 *
 * The callback is held in a ref, so a caller whose `useCallback` identity
 * changes on every render does not restart the timer (and therefore never
 * starves the poll).
 */
export function useHkAutoRefresh(
  refresh: (() => void | Promise<void>) | null,
  enabled: boolean = true
): void {
  const refreshRef = useRef(refresh)
  const enabledRef = useRef(enabled)

  // Seeded at mount so the first poll lands one full interval AFTER the
  // caller's own initial load, not immediately on top of it.
  const lastRunRef = useRef(0)

  // Synced in an effect, not during render (`react-hooks/refs`). Declared
  // BEFORE the setup effect below so that on mount it runs first — effects
  // fire in declaration order, so the timer is never armed with a stale
  // callback.
  useEffect(() => {
    refreshRef.current = refresh
    enabledRef.current = enabled
  })

  useEffect(() => {
    lastRunRef.current = Date.now()

    const run = () => {
      if (!enabledRef.current) return
      const fn = refreshRef.current
      if (!fn) return
      // A hidden page must not poll: this is the whole battery/data story on a
      // phone that lives in a pocket between rooms.
      if (typeof document !== 'undefined' && document.hidden) return
      const now = Date.now()
      if (now - lastRunRef.current < HK_MIN_REFETCH_GAP_MS) return
      lastRunRef.current = now
      void fn()
    }

    const onVisibility = () => {
      // Only the hidden -> visible edge is interesting. Going hidden is
      // handled by `run`'s own guard.
      if (!document.hidden) run()
    }

    const interval = setInterval(run, HK_POLL_MS)
    document.addEventListener('visibilitychange', onVisibility)
    window.addEventListener('focus', run)

    return () => {
      clearInterval(interval)
      document.removeEventListener('visibilitychange', onVisibility)
      window.removeEventListener('focus', run)
    }
  }, [])
}
