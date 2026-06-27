'use client'

import { useCallback, useEffect, useMemo, useRef, useState } from 'react'
import { useRouter } from 'next/navigation'
import { FileText, Search } from 'lucide-react'
import { useBranch, BRANCH_LABELS } from '@/contexts/BranchContext'
import { useBranchFetch } from '@/lib/use-branch-fetch'
import { formatStoredDateTime, formatStoredDayMonth, formatCurrency } from '@/lib/format'
import { V2PageHeader, V2Spinner, V2Empty } from '@/components/v2/primitives'

/**
 * v2 tax-invoice picker — task #44. Lists the folios you can issue a tax
 * invoice (ใบกำกับภาษี) for: everyone currently in-house plus today's
 * departures (a guest checking out today still needs their invoice). Each
 * row links to `/v2/invoice/[cinId]`, the view/print page. There is also a
 * direct check-in-ID box for the rare case the stay isn't in either list.
 *
 * Reuses the branch-aware `/api/rosters/{in-house,departures}` endpoints so
 * no new backend surface is required.
 */
interface StayRow {
  branch: string
  cinId: number
  cinNo: string
  customerName: string | null
  phone: string | null
  roomNo: string | null
  roomTypeName: string | null
  checkInTime: string | null
  expectedCheckout: string | null
  totalAmount: number | null
}

/** YYYY-MM-DD in Bangkok time. */
function bkkToday(): string {
  return new Date().toLocaleDateString('en-CA', { timeZone: 'Asia/Bangkok' })
}

export default function V2InvoicePicker() {
  const router = useRouter()
  const branchFetch = useBranchFetch()
  const { branch } = useBranch()
  const [rows, setRows] = useState<StayRow[]>([])
  const [loading, setLoading] = useState(true)
  const [query, setQuery] = useState('')
  const [directId, setDirectId] = useState('')
  const reqRef = useRef(0)

  const fetchStays = useCallback(async () => {
    const token = ++reqRef.current
    setLoading(true)
    try {
      const qs = `date=${encodeURIComponent(bkkToday())}`
      const [inRes, depRes] = await Promise.all([
        branchFetch(`/api/rosters/in-house?${qs}`),
        branchFetch(`/api/rosters/departures?${qs}`),
      ])
      if (token !== reqRef.current) return
      const inHouse: StayRow[] = inRes.ok ? ((await inRes.json()).data ?? []) : []
      const departures: StayRow[] = depRes.ok ? ((await depRes.json()).data ?? []) : []
      // Merge + dedupe by cinId (a same-day departure is also "in-house").
      const byId = new Map<number, StayRow>()
      for (const r of [...inHouse, ...departures]) byId.set(r.cinId, r)
      setRows([...byId.values()])
    } catch {
      if (token === reqRef.current) setRows([])
    } finally {
      if (token === reqRef.current) setLoading(false)
    }
  }, [branchFetch])

  useEffect(() => {
    fetchStays()
  }, [fetchStays])

  const filtered = useMemo(() => {
    const q = query.trim().toLowerCase()
    if (!q) return rows
    return rows.filter((r) =>
      [r.cinNo, r.customerName, r.roomNo, r.phone]
        .filter(Boolean)
        .some((v) => String(v).toLowerCase().includes(q)),
    )
  }, [rows, query])

  const branchLabel = branch === 'all' ? BRANCH_LABELS.all : BRANCH_LABELS[branch]
  const showBranchTag = branch === 'all'

  const goDirect = () => {
    const id = directId.trim()
    if (/^\d+$/.test(id)) router.push(`/v2/invoice/${id}`)
  }

  return (
    <div className="space-y-5">
      <V2PageHeader eyebrow="เอกสารภาษี · Tax invoice" title="ออกใบกำกับภาษี" />

      <div className="flex flex-wrap items-end gap-3">
        <label className="flex flex-col gap-1 flex-1 min-w-[200px]">
          <span className="text-[11px] font-medium" style={{ color: 'var(--v2-ink-3)' }}>
            ค้นหา · ห้อง / ชื่อ / เลขเช็คอิน
          </span>
          <div className="relative">
            <Search
              size={15}
              className="absolute left-3 top-1/2 -translate-y-1/2"
              style={{ color: 'var(--v2-ink-3)' }}
            />
            <input
              value={query}
              onChange={(e) => setQuery(e.target.value)}
              placeholder="พิมพ์เพื่อค้นหา…"
              className="h-9 w-full pl-9 pr-3 rounded-[10px] text-[14px] outline-none"
              style={{ background: 'var(--v2-surface)', border: '1px solid var(--v2-line-2)' }}
            />
          </div>
        </label>
        <label className="flex flex-col gap-1">
          <span className="text-[11px] font-medium" style={{ color: 'var(--v2-ink-3)' }}>
            เปิดด้วยรหัสเช็คอิน · Check-in ID
          </span>
          <div className="flex gap-2">
            <input
              value={directId}
              onChange={(e) => setDirectId(e.target.value)}
              onKeyDown={(e) => e.key === 'Enter' && goDirect()}
              inputMode="numeric"
              placeholder="เช่น 5228"
              className="v2-num h-9 w-28 px-3 rounded-[10px] text-[14px] outline-none"
              style={{ background: 'var(--v2-surface)', border: '1px solid var(--v2-line-2)' }}
            />
            <button
              onClick={goDirect}
              disabled={!/^\d+$/.test(directId.trim())}
              className="v2-btn v2-btn-ghost v2-btn-sm"
            >
              เปิด
            </button>
          </div>
        </label>
      </div>

      {loading ? (
        <V2Spinner label="กำลังโหลดรายการเข้าพัก…" />
      ) : filtered.length === 0 ? (
        <V2Empty
          title="ไม่พบรายการเข้าพัก"
          hint={`${branchLabel} · ลองค้นหาด้วยคำอื่น หรือเปิดด้วยรหัสเช็คอินโดยตรง`}
        />
      ) : (
        <div className="v2-card overflow-x-auto">
          <table className="w-full text-[13px] border-collapse">
            <thead>
              <tr style={{ borderBottom: '1px solid var(--v2-line)' }}>
                {['ห้อง', 'ชื่อแขก', 'เลขเช็คอิน', 'เข้า', 'ออก', 'ยอดรวม', ''].map((c, i) => (
                  <th
                    key={c || i}
                    className={`px-3 py-2.5 font-semibold whitespace-nowrap ${i === 5 ? 'text-right' : 'text-left'}`}
                    style={{ color: 'var(--v2-ink-2)' }}
                  >
                    {c}
                  </th>
                ))}
              </tr>
            </thead>
            <tbody>
              {filtered.map((r) => (
                <tr
                  key={`${r.branch}-${r.cinId}`}
                  className="cursor-pointer transition-colors hover:bg-[var(--v2-surface-2)]"
                  onClick={() => router.push(`/v2/invoice/${r.cinId}`)}
                >
                  <td className="px-3 py-2.5 font-semibold whitespace-nowrap">
                    {showBranchTag && (
                      <span
                        className="mr-1.5 text-[10px] px-1.5 py-0.5 rounded align-middle"
                        style={{ background: 'var(--v2-surface-2)', color: 'var(--v2-ink-3)' }}
                      >
                        {r.branch === 'hfville' ? 'Ville' : 'HF'}
                      </span>
                    )}
                    {r.roomNo || '—'}
                  </td>
                  <td className="px-3 py-2.5 font-semibold">{r.customerName || 'ไม่ระบุชื่อ'}</td>
                  <td className="px-3 py-2.5 v2-num" style={{ color: 'var(--v2-ink-3)' }}>
                    {r.cinNo}
                  </td>
                  <td className="px-3 py-2.5" style={{ color: 'var(--v2-ink-3)' }}>
                    {formatStoredDateTime(r.checkInTime)}
                  </td>
                  <td className="px-3 py-2.5" style={{ color: 'var(--v2-ink-3)' }}>
                    {formatStoredDayMonth(r.expectedCheckout)}
                  </td>
                  <td className="px-3 py-2.5 v2-num text-right whitespace-nowrap">
                    {r.totalAmount != null ? formatCurrency(r.totalAmount, 2) : '—'}
                  </td>
                  <td className="px-3 py-2.5 text-right whitespace-nowrap">
                    <span
                      className="inline-flex items-center gap-1.5 text-[12.5px] font-semibold"
                      style={{ color: 'var(--v2-wine-600)' }}
                    >
                      <FileText size={14} /> ใบกำกับภาษี
                    </span>
                  </td>
                </tr>
              ))}
            </tbody>
          </table>
        </div>
      )}
    </div>
  )
}
