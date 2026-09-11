/**
 * @jest-environment jsdom
 */

import { render } from '@testing-library/react'
import LegacyStaleBridge from '@/components/LegacyStaleBridge'

// The bridge gates its EventSource on an authenticated user (see the
// component's doc comment — /api/events sits behind auth, and an unauthed
// EventSource would 401-reconnect forever on /login). Default every test to
// a logged-in user; the logged-out test overrides this mock per-case.
const mockUseAuth = jest.fn()
jest.mock('@/contexts/AuthContext', () => ({
  useAuth: () => mockUseAuth(),
}))

/**
 * Minimal `EventSource` stand-in. jsdom doesn't implement `EventSource`, so
 * the component under test would throw on mount without this. Captures every
 * constructed instance so a test can reach in and fire a named event exactly
 * like the browser would call a registered listener.
 */
class MockEventSource {
  static instances: MockEventSource[] = []
  url: string
  closed = false
  private listeners: Record<string, ((event: Event) => void)[]> = {}

  constructor(url: string) {
    this.url = url
    MockEventSource.instances.push(this)
  }

  addEventListener(type: string, listener: (event: Event) => void) {
    ;(this.listeners[type] ??= []).push(listener)
  }

  removeEventListener(type: string, listener: (event: Event) => void) {
    this.listeners[type] = (this.listeners[type] ?? []).filter((l) => l !== listener)
  }

  close() {
    this.closed = true
  }

  /** Test helper: dispatch a named SSE message to every registered listener. */
  emit(type: string, data: string) {
    for (const listener of this.listeners[type] ?? []) {
      listener({ data } as MessageEvent)
    }
  }

  /** Test helper: how many listeners are currently registered for `type`. */
  listenerCount(type: string): number {
    return (this.listeners[type] ?? []).length
  }
}

const SAMPLE_PAYLOAD = {
  id: '11111111-1111-1111-1111-111111111111',
  site: 'hfhotel',
  count: 1,
  summary: 'เช็คอิน ห้อง 302',
  items: ['เช็คอิน ห้อง 302'],
  emitted_at: '2026-08-10T10:00:00Z',
}

describe('LegacyStaleBridge', () => {
  beforeEach(() => {
    MockEventSource.instances = []
    ;(global as unknown as { EventSource: typeof MockEventSource }).EventSource = MockEventSource
    global.fetch = jest.fn().mockResolvedValue({ ok: true })
    mockUseAuth.mockReturnValue({ user: { user_id: 1, username: 'reception' } })
  })

  afterEach(() => {
    jest.restoreAllMocks()
  })

  test('renders nothing', () => {
    const { container } = render(<LegacyStaleBridge />)
    expect(container).toBeEmptyDOMElement()
  })

  test('does not open an EventSource while logged out (no 401 reconnect loop on /login)', () => {
    mockUseAuth.mockReturnValue({ user: null })
    render(<LegacyStaleBridge />)
    expect(MockEventSource.instances).toHaveLength(0)
  })

  test('opens the EventSource once the user logs in', () => {
    mockUseAuth.mockReturnValue({ user: null })
    const { rerender } = render(<LegacyStaleBridge />)
    expect(MockEventSource.instances).toHaveLength(0)
    mockUseAuth.mockReturnValue({ user: { user_id: 1, username: 'reception' } })
    rerender(<LegacyStaleBridge />)
    expect(MockEventSource.instances).toHaveLength(1)
  })

  test('POSTs the payload to the middleware /notify endpoint on a legacy_stale event', () => {
    render(<LegacyStaleBridge />)
    const es = MockEventSource.instances[0]
    expect(es).toBeDefined()

    es.emit('legacy_stale', JSON.stringify(SAMPLE_PAYLOAD))

    expect(global.fetch).toHaveBeenCalledTimes(1)
    const [url, options] = (global.fetch as jest.Mock).mock.calls[0]
    expect(url).toBe('http://localhost:9898/notify')
    expect(options.method).toBe('POST')
    expect(options.headers).toEqual({ 'Content-Type': 'application/json' })
    expect(JSON.parse(options.body)).toEqual(SAMPLE_PAYLOAD)
  })

  test('does NOT POST on a refresh event', () => {
    render(<LegacyStaleBridge />)
    const es = MockEventSource.instances[0]

    // The bridge never even registers a listener for 'refresh' — emitting it
    // must be a pure no-op regardless.
    es.emit('refresh', '{}')

    expect(global.fetch).not.toHaveBeenCalled()
    expect(es.listenerCount('refresh')).toBe(0)
  })

  test('is silent when the middleware fetch rejects (no middleware installed)', async () => {
    global.fetch = jest.fn().mockRejectedValue(new Error('connect ECONNREFUSED 127.0.0.1:9898'))
    const errorSpy = jest.spyOn(console, 'error').mockImplementation(() => {})
    const warnSpy = jest.spyOn(console, 'warn').mockImplementation(() => {})

    render(<LegacyStaleBridge />)
    const es = MockEventSource.instances[0]

    es.emit('legacy_stale', JSON.stringify(SAMPLE_PAYLOAD))

    // Let the rejected fetch promise's .catch() handler run.
    await new Promise((resolve) => setTimeout(resolve, 0))

    expect(errorSpy).not.toHaveBeenCalled()
    expect(warnSpy).not.toHaveBeenCalled()
  })

  test('cleans up the subscription on unmount', () => {
    const { unmount } = render(<LegacyStaleBridge />)
    const es = MockEventSource.instances[0]
    expect(es.closed).toBe(false)
    expect(es.listenerCount('legacy_stale')).toBe(1)

    unmount()

    expect(es.closed).toBe(true)
    expect(es.listenerCount('legacy_stale')).toBe(0)
  })

  test('does not POST for events received after unmount', () => {
    const { unmount } = render(<LegacyStaleBridge />)
    const es = MockEventSource.instances[0]
    unmount()

    // Simulate a straggler event slipping in after teardown (defensive —
    // removeEventListener should already prevent this in a real browser).
    es.emit('legacy_stale', JSON.stringify(SAMPLE_PAYLOAD))

    expect(global.fetch).not.toHaveBeenCalled()
  })
})
