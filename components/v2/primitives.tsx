'use client'

import type { ReactNode } from 'react'
import { Loader2, Info } from 'lucide-react'
import type { V2StatusView } from '@/lib/v2/status'
import { useBranch, type Branch } from '@/contexts/BranchContext'

/**
 * View-only banner for branches the new UI can't WRITE to. `/v2` reads are
 * branch-aware (HF Ville reads ville_pool) so data always DISPLAYS; write
 * capability is driven by `canWrite` (HF Hotel always; HF Ville only when
 * `HFVILLE_WRITES_ENABLED`, surfaced via `/api/mode`). The banner now keys off
 * `canWrite` — NOT the branch — so it disappears the moment HF Ville writes are
 * enabled (it previously lied, hard-coded to every non-HF-Hotel branch). Renders
 * nothing whenever the active branch is writable.
 */
export function VilleNotice({ branch }: { branch: Branch }) {
  const { canWrite } = useBranch()
  if (canWrite) return null
  const message =
    branch === 'hfville'
      ? 'HF Ville: โหมดดูอย่างเดียวในดีไซน์ใหม่ — ทำรายการผ่าน iHOTEL / หน้าจอเดิม'
      : 'มุมมองรวม (ทั้งหมด): โหมดดูอย่างเดียว — เลือกสาขา HF Hotel เพื่อทำรายการ'
  return (
    <div
      className="v2-card flex items-center gap-3 px-4 py-3"
      style={{ borderColor: 'var(--v2-dep)', background: 'var(--v2-dep-bg)' }}
      role="status"
    >
      <Info size={18} style={{ color: 'var(--v2-dep)' }} />
      <p className="text-[13.5px] flex-1" style={{ color: 'var(--v2-ink)' }}>{message}</p>
      <a href="/" className="v2-btn v2-btn-soft v2-btn-sm">หน้าจอเดิม</a>
    </div>
  )
}

export function V2Spinner({ label }: { label?: string }) {
  return (
    <div className="flex flex-col items-center justify-center py-20 gap-3">
      <Loader2 className="animate-spin" size={26} style={{ color: 'var(--v2-wine-500)' }} />
      {label && <p className="text-[13px]" style={{ color: 'var(--v2-ink-3)' }}>{label}</p>}
    </div>
  )
}

export function LiveDot({ connected }: { connected: boolean }) {
  return (
    <span className="inline-flex items-center gap-1.5 text-[12px]" style={{ color: 'var(--v2-ink-3)' }}>
      <span
        className={`v2-dot ${connected ? 'v2-live-dot' : ''}`}
        style={{ background: connected ? 'var(--v2-ok)' : 'var(--v2-ink-3)' }}
      />
      {connected ? 'อัปเดตสด' : 'กำลังเชื่อมต่อ'}
    </span>
  )
}

export function StatusPill({ view }: { view: V2StatusView }) {
  return (
    <span className={`v2-pill ${view.cls}`}>
      <span className={`v2-dot ${view.dot}`} />
      {view.label}
    </span>
  )
}

export function V2PageHeader({
  eyebrow,
  title,
  right,
}: {
  eyebrow?: string
  title: string
  right?: ReactNode
}) {
  return (
    <div className="flex items-end justify-between gap-3 mb-5">
      <div>
        {eyebrow && <div className="v2-eyebrow mb-1">{eyebrow}</div>}
        <h1 className="v2-display text-[26px] lg:text-[30px] leading-none">{title}</h1>
      </div>
      {right && <div className="flex items-center gap-2 shrink-0">{right}</div>}
    </div>
  )
}

export function V2Section({
  title,
  count,
  tone,
  action,
  children,
}: {
  title: string
  count?: number
  tone?: string
  action?: ReactNode
  children: ReactNode
}) {
  return (
    <section className="v2-card overflow-hidden">
      <div className="flex items-center gap-2.5 px-4 lg:px-5 h-13 py-3" style={{ borderBottom: '1px solid var(--v2-line)' }}>
        <h2 className="text-[15px] font-semibold">{title}</h2>
        {count !== undefined && (
          <span className={`v2-num text-[12px] font-bold px-2 py-0.5 rounded-full ${tone || 's-mut'}`}>{count}</span>
        )}
        <div className="flex-1" />
        {action}
      </div>
      {children}
    </section>
  )
}

export function V2Empty({ title, hint }: { title: string; hint?: string }) {
  return (
    <div className="px-5 py-10 text-center">
      <p className="text-[14px] font-medium" style={{ color: 'var(--v2-ink-2)' }}>{title}</p>
      {hint && <p className="text-[12.5px] mt-1" style={{ color: 'var(--v2-ink-3)' }}>{hint}</p>}
    </div>
  )
}
