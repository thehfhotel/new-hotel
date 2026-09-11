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
// block, แจ้งขาดผ้า and เติมผ้าแล้ว — while every informational part of the
// screen (chips, the ขาดผ้าค้างอยู่ card and its kinds, today's linen totals,
// the event list, the stale notice) stays exactly as a maid sees it:
// reception's reason for opening this screen is to READ it. The hiding is UX
// only; the server refuses a viewer's POST with 403 regardless.

import { useCallback, useEffect, useRef, useState } from 'react'
import Link from 'next/link'
import { useParams } from 'next/navigation'
import {
  AlertCircle,
  AlertTriangle,
  Bell,
  Check,
  ChevronLeft,
  ClipboardList,
  Info,
  Loader2,
  Minus,
  Plus,
  Send,
  Shirt,
  Sparkles,
} from 'lucide-react'
import {
  actOnHkSignal,
  answerHkRoomCheck,
  canActOnSignal,
  canCancelSignal,
  canReport,
  clampLinenQty,
  emptyLinenCounts,
  fetchHkSignals,
  hkFetch,
  hkFetchMe,
  isIncomingSignal,
  isRoomCheck,
  legacyStatusNote,
  LINEN_KINDS,
  LINEN_MAX_QTY,
  LINEN_MIN_QTY,
  LINEN_OPEN_CARD_TITLE,
  linenResolveConfirmMessage,
  linenShortageItems,
  linenShortageSummary,
  linenShortageTag,
  markDirtyConfirmMessage,
  mergeSignal,
  mergeSignals,
  movementTags,
  occupancyIndicator,
  openLinenRows,
  progressLabel,
  readStoredBranch,
  resolveHkLinenShortage,
  resolveInitialBranch,
  roomCleanChip,
  ROOM_CHECK_PROBLEMS,
  sendableSignals,
  sendHkSignal,
  signalActorLabel,
  signalLabel,
  signalOriginLabel,
  signalRole,
  signalStatusLabel,
  signalsForRoom,
  timeLabel,
  type Branch,
  type CleaningStatus,
  type HkLinenShortageResponse,
  type HkMe,
  type HkRoomDetail,
  type LinenCounts,
  type LinenKind,
  type LinenShortageItem,
  type RoomCheckProblem,
  type RoomSignal,
  type SignalType,
} from '../../hk-lib'
import { useHkAutoRefresh } from '../../use-hk-auto-refresh'
import { playHkSignalCue, useHkSignalEvents } from '../../use-hk-signal-events'

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

  // เติมผ้าแล้ว (resolve every open shortage for this room). Two-step, the same
  // in-place idiom as แจ้งห้องไม่สะอาด and for a comparable reason: one tap
  // clears this room's ขาดผ้า flag for EVERYONE — the list's queue panel and
  // reception's viewer included — and there is no un-resolve on this phone.
  // Unlike แจ้งขาดผ้า, which asks a question the panel exists to answer, this
  // one has nothing to fill in, so the second step IS the confirm.
  const [confirmingLinenResolve, setConfirmingLinenResolve] = useState(false)
  const [linenResolving, setLinenResolving] = useState(false)

  // Room signals (ADR 0008). The branch's whole live list is held — that is
  // what both `/signals` and the SSE stream deliver — and `signalsForRoom`
  // narrows it to this room for rendering. A ref mirrors it so the sound cue
  // can tell a NEW arrival from an echo without waiting for a state flush.
  const [signals, setSignals] = useState<RoomSignal[]>([])
  const signalsRef = useRef<RoomSignal[]>([])
  // WHICH signal write is in flight, as `"<action>:<id|type>"` — one at a
  // time, for the same reason the two report paths above share a lock (two
  // acks from one thumb is never what she meant), and keyed by action rather
  // than by row so the spinner lands on the exact button that was tapped.
  const [signalBusy, setSignalBusy] = useState<string | null>(null)
  // The send panel expands in place, exactly like แจ้งขาดผ้า: the first tap
  // opens a list of canned types and files NOTHING.
  const [sendOpen, setSendOpen] = useState(false)
  // Per-ขอเช็คห้อง problem toggles, keyed by signalId — the maid may find both
  // มีของหาย and มีของเสียหาย in one room, and the answer carries both.
  const [problemDraft, setProblemDraft] = useState<Record<number, RoomCheckProblem[]>>({})

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
  useHkAutoRefresh(
    refreshDetail,
    Boolean(branch) &&
      posting === null &&
      !linenPosting &&
      !linenResolving &&
      signalBusy === null
  )

  // --- room signals (ADR 0008) ---------------------------------------------
  //
  // Their own request, their own failure story: a signals read that fails must
  // cost the signal list, never the room screen. Nothing in this block raises
  // the page's error banner on a READ — only on an action the user just took,
  // where silence would be a lie.
  const role = signalRole(canReportFlag)

  const applySignals = useCallback((next: RoomSignal[]) => {
    signalsRef.current = next
    setSignals(next)
  }, [])

  const loadSignals = useCallback(async () => {
    if (!branch) return
    try {
      applySignals(await fetchHkSignals(branch))
    } catch {
      /* keep what is on screen; the stream or the poll will catch up */
    }
  }, [branch, applySignals])

  useEffect(() => {
    if (branch) loadSignals()
  }, [branch, loadSignals])

  const handleSignal = useCallback(
    (signal: RoomSignal) => {
      const isNew = !signalsRef.current.some((s) => s.signalId === signal.signalId)
      applySignals(mergeSignal(signalsRef.current, signal))
      // Only a new signal aimed at THIS role, and only for THIS room: a maid
      // standing in 104 does not need a chime for the desk's request about
      // 312 — she is looking at 104, and the list screen already carries the
      // branch-wide cue.
      if (isNew && signal.roomId === roomId && isIncomingSignal(signal, role)) {
        playHkSignalCue()
      }
    },
    [role, roomId, applySignals]
  )

  const signalsLive = useHkSignalEvents(branch, handleSignal, Boolean(branch))
  useHkAutoRefresh(loadSignals, Boolean(branch) && !signalsLive && signalBusy === null)

  /** Run one signal write, merge whatever the server hands back, and say so.
   * Every signal action goes through here so the busy lock, the banners and
   * the merge cannot drift apart between five call sites. */
  const runSignalAction = async (
    key: string,
    successNotice: string,
    action: () => Promise<RoomSignal[]>
  ) => {
    if (!branch) return
    setSignalBusy(key)
    setNotice(null)
    try {
      const updated = await action()
      applySignals(mergeSignals(signalsRef.current, updated))
      setError(null)
      setNotice(successNotice)
    } catch (err) {
      setError(err instanceof Error ? err.message : 'เกิดข้อผิดพลาด')
    } finally {
      setSignalBusy(null)
    }
  }

  const sendSignal = (type: SignalType, label: string) =>
    runSignalAction(`send:${type}`, `ส่งแล้ว: ${label}`, async () => {
      const signal = await sendHkSignal(branch, roomId, type)
      // Only a landed signal closes the panel — a failure leaves it open so
      // the retry is one tap, not four.
      setSendOpen(false)
      return [signal]
    })

  const ackSignal = (signal: RoomSignal) =>
    runSignalAction(`ack:${signal.signalId}`, 'บันทึกแล้ว: รับทราบ', async () => [
      await actOnHkSignal(branch, signal.signalId, 'ack'),
    ])

  const doneSignal = (signal: RoomSignal) =>
    runSignalAction(`done:${signal.signalId}`, 'บันทึกแล้ว: เสร็จสิ้น', async () => [
      await actOnHkSignal(branch, signal.signalId, 'done'),
    ])

  const cancelSignal = (signal: RoomSignal) =>
    runSignalAction(`cancel:${signal.signalId}`, 'ยกเลิกแล้ว', async () => [
      await actOnHkSignal(branch, signal.signalId, 'cancel'),
    ])

  const answerRoomCheck = (signal: RoomSignal, problems: RoomCheckProblem[]) =>
    runSignalAction(
      problems.length === 0
        ? `answer-clear:${signal.signalId}`
        : `answer-problems:${signal.signalId}`,
      problems.length === 0 ? 'ส่งคำตอบแล้ว: เคลียร์' : 'ส่งคำตอบแล้ว',
      async () => {
        const { signal: answered, spawned } = await answerHkRoomCheck(
          branch,
          signal.signalId,
          problems.length === 0
            ? { outcome: 'clear' }
            : { outcome: 'problems', problems }
        )
        // The draft has done its job; a stale set of toggles behind a
        // completed check is a trap for the next one on the same room.
        setProblemDraft((prev) => {
          const next = { ...prev }
          delete next[signal.signalId]
          return next
        })
        // The children (one per problem) come back in the SAME transaction and
        // must appear together with the answered check's removal.
        return [answered, ...spawned]
      }
    )

  const toggleProblem = (signalId: number, problem: RoomCheckProblem) => {
    setProblemDraft((prev) => {
      const current = prev[signalId] ?? []
      const next = current.includes(problem)
        ? current.filter((p) => p !== problem)
        : // Kept in ROOM_CHECK_PROBLEMS order, so the body reads the same way
          // the buttons did however they were tapped.
          ROOM_CHECK_PROBLEMS.filter(
            ({ type }) => type === problem || current.includes(type)
          ).map(({ type }) => type)
      return { ...prev, [signalId]: next }
    })
  }

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
      // A เสร็จแล้ว report auto-completes this room's open/acked ทำห้องนี้ก่อน
      // and แขกเช็คเอาท์แล้ว server-side, in the SAME transaction (ADR 0008 /
      // CONTEXT.md §Housekeeping). Those rows are now done, so re-read them —
      // otherwise the maid is left looking at requests she has just satisfied
      // and will tap เสร็จสิ้น on each of them for nothing.
      if (status === 'done') await loadSignals()
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

  /**
   * เติมผ้าแล้ว — closes EVERY open shortage row for this room in one call
   * (room-level by design: a maid restocks a room in one trip). Mirrors
   * `reportLinenShortage` exactly — the same viewer guard, the same
   * success-only banner, the same `load()` behind it — because the two are
   * halves of one loop and a maid should not be able to tell them apart by
   * how they behave.
   */
  const resolveLinen = async () => {
    if (!branch) return
    // Same guard as the two report paths: a viewer has no button to reach this
    // with, and a caller bug must become nothing at all rather than a request
    // the server has to refuse.
    if (!canReportFlag) return
    setLinenResolving(true)
    setNotice(null)
    try {
      // A `resolved: 0` answer is a SUCCESS, not a no-op to complain about:
      // another maid may have restocked the room thirty seconds ago, and the
      // right outcome for this tap is still "the room is stocked".
      await resolveHkLinenShortage(branch, roomId)
      setError(null)
      setNotice('บันทึกแล้ว: เติมผ้าแล้ว')
      // The card, the header chip and the list's queue row all hang off the
      // same re-read the report path uses.
      await load()
    } catch (err) {
      setError(err instanceof Error ? err.message : 'เกิดข้อผิดพลาด')
    } finally {
      setLinenResolving(false)
      // Collapse only once the request has SETTLED, so ยืนยัน can carry the
      // spinner and a failure never leaves a re-armed one-tap button behind.
      setConfirmingLinenResolve(false)
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
  // What is STILL owed to this room, whatever day it was asked for — the task
  // card's rows. `[]` for an empty list AND for a backend that does not send
  // the field, which is what keeps the old totals line rendering under a
  // rolled-back backend instead of a card whose button has no endpoint.
  const openLinen = openLinenRows(detail?.linenShortagesOpen)
  // ONE in-flight report at a time, whichever kind it is: two reports on one
  // room from one thumb is never what she meant, and the auto-refresh gate
  // above already treats them the same way. เติมผ้าแล้ว joins the same lock —
  // resolving a room while reporting more shortage on it is a race with itself.
  const busy = posting !== null || linenPosting || linenResolving
  // What ส่งแจ้ง would send right now — also the answer to "is anything
  // selected", so the button and the request can never disagree.
  const linenItems = linenShortageItems(linenCounts)

  // Signals for THIS room, oldest first — the order they must be worked. The
  // signal controls have their own lock (`signalBusy`) rather than sharing
  // `busy`: acking the desk's ขอเช็คห้อง while a linen report is in flight is
  // a perfectly sensible thing to do, and a guest is waiting on it.
  const roomSignals = signalsForRoom(signals, roomId)
  const signalsBusy = signalBusy !== null
  const sendable = sendableSignals(role)
  const sendPanelLabel = role === 'maid' ? 'แจ้งแผนกต้อนรับ' : 'แจ้งแม่บ้าน'

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
                {/* Third chip, when this room has an OPEN linen shortage —
                    same chip, same words, same row position as the room list,
                    so the two screens can never disagree about this room. It
                    now clears on เติมผ้าแล้ว rather than on the day rolling
                    over, and is rendered beside EVERY cleaning state,
                    เสร็จแล้ว included. */}
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

          {/* ------------------------------------------------------------- *
           * Report HK — this room's daily report (Report HK.xlsx digitized).
           * A different artifact from everything else on this screen: the
           * progress buttons and the signals are about work in flight, while
           * the report is the day's ATTESTATION about this room, filed once
           * and countersigned by reception.
           *
           * Rendered for BOTH roles and placed directly under the header,
           * above the signals: it is the one control here whose destination
           * depends on who is looking (a maid files, reception verifies), and
           * it must not be something either of them has to scroll for. Teal,
           * the hue this feature owns across /hk.
           * ------------------------------------------------------------- */}
          <Link
            href={`/hk/rooms/${roomId}/report`}
            data-testid="hk-room-report-entry"
            className="mb-6 flex min-h-[44px] w-full items-center justify-center gap-1.5 rounded-xl border border-teal-300 bg-teal-50 px-3 py-3 text-sm font-semibold text-teal-800 active:bg-teal-100"
          >
            <ClipboardList className="h-4 w-4" />
            รายงานประจำวัน
          </Link>

          {/* ------------------------------------------------------------- *
           * ROOM SIGNALS (ADR 0008). Canned, one-room notices between the
           * desk and the maids — deliberately NOT a chat: there is no free
           * text here, and the ADR's Consequences section asks anyone tempted
           * to add some to read its Context first.
           *
           * Placed ABOVE the cleaning buttons on purpose. For a maid the top
           * item is frequently ขอเช็คห้อง, which means a guest is standing at
           * the reception counter waiting for her answer — the most urgent
           * thing on this screen, and it must not sit below a form. For a
           * reception viewer this section IS the screen: it is the only place
           * she can act, and everything below it is read-only for her.
           * ------------------------------------------------------------- */}
          <section className="mb-6" data-testid="hk-signals">
            <h2 className="mb-2 flex items-center gap-1.5 text-sm font-semibold text-gray-600">
              <Bell className="h-4 w-4 text-indigo-600" />
              งานแจ้งของห้องนี้
            </h2>

            {roomSignals.length === 0 ? (
              <p className="text-xs text-gray-400">ยังไม่มีงานแจ้งของห้องนี้</p>
            ) : (
              <ul className="space-y-2">
                {roomSignals.map((signal) => {
                  const actionable = canActOnSignal(signal, role)
                  const cancellable = canCancelSignal(signal, role)
                  const problems = problemDraft[signal.signalId] ?? []
                  const sender = signalActorLabel(signal.createdBy)
                  return (
                    <li
                      key={signal.signalId}
                      data-testid={`hk-signal-${signal.signalId}`}
                      className={`rounded-xl border p-3 ${
                        isRoomCheck(signal)
                          ? 'border-indigo-300 bg-indigo-50'
                          : 'border-gray-200 bg-white'
                      }`}
                    >
                      <p className="text-base font-semibold text-gray-900">
                        {signalLabel(signal.type)}
                      </p>
                      <p className="mt-0.5 text-xs text-gray-500">
                        {signalOriginLabel(signal, role)}
                        {sender ? ` · ${sender}` : ''}
                        {` · ${timeLabel(signal.createdAt)}`}
                      </p>
                      <p
                        className={`text-xs ${
                          signal.status === 'acked' ? 'text-emerald-700' : 'text-gray-500'
                        }`}
                      >
                        {signalStatusLabel(signal)}
                      </p>

                      {/* ขอเช็คห้อง cannot be closed by a bare tap: the desk
                          needs an ANSWER (CONTEXT.md §Housekeeping). เคลียร์
                          is one tap because it is the common case and a guest
                          is waiting; the two problem toggles are laid out flat
                          beside it rather than behind an expand, for the same
                          reason. ส่งคำตอบ stays dead until at least one is on,
                          so it can never send an empty `problems` list. */}
                      {actionable && isRoomCheck(signal) && (
                        <div className="mt-3 space-y-2">
                          <button
                            type="button"
                            onClick={() => answerRoomCheck(signal, [])}
                            disabled={signalsBusy}
                            className="flex min-h-[44px] w-full items-center justify-center rounded-lg border border-emerald-400 bg-emerald-600 px-3 py-3 text-sm font-semibold text-white active:bg-emerald-700 disabled:opacity-50"
                          >
                            {signalBusy === `answer-clear:${signal.signalId}` ? (
                              <Loader2 className="h-5 w-5 animate-spin" />
                            ) : (
                              'เคลียร์'
                            )}
                          </button>
                          <div className="grid grid-cols-2 gap-2">
                            {ROOM_CHECK_PROBLEMS.map(({ type, label }) => {
                              const on = problems.includes(type)
                              return (
                                <button
                                  key={type}
                                  type="button"
                                  aria-pressed={on}
                                  onClick={() => toggleProblem(signal.signalId, type)}
                                  disabled={signalsBusy}
                                  className={`min-h-[44px] rounded-lg border px-3 py-3 text-sm font-semibold disabled:opacity-50 ${
                                    on
                                      ? 'border-red-500 bg-red-600 text-white'
                                      : 'border-red-300 bg-white text-red-700 active:bg-red-50'
                                  }`}
                                >
                                  {label}
                                </button>
                              )
                            })}
                          </div>
                          <button
                            type="button"
                            onClick={() => answerRoomCheck(signal, problems)}
                            disabled={signalsBusy || problems.length === 0}
                            className="flex min-h-[44px] w-full items-center justify-center rounded-lg border border-red-400 bg-white px-3 py-3 text-sm font-semibold text-red-700 active:bg-red-50 disabled:opacity-50"
                          >
                            {signalBusy === `answer-problems:${signal.signalId}` ? (
                              <Loader2 className="h-5 w-5 animate-spin" />
                            ) : (
                              'ส่งคำตอบ'
                            )}
                          </button>
                        </div>
                      )}

                      {/* Every other type closes on a tap. รับทราบ disappears
                          once somebody has taken it — the row already names
                          who, and a second ack would only overwrite that. */}
                      <div className="mt-3 flex flex-wrap gap-2">
                        {actionable && signal.status === 'open' && (
                          <button
                            type="button"
                            onClick={() => ackSignal(signal)}
                            disabled={signalsBusy}
                            className="min-h-[44px] flex-1 rounded-lg border border-sky-400 bg-white px-3 py-3 text-sm font-semibold text-sky-800 active:bg-sky-50 disabled:opacity-50"
                          >
                            รับทราบ
                          </button>
                        )}
                        {actionable && !isRoomCheck(signal) && (
                          <button
                            type="button"
                            onClick={() => doneSignal(signal)}
                            disabled={signalsBusy}
                            className="flex min-h-[44px] flex-1 items-center justify-center rounded-lg border border-emerald-400 bg-emerald-600 px-3 py-3 text-sm font-semibold text-white active:bg-emerald-700 disabled:opacity-50"
                          >
                            {signalBusy === `done:${signal.signalId}` ? (
                              <Loader2 className="h-5 w-5 animate-spin" />
                            ) : (
                              'เสร็จสิ้น'
                            )}
                          </button>
                        )}
                        {/* The sender's own escape hatch, and only while
                            nobody has picked it up: once it is acked, someone
                            is already walking to the room. */}
                        {cancellable && (
                          <button
                            type="button"
                            onClick={() => cancelSignal(signal)}
                            disabled={signalsBusy}
                            className="min-h-[44px] flex-1 rounded-lg border border-gray-300 bg-white px-3 py-3 text-sm font-semibold text-gray-700 active:bg-gray-100 disabled:opacity-50"
                          >
                            ยกเลิก
                          </button>
                        )}
                      </div>
                    </li>
                  )
                })}
              </ul>
            )}

            {/* Sending one. The panel expands IN PLACE (the แจ้งขาดผ้า idiom):
                the first tap opens a list of canned types and files NOTHING,
                and each type then sends on a SINGLE tap.

                No second confirm here, unlike แจ้งห้องไม่สะอาด, and the
                difference is deliberate. Mark-dirty flips a real room in
                iHOTEL, which the maid cannot undo from her phone — so it earns
                a confirm. A signal is cancellable in place: a mis-tap appears
                in the list directly above with its own ยกเลิก while it is
                still open, and it writes nothing to iHOTEL at all. Adding a
                confirm to a recoverable, frequent action is how confirms stop
                being read on the one action that needs it. */}
            {sendOpen ? (
              <div className="mt-3 rounded-xl border border-indigo-300 bg-indigo-50 p-3">
                <p className="mb-3 flex items-center gap-1.5 text-sm font-semibold text-indigo-900">
                  <Send className="h-4 w-4 shrink-0" />
                  <span>{sendPanelLabel}</span>
                </p>
                <div className="space-y-2">
                  {sendable.map(({ type, label }) => (
                    <button
                      key={type}
                      type="button"
                      onClick={() => sendSignal(type, label)}
                      disabled={signalsBusy}
                      className="flex min-h-[44px] w-full items-center justify-center rounded-lg border border-indigo-300 bg-white px-3 py-3 text-sm font-semibold text-indigo-900 active:bg-indigo-100 disabled:opacity-50"
                    >
                      {signalBusy === `send:${type}` ? (
                        <Loader2 className="h-5 w-5 animate-spin" />
                      ) : (
                        label
                      )}
                    </button>
                  ))}
                </div>
                <button
                  type="button"
                  onClick={() => setSendOpen(false)}
                  disabled={signalsBusy}
                  className="mt-3 min-h-[44px] w-full rounded-lg border border-gray-300 bg-white px-3 py-3 text-sm font-semibold text-gray-700 active:bg-gray-100 disabled:opacity-50"
                >
                  ปิด
                </button>
              </div>
            ) : (
              /* Opens the list. Fires NO request. */
              <button
                type="button"
                onClick={() => setSendOpen(true)}
                disabled={signalsBusy}
                className="mt-3 flex min-h-[44px] w-full items-center justify-center gap-1.5 rounded-xl border border-indigo-300 bg-indigo-50 px-3 py-3 text-sm font-semibold text-indigo-900 active:bg-indigo-100 disabled:opacity-50"
              >
                <Send className="h-4 w-4" />
                {sendPanelLabel}
              </button>
            )}
          </section>

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

            {/* ------------------------------------------------------------ *
             * ขาดผ้าค้างอยู่ — what this room is STILL short of, and the one
             * control that finishes it.
             *
             * This replaced a one-line grey summary, and the promotion is the
             * point (owner request, 2026-09-01): an open shortage now survives
             * the day rollover, so it is outstanding WORK rather than a note
             * about today, and a line of 12px grey text under a button is not
             * how work gets seen. Same sky palette as everything else ขาดผ้า —
             * one subject, one colour — but a bordered filled card, matching
             * the queue panel on the list this room was tapped from.
             *
             * Rendered for BOTH roles; only the button is a maid's. A viewer
             * reading "ปลอกหมอน 2 ค้างอยู่" is exactly why reception holds
             * this screen, and a greyed-out button she can never use would
             * read as breakage.
             *
             * When there is nothing open — or the backend is older and cannot
             * answer the question — the original day-scoped totals line renders
             * unchanged in its place. Today's record is still worth showing to
             * a maid about to count the same pillowcases twice.
             * ------------------------------------------------------------ */}
            {openLinen.length > 0 ? (
              <div
                data-testid="hk-linen-open-card"
                className="mt-3 rounded-xl border-2 border-sky-400 bg-sky-50 p-3"
              >
                <p className="mb-2 flex items-center gap-1.5 text-sm font-bold text-sky-900">
                  <Shirt className="h-4 w-4 shrink-0" />
                  <span>{LINEN_OPEN_CARD_TITLE}</span>
                </p>
                <ul className="space-y-1">
                  {openLinen.map(({ kind, label, qty }) => (
                    <li
                      key={kind}
                      className="flex items-center justify-between gap-2 rounded-lg border border-sky-200 bg-white px-3 py-2"
                    >
                      <span className="text-sm font-medium text-gray-800">{label}</span>
                      <span className="text-base font-semibold tabular-nums text-sky-900">
                        {qty}
                      </span>
                    </li>
                  ))}
                </ul>

                {canReportFlag &&
                  (confirmingLinenResolve ? (
                    /* Step 2 — in place of the button, never over it, and with
                       ยกเลิก first and calmer so the reflex tap is the one that
                       changes nothing. Same shape as the mark-dirty confirm. */
                    <div className="mt-3 rounded-xl border border-sky-300 bg-white p-3">
                      <p className="mb-3 flex items-start gap-1.5 text-sm font-semibold text-sky-900">
                        <AlertTriangle className="mt-0.5 h-4 w-4 shrink-0" />
                        <span>{linenResolveConfirmMessage(room.roomNo)}</span>
                      </p>
                      <div className="grid grid-cols-2 gap-3">
                        <button
                          type="button"
                          onClick={() => setConfirmingLinenResolve(false)}
                          disabled={busy}
                          className="rounded-lg border border-gray-300 bg-white px-3 py-3 text-sm font-semibold text-gray-700 active:bg-gray-100 disabled:opacity-50"
                        >
                          ยกเลิก
                        </button>
                        <button
                          type="button"
                          onClick={resolveLinen}
                          disabled={busy}
                          className="flex items-center justify-center rounded-lg border border-sky-500 bg-sky-600 px-3 py-3 text-sm font-semibold text-white active:bg-sky-700 disabled:opacity-50"
                        >
                          {linenResolving ? (
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
                      onClick={() => setConfirmingLinenResolve(true)}
                      disabled={busy}
                      className="mt-3 flex min-h-[44px] w-full items-center justify-center gap-1.5 rounded-lg border border-sky-500 bg-sky-600 px-3 py-3 text-sm font-semibold text-white active:bg-sky-700 disabled:opacity-50"
                    >
                      <Check className="h-4 w-4" />
                      เติมผ้าแล้ว
                    </button>
                  ))}
              </div>
            ) : (
              linenSummary && <p className="mt-2 text-xs text-gray-500">{linenSummary}</p>
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
