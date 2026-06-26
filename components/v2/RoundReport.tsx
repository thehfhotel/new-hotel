'use client'

import { useEffect, useState } from 'react'
import { X } from 'lucide-react'
import { useBranchFetch } from '@/lib/use-branch-fetch'
import { formatCurrency } from '@/lib/format'
import { V2Spinner, V2Empty } from './primitives'

/**
 * Round (cashier-shift) reconciliation report — Track J7d. Renders the
 * payload from GET /api/shifts/{id}/report: income split by payment tender,
 * sales by category, and the cash-drawer reconciliation (expected vs counted
 * vs variance). The NUMBERS mirror iHOTEL's ReportShipCash / ReportIncome2
 * (both come from the same canonical ht_payment_ledger). Used at round close
 * and via the read-only "view report" action.
 */

export interface RoundReportData {
  success: boolean
  shift: {
    shiftId: number
    shiftNo: number
    openingFloat: number
    openedBy: string
    openedAt: string
    closedAt: string | null
    closedBy: string | null
    legacyRoundId: number | null
  }
  windowFrom: string
  windowTo: string
  open: boolean
  income: {
    cashReceived: number
    cashPaid: number
    cashNet: number
    credit: number
    transfer: number
    free: number
    web: number
    lineCount: number
    paymentCount: number
  }
  deposits: { received: number; returned: number }
  grandTotal: number
  sales: { category: string; amount: number; lines: number }[]
  expectedCash: number
  countedCash: number | null
  cashVariance: number | null
}

/** ht_shifts timestamps are real UTC instants (TIMESTAMPTZ) — Bangkok wall-clock. */
export function formatThai(iso: string): string {
  try {
    return new Date(iso).toLocaleString('th-TH', {
      day: 'numeric',
      month: 'short',
      hour: '2-digit',
      minute: '2-digit',
      timeZone: 'Asia/Bangkok',
    })
  } catch {
    return iso
  }
}

const CATEGORY_LABEL: Record<string, string> = { room: 'ค่าห้อง', product: 'สินค้า / บริการ' }

function Row({
  label,
  value,
  sub,
  strong,
  negative,
}: {
  label: string
  value: number
  sub?: string
  strong?: boolean
  /** Render as an outflow: muted "−" prefix (cash paid out, deposit returned). */
  negative?: boolean
}) {
  return (
    <div className="flex items-baseline gap-3 px-4 py-2.5">
      <span className={`text-[13.5px] ${strong ? 'font-semibold' : ''}`} style={{ color: strong ? 'var(--v2-ink)' : 'var(--v2-ink-2)' }}>
        {label}
      </span>
      {sub && <span className="text-[11.5px]" style={{ color: 'var(--v2-ink-3)' }}>{sub}</span>}
      <span className="flex-1" />
      <span
        className={`v2-num text-[14px] ${strong ? 'font-bold' : ''}`}
        style={{ color: negative && value !== 0 ? 'var(--v2-ink-2)' : 'var(--v2-ink)' }}
      >
        {negative && value !== 0 ? '−' : ''}{formatCurrency(value, 2)}
      </span>
    </div>
  )
}

export default function RoundReport({ data }: { data: RoundReportData }) {
  const { shift, income, deposits, grandTotal, sales, expectedCash, countedCash, cashVariance } = data
  // Variance tone: balanced → ok (green); short (negative) → occ (wine);
  // over (positive) → arr (amber).
  const tone = cashVariance == null ? null : cashVariance === 0 ? 'ok' : cashVariance < 0 ? 'occ' : 'arr'
  const hasDeposits = deposits.received !== 0 || deposits.returned !== 0

  return (
    <div className="space-y-4">
      {/* Header / window */}
      <div className="v2-inset px-4 py-3 flex items-center gap-3">
        <div className="flex-1 min-w-0">
          <div className="v2-eyebrow mb-0.5">รายงานรอบบิล</div>
          <div className="text-[15px] font-semibold">รอบ #{shift.shiftNo}</div>
          <div className="text-[12px] v2-num mt-0.5 truncate" style={{ color: 'var(--v2-ink-3)' }}>
            {formatThai(data.windowFrom)} → {data.open ? 'ปัจจุบัน' : formatThai(data.windowTo)} · {shift.openedBy}
          </div>
        </div>
        <span className={`v2-pill ${data.open ? 's-ok' : 's-mut'}`}>
          <span className={`v2-dot ${data.open ? 'd-ok' : 'd-mut'}`} />
          {data.open ? 'เปิดอยู่' : 'ปิดแล้ว'}
        </span>
      </div>

      {/* Income by tender — mirrors iHOTEL View_RBill_H: cash received vs
          paid-out, credit, transfer (+ our free/web extras), grand total. */}
      <div>
        <div className="v2-eyebrow mb-2">รายได้แยกตามประเภทการชำระ</div>
        <div className="v2-card overflow-hidden divide-y" style={{ borderColor: 'var(--v2-line)' }}>
          <Row label="เงินสด" value={income.cashReceived} />
          <Row label="ยอดจ่าย" value={income.cashPaid} negative />
          <Row label="โอนเงิน" value={income.transfer} />
          <Row label="บัตรเครดิต" value={income.credit} />
          {income.web !== 0 && <Row label="ออนไลน์" value={income.web} />}
          {income.free !== 0 && <Row label="ส่วนลด / ฟรี" value={income.free} />}
          <Row label="รวมทั้งหมด" value={grandTotal} strong />
        </div>
        <div className="text-[11.5px] mt-1.5 v2-num" style={{ color: 'var(--v2-ink-3)' }}>
          {income.paymentCount} ใบเสร็จ · {income.lineCount} รายการ
        </div>
      </div>

      {/* Deposits — iHOTEL Dep_Rec / Dep_pay */}
      {hasDeposits && (
        <div>
          <div className="v2-eyebrow mb-2">เงินมัดจำ</div>
          <div className="v2-card overflow-hidden divide-y" style={{ borderColor: 'var(--v2-line)' }}>
            <Row label="รับเงินมัดจำ" value={deposits.received} />
            <Row label="จ่ายเงินมัดจำ" value={deposits.returned} negative />
          </div>
        </div>
      )}

      {/* Sales by category */}
      {sales.length > 0 && (
        <div>
          <div className="v2-eyebrow mb-2">ยอดขายแยกหมวด</div>
          <div className="v2-card overflow-hidden divide-y" style={{ borderColor: 'var(--v2-line)' }}>
            {sales.map((s) => (
              <Row
                key={s.category}
                label={CATEGORY_LABEL[s.category] || s.category}
                value={s.amount}
                sub={`${s.lines} รายการ`}
              />
            ))}
          </div>
        </div>
      )}

      {/* Cash-drawer reconciliation */}
      <div>
        <div className="v2-eyebrow mb-2">กระทบยอดเงินสด</div>
        <div className="v2-card overflow-hidden divide-y" style={{ borderColor: 'var(--v2-line)' }}>
          <Row label="เงินในลิ้นชัก (ตั้งต้น)" value={shift.openingFloat} />
          <Row label="รวมเงินสดคงเหลือ" value={income.cashNet} />
          <Row label="จำนวนเงินในลิ้นชัก" value={expectedCash} strong />
          {countedCash != null && <Row label="นับได้จริง" value={countedCash} />}
          {cashVariance != null && tone && (
            <div className="flex items-baseline gap-3 px-4 py-2.5" style={{ background: `var(--v2-${tone}-bg)` }}>
              <span className="text-[13.5px] font-semibold" style={{ color: `var(--v2-${tone})` }}>
                {cashVariance === 0 ? 'ตรงพอดี' : cashVariance < 0 ? 'ขาด' : 'เกิน'}
              </span>
              <span className="flex-1" />
              <span className="v2-num text-[14px] font-bold" style={{ color: `var(--v2-${tone})` }}>
                {cashVariance > 0 ? '+' : ''}{formatCurrency(cashVariance, 2)}
              </span>
            </div>
          )}
        </div>
        {countedCash == null && (
          <p className="text-[11.5px] mt-1.5" style={{ color: 'var(--v2-ink-3)' }}>
            ยังไม่ได้นับเงินในลิ้นชักสำหรับรอบนี้
          </p>
        )}
      </div>
    </div>
  )
}

/**
 * Read-only modal that fetches GET /api/shifts/{shiftId}/report and renders
 * [`RoundReport`] in a v2 sheet. Used by the "view round report" action — no
 * write capability required (purely informational).
 */
export function RoundReportSheet({ shiftId, onClose }: { shiftId: number; onClose: () => void }) {
  const branchFetch = useBranchFetch()
  const [data, setData] = useState<RoundReportData | null>(null)
  const [loaded, setLoaded] = useState(false)

  useEffect(() => {
    let alive = true
    branchFetch(`/api/shifts/${shiftId}/report`)
      .then(async (r) => {
        if (alive && r.ok) setData(await r.json())
      })
      .catch(() => {})
      .finally(() => {
        if (alive) setLoaded(true)
      })
    return () => {
      alive = false
    }
  }, [branchFetch, shiftId])

  return (
    <div className="v2-sheet-backdrop" onClick={onClose} role="dialog" aria-modal="true">
      <div
        className="mt-auto lg:mt-0 lg:ml-auto w-full lg:w-[440px] lg:h-full v2-sheet-up lg:[animation:v2-panel-in_.24s_cubic-bezier(.2,.8,.2,1)]"
        style={{ background: 'var(--v2-surface)' }}
        onClick={(e) => e.stopPropagation()}
      >
        <div className="flex items-center gap-3 px-5 py-4" style={{ borderBottom: '1px solid var(--v2-line)' }}>
          <div className="v2-eyebrow flex-1">รายงานรอบบิล</div>
          <button onClick={onClose} className="p-2 rounded-full" style={{ color: 'var(--v2-ink-3)' }} aria-label="ปิด">
            <X size={20} />
          </button>
        </div>
        <div className="px-5 py-4 overflow-y-auto" style={{ maxHeight: 'calc(100vh - 80px)' }}>
          {!loaded ? (
            <V2Spinner label="กำลังโหลดรายงาน…" />
          ) : data ? (
            <RoundReport data={data} />
          ) : (
            <V2Empty title="ไม่พบรายงานรอบนี้" hint="รอบอาจยังไม่มีข้อมูลการชำระเงิน" />
          )}
        </div>
      </div>
    </div>
  )
}
