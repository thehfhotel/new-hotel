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

import { useCallback, useEffect, useState } from 'react'
import Link from 'next/link'
import { AlertCircle, Info, Loader2, RefreshCw, Sparkles } from 'lucide-react'
import {
  countRoomsNeedingClean,
  groupRoomsByFloor,
  hkFetch,
  hkFetchMe,
  legacyStatusNote,
  linenShortageTag,
  movementTags,
  occupancyIndicator,
  progressLabel,
  readStoredBranch,
  resolveInitialBranch,
  roomCleanChip,
  storeBranch,
  timeLabel,
  type Branch,
  type HkMe,
  type HkRoom,
  type HkRoomsResponse,
} from './hk-lib'
import { HkBranchChip, HkBranchesUnavailable, HkBranchPicker } from './HkBranchPicker'
import { useHkAutoRefresh } from './use-hk-auto-refresh'

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

  const pickBranch = (next: Branch) => {
    storeBranch(next)
    setRooms([])
    setError(null)
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
        {me && branch && (
          <HkBranchChip branches={me.branches} current={branch} onSwitch={pickBranch} />
        )}
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
                              Third, when there is one: ขาดผ้า. It lives in the
                              CHIP row, which every room always renders, rather
                              than in the conditional tag row above — so it
                              shows next to EVERY cleaning state, and above all
                              next to เสร็จแล้ว: a finished room still short of
                              linen must not read as finished-and-forgotten. */}
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
