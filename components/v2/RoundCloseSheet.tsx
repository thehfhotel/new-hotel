'use client'

import { useEffect, useMemo, useState } from 'react'
import { X, Lock, Loader2, CheckCircle2 } from 'lucide-react'
import { useBranchFetch } from '@/lib/use-branch-fetch'
import { useAuth } from '@/contexts/AuthContext'
import { formatCurrency } from '@/lib/format'
import { V2Spinner } from './primitives'
import RoundReport, { type RoundReportData } from './RoundReport'

/**
 * Round-close reconciliation flow — Track J7d. Shows the live round report
 * (income by tender + expected cash), takes a physical cash-drawer count by
 * denomination with a live counted-vs-expected variance, then POSTs the close
 * with `cashCount` and shows the final report. Mounted only when the cashier
 * can manage the round (`roundWritebackEnabled && canWrite`).
 */

// Thai bank notes + coins, largest first.
const DENOMS = [1000, 500, 100, 50, 20, 10, 5, 1, 0.5, 0.25]

function denomLabel(d: number): string {
  return d >= 1 ? `฿${d}` : `${(d * 100).toFixed(0)} สต.`
}

export default function RoundCloseSheet({
  shiftId,
  onClose,
  onClosed,
}: {
  shiftId: number
  onClose: () => void
  onClosed: () => void
}) {
  const branchFetch = useBranchFetch()
  const { user } = useAuth()

  const [preview, setPreview] = useState<RoundReportData | null>(null)
  const [loaded, setLoaded] = useState(false)
  const [counts, setCounts] = useState<Record<string, string>>({})
  const [closedBy, setClosedBy] = useState('')
  const [busy, setBusy] = useState(false)
  const [error, setError] = useState<string | null>(null)
  const [final, setFinal] = useState<RoundReportData | null>(null)

  useEffect(() => {
    let alive = true
    branchFetch(`/api/shifts/${shiftId}/report`)
      .then(async (r) => {
        if (alive && r.ok) setPreview(await r.json())
      })
      .catch(() => {})
      .finally(() => {
        if (alive) setLoaded(true)
      })
    return () => {
      alive = false
    }
  }, [branchFetch, shiftId])

  useEffect(() => {
    setClosedBy((a) => a || user?.username || '')
  }, [user])

  const countedCash = useMemo(
    () =>
      DENOMS.reduce((sum, d) => {
        const n = parseFloat(counts[String(d)] || '0')
        return sum + (Number.isFinite(n) && n > 0 ? d * n : 0)
      }, 0),
    [counts],
  )
  const expected = preview?.expectedCash ?? 0
  const variance = Math.round((countedCash - expected) * 100) / 100
  const anyCount = countedCash > 0
  const tone = !anyCount ? 'mut' : variance === 0 ? 'ok' : variance < 0 ? 'occ' : 'arr'

  const submit = async () => {
    if (!closedBy.trim()) return setError('กรุณาระบุชื่อผู้ปิดรอบ')
    setBusy(true)
    setError(null)
    const cashCount: Record<string, number> = {}
    for (const d of DENOMS) {
      const n = parseFloat(counts[String(d)] || '0')
      if (Number.isFinite(n) && n > 0) cashCount[String(d)] = n
    }
    try {
      const res = await branchFetch('/api/shifts/close', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ closedBy: closedBy.trim(), cashCount }),
      })
      if (!res.ok) {
        let msg = `เกิดข้อผิดพลาด (${res.status})`
        try {
          const d = await res.json()
          msg = d?.error || d?.message || msg
        } catch {
          /* keep default */
        }
        setError(msg)
        return
      }
      onClosed() // refresh the dashboard / parent
      // Re-fetch the now-closed report (carries counted + variance).
      const rep = await branchFetch(`/api/shifts/${shiftId}/report`)
      if (rep.ok) setFinal(await rep.json())
      else onClose() // closed fine, but report fetch failed — just dismiss
    } catch {
      setError('เชื่อมต่อไม่สำเร็จ')
    } finally {
      setBusy(false)
    }
  }

  return (
    <div className="v2-sheet-backdrop" onClick={busy ? undefined : onClose} role="dialog" aria-modal="true">
      <div
        className="mt-auto lg:mt-0 lg:ml-auto w-full lg:w-[460px] lg:h-full v2-sheet-up lg:[animation:v2-panel-in_.24s_cubic-bezier(.2,.8,.2,1)]"
        style={{ background: 'var(--v2-surface)' }}
        onClick={(e) => e.stopPropagation()}
      >
        <div className="flex items-center gap-3 px-5 py-4" style={{ borderBottom: '1px solid var(--v2-line)' }}>
          <div className="v2-eyebrow flex-1">{final ? 'ปิดรอบบิลเรียบร้อย' : 'ปิดรอบบิล — กระทบยอด'}</div>
          <button onClick={onClose} className="p-2 rounded-full" style={{ color: 'var(--v2-ink-3)' }} aria-label="ปิด">
            <X size={20} />
          </button>
        </div>

        <div className="px-5 py-4 overflow-y-auto" style={{ maxHeight: 'calc(100vh - 80px)' }}>
          {final ? (
            <div className="space-y-4">
              <div
                className="flex items-center gap-2.5 px-4 py-3 rounded-[12px]"
                style={{ background: 'var(--v2-ok-bg)', color: 'var(--v2-ok)' }}
              >
                <CheckCircle2 size={18} />
                <span className="text-[13.5px] font-semibold">ปิดรอบบิล #{final.shift.shiftNo} เรียบร้อยแล้ว</span>
              </div>
              <RoundReport data={final} />
              <button className="v2-btn v2-btn-primary w-full" onClick={onClose}>
                เสร็จสิ้น
              </button>
            </div>
          ) : !loaded ? (
            <V2Spinner label="กำลังโหลดสรุปรอบบิล…" />
          ) : (
            <div className="space-y-5">
              {preview && <RoundReport data={preview} />}

              {/* Cash-drawer count */}
              <div>
                <div className="v2-eyebrow mb-2">นับเงินในลิ้นชัก</div>
                <div className="grid grid-cols-2 gap-2">
                  {DENOMS.map((d) => {
                    const n = parseFloat(counts[String(d)] || '0')
                    const sub = Number.isFinite(n) && n > 0 ? d * n : 0
                    return (
                      <div
                        key={d}
                        className="flex items-center gap-2 px-2.5 h-10 rounded-[10px]"
                        style={{ background: 'var(--v2-surface-2)', border: '1px solid var(--v2-line)' }}
                      >
                        <span className="v2-num text-[13px] w-12" style={{ color: 'var(--v2-ink-2)' }}>{denomLabel(d)}</span>
                        <span style={{ color: 'var(--v2-ink-3)' }}>×</span>
                        <input
                          value={counts[String(d)] || ''}
                          onChange={(e) => setCounts((c) => ({ ...c, [String(d)]: e.target.value.replace(/[^0-9]/g, '') }))}
                          inputMode="numeric"
                          placeholder="0"
                          className="v2-num w-12 h-7 px-1.5 rounded-[6px] text-[13px] text-center outline-none"
                          style={{ background: 'var(--v2-surface)', border: '1px solid var(--v2-line-2)' }}
                        />
                        <span className="flex-1" />
                        <span className="v2-num text-[12px]" style={{ color: sub > 0 ? 'var(--v2-ink-2)' : 'var(--v2-ink-3)' }}>
                          {formatCurrency(sub, 0)}
                        </span>
                      </div>
                    )
                  })}
                </div>

                {/* Live counted + variance */}
                <div className="v2-inset mt-3 divide-y" style={{ borderColor: 'var(--v2-line)' }}>
                  <div className="flex items-baseline gap-3 px-4 py-2.5">
                    <span className="text-[13.5px]" style={{ color: 'var(--v2-ink-2)' }}>จำนวนเงินในลิ้นชัก</span>
                    <span className="flex-1" />
                    <span className="v2-num text-[14px]">{formatCurrency(expected, 2)}</span>
                  </div>
                  <div className="flex items-baseline gap-3 px-4 py-2.5">
                    <span className="text-[13.5px]" style={{ color: 'var(--v2-ink-2)' }}>นับได้จริง</span>
                    <span className="flex-1" />
                    <span className="v2-num text-[14px] font-semibold">{formatCurrency(countedCash, 2)}</span>
                  </div>
                  <div className="flex items-baseline gap-3 px-4 py-2.5" style={{ background: `var(--v2-${tone}-bg)` }}>
                    <span className="text-[13.5px] font-semibold" style={{ color: `var(--v2-${tone})` }}>
                      {!anyCount ? 'ส่วนต่าง' : variance === 0 ? 'ตรงพอดี' : variance < 0 ? 'ขาด' : 'เกิน'}
                    </span>
                    <span className="flex-1" />
                    <span className="v2-num text-[14px] font-bold" style={{ color: `var(--v2-${tone})` }}>
                      {variance > 0 ? '+' : ''}{formatCurrency(variance, 2)}
                    </span>
                  </div>
                </div>
                <p className="text-[11.5px] mt-1.5" style={{ color: 'var(--v2-ink-3)' }}>
                  เว้นว่างได้หากไม่ต้องการนับเงิน — ระบบจะบันทึกเฉพาะยอดที่กรอก
                </p>
              </div>

              {/* Closer + confirm */}
              <div className="flex flex-wrap items-end gap-2">
                <label className="flex flex-col gap-1">
                  <span className="text-[11px] font-medium" style={{ color: 'var(--v2-ink-3)' }}>ผู้ปิดรอบ</span>
                  <input
                    value={closedBy}
                    onChange={(e) => setClosedBy(e.target.value)}
                    className="v2-num h-9 px-3 rounded-[10px] text-[14px] outline-none w-44"
                    style={{ background: 'var(--v2-surface)', border: '1px solid var(--v2-line-2)' }}
                  />
                </label>
                <button className="v2-btn v2-btn-primary v2-btn-sm" disabled={busy} onClick={submit}>
                  {busy ? <Loader2 size={15} className="animate-spin" /> : <Lock size={15} />} ยืนยันปิดรอบบิล
                </button>
                <button className="v2-btn v2-btn-ghost v2-btn-sm" disabled={busy} onClick={onClose}>
                  ยกเลิก
                </button>
              </div>
              {error && <p className="text-[12.5px]" style={{ color: 'var(--v2-wine-600)' }}>{error}</p>}
            </div>
          )}
        </div>
      </div>
    </div>
  )
}
