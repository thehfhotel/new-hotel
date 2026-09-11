'use client'

// Maid room list (/hk) — employee-login plan Phase 4, wave-4 §A+B.
//
// Shows every active room of the CHOSEN property with today's maid-reported
// cleaning progress; tapping a room opens /hk/rooms/[id] to report progress.
// แจ้งซ่อม / เบิกของ are NOT reachable from here: both are top-level tiles on
// the LINE rich menu, and a second route to the same place is a cost for this
// audience. Identity is resolved server-side from the Cloudflare Access
// assertion (silent HF ID); a 401/403 renders the fail-closed notice — there
// is deliberately NO login UI.
//
// The property is an explicit, never-defaulted pick (§A1): `GET /api/hk/me`
// is fetched first (branch-free — it's what tells us which branches exist),
// then `resolveInitialBranch` either auto-selects (single branch — the
// shipping state today) or blocks on `HkBranchPicker` until the maid taps
// one. A branch chosen or switched here is what every subsequent `/hk`
// fetch (this page and the room-detail page) carries — `hkFetch` refuses to
// run with none.

import { useCallback, useEffect, useRef, useState } from 'react'
import Link from 'next/link'
import {
  AlertCircle,
  ChevronRight,
  ClipboardList,
  Info,
  Loader2,
  RefreshCw,
  Shirt,
  Sparkles,
  Volume2,
  VolumeX,
} from 'lucide-react'
import {
  canReport,
  countRoomsNeedingClean,
  fetchHkSignals,
  groupRoomsByFloor,
  hkFetch,
  hkFetchMe,
  isIncomingSignal,
  legacyStatusNote,
  linenShortageTag,
  mergeSignal,
  movementTags,
  occupancyIndicator,
  openLinenCountLabel,
  openLinenRooms,
  progressLabel,
  readSignalSoundMuted,
  readStoredBranch,
  resolveInitialBranch,
  roomCleanChip,
  roomSignalChip,
  signalCountsByRoom,
  signalRole,
  storeBranch,
  storeSignalSoundMuted,
  timeLabel,
  type Branch,
  type HkMe,
  type HkRoom,
  type HkRoomsResponse,
  type RoomSignal,
} from './hk-lib'
import { HkBranchChip, HkBranchesUnavailable, HkBranchPicker } from './HkBranchPicker'
import { useHkAutoRefresh } from './use-hk-auto-refresh'
import { playHkSignalCue, useHkSignalEvents } from './use-hk-signal-events'

export default function HkRoomListPage() {
  const [me, setMe] = useState<HkMe | null>(null)
  const [meError, setMeError] = useState<string | null>(null)
  const [branch, setBranch] = useState<Branch | null>(null)
  const [branchResolved, setBranchResolved] = useState(false)
  const [rooms, setRooms] = useState<HkRoom[]>([])
  const [loading, setLoading] = useState(true)
  const [error, setError] = useState<string | null>(null)
  // CR-1: the room-clean column normally shows iHOTEL's answer. When the
  // backend could not reach iHOTEL it serves the PMS mirror and sets this, and
  // the maid is TOLD — a stale list she can work beats an error page.
  const [legacyStale, setLegacyStale] = useState(false)

  // Room signals (ADR 0008) — the branch's live open+acked signals, which this
  // screen renders as ONE chip per room ("แจ้ง 2"). Held as the branch's whole
  // list rather than per-room, because that is what both the `/signals` read
  // and the SSE stream deliver, and `signalCountsByRoom` turns it into the
  // per-card number in a single pass.
  const [signals, setSignals] = useState<RoomSignal[]>([])
  // The same list, eagerly. The cue must fire only for a signal we have NOT
  // seen, and two events can land inside one React batch — a ref answers
  // "already known?" without depending on a state update having flushed.
  const signalsRef = useRef<RoomSignal[]>([])
  const [soundMuted, setSoundMuted] = useState(false)

  // Step 1: identity + which branches exist. Runs once; never guesses a
  // branch itself — it only ever resolves what's already valid.
  const loadMe = useCallback(async () => {
    try {
      const res = await hkFetchMe()
      if (!res.ok) throw new Error('ไม่สามารถดึงข้อมูลผู้ใช้ได้ กรุณาลองใหม่')
      const data: HkMe = await res.json()
      if (!data.success) throw new Error('ไม่สามารถดึงข้อมูลผู้ใช้ได้ กรุณาลองใหม่')
      setMe(data)
      const resolved = resolveInitialBranch(data.branches, readStoredBranch())
      if (resolved) storeBranch(resolved)
      setBranch(resolved)
      setMeError(null)
    } catch (err) {
      setMeError(err instanceof Error ? err.message : 'เกิดข้อผิดพลาด')
      setLoading(false)
    } finally {
      setBranchResolved(true)
    }
  }, [])

  useEffect(() => {
    loadMe()
  }, [loadMe])

  // Step 2: rooms, once a branch is known. Re-runs whenever the maid picks
  // or switches branch.
  //
  // `background` is the auto-refresh path (`useHkAutoRefresh`), and it is
  // deliberately QUIETER than a load the maid asked for:
  //  * no spinner — the refresh icon must not twitch every minute on a screen
  //    she is reading, and the list is already on screen anyway;
  //  * a failure is SWALLOWED, keeping the last good list. A maid walks
  //    through stairwells and lift lobbies; a poll that fails because she lost
  //    signal for eight seconds is a self-recovering blip, and painting a red
  //    banner for it every minute only teaches her to ignore the banner that
  //    matters. Genuine breakage still surfaces on the next tap of รีเฟรช, on
  //    a branch switch, or on the next reload.
  const loadRooms = useCallback(
    async (background = false) => {
      if (!branch) return
      if (!background) setLoading(true)
      try {
        const res = await hkFetch('/rooms', branch)
        if (!res.ok) throw new Error('ไม่สามารถดึงข้อมูลห้องได้ กรุณาลองใหม่')
        const roomsData: HkRoomsResponse = await res.json()
        if (!roomsData.success) throw new Error('ไม่สามารถดึงข้อมูลห้องได้ กรุณาลองใหม่')
        setRooms(roomsData.data)
        setLegacyStale(roomsData.legacyStatusStale === true)
        setError(null)
      } catch (err) {
        if (!background) setError(err instanceof Error ? err.message : 'เกิดข้อผิดพลาด')
      } finally {
        if (!background) setLoading(false)
      }
    },
    [branch]
  )

  useEffect(() => {
    if (branch) loadRooms()
  }, [branch, loadRooms])

  // The fix for "status doesn't sync": this page is opened from a LINE tile
  // and the WebView keeps it alive for hours, so without this it shows the
  // rooms as they were when the maid started her round and never again.
  const refreshRooms = useCallback(() => loadRooms(true), [loadRooms])
  useHkAutoRefresh(refreshRooms, Boolean(branch))

  // Step 3: this branch's live signals. Deliberately its OWN request and its
  // own failure story: a signal read that fails must cost the chips, never the
  // room list — the list is what a maid works from, and it is already on
  // screen. Nothing here ever raises the page's error banner.
  const applySignals = useCallback((next: RoomSignal[]) => {
    signalsRef.current = next
    setSignals(next)
  }, [])

  const loadSignals = useCallback(async () => {
    if (!branch) return
    try {
      applySignals(await fetchHkSignals(branch))
    } catch {
      /* chips stay as they were; the poll or the stream will catch up */
    }
  }, [branch, applySignals])

  useEffect(() => {
    if (branch) loadSignals()
  }, [branch, loadSignals])

  // Which side of the conversation this identity is on — the same `/me` flag
  // that gates the reporting surface on the room screen. It decides only which
  // arrivals are worth a SOUND here; the chips count both directions, because
  // "this room has work outstanding" is true for either role.
  const soundMutedRef = useRef(false)
  const role = signalRole(canReport(me))
  const handleSignal = useCallback(
    (signal: RoomSignal) => {
      const isNew = !signalsRef.current.some((s) => s.signalId === signal.signalId)
      applySignals(mergeSignal(signalsRef.current, signal))
      // Only a brand-new signal pointed at THIS role: never an ack/done echo,
      // never one this role sent itself.
      if (isNew && isIncomingSignal(signal, role)) playHkSignalCue()
    },
    [role, applySignals]
  )

  // Live push, with the existing poll as the fallback beneath it: while the
  // stream is connected the poll would only duplicate it, and the moment it
  // drops (a lift, a captive portal, a backend restart) the sixty-second
  // cadence takes over on its own. An SSE failure can never break this page —
  // the hook swallows everything and simply reports `live: false`.
  const signalsLive = useHkSignalEvents(branch, handleSignal, Boolean(branch))
  useHkAutoRefresh(loadSignals, Boolean(branch) && !signalsLive)

  // The mute is stored, not React state alone, so BOTH /hk screens honour one
  // toggle. Read after mount — localStorage is not available during the
  // server render.
  useEffect(() => {
    const stored = readSignalSoundMuted()
    soundMutedRef.current = stored
    setSoundMuted(stored)
  }, [])

  const toggleSound = () => {
    const next = !soundMutedRef.current
    soundMutedRef.current = next
    setSoundMuted(next)
    storeSignalSoundMuted(next)
  }

  const pickBranch = (next: Branch) => {
    storeBranch(next)
    setRooms([])
    setError(null)
    // Signals are branch-scoped exactly as rooms are; carrying the old
    // branch's chips across would flag the WRONG hotel's rooms for a beat.
    applySignals([])
    // The note describes the OTHER branch's read; carrying it across would
    // claim iHOTEL is unreachable for a property we have not asked about yet.
    setLegacyStale(false)
    setBranch(next)
  }

  const doneCount = rooms.filter((r) => r.cleaning?.status === 'done').length
  const startedCount = rooms.filter((r) => r.cleaning?.status === 'started').length
  // The number a maid actually plans her round by — merged iHOTEL-wins
  // roomClean, not today's progress. See countRoomsNeedingClean in hk-lib.
  const dirtyCount = countRoomsNeedingClean(rooms)
  // One pass for the whole grid rather than a scan per card — ~58 cards on a
  // phone, re-rendered on every stream event.
  const signalCounts = signalCountsByRoom(signals)
  // The ขาดผ้า queue: every room with an OPEN shortage, whatever day it was
  // filed on. Strict on `linenShortageOpen` (see `openLinenRooms`), so an older
  // backend renders no panel at all rather than a queue of rows whose
  // completion button has no endpoint behind it.
  const openLinen = openLinenRooms(rooms)

  // The EMPTY case (§C): location enforcement resolved this maid to no branch
  // at all. Blocks like the picker does, but offers nothing to tap — there is
  // no branch to guess. Checked BEFORE `showPicker` so the "choose one" screen
  // is never rendered with zero choices.
  const showUnavailable = branchResolved && me && !meError && me.branches.length === 0

  // Blocking picker: identity resolved, more than one branch configured,
  // nothing valid stored. Never falls through to fetching rooms for a
  // guessed branch.
  const showPicker = branchResolved && me && !meError && !branch && !showUnavailable

  return (
    <main>
      {/* Header */}
      <header className="mb-4 flex items-start justify-between gap-2">
        <div>
          <h1 className="flex items-center gap-2 text-xl font-bold">
            <Sparkles className="h-6 w-6 text-red-600" />
            งานแม่บ้าน
          </h1>
          {me && (
            <p className="mt-1 text-sm text-gray-500">
              สวัสดี {me.displayName || `รหัส ${me.badge}`}
            </p>
          )}
        </div>
        <div className="flex items-center gap-2">
          {/* The cue's mute. Lives here, on the screen a maid leaves open all
              morning, and is honoured by the room screen too (it is stored,
              not component state). 44px target like every other control on
              this surface. */}
          {branch && (
            <button
              type="button"
              onClick={toggleSound}
              aria-label={soundMuted ? 'เปิดเสียงแจ้งเตือน' : 'ปิดเสียงแจ้งเตือน'}
              aria-pressed={soundMuted}
              className="flex h-11 w-11 items-center justify-center rounded-lg border border-gray-300 text-gray-500 active:bg-gray-100"
            >
              {soundMuted ? <VolumeX className="h-5 w-5" /> : <Volume2 className="h-5 w-5" />}
            </button>
          )}
          {me && branch && (
            <HkBranchChip branches={me.branches} current={branch} onSwitch={pickBranch} />
          )}
        </div>
      </header>

      {/* /me failure — fail-closed, no fallback branch */}
      {meError && (
        <div className="mb-4 flex items-start gap-2 rounded-lg border border-red-200 bg-red-50 p-3 text-sm text-red-700">
          <AlertCircle className="mt-0.5 h-5 w-5 shrink-0" />
          <span>{meError}</span>
        </div>
      )}

      {showUnavailable && me && (
        <HkBranchesUnavailable reason={me.branchesUnavailableReason ?? null} />
      )}

      {showPicker && me && <HkBranchPicker branches={me.branches} onPick={pickBranch} />}

      {!showPicker && !showUnavailable && !meError && (
        <>
          {/* ------------------------------------------------------------- *
           * Report HK entry point. The daily room report has its own LINE
           * tile, but a maid already standing on this screen must not have to
           * go back to the rich menu to reach it — and a reception viewer has
           * no rich menu at all, so this is her ONLY way in.
           *
           * TEAL, a hue nothing on this surface has claimed: red = dirty,
           * emerald = done, amber = in progress, sky = ขาดผ้า, indigo =
           * signals, violet/orange = today's movement. The report is a
           * different KIND of thing again, and reads as one.
           * ------------------------------------------------------------- */}
          <Link
            href="/hk/report"
            data-testid="hk-report-entry"
            className="mb-4 flex min-h-[44px] w-full items-center justify-center gap-1.5 rounded-xl border border-teal-300 bg-teal-50 px-3 py-3 text-sm font-semibold text-teal-800 active:bg-teal-100"
          >
            <ClipboardList className="h-4 w-4" />
            รายงานประจำวัน
          </Link>

          {/* Room-list error notice */}
          {error && (
            <div className="mb-4 flex items-start gap-2 rounded-lg border border-red-200 bg-red-50 p-3 text-sm text-red-700">
              <AlertCircle className="mt-0.5 h-5 w-5 shrink-0" />
              <span>{error}</span>
            </div>
          )}

          {/* CR-1 fallback notice: iHOTEL unreachable, showing the PMS mirror.
              AMBER, not red, and above the list rather than in place of it —
              this is a notice about the data, not a failure of the screen. */}
          {!error && legacyStatusNote(legacyStale) && (
            <div
              role="status"
              className="mb-4 flex items-start gap-2 rounded-lg border border-amber-200 bg-amber-50 p-3 text-sm text-amber-800"
            >
              <Info className="mt-0.5 h-5 w-5 shrink-0" />
              <span>{legacyStatusNote(legacyStale)}</span>
            </div>
          )}

          {/* ------------------------------------------------------------- *
           * ขาดผ้า WORK QUEUE. The owner's ask (2026-09-01) in one panel: an
           * open linen shortage must be impossible to miss and possible to
           * FINISH. The per-room chip further down is still there, but a pale
           * chip inside one card of a two-column grid of ~58 near-identical
           * cards is exactly the thing an eye skims past — and a shortage now
           * survives the day rollover, so skimming past it means a room stays
           * short for days.
           *
           * So it is lifted OUT of the grid to the top of the list, above the
           * floor groups and below the header: a heading, the number of rooms,
           * and one 44px tappable row per room straight into that room's
           * screen, where เติมผ้าแล้ว lives. Sky-toned like every other ขาดผ้า
           * surface (same subject, one colour), but bordered and filled rather
           * than a chip outline — this is the maid's work queue, not a badge.
           *
           * Rendered for BOTH roles: a reception viewer cannot resolve a
           * shortage, but "which rooms are still short" is precisely the fact
           * she opens this screen for.
           * ------------------------------------------------------------- */}
          {openLinen.length > 0 && (
            <section
              data-testid="hk-linen-panel"
              className="mb-4 rounded-xl border-2 border-sky-400 bg-sky-50 p-3"
            >
              <h2 className="mb-2 flex items-center gap-1.5 text-base font-bold text-sky-900">
                <Shirt className="h-5 w-5 shrink-0" />
                <span>ขาดผ้า</span>
                <span className="ml-auto rounded-full border border-sky-300 bg-white px-2 py-0.5 text-xs font-semibold text-sky-800">
                  {openLinenCountLabel(openLinen.length)}
                </span>
              </h2>
              <ul className="space-y-2">
                {openLinen.map((room) => (
                  <li key={room.roomId}>
                    <Link
                      href={`/hk/rooms/${room.roomId}`}
                      className="flex min-h-[44px] items-center justify-between gap-2 rounded-lg border border-sky-300 bg-white px-3 py-2 active:bg-sky-100"
                    >
                      {/* "ห้อง 301", not a bare "301": this row sits far from
                          the grid card that carries the same number, and the
                          word is what makes it read as a room rather than a
                          quantity. */}
                      <span className="text-base font-semibold text-gray-900">
                        ห้อง {room.roomNo}
                      </span>
                      <ChevronRight className="h-5 w-5 shrink-0 text-sky-700" />
                    </Link>
                  </li>
                ))}
              </ul>
            </section>
          )}

          {/* Progress summary + refresh */}
          {!error && (
            <div
              data-testid="hk-summary"
              className="mb-4 flex items-center justify-between rounded-xl border border-gray-200 bg-white px-4 py-3 text-sm"
            >
              <div className="flex flex-wrap gap-x-4 gap-y-1">
                {/* The number a maid actually plans by — merged-dirty rooms,
                    listed first because it is the one that drives her round. */}
                <span className="text-red-700">
                  รอทำความสะอาด <strong>{dirtyCount}</strong>
                </span>
                <span className="text-emerald-700">
                  เสร็จแล้ว <strong>{doneCount}</strong>
                </span>
                <span className="text-amber-700">
                  กำลังทำ <strong>{startedCount}</strong>
                </span>
                <span className="text-gray-500">
                  ทั้งหมด <strong>{rooms.length}</strong>
                </span>
              </div>
              <button
                type="button"
                onClick={() => loadRooms()}
                disabled={loading}
                aria-label="รีเฟรช"
                className="rounded-lg border border-gray-300 p-2 text-gray-500 active:bg-gray-100 disabled:opacity-50"
              >
                <RefreshCw className={`h-4 w-4 ${loading ? 'animate-spin' : ''}`} />
              </button>
            </div>
          )}

          {/* Room list grouped by floor */}
          {loading && rooms.length === 0 && !error ? (
            <div className="flex items-center justify-center py-16 text-gray-500">
              <Loader2 className="mr-2 h-6 w-6 animate-spin" />
              กำลังโหลด...
            </div>
          ) : (
            groupRoomsByFloor(rooms).map(({ floor, rooms: floorRooms }) => (
              <section key={floor ?? 'none'} className="mb-5">
                <h2 className="mb-2 text-sm font-semibold text-gray-500">
                  {floor !== null ? `ชั้น ${floor}` : 'อื่น ๆ'}
                </h2>
                <ul className="grid grid-cols-2 gap-2">
                  {floorRooms.map((room) => {
                    const badge = progressLabel(room.cleaning?.status)
                    const cleanChip = roomCleanChip(room.roomClean)
                    const occupancy = occupancyIndicator(room.occupancy)
                    const tags = movementTags(room)
                    const linenTag = linenShortageTag(room)
                    const signalChip = roomSignalChip(signalCounts.get(room.roomId) ?? 0)
                    return (
                      <li key={room.roomId}>
                        <Link
                          href={`/hk/rooms/${room.roomId}`}
                          className="block rounded-xl border border-gray-200 bg-white p-3 active:bg-gray-50"
                        >
                          {/* Room number answers "where"; this top-right slot
                              answers "can I enter" (guest occupancy) — a
                              different question from the chips below, which
                              answer "what work". */}
                          <div className="flex items-center justify-between">
                            <span className="text-lg font-bold">{room.roomNo}</span>
                            {occupancy && (
                              <span
                                className={`flex items-center gap-1 text-xs font-medium ${occupancy.className}`}
                              >
                                {room.occupancy === 'occupied' && (
                                  <span className="inline-block h-2 w-2 rounded-full bg-sky-500" />
                                )}
                                {occupancy.label}
                              </span>
                            )}
                          </div>
                          {/* Day-scoped movement (arrivals/departures today) —
                              a different axis from occupancy (right now) and
                              the chips (what work). Departure first. Renders
                              nothing at all, no placeholder, when there is
                              nothing to say today. */}
                          {tags.length > 0 && (
                            <div className="mt-1 flex flex-wrap gap-1">
                              {tags.map((tag) => (
                                <span
                                  key={tag.key}
                                  className={`inline-block rounded-full border px-1.5 py-0.5 text-[11px] ${tag.className}`}
                                >
                                  {tag.label}
                                </span>
                              ))}
                            </div>
                          )}
                          {/* Primary: explicit clean/dirty (merged iHOTEL-wins
                              roomClean). Secondary: today's maid-reported
                              progress — dirty + ยังไม่เริ่ม is the ordinary
                              morning state, both chips stay visible together.
                              Third, when there is one: ขาดผ้า — an OPEN linen
                              shortage of any age, cleared by เติมผ้าแล้ว and
                              no longer by the day rolling over. It lives in the
                              CHIP row, which every room always renders, rather
                              than in the conditional tag row above — so it
                              shows next to EVERY cleaning state, and above all
                              next to เสร็จแล้ว: a finished room still short of
                              linen must not read as finished-and-forgotten.
                              The queue panel at the top of the list is the
                              same fact lifted out of the grid; this chip is
                              what keeps it visible while scanning by floor. */}
                          <div className="mt-2 flex flex-wrap items-center gap-1">
                            <span
                              className={`inline-block rounded-full border px-2 py-0.5 text-xs ${cleanChip.className}`}
                            >
                              {cleanChip.label}
                            </span>
                            <span
                              className={`inline-block rounded-full border px-2 py-0.5 text-xs ${badge.className}`}
                            >
                              {badge.label}
                            </span>
                            {linenTag && (
                              <span
                                className={`inline-block rounded-full border px-2 py-0.5 text-xs ${linenTag.className}`}
                              >
                                {linenTag.label}
                              </span>
                            )}
                            {/* Fourth chip, when this room carries live room
                                signals (ADR 0008). Same row and same idiom as
                                ขาดผ้า — a fact about the room, not a third
                                cleanliness severity — but SOLID, because it is
                                the only chip here describing somebody
                                WAITING: the desk asking for a checkout
                                inspection while a guest stands at the counter
                                must not read like a pale badge in a grid of
                                pale badges. The room screen lists what the
                                signals actually are. */}
                            {signalChip && (
                              <span
                                className={`inline-block rounded-full border px-2 py-0.5 text-xs font-semibold ${signalChip.className}`}
                              >
                                {signalChip.label}
                              </span>
                            )}
                          </div>
                          {room.cleaning && (
                            <p className="mt-1 text-[11px] text-gray-400">
                              {timeLabel(room.cleaning.at)}
                              {room.cleaning.name ? ` โดย ${room.cleaning.name}` : ''}
                            </p>
                          )}
                        </Link>
                      </li>
                    )
                  })}
                </ul>
              </section>
            ))
          )}
        </>
      )}
    </main>
  )
}
