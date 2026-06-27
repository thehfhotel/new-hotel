/**
 * @jest-environment jsdom
 */

import { act, renderHook } from '@testing-library/react'
import { IDLE_TIMEOUT_MS, useIdleLogout } from '@/contexts/AuthContext'

// next/navigation is imported by the AuthContext module; mock it so importing
// the module under jsdom doesn't reach into Next's router runtime. The hook
// under test never calls these — they're only used by provider components.
jest.mock('next/navigation', () => ({
  useRouter: () => ({ replace: jest.fn(), push: jest.fn() }),
  usePathname: () => '/',
  useSearchParams: () => new URLSearchParams(),
}))

describe('useIdleLogout', () => {
  beforeEach(() => {
    jest.useFakeTimers()
  })

  afterEach(() => {
    jest.runOnlyPendingTimers()
    jest.useRealTimers()
  })

  test('exports a 30-minute default timeout', () => {
    expect(IDLE_TIMEOUT_MS).toBe(30 * 60 * 1000)
  })

  test('does nothing when disabled (auth dark)', () => {
    const onIdle = jest.fn()
    renderHook(() => useIdleLogout({ enabled: false, onIdle, timeoutMs: 1000 }))

    act(() => {
      jest.advanceTimersByTime(10_000)
    })

    expect(onIdle).not.toHaveBeenCalled()
  })

  test('fires onIdle after the timeout when enabled', () => {
    const onIdle = jest.fn()
    renderHook(() => useIdleLogout({ enabled: true, onIdle, timeoutMs: 1000 }))

    expect(onIdle).not.toHaveBeenCalled()
    act(() => {
      jest.advanceTimersByTime(1000)
    })
    expect(onIdle).toHaveBeenCalledTimes(1)
  })

  test('user activity resets the countdown', () => {
    const onIdle = jest.fn()
    renderHook(() => useIdleLogout({ enabled: true, onIdle, timeoutMs: 1000 }))

    // 600ms elapsed — still idle.
    act(() => {
      jest.advanceTimersByTime(600)
    })
    // Activity resets the timer back to a full window.
    act(() => {
      window.dispatchEvent(new Event('keydown'))
    })
    // 600ms after the reset (1200ms total) — should NOT have fired yet.
    act(() => {
      jest.advanceTimersByTime(600)
    })
    expect(onIdle).not.toHaveBeenCalled()
    // Cross the full window measured from the reset.
    act(() => {
      jest.advanceTimersByTime(400)
    })
    expect(onIdle).toHaveBeenCalledTimes(1)
  })

  test('stops firing after unmount', () => {
    const onIdle = jest.fn()
    const { unmount } = renderHook(() =>
      useIdleLogout({ enabled: true, onIdle, timeoutMs: 1000 }),
    )

    unmount()
    act(() => {
      jest.advanceTimersByTime(5000)
    })

    expect(onIdle).not.toHaveBeenCalled()
  })
})
