'use client'

import { useEffect, useState, type FormEvent } from 'react'
import { Loader2, X, UserPlus, AlertCircle } from 'lucide-react'
import { ApiError, apiFetch } from '@/lib/api'
import type { Role, UserDto } from '@/contexts/AuthContext'

/**
 * Modal that POSTs to /api/admin/users to provision a new user. Lifts
 * the resulting user up to the parent so the page can refetch (or
 * optimistically prepend) without re-implementing the network call.
 *
 * Error mapping mirrors the backend's machine-readable codes:
 *  - 409 username_taken     -> "ชื่อผู้ใช้นี้ถูกใช้งานแล้ว"
 *  - 400 invalid_username   -> "ชื่อผู้ใช้ไม่ถูกต้อง (3-64 ตัวอักษร, A-Z 0-9 . _)"
 *  - 400 invalid_password   -> "รหัสผ่านต้องมีอย่างน้อย 8 ตัวอักษร"
 *  - anything else          -> generic Thai "create failed" message
 */

interface CreateUserModalProps {
  isOpen: boolean
  onClose: () => void
  onCreated: (user: UserDto) => void
}

interface CreateResponse {
  user: UserDto
}

const ROLE_OPTIONS: { value: Role; label: string }[] = [
  { value: 'receptionist', label: 'พนักงานต้อนรับ (Receptionist)' },
  { value: 'admin', label: 'ผู้ดูแลระบบ (Admin)' },
]

export default function CreateUserModal({
  isOpen,
  onClose,
  onCreated,
}: CreateUserModalProps) {
  const [username, setUsername] = useState('')
  const [password, setPassword] = useState('')
  const [role, setRole] = useState<Role>('receptionist')
  const [submitting, setSubmitting] = useState(false)
  const [error, setError] = useState<string | null>(null)

  // Reset every field when the modal opens — avoids leaking the previous
  // attempt's state into the next "create user" click.
  useEffect(() => {
    if (!isOpen) return
    setUsername('')
    setPassword('')
    setRole('receptionist')
    setSubmitting(false)
    setError(null)
  }, [isOpen])

  if (!isOpen) return null

  const handleSubmit = async (event: FormEvent<HTMLFormElement>) => {
    event.preventDefault()
    if (submitting) return
    setError(null)
    setSubmitting(true)
    try {
      const data = await apiFetch<CreateResponse>('/api/admin/users', {
        method: 'POST',
        json: { username: username.trim(), password, role },
      })
      onCreated(data.user)
      onClose()
    } catch (err) {
      setError(messageForError(err))
    } finally {
      setSubmitting(false)
    }
  }

  const isSubmitDisabled =
    submitting || username.trim().length === 0 || password.length === 0

  return (
    <div
      className="fixed inset-0 bg-black/30 flex items-center justify-center z-50"
      onClick={onClose}
    >
      <div
        className="bg-panel border border-border w-full max-w-sm mx-4"
        onClick={(event) => event.stopPropagation()}
      >
        <div className="flex items-center justify-between px-3 py-2 border-b border-border bg-headerBar">
          <div className="flex items-center gap-2">
            <UserPlus size={16} className="text-brand-500" />
            <h2 className="text-[13px] font-semibold text-text">
              สร้างผู้ใช้งานใหม่
            </h2>
          </div>
          <button
            type="button"
            onClick={onClose}
            aria-label="ปิด"
            className="text-textMuted hover:text-text transition-colors"
          >
            <X size={16} />
          </button>
        </div>

        <form onSubmit={handleSubmit} className="p-3 space-y-3" noValidate>
          <div className="space-y-1">
            <label
              htmlFor="create-user-username"
              className="block text-[12px] font-medium text-text"
            >
              ชื่อผู้ใช้
            </label>
            <input
              id="create-user-username"
              type="text"
              value={username}
              onChange={(event) => setUsername(event.target.value)}
              autoComplete="off"
              autoFocus
              required
              minLength={3}
              maxLength={64}
              className="w-full h-9 px-2 bg-panel border border-border focus:border-brand-500 focus:outline-none text-[13px] text-text"
            />
            <p className="text-[10px] text-textMuted">
              3-64 ตัวอักษร, ใช้ A-Z, 0-9, . หรือ _
            </p>
          </div>

          <div className="space-y-1">
            <label
              htmlFor="create-user-password"
              className="block text-[12px] font-medium text-text"
            >
              รหัสผ่านชั่วคราว
            </label>
            <input
              id="create-user-password"
              type="password"
              value={password}
              onChange={(event) => setPassword(event.target.value)}
              autoComplete="new-password"
              required
              minLength={8}
              className="w-full h-9 px-2 bg-panel border border-border focus:border-brand-500 focus:outline-none text-[13px] text-text"
            />
            <p className="text-[10px] text-textMuted">
              อย่างน้อย 8 ตัวอักษร — แจ้งให้ผู้ใช้เปลี่ยนหลังเข้าใช้ครั้งแรก
            </p>
          </div>

          <div className="space-y-1">
            <label
              htmlFor="create-user-role"
              className="block text-[12px] font-medium text-text"
            >
              สิทธิ์การใช้งาน
            </label>
            <select
              id="create-user-role"
              value={role}
              onChange={(event) => setRole(event.target.value as Role)}
              className="w-full h-9 px-2 bg-panel border border-border focus:border-brand-500 focus:outline-none text-[13px] text-text"
            >
              {ROLE_OPTIONS.map((option) => (
                <option key={option.value} value={option.value}>
                  {option.label}
                </option>
              ))}
            </select>
          </div>

          {error && (
            <div
              role="alert"
              className="flex items-center gap-2 p-2 bg-error/10 border border-error/40 text-error text-[12px]"
            >
              <AlertCircle className="w-4 h-4 shrink-0" />
              <span>{error}</span>
            </div>
          )}

          <div className="flex gap-2 pt-1">
            <button
              type="button"
              onClick={onClose}
              disabled={submitting}
              className="flex-1 h-9 px-3 border border-border text-[13px] text-text hover:bg-headerBar disabled:opacity-50 transition-colors"
            >
              ยกเลิก
            </button>
            <button
              type="submit"
              disabled={isSubmitDisabled}
              className="flex-1 h-9 flex items-center justify-center gap-1.5 bg-brand-600 text-white hover:bg-brand-700 disabled:opacity-50 transition-colors text-[13px] font-medium"
            >
              {submitting ? (
                <>
                  <Loader2 size={14} className="animate-spin" />
                  กำลังบันทึก...
                </>
              ) : (
                <>
                  <UserPlus size={14} />
                  สร้างผู้ใช้
                </>
              )}
            </button>
          </div>
        </form>
      </div>
    </div>
  )
}

function messageForError(err: unknown): string {
  if (err instanceof ApiError) {
    if (err.status === 409) return 'ชื่อผู้ใช้นี้ถูกใช้งานแล้ว'
    if (err.status === 400 && err.message === 'invalid_username') {
      return 'ชื่อผู้ใช้ไม่ถูกต้อง (3-64 ตัวอักษร, A-Z 0-9 . _)'
    }
    if (err.status === 400 && err.message === 'invalid_password') {
      return 'รหัสผ่านต้องมีอย่างน้อย 8 ตัวอักษร'
    }
    if (err.status === 403) return 'คุณไม่มีสิทธิ์ในการสร้างผู้ใช้'
  }
  return 'ไม่สามารถสร้างผู้ใช้ได้ กรุณาลองอีกครั้ง'
}
