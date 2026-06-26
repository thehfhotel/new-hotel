'use client'

import { useCallback, useEffect, useRef, useState, type ReactNode } from 'react'
import Link from 'next/link'
import { AlertTriangle, Clock, FileText, Loader2, Lock, Unlock, X } from 'lucide-react'
import { useBranch } from '@/contexts/BranchContext'
import { useBranchFetch } from '@/lib/use-branch-fetch'
import { useAuth } from '@/contexts/AuthContext'
import RoundCloseSheet from './RoundCloseSheet'
import { RoundReportSheet } from './RoundReport'

/**
 * Cashier-round (HT_Round_Bill / รอบบิล) control for the v2 home screen.
 *
 * Reads the current open round per-branch from GET /api/shifts/current
 * (mirrored from iHOTEL by `sync_round_bills`). When the backend enables
 * co-equal round writeback (`roundWritebackEnabled`, from /api/mode) AND the
 * branch is writable (`canWrite` — HF Hotel only today), it exposes Open/Close
 * actions that POST /api/shifts/open|close. When writeback is OFF the control
 * is read-only: it still surfaces the "no open round" gate warning (check-in /
 * check-out are suspended) and points at the classic screen, where iHOTEL owns
 * the round. Subsumes the inline shift-gate banner the home page used to render.
 */

interface ShiftDto {
  shiftId: number
  shiftNo: number
  openedBy: string
  openedAt: string
  legacyRoundId: number | null
}

/** ht_shifts timestamps are real UTC instants (TIMESTAMPTZ), unlike the
 *  legacy fake-Z values — so render them in Bangkok wall-clock time. */
function formatThai(iso: string): string {
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

export default function RoundControl({ onChanged }: { onChanged?: () => void }) {
  const { branch, canWrite, roundWritebackEnabled } = useBranch()
  const branchFetch = useBranchFetch()
  const { user } = useAuth()

  const [shift, setShift] = useState<ShiftDto | null>(null)
  const [loaded, setLoaded] = useState(false)
  const [mode, setMode] = useState<'idle' | 'opening'>('idle')
  const [busy, setBusy] = useState(false)
  const [error, setError] = useState<string | null>(null)
  const [openingFloat, setOpeningFloat] = useState('3000')
  const [actor, setActor] = useState('')
  // Close/report sheets carry the shiftId so they survive `shift` going null
  // after a successful close (the close sheet shows the final report).
  const [closingShiftId, setClosingShiftId] = useState<number | null>(null)
  const [viewingShiftId, setViewingShiftId] = useState<number | null>(null)

  // Latest-wins guard: branch can flip (hfhotel default → stored value) mid-flight.
  const reqRef = useRef(0)

  const canManage = roundWritebackEnabled && canWrite

  const fetchCurrent = useCallback(async () => {
    const token = ++reqRef.current
    try {
      const res = await branchFetch('/api/shifts/current')
      if (token !== reqRef.current) return
      if (res.ok) {
        const d = await res.json()
        setShift((d?.shift ?? null) as ShiftDto | null)
      } else {
        // 404 = no open round for this site (the gate-warning state).
        setShift(null)
      }
    } catch {
      setShift(null)
    } finally {
      if (token === reqRef.current) setLoaded(true)
    }
  }, [branchFetch])

  useEffect(() => {
    setLoaded(false)
    fetchCurrent()
  }, [fetchCurrent])

  // Prefill the cashier name from the signed-in user whenever a form opens.
  useEffect(() => {
    if (mode !== 'idle') setActor((a) => a || user?.username || '')
  }, [mode, user])

  const readError = async (res: Response): Promise<string> => {
    try {
      const d = await res.json()
      return d?.error || d?.message || `เกิดข้อผิดพลาด (${res.status})`
    } catch {
      return `เกิดข้อผิดพลาด (${res.status})`
    }
  }

  const submitOpen = async () => {
    const floatVal = Number(openingFloat)
    if (!actor.trim()) return setError('กรุณาระบุชื่อผู้เปิดรอบ')
    if (!Number.isFinite(floatVal) || floatVal < 0) return setError('เงินในลิ้นชักไม่ถูกต้อง')
    setBusy(true)
    setError(null)
    try {
      const res = await branchFetch('/api/shifts/open', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ openedBy: actor.trim(), openingFloat: floatVal }),
      })
      if (!res.ok) {
        setError(await readError(res))
        return
      }
      setMode('idle')
      await fetchCurrent()
      onChanged?.()
    } catch {
      setError('เชื่อมต่อไม่สำเร็จ')
    } finally {
      setBusy(false)
    }
  }

  if (!loaded) return null

  let body: ReactNode
  if (shift) {
    // --- OPEN round ---
    const meta = `รอบ #${shift.shiftNo} · เปิดโดย ${shift.openedBy} · ${formatThai(shift.openedAt)}`
    const viewBtn = (
      <button className="v2-btn v2-btn-ghost v2-btn-sm" onClick={() => setViewingShiftId(shift.shiftId)}>
        <FileText size={15} /> ดูรายงานรอบบิล
      </button>
    )
    if (!canManage) {
      // Read-only: calm status + the (informational) report view. iHOTEL owns
      // the round (open/close stays gated on canManage), but the income-by-tender
      // REPORT is read-only and safe to surface even with writes off — it reads
      // ht_payment_ledger, which the J7e backfill populated for both sites
      // (HF Hotel + Ville), so the numbers match iHOTEL's View_RBill_H.
      body = (
        <div className="v2-inset flex flex-wrap items-center gap-2.5 px-4 py-2.5">
          <Clock size={15} style={{ color: 'var(--v2-ink-3)' }} />
          <span className="text-[13px]" style={{ color: 'var(--v2-ink-2)' }}>รอบบิลเปิดอยู่</span>
          <span className="text-[12.5px] v2-num" style={{ color: 'var(--v2-ink-3)' }}>{meta}</span>
          <span className="flex-1" />
          {viewBtn}
          <span className="text-[11.5px]" style={{ color: 'var(--v2-ink-3)' }}>จัดการผ่าน iHOTEL</span>
        </div>
      )
    } else {
      body = (
        <div className="v2-card px-4 py-3" style={{ borderColor: 'var(--v2-line)' }}>
          <div className="flex flex-wrap items-center gap-3">
            <span className="v2-dot d-ok" style={{ width: 9, height: 9 }} />
            <div className="flex-1 min-w-0">
              <div className="text-[14px] font-semibold">รอบบิลเปิดอยู่</div>
              <div className="text-[12.5px] v2-num truncate" style={{ color: 'var(--v2-ink-3)' }}>{meta}</div>
            </div>
            {viewBtn}
            <button className="v2-btn v2-btn-ghost v2-btn-sm" onClick={() => setClosingShiftId(shift.shiftId)}>
              <Lock size={15} /> ปิดรอบบิล
            </button>
          </div>
        </div>
      )
    }
  } else {
    // --- CLOSED (no open round): gate warning ---
    body = (
      <div className="v2-card px-4 py-3" style={{ borderColor: 'var(--v2-arr)', background: 'var(--v2-arr-bg)' }}>
        <div className="flex items-center gap-3">
          <AlertTriangle size={18} style={{ color: 'var(--v2-arr)' }} />
          <p className="text-[13.5px] flex-1" style={{ color: 'var(--v2-ink)' }}>
            ยังไม่ได้เปิดรอบบิล — การเช็คอิน/เช็คเอาท์จะถูกระงับจนกว่าจะเปิดรอบบิล
          </p>
          {canManage ? (
            mode !== 'opening' && (
              <button className="v2-btn v2-btn-soft v2-btn-sm" onClick={() => { setError(null); setMode('opening') }}>
                <Unlock size={15} /> เปิดรอบบิล
              </button>
            )
          ) : (
            <Link href="/billing" className="v2-btn v2-btn-soft v2-btn-sm">เปิดรอบบิล</Link>
          )}
        </div>
        {canManage && mode === 'opening' && (
          <div className="mt-3 pt-3 flex flex-wrap items-end gap-2" style={{ borderTop: '1px solid var(--v2-arr)' }}>
            <Field label="ผู้เปิดรอบ">
              <input
                value={actor}
                onChange={(e) => setActor(e.target.value)}
                className="v2-num h-9 px-3 rounded-[10px] text-[14px] outline-none w-40"
                style={{ background: 'var(--v2-surface)', border: '1px solid var(--v2-line-2)' }}
              />
            </Field>
            <Field label="เงินในลิ้นชัก (บาท)">
              <input
                value={openingFloat}
                onChange={(e) => setOpeningFloat(e.target.value)}
                inputMode="numeric"
                className="v2-num h-9 px-3 rounded-[10px] text-[14px] outline-none w-32"
                style={{ background: 'var(--v2-surface)', border: '1px solid var(--v2-line-2)' }}
              />
            </Field>
            <button className="v2-btn v2-btn-primary v2-btn-sm" disabled={busy} onClick={submitOpen}>
              {busy ? <Loader2 size={15} className="animate-spin" /> : <Unlock size={15} />} ยืนยันเปิดรอบบิล
            </button>
            <button className="v2-btn v2-btn-ghost v2-btn-sm" disabled={busy} onClick={() => { setMode('idle'); setError(null) }}>
              <X size={15} /> ยกเลิก
            </button>
          </div>
        )}
        {error && <ErrorLine msg={error} />}
      </div>
    )
  }

  return (
    <>
      {body}
      {/* Sheets are gated on their own shiftId (not `shift`) so the close sheet
          stays mounted to show the final report after the round goes closed. */}
      {closingShiftId != null && (
        <RoundCloseSheet
          shiftId={closingShiftId}
          onClose={() => setClosingShiftId(null)}
          onClosed={() => {
            fetchCurrent()
            onChanged?.()
          }}
        />
      )}
      {viewingShiftId != null && (
        <RoundReportSheet shiftId={viewingShiftId} onClose={() => setViewingShiftId(null)} />
      )}
    </>
  )
}

function Field({ label, children }: { label: string; children: React.ReactNode }) {
  return (
    <label className="flex flex-col gap-1">
      <span className="text-[11px] font-medium" style={{ color: 'var(--v2-ink-3)' }}>{label}</span>
      {children}
    </label>
  )
}

function ErrorLine({ msg }: { msg: string }) {
  return (
    <p className="text-[12.5px] mt-2" style={{ color: 'var(--v2-wine-600)' }}>{msg}</p>
  )
}
