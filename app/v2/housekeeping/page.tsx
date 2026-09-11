'use client'

import { useCallback, useEffect, useMemo, useState } from 'react'
import { AlertCircle, Info, User } from 'lucide-react'
import { useBranch } from '@/contexts/BranchContext'
import { useBranchFetch } from '@/lib/use-branch-fetch'
import { useLiveRefresh } from '@/lib/v2/use-live-refresh'
import { legacyStatusNote } from '@/app/hk/hk-lib'
import { HK_STATUS_LABELS, type HkStatus } from '@/lib/v2/status'
import {
  LiveDot,
  V2Empty,
  V2PageHeader,
  V2Section,
  V2Spinner,
  VilleNotice,
} from '@/components/v2/primitives'
import SignalsPanel from '@/components/v2/signals/SignalsPanel'
import SendSignalMenu from '@/components/v2/signals/SendSignalMenu'
import { useDeskSignalsCore } from '@/components/v2/signals/use-desk-signals'
import {
  SIGNAL_LIVE_EVENTS,
  signalLabel,
  signalTone,
  signalsByRoomNo,
  type RoomSignal,
} from '@/components/v2/signals/signal-lib'

/**
 * RECEPTION's housekeeping screen (v2), superseding the v1 kanban board at
 * `/housekeeping`.
 *
 * READ-ONLY ABOUT CLEANLINESS, by design: it still adds zero clean/dirty write
 * shapes — marking a room clean or dirty stays on the maid surface (`/hk`) and
 * in iHOTEL, so this screen can never become a second, competing source of
 * housekeeping truth.
 *
 * What it now DOES write is room signals (ADR 0008): canned, room-scoped
 * notices between reception and maids. They are a separate domain object with
 * their own table and no legacy mirror — sending ทำห้องนี้ก่อน asserts nothing
 * about whether the room is clean, so the invariant above is untouched. The
 * signal client, its endpoints and its rules live in
 * `components/v2/signals/`.
 *
 * The cleaning half comes from ONE read, `GET /api/housekeeping/cleaning`:
 *
 * - `housekeeping[]` — EVERY active room on the derived housekeeping axis,
 *   already merged iHOTEL-wins by the backend. This is what the three groups
 *   are keyed off, which is why the middle group (กำลังทำความสะอาด) is finally
 *   real: the v1 board could only infer "cleaning" from a same-day `started`
 *   event, so it sat permanently empty.
 * - `data[]` — today's maid-reported progress, present ONLY for rooms with an
 *   event today. It supplies WHO and WHEN, never the group.
 * - `divergent` — iHOTEL and our canonical mirror disagree about this room.
 *   The maid surface suppresses this on purpose (she can only act on one
 *   answer); reception is the desk that reconciles the two systems, so here it
 *   is shown per room with a legend.
 * - `legacyStatusStale` — the iHOTEL read could not answer, so what is on
 *   screen is our own mirror. Same Thai note as `/hk`, from the same constant.
 */

/** One room's latest maid-reported progress today. `at` is a genuine PG
 *  `timestamptz` — see `formatEventTime` before displaying it anywhere new. */
interface CleaningEvent {
  roomId: number
  roomNo: string
  status: 'started' | 'done' | 'dirty'
  badge: string
  name: string | null
  at: string
}

/** One active room on the merged housekeeping axis. */
interface HousekeepingRoom {
  roomNo: string
  hkStatus: HkStatus
  divergent: boolean
}

interface CleaningBoardResponse {
  success: boolean
  data: CleaningEvent[]
  legacyStatusStale?: boolean
  legacyClean: { roomNo: string; clean: boolean }[]
  housekeeping: HousekeepingRoom[]
}

/** The same event names the v1 board subscribes to — published both by this
 *  app's own `/hk` surface and by the inbound CT room mapper when reception
 *  flips a room in iHOTEL directly (`sync/mappers/room.rs`). Module-level so
 *  the array identity stays stable and `useLiveRefresh` doesn't reopen the
 *  stream on every render. */
const CLEANING_LIVE_EVENTS = ['RoomMarkedClean', 'RoomMarkedDirty', 'RoomCleaningStarted']

/** ONE stream for the whole screen: the cleaning events plus the ADR 0008
 *  signal events, folded into the board's single `useLiveRefresh`. The signals
 *  client is therefore the no-subscription `useDeskSignalsCore` — a second
 *  EventSource to the same relay would buy nothing and burn one of the
 *  browser's six per-host connections. */
const BOARD_LIVE_EVENTS = [...CLEANING_LIVE_EVENTS, ...SIGNAL_LIVE_EVENTS]

/** Belt to the SSE braces, matching the v1 board's interval: this only covers
 *  a missed or dropped stream, so 60s (not 30s) is deliberate. */
const SAFETY_POLL_MS = 60000

/** Worst first — a receptionist scanning this screen is looking for what is
 *  not ready yet, not for what already is. */
const GROUP_ORDER: HkStatus[] = ['dirty', 'cleaning', 'clean']

/** Stable identity for "this room has no signals", so a row without any does
 *  not get a fresh array (and a re-render) on every board refresh. */
const EMPTY_SIGNALS: RoomSignal[] = []

/** `cleaning` shares the dirt tone with `dirty` (see `vacantHkView` in
 *  lib/v2/status.ts): neither is sellable-clean. The group heading is what
 *  draws the finer distinction. */
const GROUP_TONE_CLASS: Record<HkStatus, string> = {
  dirty: 's-dirt',
  cleaning: 's-dirt',
  clean: 's-vac',
}

export const DIVERGENT_BADGE = 'ไม่ตรงกับ iHOTEL'

/** Divergence is the anomaly and matching is the norm, so the explainer only
 *  appears when there is at least one badge on screen to explain. It names the
 *  one thing that changes behaviour: the value shown is iHOTEL's. */
const DIVERGENT_LEGEND =
  `ป้าย "${DIVERGENT_BADGE}" หมายถึง iHOTEL กับระบบของเราบันทึกความสะอาดของห้องนั้นไม่ตรงกัน ` +
  'หน้าจอนี้แสดงค่าจาก iHOTEL'

const LOAD_ERROR = 'ไม่สามารถโหลดสถานะความสะอาดได้'

/**
 * Format an event instant for display.
 *
 * `at` is a real PG `timestamptz` — it already carries its own offset, so the
 * browser's NORMAL local conversion is exactly right. Deliberately no
 * `timeZone` override of any kind: `'UTC'` belongs only to naive datetimes
 * mirrored out of legacy MSSQL, and `formatThai`
 * (`components/v2/RoundReport`) pins `'Asia/Bangkok'`, which is the same
 * mistake in the other direction. This mirrors `formatInstant` in the v1
 * board's RoomCleaningCard, which carries the long-form note and the
 * regression test.
 */
function formatEventTime(at: string): string {
  const instant = new Date(at)
  if (Number.isNaN(instant.getTime())) return ''
  return instant.toLocaleTimeString('th-TH', { hour: '2-digit', minute: '2-digit' })
}

/** `name ?? badge` — the CF IdP forwards only `apps`+`badge` today, so `name`
 *  is usually null. */
function reporterName(event: CleaningEvent): string {
  return event.name ?? event.badge
}

/** Today's progress keyed by room number — `housekeeping[]` carries no roomId,
 *  so the join is on `roomNo`. Last entry wins, matching the v1 board. */
export function progressByRoomNo(events: CleaningEvent[]): Map<string, CleaningEvent> {
  return new Map(events.map((event) => [event.roomNo, event]))
}

/** Split the rooms across the three groups, each sorted by room number the way
 *  a human reads them (`102` before `1010`).
 *
 *  An `hkStatus` this build does not know about is SKIPPED rather than thrown
 *  on: a newer backend adding a fourth value must not blank reception's
 *  screen. */
export function groupRoomsByHkStatus(
  rooms: HousekeepingRoom[],
): Record<HkStatus, HousekeepingRoom[]> {
  const groups: Record<HkStatus, HousekeepingRoom[]> = { dirty: [], cleaning: [], clean: [] }
  for (const room of rooms) {
    const group = groups[room.hkStatus]
    if (group) group.push(room)
  }
  for (const status of GROUP_ORDER) {
    groups[status].sort((a, b) => a.roomNo.localeCompare(b.roomNo, undefined, { numeric: true }))
  }
  return groups
}

function RoomRow({
  room,
  progress,
  signals,
  roomId,
  sending,
  onSend,
}: {
  room: HousekeepingRoom
  progress?: CleaningEvent
  /** This room's open/acked signals — the per-room indication (ADR 0008). */
  signals: RoomSignal[]
  /** `/api/housekeeping/cleaning` carries only `roomNo`, so the id for the
   *  send endpoint is resolved from the room list. Undefined ⇒ no send action
   *  on this row rather than a POST to an invented id. */
  roomId?: number
  sending?: boolean
  onSend?: (roomId: number, type: string) => void
}) {
  return (
    <div className="flex flex-wrap items-center gap-2.5 px-4 lg:px-5 py-2.5">
      <span className="v2-num text-[14px] font-semibold" style={{ color: 'var(--v2-ink)' }}>
        {room.roomNo}
      </span>

      {room.divergent && <span className="v2-pill s-fix">{DIVERGENT_BADGE}</span>}

      {signals.map((signal) => (
        <span key={signal.signalId} className={`v2-pill s-${signalTone(signal)}`}>
          {signalLabel(signal.type)}
        </span>
      ))}

      <div className="flex-1" />

      {progress && (
        <span
          className="inline-flex items-center gap-1.5 text-[11.5px]"
          style={{ color: 'var(--v2-ink-3)' }}
        >
          <User size={13} />
          {reporterName(progress)} · {formatEventTime(progress.at)}
        </span>
      )}

      {roomId !== undefined && onSend && (
        <SendSignalMenu
          roomId={roomId}
          roomNo={room.roomNo}
          busy={sending}
          onSend={onSend}
        />
      )}
    </div>
  )
}

/** A notice ABOUT the data, not a failure of the screen — amber, above the
 *  board, with the board still fully usable beneath it. Same treatment as the
 *  maid surface's stale note (`app/hk/page.tsx`). */
function BoardNotice({ text }: { text: string }) {
  return (
    <div
      className="v2-card flex items-start gap-3 px-4 py-3"
      style={{ borderColor: 'var(--v2-arr)', background: 'var(--v2-arr-bg)' }}
      role="status"
    >
      <Info size={18} className="mt-0.5 shrink-0" style={{ color: 'var(--v2-arr)' }} />
      <p className="text-[13.5px] flex-1" style={{ color: 'var(--v2-ink)' }}>
        {text}
      </p>
    </div>
  )
}

export default function V2Housekeeping() {
  const { branch } = useBranch()
  const branchFetch = useBranchFetch()

  const [board, setBoard] = useState<CleaningBoardResponse | null>(null)
  const [loading, setLoading] = useState(true)
  const [error, setError] = useState<string | null>(null)

  const fetchBoard = useCallback(async () => {
    try {
      const response = await branchFetch('/api/housekeeping/cleaning')
      if (!response.ok) throw new Error(LOAD_ERROR)
      const payload: CleaningBoardResponse = await response.json()
      if (!payload?.success) throw new Error(LOAD_ERROR)
      setBoard(payload)
      setError(null)
    } catch {
      // The previous board is deliberately LEFT ON SCREEN alongside the error
      // (same reasoning as /hk): a stale board reception can still read beats
      // a blank one, and the note says why it may be behind.
      setError(LOAD_ERROR)
    } finally {
      setLoading(false)
    }
  }, [branchFetch])

  useEffect(() => {
    fetchBoard()
    const interval = setInterval(fetchBoard, SAFETY_POLL_MS)
    return () => clearInterval(interval)
  }, [fetchBoard])

  // ADR 0008 room signals. `Core` = no stream of its own; this page folds the
  // signal events into the single subscription below.
  const deskSignals = useDeskSignalsCore()
  const refreshSignals = deskSignals.refresh

  // `/api/housekeeping/cleaning` returns room NUMBERS only, but sending a
  // signal needs the room ID. Resolved once per branch off the same room list
  // the rooms board reads; a row whose number is unresolved simply shows no
  // send action (never a POST to a guessed id).
  const [roomIds, setRoomIds] = useState<Map<string, number>>(new Map())
  useEffect(() => {
    let cancelled = false
    branchFetch('/api/rooms?limit=300')
      .then((response) => (response.ok ? response.json() : null))
      .then((payload) => {
        if (cancelled || !payload) return
        const list = (payload.data ?? payload) as unknown
        if (!Array.isArray(list)) return
        const pairs = list
          .filter(
            (room): room is { id: number; roomNo: string } =>
              typeof room?.id === 'number' && typeof room?.roomNo === 'string',
          )
          .map((room) => [room.roomNo, room.id] as const)
        setRoomIds(new Map(pairs))
      })
      .catch(() => {
        /* send actions stay hidden; the board itself is unaffected */
      })
    return () => {
      cancelled = true
    }
  }, [branchFetch])

  const refreshAll = useCallback(() => {
    fetchBoard()
    refreshSignals()
  }, [fetchBoard, refreshSignals])

  const live = useLiveRefresh(branch, BOARD_LIVE_EVENTS, refreshAll)

  const rooms = useMemo(() => board?.housekeeping ?? [], [board])
  const groups = useMemo(() => groupRoomsByHkStatus(rooms), [rooms])
  const progress = useMemo(() => progressByRoomNo(board?.data ?? []), [board])
  const hasDivergence = rooms.some((room) => room.divergent)
  const signalsByRoom = useMemo(
    () => signalsByRoomNo(deskSignals.signals),
    [deskSignals.signals],
  )

  // `=== true` only — a backend that stops sending the flag (rollback) must
  // read as "nothing to say", never as a permanent warning banner.
  const staleNote = legacyStatusNote(board?.legacyStatusStale)

  return (
    <div className="space-y-5">
      <V2PageHeader
        eyebrow="แผนกแม่บ้าน"
        title="สถานะความสะอาดห้องพัก"
        right={<LiveDot connected={live} />}
      />

      <VilleNotice branch={branch} />

      {staleNote && <BoardNotice text={staleNote} />}

      {error && (
        <div
          className="v2-card flex items-start gap-3 px-4 py-3"
          style={{ borderColor: 'var(--v2-fix)', background: 'var(--v2-fix-bg)' }}
          role="alert"
        >
          <AlertCircle size={18} className="mt-0.5 shrink-0" style={{ color: 'var(--v2-fix)' }} />
          <p className="text-[13.5px] flex-1" style={{ color: 'var(--v2-ink)' }}>
            {error}
          </p>
        </div>
      )}

      {deskSignals.error && (
        <div
          className="v2-card flex items-start gap-3 px-4 py-3"
          style={{ borderColor: 'var(--v2-fix)', background: 'var(--v2-fix-bg)' }}
          role="alert"
        >
          <AlertCircle size={18} className="mt-0.5 shrink-0" style={{ color: 'var(--v2-fix)' }} />
          <p className="text-[13.5px] flex-1" style={{ color: 'var(--v2-ink)' }}>
            {deskSignals.error}
          </p>
        </div>
      )}

      {/* Room signals sit ABOVE the cleaning groups: a signal is somebody
          waiting on reception right now, while the groups are the state of the
          floor. Hidden entirely in the aggregate ("ทั้งหมด") view, which has no
          single branch to scope a signal to. */}
      {deskSignals.enabled && (
        <SignalsPanel
          signals={deskSignals.signals}
          busySignalId={deskSignals.busySignalId}
          loading={deskSignals.loading}
          onAct={deskSignals.act}
        />
      )}

      {loading ? (
        <V2Spinner label="กำลังโหลดสถานะความสะอาด…" />
      ) : rooms.length === 0 ? (
        <V2Empty title="ไม่มีข้อมูลห้องพัก" hint="ยังไม่มีห้องที่เปิดใช้งานสำหรับสาขานี้" />
      ) : (
        <div className="space-y-4">
          {GROUP_ORDER.map((status) => (
            <V2Section
              key={status}
              title={HK_STATUS_LABELS[status]}
              count={groups[status].length}
              tone={GROUP_TONE_CLASS[status]}
            >
              {groups[status].length === 0 ? (
                <V2Empty title="ไม่มีห้องในกลุ่มนี้" />
              ) : (
                <div className="divide-y" style={{ borderColor: 'var(--v2-line)' }}>
                  {groups[status].map((room) => (
                    <RoomRow
                      key={room.roomNo}
                      room={room}
                      progress={progress.get(room.roomNo)}
                      signals={signalsByRoom.get(room.roomNo) ?? EMPTY_SIGNALS}
                      roomId={deskSignals.enabled ? roomIds.get(room.roomNo) : undefined}
                      sending={deskSignals.sendingRoomId === roomIds.get(room.roomNo)}
                      onSend={deskSignals.send}
                    />
                  ))}
                </div>
              )}
            </V2Section>
          ))}

          {hasDivergence && (
            <p
              className="flex items-start gap-1.5 text-[11.5px]"
              style={{ color: 'var(--v2-ink-3)' }}
            >
              <Info size={13} className="mt-0.5 shrink-0" />
              {DIVERGENT_LEGEND}
            </p>
          )}
        </div>
      )}
    </div>
  )
}
