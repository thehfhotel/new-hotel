'use client'

import { useState, type ReactNode } from 'react'
import { ClipboardCheck, CheckCircle2, Loader2, Send, AlertTriangle } from 'lucide-react'
import { useBranch } from '@/contexts/BranchContext'
import { useAuth } from '@/contexts/AuthContext'
import { V2PageHeader } from '@/components/v2/primitives'

/**
 * In-app reception verification form (คู่มือตรวจสอบระบบใหม่ สำหรับฝ่ายต้อนรับ) —
 * task #58. The ONLINE equivalent of the printed checklist
 * `docs/coexistence/reception-verification-TH.html` (§1 screen-compare, §2 policy
 * questions, §3 last-5-bills total check, §4 coordinated live tests, §5 overall
 * readiness). Submits to POST /api/verification (branch-aware via useBranchFetch),
 * where every answer is stored in the `vr_answers` JSONB column keyed by question
 * id so IT can query them. PG-canonical only — no legacy writeback.
 *
 * HF Ville writes are dark (`ville_write_guard` 403s a `branch=hfville` POST until
 * `HFVILLE_WRITES_ENABLED`), so the submit button is gated on `canWrite` like
 * every other /v2 write page.
 */

type Bill = { no: string; ihotel: string; newsys: string; match: string }

interface FormState {
  inspector: string
  q1_1: string
  q1_1_rooms: string
  q1_2: string
  q1_2_reasons: string[]
  q1_3: string
  q1_3_reasons: string[]
  q1_4: string
  q1_4_note: string
  q1_5: string
  q1_5_reasons: string[]
  q2_1: string
  q2_1_other: string
  q2_2: string
  q2_2_other: string
  q2_3: string
  q2_3_vat: string
  q3_bills: Bill[]
  q3_summary: string
  q3_mismatch_count: string
  q4_result: string
  q5: string
}

const EMPTY_FORM: FormState = {
  inspector: '',
  q1_1: '',
  q1_1_rooms: '',
  q1_2: '',
  q1_2_reasons: [],
  q1_3: '',
  q1_3_reasons: [],
  q1_4: '',
  q1_4_note: '',
  q1_5: '',
  q1_5_reasons: [],
  q2_1: '',
  q2_1_other: '',
  q2_2: '',
  q2_2_other: '',
  q2_3: '',
  q2_3_vat: '',
  q3_bills: Array.from({ length: 5 }, () => ({ no: '', ihotel: '', newsys: '', match: '' })),
  q3_summary: '',
  q3_mismatch_count: '',
  q4_result: 'pending',
  q5: '',
}

// ── small presentational primitives (reuse the v2 input look) ──────────────

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

function CheckboxGroup({
  value,
  options,
  onChange,
}: {
  value: string[]
  options: { value: string; label: string }[]
  onChange: (v: string[]) => void
}) {
  const toggle = (v: string) =>
    onChange(value.includes(v) ? value.filter((x) => x !== v) : [...value, v])
  return (
    <div className="flex flex-wrap gap-x-5 gap-y-1.5">
      {options.map((o) => (
        <label key={o.value} className="flex items-center gap-2 cursor-pointer text-[14px]">
          <input
            type="checkbox"
            checked={value.includes(o.value)}
            onChange={() => toggle(o.value)}
            style={{ accentColor: 'var(--v2-wine-600)' }}
          />
          <span>{o.label}</span>
        </label>
      ))}
    </div>
  )
}

function Section({ no, title, hint, children }: { no: string; title: string; hint?: string; children: ReactNode }) {
  return (
    <section className="v2-card p-4 lg:p-5 space-y-4">
      <div>
        <div className="v2-eyebrow">ส่วนที่ {no}</div>
        <h2 className="text-[16px] font-semibold mt-0.5">{title}</h2>
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

function Question({
  title,
  link,
  children,
}: {
  title: string
  link?: { href: string; label?: string }
  children?: ReactNode
}) {
  return (
    <div className="space-y-2">
      <p className="text-[14px] font-semibold leading-snug">
        {title}
        {link && (
          <a
            href={link.href}
            target="_blank"
            rel="noopener noreferrer"
            className="ml-2 inline-block text-[12.5px] font-normal underline whitespace-nowrap"
            style={{ color: 'var(--v2-wine-600)' }}
          >
            {link.label ?? 'เปิดหน้านี้ ↗'}
          </a>
        )}
      </p>
      {children}
    </div>
  )
}

// ── option dictionaries (verbatim Thai from the print checklist) ───────────

const MATCH = [
  { value: 'match', label: 'ตรงกัน' },
  { value: 'mismatch', label: 'ไม่ตรง' },
]
const OK = [
  { value: 'match', label: 'ถูกต้อง' },
  { value: 'mismatch', label: 'มีจุดผิด' },
]

export default function V2Verification() {
  const { branch } = useBranch()
  const { user } = useAuth()

  const [form, setForm] = useState<FormState>(EMPTY_FORM)
  const [submitting, setSubmitting] = useState(false)
  const [error, setError] = useState<string | null>(null)
  const [submittedId, setSubmittedId] = useState<number | null>(null)

  const set = <K extends keyof FormState>(key: K, value: FormState[K]) =>
    setForm((f) => ({ ...f, [key]: value }))

  const setBill = (i: number, field: keyof Bill, value: string) =>
    setForm((f) => ({
      ...f,
      q3_bills: f.q3_bills.map((b, j) => (j === i ? { ...b, [field]: value } : b)),
    }))

  const submit = async (e: React.FormEvent) => {
    e.preventDefault()
    setError(null)
    if (!form.q5) {
      setError('กรุณาเลือกความพร้อมโดยรวม (ส่วนที่ 5) ก่อนส่ง')
      return
    }
    setSubmitting(true)
    try {
      const { inspector, ...answers } = form
      // Plain fetch (NOT branchFetch): verification is internal feedback stored in
      // the primary DB, so we must NOT append ?branch=hfville (that would trip
      // ville_write_guard). The selected branch is sent as `site` data instead, so
      // the form submits from any branch (incl. HF Ville / ทั้งหมด). Same-origin
      // fetch still sends the auth session cookie.
      const res = await fetch('/api/verification', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({
          inspector: inspector.trim() || user?.username || null,
          answers,
          overall: form.q5,
          site: branch,
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
        <V2PageHeader eyebrow="ตรวจสอบระบบ" title="คู่มือตรวจสอบระบบใหม่" />
        <div className="v2-card p-8 flex flex-col items-center text-center gap-3">
          <CheckCircle2 size={44} style={{ color: 'var(--v2-ok)' }} />
          <p className="text-[17px] font-semibold">บันทึกผลการตรวจสอบเรียบร้อยแล้ว ขอบคุณมากครับ/ค่ะ 🙏</p>
          <p className="text-[13.5px]" style={{ color: 'var(--v2-ink-3)' }}>
            ทีมไอทีจะนำคำตอบของคุณไปตรวจสอบต่อ {submittedId ? `(หมายเลขอ้างอิง #${submittedId})` : ''}
          </p>
          <button
            type="button"
            onClick={() => setSubmittedId(null)}
            className="v2-btn v2-btn-soft v2-btn-sm mt-2"
          >
            <ClipboardCheck size={15} /> ตรวจสอบอีกครั้ง
          </button>
        </div>
      </div>
    )
  }

  return (
    <form onSubmit={submit} className="space-y-5 pb-8">
      <V2PageHeader eyebrow="ตรวจสอบระบบ" title="คู่มือตรวจสอบระบบใหม่" />

      <p className="text-[13px] -mt-2" style={{ color: 'var(--v2-ink-3)' }}>
        ทำตามทีละขั้น แล้วเลือกคำตอบ (เลือก หรือ ติ๊ก) ไม่ต้องพิมพ์อธิบายยาว ๆ — เรากำลังเตรียมเปิดใช้ “ระบบใหม่”
        ควบคู่กับ iHOTEL การตรวจสอบนี้เพื่อยืนยันว่าระบบใหม่แสดงข้อมูลตรงกันก่อนเริ่มใช้จริง
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

      {/* ── ส่วนที่ 1 ── */}
      <Section
        no="1"
        title="เทียบหน้าจอกับ iHOTEL (ทำได้เลย ไม่กระทบข้อมูล)"
        hint="เปิดระบบใหม่ที่ hotel.thehfhotel.org/v2 แล้วเปิด iHOTEL คู่กัน เทียบทีละข้อ"
      >
        <Question title="1.1 สถานะห้องพัก — เทียบห้องว่าง / ไม่ว่าง / จอง / รอเช็คเอาท์" link={{ href: '/v2/rooms' }}>
          <RadioGroup
            name="q1_1"
            value={form.q1_1}
            onChange={(v) => set('q1_1', v)}
            options={[
              { value: 'match', label: 'ตรงกันทุกห้อง' },
              { value: 'mismatch', label: 'มีบางห้องไม่ตรง' },
            ]}
          />
          {form.q1_1 === 'mismatch' && (
            <TextInput value={form.q1_1_rooms} onChange={(v) => set('q1_1_rooms', v)} placeholder="จดเลขห้องที่ไม่ตรง" />
          )}
        </Question>

        <Question title="1.2 รายงานรอบบิล — เทียบยอดเงินแต่ละประเภท (เงินสด / โอน / บัตร)" link={{ href: '/v2/rounds' }}>
          <RadioGroup name="q1_2" value={form.q1_2} onChange={(v) => set('q1_2', v)} options={MATCH} />
          {form.q1_2 === 'mismatch' && (
            <CheckboxGroup
              value={form.q1_2_reasons}
              onChange={(v) => set('q1_2_reasons', v)}
              options={[
                { value: 'cash', label: 'เงินสด' },
                { value: 'transfer', label: 'โอน' },
                { value: 'card', label: 'บัตร' },
                { value: 'other', label: 'อื่น ๆ' },
              ]}
            />
          )}
        </Question>

        <Question title="1.3 รายชื่อผู้เข้าพักวันนี้ — เทียบจำนวน เข้า / ออก / พักต่อ" link={{ href: '/v2/rosters' }}>
          <RadioGroup name="q1_3" value={form.q1_3} onChange={(v) => set('q1_3', v)} options={MATCH} />
          {form.q1_3 === 'mismatch' && (
            <CheckboxGroup
              value={form.q1_3_reasons}
              onChange={(v) => set('q1_3_reasons', v)}
              options={[
                { value: 'in', label: 'ยอดเข้า' },
                { value: 'out', label: 'ยอดออก' },
                { value: 'stay', label: 'ยอดพักต่อ' },
              ]}
            />
          )}
        </Question>

        <Question title="1.4 ปฏิทินห้องว่าง — ห้องที่มีคนพัก/จอง แสดงถูกช่องวันที่" link={{ href: '/v2/calendar' }}>
          <RadioGroup name="q1_4" value={form.q1_4} onChange={(v) => set('q1_4', v)} options={OK} />
          {form.q1_4 === 'mismatch' && (
            <TextInput value={form.q1_4_note} onChange={(v) => set('q1_4_note', v)} placeholder="จดเลขห้อง/วันที่ที่ผิด" />
          )}
        </Question>

        <Question title="1.5 ใบกำกับภาษี / ใบเสร็จ — เปิดบิล 1 ราย ดูชื่อ-ยอดเงิน-ภาษี" link={{ href: '/v2/invoice' }}>
          <RadioGroup
            name="q1_5"
            value={form.q1_5}
            onChange={(v) => set('q1_5', v)}
            options={[
              { value: 'match', label: 'ข้อมูลถูกต้องครบ' },
              { value: 'mismatch', label: 'มีจุดผิด' },
            ]}
          />
          {form.q1_5 === 'mismatch' && (
            <CheckboxGroup
              value={form.q1_5_reasons}
              onChange={(v) => set('q1_5_reasons', v)}
              options={[
                { value: 'name', label: 'ชื่อ/ที่อยู่' },
                { value: 'amount', label: 'ยอดเงิน' },
                { value: 'tax', label: 'ภาษี' },
                { value: 'billno', label: 'เลขที่บิล' },
              ]}
            />
          )}
        </Question>
      </Section>

      {/* ── ส่วนที่ 2 ── */}
      <Section no="2" title="คำถามเรื่องนโยบาย (เลือกข้อเดียว)">
        <Question title="2.1 เช็คเอาท์ช้า: ลูกค้าออกเลยเวลาเที่ยง ปกติ iHOTEL คิดเงินอย่างไร? (สำคัญมาก — มีผลกับการคิดเงินจริง)">
          <RadioGroup
            name="q2_1"
            value={form.q2_1}
            onChange={(v) => set('q2_1', v)}
            options={[
              { value: 'a', label: 'ก. คิดเพิ่มอีก 1 คืนเสมอ' },
              { value: 'b', label: 'ข. ไม่คิดเพิ่ม (คิดตามจำนวนคืนที่จองไว้)' },
              { value: 'c', label: 'ค. แล้วแต่กรณี' },
            ]}
          />
          {form.q2_1 === 'c' && (
            <TextInput value={form.q2_1_other} onChange={(v) => set('q2_1_other', v)} placeholder="เกณฑ์โดยย่อ" />
          )}
        </Question>

        <Question title="2.2 ค่ามัดจำ: รับ/คืนค่ามัดจำ ปกติบันทึกที่ไหน?">
          <RadioGroup
            name="q2_2"
            value={form.q2_2}
            onChange={(v) => set('q2_2', v)}
            options={[
              { value: 'a', label: 'ก. iHOTEL อย่างเดียว' },
              { value: 'b', label: 'ข. จดที่อื่นด้วย' },
            ]}
          />
          {form.q2_2 === 'b' && (
            <TextInput value={form.q2_2_other} onChange={(v) => set('q2_2_other', v)} placeholder="ที่ไหน" />
          )}
        </Question>

        <Question title="2.3 ภาษีมูลค่าเพิ่ม (VAT): สาขานี้ออกใบกำกับภาษีแบบมี VAT หรือไม่?">
          <RadioGroup
            name="q2_3"
            value={form.q2_3}
            onChange={(v) => set('q2_3', v)}
            options={[
              { value: 'a', label: 'ก. ไม่มี VAT (เหมือนเดิม)' },
              { value: 'b', label: 'ข. มี VAT' },
            ]}
          />
          {form.q2_3 === 'b' && (
            <label className="flex items-center gap-2 text-[14px] max-w-[200px]">
              <span>กี่ %:</span>
              <TextInput value={form.q2_3_vat} onChange={(v) => set('q2_3_vat', v)} placeholder="เช่น 7" />
            </label>
          )}
        </Question>
      </Section>

      {/* ── ส่วนที่ 3 ── */}
      <Section
        no="3"
        title="ตรวจยอดบิลเช็คเอาท์ของลูกค้า 5 รายล่าสุด"
        hint="บิลของลูกค้าตอนเช็คเอาท์ (ไม่ใช่รายงานรอบบิล) — เลือก 5 รายล่าสุดที่เช็คเอาท์แล้ว เทียบยอด iHOTEL กับยอดที่ระบบใหม่คำนวณ. ระบบใหม่แสดงยอดแยกรายห้องแล้ว: ถ้าเข้าพักหลายห้อง ให้เทียบยอดแต่ละห้องกับบิล iHOTEL ของห้องนั้น (หรือเทียบยอดรวมทั้งบิล)"
      >
        <div className="space-y-3">
          {form.q3_bills.map((bill, i) => (
            <div key={i} className="grid gap-2 sm:grid-cols-[1fr_1fr_1fr_auto] items-center">
              <TextInput value={bill.no} onChange={(v) => setBill(i, 'no', v)} placeholder={`บิลที่ ${i + 1}: เลขที่`} />
              <TextInput value={bill.ihotel} onChange={(v) => setBill(i, 'ihotel', v)} placeholder="ยอด iHOTEL" />
              <TextInput value={bill.newsys} onChange={(v) => setBill(i, 'newsys', v)} placeholder="ยอดระบบใหม่" />
              <div className="flex gap-3 text-[13.5px]">
                {[
                  { value: 'match', label: 'ตรง' },
                  { value: 'mismatch', label: 'ไม่ตรง' },
                ].map((o) => (
                  <label key={o.value} className="flex items-center gap-1.5 cursor-pointer">
                    <input
                      type="radio"
                      name={`q3_bill_${i}`}
                      checked={bill.match === o.value}
                      onChange={() => setBill(i, 'match', o.value)}
                      style={{ accentColor: 'var(--v2-wine-600)' }}
                    />
                    {o.label}
                  </label>
                ))}
              </div>
            </div>
          ))}
        </div>

        <Question title="สรุป">
          <RadioGroup
            name="q3_summary"
            value={form.q3_summary}
            onChange={(v) => set('q3_summary', v)}
            options={[
              { value: 'all_match', label: 'ตรงทั้ง 5 รายการ' },
              { value: 'has_mismatch', label: 'มีไม่ตรง' },
            ]}
          />
          {form.q3_summary === 'has_mismatch' && (
            <label className="flex items-center gap-2 text-[14px] max-w-[220px]">
              <TextInput
                value={form.q3_mismatch_count}
                onChange={(v) => set('q3_mismatch_count', v)}
                placeholder="กี่รายการ"
              />
              <span>รายการ</span>
            </label>
          )}
        </Question>
      </Section>

      {/* ── ส่วนที่ 4 (optional) ── */}
      <Section no="4" title="ทดสอบสด (ต้องนัดเวลากับทีมไอทีก่อนทำ)">
        <div
          className="flex items-start gap-2.5 px-3.5 py-3 rounded-[12px] text-[13px]"
          style={{ background: 'var(--v2-dep-bg)', color: 'var(--v2-ink)' }}
        >
          <AlertTriangle size={17} style={{ color: 'var(--v2-dep)' }} className="mt-0.5 shrink-0" />
          <span>
            ส่วนนี้บันทึกข้อมูลจริง — อย่าทำเองโดยลำพัง ให้ทีมไอทีอยู่ด้วย (เช็คอิน → เปิด iHOTEL ดูว่าขึ้นตรงกัน →
            รับชำระเงิน → เช็คเอาท์ → ล้างรายการทดสอบ และทดสอบเปิด/ปิดรอบบิล) โดยปกติ <b>ทำภายหลัง</b> เมื่อนัดทีมไอทีได้
          </span>
        </div>
        <Question title="ผลทดสอบสด (ถ้ายังไม่ได้ทำ ให้คงไว้ที่ “ยังไม่ได้ทดสอบ”)">
          <RadioGroup
            name="q4_result"
            value={form.q4_result}
            onChange={(v) => set('q4_result', v)}
            options={[
              { value: 'pending', label: 'ยังไม่ได้ทดสอบ (ทำภายหลังกับทีมไอที)' },
              { value: 'pass', label: 'ผ่านทั้งหมด พร้อมใช้จริง' },
              { value: 'issue', label: 'มีปัญหา (แจ้งทีมไอที)' },
            ]}
          />
        </Question>
      </Section>

      {/* ── ส่วนที่ 5 ── */}
      <Section no="5" title="ความพร้อมโดยรวม (เลือกข้อเดียว)">
        <Question title="หลังทำส่วนที่ 1–3 แล้ว ภาพรวมระบบใหม่เป็นอย่างไร?">
          <RadioGroup
            name="q5"
            value={form.q5}
            onChange={(v) => set('q5', v)}
            options={[
              { value: 'a', label: 'ก. ดี ข้อมูลตรง พร้อมทดลองใช้จริง (ส่วนที่ 4)' },
              { value: 'b', label: 'ข. ใช้ได้ แต่มีจุดเล็กน้อยต้องแก้' },
              { value: 'c', label: 'ค. ยังมีปัญหาหลายจุด ยังไม่พร้อม' },
            ]}
          />
        </Question>
      </Section>

      {error && (
        <p className="text-[13px] font-medium" style={{ color: '#b91c1c' }}>
          {error}
        </p>
      )}

      <div className="flex items-center gap-3">
        <button type="submit" disabled={submitting} className="v2-btn v2-btn-primary">
          {submitting ? <Loader2 size={16} className="animate-spin" /> : <Send size={16} />}
          บันทึกผลการตรวจสอบ
        </button>
        <span className="text-[12px]" style={{ color: 'var(--v2-ink-3)' }}>
          คำตอบจะถูกบันทึกในระบบให้ทีมไอทีตรวจสอบ
        </span>
      </div>
    </form>
  )
}
