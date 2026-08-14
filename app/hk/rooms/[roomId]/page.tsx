'use client'

// Room screen (/hk/rooms/[roomId]) — report cleaning progress for one room,
// plus deep links to the Housekeeping ops app for แจ้งซ่อม / เบิกของ. Part of
// the maid-facing housekeeping surface (employee-login plan Phase 4, wave-4
// §A+B). The reporter identity is stamped SERVER-SIDE from the verified
// Cloudflare Access assertion; nothing identity-like is sent from this form.
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
  Loader2,
  Sparkles,
} from 'lucide-react'
import {
  hkFetch,
  hkFetchMe,
  progressLabel,
  readStoredBranch,
  resolveInitialBranch,
  timeLabel,
  type Branch,
  type CleaningStatus,
  type HkMe,
  type HkRoomDetail,
} from '../../hk-lib'
import HkOpsLinks from '../../HkOpsLinks'

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

  const load = useCallback(async () => {
    if (!Number.isFinite(roomId)) {
      setError('ไม่พบห้องนี้')
      setLoading(false)
      return
    }
    if (!branch) return
    setLoading(true)
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
      setError(err instanceof Error ? err.message : 'เกิดข้อผิดพลาด')
    } finally {
      setLoading(false)
    }
  }, [roomId, branch])

  useEffect(() => {
    if (branch) load()
  }, [branch, load])

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

      {room && (
        <>
          <header className="mb-4 rounded-xl border border-gray-200 bg-white p-4">
            <div className="flex items-center justify-between">
              <h1 className="text-2xl font-bold">ห้อง {room.roomNo}</h1>
              <span
                className={`inline-block rounded-full border px-2.5 py-1 text-xs ${badge.className}`}
              >
                {badge.label}
              </span>
            </div>
            <p className="mt-1 text-sm text-gray-500">
              {room.floor !== null ? `ชั้น ${room.floor}` : ''}
              {room.building ? ` · ${room.building}` : ''}
              {!room.roomClean && (
                <span className="ml-1 text-red-600">· ห้องยังไม่สะอาด</span>
              )}
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
            {markDirtyEnabled && (
              <button
                type="button"
                onClick={() => reportCleaning('dirty')}
                disabled={posting !== null}
                className="mt-3 flex w-full items-center justify-center gap-1.5 rounded-xl border border-red-300 bg-red-50 px-3 py-3 text-sm font-semibold text-red-800 active:bg-red-100 disabled:opacity-50"
              >
                {posting === 'dirty' ? (
                  <Loader2 className="h-5 w-5 animate-spin" />
                ) : (
                  <>
                    <AlertTriangle className="h-4 w-4" />
                    แจ้งห้องไม่สะอาด
                  </>
                )}
              </button>
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

          <HkOpsLinks />
        </>
      )}
    </main>
  )
}
