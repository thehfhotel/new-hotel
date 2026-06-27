'use client'

import { useCallback, useEffect, useRef, useState } from 'react'
import Link from 'next/link'
import { Bell, X, CalendarClock, Wallet, Loader2 } from 'lucide-react'
import { useBranchFetch } from '@/lib/use-branch-fetch'
import { formatCurrency, formatStoredDayMonth } from '@/lib/format'

/**
 * In-shell notification bell (task #53) — surfaces upcoming arrivals and
 * balance-due bookings from GET /api/calendar/reminders. Branch-aware (the feed
 * follows the selected branch; dismiss targets each item's own site so the
 * `all` view dismisses against the correct pool). Dismiss is PG-canonical only.
 */

interface Reminder {
  bookId: number
  bookNo: string
  legacyBookId?: string
  customerName: string | null
  checkIn: string | null
  checkOut: string | null
  status: string
  totalAmount: number
  depositAmount: number
  balanceDue: number
  daysUntilCheckin: number
  isUpcoming: boolean
  isBalanceDue: boolean
  notifyNote?: string
  branch: string
}

interface ReminderResponse {
  success: boolean
  reminders: Reminder[]
  upcomingCount: number
  balanceDueCount: number
}

const POLL_MS = 60_000

function arrivalLabel(days: number): string {
  if (days <= 0) return 'มาถึงวันนี้'
  if (days === 1) return 'มาถึงพรุ่งนี้'
  return `มาถึงใน ${days} วัน`
}

export default function ReminderBell({ align = 'right' }: { align?: 'left' | 'right' }) {
  // branchFetch already appends the selected branch and changes identity when
  // the branch flips, so `load` (and its effect) re-runs on branch change.
  const branchFetch = useBranchFetch()
  const [data, setData] = useState<ReminderResponse | null>(null)
  const [open, setOpen] = useState(false)
  const [dismissing, setDismissing] = useState<string | null>(null)
  const wrapRef = useRef<HTMLDivElement>(null)

  const load = useCallback(async () => {
    try {
      const res = await branchFetch('/api/calendar/reminders')
      const json = res.ok ? await res.json() : null
      // Only accept a well-formed payload so the render path can assume an array.
      setData(json && Array.isArray(json.reminders) ? (json as ReminderResponse) : null)
    } catch {
      setData(null)
    }
  }, [branchFetch])

  // Initial + branch-change fetch, plus a slow background poll.
  useEffect(() => {
    load()
    const t = setInterval(load, POLL_MS)
    return () => clearInterval(t)
  }, [load])

  // Close on outside click / Escape.
  useEffect(() => {
    if (!open) return
    const onDown = (e: MouseEvent) => {
      if (wrapRef.current && !wrapRef.current.contains(e.target as Node)) setOpen(false)
    }
    const onKey = (e: KeyboardEvent) => {
      if (e.key === 'Escape') setOpen(false)
    }
    window.addEventListener('mousedown', onDown)
    window.addEventListener('keydown', onKey)
    return () => {
      window.removeEventListener('mousedown', onDown)
      window.removeEventListener('keydown', onKey)
    }
  }, [open])

  const dismiss = async (r: Reminder) => {
    const key = `${r.branch}-${r.bookId}`
    setDismissing(key)
    try {
      // Target the item's OWN site (not the globally-selected branch) so the
      // 'all' view dismisses against the correct pool.
      const res = await fetch(
        `/api/calendar/reminders/${r.bookId}/dismiss?branch=${encodeURIComponent(r.branch)}`,
        { method: 'POST' },
      )
      if (res.ok) {
        setData((prev) =>
          prev
            ? {
                ...prev,
                reminders: prev.reminders.filter(
                  (x) => !(x.bookId === r.bookId && x.branch === r.branch),
                ),
              }
            : prev,
        )
      }
    } catch {
      /* ignore — next poll reconciles */
    } finally {
      setDismissing(null)
    }
  }

  const count = data?.reminders?.length ?? 0

  return (
    <div ref={wrapRef} className="relative">
      <button
        onClick={() => setOpen((v) => !v)}
        className="relative p-2 rounded-full transition-colors"
        style={{ background: 'var(--v2-surface-2)', border: '1px solid var(--v2-line)', color: 'var(--v2-ink-2)' }}
        aria-label="การแจ้งเตือน"
        aria-expanded={open}
      >
        <Bell size={18} />
        {count > 0 && (
          <span
            className="v2-num absolute -top-1 -right-1 min-w-[17px] h-[17px] px-1 rounded-full text-[10px] font-bold grid place-items-center"
            style={{ background: 'var(--v2-wine-600)', color: '#fff' }}
          >
            {count > 99 ? '99+' : count}
          </span>
        )}
      </button>

      {open && (
        <div
          className={`absolute ${align === 'left' ? 'left-0' : 'right-0'} mt-2 w-[330px] max-w-[88vw] z-50 rounded-[14px] overflow-hidden`}
          style={{ background: 'var(--v2-surface)', border: '1px solid var(--v2-line)', boxShadow: 'var(--v2-shadow-lg)' }}
        >
          <div
            className="flex items-center justify-between px-4 py-3"
            style={{ borderBottom: '1px solid var(--v2-line)' }}
          >
            <span className="text-[14px] font-semibold">การแจ้งเตือน</span>
            {data && (data.upcomingCount > 0 || data.balanceDueCount > 0) && (
              <span className="text-[11.5px]" style={{ color: 'var(--v2-ink-3)' }}>
                มาถึง {data.upcomingCount} · ค้างชำระ {data.balanceDueCount}
              </span>
            )}
          </div>

          <div className="max-h-[60vh] overflow-y-auto">
            {count === 0 ? (
              <div className="px-4 py-8 text-center text-[13px]" style={{ color: 'var(--v2-ink-3)' }}>
                ไม่มีการแจ้งเตือน
              </div>
            ) : (
              data!.reminders.map((r) => {
                const key = `${r.branch}-${r.bookId}`
                return (
                  <div
                    key={key}
                    className="flex items-start gap-2.5 px-4 py-3"
                    style={{ borderBottom: '1px solid var(--v2-line)' }}
                  >
                    <div className="min-w-0 flex-1">
                      <div className="flex items-center gap-2">
                        <span className="text-[13.5px] font-semibold truncate" style={{ color: 'var(--v2-ink)' }}>
                          {r.customerName || 'ไม่ระบุชื่อ'}
                        </span>
                        <span className="v2-num text-[11px]" style={{ color: 'var(--v2-ink-3)' }}>
                          {r.bookNo}
                        </span>
                      </div>

                      <div className="flex flex-wrap items-center gap-x-3 gap-y-1 mt-1">
                        {r.isUpcoming && (
                          <span className="inline-flex items-center gap-1 text-[11.5px]" style={{ color: 'var(--v2-arr)' }}>
                            <CalendarClock size={12} />
                            {arrivalLabel(r.daysUntilCheckin)}
                            {r.checkIn && <span style={{ color: 'var(--v2-ink-3)' }}>· {formatStoredDayMonth(r.checkIn)}</span>}
                          </span>
                        )}
                        {r.isBalanceDue && (
                          <span className="inline-flex items-center gap-1 text-[11.5px]" style={{ color: 'var(--v2-occ)' }}>
                            <Wallet size={12} />
                            ค้างชำระ {formatCurrency(r.balanceDue)}
                          </span>
                        )}
                      </div>

                      {r.notifyNote && (
                        <p className="text-[11.5px] mt-1 whitespace-pre-wrap" style={{ color: 'var(--v2-ink-2)' }}>
                          {r.notifyNote}
                        </p>
                      )}
                    </div>

                    <button
                      onClick={() => dismiss(r)}
                      disabled={dismissing === key}
                      className="shrink-0 p-1.5 rounded-[8px] transition-colors"
                      style={{ color: 'var(--v2-ink-3)' }}
                      title="ปิดการแจ้งเตือนนี้"
                      aria-label="ปิดการแจ้งเตือน"
                    >
                      {dismissing === key ? <Loader2 size={14} className="animate-spin" /> : <X size={14} />}
                    </button>
                  </div>
                )
              })
            )}
          </div>

          <div className="flex items-center justify-between px-4 py-2.5" style={{ borderTop: '1px solid var(--v2-line)' }}>
            <Link
              href="/v2/reservations?balanceDue=1"
              onClick={() => setOpen(false)}
              className="text-[12px] font-medium"
              style={{ color: 'var(--v2-wine-600)' }}
            >
              ดูรายการค้างชำระ
            </Link>
            <Link
              href="/v2/calendar"
              onClick={() => setOpen(false)}
              className="text-[12px]"
              style={{ color: 'var(--v2-ink-3)' }}
            >
              ปฏิทินห้องว่าง
            </Link>
          </div>
        </div>
      )}
    </div>
  )
}
