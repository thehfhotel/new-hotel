'use client'

// Room screen (/hk/rooms/[roomId]) — report cleaning progress or a broken
// item for one room. Part of the maid-facing housekeeping surface (employee-
// login plan Phase 4). The reporter identity is stamped SERVER-SIDE from the
// verified Cloudflare Access assertion; nothing identity-like is sent from
// this form.

import { useCallback, useEffect, useRef, useState } from 'react'
import Link from 'next/link'
import { useParams } from 'next/navigation'
import {
  AlertCircle,
  Camera,
  Check,
  ChevronLeft,
  Loader2,
  Sparkles,
  Wrench,
  X,
} from 'lucide-react'
import {
  hkFetch,
  preparePhoto,
  progressLabel,
  timeLabel,
  type CleaningStatus,
  type HkRoomDetail,
} from '../../hk-lib'

export default function HkRoomPage() {
  const params = useParams<{ roomId: string }>()
  const roomId = Number(params?.roomId)

  const [detail, setDetail] = useState<HkRoomDetail | null>(null)
  const [loading, setLoading] = useState(true)
  const [error, setError] = useState<string | null>(null)
  const [posting, setPosting] = useState<CleaningStatus | null>(null)
  const [notice, setNotice] = useState<string | null>(null)

  // Broken-item form state
  const [description, setDescription] = useState('')
  const [photoFile, setPhotoFile] = useState<File | null>(null)
  const [submittingReport, setSubmittingReport] = useState(false)
  const fileInputRef = useRef<HTMLInputElement>(null)

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

  const submitBrokenItem = async (event: React.FormEvent) => {
    event.preventDefault()
    if (!description.trim()) return
    setSubmittingReport(true)
    setNotice(null)
    try {
      let photo: { photoBase64: string; photoMime: string } | undefined
      if (photoFile) {
        photo = await preparePhoto(photoFile)
      }
      const res = await hkFetch(`/rooms/${roomId}/broken-items`, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ description: description.trim(), ...photo }),
      })
      if (!res.ok) throw new Error('ส่งรายงานไม่สำเร็จ กรุณาลองใหม่')
      setDescription('')
      setPhotoFile(null)
      if (fileInputRef.current) fileInputRef.current.value = ''
      setNotice('ส่งรายงานของชำรุดแล้ว')
      await load()
    } catch (err) {
      setError(err instanceof Error ? err.message : 'เกิดข้อผิดพลาด')
    } finally {
      setSubmittingReport(false)
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

          {/* Broken-item report */}
          <section className="mb-6">
            <h2 className="mb-2 flex items-center gap-1.5 text-sm font-semibold text-gray-600">
              <Wrench className="h-4 w-4 text-orange-600" />
              แจ้งของชำรุด / เสียหาย
            </h2>
            <form
              onSubmit={submitBrokenItem}
              className="rounded-xl border border-gray-200 bg-white p-3"
            >
              <textarea
                value={description}
                onChange={(e) => setDescription(e.target.value)}
                rows={3}
                placeholder="เช่น ก๊อกน้ำรั่ว, แอร์ไม่เย็น, รีโมทหาย..."
                className="w-full rounded-lg border border-gray-300 p-2 text-sm focus:border-red-500 focus:outline-none focus:ring-1 focus:ring-red-500"
              />
              <div className="mt-2 flex items-center justify-between gap-2">
                <label className="flex cursor-pointer items-center gap-1.5 text-sm text-gray-500">
                  <Camera className="h-5 w-5" />
                  {photoFile ? (
                    <span className="max-w-[10rem] truncate text-gray-700">
                      {photoFile.name}
                    </span>
                  ) : (
                    'แนบรูป (ถ้ามี)'
                  )}
                  <input
                    ref={fileInputRef}
                    type="file"
                    accept="image/*"
                    className="hidden"
                    onChange={(e) => setPhotoFile(e.target.files?.[0] ?? null)}
                  />
                </label>
                {photoFile && (
                  <button
                    type="button"
                    aria-label="ลบรูป"
                    onClick={() => {
                      setPhotoFile(null)
                      if (fileInputRef.current) fileInputRef.current.value = ''
                    }}
                    className="p-1 text-gray-400"
                  >
                    <X className="h-4 w-4" />
                  </button>
                )}
                <button
                  type="submit"
                  disabled={submittingReport || !description.trim()}
                  className="rounded-lg bg-red-600 px-4 py-2 text-sm font-semibold text-white active:bg-red-700 disabled:opacity-50"
                >
                  {submittingReport ? (
                    <Loader2 className="h-4 w-4 animate-spin" />
                  ) : (
                    'ส่งรายงาน'
                  )}
                </button>
              </div>
            </form>

            {detail && detail.reports.length > 0 && (
              <ul className="mt-3 space-y-2">
                {detail.reports.map((report) => (
                  <li
                    key={report.reportId}
                    className="rounded-lg border border-gray-200 bg-white p-3 text-sm"
                  >
                    <p className="text-gray-800">{report.description}</p>
                    <p className="mt-1 text-xs text-gray-400">
                      {timeLabel(report.at)}
                      {report.name ? ` โดย ${report.name}` : ` (${report.badge})`}
                      {report.status === 'open' ? ' · รอดำเนินการ' : ''}
                    </p>
                    {report.hasPhoto && (
                      // eslint-disable-next-line @next/next/no-img-element
                      <img
                        src={`/hk/api/broken-items/${report.reportId}/photo`}
                        alt="รูปของชำรุด"
                        loading="lazy"
                        className="mt-2 max-h-40 rounded-lg border border-gray-100 object-cover"
                      />
                    )}
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
