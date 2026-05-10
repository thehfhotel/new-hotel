'use client'

import { useEffect, useState, type FormEvent } from 'react'
import { Loader2, X, KeyRound, AlertCircle } from 'lucide-react'
import { ApiError, apiFetch } from '@/lib/api'
import type { UserDto } from '@/contexts/AuthContext'

/**
 * Modal that PATCHes /api/admin/users/{id} with `{password: "..."}`.
 * Used by the admin to reset a user's password (e.g. when they forget
 * it). The plaintext is sent over the existing HTTPS session cookie —
 * the backend hashes it before persisting.
 */

interface ResetPasswordModalProps {
  isOpen: boolean
  user: UserDto | null
  onClose: () => void
  onUpdated: (user: UserDto) => void
}

interface PatchResponse {
  user: UserDto
}

export default function ResetPasswordModal({
  isOpen,
  user,
  onClose,
  onUpdated,
}: ResetPasswordModalProps) {
  const [password, setPassword] = useState('')
  const [submitting, setSubmitting] = useState(false)
  const [error, setError] = useState<string | null>(null)

  useEffect(() => {
    if (!isOpen) return
    setPassword('')
    setSubmitting(false)
    setError(null)
  }, [isOpen])

  if (!isOpen || !user) return null

  const handleSubmit = async (event: FormEvent<HTMLFormElement>) => {
    event.preventDefault()
    if (submitting) return
    setError(null)
    setSubmitting(true)
    try {
      const data = await apiFetch<PatchResponse>(
        `/api/admin/users/${user.user_id}`,
        {
          method: 'PATCH',
          json: { password },
        },
      )
      onUpdated(data.user)
      onClose()
    } catch (err) {
      setError(messageForError(err))
    } finally {
      setSubmitting(false)
    }
  }

  const isSubmitDisabled = submitting || password.length < 8

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
            <KeyRound size={16} className="text-brand-500" />
            <h2 className="text-[13px] font-semibold text-text">
              รีเซ็ตรหัสผ่าน
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
          <p className="text-[12px] text-text">
            กำหนดรหัสผ่านใหม่สำหรับ{' '}
            <span className="font-semibold">{user.username}</span>
          </p>

          <div className="space-y-1">
            <label
              htmlFor="reset-password-input"
              className="block text-[12px] font-medium text-text"
            >
              รหัสผ่านใหม่
            </label>
            <input
              id="reset-password-input"
              type="password"
              value={password}
              onChange={(event) => setPassword(event.target.value)}
              autoComplete="new-password"
              autoFocus
              required
              minLength={8}
              className="w-full h-9 px-2 bg-panel border border-border focus:border-brand-500 focus:outline-none text-[13px] text-text"
            />
            <p className="text-[10px] text-textMuted">
              อย่างน้อย 8 ตัวอักษร
            </p>
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
                  <KeyRound size={14} />
                  ตั้งรหัสผ่าน
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
    if (err.status === 400 && err.message === 'invalid_password') {
      return 'รหัสผ่านต้องมีอย่างน้อย 8 ตัวอักษร'
    }
    if (err.status === 404) return 'ไม่พบผู้ใช้รายนี้'
    if (err.status === 403) return 'คุณไม่มีสิทธิ์ในการเปลี่ยนรหัสผ่าน'
  }
  return 'ไม่สามารถเปลี่ยนรหัสผ่านได้ กรุณาลองอีกครั้ง'
}
