'use client'

// Room screen (/hk/rooms/[roomId]) — report cleaning progress for one room.
// Part of the maid-facing housekeeping surface (employee-login plan Phase 4,
// wave-4 §A+B). แจ้งซ่อม / เบิกของ are NOT linked from here: both are
// top-level tiles on the LINE rich menu, and a second route to the same place
// is a cost for this audience. The reporter identity is stamped SERVER-SIDE
// from the verified Cloudflare Access assertion; nothing identity-like is
// sent from this form.
//
// The branch is never chosen here — only the room LIST page (/hk) offers the
// picker. This screen only ever READS what's already stored (§A1: never
// guess, never default); a missing or stale value sends the maid back to
// /hk to pick, rather than silently assuming a property. `markDirtyEnabled`
// comes from the same `/me` call and gates the third "แจ้งห้องไม่สะอาด" button
// (§B5 — off by default; the write shape is proven, the phone-as-trigger is
// new).

import { useCallback, useEffect, useState } from 'react'
import Link from 'next/link'
import { useParams } from 'next/navigation'
import {
  AlertCircle,
  AlertTriangle,
  Check,
  ChevronLeft,
  Info,
  Loader2,
  Sparkles,
} from 'lucide-react'
import {
  hkFetch,
  hkFetchMe,
  legacyStatusNote,
  markDirtyConfirmMessage,
  movementTags,
  occupancyIndicator,
  progressLabel,
  readStoredBranch,
  resolveInitialBranch,
  roomCleanChip,
  timeLabel,
  type Branch,
  type CleaningStatus,
  type HkMe,
  type HkRoomDetail,
} from '../../hk-lib'
import { useHkAutoRefresh } from '../../use-hk-auto-refresh'

export default function HkRoomPage() {
  const params = useParams<{ roomId: string }>()
  const roomId = Number(params?.roomId)

  const [branch, setBranch] = useState<Branch | null>(null)
  const [branchChecked, setBranchChecked] = useState(false)
  const [markDirtyEnabled, setMarkDirtyEnabled] = useState(false)

  const [detail, setDetail] = useState<HkRoomDetail | null>(null)
  const [loading, setLoading] = useState(true)
  const [error, setError] = useState<string | null>(null)
  const [posting, setPosting] = useState<CleaningStatus | null>(null)
  const [notice, setNotice] = useState<string | null>(null)
  // Two-step confirm for แจ้งห้องไม่สะอาด (wave-5 R2b). It used to fire on a
  // single tap, and a mis-tap flips a real room dirty in iHOTEL — reception
  // then sends someone to a room nobody asked about. `done` / `started` stay
  // single-tap: they are the normal flow, and a confirm on every tap is a
  // confirm nobody reads.
  const [confirmingDirty, setConfirmingDirty] = useState(false)

  // Resolve the already-chosen branch from storage; only /hk itself may pick
  // one. `resolveInitialBranch` still discards a stale value (branch removed
  // from HK_BRANCHES since it was stored) — that state renders the same
  // "go back and choose" notice as never having picked at all.
  useEffect(() => {
    let live = true
    hkFetchMe()
      .then(async (res) => {
        if (!res.ok) throw new Error()
        const data: HkMe = await res.json()
        if (!live || !data.success) return
        const resolved = resolveInitialBranch(data.branches, readStoredBranch())
        setMarkDirtyEnabled(Boolean(data.markDirtyEnabled))
        setBranch(resolved)
      })
      .catch(() => {
        /* branch stays null; the "go back" notice below covers this too */
      })
      .finally(() => {
        if (live) setBranchChecked(true)
      })
    return () => {
      live = false
    }
  }, [])

  // `background` is the auto-refresh path — same contract as the room list:
  // no spinner, and a transient failure keeps the last good screen instead of
  // raising a banner a maid in a lift lobby can do nothing about.
  const load = useCallback(
    async (background = false) => {
      if (!Number.isFinite(roomId)) {
        setError('ไม่พบห้องนี้')
        setLoading(false)
        return
      }
      if (!branch) return
      if (!background) setLoading(true)
      try {
        const res = await hkFetch(`/rooms/${roomId}`, branch)
        if (!res.ok) {
          throw new Error(res.status === 404 ? 'ไม่พบห้องนี้' : 'ไม่สามารถดึงข้อมูลห้องได้')
        }
        const data: HkRoomDetail = await res.json()
        if (!data.success) throw new Error('ไม่สามารถดึงข้อมูลห้องได้')
        setDetail(data)
        setError(null)
      } catch (err) {
        if (!background) setError(err instanceof Error ? err.message : 'เกิดข้อผิดพลาด')
      } finally {
        if (!background) setLoading(false)
      }
    },
    [roomId, branch]
  )

  useEffect(() => {
    if (branch) load()
  }, [branch, load])

  // Same "doesn't sync" fix as the list: reception can flip this very room in
  // iHOTEL while the maid has its screen open. DISABLED while her own report
  // is in flight — `reportCleaning` does its own `load()` on completion, and a
  // poll landing mid-POST could otherwise paint the pre-write answer back over
  // the room she just finished.
  const refreshDetail = useCallback(() => load(true), [load])
  useHkAutoRefresh(refreshDetail, Boolean(branch) && posting === null)

  const reportCleaning = async (status: CleaningStatus) => {
    if (!branch) return
    setPosting(status)
    setNotice(null)
    try {
      const res = await hkFetch(`/rooms/${roomId}/cleaning`, branch, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ status }),
      })
      if (!res.ok) throw new Error('บันทึกไม่สำเร็จ กรุณาลองใหม่')
      setNotice(
        status === 'done'
          ? 'บันทึกแล้ว: เสร็จแล้ว'
          : status === 'dirty'
            ? 'บันทึกแล้ว: แจ้งห้องไม่สะอาด'
            : 'บันทึกแล้ว: เริ่มทำความสะอาด'
      )
      await load()
    } catch (err) {
      setError(err instanceof Error ? err.message : 'เกิดข้อผิดพลาด')
    } finally {
      setPosting(null)
      // Collapse the confirm only once the request has SETTLED — so the
      // ยืนยัน button can carry the spinner while it is in flight, and a
      // failed report does not leave a re-armed one-tap button behind.
      setConfirmingDirty(false)
    }
  }

  // No valid branch stored — never guess one here. Send the maid back to the
  // room list, which is the only screen that may pick a branch.
  if (branchChecked && !branch) {
    return (
      <main>
        <div className="mb-4 flex items-start gap-2 rounded-lg border border-red-200 bg-red-50 p-3 text-sm text-red-700">
          <AlertCircle className="mt-0.5 h-5 w-5 shrink-0" />
          <span>ยังไม่ได้เลือกสาขา กรุณากลับไปหน้ารายการห้องเพื่อเลือกสาขาก่อน</span>
        </div>
        <Link href="/hk" className="inline-flex items-center gap-1 text-sm text-gray-500">
          <ChevronLeft className="h-4 w-4" />
          กลับไปหน้ารายการห้อง
        </Link>
      </main>
    )
  }

  if ((loading && !detail) || !branchChecked) {
    return (
      <main className="flex items-center justify-center py-16 text-gray-500">
        <Loader2 className="mr-2 h-6 w-6 animate-spin" />
        กำลังโหลด...
      </main>
    )
  }

  const room = detail?.room
  const badge = progressLabel(room?.cleaning?.status)
  const occupancy = occupancyIndicator(room?.occupancy)
  const tags = room ? movementTags(room) : []

  return (
    <main>
      {/* Back + header */}
      <Link href="/hk" className="mb-3 inline-flex items-center gap-1 text-sm text-gray-500">
        <ChevronLeft className="h-4 w-4" />
        กลับไปหน้ารายการห้อง
      </Link>

      {error && (
        <div className="mb-4 flex items-start gap-2 rounded-lg border border-red-200 bg-red-50 p-3 text-sm text-red-700">
          <AlertCircle className="mt-0.5 h-5 w-5 shrink-0" />
          <span>{error}</span>
        </div>
      )}

      {notice && !error && (
        <div className="mb-4 flex items-center gap-2 rounded-lg border border-emerald-200 bg-emerald-50 p-3 text-sm text-emerald-700">
          <Check className="h-5 w-5 shrink-0" />
          <span>{notice}</span>
        </div>
      )}

      {/* CR-1 fallback notice — same copy and same amber treatment as the room
          list, so the two screens never disagree about what she is looking at. */}
      {!error && legacyStatusNote(detail?.legacyStatusStale) && (
        <div
          role="status"
          className="mb-4 flex items-start gap-2 rounded-lg border border-amber-200 bg-amber-50 p-3 text-sm text-amber-800"
        >
          <Info className="mt-0.5 h-5 w-5 shrink-0" />
          <span>{legacyStatusNote(detail?.legacyStatusStale)}</span>
        </div>
      )}

      {room && (
        <>
          <header className="mb-4 rounded-xl border border-gray-200 bg-white p-4">
            <div className="flex items-center justify-between gap-2">
              <span className="flex items-center gap-2">
                <h1 className="text-2xl font-bold">ห้อง {room.roomNo}</h1>
                {/* Answers "can I enter" — guest occupancy, distinct from the
                    clean/dirty chips ("what work") on the right. */}
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
              </span>
              {/* Primary: explicit clean/dirty (merged iHOTEL-wins roomClean),
                  same words reception reads at /v2/housekeeping. Secondary:
                  today's maid-reported progress — both stay visible. */}
              <span className="flex items-center gap-1.5">
                <span
                  className={`inline-block rounded-full border px-2.5 py-1 text-xs ${roomCleanChip(room.roomClean).className}`}
                >
                  {roomCleanChip(room.roomClean).label}
                </span>
                <span
                  className={`inline-block rounded-full border px-2.5 py-1 text-xs ${badge.className}`}
                >
                  {badge.label}
                </span>
              </span>
            </div>
            {/* Day-scoped movement (arrivals/departures today) — a different
                axis from occupancy (right now) and the chips (what work).
                Departure first. Nothing rendered, no placeholder, when there
                is nothing to say today. */}
            {tags.length > 0 && (
              <div className="mt-2 flex flex-wrap gap-1">
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
            <p className="mt-1 text-sm text-gray-500">
              {room.floor !== null ? `ชั้น ${room.floor}` : ''}
              {room.building ? ` · ${room.building}` : ''}
            </p>
          </header>

          {/* Cleaning progress buttons */}
          <section className="mb-6">
            <h2 className="mb-2 flex items-center gap-1.5 text-sm font-semibold text-gray-600">
              <Sparkles className="h-4 w-4 text-red-600" />
              รายงานความคืบหน้า
            </h2>
            <div className="grid grid-cols-2 gap-3">
              <button
                type="button"
                onClick={() => reportCleaning('started')}
                disabled={posting !== null}
                className="rounded-xl border border-amber-300 bg-amber-50 px-3 py-4 text-base font-semibold text-amber-800 active:bg-amber-100 disabled:opacity-50"
              >
                {posting === 'started' ? (
                  <Loader2 className="mx-auto h-5 w-5 animate-spin" />
                ) : (
                  'เริ่มทำความสะอาด'
                )}
              </button>
              <button
                type="button"
                onClick={() => reportCleaning('done')}
                disabled={posting !== null}
                className="rounded-xl border border-emerald-300 bg-emerald-50 px-3 py-4 text-base font-semibold text-emerald-800 active:bg-emerald-100 disabled:opacity-50"
              >
                {posting === 'done' ? (
                  <Loader2 className="mx-auto h-5 w-5 animate-spin" />
                ) : (
                  'เสร็จแล้ว'
                )}
              </button>
            </div>

            {/* Mark-dirty (§B5): dark-shipped, HK_MARK_DIRTY_ENABLED-gated via
                /api/hk/me. Hidden entirely rather than offered as a dead tap
                that would 403. */}
            {markDirtyEnabled &&
              (confirmingDirty ? (
                /* Step 2 — the confirm. Rendered IN PLACE of the button, not
                   over it: a modal/`window.confirm` on a phone in a corridor is
                   easy to dismiss by reflex, while replacing the control forces
                   a deliberate second tap. ยกเลิก is listed FIRST and is the
                   visually calmer option, so the reflex tap is the safe one. */
                <div className="mt-3 rounded-xl border border-red-300 bg-red-50 p-3">
                  <p className="mb-3 flex items-start gap-1.5 text-sm font-semibold text-red-800">
                    <AlertTriangle className="mt-0.5 h-4 w-4 shrink-0" />
                    <span>{markDirtyConfirmMessage(room.roomNo)}</span>
                  </p>
                  <div className="grid grid-cols-2 gap-3">
                    <button
                      type="button"
                      onClick={() => setConfirmingDirty(false)}
                      disabled={posting !== null}
                      className="rounded-lg border border-gray-300 bg-white px-3 py-3 text-sm font-semibold text-gray-700 active:bg-gray-100 disabled:opacity-50"
                    >
                      ยกเลิก
                    </button>
                    <button
                      type="button"
                      onClick={() => reportCleaning('dirty')}
                      disabled={posting !== null}
                      className="flex items-center justify-center rounded-lg border border-red-400 bg-red-600 px-3 py-3 text-sm font-semibold text-white active:bg-red-700 disabled:opacity-50"
                    >
                      {posting === 'dirty' ? (
                        <Loader2 className="h-5 w-5 animate-spin" />
                      ) : (
                        'ยืนยัน'
                      )}
                    </button>
                  </div>
                </div>
              ) : (
                /* Step 1 — arms the confirm. Fires NO request. */
                <button
                  type="button"
                  onClick={() => setConfirmingDirty(true)}
                  disabled={posting !== null}
                  className="mt-3 flex w-full items-center justify-center gap-1.5 rounded-xl border border-red-300 bg-red-50 px-3 py-3 text-sm font-semibold text-red-800 active:bg-red-100 disabled:opacity-50"
                >
                  <AlertTriangle className="h-4 w-4" />
                  แจ้งห้องไม่สะอาด
                </button>
              ))}

            {detail && detail.events.length > 0 && (
              <ul className="mt-3 space-y-1 text-xs text-gray-500">
                {detail.events.map((ev) => (
                  <li key={ev.eventId}>
                    {timeLabel(ev.at)} — {progressLabel(ev.status).label}
                    {ev.name ? ` โดย ${ev.name}` : ` (${ev.badge})`}
                  </li>
                ))}
              </ul>
            )}
          </section>
        </>
      )}
    </main>
  )
}
