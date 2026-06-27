'use client'

import { useCallback, useEffect, useState } from 'react'
import { Plus, X, Loader2, Check, DoorClosed, Users, StickyNote } from 'lucide-react'
import { useBranchFetch } from '@/lib/use-branch-fetch'
import { useBranch } from '@/contexts/BranchContext'
import { useAuth } from '@/contexts/AuthContext'
import { V2PageHeader, V2Spinner, V2Empty, VilleNotice } from '@/components/v2/primitives'
import { formatThai } from '@/components/v2/RoundReport'

/**
 * Room & staff sticky notes (โน้ตห้อง / โน้ตพนักงาน) — task #47 / migration 062.
 * iHOTEL equivalent: Room_Note.cs / EMP_Note.cs (HT_Room_SMS / HT_EMP_SMS with
 * the SMS_Readed read-flag flow). Consumes GET /api/notes for the list +
 * unread count, POST /api/notes to add a note, and POST /api/notes/{id}/read to
 * mark one read. All calls are branch-aware via useBranchFetch (?branch=).
 */

type NoteKind = 'room' | 'staff'

interface Note {
  noteId: number
  targetKind: NoteKind
  targetKey: string
  body: string
  createdBy: string | null
  isRead: boolean
  legacyId: number | null
  source: string
  createdAt: string
  updatedAt: string
}

interface NoteListData {
  success: boolean
  notes: Note[]
  unreadCount: number
}

const KIND_TABS: { value: 'all' | NoteKind; label: string }[] = [
  { value: 'all', label: 'ทั้งหมด' },
  { value: 'room', label: 'โน้ตห้อง' },
  { value: 'staff', label: 'โน้ตพนักงาน' },
]

const EMPTY_FORM = { targetKind: 'room' as NoteKind, targetKey: '', body: '' }

export default function V2Notes() {
  const branchFetch = useBranchFetch()
  const { branch } = useBranch()
  const { user } = useAuth()

  const [kindFilter, setKindFilter] = useState<'all' | NoteKind>('all')
  const [unreadOnly, setUnreadOnly] = useState(false)
  const [data, setData] = useState<NoteListData | null>(null)
  const [loading, setLoading] = useState(true)
  const [showForm, setShowForm] = useState(false)
  const [form, setForm] = useState(EMPTY_FORM)
  const [submitting, setSubmitting] = useState(false)
  const [formError, setFormError] = useState<string | null>(null)
  const [markingId, setMarkingId] = useState<number | null>(null)

  const fetchList = useCallback(async () => {
    setLoading(true)
    try {
      const qs = new URLSearchParams()
      if (kindFilter !== 'all') qs.set('kind', kindFilter)
      if (unreadOnly) qs.set('unread', 'true')
      const suffix = qs.toString()
      const res = await branchFetch(`/api/notes${suffix ? `?${suffix}` : ''}`)
      setData(res.ok ? await res.json() : null)
    } catch {
      setData(null)
    } finally {
      setLoading(false)
    }
  }, [branchFetch, kindFilter, unreadOnly])

  useEffect(() => {
    fetchList()
  }, [fetchList])

  const submit = async (e: React.FormEvent) => {
    e.preventDefault()
    setFormError(null)
    if (!form.targetKey.trim()) {
      setFormError(form.targetKind === 'room' ? 'กรุณาระบุหมายเลขห้อง' : 'กรุณาระบุชื่อพนักงาน')
      return
    }
    if (!form.body.trim()) {
      setFormError('กรุณาระบุข้อความ')
      return
    }
    setSubmitting(true)
    try {
      const res = await branchFetch('/api/notes', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({
          targetKind: form.targetKind,
          targetKey: form.targetKey.trim(),
          body: form.body.trim(),
          createdBy: user?.username ?? null,
        }),
      })
      if (!res.ok) {
        const b = await res.json().catch(() => null)
        setFormError(b?.error || b?.message || 'บันทึกไม่สำเร็จ')
        return
      }
      setForm({ ...EMPTY_FORM, targetKind: form.targetKind })
      setShowForm(false)
      await fetchList()
    } catch {
      setFormError('เกิดข้อผิดพลาดในการเชื่อมต่อ')
    } finally {
      setSubmitting(false)
    }
  }

  const markRead = async (noteId: number) => {
    setMarkingId(noteId)
    try {
      const res = await branchFetch(`/api/notes/${noteId}/read`, { method: 'POST' })
      if (res.ok) {
        // Optimistic local update so the badge clears immediately.
        setData((prev) =>
          prev
            ? {
                ...prev,
                unreadCount: Math.max(0, prev.unreadCount - 1),
                notes: unreadOnly
                  ? prev.notes.filter((n) => n.noteId !== noteId)
                  : prev.notes.map((n) => (n.noteId === noteId ? { ...n, isRead: true } : n)),
              }
            : prev
        )
      }
    } catch {
      /* ignore — next refresh reconciles */
    } finally {
      setMarkingId(null)
    }
  }

  return (
    <div className="space-y-5">
      <V2PageHeader
        eyebrow="ปฏิบัติการ"
        title="โน้ตห้อง / โน้ตพนักงาน"
        right={
          <button
            onClick={() => {
              setShowForm((v) => !v)
              setFormError(null)
            }}
            className="v2-btn v2-btn-soft v2-btn-sm"
          >
            {showForm ? <X size={15} /> : <Plus size={15} />}
            {showForm ? 'ปิดฟอร์ม' : 'เพิ่มโน้ต'}
          </button>
        }
      />

      <VilleNotice branch={branch} />

      {/* Add-note form */}
      {showForm && (
        <form onSubmit={submit} className="v2-card p-4 space-y-3">
          <div className="flex gap-2">
            {(['room', 'staff'] as const).map((k) => (
              <button
                key={k}
                type="button"
                onClick={() => setForm((f) => ({ ...f, targetKind: k }))}
                className="v2-btn v2-btn-sm"
                style={{
                  background: form.targetKind === k ? 'var(--v2-wine-50)' : 'var(--v2-surface-2)',
                  color: form.targetKind === k ? 'var(--v2-wine-600)' : 'var(--v2-ink-2)',
                  fontWeight: form.targetKind === k ? 600 : 400,
                }}
              >
                {k === 'room' ? <DoorClosed size={14} /> : <Users size={14} />}
                {k === 'room' ? 'โน้ตห้อง' : 'โน้ตพนักงาน'}
              </button>
            ))}
          </div>

          <label className="flex flex-col gap-1">
            <span className="text-[11px] font-medium" style={{ color: 'var(--v2-ink-3)' }}>
              {form.targetKind === 'room' ? 'หมายเลขห้อง' : 'ชื่อพนักงาน (username)'}
            </span>
            <input
              type="text"
              value={form.targetKey}
              onChange={(e) => setForm((f) => ({ ...f, targetKey: e.target.value }))}
              placeholder={form.targetKind === 'room' ? 'เช่น 402' : 'เช่น somchai'}
              className="h-9 px-3 rounded-[10px] text-[14px] outline-none"
              style={{ background: 'var(--v2-surface)', border: '1px solid var(--v2-line-2)' }}
            />
          </label>

          <label className="flex flex-col gap-1">
            <span className="text-[11px] font-medium" style={{ color: 'var(--v2-ink-3)' }}>ข้อความ</span>
            <textarea
              value={form.body}
              onChange={(e) => setForm((f) => ({ ...f, body: e.target.value }))}
              rows={3}
              className="px-3 py-2 rounded-[10px] text-[14px] outline-none resize-y"
              style={{ background: 'var(--v2-surface)', border: '1px solid var(--v2-line-2)' }}
            />
          </label>

          {formError && (
            <p className="text-[12.5px]" style={{ color: '#b91c1c' }}>{formError}</p>
          )}

          <div className="flex gap-2">
            <button type="submit" disabled={submitting} className="v2-btn v2-btn-primary v2-btn-sm">
              {submitting ? <Loader2 size={15} className="animate-spin" /> : <Plus size={15} />}
              บันทึกโน้ต
            </button>
          </div>
        </form>
      )}

      {/* Filters */}
      <div className="flex flex-wrap items-center gap-2">
        {KIND_TABS.map((tab) => (
          <button
            key={tab.value}
            onClick={() => setKindFilter(tab.value)}
            className="v2-btn v2-btn-sm"
            style={{
              background: kindFilter === tab.value ? 'var(--v2-wine-50)' : 'var(--v2-surface-2)',
              color: kindFilter === tab.value ? 'var(--v2-wine-600)' : 'var(--v2-ink-2)',
              fontWeight: kindFilter === tab.value ? 600 : 400,
            }}
          >
            {tab.label}
          </button>
        ))}
        <button
          onClick={() => setUnreadOnly((v) => !v)}
          className="v2-btn v2-btn-sm"
          style={{
            background: unreadOnly ? 'var(--v2-wine-50)' : 'var(--v2-surface-2)',
            color: unreadOnly ? 'var(--v2-wine-600)' : 'var(--v2-ink-2)',
            fontWeight: unreadOnly ? 600 : 400,
          }}
        >
          เฉพาะที่ยังไม่อ่าน
          {data && data.unreadCount > 0 && (
            <span
              className="v2-num ml-1 px-1.5 rounded-full text-[11px]"
              style={{ background: 'var(--v2-wine-600)', color: '#fff' }}
            >
              {data.unreadCount}
            </span>
          )}
        </button>
      </div>

      {loading ? (
        <V2Spinner label="กำลังโหลดโน้ต…" />
      ) : !data || data.notes.length === 0 ? (
        <V2Empty title="ไม่มีโน้ต" hint="กดเพิ่มโน้ตเพื่อฝากข้อความถึงห้องหรือพนักงาน" />
      ) : (
        <div className="space-y-2.5">
          {data.notes.map((n) => (
            <div
              key={n.noteId}
              className="v2-card p-3.5 flex items-start gap-3"
              style={{ opacity: n.isRead ? 0.7 : 1 }}
            >
              <div
                className="flex h-8 w-8 shrink-0 items-center justify-center rounded-full"
                style={{ background: 'var(--v2-surface-2)', color: 'var(--v2-wine-600)' }}
              >
                {n.targetKind === 'room' ? <DoorClosed size={15} /> : <Users size={15} />}
              </div>

              <div className="min-w-0 flex-1">
                <div className="flex flex-wrap items-center gap-2">
                  <span className="text-[13px] font-semibold" style={{ color: 'var(--v2-ink)' }}>
                    {n.targetKind === 'room' ? `ห้อง ${n.targetKey}` : n.targetKey}
                  </span>
                  {!n.isRead && (
                    <span
                      className="text-[10.5px] px-1.5 py-0.5 rounded"
                      style={{ background: 'var(--v2-wine-50)', color: 'var(--v2-wine-600)' }}
                    >
                      ยังไม่อ่าน
                    </span>
                  )}
                </div>
                <p className="mt-0.5 text-[13.5px] whitespace-pre-wrap" style={{ color: 'var(--v2-ink-2)' }}>
                  {n.body}
                </p>
                <div className="mt-1 text-[11px]" style={{ color: 'var(--v2-ink-3)' }}>
                  {[n.createdBy, formatThai(n.createdAt)].filter(Boolean).join(' · ')}
                </div>
              </div>

              {!n.isRead && (
                <button
                  onClick={() => markRead(n.noteId)}
                  disabled={markingId === n.noteId}
                  className="v2-btn v2-btn-ghost v2-btn-sm shrink-0"
                  aria-label="ทำเครื่องหมายว่าอ่านแล้ว"
                >
                  {markingId === n.noteId ? (
                    <Loader2 size={14} className="animate-spin" />
                  ) : (
                    <Check size={14} />
                  )}
                  อ่านแล้ว
                </button>
              )}
            </div>
          ))}

          <p className="flex items-center gap-1.5 text-[11.5px]" style={{ color: 'var(--v2-ink-3)' }}>
            <StickyNote size={13} />
            {data.notes.length} โน้ต · {data.unreadCount} ยังไม่อ่าน
          </p>
        </div>
      )}
    </div>
  )
}
