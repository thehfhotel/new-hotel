'use client'

import { useCallback, useEffect, useState } from 'react'
import { Printer, Users } from 'lucide-react'
import { useBranchFetch } from '@/lib/use-branch-fetch'
import { useLiveRefresh } from '@/lib/v2/use-live-refresh'
import { useBranch } from '@/contexts/BranchContext'
import { formatCurrency } from '@/lib/format'
import { printRegion } from '@/lib/print'
import { V2PageHeader, V2Spinner, V2Empty } from '@/components/v2/primitives'
import { formatThai, RoundReportSheet } from '@/components/v2/RoundReport'

/**
 * Round summary / analytics — Track J7f. iHOTEL equivalent: the FrmDueBill
 * history list (closed rounds over a date range) + a period roll-up (which
 * iHOTEL lacks). Consumes GET /api/shifts/summary; per-round numbers + wording
 * mirror iHOTEL's View_RBill_H. A round row drills into the single-round report.
 */

interface SummaryRound {
  shiftId: number
  /** Site this round is from ("hfhotel" | "hfville") — set on every row so the
   *  "ทั้งหมด" (All) view can show + drill into the correct site. */
  branch: string
  shiftNo: number
  legacyRoundId: number | null
  openedAt: string
  closedAt: string | null
  openedBy: string
  closedBy: string | null
  opening: number
  cashReceived: number
  cashPaid: number
  cashNet: number
  credit: number
  transfer: number
  free: number
  web: number
  depositsReceived: number
  depositsReturned: number
  grandTotal: number
}

interface SummaryData {
  success: boolean
  from: string
  to: string
  rounds: SummaryRound[]
  totals: {
    roundCount: number
    cashReceived: number
    cashPaid: number
    cashNet: number
    credit: number
    transfer: number
    free: number
    web: number
    depositsReceived: number
    depositsReturned: number
    grandTotal: number
  }
  byCashier: { cashier: string; roundCount: number; grandTotal: number }[]
}

/** YYYY-MM-DD in Bangkok time, `daysAgo` days back from today. */
function bkkDate(daysAgo = 0): string {
  const now = new Date(Date.now() - daysAgo * 86400000)
  return now.toLocaleDateString('en-CA', { timeZone: 'Asia/Bangkok' }) // en-CA → YYYY-MM-DD
}

/** A Bangkok calendar day → RFC3339 UTC. `endExclusive` returns the start of
 *  the NEXT day so the backend's `< to` covers the whole `to` date. */
function toIso(date: string, endExclusive = false): string {
  const base = new Date(`${date}T00:00:00+07:00`)
  if (endExclusive) base.setTime(base.getTime() + 86400000)
  return base.toISOString()
}

function StatCard({ label, value, sub, accent }: { label: string; value: string; sub?: string; accent?: boolean }) {
  return (
    <div className="v2-card p-4">
      <div className="v2-eyebrow mb-1.5">{label}</div>
      <div
        className="v2-num text-[20px] font-bold leading-none"
        style={{ color: accent ? 'var(--v2-wine-600)' : 'var(--v2-ink)' }}
      >
        {value}
      </div>
      {sub && <div className="text-[11.5px] mt-1" style={{ color: 'var(--v2-ink-3)' }}>{sub}</div>}
    </div>
  )
}

// iHOTEL FrmDueBill history columns, in order. `num` right-aligns money.
const COLS: { key: keyof SummaryRound | 'dates' | 'income'; label: string; num?: boolean }[] = [
  { key: 'shiftNo', label: 'หมายเลข' },
  { key: 'dates', label: 'วันที่เปิด / ปิด' },
  { key: 'opening', label: 'เงินในลิ้นชัก', num: true },
  { key: 'cashReceived', label: 'เงินสด', num: true },
  { key: 'cashPaid', label: 'ยอดจ่าย', num: true },
  { key: 'depositsReceived', label: 'รับเงินมัดจำ', num: true },
  { key: 'depositsReturned', label: 'จ่ายเงินมัดจำ', num: true },
  { key: 'credit', label: 'บัตรเครดิต', num: true },
  { key: 'transfer', label: 'โอนเงิน', num: true },
  { key: 'income', label: 'รวมเงินรับ', num: true },
  { key: 'grandTotal', label: 'รวมทั้งหมด (รวมเงินทอน)', num: true },
  { key: 'closedBy', label: 'ผู้ปิดรอบ' },
]

export default function V2Rounds() {
  const branchFetch = useBranchFetch()
  const { branch } = useBranch()
  const [from, setFrom] = useState(() => bkkDate(30))
  const [to, setTo] = useState(() => bkkDate(0))
  const [data, setData] = useState<SummaryData | null>(null)
  const [loading, setLoading] = useState(true)
  // Drill-down carries the site too: shift_id is per-database, so an "All" view
  // mixes both sites' rounds and the report must be fetched from the right one.
  const [report, setReport] = useState<{ shiftId: number; branch: string } | null>(null)

  const fetchSummary = useCallback(async () => {
    setLoading(true)
    try {
      const qs = new URLSearchParams({ from: toIso(from), to: toIso(to, true) })
      const res = await branchFetch(`/api/shifts/summary?${qs.toString()}`)
      if (res.ok) setData(await res.json())
      else setData(null)
    } catch {
      setData(null)
    } finally {
      setLoading(false)
    }
  }, [branchFetch, from, to])

  useEffect(() => {
    fetchSummary()
  }, [fetchSummary])

  // Live-refresh when a payment/check-out happens in iHOTEL or the other app —
  // round income is computed from the payment ledger over the window.
  useLiveRefresh(
    branch,
    ['PaymentReceived', 'PaymentRefunded', 'CheckOutCompleted', 'CheckInCreated'],
    fetchSummary,
  )

  const t = data?.totals
  const depNet = t ? t.depositsReceived - t.depositsReturned : 0

  return (
    <div className="space-y-5">
      <V2PageHeader
        eyebrow="รายงาน"
        title="สรุปรอบบิล"
        right={
          t && t.roundCount > 0 ? (
            <button
              onClick={() => printRegion(document.getElementById('round-summary-print'))}
              className="v2-btn v2-btn-ghost v2-btn-sm v2-no-print"
              aria-label="พิมพ์รายงาน"
            >
              <Printer size={15} /> พิมพ์
            </button>
          ) : undefined
        }
      />

      {/* Date range */}
      <div className="flex flex-wrap items-end gap-3">
        <label className="flex flex-col gap-1">
          <span className="text-[11px] font-medium" style={{ color: 'var(--v2-ink-3)' }}>จากวันที่เปิด</span>
          <input
            type="date"
            value={from}
            max={to}
            onChange={(e) => setFrom(e.target.value)}
            className="v2-num h-9 px-3 rounded-[10px] text-[14px] outline-none"
            style={{ background: 'var(--v2-surface)', border: '1px solid var(--v2-line-2)' }}
          />
        </label>
        <label className="flex flex-col gap-1">
          <span className="text-[11px] font-medium" style={{ color: 'var(--v2-ink-3)' }}>ถึงวันที่</span>
          <input
            type="date"
            value={to}
            min={from}
            max={bkkDate(0)}
            onChange={(e) => setTo(e.target.value)}
            className="v2-num h-9 px-3 rounded-[10px] text-[14px] outline-none"
            style={{ background: 'var(--v2-surface)', border: '1px solid var(--v2-line-2)' }}
          />
        </label>
      </div>

      {loading ? (
        <V2Spinner label="กำลังโหลดสรุปรอบบิล…" />
      ) : !t || t.roundCount === 0 ? (
        <V2Empty title="ไม่มีรอบบิลที่ปิดแล้วในช่วงนี้" hint="ลองขยายช่วงวันที่" />
      ) : (
        <div id="round-summary-print" className="v2-print space-y-5">
          {/* Print-only header (the on-screen page header + date pickers
              don't print; this gives the printout title + range context). */}
          <div className="v2-print-only" style={{ marginBottom: '10px' }}>
            <div style={{ fontSize: '15px', fontWeight: 700 }}>สรุปรอบบิล</div>
            <div style={{ fontSize: '11px', color: '#555' }}>
              {formatThai(data!.from)} – {formatThai(data!.to)}
            </div>
          </div>
          {/* Analytics roll-up */}
          <div className="grid gap-3 grid-cols-2 lg:grid-cols-3">
            <StatCard label="รวมเงินรับ (เทียบ iHOTEL)" value={formatCurrency(t.cashNet + t.credit + t.transfer, 0)} sub="เงินสด + บัตร + โอน · ไม่รวมเงินทอนตั้งต้น" accent />
            <StatCard label="รวมทั้งหมด (รวมเงินทอน)" value={formatCurrency(t.grandTotal, 0)} sub={`รวมเงินทอนตั้งต้น · ${t.roundCount} รอบ`} />
            <StatCard label="รวมเงินสดคงเหลือ" value={formatCurrency(t.cashNet, 0)} sub={`รับ ${formatCurrency(t.cashReceived, 0)} · จ่าย ${formatCurrency(t.cashPaid, 0)}`} />
            <StatCard label="บัตรเครดิต" value={formatCurrency(t.credit, 0)} />
            <StatCard label="โอนเงิน" value={formatCurrency(t.transfer, 0)} />
            <StatCard label="เงินมัดจำ (สุทธิ)" value={formatCurrency(depNet, 0)} sub={`รับ ${formatCurrency(t.depositsReceived, 0)} · คืน ${formatCurrency(t.depositsReturned, 0)}`} />
            <StatCard label="จำนวนรอบบิล" value={String(t.roundCount)} sub={`${formatThai(data!.from)} – ${formatThai(data!.to)}`} />
          </div>

          {/* Per-round table (iHOTEL FrmDueBill history) */}
          <div>
            <div className="v2-eyebrow mb-2">รายการรอบบิล</div>
            <div className="v2-card overflow-x-auto">
              <table className="w-full text-[13px] border-collapse">
                <thead>
                  <tr style={{ borderBottom: '1px solid var(--v2-line)' }}>
                    {COLS.map((c) => (
                      <th
                        key={String(c.key)}
                        className={`px-3 py-2.5 font-semibold whitespace-nowrap ${c.num ? 'text-right' : 'text-left'}`}
                        style={{ color: 'var(--v2-ink-2)' }}
                      >
                        {c.label}
                      </th>
                    ))}
                  </tr>
                </thead>
                <tbody>
                  {data!.rounds.map((r) => (
                    <tr
                      key={`${r.branch}-${r.shiftId}`}
                      onClick={() => setReport({ shiftId: r.shiftId, branch: r.branch })}
                      className="cursor-pointer transition-colors hover:bg-[var(--v2-surface-2)]"
                      style={{ borderBottom: '1px solid var(--v2-line)' }}
                    >
                      <td className="px-3 py-2.5 v2-num whitespace-nowrap">
                        {branch === 'all' && (
                          <span
                            className="mr-1.5 text-[10px] px-1.5 py-0.5 rounded align-middle"
                            style={{ background: 'var(--v2-surface-2)', color: 'var(--v2-ink-3)' }}
                          >
                            {r.branch === 'hfville' ? 'Ville' : 'HF'}
                          </span>
                        )}
                        #{r.shiftNo}
                      </td>
                      <td className="px-3 py-2.5 v2-num whitespace-nowrap" style={{ color: 'var(--v2-ink-3)' }}>
                        {formatThai(r.openedAt)}
                        {r.closedAt ? ` → ${formatThai(r.closedAt)}` : ''}
                      </td>
                      <td className="px-3 py-2.5 v2-num text-right">{formatCurrency(r.opening, 2)}</td>
                      <td className="px-3 py-2.5 v2-num text-right">{formatCurrency(r.cashReceived, 2)}</td>
                      <td className="px-3 py-2.5 v2-num text-right" style={{ color: r.cashPaid ? 'var(--v2-ink-2)' : undefined }}>
                        {r.cashPaid ? `−${formatCurrency(r.cashPaid, 2)}` : formatCurrency(0, 2)}
                      </td>
                      <td className="px-3 py-2.5 v2-num text-right">{formatCurrency(r.depositsReceived, 2)}</td>
                      <td className="px-3 py-2.5 v2-num text-right" style={{ color: r.depositsReturned ? 'var(--v2-ink-2)' : undefined }}>
                        {r.depositsReturned ? `−${formatCurrency(r.depositsReturned, 2)}` : formatCurrency(0, 2)}
                      </td>
                      <td className="px-3 py-2.5 v2-num text-right">{formatCurrency(r.credit, 2)}</td>
                      <td className="px-3 py-2.5 v2-num text-right">{formatCurrency(r.transfer, 2)}</td>
                      <td className="px-3 py-2.5 v2-num text-right font-semibold" style={{ color: 'var(--v2-wine-600)' }}>
                        {formatCurrency(r.cashNet + r.credit + r.transfer, 2)}
                      </td>
                      <td className="px-3 py-2.5 v2-num text-right" style={{ color: 'var(--v2-ink-3)' }}>{formatCurrency(r.grandTotal, 2)}</td>
                      <td className="px-3 py-2.5 whitespace-nowrap" style={{ color: 'var(--v2-ink-2)' }}>{r.closedBy || '—'}</td>
                    </tr>
                  ))}
                </tbody>
              </table>
            </div>
            <p className="text-[11.5px] mt-1.5" style={{ color: 'var(--v2-ink-3)' }}>แตะแถวเพื่อดูรายงานรอบบิลฉบับเต็ม</p>
          </div>

          {/* By cashier */}
          {data!.byCashier.length > 0 && (
            <div>
              <div className="v2-eyebrow mb-2 flex items-center gap-1.5"><Users size={13} /> แยกตามผู้ปิดรอบ</div>
              <div className="v2-card overflow-hidden divide-y" style={{ borderColor: 'var(--v2-line)' }}>
                {data!.byCashier.map((c) => (
                  <div key={c.cashier} className="flex items-baseline gap-3 px-4 py-2.5">
                    <span className="text-[13.5px]" style={{ color: 'var(--v2-ink-2)' }}>{c.cashier}</span>
                    <span className="text-[11.5px]" style={{ color: 'var(--v2-ink-3)' }}>{c.roundCount} รอบ</span>
                    <span className="flex-1" />
                    <span className="v2-num text-[14px] font-semibold">{formatCurrency(c.grandTotal, 2)}</span>
                  </div>
                ))}
              </div>
            </div>
          )}
        </div>
      )}

      {report != null && (
        <RoundReportSheet shiftId={report.shiftId} branch={report.branch} onClose={() => setReport(null)} />
      )}
    </div>
  )
}
