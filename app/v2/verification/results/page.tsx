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
 * /v2/verification and the lightweight re-verify forms at
 * /v2/verification/reverify). Read-only: it GETs every submission from
 * /api/verification and renders (1) hub links to the forms, (2) client-side
 * summary tallies, and (3) a table of all submissions. PG-canonical only — no
 * writes here.
 *
 * The `answers` JSONB is an arbitrary object, so EVERY key is read defensively
 * with `asStr()` (returns undefined for missing / non-string values). Reverify
 * submissions carry `answers.kind === 'reverify'`; the original full checklist
 * has no `kind` and is summarised from its `overall` (= q5) verdict.
 *
 * vr_id 1 is a known misclick test row (inspector starts with '[misclick]') —
 * such rows are visually de-emphasised in the table AND excluded from the
 * reverify per-item tallies so they don't skew the stats.
 */

// ── data shape ─────────────────────────────────────────────────────────────

interface VerificationResponse {
  id: number
  submittedAt: string
  site: string | null
  inspector: string | null
  answers: Record<string, unknown>
  overall: string | null
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

const matchLabel = (v?: string): string | undefined =>
  v === 'match' ? 'ตรง' : v === 'mismatch' ? 'ไม่ตรง' : undefined

// known answer-key labels (for the expanded raw detail view) ─────────────────
const KEY_LABELS: Record<string, string> = {
  kind: 'ประเภท',
  site: 'สาขา',
  rv_invoice: 'บิล INV2606-019832',
  rv_invoice_note: 'หมายเหตุบิล',
  rv_round_summary: 'สรุปรอบบิล',
  rv_round_summary_note: 'หมายเหตุสรุปรอบบิล',
  rv_round816: 'รอบบิล 816',
  rv_round816_note: 'หมายเหตุรอบ 816',
  rv_room114: 'สถานะห้อง 114',
  rv_arrivals: 'ความหมายของคำว่า "เข้า"',
  rv_arrivals_screen: 'หน้าจอ iHOTEL ที่ดู',
  rv_livetest: 'พร้อมทดสอบสด',
  rv_livetest_slot: 'วัน/เวลาที่สะดวก',
  rv_note: 'หมายเหตุเพิ่มเติม',
  // original full checklist
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

// ── compact inline summary builders ──────────────────────────────────────────

function reverifyTags(a: Record<string, unknown>): { tone: Tone; text: string }[] {
  const out: { tone: Tone; text: string }[] = []
  const inv = asStr(a.rv_invoice)
  if (inv) out.push({ tone: inv === 'match' ? 'ok' : 'bad', text: `บิล: ${matchLabel(inv)}` })
  const rs = asStr(a.rv_round_summary)
  if (rs) out.push({ tone: rs === 'match' ? 'ok' : 'bad', text: `สรุปรอบบิล: ${matchLabel(rs)}` })
  const r816 = asStr(a.rv_round816)
  if (r816) out.push({ tone: r816 === 'match' ? 'ok' : 'bad', text: `รอบ 816: ${matchLabel(r816)}` })
  const rm = asStr(a.rv_room114)
  if (rm)
    out.push({
      tone: rm === 'vacant' ? 'ok' : 'warn',
      text: `ห้อง 114: ${rm === 'vacant' ? 'ว่างแล้ว' : 'ยังมีคนพัก'}`,
    })
  const arr = asStr(a.rv_arrivals)
  if (arr)
    out.push({
      tone: 'mut',
      text: `เข้า = ${arr === 'a' ? 'ก (เข้าจริงวันนี้)' : arr === 'b' ? 'ข (รอเช็คอิน)' : arr}`,
    })
  const lt = asStr(a.rv_livetest)
  if (lt)
    out.push({
      tone: lt === 'ready' ? 'ok' : 'warn',
      text: `ทดสอบสด: ${lt === 'ready' ? 'พร้อมนัด' : 'ยังไม่พร้อม'}`,
    })
  return out
}

/** Original full-checklist verdict from the top-level `overall` (= q5). */
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

// ── tally widget ─────────────────────────────────────────────────────────────

function TwoCount({
  aLabel,
  a,
  aTone,
  bLabel,
  b,
  bTone,
}: {
  aLabel: string
  a: number
  aTone: Tone
  bLabel: string
  b: number
  bTone: Tone
}) {
  return (
    <span className="flex items-center gap-1.5 flex-wrap">
      <Tag tone={aTone}>
        {aLabel} <span className="v2-num ml-1">{a}</span>
      </Tag>
      <Tag tone={bTone}>
        {bLabel} <span className="v2-num ml-1">{b}</span>
      </Tag>
    </span>
  )
}

// ── page ─────────────────────────────────────────────────────────────────────

export default function VerificationResults() {
  const [responses, setResponses] = useState<VerificationResponse[] | null>(null)
  const [loading, setLoading] = useState(true)
  const [error, setError] = useState<string | null>(null)
  const [expanded, setExpanded] = useState<Set<number>>(new Set())

  const load = useCallback(async () => {
    setLoading(true)
    setError(null)
    try {
      const res = await fetch('/api/verification?limit=200', { headers: { Accept: 'application/json' } })
      if (!res.ok) throw new Error(`HTTP ${res.status}`)
      const body = await res.json()
      const list = Array.isArray(body?.responses) ? (body.responses as VerificationResponse[]) : []
      setResponses(list)
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
    const tally = (key: string, av: string, bv: string) => {
      let a = 0
      let b = 0
      for (const r of rv) {
        const v = asStr(r.answers?.[key])
        if (v === av) a++
        else if (v === bv) b++
      }
      return { a, b }
    }
    const inv = tally('rv_invoice', 'match', 'mismatch')
    const roundSummary = tally('rv_round_summary', 'match', 'mismatch')
    const r816 = tally('rv_round816', 'match', 'mismatch')
    const room114 = tally('rv_room114', 'vacant', 'occupied')
    const arrivals = tally('rv_arrivals', 'a', 'b')
    const livetest = tally('rv_livetest', 'ready', 'notyet')

    const hasReverifyData =
      inv.a + inv.b + roundSummary.a + roundSummary.b + r816.a + r816.b + room114.a + room114.b + arrivals.a + arrivals.b + livetest.a + livetest.b >
      0

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

        {/* reverify per-item tallies */}
        {hasReverifyData ? (
          <div className="grid gap-2.5 grid-cols-1 sm:grid-cols-2 lg:grid-cols-3">
            <StatChip label="บิล INV2606-019832">
              <TwoCount aLabel="ตรง" a={inv.a} aTone="ok" bLabel="ไม่ตรง" b={inv.b} bTone="bad" />
            </StatChip>
            <StatChip label="สรุปรอบบิล (HF Hotel)">
              <TwoCount aLabel="ตรง" a={roundSummary.a} aTone="ok" bLabel="ไม่ตรง" b={roundSummary.b} bTone="bad" />
            </StatChip>
            <StatChip label="รอบบิล 816 (HF Ville)">
              <TwoCount aLabel="ตรง" a={r816.a} aTone="ok" bLabel="ไม่ตรง" b={r816.b} bTone="bad" />
            </StatChip>
            <StatChip label="ห้อง 114">
              <TwoCount aLabel="ว่างแล้ว" a={room114.a} aTone="ok" bLabel="ยังมีคนพัก" b={room114.b} bTone="warn" />
            </StatChip>
            <StatChip label='ความหมาย "เข้า"'>
              <TwoCount aLabel="ก เข้าจริง" a={arrivals.a} aTone="mut" bLabel="ข รอเช็คอิน" b={arrivals.b} bTone="mut" />
            </StatChip>
            <StatChip label="พร้อมทดสอบสด">
              <TwoCount aLabel="พร้อมนัด" a={livetest.a} aTone="ok" bLabel="ยังไม่พร้อม" b={livetest.b} bTone="warn" />
            </StatChip>
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
          <button
            type="button"
            onClick={() => void load()}
            className="v2-btn v2-btn-soft v2-btn-sm"
          >
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
                  const tags = reverify ? reverifyTags(r.answers ?? {}) : fullChecklistTags(r)
                  const note = asStr(r.answers?.rv_note) ?? asStr(r.answers?.rv_livetest_slot)
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
                            {note && (
                              <span className="text-[11.5px]" style={{ color: 'var(--v2-ink-3)' }}>
                                · {note}
                              </span>
                            )}
                          </div>
                        </td>
                      </tr>
                      {open && (
                        <tr style={{ borderBottom: '1px solid var(--v2-line)' }}>
                          <td colSpan={6} className="px-3 pb-4 pt-1" style={{ background: 'var(--v2-surface-2)' }}>
                            <AnswerDetail answers={r.answers ?? {}} overall={r.overall} id={r.id} />
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

// ── expanded raw answer detail ───────────────────────────────────────────────

function AnswerDetail({
  answers,
  overall,
  id,
}: {
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
                {KEY_LABELS[k] ?? k}:
              </dt>
              <dd className="min-w-0 break-words">{renderValue(v)}</dd>
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

function renderValue(v: unknown): ReactNode {
  if (v === null || v === undefined || v === '') return <span style={{ color: 'var(--v2-ink-3)' }}>—</span>
  if (typeof v === 'string') return VALUE_LABELS[v] ?? v
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
