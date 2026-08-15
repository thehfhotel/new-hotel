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
import { AlertCircle, Loader2, RefreshCw, Sparkles } from 'lucide-react'
import {
  groupRoomsByFloor,
  hkFetch,
  hkFetchMe,
  progressLabel,
  readStoredBranch,
  resolveInitialBranch,
  storeBranch,
  timeLabel,
  type Branch,
  type HkMe,
  type HkRoom,
} from './hk-lib'
import { HkBranchChip, HkBranchesUnavailable, HkBranchPicker } from './HkBranchPicker'

export default function HkRoomListPage() {
  const [me, setMe] = useState<HkMe | null>(null)
  const [meError, setMeError] = useState<string | null>(null)
  const [branch, setBranch] = useState<Branch | null>(null)
  const [branchResolved, setBranchResolved] = useState(false)
  const [rooms, setRooms] = useState<HkRoom[]>([])
  const [loading, setLoading] = useState(true)
  const [error, setError] = useState<string | null>(null)

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
  const loadRooms = useCallback(async () => {
    if (!branch) return
    setLoading(true)
    try {
      const res = await hkFetch('/rooms', branch)
      if (!res.ok) throw new Error('ไม่สามารถดึงข้อมูลห้องได้ กรุณาลองใหม่')
      const roomsData: { success: boolean; data: HkRoom[] } = await res.json()
      if (!roomsData.success) throw new Error('ไม่สามารถดึงข้อมูลห้องได้ กรุณาลองใหม่')
      setRooms(roomsData.data)
      setError(null)
    } catch (err) {
      setError(err instanceof Error ? err.message : 'เกิดข้อผิดพลาด')
    } finally {
      setLoading(false)
    }
  }, [branch])

  useEffect(() => {
    if (branch) loadRooms()
  }, [branch, loadRooms])

  const pickBranch = (next: Branch) => {
    storeBranch(next)
    setRooms([])
    setError(null)
    setBranch(next)
  }

  const doneCount = rooms.filter((r) => r.cleaning?.status === 'done').length
  const startedCount = rooms.filter((r) => r.cleaning?.status === 'started').length

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

          {/* Progress summary + refresh */}
          {!error && (
            <div className="mb-4 flex items-center justify-between rounded-xl border border-gray-200 bg-white px-4 py-3 text-sm">
              <div className="flex gap-4">
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
                onClick={loadRooms}
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
                    return (
                      <li key={room.roomId}>
                        <Link
                          href={`/hk/rooms/${room.roomId}`}
                          className="block rounded-xl border border-gray-200 bg-white p-3 active:bg-gray-50"
                        >
                          <div className="flex items-center justify-between">
                            <span className="text-lg font-bold">{room.roomNo}</span>
                            <span className="flex items-center gap-1">
                              {!room.roomClean && (
                                <span
                                  title="ห้องยังไม่สะอาด"
                                  className="inline-block h-2.5 w-2.5 rounded-full bg-red-500"
                                />
                              )}
                            </span>
                          </div>
                          <span
                            className={`mt-2 inline-block rounded-full border px-2 py-0.5 text-xs ${badge.className}`}
                          >
                            {badge.label}
                          </span>
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
