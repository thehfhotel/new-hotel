'use client'

import { useCallback, useEffect, useMemo, useRef, useState } from 'react'
import { useBranch } from '@/contexts/BranchContext'
import { useBranchFetch } from '@/lib/use-branch-fetch'
import { useLiveRefresh } from '@/lib/v2/use-live-refresh'
import {
  DESK_SIGNALS_ENDPOINT,
  SIGNAL_LIVE_EVENTS,
  deskActionEndpoint,
  deskSendEndpoint,
  sortSignals,
  type DeskSignalAction,
  type RoomSignal,
} from './signal-lib'

/**
 * The reception desk's room-signal client (ADR 0008).
 *
 * Reads `GET /api/housekeeping/signals?branch=` — open + acked only, by
 * contract — and issues the three desk taps plus the desk→maid send. All four
 * writes are canonical PostgreSQL rows that never touch legacy MSSQL.
 *
 * TWO gates, deliberately different from every other v2 write surface:
 *
 * 1. **Not gated on `canWrite`.** The HF Ville write guard exists to stop a
 *    write misrouting into the wrong property's LEGACY pool; a room signal is
 *    a branch-scoped PG row with no writeback recipe and no iHOTEL mirror (the
 *    maid endpoints carry the same ville-guard exemption for the same reason).
 *    Gating it would leave HF Ville's reception unable to answer HF Ville's
 *    maids, which is half of what the feature is for.
 * 2. **Gated on a single branch.** `branch === 'all'` is the aggregate view; a
 *    signal is always about one room at one property, and the endpoints
 *    require a real `?branch=`. In the aggregate view the client reads
 *    nothing and reports `enabled: false` so callers can hide the UI instead
 *    of showing empty panels.
 *
 * `useDeskSignalsCore` does NOT open an event stream. Use it on a surface that
 * already runs `useLiveRefresh` and can fold `SIGNAL_LIVE_EVENTS` into its own
 * event list — one EventSource per tab instead of two. `useDeskSignals` is the
 * same client with its own subscription, for a surface (the checkout modal)
 * that has none.
 */

const LOAD_ERROR = 'ไม่สามารถโหลดสัญญาณห้องพักได้'
const SEND_ERROR = 'ส่งสัญญาณไม่สำเร็จ'
const ACTION_ERROR = 'อัปเดตสัญญาณไม่สำเร็จ'

/** Belt to the SSE braces. Matches the cleaning board's own 60s safety poll —
 *  this only ever covers a missed or dropped stream. */
export const SIGNAL_POLL_MS = 60000

export interface DeskSignalsOptions {
  /** Safety-poll interval in ms; `0` disables the poll entirely. */
  pollMs?: number
}

export interface DeskSignalsClient {
  /** Open + acked signals for the active branch, urgent-first. */
  signals: RoomSignal[]
  /** False in the aggregate (`all`) view — nothing was read, nothing can be sent. */
  enabled: boolean
  loading: boolean
  /** Thai copy for a failed read or write; the last good list stays on screen. */
  error: string | null
  /** The signal id whose tap is in flight, so one row can show its own spinner. */
  busySignalId: number | null
  /** The room id whose send is in flight. */
  sendingRoomId: number | null
  refresh: () => Promise<void>
  /** Fire one desk→maid signal. Resolves to the created DTO, or null on failure. */
  send: (roomId: number, type: string) => Promise<RoomSignal | null>
  /** รับทราบ / เสร็จสิ้น / ยกเลิก. Resolves true when the server accepted it. */
  act: (signalId: number, action: DeskSignalAction) => Promise<boolean>
}

export function useDeskSignalsCore(options: DeskSignalsOptions = {}): DeskSignalsClient {
  const { pollMs = SIGNAL_POLL_MS } = options
  const { branch } = useBranch()
  const branchFetch = useBranchFetch()

  const enabled = branch !== 'all'

  const [signals, setSignals] = useState<RoomSignal[]>([])
  const [loading, setLoading] = useState(enabled)
  const [error, setError] = useState<string | null>(null)
  const [busySignalId, setBusySignalId] = useState<number | null>(null)
  const [sendingRoomId, setSendingRoomId] = useState<number | null>(null)

  // Latest-wins guard: the branch can flip mid-flight (default hfhotel → the
  // stored hfville), and a late response from the old branch must not paint
  // the wrong property's signals onto the screen.
  const reqRef = useRef(0)

  const refresh = useCallback(async () => {
    if (!enabled) {
      setSignals([])
      setLoading(false)
      return
    }
    const token = ++reqRef.current
    try {
      const response = await branchFetch(DESK_SIGNALS_ENDPOINT)
      if (token !== reqRef.current) return
      if (!response.ok) throw new Error(LOAD_ERROR)
      const payload = await response.json()
      if (token !== reqRef.current) return
      if (!payload?.success) throw new Error(LOAD_ERROR)
      setSignals(Array.isArray(payload.signals) ? payload.signals : [])
      setError(null)
    } catch {
      // The previous list is deliberately LEFT ON SCREEN (the /hk rule): a
      // stale list reception can still read beats a blank one.
      if (token === reqRef.current) setError(LOAD_ERROR)
    } finally {
      if (token === reqRef.current) setLoading(false)
    }
  }, [branchFetch, enabled])

  useEffect(() => {
    refresh()
    if (!enabled || !pollMs) return
    const interval = setInterval(refresh, pollMs)
    return () => clearInterval(interval)
  }, [refresh, enabled, pollMs])

  const send = useCallback(
    async (roomId: number, type: string): Promise<RoomSignal | null> => {
      if (!enabled) return null
      setSendingRoomId(roomId)
      setError(null)
      try {
        const response = await branchFetch(deskSendEndpoint(roomId), {
          method: 'POST',
          headers: { 'Content-Type': 'application/json' },
          body: JSON.stringify({ type }),
        })
        const payload = await response.json().catch(() => null)
        if (!response.ok || !payload?.success) throw new Error(SEND_ERROR)
        await refresh()
        return (payload.signal ?? null) as RoomSignal | null
      } catch {
        setError(SEND_ERROR)
        return null
      } finally {
        setSendingRoomId(null)
      }
    },
    [branchFetch, enabled, refresh],
  )

  const act = useCallback(
    async (signalId: number, action: DeskSignalAction): Promise<boolean> => {
      if (!enabled) return false
      setBusySignalId(signalId)
      setError(null)
      try {
        const response = await branchFetch(deskActionEndpoint(signalId, action), {
          method: 'POST',
          headers: { 'Content-Type': 'application/json' },
          body: JSON.stringify({}),
        })
        const payload = await response.json().catch(() => null)
        if (!response.ok || !payload?.success) throw new Error(ACTION_ERROR)
        await refresh()
        return true
      } catch {
        setError(ACTION_ERROR)
        return false
      } finally {
        setBusySignalId(null)
      }
    },
    [branchFetch, enabled, refresh],
  )

  const ordered = useMemo(() => sortSignals(signals), [signals])

  return {
    signals: ordered,
    enabled,
    loading,
    error,
    busySignalId,
    sendingRoomId,
    refresh,
    send,
    act,
  }
}

/** `useDeskSignalsCore` plus its own `/api/events` subscription. */
export function useDeskSignals(
  options: DeskSignalsOptions = {},
): DeskSignalsClient & { live: boolean } {
  const client = useDeskSignalsCore(options)
  const { branch } = useBranch()
  const live = useLiveRefresh(branch, SIGNAL_LIVE_EVENTS, client.refresh)
  return { ...client, live }
}
