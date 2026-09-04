'use client'

// Report HK — ONE ROOM's daily report (/hk/rooms/[roomId]/report).
//
// Three screens in one route, switched by WHO is looking and WHERE the report
// stands (CONTEXT.md §Housekeeping "Room report" / "Report verification"):
//
//   maid   + nothing filed today  → THE ZONE STEPPER
//   maid   + returned report      → THE ZONE STEPPER again, ticks prefilled
//                                   with what she sent and ZERO photos, under
//                                   the canned reason it came back for. History
//                                   is append-only: this files a NEW report
//                                   carrying `parentReportId`, never an edit.
//   maid   + submitted            → read-only; she is waiting on reception,
//                                   and a maid NEVER verifies — including one
//                                   who also holds the reception grant.
//   reception + submitted         → THE VERIFY VIEW: her evidence grouped the
//                                   way it was shot, plus reception's own
//                                   photos and the two canned outcomes.
//   anyone + verified             → read-only summary with BOTH photo sets.
//
// THE ZONE STEPPER (owner directive 2026-09-02 — "1 picture for each tick",
// "fast and easy for a maid working against the clock and physically"). Four
// steps, one per `REPORT_ZONES` entry, then a review:
//
//   1. Tap the camera. The thumbnail is on screen before the upload starts,
//      and that zone's items appear PRE-TICKED ครบ against the shot.
//   2. Tap only what is wrong: an item cycles ครบ → หาย → ชำรุด, a quantity
//      stepper appears on a problem, and ถ่ายรูปใกล้ attaches a close-up that
//      backs that one tick instead of the zone shot.
//   3. ถัดไป. A perfect room is four shots, four ถัดไป, ส่งรายงาน, ยืนยัน —
//      TEN taps, with the room status already prefilled.
//
// Designed for one thumb, a glove, a corridor with bad signal, and a phone that
// locks mid-room:
//
//   * EVERY control is ≥44px and lives in the bottom third of the screen.
//   * UPLOADS RUN BEHIND HER. The queue retries with backoff, says
//     "อัปโหลดแล้ว 3/5" the whole time, and only the SUBMIT waits for it.
//   * THE DRAFT IS WRITTEN AFTER EVERY TAP (sessionStorage, keyed by branch +
//     room + day). A reload restores the ticks, the zone she was on and her
//     photo ids — each one re-checked against the server, because a photo that
//     is gone or already attached cannot back a tick. Ticks whose photo did not
//     survive are UNBOUND, never dropped: she keeps her judgements and owes
//     only the pictures.
//   * REMOVING A PHOTO NEVER SILENTLY LOSES TICKS — same rule, said in the UI.
//
// The branch is never chosen here — /hk and /hk/report are the two screens that
// may pick one. This one only READS what is stored (§A1: never guess, never
// default) and sends the user back to choose when there is nothing valid.

import { useCallback, useEffect, useMemo, useRef, useState } from 'react'
import Link from 'next/link'
import { useParams, useRouter } from 'next/navigation'
import {
  AlertCircle,
  AlertTriangle,
  Camera,
  Check,
  CheckCircle2,
  ChevronLeft,
  ChevronRight,
  ClipboardList,
  Images,
  Loader2,
  Minus,
  Plus,
  RefreshCw,
  Trash2,
  Undo2,
  X,
} from 'lucide-react'
import {
  applyZoneCapture,
  bindTickPhoto,
  buildReportTicks,
  canFileReport,
  canReport,
  canReturnReport,
  canSubmitReport,
  canVerifyReport,
  canVerifyReports,
  clearReportDraft,
  cycleTickPhoto,
  cycleTickState,
  deleteHkReportPhoto,
  downscalePhoto,
  fetchHkReportPhotoMeta,
  fetchHkRoomReport,
  findLocalPhoto,
  hkFetch,
  hkFetchMe,
  hkReportPhotoUrl,
  localPhotoKey,
  nextUploadPhoto,
  nextUploadWakeMs,
  photoChipLabel,
  prefillRoomStatus,
  readStoredBranch,
  reconcileReportDraft,
  reduceUploadQueue,
  REPORT_MAX_PHOTOS,
  REPORT_MAX_PHOTOS_TOTAL,
  REPORT_MAX_QTY,
  REPORT_MIN_QTY,
  REPORT_RETURNED_NOTICE,
  REPORT_SUBMITTED_NOTICE,
  REPORT_VERIFIED_NOTICE,
  REPORT_ZONES,
  reportDateLabel,
  reportExtraPhotoIds,
  reportItemRows,
  reportPhotoGroups,
  reportSidePhotoIds,
  reportState,
  reportStateChip,
  reportTicksByZone,
  reportTicksSubmission,
  reportZoneItems,
  reportZoneLabel,
  reportZoneProgress,
  resolveInitialBranch,
  RETURN_REASONS,
  returnHkReport,
  returnReasonLabel,
  ROOM_STATUS_CODES,
  roomStatusLabel,
  reportItemLabel,
  readReportDraft,
  signalActorLabel,
  signalRole,
  stashHkReportNotice,
  stepTickQty,
  submitHkReport,
  tickDraftFromReport,
  ticksBackedBy,
  tickStateLabel,
  timeLabel,
  unbindPhotoTicks,
  uploadCounts,
  uploadHkReportPhoto,
  uploadProgressLabel,
  uploadsSettled,
  verifyHkReport,
  writeReportDraft,
  type Branch,
  type HkLocalPhoto,
  type HkMe,
  type HkReport,
  type HkReportRoom,
  type HkRoomDetail,
  type HkTickDraft,
  type ReturnReason,
  type RoomStatusCode,
  type TickState,
} from '../../../hk-lib'
import { useHkAutoRefresh } from '../../../use-hk-auto-refresh'

/** The review step's index — one past the last zone. */
const STEP_REVIEW = REPORT_ZONES.length

/** Refused HERE rather than after a slow upload. */
const PHOTO_LIMIT_MESSAGE = `แนบรูปได้ไม่เกิน ${REPORT_MAX_PHOTOS_TOTAL} รูปต่อรายงาน`
const RECEPTION_PHOTO_LIMIT_MESSAGE = `แนบรูปได้ไม่เกิน ${REPORT_MAX_PHOTOS} รูป`
const DRAFT_PHOTOS_DROPPED_NOTICE = 'รูปบางรูปใช้ไม่ได้แล้ว กรุณาถ่ายใหม่ในโซนที่ยังไม่มีรูป'

/** What the album control SAYS. Short on purpose — it has to sit beside a
 *  camera button without shrinking it. The aria-label at each call site is the
 *  long, unique one. */
const GALLERY_BUTTON_TEXT = 'เลือกรูป'

/** …and how it LOOKS: secondary (white, grey border) so the camera stays the
 *  obvious tap, but never under 44px. */
const GALLERY_BUTTON_CLASS =
  'min-h-[44px] shrink-0 rounded-lg border border-gray-300 bg-white px-3 text-sm font-semibold text-gray-700 active:bg-gray-100'

/** A local object URL for the shot she just took, or '' where the runtime has
 *  no `createObjectURL` (jsdom, an ancient WebView) — the thumbnail then waits
 *  for the uploaded copy rather than breaking the row. */
function previewUrl(blob: Blob): string {
  try {
    return URL.createObjectURL(blob)
  } catch {
    return ''
  }
}

function revokePreview(url: string | undefined): void {
  if (!url) return
  try {
    URL.revokeObjectURL(url)
  } catch {
    /* nothing to do */
  }
}

/**
/**
 * ONE file control: a label wrapping a visually-hidden input, because a bare
 * file input cannot be made 44px tall reliably across WebViews.
 *
 * TWO sources, and BOTH are always offered (HF Ville, 2026-09-04):
 *
 *   * `camera` — `capture="environment"`, which opens the camera straight away
 *     instead of a file browser. The primary control, and the bigger one.
 *   * `gallery` — deliberately NO `capture` and `multiple`, i.e. the album.
 *     An old Android / LINE in-app WebView is unreliable re-launching the
 *     camera intent after the FIRST shot, which is why a maid reported being
 *     "limited to one photo": there was no second way in. The album picker is
 *     that way, and it hands over several files at once.
 *
 * `label` is what assistive tech and the tests see; `text` is what fits on the
 * button, so the album control can read "เลือกรูป" while still being uniquely
 * addressable next to three other album controls on the same screen.
 */
function PhotoInputButton({
  label,
  text,
  source,
  onPick,
  disabled,
  busy,
  className,
  icon,
}: {
  label: string
  text?: string
  source: 'camera' | 'gallery'
  onPick: (files: FileList | null) => void
  disabled?: boolean
  busy?: boolean
  className?: string
  icon?: React.ReactNode
}) {
  const isCamera = source === 'camera'
  return (
    <label
      className={`flex items-center justify-center gap-1.5 ${className ?? ''} ${
        disabled ? 'opacity-50' : ''
      }`}
    >
      {busy ? (
        <Loader2 className="h-5 w-5 animate-spin" />
      ) : (
        (icon ?? (isCamera ? <Camera className="h-5 w-5" /> : <Images className="h-5 w-5" />))
      )}
      <span>{text ?? label}</span>
      <input
        type="file"
        accept="image/*"
        // `undefined`/`false` render NO attribute, which is the point: a
        // `capture` on the album control would send her back to the camera.
        capture={isCamera ? 'environment' : undefined}
        multiple={!isCamera}
        aria-label={label}
        disabled={disabled}
        className="sr-only"
        onChange={(e) => {
          onPick(e.target.files)
          // Let the SAME photo be picked twice (a retake of a shot she just
          // removed): without this the input holds the old value and fires no
          // change event.
          e.target.value = ''
        }}
      />
    </label>
  )
}

/** The camera, unchanged for every caller that already had one. */
function CameraButton(props: Omit<React.ComponentProps<typeof PhotoInputButton>, 'source'>) {
  return <PhotoInputButton source="camera" {...props} />
}

/** The album, always rendered beside a camera. Secondary by colour, never by
 *  size — 44px is the floor for a gloved thumb in a corridor. */
function GalleryButton({
  label,
  onPick,
  disabled,
  className,
}: {
  label: string
  onPick: (files: FileList | null) => void
  disabled?: boolean
  className?: string
}) {
  return (
    <PhotoInputButton
      source="gallery"
      label={label}
      text={GALLERY_BUTTON_TEXT}
      onPick={onPick}
      disabled={disabled}
      className={className ?? GALLERY_BUTTON_CLASS}
    />
  )
}

/** ครบ / หาย / ชำรุด, as one chip vocabulary shared by the form and every
 *  read-only surface — two spellings of a tick is how a report stops meaning
 *  one thing. */
function tickChipClass(state: string): string {
  if (state === 'ok') return 'border-emerald-600 bg-emerald-600 text-white'
  if (state === 'damaged') return 'border-orange-600 bg-orange-600 text-white'
  return 'border-red-600 bg-red-600 text-white'
}

/** One captured shot in a zone's strip: tap to view it full-screen, or remove
 *  it. The thumbnail is the LOCAL preview while it exists, so it is on screen
 *  before the upload is, and falls back to the stored copy afterwards. */
function DraftThumb({
  photo,
  index,
  count,
  zoneLabel,
  preview,
  branch,
  disabled,
  onOpen,
  onRemove,
}: {
  photo: HkLocalPhoto
  index: number
  count: number
  zoneLabel: string
  preview: string
  branch: Branch | null
  disabled: boolean
  onOpen: () => void
  onRemove: () => void
}) {
  const src = preview || (photo.photoId !== null ? hkReportPhotoUrl(photo.photoId, branch) : '')
  const name = `รูปที่ ${index + 1} โซน${zoneLabel}`
  return (
    <li className="relative" data-testid={`hk-draft-photo-${photo.key}`}>
      <button
        type="button"
        aria-label={`ดู${name}`}
        onClick={onOpen}
        className="block h-24 w-full overflow-hidden rounded-lg border border-gray-200 bg-gray-100"
      >
        {src ? (
          /* eslint-disable-next-line @next/next/no-img-element */
          <img src={src} alt={name} className="h-24 w-full object-cover" />
        ) : (
          <span className="flex h-24 w-full items-center justify-center text-gray-400">
            <Images className="h-6 w-6" />
          </span>
        )}
      </button>
      <button
        type="button"
        aria-label={`ลบ${name}`}
        onClick={onRemove}
        disabled={disabled}
        className="absolute -right-1 -top-1 flex h-9 w-9 items-center justify-center rounded-full border border-gray-300 bg-white text-gray-600 active:bg-gray-100 disabled:opacity-50"
      >
        <X className="h-4 w-4" />
      </button>
      {/* The upload's state, on the picture it belongs to: a spinner while it
          is going up, a tick when it has landed, a word when it has not. */}
      <span className="absolute bottom-1 left-1 rounded bg-black/60 px-1.5 py-0.5 text-[10px] font-semibold text-white">
        {photo.status === 'uploaded' ? (
          <Check className="inline h-3 w-3" />
        ) : photo.status === 'failed' ? (
          'ยังไม่ขึ้น'
        ) : (
          <Loader2 className="inline h-3 w-3 animate-spin" />
        )}
        <span className="ml-1">
          {index + 1}/{count}
        </span>
      </span>
    </li>
  )
}

/**
 * ONE ITEM's tick. The whole per-item interaction is here and it is one tap:
 * the row IS the button and it cycles ครบ → หาย → ชำรุด. Everything else
 * (quantity, close-up, which photo backs it) appears only once it is relevant,
 * because a maid reading twenty-two rows of controls is a maid reading, not
 * working.
 */
function TickRow({
  item,
  label,
  entry,
  photos,
  rebindable,
  busy,
  onCycle,
  onStep,
  onCyclePhoto,
  onCloseUp,
}: {
  item: string
  label: string
  entry: { state: TickState; qty: number | null; photo: string | null } | undefined
  photos: HkLocalPhoto[]
  rebindable: boolean
  busy: boolean
  onCycle: () => void
  onStep: (delta: number) => void
  onCyclePhoto: () => void
  onCloseUp: (files: FileList | null) => void
}) {
  // No entry at all = this zone has not been photographed yet. The row says so
  // rather than offering a tick that nothing could back.
  if (!entry) {
    return (
      <li
        data-testid={`hk-tick-${item}`}
        className="flex items-center justify-between gap-2 rounded-xl border border-dashed border-gray-300 bg-gray-50 px-3 py-3"
      >
        <span className="text-sm font-medium text-gray-500">{label}</span>
        <span className="text-xs text-gray-400">ถ่ายรูปโซนนี้ก่อน</span>
      </li>
    )
  }
  const problem = entry.state !== 'ok'
  const chip = photoChipLabel(photos, entry.photo)
  return (
    <li
      data-testid={`hk-tick-${item}`}
      className={`rounded-xl border px-3 py-2 ${
        problem ? 'border-red-300 bg-red-50' : 'border-gray-200 bg-white'
      }`}
    >
      {/* The row itself is the control — a 44px target the width of the screen,
          which is what a gloved thumb in a corridor can actually hit. */}
      <button
        type="button"
        aria-label={`${label} ${tickStateLabel(entry.state)}`}
        onClick={onCycle}
        disabled={busy}
        className="flex min-h-[44px] w-full items-center justify-between gap-2 disabled:opacity-50"
      >
        <span className="text-left text-sm font-medium text-gray-800">{label}</span>
        <span
          className={`shrink-0 rounded-full border px-3 py-1.5 text-sm font-bold ${tickChipClass(
            entry.state
          )}`}
        >
          {tickStateLabel(entry.state)}
        </span>
      </button>

      {/* Evidence owed: the photo that backed this tick was removed. Said out
          loud, because a tick nobody can see is a tick she thinks she made. */}
      {!entry.photo && (
        <p className="mt-1 flex items-center gap-1 text-xs font-semibold text-amber-700">
          <AlertTriangle className="h-3.5 w-3.5 shrink-0" />
          ต้องถ่ายรูปใหม่
        </p>
      )}

      {problem && (
        <div className="mt-2 flex items-center justify-between gap-2 rounded-lg border border-red-200 bg-white px-2 py-2">
          <span className="text-xs font-medium text-red-800">จำนวน</span>
          <span className="flex items-center gap-2">
            <button
              type="button"
              aria-label={`ลด ${label}`}
              onClick={() => onStep(-1)}
              disabled={busy || (entry.qty ?? REPORT_MIN_QTY) <= REPORT_MIN_QTY}
              className="flex h-11 w-11 items-center justify-center rounded-lg border border-red-300 bg-white text-red-800 active:bg-red-100 disabled:opacity-40"
            >
              <Minus className="h-5 w-5" />
            </button>
            <span className="w-7 text-center text-base font-semibold tabular-nums text-gray-900">
              {entry.qty ?? REPORT_MIN_QTY}
            </span>
            <button
              type="button"
              aria-label={`เพิ่ม ${label}`}
              onClick={() => onStep(1)}
              disabled={busy || (entry.qty ?? REPORT_MIN_QTY) >= REPORT_MAX_QTY}
              className="flex h-11 w-11 items-center justify-center rounded-lg border border-red-300 bg-white text-red-800 active:bg-red-100 disabled:opacity-40"
            >
              <Plus className="h-5 w-5" />
            </button>
          </span>
        </div>
      )}

      <div className="mt-2 flex items-center justify-between gap-2">
        {/* Which of the zone's shots vouches for this item. One tap cycles to
            the next — a two-way choice does not deserve a menu. Hidden when
            the zone has a single photo, which is the usual room. */}
        {chip && rebindable ? (
          <button
            type="button"
            aria-label={`เปลี่ยนรูปของ ${label}`}
            onClick={onCyclePhoto}
            disabled={busy}
            className="min-h-[36px] rounded-full border border-gray-300 bg-white px-3 text-xs font-semibold text-gray-600 active:bg-gray-100 disabled:opacity-50"
          >
            {chip}
          </button>
        ) : (
          <span className="text-xs text-gray-400">{chip}</span>
        )}
        {/* A close-up is ALLOWED, never demanded: the server does not enforce
            it (CONTEXT.md §Housekeeping "Photo-backed tick") and a maid who cannot get closer
            must still be able to file. */}
        {problem && (
          <span className="flex items-center gap-2">
            <CameraButton
              label={`ถ่ายรูปใกล้ ${label}`}
              onPick={onCloseUp}
              disabled={busy}
              className="min-h-[44px] rounded-lg border border-red-400 bg-white px-3 text-xs font-semibold text-red-700 active:bg-red-50"
            />
            <GalleryButton
              label={`เลือกรูปใกล้ ${label}`}
              onPick={onCloseUp}
              disabled={busy}
              className="min-h-[44px] shrink-0 rounded-lg border border-gray-300 bg-white px-3 text-xs font-semibold text-gray-700 active:bg-gray-100"
            />
          </span>
        )}
      </div>
    </li>
  )
}

/** The full-screen viewer. One picture, big, with the list of items it backs —
 *  which is what makes "can I delete this?" answerable. */
function PhotoViewer({
  src,
  caption,
  items,
  onClose,
  onPrev,
  onNext,
  onRemove,
  onRetake,
}: {
  src: string
  caption: string
  items: string[]
  onClose: () => void
  onPrev?: () => void
  onNext?: () => void
  onRemove?: () => void
  /** Remove-and-shoot-again in ONE tap: the whole reason a maid opens a photo
   *  full-screen is to decide whether it will do, and "no" should not cost her
   *  two controls in two places. */
  onRetake?: (files: FileList | null) => void
}) {
  return (
    <div
      data-testid="hk-photo-viewer"
      className="fixed inset-0 z-50 flex flex-col bg-black/90 p-3"
    >
      <div className="flex items-center justify-between gap-2 text-white">
        <span className="text-sm font-semibold">{caption}</span>
        <button
          type="button"
          aria-label="ปิดรูป"
          onClick={onClose}
          className="flex h-11 w-11 items-center justify-center rounded-full border border-white/40"
        >
          <X className="h-5 w-5" />
        </button>
      </div>
      <div className="flex flex-1 items-center justify-center overflow-hidden">
        {src ? (
          /* eslint-disable-next-line @next/next/no-img-element */
          <img src={src} alt={caption} className="max-h-full max-w-full object-contain" />
        ) : (
          <Images className="h-10 w-10 text-white/60" />
        )}
      </div>
      <div className="mt-2 rounded-xl bg-white/95 p-3">
        <p className="text-xs font-semibold text-gray-500">รูปนี้ยืนยันรายการ</p>
        {items.length > 0 ? (
          <p className="mt-1 text-sm text-gray-800">{items.join(' · ')}</p>
        ) : (
          <p className="mt-1 text-sm text-gray-500">รูปเพิ่มเติม (ยังไม่ผูกกับรายการ)</p>
        )}
        <div className="mt-3 flex items-center justify-between gap-2">
          <span className="flex gap-2">
            {onPrev && (
              <button
                type="button"
                aria-label="รูปก่อนหน้า"
                onClick={onPrev}
                className="flex h-11 w-11 items-center justify-center rounded-lg border border-gray-300"
              >
                <ChevronLeft className="h-5 w-5" />
              </button>
            )}
            {onNext && (
              <button
                type="button"
                aria-label="รูปถัดไป"
                onClick={onNext}
                className="flex h-11 w-11 items-center justify-center rounded-lg border border-gray-300"
              >
                <ChevronRight className="h-5 w-5" />
              </button>
            )}
          </span>
          <span className="flex gap-2">
            {onRetake && (
              <>
                <CameraButton
                  label="ถ่ายรูปนี้ใหม่"
                  onPick={onRetake}
                  className="min-h-[44px] rounded-lg border border-teal-600 bg-teal-600 px-3 text-sm font-semibold text-white active:bg-teal-700"
                />
                <GalleryButton label="เลือกรูปแทนรูปนี้" onPick={onRetake} />
              </>
            )}
            {onRemove && (
              <button
                type="button"
                onClick={onRemove}
                className="flex min-h-[44px] items-center gap-1.5 rounded-lg border border-red-400 bg-white px-3 text-sm font-semibold text-red-700 active:bg-red-50"
              >
                <Trash2 className="h-4 w-4" />
                ลบรูปนี้
              </button>
            )}
          </span>
        </div>
      </div>
    </div>
  )
}

/**
 * The photo strip reception fills in — capture button, thumbnails, remove.
 * Unchanged from v1 on purpose: a verify is still 1..4 photos of her own, and
 * the tick model is about the MAID's evidence.
 */
function ReportPhotoStrip({
  captureLabel,
  galleryLabel,
  branch,
  photoIds,
  onPick,
  onRemove,
  uploading,
  disabled,
  testId,
}: {
  captureLabel: string
  galleryLabel: string
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
      <div className="flex items-stretch gap-2">
        <CameraButton
          label={captureLabel}
          onPick={onPick}
          busy={uploading}
          disabled={disabled || uploading || full}
          className="min-h-[44px] flex-1 rounded-lg border border-teal-400 bg-white px-3 py-3 text-sm font-semibold text-teal-800 active:bg-teal-50"
        />
        {/* Reception's own escape hatch — a desk phone whose camera intent
            misbehaves is the same bug on the other side of the counter. */}
        <GalleryButton
          label={galleryLabel}
          onPick={onPick}
          disabled={disabled || uploading || full}
        />
      </div>
      <p className="mt-1 text-[11px] text-gray-400">
        แนบรูปได้ {photoIds.length}/{REPORT_MAX_PHOTOS} รูป
      </p>
    </div>
  )
}

/** Somebody else's photos, read-only and ungrouped — the fallback for a report
 *  that predates `photos` metadata, and reception's own set. */
function ReportPhotoGallery({
  title,
  branch,
  photoIds,
  testId,
  onOpen,
}: {
  title: string
  branch: Branch | null
  photoIds: number[]
  testId: string
  onOpen?: (photoId: number) => void
}) {
  if (photoIds.length === 0) return null
  return (
    <div data-testid={testId} className="mt-3">
      <p className="mb-1 text-xs font-semibold text-gray-500">{title}</p>
      <ul className="grid grid-cols-4 gap-2">
        {photoIds.map((photoId) => (
          <li key={photoId}>
            <button
              type="button"
              aria-label={`ดู${title} ${photoId}`}
              onClick={() => onOpen?.(photoId)}
              className="block w-full"
            >
              {/* eslint-disable-next-line @next/next/no-img-element */}
              <img
                src={hkReportPhotoUrl(photoId, branch)}
                alt={`${title} ${photoId}`}
                className="h-20 w-full rounded-lg border border-gray-200 object-cover"
              />
            </button>
          </li>
        ))}
      </ul>
    </div>
  )
}

/**
 * THE VERIFY VIEW's body: the maid's photos grouped BY CAPTURE ZONE, each
 * carrying the items it vouches for — reception judges a picture against the
 * ticks it backs, which is the whole point of the photo-backed model. Problem
 * items lead their photo with the quantity in the largest type on the row.
 */
function ReportZoneEvidence({
  report,
  branch,
  onOpen,
}: {
  report: HkReport
  branch: Branch | null
  onOpen: (photoId: number) => void
}) {
  const groups = reportPhotoGroups(report.photos, report.ticks, 'maid')
  if (groups.length === 0) return null
  return (
    <div data-testid="hk-report-zone-evidence" className="space-y-3">
      {groups.map((group) => (
        <div
          key={group.zone || 'other'}
          data-testid={`hk-report-zone-${group.zone || 'other'}`}
          className="rounded-xl border border-gray-200 bg-white p-3"
        >
          <p className="mb-2 text-sm font-semibold text-gray-700">{group.label}</p>
          <ul className="space-y-3">
            {group.photos.map((photo) => (
              <li key={photo.photoId}>
                <button
                  type="button"
                  aria-label={`ดูรูป ${photo.photoId} โซน${group.label}`}
                  onClick={() => onOpen(photo.photoId)}
                  className="block w-full"
                >
                  {/* eslint-disable-next-line @next/next/no-img-element */}
                  <img
                    src={hkReportPhotoUrl(photo.photoId, branch)}
                    alt={`รูปโซน${group.label} ${photo.photoId}`}
                    className="h-40 w-full rounded-lg border border-gray-200 object-cover"
                  />
                </button>
                {photo.ticks.length > 0 ? (
                  <ul className="mt-1 flex flex-wrap gap-1">
                    {photo.ticks.map((tick) => (
                      <li
                        key={tick.key}
                        className={`rounded-full border px-2 py-0.5 text-xs ${
                          tick.problem
                            ? 'border-red-300 bg-red-50 font-bold text-red-800'
                            : 'border-gray-200 bg-gray-50 text-gray-600'
                        }`}
                      >
                        {tick.label} · {tick.stateLabel}
                        {tick.problem ? ` ${tick.qty ?? ''}` : ''}
                      </li>
                    ))}
                  </ul>
                ) : (
                  <p className="mt-1 text-xs text-gray-400">รูปเพิ่มเติม</p>
                )}
              </li>
            ))}
          </ul>
        </div>
      ))}
    </div>
  )
}

/** What was reported, as text: the 22 ticks grouped by zone for a v2 report,
 *  or v1's exception list for one filed before the ticks existed. */
function ReportTickSummary({ report }: { report: HkReport }) {
  const groups = reportTicksByZone(report.ticks)
  if (groups.length === 0) {
    const rows = reportItemRows(report.items)
    // The flag says the room was fine; the rows say what was not. When they
    // disagree the ROWS win — showing "ครบทุกรายการ" over a list of missing
    // items is the one failure here that could cost a guest a charge nobody
    // can explain.
    if (report.allItemsOk !== false && rows.length === 0) {
      return (
        <p className="flex items-center gap-1.5 text-base font-semibold text-emerald-700">
          <CheckCircle2 className="h-4 w-4 shrink-0" />
          ครบทุกรายการ
        </p>
      )
    }
    return (
      <ul data-testid="hk-report-exceptions" className="mt-1 space-y-1">
        {rows.map(({ key, label, problemLabel, qty }) => (
          <li
            key={key}
            className="flex items-center justify-between gap-2 rounded-lg border border-red-200 bg-red-50 px-3 py-2"
          >
            <span className="text-sm font-medium text-gray-800">
              {label} · {problemLabel}
            </span>
            <span className="text-base font-semibold tabular-nums text-red-800">{qty}</span>
          </li>
        ))}
      </ul>
    )
  }
  return (
    <div data-testid="hk-report-ticks" className="mt-1 space-y-3">
      {groups.map((group) => (
        <div key={group.zone || 'other'}>
          <p className="mb-1 text-xs font-semibold text-gray-500">{group.label}</p>
          <ul className="space-y-1">
            {group.ticks.map((tick) => (
              <li
                key={tick.key}
                className={`flex items-center justify-between gap-2 rounded-lg border px-3 py-2 ${
                  tick.problem ? 'border-red-200 bg-red-50' : 'border-gray-200 bg-white'
                }`}
              >
                <span className="text-sm font-medium text-gray-800">{tick.label}</span>
                <span className="flex items-center gap-2">
                  <span
                    className={`rounded-full border px-2 py-0.5 text-xs font-semibold ${tickChipClass(
                      tick.state
                    )}`}
                  >
                    {tick.stateLabel}
                  </span>
                  {tick.problem && (
                    <span className="text-base font-bold tabular-nums text-red-800">
                      {tick.qty ?? ''}
                    </span>
                  )}
                </span>
              </li>
            ))}
          </ul>
        </div>
      ))}
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

  // --- the maid's stepper ---------------------------------------------------
  const [step, setStep] = useState(0)
  const [roomStatus, setRoomStatus] = useState<RoomStatusCode | null>(null)
  const [ticks, setTicks] = useState<HkTickDraft>({})
  const [photos, setPhotos] = useState<HkLocalPhoto[]>([])
  const [viewerKey, setViewerKey] = useState<string | null>(null)
  // Bumped when a timer or a settled upload should make the queue look again.
  const [queueTick, setQueueTick] = useState(0)

  // The bytes and the object URLs live in refs, NEVER in state: they are not
  // serializable, they must not be written to the draft, and a re-render must
  // not re-create them. Keyed by the local photo key.
  const blobsRef = useRef<Record<string, Promise<Blob>>>({})
  const previewsRef = useRef<Record<string, string>>({})
  const seqRef = useRef(0)
  const inFlightRef = useRef(false)
  const mountedRef = useRef(true)

  // --- reception's verify ---------------------------------------------------
  const [receptionPhotoIds, setReceptionPhotoIds] = useState<number[]>([])
  const [returnReason, setReturnReason] = useState<ReturnReason | null>(null)
  const [viewerPhotoId, setViewerPhotoId] = useState<number | null>(null)

  const [uploading, setUploading] = useState(false)
  const [submitting, setSubmitting] = useState(false)
  // In-place confirms, the /hk idiom: the first tap arms, the second files.
  const [confirming, setConfirming] = useState<'submit' | 'verify' | 'return' | null>(null)

  // The form is seeded EXACTLY ONCE, on the first successful load. A poll that
  // re-seeded would wipe a half-filled room under a maid's thumb — the whole
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
          await seedForm(branch, day.date, day.report, roomDetail)
        }
      } catch (err) {
        if (!background) setError(err instanceof Error ? err.message : 'เกิดข้อผิดพลาด')
      } finally {
        if (!background) setLoading(false)
      }
    },
    // eslint-disable-next-line react-hooks/exhaustive-deps
    [roomId, branch]
  )

  /**
   * Seed the stepper, ONCE. Three sources, in this order:
   *
   *   1. A saved DRAFT for this branch + room + day — she was here minutes ago
   *      and the phone locked. Its photo ids are re-checked against the server
   *      before they are trusted (`fetchHkReportPhotoMeta`), and anything that
   *      is gone or already attached is dropped with its ticks UNBOUND.
   *   2. A RETURNED report — her previous ticks come back so she fixes what was
   *      wrong, with ZERO photos: reception rejected the evidence, and
   *      re-sending it is not a fix.
   *   3. Nothing — an empty checklist and the room-status prefill.
   */
  const seedForm = async (
    forBranch: Branch,
    day: string,
    filed: HkReport | null,
    roomDetail: HkRoomDetail
  ) => {
    const returned = filed?.status === 'returned'
    const previousStatus = ROOM_STATUS_CODES.find(({ code }) => code === filed?.roomStatus)?.code
    const fallbackStatus =
      (returned ? previousStatus : undefined) ?? prefillRoomStatus(roomDetail.room)

    const stored = readReportDraft(forBranch, roomId, day)
    if (stored) {
      const metas = await Promise.all(
        stored.photos.map((photo) =>
          photo.photoId === null
            ? Promise.resolve(null)
            : fetchHkReportPhotoMeta(forBranch, photo.photoId)
        )
      )
      const usable = metas
        .filter((meta) => meta !== null && meta.attached !== true)
        .map((meta) => (meta as { photoId: number }).photoId)
      const restored = reconcileReportDraft(stored, usable)
      setRoomStatus(restored.roomStatus ?? fallbackStatus)
      setTicks(restored.ticks)
      setPhotos(restored.photos)
      setStep(Math.min(Math.max(0, restored.step), STEP_REVIEW))
      seqRef.current = Math.max(restored.seq, stored.photos.length)
      if (restored.photos.length < stored.photos.length) {
        setNotice(DRAFT_PHOTOS_DROPPED_NOTICE)
      }
      return
    }

    setRoomStatus(fallbackStatus)
    setTicks(returned ? tickDraftFromReport(filed) : {})
    setPhotos([])
    setStep(0)
  }

  useEffect(() => {
    if (branch) load()
  }, [branch, load])

  const busy = uploading || submitting

  // Same contract as the room screen: poll while she is looking, never while a
  // write of hers is in flight. The seed guard above is what keeps a poll from
  // touching the stepper itself.
  const refresh = useCallback(() => load(true), [load])
  useHkAutoRefresh(refresh, Boolean(branch) && !busy)

  const role = signalRole(canReportFlag)
  const isMaid = canFileReport(role)
  const state = reportState(report)
  const room = detail?.room ?? null
  const roomNo = room?.roomNo ?? row?.roomNo ?? ''
  const showForm = isMaid && (state === 'unsent' || state === 'returned')
  const showVerify = canVerifyReports(role) && state === 'submitted' && report !== null

  // ------------------------------------------------------------------------
  // THE UPLOAD QUEUE. One photo at a time, behind her, with backoff. The rules
  // are the pure reducer's (`hk-lib`); this effect only owns the clock, the
  // bytes and the fetch.
  // ------------------------------------------------------------------------
  useEffect(() => {
    if (!branch || !showForm) return
    if (inFlightRef.current) return
    const now = Date.now()
    const next = nextUploadPhoto(photos, now)
    if (!next) {
      const wake = nextUploadWakeMs(photos, now)
      if (wake === null) return
      const timer = setTimeout(() => setQueueTick((n) => n + 1), Math.max(250, wake))
      return () => clearTimeout(timer)
    }
    const pending = blobsRef.current[next.key]
    if (!pending) {
      // Bytes we no longer hold (a reload dropped them). The photo cannot be
      // sent, so it goes — with its ticks unbound rather than deleted.
      setTicks((prev) => unbindPhotoTicks(prev, next.key))
      setPhotos((prev) => reduceUploadQueue(prev, { type: 'remove', key: next.key }))
      return
    }
    // NO per-effect `live` flag here, deliberately: dispatching `start` below
    // changes `photos`, which re-runs THIS effect, and a cleanup that cancelled
    // the in-flight upload would throw away every result the moment it started
    // one. The mounted ref is the only guard that is actually about the page
    // going away.
    inFlightRef.current = true
    setPhotos((prev) => reduceUploadQueue(prev, { type: 'start', key: next.key }))
    pending
      .then((blob) => uploadHkReportPhoto(branch, blob, { zone: next.zone }))
      .then(({ photoId, bytes }) => {
        if (!mountedRef.current) return
        setPhotos((prev) =>
          reduceUploadQueue(prev, { type: 'uploaded', key: next.key, photoId, bytes })
        )
      })
      .catch(() => {
        if (!mountedRef.current) return
        setPhotos((prev) =>
          reduceUploadQueue(prev, { type: 'failed', key: next.key, at: Date.now() })
        )
      })
      .finally(() => {
        inFlightRef.current = false
        if (mountedRef.current) setQueueTick((n) => n + 1)
      })
  }, [branch, showForm, photos, queueTick])

  // ------------------------------------------------------------------------
  // THE DRAFT. Written after every tap that changes anything — the phone that
  // locks mid-room is the design case, not the edge case.
  // ------------------------------------------------------------------------
  useEffect(() => {
    if (!showForm || !branch || !date) return
    writeReportDraft(branch, roomId, date, {
      roomStatus,
      step,
      ticks,
      photos,
      seq: seqRef.current,
    })
  }, [showForm, branch, date, roomId, roomStatus, step, ticks, photos])

  // Object URLs are a leak if nobody revokes them; a WebView that keeps this
  // page for a whole round would hold every shot of every room.
  useEffect(() => {
    mountedRef.current = true
    const previews = previewsRef.current
    return () => {
      mountedRef.current = false
      for (const url of Object.values(previews)) revokePreview(url)
    }
  }, [])

  /**
   * A shot has been taken. The thumbnail and the ticks land IMMEDIATELY; the
   * bytes are downscaled and uploaded behind her.
   *
   * `closeUpFor` makes this a close-up: the new photo backs that ONE tick
   * instead of pre-ticking the zone.
   */
  const capture = (zone: string, files: FileList | null, closeUpFor?: string) => {
    if (!branch || !files || files.length === 0) return
    const room = REPORT_MAX_PHOTOS_TOTAL - photos.length
    if (room <= 0) {
      setError(PHOTO_LIMIT_MESSAGE)
      return
    }
    setError(null)
    for (const file of Array.from(files).slice(0, room)) {
      const key = localPhotoKey(seqRef.current)
      seqRef.current += 1
      // Downscaled BEFORE the wire, and every failure path in the helper hands
      // the original file back rather than throwing — a maid must always be
      // able to file. Stored as a PROMISE so the tick can bind to the shot
      // before a single byte has been resized.
      blobsRef.current[key] = downscalePhoto(file)
      previewsRef.current[key] = previewUrl(file)
      setPhotos((prev) => reduceUploadQueue(prev, { type: 'add', key, zone }))
      setTicks((prev) =>
        closeUpFor ? bindTickPhoto(prev, closeUpFor, key) : applyZoneCapture(prev, zone, key)
      )
    }
  }

  /**
   * Remove one shot. The ticks it backed are UNBOUND, never deleted (they show
   * as ต้องถ่ายรูปใหม่ and the submit refuses on them), and the server-side row
   * is deleted BEST EFFORT — a maid in a corridor must not wait on a round trip
   * to retake a picture, and photos are kept forever anyway.
   */
  const removePhoto = (key: string) => {
    const photo = findLocalPhoto(photos, key)
    setTicks((prev) => unbindPhotoTicks(prev, key))
    setPhotos((prev) => reduceUploadQueue(prev, { type: 'remove', key }))
    revokePreview(previewsRef.current[key])
    delete previewsRef.current[key]
    delete blobsRef.current[key]
    if (viewerKey === key) setViewerKey(null)
    if (photo?.photoId != null && branch) {
      deleteHkReportPhoto(branch, photo.photoId).catch(() => {
        /* the row stays; photos are kept forever and an orphan is harmless */
      })
    }
  }

  /** Upload picked photos into reception's strip, honouring the 1..4 cap. */
  const addReceptionPhotos = async (files: FileList | null) => {
    if (!branch || !files || files.length === 0) return
    const room = REPORT_MAX_PHOTOS - receptionPhotoIds.length
    if (room <= 0) {
      setError(RECEPTION_PHOTO_LIMIT_MESSAGE)
      return
    }
    setUploading(true)
    setNotice(null)
    try {
      for (const file of Array.from(files).slice(0, room)) {
        const blob = await downscalePhoto(file)
        const { photoId } = await uploadHkReportPhoto(branch, blob)
        setReceptionPhotoIds((prev) => (prev.includes(photoId) ? prev : [...prev, photoId]))
      }
      setError(null)
    } catch (err) {
      setError(err instanceof Error ? err.message : 'เกิดข้อผิดพลาด')
    } finally {
      setUploading(false)
    }
  }

  // What the stepper would send right now — also the answer to "may she
  // submit", so the button and the request can never disagree.
  const tickDrafts = useMemo(() => buildReportTicks(ticks, photos), [ticks, photos])
  const extraPhotoIds = useMemo(
    () => reportExtraPhotoIds(tickDrafts, photos),
    [tickDrafts, photos]
  )
  const zoneProgress = useMemo(() => reportZoneProgress(ticks, photos), [ticks, photos])
  const queue = uploadCounts(photos)
  const settled = uploadsSettled(photos)
  const submitReady =
    settled && canSubmitReport({ roomStatus, ticks: tickDrafts, extraPhotoIds })

  const submit = async () => {
    if (!branch || !isMaid || !roomStatus || !submitReady) return
    const body = reportTicksSubmission(tickDrafts)
    if (!body) return
    setSubmitting(true)
    setNotice(null)
    try {
      await submitHkReport(branch, roomId, {
        roomStatus,
        ticks: body,
        ...(extraPhotoIds.length > 0 ? { extraPhotoIds } : {}),
        // Append-only: a fix POINTS AT the report it supersedes rather than
        // editing it. Absent for a first submission of the day.
        ...(state === 'returned' && report ? { parentReportId: report.reportId } : {}),
      })
      clearReportDraft(branch, roomId, date)
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
  const zone = step < STEP_REVIEW ? REPORT_ZONES[step] : null
  const zoneShots = zone ? photos.filter((photo) => photo.zone === zone.zone) : []
  const viewerPhoto = findLocalPhoto(photos, viewerKey)
  // The filed report's photos, flattened in the order the verify view shows
  // them — the full-screen viewer's next/previous.
  const filedPhotoIds = report
    ? [
        ...reportPhotoGroups(report.photos, report.ticks, 'maid').flatMap((group) =>
          group.photos.map((photo) => photo.photoId)
        ),
        ...reportSidePhotoIds(report, 'reception'),
      ]
    : []
  const viewerIndex = viewerPhotoId === null ? -1 : filedPhotoIds.indexOf(viewerPhotoId)

  return (
    <main className={showForm ? 'pb-44' : ''}>
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
        <div className="mb-4 flex items-center gap-2 rounded-lg border border-amber-200 bg-amber-50 p-3 text-sm text-amber-800">
          <AlertTriangle className="h-5 w-5 shrink-0" />
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
       * THE ZONE STEPPER. Rendered for a maid with nothing filed today, and
       * for one fixing a returned report. A reception viewer never sees it at
       * all — hidden WHOLE rather than disabled, the same rule the room screen
       * follows: absence reads as "not my job", greyed-out reads as "broken".
       * ================================================================== */}
      {showForm && (
        <section data-testid="hk-report-form" className="space-y-4">
          {/* Where she is, in one line and four dots. */}
          <div data-testid="hk-zone-step" className="flex items-center justify-between gap-2">
            <p className="text-sm font-semibold text-gray-700">
              {step < STEP_REVIEW ? (
                <>
                  โซน {step + 1}/{STEP_REVIEW} · {zone?.label}
                </>
              ) : (
                'ตรวจทานก่อนส่ง'
              )}
            </p>
            <span className="flex items-center gap-1.5">
              {zoneProgress.map((progress) => (
                <span
                  key={progress.zone}
                  aria-label={`${progress.label} ${progress.done ? 'ครบแล้ว' : 'ยังไม่ครบ'}`}
                  className={`h-2.5 w-2.5 rounded-full ${
                    progress.done
                      ? 'bg-emerald-500'
                      : progress.index === step
                        ? 'bg-teal-600'
                        : 'bg-gray-300'
                  }`}
                />
              ))}
              <span
                className={`h-2.5 w-2.5 rounded-full ${
                  step === STEP_REVIEW ? 'bg-teal-600' : 'bg-gray-300'
                }`}
              />
            </span>
          </div>

          {/* ---------------------------------------------------------- *
           * ONE ZONE: its shots, then its items.
           * ---------------------------------------------------------- */}
          {zone && (
            <>
              {zoneShots.length > 0 ? (
                <ul className="grid grid-cols-3 gap-2">
                  {zoneShots.map((photo, index) => (
                    <DraftThumb
                      key={photo.key}
                      photo={photo}
                      index={index}
                      count={zoneShots.length}
                      zoneLabel={zone.label}
                      preview={previewsRef.current[photo.key] ?? ''}
                      branch={branch}
                      disabled={submitting}
                      onOpen={() => setViewerKey(photo.key)}
                      onRemove={() => removePhoto(photo.key)}
                    />
                  ))}
                </ul>
              ) : (
                <p className="rounded-xl border border-dashed border-teal-300 bg-teal-50 p-4 text-center text-sm font-semibold text-teal-800">
                  ถ่ายรูป{zone.label} 1 รูป แล้วรายการทั้งหมดจะถูกติ๊ก ครบ ให้อัตโนมัติ
                </p>
              )}

              <ul data-testid="hk-zone-items" className="space-y-2">
                {reportZoneItems(zone.zone).map((item) => (
                  <TickRow
                    key={item}
                    item={item}
                    label={reportItemLabel(item)}
                    entry={ticks[item]}
                    photos={photos}
                    rebindable={zoneShots.length > 1}
                    busy={submitting}
                    onCycle={() => setTicks((prev) => cycleTickState(prev, item))}
                    onStep={(delta) => setTicks((prev) => stepTickQty(prev, item, delta))}
                    onCyclePhoto={() => setTicks((prev) => cycleTickPhoto(prev, item, photos))}
                    onCloseUp={(files) => capture(zone.zone, files, item)}
                  />
                ))}
              </ul>
            </>
          )}

          {/* ---------------------------------------------------------- *
           * THE REVIEW STEP: all 22 ticks, grouped as they were shot, with
           * the picture that backs each one; then the room status and the
           * confirm.
           * ---------------------------------------------------------- */}
          {step === STEP_REVIEW && (
            <div data-testid="hk-report-review" className="space-y-4">
              {zoneProgress.map((progress) => (
                <div key={progress.zone}>
                  <p className="mb-1 flex items-center justify-between text-xs font-semibold text-gray-500">
                    <span>{progress.label}</span>
                    <span>
                      {progress.backedCount}/{progress.itemCount}
                      {progress.problemCount > 0 ? ` · ผิดปกติ ${progress.problemCount}` : ''}
                    </span>
                  </p>
                  <ul className="space-y-1">
                    {reportZoneItems(progress.zone).map((item) => {
                      const entry = ticks[item]
                      const photo = findLocalPhoto(photos, entry?.photo ?? null)
                      const src = photo
                        ? previewsRef.current[photo.key] ||
                          (photo.photoId !== null ? hkReportPhotoUrl(photo.photoId, branch) : '')
                        : ''
                      const problem = Boolean(entry && entry.state !== 'ok')
                      return (
                        <li
                          key={item}
                          data-testid={`hk-review-${item}`}
                          className={`flex items-center gap-2 rounded-lg border px-2 py-1.5 ${
                            problem
                              ? 'border-red-300 bg-red-50'
                              : entry?.photo
                                ? 'border-gray-200 bg-white'
                                : 'border-amber-300 bg-amber-50'
                          }`}
                        >
                          {src ? (
                            /* eslint-disable-next-line @next/next/no-img-element */
                            <img
                              src={src}
                              alt={`รูปของ ${reportItemLabel(item)}`}
                              className="h-10 w-10 shrink-0 rounded object-cover"
                            />
                          ) : (
                            <span className="flex h-10 w-10 shrink-0 items-center justify-center rounded bg-gray-100 text-gray-400">
                              <Camera className="h-4 w-4" />
                            </span>
                          )}
                          <span className="flex-1 text-sm text-gray-800">
                            {reportItemLabel(item)}
                          </span>
                          {entry ? (
                            <>
                              <span
                                className={`rounded-full border px-2 py-0.5 text-xs font-semibold ${tickChipClass(
                                  entry.state
                                )}`}
                              >
                                {tickStateLabel(entry.state)}
                              </span>
                              {problem && (
                                <span className="w-6 text-right text-base font-bold tabular-nums text-red-800">
                                  {entry.qty ?? REPORT_MIN_QTY}
                                </span>
                              )}
                            </>
                          ) : (
                            <span className="text-xs font-semibold text-amber-700">
                              ยังไม่ได้ติ๊ก
                            </span>
                          )}
                        </li>
                      )
                    })}
                  </ul>
                </div>
              ))}

              {/* --- room status ------------------------------------- */}
              <div>
                <h2 className="mb-2 text-sm font-semibold text-gray-600">สถานะห้อง</h2>
                {/* PREFILLED from the room's own facts (see `prefillRoomStatus`),
                    never locked: what she leaves selected is what is stored, so
                    a wrong guess costs one tap and the perfect room costs none. */}
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
            </div>
          )}

          {/* ---------------------------------------------------------- *
           * THE STICKY BAR. Every control she needs is here, in the bottom
           * third of the screen, at 56px: one thumb, a glove, a lift lobby.
           * ---------------------------------------------------------- */}
          <div className="fixed inset-x-0 bottom-0 z-40 border-t border-gray-200 bg-white p-3 shadow-[0_-2px_8px_rgba(0,0,0,0.06)]">
            {photos.length > 0 && (
              <div
                data-testid="hk-upload-status"
                className="mb-2 flex items-center justify-between gap-2 text-xs"
              >
                <span
                  className={
                    settled ? 'font-semibold text-emerald-700' : 'font-semibold text-amber-700'
                  }
                >
                  {settled ? (
                    <>
                      <Check className="mr-1 inline h-3.5 w-3.5" />
                      {uploadProgressLabel(photos)}
                    </>
                  ) : (
                    <>
                      <Loader2 className="mr-1 inline h-3.5 w-3.5 animate-spin" />
                      {uploadProgressLabel(photos)}
                    </>
                  )}
                </span>
                {queue.stuck > 0 && (
                  <button
                    type="button"
                    onClick={() => {
                      setPhotos((prev) => reduceUploadQueue(prev, { type: 'resume' }))
                      setQueueTick((n) => n + 1)
                    }}
                    className="flex min-h-[36px] items-center gap-1 rounded-lg border border-amber-400 bg-white px-3 font-semibold text-amber-800 active:bg-amber-50"
                  >
                    <RefreshCw className="h-3.5 w-3.5" />
                    ลองอัปโหลดใหม่
                  </button>
                )}
              </div>
            )}

            {step < STEP_REVIEW ? (
              <div className="space-y-2">
                <div className="flex items-center gap-2">
                  {step > 0 && (
                    <button
                      type="button"
                      aria-label="ย้อนกลับ"
                      onClick={() => setStep((n) => Math.max(0, n - 1))}
                      className="flex h-14 w-14 shrink-0 items-center justify-center rounded-xl border border-gray-300 bg-white text-gray-600 active:bg-gray-100"
                    >
                      <ChevronLeft className="h-6 w-6" />
                    </button>
                  )}
                  <CameraButton
                    label={`ถ่ายรูป${zone?.label ?? ''}`}
                    onPick={(files) => zone && capture(zone.zone, files)}
                    disabled={submitting}
                    className="min-h-[56px] flex-1 rounded-xl border border-teal-600 bg-teal-600 px-3 text-base font-bold text-white active:bg-teal-700"
                  />
                  <button
                    type="button"
                    onClick={() => setStep((n) => Math.min(STEP_REVIEW, n + 1))}
                    className="flex min-h-[56px] shrink-0 items-center gap-1 rounded-xl border border-gray-300 bg-white px-4 text-base font-semibold text-gray-800 active:bg-gray-100"
                  >
                    {step === STEP_REVIEW - 1 ? 'ตรวจทาน' : 'ถัดไป'}
                    <ChevronRight className="h-5 w-5" />
                  </button>
                </div>
                {/* THE SECOND WAY IN, on its own line so the camera keeps the
                    width it needs. This is what the HF Ville "limited to one
                    photo" report was really about: the LINE WebView stops
                    re-launching the camera intent after the first shot, and
                    until now there was no other door. `multiple`, so a whole
                    zone's worth of shots lands in one tap. */}
                <GalleryButton
                  label={`เลือกรูป${zone?.label ?? ''}`}
                  onPick={(files) => zone && capture(zone.zone, files)}
                  disabled={submitting}
                  className="min-h-[48px] w-full rounded-xl border border-teal-300 bg-white px-3 text-sm font-semibold text-teal-800 active:bg-teal-50"
                />
              </div>
            ) : confirming === 'submit' ? (
              /* Rendered IN PLACE of the button, never over it, ยกเลิก first
                 and calmer — the same shape as every other confirm on this
                 surface, so the reflex tap is the one that files nothing. */
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
                    className="min-h-[48px] rounded-lg border border-gray-300 bg-white px-3 text-sm font-semibold text-gray-700 active:bg-gray-100 disabled:opacity-50"
                  >
                    ยกเลิก
                  </button>
                  <button
                    type="button"
                    onClick={submit}
                    disabled={busy || !submitReady}
                    className="flex min-h-[48px] items-center justify-center rounded-lg border border-teal-600 bg-teal-600 px-3 text-sm font-semibold text-white active:bg-teal-700 disabled:opacity-50"
                  >
                    {submitting ? <Loader2 className="h-5 w-5 animate-spin" /> : 'ยืนยัน'}
                  </button>
                </div>
              </div>
            ) : (
              <div className="flex items-center gap-2">
                <button
                  type="button"
                  aria-label="ย้อนกลับ"
                  onClick={() => setStep(STEP_REVIEW - 1)}
                  className="flex h-14 w-14 shrink-0 items-center justify-center rounded-xl border border-gray-300 bg-white text-gray-600 active:bg-gray-100"
                >
                  <ChevronLeft className="h-6 w-6" />
                </button>
                {/* Step 1 — arms the confirm. Fires NO request. Dead until
                    every tick is photo-backed AND every photo has landed; the
                    line underneath says which of the two is missing, because
                    a disabled button with no reason is a bug report. */}
                <button
                  type="button"
                  onClick={() => setConfirming('submit')}
                  disabled={busy || !submitReady}
                  className="flex min-h-[56px] flex-1 items-center justify-center gap-1.5 rounded-xl border border-teal-600 bg-teal-600 px-3 text-base font-bold text-white active:bg-teal-700 disabled:opacity-50"
                >
                  <ClipboardList className="h-5 w-5" />
                  ส่งรายงาน
                </button>
              </div>
            )}

            {step === STEP_REVIEW && !submitReady && (
              <p data-testid="hk-submit-blocked" className="mt-2 text-center text-xs text-amber-700">
                {!settled
                  ? `รอรูปอัปโหลดให้ครบก่อน (${uploadProgressLabel(photos)})`
                  : 'ยังติ๊กไม่ครบทุกรายการ หรือมีรายการที่ยังไม่มีรูป'}
              </p>
            )}
          </div>
        </section>
      )}

      {/* ================================================================== *
       * WHAT WAS REPORTED — the read-only body of a filed report. Rendered
       * for every audience that is not filling in the stepper: the maid
       * waiting on her verification, reception about to verify (with her own
       * controls appended below), and both roles on a finished report.
       * ================================================================== */}
      {report && !showForm && (
        <section data-testid="hk-report-summary" className="mb-6 space-y-3">
          <div className="rounded-xl border border-gray-200 bg-white p-4">
            <p className="text-sm text-gray-500">สถานะห้อง</p>
            <p className="text-base font-semibold text-gray-900">
              {roomStatusLabel(report.roomStatus)}
            </p>

            <p className="mt-3 text-sm text-gray-500">อุปกรณ์ภายในห้อง</p>
            <ReportTickSummary report={report} />

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
          </div>

          {/* Her evidence, grouped the way it was SHOT, each picture carrying
              the items it vouches for. The gallery below it is the fallback
              for a v1 report, which has photo ids and no metadata. */}
          {(report.photos?.length ?? 0) > 0 ? (
            <ReportZoneEvidence
              report={report}
              branch={branch}
              onOpen={(photoId) => setViewerPhotoId(photoId)}
            />
          ) : (
            <ReportPhotoGallery
              testId="hk-report-maid-gallery"
              title="รูปจากแม่บ้าน"
              branch={branch}
              photoIds={reportSidePhotoIds(report, 'maid')}
              onOpen={(photoId) => setViewerPhotoId(photoId)}
            />
          )}
          <ReportPhotoGallery
            testId="hk-report-reception-gallery"
            title="รูปจากแผนกต้อนรับ"
            branch={branch}
            photoIds={reportSidePhotoIds(report, 'reception')}
            onOpen={(photoId) => setViewerPhotoId(photoId)}
          />

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
              galleryLabel="เลือกรูปการตรวจ"
              branch={branch}
              photoIds={receptionPhotoIds}
              uploading={uploading}
              disabled={submitting}
              onPick={addReceptionPhotos}
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
      {!showForm && (
        <Link
          href={`/hk/rooms/${roomId}`}
          className="mt-6 inline-flex items-center gap-1 text-sm text-gray-500"
        >
          <ChevronLeft className="h-4 w-4" />
          ไปหน้าห้อง {roomNo}
        </Link>
      )}

      {/* The maid's own draft photo, full-screen, with what it backs and the
          one control that can lose ticks — which is why the list is above it. */}
      {viewerPhoto && (
        <PhotoViewer
          src={
            previewsRef.current[viewerPhoto.key] ||
            (viewerPhoto.photoId !== null ? hkReportPhotoUrl(viewerPhoto.photoId, branch) : '')
          }
          caption={`${reportZoneLabel(viewerPhoto.zone)} · ${photoChipLabel(photos, viewerPhoto.key)}`}
          items={ticksBackedBy(ticks, viewerPhoto.key).map((item) => reportItemLabel(item))}
          onClose={() => setViewerKey(null)}
          onRemove={() => removePhoto(viewerPhoto.key)}
          onRetake={(files) => {
            // Order matters and is load-bearing: the removal unbinds this
            // photo's ticks, and the capture that follows re-binds exactly
            // those — a retake repairs the evidence without touching a single
            // one of her answers.
            removePhoto(viewerPhoto.key)
            capture(viewerPhoto.zone, files)
          }}
        />
      )}

      {/* A FILED report's photo, full-screen, with swipe-equivalent next/prev
          across everything the report carries. */}
      {viewerPhotoId !== null && report && (
        <PhotoViewer
          src={hkReportPhotoUrl(viewerPhotoId, branch)}
          caption={`รูปที่ ${viewerIndex + 1}/${filedPhotoIds.length}`}
          items={(report.ticks ?? [])
            .filter((tick) => tick.photoId === viewerPhotoId)
            .map((tick) => `${reportItemLabel(tick.item)} · ${tickStateLabel(tick.state)}`)}
          onClose={() => setViewerPhotoId(null)}
          onPrev={
            viewerIndex > 0 ? () => setViewerPhotoId(filedPhotoIds[viewerIndex - 1]) : undefined
          }
          onNext={
            viewerIndex >= 0 && viewerIndex < filedPhotoIds.length - 1
              ? () => setViewerPhotoId(filedPhotoIds[viewerIndex + 1])
              : undefined
          }
        />
      )}
    </main>
  )
}
