/**
 * @jest-environment jsdom
 */

import { renderHook, act } from '@testing-library/react'
import {
  EVENT_TO_QUERY_KEYS,
  REALTIME_INVALIDATE_EVENT,
  useRealtimeEvents,
  useRealtimeInvalidate,
  type DomainEventName,
  type RealtimeInvalidateDetail,
} from '@/lib/use-realtime-events'

type Listener = (event: MessageEvent) => void

class FakeEventSource {
  public url: string
  public onerror: ((event: Event) => void) | null = null
  public closed = false
  private listeners = new Map<string, Listener[]>()

  static instances: FakeEventSource[] = []

  constructor(url: string) {
    this.url = url
    FakeEventSource.instances.push(this)
  }

  addEventListener(name: string, listener: Listener): void {
    const bucket = this.listeners.get(name) ?? []
    bucket.push(listener)
    this.listeners.set(name, bucket)
  }

  removeEventListener(name: string, listener: Listener): void {
    const bucket = this.listeners.get(name)
    if (!bucket) return
    this.listeners.set(
      name,
      bucket.filter((existing) => existing !== listener),
    )
  }

  close(): void {
    this.closed = true
  }

  dispatch(name: string, data: unknown): void {
    const bucket = this.listeners.get(name) ?? []
    const event = new MessageEvent(name, { data: typeof data === 'string' ? data : JSON.stringify(data) })
    for (const listener of bucket) {
      listener(event)
    }
  }
}

describe('useRealtimeEvents hook', () => {
  let originalEventSource: typeof EventSource | undefined

  beforeEach(() => {
    FakeEventSource.instances = []
    originalEventSource = (globalThis as { EventSource?: typeof EventSource }).EventSource
    ;(globalThis as unknown as { EventSource: typeof FakeEventSource }).EventSource = FakeEventSource
  })

  afterEach(() => {
    if (originalEventSource) {
      ;(globalThis as { EventSource?: typeof EventSource }).EventSource = originalEventSource
    } else {
      delete (globalThis as { EventSource?: typeof EventSource }).EventSource
    }
  })

  test('opens a single EventSource against /api/events on mount', () => {
    renderHook(() => useRealtimeEvents())

    expect(FakeEventSource.instances).toHaveLength(1)
    expect(FakeEventSource.instances[0]?.url).toBe('/api/events')
  })

  test('closes the EventSource on unmount', () => {
    const { unmount } = renderHook(() => useRealtimeEvents())
    const source = FakeEventSource.instances[0]

    unmount()

    expect(source?.closed).toBe(true)
  })

  test.each(Object.keys(EVENT_TO_QUERY_KEYS) as DomainEventName[])(
    'fans %s out to its mapped query keys via window CustomEvent',
    (eventName) => {
      renderHook(() => useRealtimeEvents())
      const source = FakeEventSource.instances[0]
      const received: RealtimeInvalidateDetail[] = []
      const handler = (event: Event) => {
        received.push((event as CustomEvent<RealtimeInvalidateDetail>).detail)
      }
      window.addEventListener(REALTIME_INVALIDATE_EVENT, handler)

      act(() => {
        source?.dispatch(eventName, { type: eventName, data: { id: 'fixture' } })
      })

      window.removeEventListener(REALTIME_INVALIDATE_EVENT, handler)

      const expectedKeys = EVENT_TO_QUERY_KEYS[eventName]
      expect(received.map((detail) => detail.key)).toEqual([...expectedKeys])
      for (const detail of received) {
        expect(detail.eventName).toBe(eventName)
      }
    },
  )
})

describe('useRealtimeInvalidate hook', () => {
  test('invokes callback only for the matching key', () => {
    const onBookings = jest.fn()
    renderHook(() => useRealtimeInvalidate('bookings', onBookings))

    act(() => {
      window.dispatchEvent(
        new CustomEvent<RealtimeInvalidateDetail>(REALTIME_INVALIDATE_EVENT, {
          detail: { key: 'rooms', eventName: 'BookingCreated' },
        }),
      )
    })
    expect(onBookings).not.toHaveBeenCalled()

    act(() => {
      window.dispatchEvent(
        new CustomEvent<RealtimeInvalidateDetail>(REALTIME_INVALIDATE_EVENT, {
          detail: { key: 'bookings', eventName: 'BookingCreated' },
        }),
      )
    })
    expect(onBookings).toHaveBeenCalledTimes(1)
  })
})
