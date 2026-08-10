'use client'

import { useEffect } from 'react'
import { useAuth } from '@/contexts/AuthContext'

// Same middleware, same port as the card reader bridge (app/card-reader/page.tsx)
// — reuse the existing env var rather than inventing a second one for the same
// local Tauri tray process.
const MIDDLEWARE_URL = process.env.NEXT_PUBLIC_CARD_READER_URL || 'http://localhost:9898'

/**
 * Browser-mediated transport for the `legacy_stale` notification
 * (`docs/adr/0006-legacy-stale-notification.md`).
 *
 * When one of our writeback jobs commits a row into the shared legacy MSSQL,
 * iHOTEL's own room grid does not know to re-read it (ADR 0006 §Context). The
 * backend already fans that fact out on the existing `/api/events` SSE stream
 * as a `legacy_stale` event (`hotel-backend/src/outbox/legacy_stale.rs`,
 * `routes/events.rs::classify_notification`). This component is the other
 * half: it relays that event, verbatim, to the reception PC's local Tauri
 * tray middleware (the same `127.0.0.1:9898` bridge the card reader already
 * uses) so it can raise a toast. Per ADR 0006 §3, the Tauri process does NOT
 * hold its own SSE connection — this browser tab is the only bridge, which is
 * why this component must be mounted app-wide instead of on a single page.
 *
 * Mounted once in `components/Providers.tsx` so every route — including
 * `/v2`, `/hk` and `/login`, which skip `AppShell`'s `BranchProvider` — still
 * carries the bridge. That statelessness is also why this opens its own
 * `EventSource` on `branch=all` rather than reusing `useLiveRefresh`
 * (`lib/v2/use-live-refresh.ts`): that hook (a) needs a `branch` from
 * `useBranch()`, a context not mounted this high, and (b) collapses every
 * named event AND the canonical `refresh` resync burst into one debounced,
 * payload-discarding callback — exactly the two things this bridge must NOT
 * do (it needs the raw payload, and `refresh` must never be confused with
 * `legacy_stale`, ADR 0006 §2). `branch=all` is `routes/events.rs`'s
 * purpose-built multiplex mode, so one connection covers both sites and this
 * component stays independent of which `BranchProvider` (if any) is mounted
 * above it on a given route.
 *
 * Failures are FULLY SILENT by design (ADR 0006 §3's "honest cost"): no
 * middleware installed is the normal case on most machines (connection
 * refused), and that must cost reception nothing — no toast, no
 * console.error, no retry queue. A single debug-level log is the only trace
 * left behind.
 *
 * Renders nothing.
 */
export default function LegacyStaleBridge(): null {
  // `/api/events` sits behind the auth middleware. Without this gate the
  // bridge would open an EventSource on `/login` too, and the browser's
  // automatic SSE reconnect would hammer a 401 every few seconds for as long
  // as the tab sits logged out. Auth state is available because Providers
  // mounts this inside AuthProvider.
  const { user } = useAuth()

  useEffect(() => {
    if (!user) return
    let unmounted = false
    const es = new EventSource('/api/events?branch=all')

    const onLegacyStale = (event: Event) => {
      if (unmounted) return
      const payload = (event as MessageEvent<string>).data

      // Fire-and-forget — never awaited, never blocks the caller. The
      // middleware being unreachable (no tray app running) is the expected
      // steady state on most machines, so the rejection path is silent by
      // design (ADR 0006 §3): no toast, no console.error, no retry.
      fetch(`${MIDDLEWARE_URL}/notify`, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: payload,
      }).catch((err) => {
        if (process.env.NODE_ENV !== 'production') {
          console.debug('[LegacyStaleBridge] middleware notify failed (tray app likely not running)', err)
        }
      })
    }

    // ONLY `legacy_stale` — deliberately no `refresh` (or any other) listener.
    // `refresh` means "refetch our own UI", fired on SSE listener reconnects
    // and lagged subscribers; it must never fabricate an "iHOTEL is stale"
    // toast (ADR 0006 §2, `legacy_stale.rs` module doc).
    es.addEventListener('legacy_stale', onLegacyStale)

    return () => {
      unmounted = true
      es.removeEventListener('legacy_stale', onLegacyStale)
      es.close()
    }
  }, [user])

  return null
}
