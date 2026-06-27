'use client'

import { useCallback, useEffect, useMemo, useState } from 'react'
import { Printer, Plus, TrendingUp, TrendingDown, Wallet, X, Loader2 } from 'lucide-react'
import { useBranchFetch } from '@/lib/use-branch-fetch'
import { useBranch } from '@/contexts/BranchContext'
import { useAuth } from '@/contexts/AuthContext'
import { formatCurrency } from '@/lib/format'
import { printRegion } from '@/lib/print'
import { V2PageHeader, V2Spinner, V2Empty, VilleNotice } from '@/components/v2/primitives'
import { formatThai } from '@/components/v2/RoundReport'

/**
 * Cash in/out petty-cash ledger (รายรับ-รายจ่าย) — migration 059. iHOTEL
 * equivalent: FrmPayMain / FrmAddPay (TB_Pay_History). Consumes GET /api/cash
 * for the dated list + income/expense/balance totals and POST
 * /api/cash/{income,expense} to add an entry. Categories (the account tree,
 * mirror of TB_SET_MyType2/_2_2/3) populate the entry-form dropdowns. All calls
 * are branch-aware via useBranchFetch (?branch=).
 */

type CashKind = 'income' | 'expense' | 'unknown'

interface CashEntry {
  cashId: number
  legacyId: number | null
  kind: CashKind
  legacyType: string | null
  entryDate: string | null
  billNo: string | null
  payee: string | null
  amount: number
  note: string | null
  programDate: string | null
  group: string | null
  account: string | null
  source: string
  createdBy: string | null
}

interface CashListData {
  success: boolean
  from: string
  to: string
  entries: CashEntry[]
  totals: { income: number; expense: number; balance: number; count: number }
}

interface CashCategory {
  catId: number
  level: string
  legacyId: number
  idFull: string | null
  name: string | null
}

/** YYYY-MM-DD in Bangkok time, `daysAgo` days back from today. */
function bkkDate(daysAgo = 0): string {
  const now = new Date(Date.now() - daysAgo * 86400000)
  return now.toLocaleDateString('en-CA', { timeZone: 'Asia/Bangkok' })
}

/** A Bangkok calendar day → RFC3339 UTC. `endExclusive` returns the start of the
 *  NEXT day so the backend's `< to` covers the whole `to` date. */
function toIso(date: string, endExclusive = false): string {
  const base = new Date(`${date}T00:00:00+07:00`)
  if (endExclusive) base.setTime(base.getTime() + 86400000)
  return base.toISOString()
}

function StatCard({
  label,
  value,
  icon: Icon,
  tone,
}: {
  label: string
  value: string
  icon: typeof Wallet
  tone?: 'income' | 'expense' | 'balance'
}) {
  const color =
    tone === 'income'
      ? 'var(--v2-wine-600)'
      : tone === 'expense'
        ? '#b91c1c'
        : 'var(--v2-ink)'
  return (
    <div className="v2-card p-4">
      <div className="v2-eyebrow mb-1.5 flex items-center gap-1.5">
        <Icon size={13} /> {label}
      </div>
      <div className="v2-num text-[20px] font-bold leading-none" style={{ color }}>
        {value}
      </div>
    </div>
  )
}

const EMPTY_FORM = {
  kind: 'income' as 'income' | 'expense',
  date: bkkDate(0),
  amount: '',
  payee: '',
  billNo: '',
  note: '',
  group: '',
  account: '',
}

export default function V2Cash() {
  const branchFetch = useBranchFetch()
  const { branch } = useBranch()
  const { user } = useAuth()
  const [from, setFrom] = useState(() => bkkDate(30))
  const [to, setTo] = useState(() => bkkDate(0))
  const [data, setData] = useState<CashListData | null>(null)
  const [loading, setLoading] = useState(true)
  const [categories, setCategories] = useState<CashCategory[]>([])
  const [showForm, setShowForm] = useState(false)
  const [form, setForm] = useState(EMPTY_FORM)
  const [submitting, setSubmitting] = useState(false)
  const [formError, setFormError] = useState<string | null>(null)

  const fetchList = useCallback(async () => {
    setLoading(true)
    try {
      const qs = new URLSearchParams({ from: toIso(from), to: toIso(to, true) })
      const res = await branchFetch(`/api/cash?${qs.toString()}`)
      setData(res.ok ? await res.json() : null)
    } catch {
      setData(null)
    } finally {
      setLoading(false)
    }
  }, [branchFetch, from, to])

  const fetchCategories = useCallback(async () => {
    try {
      const res = await branchFetch('/api/cash/categories')
      if (res.ok) {
        const body = await res.json()
        setCategories(body.categories ?? [])
      } else {
        setCategories([])
      }
    } catch {
      setCategories([])
    }
  }, [branchFetch])

  useEffect(() => {
    fetchList()
  }, [fetchList])

  useEffect(() => {
    fetchCategories()
  }, [fetchCategories])

  // group = the higher tree levels (2 / 2_2); account = the leaf level (3).
  const groupOptions = useMemo(
    () => categories.filter((c) => c.level === '2' || c.level === '2_2'),
    [categories]
  )
  const accountOptions = useMemo(() => categories.filter((c) => c.level === '3'), [categories])

  const submit = async (e: React.FormEvent) => {
    e.preventDefault()
    setFormError(null)
    const amount = Number(form.amount)
    if (!Number.isFinite(amount) || amount <= 0) {
      setFormError('กรุณาระบุจำนวนเงินที่มากกว่า 0')
      return
    }
    setSubmitting(true)
    try {
      const res = await branchFetch(`/api/cash/${form.kind}`, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({
          entryDate: toIso(form.date),
          amount,
          payee: form.payee || null,
          billNo: form.billNo || null,
          note: form.note || null,
          group: form.group || null,
          account: form.account || null,
          createdBy: user?.username ?? null,
        }),
      })
      if (!res.ok) {
        const body = await res.json().catch(() => null)
        setFormError(body?.error || body?.message || 'บันทึกไม่สำเร็จ')
        return
      }
      setForm({ ...EMPTY_FORM, date: form.date, kind: form.kind })
      setShowForm(false)
      await fetchList()
    } catch {
      setFormError('เกิดข้อผิดพลาดในการเชื่อมต่อ')
    } finally {
      setSubmitting(false)
    }
  }

  const t = data?.totals

  return (
    <div className="space-y-5">
      <V2PageHeader
        eyebrow="การเงิน"
        title="รายรับ-รายจ่าย"
        right={
          <div className="flex items-center gap-2 v2-no-print">
            {t && t.count > 0 && (
              <button
                onClick={() => printRegion(document.getElementById('cash-print'))}
                className="v2-btn v2-btn-ghost v2-btn-sm"
                aria-label="พิมพ์รายงาน"
              >
                <Printer size={15} /> พิมพ์
              </button>
            )}
            <button
              onClick={() => {
                setShowForm((v) => !v)
                setFormError(null)
              }}
              className="v2-btn v2-btn-soft v2-btn-sm"
            >
              {showForm ? <X size={15} /> : <Plus size={15} />}
              {showForm ? 'ปิดฟอร์ม' : 'เพิ่มรายการ'}
            </button>
          </div>
        }
      />

      <VilleNotice branch={branch} />

      {/* Add-entry form */}
      {showForm && (
        <form onSubmit={submit} className="v2-card p-4 space-y-3">
          <div className="flex gap-2">
            {(['income', 'expense'] as const).map((k) => (
              <button
                key={k}
                type="button"
                onClick={() => setForm((f) => ({ ...f, kind: k }))}
                className="v2-btn v2-btn-sm"
                data-active={form.kind === k}
                style={{
                  background: form.kind === k ? 'var(--v2-wine-50)' : 'var(--v2-surface-2)',
                  color: form.kind === k ? 'var(--v2-wine-600)' : 'var(--v2-ink-2)',
                  fontWeight: form.kind === k ? 600 : 400,
                }}
              >
                {k === 'income' ? 'รายรับ' : 'รายจ่าย'}
              </button>
            ))}
          </div>

          <div className="grid gap-3 sm:grid-cols-2 lg:grid-cols-3">
            <label className="flex flex-col gap-1">
              <span className="text-[11px] font-medium" style={{ color: 'var(--v2-ink-3)' }}>วันที่</span>
              <input
                type="date"
                value={form.date}
                max={bkkDate(0)}
                onChange={(e) => setForm((f) => ({ ...f, date: e.target.value }))}
                className="v2-num h-9 px-3 rounded-[10px] text-[14px] outline-none"
                style={{ background: 'var(--v2-surface)', border: '1px solid var(--v2-line-2)' }}
              />
            </label>
            <label className="flex flex-col gap-1">
              <span className="text-[11px] font-medium" style={{ color: 'var(--v2-ink-3)' }}>จำนวนเงิน (บาท)</span>
              <input
                type="number"
                min="0"
                step="0.01"
                value={form.amount}
                onChange={(e) => setForm((f) => ({ ...f, amount: e.target.value }))}
                placeholder="0.00"
                className="v2-num h-9 px-3 rounded-[10px] text-[14px] outline-none"
                style={{ background: 'var(--v2-surface)', border: '1px solid var(--v2-line-2)' }}
              />
            </label>
            <label className="flex flex-col gap-1">
              <span className="text-[11px] font-medium" style={{ color: 'var(--v2-ink-3)' }}>ชื่อผู้รับ/ผู้จ่าย</span>
              <input
                type="text"
                value={form.payee}
                onChange={(e) => setForm((f) => ({ ...f, payee: e.target.value }))}
                className="h-9 px-3 rounded-[10px] text-[14px] outline-none"
                style={{ background: 'var(--v2-surface)', border: '1px solid var(--v2-line-2)' }}
              />
            </label>
            <label className="flex flex-col gap-1">
              <span className="text-[11px] font-medium" style={{ color: 'var(--v2-ink-3)' }}>เลขที่บิล</span>
              <input
                type="text"
                value={form.billNo}
                onChange={(e) => setForm((f) => ({ ...f, billNo: e.target.value }))}
                className="h-9 px-3 rounded-[10px] text-[14px] outline-none"
                style={{ background: 'var(--v2-surface)', border: '1px solid var(--v2-line-2)' }}
              />
            </label>
            <label className="flex flex-col gap-1">
              <span className="text-[11px] font-medium" style={{ color: 'var(--v2-ink-3)' }}>หมวด</span>
              <select
                value={form.group}
                onChange={(e) => setForm((f) => ({ ...f, group: e.target.value }))}
                className="h-9 px-3 rounded-[10px] text-[14px] outline-none"
                style={{ background: 'var(--v2-surface)', border: '1px solid var(--v2-line-2)' }}
              >
                <option value="">— ไม่ระบุ —</option>
                {groupOptions.map((c) => (
                  <option key={c.catId} value={c.idFull ?? ''}>
                    {c.name ?? c.idFull ?? c.legacyId}
                  </option>
                ))}
              </select>
            </label>
            <label className="flex flex-col gap-1">
              <span className="text-[11px] font-medium" style={{ color: 'var(--v2-ink-3)' }}>บัญชี</span>
              <select
                value={form.account}
                onChange={(e) => setForm((f) => ({ ...f, account: e.target.value }))}
                className="h-9 px-3 rounded-[10px] text-[14px] outline-none"
                style={{ background: 'var(--v2-surface)', border: '1px solid var(--v2-line-2)' }}
              >
                <option value="">— ไม่ระบุ —</option>
                {accountOptions.map((c) => (
                  <option key={c.catId} value={c.idFull ?? ''}>
                    {c.name ?? c.idFull ?? c.legacyId}
                  </option>
                ))}
              </select>
            </label>
          </div>

          <label className="flex flex-col gap-1">
            <span className="text-[11px] font-medium" style={{ color: 'var(--v2-ink-3)' }}>หมายเหตุ</span>
            <input
              type="text"
              value={form.note}
              onChange={(e) => setForm((f) => ({ ...f, note: e.target.value }))}
              className="h-9 px-3 rounded-[10px] text-[14px] outline-none"
              style={{ background: 'var(--v2-surface)', border: '1px solid var(--v2-line-2)' }}
            />
          </label>

          {formError && (
            <p className="text-[12.5px]" style={{ color: '#b91c1c' }}>{formError}</p>
          )}

          <div className="flex gap-2">
            <button type="submit" disabled={submitting} className="v2-btn v2-btn-primary v2-btn-sm">
              {submitting ? <Loader2 size={15} className="animate-spin" /> : <Plus size={15} />}
              บันทึก{form.kind === 'income' ? 'รายรับ' : 'รายจ่าย'}
            </button>
          </div>
        </form>
      )}

      {/* Date range */}
      <div className="flex flex-wrap items-end gap-3 v2-no-print">
        <label className="flex flex-col gap-1">
          <span className="text-[11px] font-medium" style={{ color: 'var(--v2-ink-3)' }}>จากวันที่</span>
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
        <V2Spinner label="กำลังโหลดรายการ…" />
      ) : !data || data.entries.length === 0 ? (
        <V2Empty title="ไม่มีรายการในช่วงนี้" hint="ลองขยายช่วงวันที่ หรือเพิ่มรายการใหม่" />
      ) : (
        <div id="cash-print" className="v2-print space-y-5">
          <div className="v2-print-only" style={{ marginBottom: '10px' }}>
            <div style={{ fontSize: '15px', fontWeight: 700 }}>รายรับ-รายจ่าย</div>
            <div style={{ fontSize: '11px', color: '#555' }}>
              {formatThai(data.from)} – {formatThai(data.to)}
            </div>
          </div>

          {/* Totals */}
          <div className="grid gap-3 grid-cols-1 sm:grid-cols-3">
            <StatCard label="รายรับ" value={formatCurrency(t!.income, 2)} icon={TrendingUp} tone="income" />
            <StatCard label="รายจ่าย" value={formatCurrency(t!.expense, 2)} icon={TrendingDown} tone="expense" />
            <StatCard label="คงเหลือ" value={formatCurrency(t!.balance, 2)} icon={Wallet} tone="balance" />
          </div>

          {/* Entries table */}
          <div className="v2-card overflow-x-auto">
            <table className="w-full text-[13px] border-collapse">
              <thead>
                <tr style={{ borderBottom: '1px solid var(--v2-line)' }}>
                  <th className="px-3 py-2.5 font-semibold text-left whitespace-nowrap" style={{ color: 'var(--v2-ink-2)' }}>วันที่</th>
                  <th className="px-3 py-2.5 font-semibold text-left whitespace-nowrap" style={{ color: 'var(--v2-ink-2)' }}>ประเภท</th>
                  <th className="px-3 py-2.5 font-semibold text-left" style={{ color: 'var(--v2-ink-2)' }}>รายละเอียด</th>
                  <th className="px-3 py-2.5 font-semibold text-left whitespace-nowrap" style={{ color: 'var(--v2-ink-2)' }}>หมวด/บัญชี</th>
                  <th className="px-3 py-2.5 font-semibold text-right whitespace-nowrap" style={{ color: 'var(--v2-ink-2)' }}>จำนวนเงิน</th>
                </tr>
              </thead>
              <tbody>
                {data.entries.map((e) => {
                  const isExpense = e.kind === 'expense'
                  const label =
                    e.kind === 'income' ? 'รายรับ' : e.kind === 'expense' ? 'รายจ่าย' : (e.legacyType || '—')
                  return (
                    <tr key={e.cashId} style={{ borderBottom: '1px solid var(--v2-line)' }}>
                      <td className="px-3 py-2.5 v2-num whitespace-nowrap" style={{ color: 'var(--v2-ink-3)' }}>
                        {e.entryDate ? formatThai(e.entryDate) : '—'}
                      </td>
                      <td className="px-3 py-2.5 whitespace-nowrap">
                        <span
                          className="text-[11px] px-1.5 py-0.5 rounded"
                          style={{
                            background: isExpense ? '#fee2e2' : 'var(--v2-wine-50)',
                            color: isExpense ? '#b91c1c' : 'var(--v2-wine-600)',
                          }}
                        >
                          {label}
                        </span>
                      </td>
                      <td className="px-3 py-2.5">
                        <div>{e.payee || '—'}</div>
                        {(e.note || e.billNo) && (
                          <div className="text-[11.5px]" style={{ color: 'var(--v2-ink-3)' }}>
                            {e.billNo ? `#${e.billNo}` : ''}{e.billNo && e.note ? ' · ' : ''}{e.note || ''}
                          </div>
                        )}
                      </td>
                      <td className="px-3 py-2.5 whitespace-nowrap" style={{ color: 'var(--v2-ink-3)' }}>
                        {[e.group, e.account].filter(Boolean).join(' / ') || '—'}
                      </td>
                      <td
                        className="px-3 py-2.5 v2-num text-right font-semibold whitespace-nowrap"
                        style={{ color: isExpense ? '#b91c1c' : 'var(--v2-ink)' }}
                      >
                        {isExpense ? '−' : ''}{formatCurrency(e.amount, 2)}
                      </td>
                    </tr>
                  )
                })}
              </tbody>
            </table>
          </div>
          <p className="text-[11.5px]" style={{ color: 'var(--v2-ink-3)' }}>
            {t!.count} รายการ · รายการที่บันทึกในแอปนี้จะยังไม่ส่งกลับไปยังระบบเดิม (อยู่ระหว่างตรวจสอบรูปแบบข้อมูล)
          </p>
        </div>
      )}
    </div>
  )
}
