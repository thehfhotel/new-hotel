'use client'

// Room screen (/hk/rooms/[roomId]) — report cleaning progress for one room,
// plus deep links to the Housekeeping ops app for แจ้งซ่อม / เบิกของ. Part of
// the maid-facing housekeeping surface (employee-login plan Phase 4). The
// reporter identity is stamped SERVER-SIDE from the verified Cloudflare
// Access assertion; nothing identity-like is sent from this form.

import { useCallback, useEffect, useState } from 'react'
import Link from 'next/link'
import { useParams } from 'next/navigation'
import { AlertCircle, Check, ChevronLeft, Loader2, Sparkles } from 'lucide-react'
import {
  hkFetch,
  progressLabel,
  timeLabel,
  type CleaningStatus,
  type HkRoomDetail,
} from '../../hk-lib'
import HkOpsLinks from '../../HkOpsLinks'

export default function HkRoomPage() {
  const params = useParams<{ roomId: string }>()
  const roomId = Number(params?.roomId)

  const [detail, setDetail] = useState<HkRoomDetail | null>(null)
  const [loading, setLoading] = useState(true)
  const [error, setError] = useState<string | null>(null)
  const [posting, setPosting] = useState<CleaningStatus | null>(null)
  const [notice, setNotice] = useState<string | null>(null)

  const load = useCallback(async () => {
    if (!Number.isFinite(roomId)) {
      setError('ไม่พบห้องนี้')
      setLoading(false)
      return
    }
    try {
      const res = await hkFetch(`/rooms/${roomId}`)
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
  }, [roomId])

  useEffect(() => {
    load()
  }, [load])

  const reportCleaning = async (status: CleaningStatus) => {
    setPosting(status)
    setNotice(null)
    try {
      const res = await hkFetch(`/rooms/${roomId}/cleaning`, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ status }),
      })
      if (!res.ok) throw new Error('บันทึกไม่สำเร็จ กรุณาลองใหม่')
      setNotice(status === 'done' ? 'บันทึกแล้ว: เสร็จแล้ว' : 'บันทึกแล้ว: เริ่มทำความสะอาด')
      await load()
    } catch (err) {
      setError(err instanceof Error ? err.message : 'เกิดข้อผิดพลาด')
    } finally {
      setPosting(null)
    }
  }

  if (loading && !detail) {
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
