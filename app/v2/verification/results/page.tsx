'use client'

import { Fragment, useCallback, useEffect, useState, type ReactNode } from 'react'
import Link from 'next/link'
import {
  ClipboardCheck,
  ClipboardList,
  RefreshCcw,
  ChevronRight,
  ChevronDown,
  AlertTriangle,
  ListChecks,
} from 'lucide-react'
import { V2PageHeader, V2Spinner, V2Empty } from '@/components/v2/primitives'
// vr_submitted_at is a PG-native TIMESTAMPTZ (a true UTC instant from NOW()), NOT the
// MSSQL "Thai-local-tagged-Z" quirk — so render it in Asia/Bangkok, not as-is UTC.
import { formatThai } from '@/components/v2/RoundReport'

/**
 * Re-verification RESULTS / HUB page (companion to the #58 form at
 * /v2/verification and the DATA-DRIVEN re-verify forms at
 * /v2/verification/reverify). Read-only.
 *
 * Schema-aware (Tier 1): it additionally GETs the form definitions from
 * /api/feedback/forms and derives the per-question labels, option labels, and the
 * radio-question set FROM the schema — so when IT edits a question (a DB write),
 * the tags / tallies / detail labels here update with no code change. The ORIGINAL
 * full checklist (q1_x + q5, no `kind`) is NOT data-driven and keeps its hardcoded
 * handling.
 *
 * The `answers` JSONB is arbitrary, so EVERY key is read defensively with
 * `asStr()`. Reverify submissions carry `answers.kind === 'reverify'`. vr_id 1 is a
 * known misclick row (inspector starts with '[misclick]') — de-emphasised in the
 * table AND excluded from the per-item tallies.
 */

// ── data shapes ──────────────────────────────────────────────────────────────

interface VerificationResponse {
  id: number
  submittedAt: string
  site: string | null
  inspector: string | null
  answers: Record<string, unknown>
  overall: string | null
}

interface SchemaOption {
  value: string
  label: string
}
interface SchemaQuestion {
  id: string
  type: string
  label: string
  options?: SchemaOption[]
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

// ── defensive readers ───────────────────────────────────────────────────────

/** Read a key as a trimmed string, or undefined when missing / not a string. */
function asStr(v: unknown): string | undefined {
  return typeof v === 'string' && v.trim() !== '' ? v : undefined
}

function isReverify(r: VerificationResponse): boolean {
  return asStr(r.answers?.kind) === 'reverify'
}

function isMisclick(r: VerificationResponse): boolean {
  return (r.inspector ?? '').trim().toLowerCase().startsWith('[misclick]')
}

function rowSite(r: VerificationResponse): string | null {
  return asStr(r.site) ?? asStr(r.answers?.site) ?? null
}

function siteLabel(site: string | null): string {
  if (site === 'hfhotel') return 'HF Hotel'
  if (site === 'hfville') return 'HF Ville'
  if (site === 'all') return 'ทั้งหมด'
  return site ?? '—'
}

function shortLabel(label: string, n = 22): string {
  const s = label.trim()
  return s.length > n ? `${s.slice(0, n)}…` : s
}

/** Tone for an answer value, derived generically (the schema carries no tone). */
function valueTone(v: string): Tone {
  if (['match', 'vacant', 'ready', 'pass', 'yes', 'ok', 'all_match'].includes(v)) return 'ok'
  if (['mismatch', 'occupied', 'fail', 'no', 'issue', 'has_mismatch'].includes(v)) return 'bad'
  if (['notyet', 'pending'].includes(v)) return 'warn'
  return 'mut'
}

// HARDCODED labels for the ORIGINAL full checklist only (not data-driven). Schema
// labels (for rv_*/data-driven questions) take precedence via `labelFor`.
const KEY_LABELS: Record<string, string> = {
  q1_1: '1.1 สถานะห้องพัก',
  q1_1_rooms: '1.1 ห้องที่ไม่ตรง',
  q1_2: '1.2 รายงานรอบบิล',
  q1_3: '1.3 รายชื่อผู้เข้าพัก',
  q1_4: '1.4 ปฏิทินห้องว่าง',
  q1_5: '1.5 ใบกำกับภาษี/ใบเสร็จ',
  q2_1: '2.1 เช็คเอาท์ช้า',
  q2_2: '2.2 ค่ามัดจำ',
  q2_3: '2.3 VAT',
  q3_summary: '3. สรุปยอดบิล 5 ราย',
  q4_result: '4. ผลทดสอบสด',
  q5: '5. ความพร้อมโดยรวม',
}

// Fallback value labels for the original checklist + overall verdict (data-driven
// option labels come from the schema via `optLabelFor`).
const VALUE_LABELS: Record<string, string> = {
  match: 'ตรง',
  mismatch: 'ไม่ตรง',
  vacant: 'ว่างแล้ว',
  occupied: 'ยังมีคนพัก',
  ready: 'พร้อมนัด',
  notyet: 'ยังไม่พร้อม',
  pending: 'ยังไม่ได้ทดสอบ',
  pass: 'ผ่านทั้งหมด',
  issue: 'มีปัญหา',
  all_match: 'ตรงทั้ง 5',
  has_mismatch: 'มีไม่ตรง',
}

// ── visual primitives (replicated from the form's v2 look) ───────────────────

type Tone = 'ok' | 'bad' | 'warn' | 'mut'

const TONE_STYLE: Record<Tone, { color: string; background: string }> = {
  ok: { color: 'var(--v2-ok)', background: 'var(--v2-ok-bg)' },
  bad: { color: '#b91c1c', background: '#fbe9e9' },
  warn: { color: '#b45309', background: '#fdf0d9' },
  mut: { color: 'var(--v2-ink-2)', background: 'var(--v2-surface-2)' },
}

function Tag({ tone = 'mut', children }: { tone?: Tone; children: ReactNode }) {
  return (
    <span
      className="inline-flex items-center px-2 py-0.5 rounded-full text-[11.5px] font-medium whitespace-nowrap"
      style={TONE_STYLE[tone]}
    >
      {children}
    </span>
  )
}

function StatChip({ label, children }: { label: string; children: ReactNode }) {
  return (
    <div className="v2-card px-3.5 py-2.5">
      <div className="text-[11px]" style={{ color: 'var(--v2-ink-3)' }}>
        {label}
      </div>
      <div className="mt-1 text-[13.5px] font-semibold leading-snug">{children}</div>
    </div>
  )
}

function Th({ cols }: { cols: string[] }) {
  return (
    <tr style={{ borderBottom: '1px solid var(--v2-line)' }}>
      {cols.map((c) => (
        <th
          key={c}
          className="px-3 py-2.5 font-semibold whitespace-nowrap text-left"
          style={{ color: 'var(--v2-ink-2)' }}
        >
          {c}
        </th>
      ))}
    </tr>
  )
}

// ── schema-derived helpers (built once per render from the fetched forms) ─────

interface SchemaIndex {
  /** question id → label */
  labelOf: Record<string, string>
  /** question id → { option value → option label } */
  optionOf: Record<string, Record<string, string>>
  /** radio questions, deduped by id, in form order (drives tags + tallies) */
  radios: SchemaQuestion[]
}

function buildSchemaIndex(forms: FeedbackForm[]): SchemaIndex {
  const labelOf: Record<string, string> = {}
  const optionOf: Record<string, Record<string, string>> = {}
  const radios: SchemaQuestion[] = []
  const seen = new Set<string>()
  for (const f of forms) {
    for (const q of f.schema?.questions ?? []) {
      if (!q?.id) continue
      if (!(q.id in labelOf)) labelOf[q.id] = q.label
      if (Array.isArray(q.options)) {
        optionOf[q.id] = optionOf[q.id] ?? {}
        for (const o of q.options) optionOf[q.id][o.value] = o.label
      }
      if (q.type === 'radio' && !seen.has(q.id)) {
        seen.add(q.id)
        radios.push(q)
      }
    }
  }
  return { labelOf, optionOf, radios }
}

const labelFor = (idx: SchemaIndex, key: string): string => idx.labelOf[key] ?? KEY_LABELS[key] ?? key
const optLabelFor = (idx: SchemaIndex, qid: string, value: string): string =>
  idx.optionOf[qid]?.[value] ?? VALUE_LABELS[value] ?? value

// Compact per-row tags for a reverify submission, driven by the schema's radios.
function reverifyTags(idx: SchemaIndex, a: Record<string, unknown>): { tone: Tone; text: string }[] {
  const out: { tone: Tone; text: string }[] = []
  for (const q of idx.radios) {
    const v = asStr(a[q.id])
    if (!v) continue
    out.push({ tone: valueTone(v), text: `${shortLabel(q.label, 14)}: ${optLabelFor(idx, q.id, v)}` })
  }
  return out
}

// ── original full-checklist summary (NOT data-driven) ─────────────────────────

function overallVerdict(overall: string | null): { tone: Tone; text: string } {
  if (overall === 'a') return { tone: 'ok', text: 'ภาพรวม: ดี พร้อมทดลองใช้' }
  if (overall === 'b') return { tone: 'warn', text: 'ภาพรวม: มีจุดเล็กน้อย' }
  if (overall === 'c') return { tone: 'bad', text: 'ภาพรวม: ยังไม่พร้อม' }
  return { tone: 'mut', text: 'ภาพรวม: —' }
}

function fullChecklistTags(r: VerificationResponse): { tone: Tone; text: string }[] {
  const out: { tone: Tone; text: string }[] = [overallVerdict(r.overall)]
  const q15 = asStr(r.answers?.q1_5)
  if (q15)
    out.push({ tone: q15 === 'match' ? 'ok' : 'bad', text: `ใบกำกับภาษี: ${q15 === 'match' ? 'ถูกต้อง' : 'มีจุดผิด'}` })
  const q4 = asStr(r.answers?.q4_result)
  if (q4 && q4 !== 'pending')
    out.push({ tone: q4 === 'pass' ? 'ok' : 'bad', text: `ทดสอบสด: ${VALUE_LABELS[q4] ?? q4}` })
  return out
}

// ── page ─────────────────────────────────────────────────────────────────────

export default function VerificationResults() {
  const [responses, setResponses] = useState<VerificationResponse[] | null>(null)
  const [forms, setForms] = useState<FeedbackForm[]>([])
  const [loading, setLoading] = useState(true)
  const [error, setError] = useState<string | null>(null)
  const [expanded, setExpanded] = useState<Set<number>>(new Set())

  const load = useCallback(async () => {
    setLoading(true)
    setError(null)
    try {
      // Responses drive the page; form schemas drive the labels/tallies. A forms
      // failure degrades gracefully (falls back to KEY_LABELS/VALUE_LABELS).
      const [respRes, formsRes] = await Promise.all([
        fetch('/api/verification?limit=200', { headers: { Accept: 'application/json' } }),
        fetch('/api/feedback/forms', { headers: { Accept: 'application/json' } }).catch(() => null),
      ])
      if (!respRes.ok) throw new Error(`HTTP ${respRes.status}`)
      const body = await respRes.json()
      const list = Array.isArray(body?.responses) ? (body.responses as VerificationResponse[]) : []
      setResponses(list)
      if (formsRes && formsRes.ok) {
        const fb = await formsRes.json().catch(() => null)
        setForms(Array.isArray(fb?.forms) ? (fb.forms as FeedbackForm[]) : [])
      }
    } catch {
      setError('โหลดข้อมูลไม่สำเร็จ กรุณาลองใหม่อีกครั้ง')
    } finally {
      setLoading(false)
    }
  }, [])

  useEffect(() => {
    void load()
  }, [load])

  const toggle = (id: number) =>
    setExpanded((prev) => {
      const next = new Set(prev)
      if (next.has(id)) next.delete(id)
      else next.add(id)
      return next
    })

  const idx = buildSchemaIndex(forms)

  // ── HUB section (always shown) — per-site re-verify status ─────────────────
  // Pending = no non-misclick reverify submission for that site yet → shown
  // OUTSTANDING (amber accent + primary CTA) so it's clear what we're waiting on.
  const SITES: { id: string; label: string }[] = [
    { id: 'hfhotel', label: 'HF Hotel' },
    { id: 'hfville', label: 'HF Ville' },
  ]
  const latestReverify = (site: string): VerificationResponse | null => {
    if (!responses) return null
    const rows = responses
      .filter((r) => isReverify(r) && !isMisclick(r) && rowSite(r) === site)
      .sort((a, b) => (new Date(b.submittedAt).getTime() || 0) - (new Date(a.submittedAt).getTime() || 0))
    return rows[0] ?? null
  }
  const pendingSites = responses ? SITES.filter((s) => !latestReverify(s.id)) : []

  const hub = (
    <section className="v2-card p-4 lg:p-5 space-y-3">
      <div>
        <div className="v2-eyebrow">การตรวจสอบซ้ำ</div>
        <h2 className="text-[16px] font-semibold mt-0.5">สถานะการตรวจซ้ำ แยกตามสาขา</h2>
      </div>

      {responses && pendingSites.length > 0 && (
        <div
          className="flex items-center gap-2 rounded-lg px-3 py-2 text-[13px] font-semibold"
          style={{ color: '#b45309', background: '#fdf0d9' }}
        >
          <AlertTriangle size={15} /> ยังรอผลตรวจซ้ำ {pendingSites.length} สาขา:{' '}
          {pendingSites.map((s) => s.label).join(', ')}
        </div>
      )}

      <div className="space-y-2">
        {SITES.map((s) => {
          const done = latestReverify(s.id)
          const pending = responses != null && !done
          return (
            <div
              key={s.id}
              className="flex flex-wrap items-center justify-between gap-2.5 rounded-lg px-3 py-2.5"
              style={{
                border: '1px solid var(--v2-line)',
                borderLeft: pending ? '3px solid #b45309' : '1px solid var(--v2-line)',
                background: pending ? '#fffdf7' : 'var(--v2-surface)',
              }}
            >
              <div className="flex items-center gap-2.5 min-w-0 flex-wrap">
                <span className="text-[14px] font-semibold">{s.label}</span>
                {responses == null ? (
                  <Tag tone="mut">…</Tag>
                ) : done ? (
                  <Tag tone="ok">
                    ✓ ได้รับแล้ว{done.inspector ? ` · ${done.inspector}` : ''} · {formatThai(done.submittedAt)}
                  </Tag>
                ) : (
                  <Tag tone="warn">⏳ รอตรวจสอบ</Tag>
                )}
              </div>
              <Link
                href={`/v2/verification/reverify?site=${s.id}`}
                className={`v2-btn ${pending ? 'v2-btn-primary' : 'v2-btn-soft'} v2-btn-sm`}
              >
                <RefreshCcw size={15} /> {done ? 'ตรวจซ้ำอีกครั้ง' : 'เปิดฟอร์มตรวจซ้ำ'}
              </Link>
            </div>
          )
        })}
      </div>

      <div className="pt-1">
        <Link href="/v2/verification" className="v2-btn v2-btn-soft v2-btn-sm">
          <ClipboardList size={15} /> แบบฟอร์มตรวจสอบฉบับเต็ม (เดิม)
        </Link>
      </div>
    </section>
  )

  // ── derived tallies (only when data present) ───────────────────────────────
  let summary: ReactNode = null
  let table: ReactNode = null

  if (responses) {
    const total = responses.length
    const bySite = (s: string) => responses.filter((r) => rowSite(r) === s).length
    const reverifyAll = responses.filter(isReverify)
    const fullAll = responses.filter((r) => !isReverify(r))

    // per-item tallies exclude misclick rows so the stats stay clean
    const rv = reverifyAll.filter((r) => !isMisclick(r))

    // Schema-driven: one tally per radio question, counting each option's answers.
    const questionTallies = idx.radios.map((q) => {
      const counts: { value: string; label: string; n: number }[] = (q.options ?? []).map((o) => ({
        value: o.value,
        label: o.label,
        n: 0,
      }))
      const byValue = new Map(counts.map((c) => [c.value, c]))
      for (const r of rv) {
        const v = asStr(r.answers?.[q.id])
        if (v && byValue.has(v)) byValue.get(v)!.n += 1
      }
      const answered = counts.reduce((s, c) => s + c.n, 0)
      return { q, counts, answered }
    })
    const hasReverifyData = questionTallies.some((t) => t.answered > 0)

    summary = (
      <section className="space-y-3">
        <div>
          <div className="v2-eyebrow">สรุปผล</div>
          <h2 className="text-[16px] font-semibold mt-0.5">ภาพรวมการตรวจสอบ</h2>
        </div>

        {/* totals */}
        <div className="grid gap-2.5 grid-cols-2 sm:grid-cols-3 lg:grid-cols-5">
          <StatChip label="ทั้งหมด">
            <span className="v2-num">{total}</span> รายการ
          </StatChip>
          <StatChip label="HF Hotel">
            <span className="v2-num">{bySite('hfhotel')}</span>
          </StatChip>
          <StatChip label="HF Ville">
            <span className="v2-num">{bySite('hfville')}</span>
          </StatChip>
          <StatChip label="ตรวจซ้ำ">
            <span className="v2-num">{reverifyAll.length}</span>
          </StatChip>
          <StatChip label="ตรวจฉบับเต็ม">
            <span className="v2-num">{fullAll.length}</span>
          </StatChip>
        </div>

        {/* reverify per-question tallies (schema-driven) */}
        {hasReverifyData ? (
          <div className="grid gap-2.5 grid-cols-1 sm:grid-cols-2 lg:grid-cols-3">
            {questionTallies
              .filter((t) => t.answered > 0)
              .map((t) => (
                <StatChip key={t.q.id} label={shortLabel(t.q.label, 30)}>
                  <span className="flex items-center gap-1.5 flex-wrap">
                    {t.counts.map((c) => (
                      <Tag key={c.value} tone={valueTone(c.value)}>
                        {c.label} <span className="v2-num ml-1">{c.n}</span>
                      </Tag>
                    ))}
                  </span>
                </StatChip>
              ))}
          </div>
        ) : (
          <p className="text-[12.5px]" style={{ color: 'var(--v2-ink-3)' }}>
            ยังไม่มีข้อมูลแบบฟอร์มตรวจสอบซ้ำ — เมื่อมีการส่งแล้วจะสรุปผลรายข้อที่นี่
          </p>
        )}
      </section>
    )

    // ── table (most-recent first) ────────────────────────────────────────────
    const sorted = [...responses].sort((a, b) => {
      const ta = new Date(a.submittedAt).getTime() || 0
      const tb = new Date(b.submittedAt).getTime() || 0
      if (tb !== ta) return tb - ta
      return b.id - a.id
    })

    table = (
      <section className="space-y-3">
        <div className="flex items-end justify-between gap-3">
          <div>
            <div className="v2-eyebrow">รายการ</div>
            <h2 className="text-[16px] font-semibold mt-0.5">การส่งทั้งหมด</h2>
          </div>
          <button type="button" onClick={() => void load()} className="v2-btn v2-btn-soft v2-btn-sm">
            <RefreshCcw size={14} /> รีเฟรช
          </button>
        </div>

        {sorted.length === 0 ? (
          <div className="v2-card">
            <V2Empty title="ยังไม่มีการส่งแบบฟอร์ม" hint="เมื่อมีผู้ตรวจส่งผลแล้วจะแสดงที่นี่" />
          </div>
        ) : (
          <div className="v2-card overflow-x-auto">
            <table className="w-full text-[13px] border-collapse">
              <thead>
                <Th cols={['', 'วันที่/เวลา', 'สาขา', 'ประเภท', 'ผู้ตรวจ', 'สรุปคำตอบ']} />
              </thead>
              <tbody>
                {sorted.map((r) => {
                  const open = expanded.has(r.id)
                  const misclick = isMisclick(r)
                  const reverify = isReverify(r)
                  const tags = reverify ? reverifyTags(idx, r.answers ?? {}) : fullChecklistTags(r)
                  return (
                    <Fragment key={r.id}>
                      <tr
                        onClick={() => toggle(r.id)}
                        className="cursor-pointer"
                        style={{
                          borderBottom: '1px solid var(--v2-line)',
                          opacity: misclick ? 0.45 : 1,
                        }}
                      >
                        <td className="px-3 py-2.5 align-top" style={{ color: 'var(--v2-ink-3)' }}>
                          {open ? <ChevronDown size={15} /> : <ChevronRight size={15} />}
                        </td>
                        <td className="px-3 py-2.5 align-top whitespace-nowrap v2-num">
                          {formatThai(r.submittedAt)}
                        </td>
                        <td className="px-3 py-2.5 align-top whitespace-nowrap">{siteLabel(rowSite(r))}</td>
                        <td className="px-3 py-2.5 align-top whitespace-nowrap">
                          <Tag tone="mut">{reverify ? 'ตรวจซ้ำ' : 'ตรวจฉบับเต็ม'}</Tag>
                        </td>
                        <td className="px-3 py-2.5 align-top whitespace-nowrap">
                          {misclick && (
                            <span className="mr-1 text-[10px]" style={{ color: 'var(--v2-ink-3)' }}>
                              [ทดสอบ]
                            </span>
                          )}
                          {r.inspector?.trim() || '—'}
                        </td>
                        <td className="px-3 py-2.5 align-top">
                          <div className="flex flex-wrap gap-1.5">
                            {tags.map((t, i) => (
                              <Tag key={i} tone={t.tone}>
                                {t.text}
                              </Tag>
                            ))}
                          </div>
                        </td>
                      </tr>
                      {open && (
                        <tr style={{ borderBottom: '1px solid var(--v2-line)' }}>
                          <td colSpan={6} className="px-3 pb-4 pt-1" style={{ background: 'var(--v2-surface-2)' }}>
                            <AnswerDetail idx={idx} answers={r.answers ?? {}} overall={r.overall} id={r.id} />
                          </td>
                        </tr>
                      )}
                    </Fragment>
                  )
                })}
              </tbody>
            </table>
          </div>
        )}
      </section>
    )
  }

  return (
    <div className="space-y-5 pb-8">
      <V2PageHeader
        eyebrow="ตรวจสอบระบบ"
        title="ผลการตรวจสอบ / ศูนย์รวม"
        right={
          <span className="hidden sm:inline-flex items-center gap-1.5 text-[12px]" style={{ color: 'var(--v2-ink-3)' }}>
            <ListChecks size={15} /> สรุป + ประวัติการส่ง
          </span>
        }
      />

      {hub}

      {loading && (
        <div className="v2-card">
          <V2Spinner label="กำลังโหลดผลการตรวจสอบ…" />
        </div>
      )}

      {!loading && error && (
        <div className="v2-card p-6 flex flex-col items-center text-center gap-3">
          <AlertTriangle size={32} style={{ color: 'var(--v2-dep)' }} />
          <p className="text-[14px] font-medium">{error}</p>
          <button type="button" onClick={() => void load()} className="v2-btn v2-btn-soft v2-btn-sm">
            <RefreshCcw size={14} /> ลองใหม่
          </button>
        </div>
      )}

      {!loading && !error && (
        <>
          {summary}
          {table}
        </>
      )}
    </div>
  )
}

// ── expanded raw answer detail (schema-aware labels) ─────────────────────────

function AnswerDetail({
  idx,
  answers,
  overall,
  id,
}: {
  idx: SchemaIndex
  answers: Record<string, unknown>
  overall: string | null
  id: number
}) {
  const entries = Object.entries(answers).filter(([k]) => k !== 'kind' && k !== 'site')
  return (
    <div className="space-y-2">
      <div className="flex items-center gap-2 text-[11px]" style={{ color: 'var(--v2-ink-3)' }}>
        <ClipboardCheck size={13} /> รายละเอียดคำตอบ (อ้างอิง #{id})
      </div>
      {entries.length === 0 ? (
        <p className="text-[12.5px]" style={{ color: 'var(--v2-ink-3)' }}>
          ไม่มีรายละเอียดเพิ่มเติม
        </p>
      ) : (
        <dl className="grid gap-x-6 gap-y-1.5 sm:grid-cols-2">
          {entries.map(([k, v]) => (
            <div key={k} className="flex gap-2 text-[12.5px]">
              <dt className="shrink-0 font-medium" style={{ color: 'var(--v2-ink-2)' }}>
                {labelFor(idx, k)}:
              </dt>
              <dd className="min-w-0 break-words">{renderValue(idx, k, v)}</dd>
            </div>
          ))}
        </dl>
      )}
      {overall && (
        <p className="text-[12px]" style={{ color: 'var(--v2-ink-3)' }}>
          overall: <span className="font-medium">{VALUE_LABELS[overall] ?? overall}</span>
        </p>
      )}
    </div>
  )
}

function renderValue(idx: SchemaIndex, key: string, v: unknown): ReactNode {
  if (v === null || v === undefined || v === '') return <span style={{ color: 'var(--v2-ink-3)' }}>—</span>
  if (typeof v === 'string') return optLabelFor(idx, key, v)
  if (typeof v === 'number' || typeof v === 'boolean') return String(v)
  if (Array.isArray(v)) {
    if (v.length === 0) return <span style={{ color: 'var(--v2-ink-3)' }}>—</span>
    return v.map((x) => (typeof x === 'string' ? VALUE_LABELS[x] ?? x : JSON.stringify(x))).join(', ')
  }
  // arbitrary nested object — render as compact JSON, never crash
  try {
    return <code className="text-[11.5px]">{JSON.stringify(v)}</code>
  } catch {
    return <span style={{ color: 'var(--v2-ink-3)' }}>—</span>
  }
}
