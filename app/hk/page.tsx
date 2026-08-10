'use client'

// Maid room list (/hk) — employee-login plan Phase 4.
//
// Shows every active room of the property with today's maid-reported cleaning
// progress; tapping a room opens /hk/rooms/[id] to report progress.
// แจ้งซ่อม / เบิกของ deep-link to the separate Housekeeping ops app via
// HkOpsLinks. Identity is resolved server-side from the Cloudflare Access
// assertion (silent HF ID); a 401/403 renders the fail-closed notice — there
// is deliberately NO login UI. v1 pins the primary property (HF Hotel); the
// backend is already branch-aware for a later property switch.

import { useCallback, useEffect, useState } from 'react'
import Link from 'next/link'
import { AlertCircle, Loader2, RefreshCw, Sparkles } from 'lucide-react'
import {
  groupRoomsByFloor,
  hkFetch,
  progressLabel,
  timeLabel,
  type HkMe,
  type HkRoom,
} from './hk-lib'
import HkOpsLinks from './HkOpsLinks'

export default function HkRoomListPage() {
  const [me, setMe] = useState<HkMe | null>(null)
  const [rooms, setRooms] = useState<HkRoom[]>([])
  const [loading, setLoading] = useState(true)
  const [error, setError] = useState<string | null>(null)

  const load = useCallback(async () => {
    setLoading(true)
    try {
      const [meRes, roomsRes] = await Promise.all([
        hkFetch('/me'),
        hkFetch('/rooms'),
      ])
      if (!meRes.ok || !roomsRes.ok) {
        throw new Error('ไม่สามารถดึงข้อมูลห้องได้ กรุณาลองใหม่')
      }
      const meData: HkMe = await meRes.json()
      const roomsData: { success: boolean; data: HkRoom[] } = await roomsRes.json()
      if (!meData.success || !roomsData.success) {
        throw new Error('ไม่สามารถดึงข้อมูลห้องได้ กรุณาลองใหม่')
      }
      setMe(meData)
      setRooms(roomsData.data)
      setError(null)
    } catch (err) {
      setError(err instanceof Error ? err.message : 'เกิดข้อผิดพลาด')
    } finally {
      setLoading(false)
    }
  }, [])

  useEffect(() => {
    load()
  }, [load])

  const doneCount = rooms.filter((r) => r.cleaning?.status === 'done').length
  const startedCount = rooms.filter((r) => r.cleaning?.status === 'started').length

  return (
    <main>
      {/* Header */}
      <header className="mb-4">
        <h1 className="flex items-center gap-2 text-xl font-bold">
          <Sparkles className="h-6 w-6 text-red-600" />
          งานแม่บ้าน
        </h1>
        {me && (
          <p className="mt-1 text-sm text-gray-500">
            สวัสดี {me.displayName || `รหัส ${me.badge}`}
          </p>
        )}
      </header>

      {/* Fail-closed / error notice */}
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
            onClick={load}
            disabled={loading}
            aria-label="รีเฟรช"
            className="rounded-lg border border-gray-300 p-2 text-gray-500 active:bg-gray-100 disabled:opacity-50"
          >
            <RefreshCw className={`h-4 w-4 ${loading ? 'animate-spin' : ''}`} />
          </button>
        </div>
      )}

      <HkOpsLinks />

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
    </main>
  )
}
