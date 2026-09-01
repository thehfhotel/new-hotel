'use client'

// Report HK — ONE ROOM's daily report (/hk/rooms/[roomId]/report).
//
// Three screens in one route, switched by WHO is looking and WHERE the report
// stands (CONTEXT.md §Housekeeping "Room report" / "Report verification"):
//
//   maid   + nothing filed today  → THE FORM
//   maid   + returned report      → THE FORM again, prefilled with what she
//                                   sent, under the canned reason it came back
//                                   for. History is append-only: this files a
//                                   NEW report carrying `parentReportId`,
//                                   never an edit of the old one.
//   maid   + submitted            → read-only; she is waiting on reception,
//                                   and a maid NEVER verifies — including one
//                                   who also holds the reception grant.
//   reception + submitted         → THE VERIFY VIEW: everything the maid sent,
//                                   plus reception's own photos and the two
//                                   canned outcomes.
//   anyone + verified             → read-only summary with BOTH photo sets.
//
// The branch is never chosen here — /hk and /hk/report are the two screens
// that may pick one. This one only READS what is stored (§A1: never guess,
// never default) and sends the user back to choose when there is nothing valid.
//
// EVIDENCE. Photos are captured with the device camera, downscaled in the
// browser (`downscalePhoto`) and uploaded ONE AT A TIME, before the report is
// filed; the submit body carries only their ids. That is why an abandoned form
// leaves unattached photo rows behind — accepted v1 debt (no GC), documented on
// `uploadHkReportPhoto`. The alternative, holding four full-size images in a
// LINE WebView until she taps ส่ง, is how a phone kills the tab mid-report.

import { useCallback, useEffect, useRef, useState } from 'react'
import Link from 'next/link'
import { useParams, useRouter } from 'next/navigation'
import {
  AlertCircle,
  AlertTriangle,
  Camera,
  Check,
  CheckCircle2,
  ChevronLeft,
  ClipboardList,
  Loader2,
  Minus,
  Plus,
  Undo2,
  X,
} from 'lucide-react'
import {
  canFileReport,
  canReport,
  canReturnReport,
  canSubmitReport,
  canVerifyReport,
  canVerifyReports,
  downscalePhoto,
  fetchHkRoomReport,
  hkFetch,
  hkFetchMe,
  hkReportPhotoUrl,
  ITEM_PROBLEMS,
  prefillRoomStatus,
  readStoredBranch,
  REPORT_ITEMS,
  REPORT_MAX_PHOTOS,
  REPORT_MAX_QTY,
  REPORT_MIN_QTY,
  REPORT_RETURNED_NOTICE,
  REPORT_SUBMITTED_NOTICE,
  REPORT_VERIFIED_NOTICE,
  reportDateLabel,
  reportExceptionDraftFrom,
  reportExceptionItems,
  reportExceptionQty,
  reportItemRows,
  reportState,
  reportStateChip,
  resolveInitialBranch,
  RETURN_REASONS,
  returnHkReport,
  returnReasonLabel,
  ROOM_STATUS_CODES,
  roomStatusLabel,
  signalActorLabel,
  signalRole,
  stashHkReportNotice,
  stepReportException,
  submitHkReport,
  timeLabel,
  toggleReportException,
  uploadHkReportPhoto,
  verifyHkReport,
  type Branch,
  type HkMe,
  type HkReport,
  type HkReportExceptionDraft,
  type HkReportRoom,
  type HkRoomDetail,
  type ReturnReason,
  type RoomStatusCode,
} from '../../../hk-lib'
import { useHkAutoRefresh } from '../../../use-hk-auto-refresh'

/** Refused HERE rather than after four slow uploads. */
const PHOTO_LIMIT_MESSAGE = `แนบรูปได้ไม่เกิน ${REPORT_MAX_PHOTOS} รูป`

/**
 * The photo strip — capture button, thumbnails, remove. Used by BOTH sides
 * (the maid's evidence and reception's), because they are the same control
 * with a different label, and two copies is how the two sides' caps drift
 * apart. Thumbnails render from the SERVER url rather than a local object url:
 * the round trip is a ≤1600px JPEG, and it proves the upload actually landed
 * before she is allowed to submit on the strength of it.
 */
function ReportPhotoStrip({
  captureLabel,
  branch,
  photoIds,
  onPick,
  onRemove,
  uploading,
  disabled,
  testId,
}: {
  captureLabel: string
  branch: Branch | null
  photoIds: number[]
  onPick: (files: FileList | null) => void
  onRemove: (photoId: number) => void
  uploading: boolean
  disabled: boolean
  testId: string
}) {
  const full = photoIds.length >= REPORT_MAX_PHOTOS
  return (
    <div data-testid={testId}>
      {photoIds.length > 0 && (
        <ul className="mb-2 grid grid-cols-4 gap-2">
          {photoIds.map((photoId) => (
            <li key={photoId} className="relative">
              {/* eslint-disable-next-line @next/next/no-img-element */}
              <img
                src={hkReportPhotoUrl(photoId, branch)}
                alt={`รูปที่แนบ ${photoId}`}
                className="h-20 w-full rounded-lg border border-gray-200 object-cover"
              />
              <button
                type="button"
                aria-label={`ลบรูป ${photoId}`}
                onClick={() => onRemove(photoId)}
                disabled={disabled || uploading}
                className="absolute -right-1 -top-1 flex h-8 w-8 items-center justify-center rounded-full border border-gray-300 bg-white text-gray-600 active:bg-gray-100 disabled:opacity-50"
              >
                <X className="h-4 w-4" />
              </button>
            </li>
          ))}
        </ul>
      )}
      {/* A label wrapping a visually-hidden input: a bare file input cannot be
          made 44px tall reliably across WebViews, and `capture` is what opens
          the camera straight away instead of a file browser. */}
      <label
        className={`flex min-h-[44px] w-full items-center justify-center gap-1.5 rounded-lg border border-teal-400 bg-white px-3 py-3 text-sm font-semibold text-teal-800 active:bg-teal-50 ${
          disabled || uploading || full ? 'opacity-50' : ''
        }`}
      >
        {uploading ? <Loader2 className="h-5 w-5 animate-spin" /> : <Camera className="h-4 w-4" />}
        <span>{captureLabel}</span>
        <input
          type="file"
          accept="image/*"
          capture="environment"
          multiple
          aria-label={captureLabel}
          disabled={disabled || uploading || full}
          className="sr-only"
          onChange={(e) => {
            onPick(e.target.files)
            // Let the SAME photo be picked twice (a retake of a shot she just
            // removed): without this the input holds the old value and fires
            // no change event.
            e.target.value = ''
          }}
        />
      </label>
      <p className="mt-1 text-[11px] text-gray-400">
        แนบรูปได้ {photoIds.length}/{REPORT_MAX_PHOTOS} รูป
      </p>
    </div>
  )
}

/** Somebody else's photos, read-only — the maid's set on reception's screen
 *  and both sets on a verified report. */
function ReportPhotoGallery({
  title,
  branch,
  photoIds,
  testId,
}: {
  title: string
  branch: Branch | null
  photoIds: number[]
  testId: string
}) {
  if (photoIds.length === 0) return null
  return (
    <div data-testid={testId} className="mt-3">
      <p className="mb-1 text-xs font-semibold text-gray-500">{title}</p>
      <ul className="grid grid-cols-4 gap-2">
        {photoIds.map((photoId) => (
          <li key={photoId}>
            {/* eslint-disable-next-line @next/next/no-img-element */}
            <img
              src={hkReportPhotoUrl(photoId, branch)}
              alt={`${title} ${photoId}`}
              className="h-20 w-full rounded-lg border border-gray-200 object-cover"
            />
          </li>
        ))}
      </ul>
    </div>
  )
}

export default function HkRoomReportPage() {
  const params = useParams<{ roomId: string }>()
  const router = useRouter()
  const roomId = Number(params?.roomId)

  const [branch, setBranch] = useState<Branch | null>(null)
  const [branchChecked, setBranchChecked] = useState(false)
  // Initial TRUE for the same skew reason the room screen documents: an absent
  // `canReport` means "maid". Nothing renders before `/me` settles.
  const [canReportFlag, setCanReportFlag] = useState(true)

  const [detail, setDetail] = useState<HkRoomDetail | null>(null)
  const [row, setRow] = useState<HkReportRoom | null>(null)
  const [report, setReport] = useState<HkReport | null>(null)
  const [date, setDate] = useState('')
  const [loaded, setLoaded] = useState(false)
  const [loading, setLoading] = useState(true)
  const [error, setError] = useState<string | null>(null)
  const [notice, setNotice] = useState<string | null>(null)

  // --- the maid's form ------------------------------------------------------
  const [roomStatus, setRoomStatus] = useState<RoomStatusCode | null>(null)
  const [allItemsOk, setAllItemsOk] = useState(true)
  const [exceptions, setExceptions] = useState<HkReportExceptionDraft>({})
  const [maidPhotoIds, setMaidPhotoIds] = useState<number[]>([])

  // --- reception's verify ---------------------------------------------------
  const [receptionPhotoIds, setReceptionPhotoIds] = useState<number[]>([])
  const [returnReason, setReturnReason] = useState<ReturnReason | null>(null)

  const [uploading, setUploading] = useState(false)
  const [submitting, setSubmitting] = useState(false)
  // In-place confirms, the /hk idiom: the first tap arms, the second files.
  const [confirming, setConfirming] = useState<'submit' | 'verify' | 'return' | null>(null)

  // The form is seeded EXACTLY ONCE, on the first successful load. A poll that
  // re-seeded would wipe a half-filled form under a maid's thumb — the whole
  // reason the auto-refresh is gated below, and the reason this is a ref rather
  // than an effect dependency.
  const seededRef = useRef(false)

  useEffect(() => {
    let live = true
    hkFetchMe()
      .then(async (res) => {
        if (!res.ok) throw new Error()
        const data: HkMe = await res.json()
        if (!live || !data.success) return
        setCanReportFlag(canReport(data))
        setBranch(resolveInitialBranch(data.branches, readStoredBranch()))
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
        // The room's own facts (for the status prefill and the header) and the
        // day's report are two different questions of two different endpoints;
        // issued together so the screen paints once.
        const [roomRes, day] = await Promise.all([
          hkFetch(`/rooms/${roomId}`, branch),
          fetchHkRoomReport(branch, roomId),
        ])
        if (!roomRes.ok) {
          throw new Error(roomRes.status === 404 ? 'ไม่พบห้องนี้' : 'ไม่สามารถดึงข้อมูลห้องได้')
        }
        const roomDetail: HkRoomDetail = await roomRes.json()
        if (!roomDetail.success) throw new Error('ไม่สามารถดึงข้อมูลห้องได้')
        setDetail(roomDetail)
        setRow(day.room)
        setReport(day.report)
        setDate(day.date)
        setError(null)
        setLoaded(true)

        if (!seededRef.current) {
          seededRef.current = true
          if (day.report && day.report.status === 'returned') {
            // A resubmission starts from what she sent, so she fixes what came
            // back instead of re-entering twenty-two rows in a corridor.
            const previous = ROOM_STATUS_CODES.find(
              ({ code }) => code === day.report?.roomStatus
            )?.code
            setRoomStatus(previous ?? prefillRoomStatus(roomDetail.room))
            // The ITEMS decide as much as the flag does: a backend that ships
            // exceptions without `allItemsOk` (skew, or a field renamed under
            // us) must not have its list silently hidden behind a
            // ครบทุกรายการ toggle she never chose.
            const hadExceptions = (day.report.items?.length ?? 0) > 0
            setAllItemsOk(!(day.report.allItemsOk === false || hadExceptions))
            setExceptions(reportExceptionDraftFrom(day.report.items))
          } else {
            setRoomStatus(prefillRoomStatus(roomDetail.room))
          }
        }
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

  const busy = uploading || submitting

  // Same contract as the room screen: poll while she is looking, never while a
  // write of hers is in flight. The seed guard above is what keeps a poll from
  // touching the form itself.
  const refresh = useCallback(() => load(true), [load])
  useHkAutoRefresh(refresh, Boolean(branch) && !busy)

  /** Upload picked photos into one of the two strips, honouring the 1..4 cap.
   *  One handler for both sides — the maid's evidence and reception's obey the
   *  same bounds because they are enforced in one place. */
  const addPhotos = async (
    files: FileList | null,
    current: number[],
    setIds: (next: (prev: number[]) => number[]) => void
  ) => {
    if (!branch || !files || files.length === 0) return
    const room = REPORT_MAX_PHOTOS - current.length
    if (room <= 0) {
      setError(PHOTO_LIMIT_MESSAGE)
      return
    }
    const chosen = Array.from(files).slice(0, room)
    setUploading(true)
    setNotice(null)
    try {
      for (const file of chosen) {
        // Downscaled BEFORE the wire, and every failure path in the helper
        // hands the original file back rather than throwing — a maid must
        // always be able to file.
        const blob = await downscalePhoto(file)
        const photoId = await uploadHkReportPhoto(branch, blob)
        setIds((prev) => (prev.includes(photoId) ? prev : [...prev, photoId]))
      }
      setError(null)
    } catch (err) {
      setError(err instanceof Error ? err.message : 'เกิดข้อผิดพลาด')
    } finally {
      setUploading(false)
    }
  }

  const role = signalRole(canReportFlag)
  const isMaid = canFileReport(role)
  const state = reportState(report)
  const room = detail?.room ?? null
  const roomNo = room?.roomNo ?? row?.roomNo ?? ''

  // What the form would send right now — also the answer to "may she submit",
  // so the button and the request can never disagree. `allItemsOk` makes the
  // list empty by construction; the remaining rule (a claim of exceptions must
  // NAME one) is what `canSubmitReport` still refuses on.
  const items = allItemsOk ? [] : reportExceptionItems(exceptions)
  const submitReady = canSubmitReport({ roomStatus, allItemsOk, items, photoIds: maidPhotoIds })

  const showForm = isMaid && (state === 'unsent' || state === 'returned')
  const showVerify = canVerifyReports(role) && state === 'submitted' && report !== null

  const submit = async () => {
    if (!branch || !isMaid || !roomStatus) return
    if (!submitReady) return
    setSubmitting(true)
    setNotice(null)
    try {
      await submitHkReport(branch, roomId, {
        roomStatus,
        allItemsOk,
        items,
        photoIds: maidPhotoIds,
        // Append-only: a fix POINTS AT the report it supersedes rather than
        // editing it. Absent for a first submission of the day.
        ...(state === 'returned' && report ? { parentReportId: report.reportId } : {}),
      })
      // The banner belongs on the screen she lands on, not this one.
      stashHkReportNotice(REPORT_SUBMITTED_NOTICE)
      router.push('/hk/report')
    } catch (err) {
      setError(err instanceof Error ? err.message : 'เกิดข้อผิดพลาด')
    } finally {
      setSubmitting(false)
      // Collapse only once the request has SETTLED, so ยืนยัน can carry the
      // spinner and a failure never leaves a re-armed one-tap button behind.
      setConfirming(null)
    }
  }

  const verify = async () => {
    if (!branch || !report || !canVerifyReports(role)) return
    if (!canVerifyReport(receptionPhotoIds)) return
    setSubmitting(true)
    setNotice(null)
    try {
      await verifyHkReport(branch, report.reportId, receptionPhotoIds)
      stashHkReportNotice(REPORT_VERIFIED_NOTICE)
      router.push('/hk/report')
    } catch (err) {
      setError(err instanceof Error ? err.message : 'เกิดข้อผิดพลาด')
    } finally {
      setSubmitting(false)
      setConfirming(null)
    }
  }

  const sendBack = async () => {
    if (!branch || !report || !canVerifyReports(role)) return
    if (!canReturnReport(returnReason)) return
    setSubmitting(true)
    setNotice(null)
    try {
      await returnHkReport(branch, report.reportId, returnReason as ReturnReason)
      stashHkReportNotice(REPORT_RETURNED_NOTICE)
      router.push('/hk/report')
    } catch (err) {
      setError(err instanceof Error ? err.message : 'เกิดข้อผิดพลาด')
    } finally {
      setSubmitting(false)
      setConfirming(null)
    }
  }

  // No valid branch stored — never guess one here.
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

  if ((loading && !loaded) || !branchChecked) {
    return (
      <main className="flex items-center justify-center py-16 text-gray-500">
        <Loader2 className="mr-2 h-6 w-6 animate-spin" />
        กำลังโหลด...
      </main>
    )
  }

  const chip = reportStateChip(report)
  const exceptionRows = reportItemRows(report?.items)

  return (
    <main>
      <Link
        href="/hk/report"
        className="mb-3 inline-flex items-center gap-1 text-sm text-gray-500"
      >
        <ChevronLeft className="h-4 w-4" />
        กลับไปหน้ารายงานประจำวัน
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

      <header className="mb-4 rounded-xl border border-gray-200 bg-white p-4">
        <div className="flex items-center justify-between gap-2">
          <h1 className="flex items-center gap-2 text-2xl font-bold">
            <ClipboardList className="h-6 w-6 shrink-0 text-teal-600" />
            ห้อง {roomNo}
          </h1>
          <span
            className={`inline-block rounded-full border px-2.5 py-1 text-xs ${chip.className}`}
          >
            {chip.label}
          </span>
        </div>
        <p className="mt-1 text-sm text-gray-500">
          รายงานประจำวัน{date ? ` · ${reportDateLabel(date)}` : ''}
        </p>
      </header>

      {/* ------------------------------------------------------------------ *
       * RETURNED — the reason, first thing on the screen, in the words the
       * vocabulary allows and nothing else. A maid walking back to a room she
       * thought she had finished is owed the WHY before the form.
       * ------------------------------------------------------------------ */}
      {state === 'returned' && report && (
        <div
          data-testid="hk-report-returned-banner"
          className="mb-4 rounded-xl border-2 border-red-300 bg-red-50 p-3"
        >
          <p className="flex items-start gap-1.5 text-sm font-bold text-red-800">
            <Undo2 className="mt-0.5 h-4 w-4 shrink-0" />
            <span>ส่งกลับให้แก้ไข: {returnReasonLabel(report.returnReason)}</span>
          </p>
          {report.verifiedBy && (
            <p className="mt-1 text-xs text-red-700">
              โดย {signalActorLabel(report.verifiedBy)}
              {report.verifiedAt ? ` · ${timeLabel(report.verifiedAt)}` : ''}
            </p>
          )}
        </div>
      )}

      {/* ================================================================== *
       * THE MAID'S FORM. Rendered for a maid with nothing filed today, and
       * for one fixing a returned report (prefilled, above). A reception
       * viewer never sees it at all — hidden WHOLE rather than disabled, the
       * same rule the room screen's action surface follows: absence reads as
       * "not my job", greyed-out reads as "broken".
       * ================================================================== */}
      {showForm && (
        <section data-testid="hk-report-form" className="mb-6 space-y-5">
          {/* --- room status ------------------------------------------------ */}
          <div>
            <h2 className="mb-2 text-sm font-semibold text-gray-600">สถานะห้อง</h2>
            {/* PREFILLED from the room's own facts (see `prefillRoomStatus`),
                never locked: what she leaves selected is what is stored, so a
                wrong guess costs one tap. */}
            <div className="grid grid-cols-2 gap-2">
              {ROOM_STATUS_CODES.map(({ code, label }) => {
                const on = roomStatus === code
                return (
                  <button
                    key={code}
                    type="button"
                    aria-pressed={on}
                    onClick={() => setRoomStatus(code)}
                    disabled={busy}
                    className={`min-h-[44px] rounded-xl border px-3 py-3 text-sm font-semibold disabled:opacity-50 ${
                      on
                        ? 'border-teal-600 bg-teal-600 text-white'
                        : 'border-gray-300 bg-white text-gray-700 active:bg-gray-100'
                    }`}
                  >
                    {label}
                  </button>
                )
              })}
            </div>
          </div>

          {/* --- the checklist, exception-based ----------------------------- */}
          <div>
            <h2 className="mb-2 text-sm font-semibold text-gray-600">อุปกรณ์ภายในห้อง</h2>
            {/* The sheet is EXCEPTION-BASED: the common answer is one tap, and
                only a room with something wrong costs the maid a list. */}
            <div className="grid grid-cols-2 gap-2">
              <button
                type="button"
                aria-pressed={allItemsOk}
                onClick={() => setAllItemsOk(true)}
                disabled={busy}
                className={`min-h-[44px] rounded-xl border px-3 py-3 text-sm font-semibold disabled:opacity-50 ${
                  allItemsOk
                    ? 'border-emerald-600 bg-emerald-600 text-white'
                    : 'border-gray-300 bg-white text-gray-700 active:bg-gray-100'
                }`}
              >
                ครบทุกรายการ
              </button>
              <button
                type="button"
                aria-pressed={!allItemsOk}
                onClick={() => setAllItemsOk(false)}
                disabled={busy}
                className={`min-h-[44px] rounded-xl border px-3 py-3 text-sm font-semibold disabled:opacity-50 ${
                  !allItemsOk
                    ? 'border-red-500 bg-red-600 text-white'
                    : 'border-gray-300 bg-white text-gray-700 active:bg-gray-100'
                }`}
              >
                มีรายการผิดปกติ
              </button>
            </div>

            {!allItemsOk && (
              <ul data-testid="hk-report-items" className="mt-2 space-y-2">
                {REPORT_ITEMS.map(({ item, label }) => (
                  <li
                    key={item}
                    className="rounded-lg border border-gray-200 bg-white px-3 py-2"
                  >
                    <p className="text-sm font-medium text-gray-800">{label}</p>
                    <div className="mt-2 grid grid-cols-2 gap-2">
                      {ITEM_PROBLEMS.map(({ problem, label: problemLabel }) => {
                        const qty = reportExceptionQty(exceptions, item, problem)
                        return (
                          <button
                            key={problem}
                            type="button"
                            aria-pressed={qty > 0}
                            aria-label={`${label} ${problemLabel}`}
                            onClick={() =>
                              setExceptions((prev) => toggleReportException(prev, item, problem))
                            }
                            disabled={busy}
                            className={`min-h-[44px] rounded-lg border px-3 py-2 text-sm font-semibold disabled:opacity-50 ${
                              qty > 0
                                ? 'border-red-500 bg-red-600 text-white'
                                : 'border-red-200 bg-white text-red-700 active:bg-red-50'
                            }`}
                          >
                            {problemLabel}
                          </button>
                        )
                      })}
                    </div>
                    {/* One stepper per SELECTED problem — an item can be both
                        หาย and ชำรุด at once (two towels gone, a third torn),
                        which is two exceptions about one item. Same 44px
                        stepper idiom as แจ้งขาดผ้า. */}
                    {ITEM_PROBLEMS.map(({ problem, label: problemLabel }) => {
                      const qty = reportExceptionQty(exceptions, item, problem)
                      if (qty <= 0) return null
                      return (
                        <div
                          key={`qty-${problem}`}
                          className="mt-2 flex items-center justify-between gap-2 rounded-lg border border-red-200 bg-red-50 px-3 py-2"
                        >
                          <span className="text-xs font-medium text-red-800">
                            {problemLabel} · จำนวน
                          </span>
                          <span className="flex items-center gap-2">
                            <button
                              type="button"
                              aria-label={`ลด ${label} ${problemLabel}`}
                              onClick={() =>
                                setExceptions((prev) =>
                                  stepReportException(prev, item, problem, -1)
                                )
                              }
                              disabled={busy || qty <= REPORT_MIN_QTY}
                              className="flex h-11 w-11 items-center justify-center rounded-lg border border-red-300 bg-white text-red-800 active:bg-red-100 disabled:opacity-40"
                            >
                              <Minus className="h-5 w-5" />
                            </button>
                            <span className="w-7 text-center text-base font-semibold tabular-nums text-gray-900">
                              {qty}
                            </span>
                            <button
                              type="button"
                              aria-label={`เพิ่ม ${label} ${problemLabel}`}
                              onClick={() =>
                                setExceptions((prev) =>
                                  stepReportException(prev, item, problem, 1)
                                )
                              }
                              disabled={busy || qty >= REPORT_MAX_QTY}
                              className="flex h-11 w-11 items-center justify-center rounded-lg border border-red-300 bg-white text-red-800 active:bg-red-100 disabled:opacity-40"
                            >
                              <Plus className="h-5 w-5" />
                            </button>
                          </span>
                        </div>
                      )
                    })}
                  </li>
                ))}
              </ul>
            )}
          </div>

          {/* --- photos ---------------------------------------------------- */}
          <div>
            <h2 className="mb-2 text-sm font-semibold text-gray-600">
              รูปถ่ายห้อง (อย่างน้อย 1 รูป)
            </h2>
            <ReportPhotoStrip
              testId="hk-report-maid-photos"
              captureLabel="ถ่ายรูปห้อง"
              branch={branch}
              photoIds={maidPhotoIds}
              uploading={uploading}
              disabled={submitting}
              onPick={(files) => addPhotos(files, maidPhotoIds, setMaidPhotoIds)}
              onRemove={(photoId) =>
                setMaidPhotoIds((prev) => prev.filter((id) => id !== photoId))
              }
            />
          </div>

          {/* --- submit, behind the in-place confirm ------------------------ */}
          {confirming === 'submit' ? (
            /* Rendered IN PLACE of the button, never over it, ยกเลิก first and
               calmer — the same shape as every other confirm on this surface,
               so the reflex tap is the one that files nothing. */
            <div className="rounded-xl border border-teal-300 bg-teal-50 p-3">
              <p className="mb-3 flex items-start gap-1.5 text-sm font-semibold text-teal-900">
                <AlertTriangle className="mt-0.5 h-4 w-4 shrink-0" />
                <span>ยืนยันส่งรายงาน ห้อง {roomNo}?</span>
              </p>
              <div className="grid grid-cols-2 gap-3">
                <button
                  type="button"
                  onClick={() => setConfirming(null)}
                  disabled={busy}
                  className="min-h-[44px] rounded-lg border border-gray-300 bg-white px-3 py-3 text-sm font-semibold text-gray-700 active:bg-gray-100 disabled:opacity-50"
                >
                  ยกเลิก
                </button>
                <button
                  type="button"
                  onClick={submit}
                  disabled={busy || !submitReady}
                  className="flex min-h-[44px] items-center justify-center rounded-lg border border-teal-600 bg-teal-600 px-3 py-3 text-sm font-semibold text-white active:bg-teal-700 disabled:opacity-50"
                >
                  {submitting ? <Loader2 className="h-5 w-5 animate-spin" /> : 'ยืนยัน'}
                </button>
              </div>
            </div>
          ) : (
            /* Step 1 — arms the confirm. Fires NO request. Dead until the
               report is actually fileable: a photo at minimum, and a named
               exception whenever she says something is wrong. */
            <button
              type="button"
              onClick={() => setConfirming('submit')}
              disabled={busy || !submitReady}
              className="flex min-h-[44px] w-full items-center justify-center gap-1.5 rounded-xl border border-teal-600 bg-teal-600 px-3 py-3 text-base font-semibold text-white active:bg-teal-700 disabled:opacity-50"
            >
              <ClipboardList className="h-4 w-4" />
              ส่งรายงาน
            </button>
          )}
        </section>
      )}

      {/* ================================================================== *
       * WHAT WAS REPORTED — the read-only body of a filed report. Rendered
       * for every audience that is not filling in the form: the maid waiting
       * on her verification, reception about to verify (with her own controls
       * appended below), and both roles on a finished report.
       * ================================================================== */}
      {report && !showForm && (
        <section data-testid="hk-report-summary" className="mb-6 space-y-3">
          <div className="rounded-xl border border-gray-200 bg-white p-4">
            <p className="text-sm text-gray-500">สถานะห้อง</p>
            <p className="text-base font-semibold text-gray-900">
              {roomStatusLabel(report.roomStatus)}
            </p>

            <p className="mt-3 text-sm text-gray-500">อุปกรณ์ภายในห้อง</p>
            {/* The flag says the room was fine; the rows say what was not. When
                they disagree — an older backend, a field renamed under us — the
                ROWS win: showing "ครบทุกรายการ" over a list of missing items is
                the one failure here that could cost a guest a charge nobody can
                explain. */}
            {report.allItemsOk !== false && exceptionRows.length === 0 ? (
              <p className="flex items-center gap-1.5 text-base font-semibold text-emerald-700">
                <CheckCircle2 className="h-4 w-4 shrink-0" />
                ครบทุกรายการ
              </p>
            ) : (
              <ul data-testid="hk-report-exceptions" className="mt-1 space-y-1">
                {exceptionRows.map(({ key, label, problemLabel, qty }) => (
                  <li
                    key={key}
                    className="flex items-center justify-between gap-2 rounded-lg border border-red-200 bg-red-50 px-3 py-2"
                  >
                    <span className="text-sm font-medium text-gray-800">
                      {label} · {problemLabel}
                    </span>
                    <span className="text-base font-semibold tabular-nums text-red-800">
                      {qty}
                    </span>
                  </li>
                ))}
              </ul>
            )}

            <p className="mt-3 text-xs text-gray-500">
              ส่งโดย {signalActorLabel(report.submittedBy)}
              {report.submittedAt ? ` · ${timeLabel(report.submittedAt)}` : ''}
            </p>
            {report.status === 'verified' && report.verifiedBy && (
              <p className="text-xs text-emerald-700">
                ตรวจโดย {signalActorLabel(report.verifiedBy)}
                {report.verifiedAt ? ` · ${timeLabel(report.verifiedAt)}` : ''}
              </p>
            )}

            <ReportPhotoGallery
              testId="hk-report-maid-gallery"
              title="รูปจากแม่บ้าน"
              branch={branch}
              photoIds={report.maidPhotoIds ?? []}
            />
            <ReportPhotoGallery
              testId="hk-report-reception-gallery"
              title="รูปจากแผนกต้อนรับ"
              branch={branch}
              photoIds={report.receptionPhotoIds ?? []}
            />
          </div>

          {/* A maid's own filed report: nothing to do but wait. Said out loud,
              because an empty screen reads as a screen that failed to load. */}
          {isMaid && state === 'submitted' && (
            <p className="text-xs text-gray-500">ส่งแล้ว รอแผนกต้อนรับตรวจ</p>
          )}
        </section>
      )}

      {/* ================================================================== *
       * RECEPTION'S VERIFY. Only for the desk side, only on a SUBMITTED
       * report. A maid never reaches it — including one who also holds the
       * reception grant, which `canReport` already resolves to the maid side.
       * ================================================================== */}
      {showVerify && report && (
        <section data-testid="hk-report-verify" className="mb-6 space-y-4">
          <div>
            <h2 className="mb-2 text-sm font-semibold text-gray-600">
              รูปถ่ายจากการตรวจ (อย่างน้อย 1 รูป)
            </h2>
            {/* Reception's OWN photos are what make a verify a walk-up rather
                than a desk stamp — the two-sided evidence IS the feature. */}
            <ReportPhotoStrip
              testId="hk-report-reception-photos"
              captureLabel="ถ่ายรูปการตรวจ"
              branch={branch}
              photoIds={receptionPhotoIds}
              uploading={uploading}
              disabled={submitting}
              onPick={(files) => addPhotos(files, receptionPhotoIds, setReceptionPhotoIds)}
              onRemove={(photoId) =>
                setReceptionPhotoIds((prev) => prev.filter((id) => id !== photoId))
              }
            />
          </div>

          {confirming === 'verify' ? (
            <div className="rounded-xl border border-emerald-300 bg-emerald-50 p-3">
              <p className="mb-3 flex items-start gap-1.5 text-sm font-semibold text-emerald-900">
                <AlertTriangle className="mt-0.5 h-4 w-4 shrink-0" />
                <span>ยืนยันว่าตรวจ ห้อง {roomNo} แล้ว?</span>
              </p>
              <div className="grid grid-cols-2 gap-3">
                <button
                  type="button"
                  onClick={() => setConfirming(null)}
                  disabled={busy}
                  className="min-h-[44px] rounded-lg border border-gray-300 bg-white px-3 py-3 text-sm font-semibold text-gray-700 active:bg-gray-100 disabled:opacity-50"
                >
                  ยกเลิก
                </button>
                <button
                  type="button"
                  onClick={verify}
                  disabled={busy || !canVerifyReport(receptionPhotoIds)}
                  className="flex min-h-[44px] items-center justify-center rounded-lg border border-emerald-600 bg-emerald-600 px-3 py-3 text-sm font-semibold text-white active:bg-emerald-700 disabled:opacity-50"
                >
                  {submitting ? <Loader2 className="h-5 w-5 animate-spin" /> : 'ยืนยัน'}
                </button>
              </div>
            </div>
          ) : (
            <button
              type="button"
              onClick={() => setConfirming('verify')}
              disabled={busy || !canVerifyReport(receptionPhotoIds)}
              className="flex min-h-[44px] w-full items-center justify-center gap-1.5 rounded-xl border border-emerald-600 bg-emerald-600 px-3 py-3 text-base font-semibold text-white active:bg-emerald-700 disabled:opacity-50"
            >
              <CheckCircle2 className="h-4 w-4" />
              ยืนยันการตรวจ
            </button>
          )}

          {/* The rejection. Canned reasons ONLY — this picker is the whole
              rejection vocabulary, and the confirm sits behind a chosen one so
              a return can never be filed without saying why. Photos are NOT
              collected here: a return is a rejection, not a walk-up. */}
          <div className="rounded-xl border border-red-200 bg-white p-3">
            <p className="mb-2 text-sm font-semibold text-red-800">ส่งกลับให้แก้ไข</p>
            <div className="space-y-2">
              {RETURN_REASONS.map(({ reason, label }) => {
                const on = returnReason === reason
                return (
                  <button
                    key={reason}
                    type="button"
                    aria-pressed={on}
                    onClick={() => setReturnReason(on ? null : reason)}
                    disabled={busy}
                    className={`min-h-[44px] w-full rounded-lg border px-3 py-3 text-sm font-semibold disabled:opacity-50 ${
                      on
                        ? 'border-red-500 bg-red-600 text-white'
                        : 'border-red-200 bg-white text-red-700 active:bg-red-50'
                    }`}
                  >
                    {label}
                  </button>
                )
              })}
            </div>

            {confirming === 'return' ? (
              <div className="mt-3 rounded-xl border border-red-300 bg-red-50 p-3">
                <p className="mb-3 flex items-start gap-1.5 text-sm font-semibold text-red-800">
                  <AlertTriangle className="mt-0.5 h-4 w-4 shrink-0" />
                  <span>
                    ส่งกลับ ห้อง {roomNo} เพราะ {returnReasonLabel(returnReason)}?
                  </span>
                </p>
                <div className="grid grid-cols-2 gap-3">
                  <button
                    type="button"
                    onClick={() => setConfirming(null)}
                    disabled={busy}
                    className="min-h-[44px] rounded-lg border border-gray-300 bg-white px-3 py-3 text-sm font-semibold text-gray-700 active:bg-gray-100 disabled:opacity-50"
                  >
                    ยกเลิก
                  </button>
                  <button
                    type="button"
                    onClick={sendBack}
                    disabled={busy || !canReturnReport(returnReason)}
                    className="flex min-h-[44px] items-center justify-center rounded-lg border border-red-500 bg-red-600 px-3 py-3 text-sm font-semibold text-white active:bg-red-700 disabled:opacity-50"
                  >
                    {submitting ? <Loader2 className="h-5 w-5 animate-spin" /> : 'ยืนยัน'}
                  </button>
                </div>
              </div>
            ) : (
              <button
                type="button"
                onClick={() => setConfirming('return')}
                disabled={busy || !canReturnReport(returnReason)}
                className="mt-3 flex min-h-[44px] w-full items-center justify-center gap-1.5 rounded-lg border border-red-400 bg-white px-3 py-3 text-sm font-semibold text-red-700 active:bg-red-50 disabled:opacity-50"
              >
                <Undo2 className="h-4 w-4" />
                ส่งกลับ
              </button>
            )}
          </div>
        </section>
      )}

      {/* Reception on a room nobody has reported yet: there is nothing to
          verify and nothing for her to file. Said plainly rather than left as
          an empty screen. */}
      {!report && !showForm && (
        <p data-testid="hk-report-empty" className="text-sm text-gray-500">
          ห้องนี้ยังไม่ส่งรายงานของวันนี้
        </p>
      )}

      {/* The maid's other work on this room is one tap away — she arrives here
          from the day overview, not from the room screen. */}
      <Link
        href={`/hk/rooms/${roomId}`}
        className="mt-6 inline-flex items-center gap-1 text-sm text-gray-500"
      >
        <ChevronLeft className="h-4 w-4" />
        ไปหน้าห้อง {roomNo}
      </Link>
    </main>
  )
}
