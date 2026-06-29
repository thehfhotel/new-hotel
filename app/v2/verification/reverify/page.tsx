'use client'

import { Suspense, useCallback, useEffect, useState, type ReactNode } from 'react'
import { useSearchParams } from 'next/navigation'
import { CheckCircle2, Loader2, Send, ClipboardCheck, ArrowLeft, AlertTriangle, RefreshCcw } from 'lucide-react'
import { useBranch } from '@/contexts/BranchContext'
import { useAuth } from '@/contexts/AuthContext'
import { V2PageHeader } from '@/components/v2/primitives'

/**
 * Per-site RE-VERIFICATION form (ตรวจสอบซ้ำ) — DATA-DRIVEN (Tier 1).
 *
 * The question list is no longer hardcoded here: this page fetches the form
 * definition from `GET /api/feedback/forms/reverify_{site}` and renders whatever
 * the `schema` says, so IT can edit/add questions with a DB write (no frontend
 * rebuild). The target site comes from `?site=hfhotel|hfville` (so IT can hand a
 * receptionist a direct link), falling back to the globally-selected branch.
 *
 * Answers POST to /api/verification tagged `{ kind: form.kind, site, ... }`.
 * Plain fetch (NOT branchFetch): verification feedback is stored in the primary
 * DB tagged by `site`, so we must NOT append ?branch=hfville (that would trip
 * ville_write_guard). The form submits fine from any selected branch.
 */

// ── schema types (mirrors the backend ht_feedback_forms.form_schema shape) ──

interface SchemaOption {
  value: string
  label: string
}
interface SchemaQuestion {
  id: string
  type: string // 'radio' | 'text' (unknown types render nothing)
  label: string
  required?: boolean
  placeholder?: string
  options?: SchemaOption[]
  showIf?: { field: string; equals: string }
}
interface FeedbackForm {
  key: string
  site: string | null
  kind: string
  title: string
  intro: string | null
  schema: { questions?: SchemaQuestion[] }
  sort: number
}

type Site = 'hfhotel' | 'hfville'

// ── small presentational primitives (replicated from the #58 form) ─────────

const inputCls = 'h-9 px-3 rounded-[10px] text-[14px] outline-none w-full'
const inputStyle = { background: 'var(--v2-surface)', border: '1px solid var(--v2-line-2)' } as const

function TextInput({
  value,
  onChange,
  placeholder,
}: {
  value: string
  onChange: (v: string) => void
  placeholder?: string
}) {
  return (
    <input
      type="text"
      value={value}
      placeholder={placeholder}
      onChange={(e) => onChange(e.target.value)}
      className={inputCls}
      style={inputStyle}
    />
  )
}

function RadioGroup({
  name,
  value,
  options,
  onChange,
}: {
  name: string
  value: string
  options: SchemaOption[]
  onChange: (v: string) => void
}) {
  return (
    <div className="flex flex-col gap-1.5" role="radiogroup">
      {options.map((o) => (
        <label key={o.value} className="flex items-start gap-2.5 cursor-pointer text-[14px]">
          <input
            type="radio"
            name={name}
            checked={value === o.value}
            onChange={() => onChange(o.value)}
            className="mt-0.5"
            style={{ accentColor: 'var(--v2-wine-600)' }}
          />
          <span>{o.label}</span>
        </label>
      ))}
    </div>
  )
}

function Question({ title, children }: { title: string; children?: ReactNode }) {
  return (
    <div className="space-y-2">
      <p className="text-[14px] font-semibold leading-snug">{title}</p>
      {children}
    </div>
  )
}

// ── visibility helper for conditional (showIf) questions ────────────────────

function isVisible(q: SchemaQuestion, answers: Record<string, string>): boolean {
  if (!q.showIf) return true
  return answers[q.showIf.field] === q.showIf.equals
}

export default function V2Reverify() {
  return (
    <Suspense
      fallback={
        <div className="v2-card p-8 flex justify-center">
          <Loader2 size={22} className="animate-spin" style={{ color: 'var(--v2-ink-3)' }} />
        </div>
      }
    >
      <ReverifyInner />
    </Suspense>
  )
}

function ReverifyInner() {
  const searchParams = useSearchParams()
  const { branch } = useBranch()
  const { user } = useAuth()

  // Resolve the target site: ?site= wins, otherwise the selected branch
  // (which may be 'all') falls back to HF Hotel.
  const siteParam = searchParams?.get('site')
  const site: Site =
    siteParam === 'hfhotel' || siteParam === 'hfville'
      ? siteParam
      : branch === 'hfville'
        ? 'hfville'
        : 'hfhotel'
  const siteLabel = site === 'hfhotel' ? 'HF Hotel' : 'HF Ville'

  const [formDef, setFormDef] = useState<FeedbackForm | null>(null)
  const [loading, setLoading] = useState(true)
  const [loadError, setLoadError] = useState<string | null>(null)

  const [answers, setAnswers] = useState<Record<string, string>>({})
  const [inspector, setInspector] = useState('')
  const [submitting, setSubmitting] = useState(false)
  const [error, setError] = useState<string | null>(null)
  const [submittedId, setSubmittedId] = useState<number | null>(null)

  const loadForm = useCallback(async () => {
    setLoading(true)
    setLoadError(null)
    try {
      const res = await fetch(`/api/feedback/forms/reverify_${site}`)
      if (res.status === 404) {
        setFormDef(null)
        setLoadError('notfound')
        return
      }
      if (!res.ok) {
        setLoadError('โหลดแบบฟอร์มไม่สำเร็จ')
        return
      }
      const body = (await res.json()) as FeedbackForm
      setFormDef(body)
      setAnswers({})
    } catch {
      setLoadError('เกิดข้อผิดพลาดในการเชื่อมต่อ')
    } finally {
      setLoading(false)
    }
  }, [site])

  useEffect(() => {
    loadForm()
  }, [loadForm])

  const questions = formDef?.schema?.questions ?? []
  const visibleQuestions = questions.filter((q) => isVisible(q, answers))

  const setAnswer = (id: string, value: string) => setAnswers((a) => ({ ...a, [id]: value }))

  // Submit-gate: every required, currently-visible question must be answered.
  const primaryComplete = visibleQuestions
    .filter((q) => q.required)
    .every((q) => !!(answers[q.id] ?? '').trim())

  const buildAnswers = (): Record<string, string> => {
    const out: Record<string, string> = { kind: formDef?.kind || 'reverify', site }
    for (const q of visibleQuestions) {
      const v = (answers[q.id] ?? '').trim()
      if (v) out[q.id] = v
    }
    return out
  }

  const submit = async (e: React.FormEvent) => {
    e.preventDefault()
    setError(null)
    if (!primaryComplete) {
      setError('กรุณาตอบข้อที่จำเป็นให้ครบก่อนส่งนะครับ/ค่ะ 🙏')
      return
    }
    setSubmitting(true)
    try {
      const res = await fetch('/api/verification', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({
          inspector: inspector.trim() || user?.username || null,
          answers: buildAnswers(),
          overall: null,
          site,
        }),
      })
      if (!res.ok) {
        const body = await res.json().catch(() => null)
        setError(body?.error || body?.message || 'บันทึกไม่สำเร็จ')
        return
      }
      const body = await res.json()
      setSubmittedId(typeof body?.id === 'number' ? body.id : 0)
      setAnswers({})
      window.scrollTo({ top: 0, behavior: 'smooth' })
    } catch {
      setError('เกิดข้อผิดพลาดในการเชื่อมต่อ')
    } finally {
      setSubmitting(false)
    }
  }

  // ── success ──
  if (submittedId !== null) {
    return (
      <div className="space-y-6">
        <V2PageHeader eyebrow="ตรวจสอบซ้ำ" title={`ยืนยันการแก้ไข — ${siteLabel}`} />
        <div className="v2-card p-8 flex flex-col items-center text-center gap-3">
          <CheckCircle2 size={44} style={{ color: 'var(--v2-ok)' }} />
          <p className="text-[17px] font-semibold">บันทึกผลตรวจสอบซ้ำเรียบร้อยแล้ว ขอบคุณมากครับ/ค่ะ 🙏</p>
          <p className="text-[13.5px]" style={{ color: 'var(--v2-ink-3)' }}>
            ทีมไอทีจะนำคำตอบของคุณไปตรวจสอบต่อ {submittedId ? `(หมายเลขอ้างอิง #${submittedId})` : ''}
          </p>
          <div className="flex flex-wrap items-center justify-center gap-2 mt-2">
            <button type="button" onClick={() => setSubmittedId(null)} className="v2-btn v2-btn-soft v2-btn-sm">
              <ClipboardCheck size={15} /> ตรวจสอบอีกครั้ง
            </button>
            <a href="/v2/verification/results" className="v2-btn v2-btn-soft v2-btn-sm">
              <ArrowLeft size={15} /> ดูผลทั้งหมด
            </a>
          </div>
        </div>
      </div>
    )
  }

  // ── loading ──
  if (loading) {
    return (
      <div className="space-y-6">
        <V2PageHeader eyebrow="ตรวจสอบซ้ำ" title={`ยืนยันการแก้ไข — ${siteLabel}`} />
        <div className="v2-card p-8 flex justify-center">
          <Loader2 size={22} className="animate-spin" style={{ color: 'var(--v2-ink-3)' }} />
        </div>
      </div>
    )
  }

  // ── not found / load error ──
  if (loadError) {
    const notFound = loadError === 'notfound'
    return (
      <div className="space-y-6">
        <V2PageHeader eyebrow="ตรวจสอบซ้ำ" title={`ยืนยันการแก้ไข — ${siteLabel}`} />
        <div className="v2-card p-8 flex flex-col items-center text-center gap-3">
          <AlertTriangle size={36} style={{ color: 'var(--v2-ink-3)' }} />
          <p className="text-[15px] font-semibold">
            {notFound ? `ยังไม่มีแบบฟอร์มตรวจสอบซ้ำสำหรับ ${siteLabel}` : 'โหลดแบบฟอร์มไม่สำเร็จ'}
          </p>
          {!notFound && (
            <button type="button" onClick={loadForm} className="v2-btn v2-btn-soft v2-btn-sm mt-1">
              <RefreshCcw size={15} /> ลองใหม่
            </button>
          )}
          <a href="/v2/verification/results" className="v2-btn v2-btn-soft v2-btn-sm">
            <ArrowLeft size={15} /> กลับไปหน้าผลตรวจสอบ
          </a>
        </div>
      </div>
    )
  }

  // ── the data-driven form ──
  return (
    <form onSubmit={submit} className="space-y-5 pb-8">
      <V2PageHeader eyebrow="ตรวจสอบซ้ำ" title={formDef?.title || `ยืนยันการแก้ไข — ${siteLabel}`} />

      {formDef?.intro && (
        <p className="text-[13px] -mt-2" style={{ color: 'var(--v2-ink-3)' }}>
          {formDef.intro}
        </p>
      )}

      {/* ผู้ตรวจ */}
      <div className="v2-card p-4">
        <label className="flex flex-col gap-1 max-w-sm">
          <span className="text-[11px] font-medium" style={{ color: 'var(--v2-ink-3)' }}>
            ผู้ตรวจ {user?.username ? `(ค่าเริ่มต้น: ${user.username})` : ''}
          </span>
          <TextInput value={inspector} onChange={setInspector} placeholder="ชื่อผู้ตรวจ" />
        </label>
      </div>

      {/* schema-driven questions */}
      <section className="v2-card p-4 lg:p-5 space-y-4">
        <h2 className="text-[16px] font-semibold">รายการตรวจสอบซ้ำ — {siteLabel}</h2>
        {visibleQuestions.length === 0 && (
          <p className="text-[13px]" style={{ color: 'var(--v2-ink-3)' }}>
            แบบฟอร์มนี้ยังไม่มีคำถาม
          </p>
        )}
        {visibleQuestions.map((q) => {
          if (q.type === 'radio') {
            return (
              <Question key={q.id} title={`${q.label}${q.required ? ' *' : ''}`}>
                <RadioGroup
                  name={q.id}
                  value={answers[q.id] ?? ''}
                  options={q.options ?? []}
                  onChange={(v) => setAnswer(q.id, v)}
                />
              </Question>
            )
          }
          if (q.type === 'text') {
            return (
              <Question key={q.id} title={`${q.label}${q.required ? ' *' : ''}`}>
                <TextInput
                  value={answers[q.id] ?? ''}
                  onChange={(v) => setAnswer(q.id, v)}
                  placeholder={q.placeholder}
                />
              </Question>
            )
          }
          // unknown type — render nothing (defensive)
          return null
        })}
      </section>

      {error && (
        <p className="text-[13px] font-medium" style={{ color: '#b91c1c' }}>
          {error}
        </p>
      )}

      <div className="flex items-center gap-3">
        <button type="submit" disabled={submitting || !primaryComplete} className="v2-btn v2-btn-primary">
          {submitting ? <Loader2 size={16} className="animate-spin" /> : <Send size={16} />}
          ส่งผลตรวจสอบซ้ำ
        </button>
        <span className="text-[12px]" style={{ color: 'var(--v2-ink-3)' }}>
          {!primaryComplete ? `กรุณาตอบข้อที่จำเป็น (*) ให้ครบก่อนส่ง` : 'คำตอบจะถูกบันทึกในระบบให้ทีมไอทีตรวจสอบ'}
        </span>
      </div>
    </form>
  )
}
