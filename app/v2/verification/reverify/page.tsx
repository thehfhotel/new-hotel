'use client'

import { Suspense, useState, type ReactNode } from 'react'
import { useSearchParams } from 'next/navigation'
import { CheckCircle2, Loader2, Send, ClipboardCheck, ArrowLeft } from 'lucide-react'
import { useBranch } from '@/contexts/BranchContext'
import { useAuth } from '@/contexts/AuthContext'
import { V2PageHeader } from '@/components/v2/primitives'

/**
 * Per-site RE-VERIFICATION form (ตรวจสอบซ้ำ) — the focused follow-up to the
 * full #58 checklist (`app/v2/verification/page.tsx`). Each item here was a
 * point that IT already fixed; reception just re-confirms it now reads correctly
 * in the new system vs iHOTEL.
 *
 * The target site comes from `?site=hfhotel|hfville` (so IT can hand a receptionist
 * a direct link), falling back to the globally-selected branch. We render ONLY the
 * active site's re-verify items plus the shared live-test / note / inspector, then
 * POST to /api/verification with answers tagged `{ kind:'reverify', site }`.
 *
 * Plain fetch (NOT branchFetch): verification is internal feedback stored in the
 * primary DB tagged by `site`, so we must NOT append ?branch=hfville (that would
 * trip ville_write_guard). The form submits fine from any selected branch.
 */

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
  options: { value: string; label: string }[]
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

function Section({ title, hint, children }: { title: string; hint?: string; children: ReactNode }) {
  return (
    <section className="v2-card p-4 lg:p-5 space-y-4">
      <div>
        <h2 className="text-[16px] font-semibold">{title}</h2>
        {hint && (
          <p className="text-[12px] mt-1" style={{ color: 'var(--v2-ink-3)' }}>
            {hint}
          </p>
        )}
      </div>
      {children}
    </section>
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

// ── option dictionaries (verbatim Thai) ────────────────────────────────────

const MATCH = [
  { value: 'match', label: 'ตรง' },
  { value: 'mismatch', label: 'ไม่ตรง' },
]
const ROOM114 = [
  { value: 'vacant', label: 'ว่างแล้ว' },
  { value: 'occupied', label: 'ยังมีคนพัก' },
]
const ARRIVALS = [
  { value: 'a', label: 'ลูกค้าที่เช็คอินเข้าจริงวันนี้' },
  { value: 'b', label: 'ลูกค้าที่จองไว้และยังรอเช็คอินวันนี้ (ยังไม่เข้า)' },
]
const LIVETEST = [
  { value: 'ready', label: 'พร้อมนัด' },
  { value: 'notyet', label: 'ยังไม่พร้อม' },
]

type Site = 'hfhotel' | 'hfville'

interface FormState {
  inspector: string
  // HF Hotel
  rv_invoice: string
  rv_invoice_note: string
  rv_round_summary: string
  rv_round_summary_note: string
  // HF Ville
  rv_round816: string
  rv_round816_note: string
  rv_room114: string
  rv_arrivals: string
  rv_arrivals_screen: string
  // both sites
  rv_livetest: string
  rv_livetest_slot: string
  rv_note: string
}

const EMPTY_FORM: FormState = {
  inspector: '',
  rv_invoice: '',
  rv_invoice_note: '',
  rv_round_summary: '',
  rv_round_summary_note: '',
  rv_round816: '',
  rv_round816_note: '',
  rv_room114: '',
  rv_arrivals: '',
  rv_arrivals_screen: '',
  rv_livetest: '',
  rv_livetest_slot: '',
  rv_note: '',
}

export default function V2Reverify() {
  return (
    <Suspense fallback={<div className="v2-card p-8 flex justify-center"><Loader2 size={22} className="animate-spin" style={{ color: 'var(--v2-ink-3)' }} /></div>}>
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

  const [form, setForm] = useState<FormState>(EMPTY_FORM)
  const [submitting, setSubmitting] = useState(false)
  const [error, setError] = useState<string | null>(null)
  const [submittedId, setSubmittedId] = useState<number | null>(null)

  const set = <K extends keyof FormState>(key: K, value: FormState[K]) =>
    setForm((f) => ({ ...f, [key]: value }))

  // Only the active site's primary radio decisions gate submit.
  const primaryComplete =
    site === 'hfhotel'
      ? !!form.rv_invoice && !!form.rv_round_summary
      : !!form.rv_round816 && !!form.rv_room114 && !!form.rv_arrivals

  const buildAnswers = (): Record<string, string> => {
    const a: Record<string, string> = { kind: 'reverify', site }
    if (site === 'hfhotel') {
      a.rv_invoice = form.rv_invoice
      if (form.rv_invoice === 'mismatch' && form.rv_invoice_note.trim())
        a.rv_invoice_note = form.rv_invoice_note.trim()
      a.rv_round_summary = form.rv_round_summary
      if (form.rv_round_summary === 'mismatch' && form.rv_round_summary_note.trim())
        a.rv_round_summary_note = form.rv_round_summary_note.trim()
    } else {
      a.rv_round816 = form.rv_round816
      if (form.rv_round816 === 'mismatch' && form.rv_round816_note.trim())
        a.rv_round816_note = form.rv_round816_note.trim()
      a.rv_room114 = form.rv_room114
      a.rv_arrivals = form.rv_arrivals
      if (form.rv_arrivals_screen.trim()) a.rv_arrivals_screen = form.rv_arrivals_screen.trim()
    }
    // shared keys (omit empties, consistent with the other optional keys)
    if (form.rv_livetest) a.rv_livetest = form.rv_livetest
    if (form.rv_livetest_slot.trim()) a.rv_livetest_slot = form.rv_livetest_slot.trim()
    if (form.rv_note.trim()) a.rv_note = form.rv_note.trim()
    return a
  }

  const submit = async (e: React.FormEvent) => {
    e.preventDefault()
    setError(null)
    if (!primaryComplete) {
      setError('กรุณาตอบข้อหลักของสาขานี้ให้ครบก่อนส่งนะครับ/ค่ะ 🙏')
      return
    }
    setSubmitting(true)
    try {
      const res = await fetch('/api/verification', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({
          inspector: form.inspector.trim() || user?.username || null,
          answers: buildAnswers(),
          overall: form.rv_livetest || null,
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
      setForm(EMPTY_FORM)
      window.scrollTo({ top: 0, behavior: 'smooth' })
    } catch {
      setError('เกิดข้อผิดพลาดในการเชื่อมต่อ')
    } finally {
      setSubmitting(false)
    }
  }

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
            <button
              type="button"
              onClick={() => setSubmittedId(null)}
              className="v2-btn v2-btn-soft v2-btn-sm"
            >
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

  return (
    <form onSubmit={submit} className="space-y-5 pb-8">
      <V2PageHeader eyebrow="ตรวจสอบซ้ำ" title={`ยืนยันการแก้ไข — ${siteLabel}`} />

      <p className="text-[13px] -mt-2" style={{ color: 'var(--v2-ink-3)' }}>
        แต่ละข้อด้านล่างคือจุดที่ทีมไอทีแก้ไขแล้ว รบกวนช่วยเปิดดูในระบบใหม่อีกครั้งว่าตรงกับ iHOTEL แล้วหรือยัง
        แล้วเลือกคำตอบ — เฉพาะรายการของสาขา <b>{siteLabel}</b> เท่านั้น
      </p>

      {/* ผู้ตรวจ */}
      <div className="v2-card p-4">
        <label className="flex flex-col gap-1 max-w-sm">
          <span className="text-[11px] font-medium" style={{ color: 'var(--v2-ink-3)' }}>
            ผู้ตรวจ {user?.username ? `(ค่าเริ่มต้น: ${user.username})` : ''}
          </span>
          <TextInput value={form.inspector} onChange={(v) => set('inspector', v)} placeholder="ชื่อผู้ตรวจ" />
        </label>
      </div>

      {/* ── HF Hotel items ── */}
      {site === 'hfhotel' && (
        <Section title="รายการตรวจสอบซ้ำ — HF Hotel">
          <Question title="เปิดบิล INV2606-019832 ในระบบใหม่ — แสดงแยกเป็น 2 ห้อง ห้องละ 1,780 รวม 3,560 (เท่ากับบิล iHOTEL 2 ใบรวมกัน) หรือไม่?">
            <RadioGroup name="rv_invoice" value={form.rv_invoice} onChange={(v) => set('rv_invoice', v)} options={MATCH} />
            {form.rv_invoice === 'mismatch' && (
              <TextInput value={form.rv_invoice_note} onChange={(v) => set('rv_invoice_note', v)} placeholder="เห็นเป็นยอดเท่าไร" />
            )}
          </Question>

          <Question title="เปิดหน้า 'สรุปรอบบิล / รายการรอบ' — ยอด (โอน/รวม) ตรงกับ iHOTEL แล้วหรือไม่?">
            <RadioGroup
              name="rv_round_summary"
              value={form.rv_round_summary}
              onChange={(v) => set('rv_round_summary', v)}
              options={MATCH}
            />
            {form.rv_round_summary === 'mismatch' && (
              <TextInput
                value={form.rv_round_summary_note}
                onChange={(v) => set('rv_round_summary_note', v)}
                placeholder="เห็นเป็นยอดเท่าไร"
              />
            )}
          </Question>
        </Section>
      )}

      {/* ── HF Ville items ── */}
      {site === 'hfville' && (
        <Section title="รายการตรวจสอบซ้ำ — HF Ville">
          <Question title="รายงานรอบบิล รอบ 816 (กะบ่าย 27/06) ยอดรวม = 14,280 หรือไม่?">
            <RadioGroup name="rv_round816" value={form.rv_round816} onChange={(v) => set('rv_round816', v)} options={MATCH} />
            {form.rv_round816 === 'mismatch' && (
              <TextInput value={form.rv_round816_note} onChange={(v) => set('rv_round816_note', v)} placeholder="เห็นเป็นยอดเท่าไร" />
            )}
          </Question>

          <Question title="สถานะห้อง 114 ขึ้นว่า 'ว่าง' แล้วหรือไม่?">
            <RadioGroup name="rv_room114" value={form.rv_room114} onChange={(v) => set('rv_room114', v)} options={ROOM114} />
          </Question>

          <Question title="คำว่า 'เข้า' ในรายชื่อผู้เข้าพัก (ที่เคยแจ้งว่าไม่ตรง) หมายถึงข้อใด?">
            <RadioGroup name="rv_arrivals" value={form.rv_arrivals} onChange={(v) => set('rv_arrivals', v)} options={ARRIVALS} />
            <TextInput
              value={form.rv_arrivals_screen}
              onChange={(v) => set('rv_arrivals_screen', v)}
              placeholder="ดูจากหน้าจอไหนของ iHOTEL"
            />
          </Question>
        </Section>
      )}

      {/* ── shared: live test + note ── */}
      <Section title="ทดสอบสด + หมายเหตุ (ทั้งสองสาขา)">
        <Question title="พร้อมนัด 'ทดสอบสด' 1 รอบ (เช็คอิน→รับเงิน→เช็คเอาท์ และเปิด/ปิดรอบบิล) หรือยัง?">
          <RadioGroup name="rv_livetest" value={form.rv_livetest} onChange={(v) => set('rv_livetest', v)} options={LIVETEST} />
          <TextInput value={form.rv_livetest_slot} onChange={(v) => set('rv_livetest_slot', v)} placeholder="สะดวกวัน/เวลาไหน" />
        </Question>

        <Question title="หมายเหตุเพิ่มเติม (ถ้ามี)">
          <TextInput value={form.rv_note} onChange={(v) => set('rv_note', v)} placeholder="หมายเหตุเพิ่มเติม" />
        </Question>
      </Section>

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
        {!primaryComplete ? (
          <span className="text-[12px]" style={{ color: 'var(--v2-ink-3)' }}>
            กรุณาตอบข้อหลักของสาขา {siteLabel} ให้ครบก่อนส่ง
          </span>
        ) : (
          <span className="text-[12px]" style={{ color: 'var(--v2-ink-3)' }}>
            คำตอบจะถูกบันทึกในระบบให้ทีมไอทีตรวจสอบ
          </span>
        )}
      </div>
    </form>
  )
}
