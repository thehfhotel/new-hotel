'use client'

// Report HK — DAY OVERVIEW (/hk/report). The paper day-sheet's heir, and the
// landing page of its own LINE tile (https://hotel.thehfhotel.org/hk/report).
//
// One row per active room of the chosen property, carrying that room's LATEST
// report for the day: ยังไม่ส่ง / ส่งแล้ว รอตรวจ / ตรวจแล้ว / ส่งกลับแก้ไข
// (with its canned reason). Tapping a row opens that room's report screen,
// which is where the filing and the verifying actually happen.
//
// ROLE-SWITCHED, but only in EMPHASIS: both roles see every room and every
// state — the sheet is a shared artifact — and `sortReportRooms` puts each
// role's own work at the top of its floor group. A maid leads with the rooms
// that came back to her; reception leads with what is waiting to be checked.
//
// BRANCH. Same never-defaulted rules as /hk (§A1): `/api/hk/me` first, then
// `resolveInitialBranch` either auto-selects the single configured branch or
// blocks on the picker. This screen may PICK one, unlike the room detail
// screens, because it is a LINE-tile landing page in its own right — a maid
// who opens the report tile first must not be sent to another screen to choose.
//
// DATE. Today only, in Bangkok, as the SERVER computes it — v1 has no date
// picker on purpose: the paper sheet is a today artifact, and a picker invites
// filing against yesterday, which the append-only history has no way to undo.
// The server echoes the day it used and that is what the header renders.

import { useCallback, useEffect, useState } from 'react'
import Link from 'next/link'
import {
  AlertCircle,
  Check,
  ChevronLeft,
  ChevronRight,
  ClipboardList,
  Loader2,
  RefreshCw,
} from 'lucide-react'
import {
  canReport,
  fetchHkReports,
  groupRoomsByFloor,
  hkFetchMe,
  readStoredBranch,
  reportDateLabel,
  reportStateChip,
  reportStateCounts,
  resolveInitialBranch,
  signalRole,
  sortReportRooms,
  storeBranch,
  takeHkReportNotice,
  type Branch,
  type HkMe,
  type HkReportRoom,
} from '../hk-lib'
import { HkBranchChip, HkBranchesUnavailable, HkBranchPicker } from '../HkBranchPicker'
import { useHkAutoRefresh } from '../use-hk-auto-refresh'

export default function HkReportOverviewPage() {
  const [me, setMe] = useState<HkMe | null>(null)
  const [meError, setMeError] = useState<string | null>(null)
  const [branch, setBranch] = useState<Branch | null>(null)
  const [branchResolved, setBranchResolved] = useState(false)

  const [rooms, setRooms] = useState<HkReportRoom[]>([])
  const [date, setDate] = useState('')
  const [loading, setLoading] = useState(true)
  const [error, setError] = useState<string | null>(null)
  // The one-shot banner a landed submit/verify/return left behind before
  // navigating here. Read once, on mount, and cleared by the read itself.
  const [notice, setNotice] = useState<string | null>(null)

  useEffect(() => {
    setNotice(takeHkReportNotice())
  }, [])

  // Step 1: identity + which branches exist. Never guesses a branch.
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

  // Step 2: the day's reports. `background` is the auto-refresh path and is
  // deliberately quieter — no spinner, and a transient failure keeps the last
  // good list rather than painting a banner a maid in a lift lobby can do
  // nothing about. Same contract as the room list.
  const loadReports = useCallback(
    async (background = false) => {
      if (!branch) return
      if (!background) setLoading(true)
      try {
        const { date: day, rooms: rows } = await fetchHkReports(branch)
        setRooms(rows)
        setDate(day)
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
    if (branch) loadReports()
  }, [branch, loadReports])

  // Same "doesn't sync" fix the room list carries: this page is opened from a
  // LINE tile and the WebView keeps it alive for hours, so a maid's screen
  // would otherwise never show the report reception verified ten minutes ago.
  const refresh = useCallback(() => loadReports(true), [loadReports])
  useHkAutoRefresh(refresh, Boolean(branch))

  const pickBranch = (next: Branch) => {
    storeBranch(next)
    // Rooms AND the date come from the branch we asked about; carrying either
    // across would show the OTHER hotel's sheet for a beat.
    setRooms([])
    setDate('')
    setError(null)
    setBranch(next)
  }

  const role = signalRole(canReport(me))
  const counts = reportStateCounts(rooms)

  const showUnavailable = branchResolved && me && !meError && me.branches.length === 0
  const showPicker = branchResolved && me && !meError && !branch && !showUnavailable

  return (
    <main>
      {/* Back to the room list. The two /hk surfaces are siblings, not a
          hierarchy — a maid may open either tile first — so this is a plain
          lateral link rather than a breadcrumb. */}
      <Link href="/hk" className="mb-3 inline-flex items-center gap-1 text-sm text-gray-500">
        <ChevronLeft className="h-4 w-4" />
        กลับไปหน้ารายการห้อง
      </Link>

      <header className="mb-4 flex items-start justify-between gap-2">
        <div>
          <h1 className="flex items-center gap-2 text-xl font-bold">
            <ClipboardList className="h-6 w-6 text-teal-600" />
            รายงานประจำวัน
          </h1>
          {/* The day the SERVER answered for, never one this client assumed. */}
          {date && <p className="mt-1 text-sm text-gray-500">{reportDateLabel(date)}</p>}
        </div>
        {me && branch && (
          <HkBranchChip branches={me.branches} current={branch} onSwitch={pickBranch} />
        )}
      </header>

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
          {/* The banner the report screen stashed before navigating back. */}
          {notice && !error && (
            <div
              data-testid="hk-report-notice"
              className="mb-4 flex items-center gap-2 rounded-lg border border-emerald-200 bg-emerald-50 p-3 text-sm text-emerald-700"
            >
              <Check className="h-5 w-5 shrink-0" />
              <span>{notice}</span>
            </div>
          )}

          {error && (
            <div className="mb-4 flex items-start gap-2 rounded-lg border border-red-200 bg-red-50 p-3 text-sm text-red-700">
              <AlertCircle className="mt-0.5 h-5 w-5 shrink-0" />
              <span>{error}</span>
            </div>
          )}

          {/* Summary bar — the same shape and position as the room list's, so
              the two screens read as one surface. Each role's OWN number leads:
              a maid plans by what is still unsent, reception by what is waiting
              to be checked. */}
          {!error && (
            <div
              data-testid="hk-report-summary"
              className="mb-4 flex items-center justify-between rounded-xl border border-gray-200 bg-white px-4 py-3 text-sm"
            >
              <div className="flex flex-wrap gap-x-4 gap-y-1">
                {role === 'maid' ? (
                  <span className="text-gray-600">
                    ยังไม่ส่ง <strong>{counts.unsent}</strong>
                  </span>
                ) : (
                  <span className="text-amber-700">
                    รอตรวจ <strong>{counts.submitted}</strong>
                  </span>
                )}
                <span className="text-red-700">
                  ส่งกลับแก้ไข <strong>{counts.returned}</strong>
                </span>
                <span className="text-emerald-700">
                  ตรวจแล้ว <strong>{counts.verified}</strong>
                </span>
                <span className="text-gray-500">
                  ทั้งหมด <strong>{rooms.length}</strong>
                </span>
              </div>
              <button
                type="button"
                onClick={() => loadReports()}
                disabled={loading}
                aria-label="รีเฟรช"
                className="rounded-lg border border-gray-300 p-2 text-gray-500 active:bg-gray-100 disabled:opacity-50"
              >
                <RefreshCw className={`h-4 w-4 ${loading ? 'animate-spin' : ''}`} />
              </button>
            </div>
          )}

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
                {/* ONE COLUMN, unlike the room list's two-up grid: the state
                    chip carries a whole clause ("ส่งกลับแก้ไข: รูปไม่ชัดเจน")
                    and a half-width card would truncate exactly the word that
                    tells a maid why she is walking back. Sorted per role
                    WITHIN the floor, so the grouping still reads as a walking
                    order. */}
                <ul className="space-y-2">
                  {sortReportRooms(floorRooms, role).map((row) => {
                    const chip = reportStateChip(row.report)
                    return (
                      <li key={row.roomId}>
                        <Link
                          href={`/hk/rooms/${row.roomId}/report`}
                          data-testid={`hk-report-row-${row.roomId}`}
                          className="flex min-h-[44px] items-center justify-between gap-2 rounded-xl border border-gray-200 bg-white px-3 py-2 active:bg-gray-50"
                        >
                          <span className="flex flex-wrap items-center gap-2">
                            <span className="text-base font-bold">ห้อง {row.roomNo}</span>
                            <span
                              className={`inline-block rounded-full border px-2 py-0.5 text-xs ${chip.className}`}
                            >
                              {chip.label}
                            </span>
                          </span>
                          <ChevronRight className="h-5 w-5 shrink-0 text-gray-400" />
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
