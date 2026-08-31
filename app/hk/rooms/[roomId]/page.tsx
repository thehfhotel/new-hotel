'use client'

// Room screen (/hk/rooms/[roomId]) — report cleaning progress for one room.
// Part of the maid-facing housekeeping surface (employee-login plan Phase 4,
// wave-4 §A+B). แจ้งซ่อม / เบิกของ are NOT linked from here: both are
// top-level tiles on the LINE rich menu, and a second route to the same place
// is a cost for this audience. แจ้งขาดผ้า IS here, for the opposite reason: a
// linen shortage is a fact about THIS room, discovered while standing in it,
// and the room is already on screen. The reporter identity is stamped
// SERVER-SIDE from the verified Cloudflare Access assertion; nothing
// identity-like is sent from this form.
//
// The branch is never chosen here — only the room LIST page (/hk) offers the
// picker. This screen only ever READS what's already stored (§A1: never
// guess, never default); a missing or stale value sends the maid back to
// /hk to pick, rather than silently assuming a property. `markDirtyEnabled`
// comes from the same `/me` call and gates the third "แจ้งห้องไม่สะอาด" button
// (§B5 — off by default; the write shape is proven, the phone-as-trigger is
// new).
//
// VIEWER MODE. `/hk` now admits two grants: `housekeeping` (a maid — full
// access) and `reception` (read-only). `canReport` from the same `/me` call
// gates the whole ACTION surface below — the progress buttons, the mark-dirty
// block and แจ้งขาดผ้า — while every informational part of the screen (chips,
// today's linen totals, the event list, the stale notice) stays exactly as a
// maid sees it: reception's reason for opening this screen is to READ it. The
// hiding is UX only; the server refuses a viewer's POST with 403 regardless.

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
  Minus,
  Plus,
  Shirt,
  Sparkles,
} from 'lucide-react'
import {
  canReport,
  clampLinenQty,
  emptyLinenCounts,
  hkFetch,
  hkFetchMe,
  legacyStatusNote,
  LINEN_KINDS,
  LINEN_MAX_QTY,
  LINEN_MIN_QTY,
  linenShortageItems,
  linenShortageSummary,
  linenShortageTag,
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
  type HkLinenShortageResponse,
  type HkMe,
  type HkRoomDetail,
  type LinenCounts,
  type LinenKind,
  type LinenShortageItem,
} from '../../hk-lib'
import { useHkAutoRefresh } from '../../use-hk-auto-refresh'

export default function HkRoomPage() {
  const params = useParams<{ roomId: string }>()
  const roomId = Number(params?.roomId)

  const [branch, setBranch] = useState<Branch | null>(null)
  const [branchChecked, setBranchChecked] = useState(false)
  const [markDirtyEnabled, setMarkDirtyEnabled] = useState(false)
  // Viewer mode, from the same `/me` call. Initial value TRUE, unlike
  // `markDirtyEnabled` above, because the skew rule for this one runs the other
  // way (see `canReport` in hk-lib): a missing answer means "maid". Nothing
  // renders before `/me` settles anyway — the screen is on its spinner until
  // `branchChecked` — so this initial value never reaches the glass.
  const [canReportFlag, setCanReportFlag] = useState(true)

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

  // แจ้งขาดผ้า (linen shortage). Same in-place expand as the confirm above: the
  // first tap opens a panel and fires NO request, because "how many of what" is
  // a question the maid answers ON the screen, not a decision the button can
  // make for her. Counts live as ONE record rather than per-row state so ยกเลิก
  // resets the whole form in a single move, and so the submit button can tell
  // "nothing selected" from "something selected" without walking the DOM.
  const [linenOpen, setLinenOpen] = useState(false)
  const [linenCounts, setLinenCounts] = useState<LinenCounts>(emptyLinenCounts)
  const [linenPosting, setLinenPosting] = useState(false)

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
        setCanReportFlag(canReport(data))
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
  // the room she just finished. The linen report is gated for the same reason
  // AND a second one: its panel is a form the maid is still filling in, and a
  // background re-render mid-submit is exactly when a half-counted report gets
  // lost.
  const refreshDetail = useCallback(() => load(true), [load])
  useHkAutoRefresh(refreshDetail, Boolean(branch) && posting === null && !linenPosting)

  const reportCleaning = async (status: CleaningStatus) => {
    if (!branch) return
    // A viewer has no button to reach this with; the guard is here for the same
    // reason `hkFetch` refuses a null branch — a caller bug must become nothing
    // at all, never a request the server has to refuse.
    if (!canReportFlag) return
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

  const stepLinen = (kind: LinenKind, delta: number) => {
    // Clamped in the reducer, not only on the buttons' `disabled`: the bounds
    // are the CONTRACT (1..LINEN_MAX_QTY on the wire), and a bound that only
    // exists as a disabled attribute is one double-tap away from not existing.
    setLinenCounts((prev) => ({ ...prev, [kind]: clampLinenQty(prev[kind] + delta) }))
  }

  const closeLinenPanel = () => {
    setLinenOpen(false)
    setLinenCounts(emptyLinenCounts())
  }

  const reportLinenShortage = async (items: LinenShortageItem[]) => {
    if (!branch) return
    // Same viewer guard as `reportCleaning` — the panel that calls this is not
    // rendered for a viewer at all.
    if (!canReportFlag) return
    // Belt to the disabled button's braces — an empty report is an errand for
    // nobody, and must never reach the linen room as a no-op row.
    if (items.length === 0) return
    setLinenPosting(true)
    setNotice(null)
    try {
      const res = await hkFetch(`/rooms/${roomId}/linen-shortage`, branch, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ items }),
      })
      // Unlike the cleaning POST, a 2xx alone is not the answer here: the
      // contract carries an explicit `success`, and a 200 that says `false` is
      // a report that did NOT land. Showing the green banner for it would send
      // the maid away believing the linen room had been told.
      const body: HkLinenShortageResponse | null = res.ok
        ? await res.json().catch(() => null)
        : null
      if (!res.ok || !body?.success) throw new Error('บันทึกไม่สำเร็จ กรุณาลองใหม่')
      // Clear a previous failure's banner, or the green notice below it would
      // stay hidden behind the red one on a successful retry.
      setError(null)
      setNotice('บันทึกแล้ว: แจ้งขาดผ้า')
      // Only a landed report resets the form and collapses the panel. On a
      // failure the counts stay exactly as she entered them — retyping six
      // steppers in a corridor is how a report stops getting filed at all.
      closeLinenPanel()
      // Same reload `reportCleaning` does, for the same reason: the report she
      // just filed is now a fact about this room (the ขาดผ้า chip and today's
      // totals line), and a screen that still shows the pre-report room would
      // invite her to file it a second time. Awaited inside the try so the
      // in-flight lock covers it, exactly as the cleaning path does.
      await load()
    } catch (err) {
      setError(err instanceof Error ? err.message : 'เกิดข้อผิดพลาด')
    } finally {
      setLinenPosting(false)
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
  const linenTag = room ? linenShortageTag(room) : null
  // Today's totals behind that tag ("วันนี้แจ้งขาดผ้า: ปลอกหมอน 2, …"), or
  // null when nothing was reported today / the backend sends no totals.
  const linenSummary = linenShortageSummary(detail?.linenShortages)
  // ONE in-flight report at a time, whichever kind it is: two reports on one
  // room from one thumb is never what she meant, and the auto-refresh gate
  // above already treats both the same way.
  const busy = posting !== null || linenPosting
  // What ส่งแจ้ง would send right now — also the answer to "is anything
  // selected", so the button and the request can never disagree.
  const linenItems = linenShortageItems(linenCounts)

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
                {/* Third chip, when today carries a linen report — same chip,
                    same words, same row position as the room list, so the two
                    screens can never disagree about this room. Rendered beside
                    EVERY cleaning state, เสร็จแล้ว included. */}
                {linenTag && (
                  <span
                    className={`inline-block rounded-full border px-2.5 py-1 text-xs ${linenTag.className}`}
                  >
                    {linenTag.label}
                  </span>
                )}
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

          {/* Cleaning progress buttons, then what has already been reported. */}
          <section className="mb-6">
            {/* THE ACTION SURFACE — every control that can FILE something,
                hidden WHOLE for a read-only (reception) viewer rather than
                disabled: a greyed-out row of buttons reads as "broken" and
                invites a support call, while their absence reads as "not my
                job", which is exactly right. Everything informational below
                this block stays, for both audiences. UX only — the server 403s
                a viewer's POST either way. */}
            {canReportFlag && (
              <>
                <h2 className="mb-2 flex items-center gap-1.5 text-sm font-semibold text-gray-600">
                  <Sparkles className="h-4 w-4 text-red-600" />
                  รายงานความคืบหน้า
                </h2>
                <div className="grid grid-cols-2 gap-3">
                  <button
                    type="button"
                    onClick={() => reportCleaning('started')}
                    disabled={busy}
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
                    disabled={busy}
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
                          disabled={busy}
                          className="rounded-lg border border-gray-300 bg-white px-3 py-3 text-sm font-semibold text-gray-700 active:bg-gray-100 disabled:opacity-50"
                        >
                          ยกเลิก
                        </button>
                        <button
                          type="button"
                          onClick={() => reportCleaning('dirty')}
                          disabled={busy}
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
                      disabled={busy}
                      className="mt-3 flex w-full items-center justify-center gap-1.5 rounded-xl border border-red-300 bg-red-50 px-3 py-3 text-sm font-semibold text-red-800 active:bg-red-100 disabled:opacity-50"
                    >
                      <AlertTriangle className="h-4 w-4" />
                      แจ้งห้องไม่สะอาด
                    </button>
                  ))}

                {/* แจ้งขาดผ้า (linen shortage). Expands IN PLACE of its button, the
                    same two-step shape as the confirm above and for the same
                    reason: a maid counting missing towels in a doorway needs the
                    room she is looking at to stay on screen behind the form, and a
                    modal on a phone is dismissed by reflex. Sky-toned so it reads
                    as neither the red destructive report nor the amber/emerald
                    progress pair — a different KIND of report, not a third
                    severity. Not gated by `markDirtyEnabled`: nothing here writes
                    to iHOTEL. */}
                {linenOpen ? (
                  <div className="mt-3 rounded-xl border border-sky-300 bg-sky-50 p-3">
                    <p className="mb-3 flex items-center gap-1.5 text-sm font-semibold text-sky-900">
                      <Shirt className="h-4 w-4 shrink-0" />
                      <span>แจ้งขาดผ้า</span>
                    </p>
                    <ul className="space-y-2">
                      {LINEN_KINDS.map(({ kind, label }) => (
                        <li
                          key={kind}
                          className="flex items-center justify-between gap-2 rounded-lg border border-sky-200 bg-white px-3 py-2"
                        >
                          <span className="text-sm font-medium text-gray-800">{label}</span>
                          {/* h-11/w-11 is 44px — the smallest target a thumb hits
                              reliably, and why these are squares rather than the
                              py-3 pills the rest of the screen uses. The aria-label
                              names the kind: "+" alone tells a screen reader (and a
                              test) nothing about WHICH row it stepped. */}
                          <span className="flex items-center gap-2">
                            <button
                              type="button"
                              aria-label={`ลด ${label}`}
                              onClick={() => stepLinen(kind, -1)}
                              disabled={busy || linenCounts[kind] <= LINEN_MIN_QTY}
                              className="flex h-11 w-11 items-center justify-center rounded-lg border border-sky-300 bg-white text-sky-800 active:bg-sky-100 disabled:opacity-40"
                            >
                              <Minus className="h-5 w-5" />
                            </button>
                            <span className="w-7 text-center text-base font-semibold tabular-nums text-gray-900">
                              {linenCounts[kind]}
                            </span>
                            <button
                              type="button"
                              aria-label={`เพิ่ม ${label}`}
                              onClick={() => stepLinen(kind, 1)}
                              disabled={busy || linenCounts[kind] >= LINEN_MAX_QTY}
                              className="flex h-11 w-11 items-center justify-center rounded-lg border border-sky-300 bg-white text-sky-800 active:bg-sky-100 disabled:opacity-40"
                            >
                              <Plus className="h-5 w-5" />
                            </button>
                          </span>
                        </li>
                      ))}
                    </ul>
                    {/* ยกเลิก first and calmer, same as the dirty confirm — the
                        reflex tap is the one that files nothing. */}
                    <div className="mt-3 grid grid-cols-2 gap-3">
                      <button
                        type="button"
                        onClick={closeLinenPanel}
                        disabled={busy}
                        className="rounded-lg border border-gray-300 bg-white px-3 py-3 text-sm font-semibold text-gray-700 active:bg-gray-100 disabled:opacity-50"
                      >
                        ยกเลิก
                      </button>
                      <button
                        type="button"
                        onClick={() => reportLinenShortage(linenItems)}
                        disabled={busy || linenItems.length === 0}
                        className="flex items-center justify-center rounded-lg border border-sky-500 bg-sky-600 px-3 py-3 text-sm font-semibold text-white active:bg-sky-700 disabled:opacity-50"
                      >
                        {linenPosting ? <Loader2 className="h-5 w-5 animate-spin" /> : 'ส่งแจ้ง'}
                      </button>
                    </div>
                  </div>
                ) : (
                  /* Opens the form. Fires NO request. */
                  <button
                    type="button"
                    onClick={() => setLinenOpen(true)}
                    disabled={busy}
                    className="mt-3 flex w-full items-center justify-center gap-1.5 rounded-xl border border-sky-300 bg-sky-50 px-3 py-3 text-sm font-semibold text-sky-800 active:bg-sky-100 disabled:opacity-50"
                  >
                    <Shirt className="h-4 w-4" />
                    แจ้งขาดผ้า
                  </button>
                )}
              </>
            )}

            {/* What has ALREADY been reported for this room today — the detail
                behind the ขาดผ้า chip in the header, in the same muted register
                as the cleaning-event list below. For a maid it sits directly
                under the button that files a report, so she can see before she
                counts anything that the morning shift already asked for two
                pillowcases; for a viewer it is one of the facts she opened the
                screen to read. Rendered for BOTH, only when there is something
                to say. */}
            {linenSummary && (
              <p className="mt-2 text-xs text-gray-500">{linenSummary}</p>
            )}

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
