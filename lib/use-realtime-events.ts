'use client'

import { useEffect } from 'react'

/**
 * Names of the cache "buckets" any UI component may listen on. These mirror
 * the React-Query / SWR key conventions described in `docs/architecture.md`
 * §3.6e so a future migration can swap the implementation without renaming
 * every listener call site.
 */
export type RealtimeQueryKey =
  | 'bookings'
  | 'checkins'
  | 'customers'
  | 'payments'
  | 'rooms'
  | 'housekeeping'

/**
 * Backend `DomainEvent::type_name()` discriminants (`hotel-backend/src/outbox/event.rs`).
 * Keeping the union literal here rather than importing keeps the frontend
 * bundle Rust-free and surfaces drift as a TS error in the mapping below.
 */
export type DomainEventName =
  | 'BookingCreated'
  | 'BookingModified'
  | 'BookingCancelled'
  | 'CheckInCreated'
  | 'CheckOutCompleted'
  | 'CheckInCancelled'
  | 'CustomerCreated'
  | 'CustomerModified'
  | 'PaymentReceived'
  | 'RoomMarkedClean'
  | 'RoomMarkedDirty'

/**
 * Mapping from each `DomainEvent` variant to the cache buckets it invalidates.
 * Sourced from `docs/architecture.md` §3.6e — keep in sync when new variants
 * land in `hotel-backend/src/outbox/event.rs`.
 */
export const EVENT_TO_QUERY_KEYS: Record<DomainEventName, ReadonlyArray<RealtimeQueryKey>> = {
  BookingCreated: ['bookings', 'rooms'],
  BookingModified: ['bookings', 'rooms'],
  BookingCancelled: ['bookings', 'rooms'],
  CheckInCreated: ['checkins', 'rooms'],
  CheckOutCompleted: ['checkins', 'rooms'],
  CheckInCancelled: ['checkins', 'rooms'],
  CustomerCreated: ['customers'],
  CustomerModified: ['customers'],
  PaymentReceived: ['payments', 'checkins'],
  RoomMarkedClean: ['rooms', 'housekeeping'],
  RoomMarkedDirty: ['rooms', 'housekeeping'],
}

/**
 * Window-level event name that components subscribe to in order to refetch.
 * Detail payload: `{ key: RealtimeQueryKey; eventName: DomainEventName; data?: unknown }`.
 *
 * TODO(arch §3.6e): once the app adopts React Query or SWR, replace the
 * window dispatch below with `queryClient.invalidateQueries({ queryKey: [key] })`
 * (or `mutate(key)` for SWR) and drop this CustomEvent layer. The mapping
 * above is the contract that survives the migration.
 */
export const REALTIME_INVALIDATE_EVENT = 'realtime:invalidate' as const

export interface RealtimeInvalidateDetail {
  key: RealtimeQueryKey
  eventName: DomainEventName
  data?: unknown
}

/**
 * Opens a single SSE connection to `/api/events` (the Axum bridge over PG
 * `LISTEN/NOTIFY domain_events`) and fans `DomainEvent`s out to per-bucket
 * `realtime:invalidate` window events. Components doing list fetches listen
 * with `useRealtimeInvalidate(key, refetch)` to auto-refresh in <100ms across
 * tabs without the user pressing F5.
 *
 * Mount this exactly once at the app root. EventSource auto-reconnects on
 * network blips (built into the spec) — `onerror` only logs.
 */
export function useRealtimeEvents(): void {
  useEffect(() => {
    if (typeof window === 'undefined' || typeof EventSource === 'undefined') {
      return
    }

    const source = new EventSource('/api/events')

    const handlers: Array<{ name: DomainEventName; listener: (event: MessageEvent) => void }> = []

    const eventNames = Object.keys(EVENT_TO_QUERY_KEYS) as DomainEventName[]
    for (const name of eventNames) {
      const listener = (event: MessageEvent) => {
        const parsed = parseEventData(event.data)
        for (const key of EVENT_TO_QUERY_KEYS[name]) {
          dispatchInvalidate(key, name, parsed)
        }
      }
      source.addEventListener(name, listener as EventListener)
      handlers.push({ name, listener })
    }

    source.onerror = (event) => {
      // EventSource auto-reconnects per WHATWG spec; log once and let the
      // browser back off + retry. Closing here would defeat the auto-reconnect.
      console.warn('[useRealtimeEvents] SSE connection error; browser will retry', event)
    }

    return () => {
      for (const { name, listener } of handlers) {
        source.removeEventListener(name, listener as EventListener)
      }
      source.close()
    }
  }, [])
}

/**
 * Subscribe a component to invalidations for a single cache bucket. Pass the
 * function that re-runs the component's data fetch.
 *
 * Example:
 * ```tsx
 * const reload = useCallback(() => fetchBookings().then(setBookings), [])
 * useRealtimeInvalidate('bookings', reload)
 * ```
 */
export function useRealtimeInvalidate(
  key: RealtimeQueryKey,
  onInvalidate: (detail: RealtimeInvalidateDetail) => void,
): void {
  useEffect(() => {
    if (typeof window === 'undefined') {
      return
    }

    const handler = (event: Event) => {
      const detail = (event as CustomEvent<RealtimeInvalidateDetail>).detail
      if (detail && detail.key === key) {
        onInvalidate(detail)
      }
    }

    window.addEventListener(REALTIME_INVALIDATE_EVENT, handler)
    return () => window.removeEventListener(REALTIME_INVALIDATE_EVENT, handler)
  }, [key, onInvalidate])
}

function parseEventData(raw: unknown): unknown {
  if (typeof raw !== 'string' || raw.length === 0) {
    return undefined
  }
  try {
    return JSON.parse(raw)
  } catch {
    return raw
  }
}

function dispatchInvalidate(
  key: RealtimeQueryKey,
  eventName: DomainEventName,
  data: unknown,
): void {
  if (typeof window === 'undefined') {
    return
  }
  const detail: RealtimeInvalidateDetail = { key, eventName, data }
  window.dispatchEvent(new CustomEvent(REALTIME_INVALIDATE_EVENT, { detail }))
}
