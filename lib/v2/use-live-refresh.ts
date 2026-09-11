'use client'

import { useEffect, useRef, useState } from 'react'

/**
 * Subscribe to the server-sent `/api/events` stream for the active branch and
 * call `onRefresh` (debounced 500ms) whenever one of `events` fires — so a
 * change made in iHOTEL (mirrored by the sync → `pg_notify('domain_events')`)
 * or by the other app live-refreshes the screen instead of needing a manual
 * reload. Returns the live-connection flag (for a `<LiveDot>`).
 *
 * Also refetches when the tab regains focus — background tabs throttle/suspend
 * EventSource, and reception staff toggle between iHOTEL and this app, so a
 * change made while our tab was hidden is picked up the moment they switch back.
 *
 * Always ALSO listens for the canonical `refresh` event
 * (`hotel-backend/src/routes/events.rs::RESYNC_EVENT`) regardless of the
 * `events` subset a caller passes in. That's the signal the backend emits
 * after a listener reconnect or a lagged subscriber — "you may have missed
 * something" — so every page must refetch on it, not just the ones that
 * happen to list a matching domain-event name. This is what lets the backend
 * keep that resync burst down to one event name instead of replaying it under
 * every `DomainEvent` variant for back-compat.
 *
 * Encapsulates the EventSource + debounce + cleanup that the rooms/dashboard
 * pages hand-rolled, so the other reception screens can subscribe in one line.
 * `onRefresh` is held in a ref so a new closure each render doesn't tear down +
 * re-open the stream; the subscription only resets when `branch` or the event
 * set changes.
 */
export function useLiveRefresh(
  branch: string,
  events: readonly string[],
  onRefresh: () => void,
): boolean {
  const [live, setLive] = useState(false)
  const onRefreshRef = useRef(onRefresh)
  const timer = useRef<ReturnType<typeof setTimeout> | null>(null)

  useEffect(() => {
    onRefreshRef.current = onRefresh
  }, [onRefresh])

  // Stable string key so a fresh array literal each render doesn't reopen the
  // stream.
  const eventKey = events.join(',')

  useEffect(() => {
    const schedule = () => {
      if (timer.current) clearTimeout(timer.current)
      timer.current = setTimeout(() => onRefreshRef.current(), 500)
    }
    const es = new EventSource(`/api/events?branch=${encodeURIComponent(branch)}`)
    es.onopen = () => setLive(true)
    es.onerror = () => setLive(false)
    const evs = eventKey ? eventKey.split(',') : []
    // De-duped: the canonical resync signal always gets a listener, even if
    // a caller's `events` list happens to include it already.
    const withResync = evs.includes('refresh') ? evs : [...evs, 'refresh']
    withResync.forEach((e) => es.addEventListener(e, schedule))
    const onVis = () => {
      if (typeof document !== 'undefined' && document.visibilityState === 'visible') {
        onRefreshRef.current()
      }
    }
    if (typeof document !== 'undefined') {
      document.addEventListener('visibilitychange', onVis)
    }
    return () => {
      if (typeof document !== 'undefined') {
        document.removeEventListener('visibilitychange', onVis)
      }
      if (timer.current) clearTimeout(timer.current)
      es.close()
      setLive(false)
    }
  }, [branch, eventKey])

  return live
}
