'use client'

import { useCallback, useEffect, useState } from 'react'
import Link from 'next/link'
import {
  Loader2,
  RefreshCw,
  UserPlus,
  Users as UsersIcon,
  CheckCircle2,
  XCircle,
  KeyRound,
  ShieldOff,
  ShieldCheck,
  AlertCircle,
  Home,
} from 'lucide-react'
import { useAuth, type Role, type UserDto } from '@/contexts/AuthContext'
import { ApiError, apiFetch } from '@/lib/api'
import { formatStoredDateTime } from '@/lib/format'
import CreateUserModal from './CreateUserModal'
import ResetPasswordModal from './ResetPasswordModal'

/**
 * /admin/users — admin-only user management.
 *
 * Renders three states based on the AuthContext:
 *  - user is null (still loading or unauthenticated): the AuthGuard in
 *    AuthContext is responsible for either rendering nothing or
 *    redirecting to /login. This page therefore returns null in that
 *    case so it doesn't flash before the guard takes effect.
 *  - user is non-null but role !== 'admin': render <ForbiddenView/>.
 *  - user is admin: fetch /api/admin/users and render the table.
 *
 * All admin API calls go through `apiFetch` so the HttpOnly session
 * cookie rides automatically (PR3).
 */

interface ListResponse {
  users: UserDto[]
}

interface PatchResponse {
  user: UserDto
}

export default function AdminUsersPage() {
  const { user } = useAuth()

  if (user === null) {
    // AuthGuard handles login redirect when AUTH_REQUIRED. We render
    // nothing here so the loader doesn't flash before the redirect.
    return null
  }

  if (user.role !== 'admin') {
    return <ForbiddenView />
  }

  return <AdminUsersView />
}

function AdminUsersView() {
  const [users, setUsers] = useState<UserDto[]>([])
  const [loading, setLoading] = useState(true)
  const [refreshing, setRefreshing] = useState(false)
  const [error, setError] = useState<string | null>(null)
  const [actionError, setActionError] = useState<string | null>(null)
  const [pendingId, setPendingId] = useState<number | null>(null)
  const [createOpen, setCreateOpen] = useState(false)
  const [resetTarget, setResetTarget] = useState<UserDto | null>(null)

  const loadUsers = useCallback(async (mode: 'initial' | 'refresh' = 'initial') => {
    if (mode === 'initial') setLoading(true)
    else setRefreshing(true)
    setError(null)
    try {
      const data = await apiFetch<ListResponse>('/api/admin/users')
      setUsers(data.users ?? [])
    } catch (err) {
      setError(err instanceof Error ? err.message : 'load_failed')
    } finally {
      setLoading(false)
      setRefreshing(false)
    }
  }, [])

  useEffect(() => {
    loadUsers('initial')
  }, [loadUsers])

  const upsertUser = useCallback((updated: UserDto) => {
    setUsers((current) => {
      const existing = current.findIndex((u) => u.user_id === updated.user_id)
      if (existing === -1) {
        return [...current, updated].sort((a, b) =>
          a.username.localeCompare(b.username),
        )
      }
      const next = [...current]
      next[existing] = updated
      return next
    })
  }, [])

  const patchUser = useCallback(
    async (target: UserDto, body: Partial<{ active: boolean; role: Role }>) => {
      setActionError(null)
      setPendingId(target.user_id)
      try {
        const data = await apiFetch<PatchResponse>(
          `/api/admin/users/${target.user_id}`,
          {
            method: 'PATCH',
            json: body,
          },
        )
        upsertUser(data.user)
      } catch (err) {
        setActionError(messageForActionError(err))
      } finally {
        setPendingId(null)
      }
    },
    [upsertUser],
  )

  const handleToggleActive = (target: UserDto) =>
    patchUser(target, { active: !target.active })

  const handleToggleRole = (target: UserDto) =>
    patchUser(target, {
      role: target.role === 'admin' ? 'receptionist' : 'admin',
    })

  return (
    <div className="min-h-screen bg-shell">
      <div className="max-w-5xl mx-auto p-4 space-y-3">
        <header className="flex items-center justify-between">
          <div className="flex items-center gap-2">
            <UsersIcon size={18} className="text-brand-500" />
            <h1 className="text-[15px] font-semibold text-text">
              ผู้ใช้งานระบบ
            </h1>
          </div>
          <div className="flex items-center gap-1.5">
            <button
              type="button"
              onClick={() => loadUsers('refresh')}
              disabled={refreshing || loading}
              className="h-8 px-2 flex items-center gap-1 border border-border text-[12px] text-text hover:bg-headerBar disabled:opacity-50 transition-colors"
            >
              <RefreshCw
                size={13}
                className={refreshing ? 'animate-spin' : undefined}
              />
              <span>รีเฟรช</span>
            </button>
            <button
              type="button"
              onClick={() => setCreateOpen(true)}
              className="h-8 px-2.5 flex items-center gap-1 bg-brand-600 text-white hover:bg-brand-700 text-[12px] font-medium transition-colors"
            >
              <UserPlus size={13} />
              <span>สร้างผู้ใช้</span>
            </button>
          </div>
        </header>

        {actionError && (
          <div
            role="alert"
            className="flex items-center gap-2 p-2 bg-error/10 border border-error/40 text-error text-[12px]"
          >
            <AlertCircle size={14} className="shrink-0" />
            <span>{actionError}</span>
          </div>
        )}

        {loading ? (
          <LoadingState />
        ) : error ? (
          <ErrorState message={error} onRetry={() => loadUsers('initial')} />
        ) : users.length === 0 ? (
          <EmptyState onCreate={() => setCreateOpen(true)} />
        ) : (
          <UsersTable
            users={users}
            pendingId={pendingId}
            onToggleActive={handleToggleActive}
            onToggleRole={handleToggleRole}
            onResetPassword={(target) => setResetTarget(target)}
          />
        )}
      </div>

      <CreateUserModal
        isOpen={createOpen}
        onClose={() => setCreateOpen(false)}
        onCreated={(user) => upsertUser(user)}
      />

      <ResetPasswordModal
        isOpen={resetTarget !== null}
        user={resetTarget}
        onClose={() => setResetTarget(null)}
        onUpdated={(user) => upsertUser(user)}
      />
    </div>
  )
}

interface UsersTableProps {
  users: UserDto[]
  pendingId: number | null
  onToggleActive: (user: UserDto) => void
  onToggleRole: (user: UserDto) => void
  onResetPassword: (user: UserDto) => void
}

function UsersTable({
  users,
  pendingId,
  onToggleActive,
  onToggleRole,
  onResetPassword,
}: UsersTableProps) {
  return (
    <div className="border border-border bg-panel overflow-x-auto">
      <table className="w-full text-[12px] text-text">
        <thead className="bg-headerBar text-[11px] uppercase tracking-wide text-textMuted">
          <tr>
            <th className="text-left px-3 py-2">ID</th>
            <th className="text-left px-3 py-2">ชื่อผู้ใช้</th>
            <th className="text-left px-3 py-2">สิทธิ์</th>
            <th className="text-left px-3 py-2">สถานะ</th>
            <th className="text-left px-3 py-2">เข้าใช้ล่าสุด</th>
            <th className="text-left px-3 py-2">สร้างเมื่อ</th>
            <th className="text-right px-3 py-2">การจัดการ</th>
          </tr>
        </thead>
        <tbody>
          {users.map((user) => {
            const pending = pendingId === user.user_id
            return (
              <tr key={user.user_id} className="border-t border-border">
                <td className="px-3 py-2 text-textMuted font-mono">
                  {user.user_id}
                </td>
                <td className="px-3 py-2 font-medium">{user.username}</td>
                <td className="px-3 py-2">
                  <RoleBadge role={user.role} />
                </td>
                <td className="px-3 py-2">
                  <ActiveBadge active={user.active} />
                </td>
                <td className="px-3 py-2 text-textMuted">
                  {formatDate(user.last_login_at)}
                </td>
                <td className="px-3 py-2 text-textMuted">
                  {formatDate(user.created_at)}
                </td>
                <td className="px-3 py-2">
                  <div className="flex items-center justify-end gap-1">
                    <button
                      type="button"
                      onClick={() => onToggleActive(user)}
                      disabled={pending}
                      title={user.active ? 'ปิดใช้งาน' : 'เปิดใช้งาน'}
                      className="h-7 px-2 flex items-center gap-1 border border-border text-[11px] hover:bg-headerBar disabled:opacity-50 transition-colors"
                    >
                      {user.active ? (
                        <ShieldOff size={12} />
                      ) : (
                        <ShieldCheck size={12} />
                      )}
                      <span>{user.active ? 'ปิด' : 'เปิด'}</span>
                    </button>
                    <button
                      type="button"
                      onClick={() => onToggleRole(user)}
                      disabled={pending}
                      title="สลับสิทธิ์ admin/receptionist"
                      className="h-7 px-2 flex items-center gap-1 border border-border text-[11px] hover:bg-headerBar disabled:opacity-50 transition-colors"
                    >
                      <span>
                        {user.role === 'admin'
                          ? '→ receptionist'
                          : '→ admin'}
                      </span>
                    </button>
                    <button
                      type="button"
                      onClick={() => onResetPassword(user)}
                      disabled={pending}
                      title="รีเซ็ตรหัสผ่าน"
                      className="h-7 px-2 flex items-center gap-1 border border-border text-[11px] hover:bg-headerBar disabled:opacity-50 transition-colors"
                    >
                      <KeyRound size={12} />
                      <span>รีเซ็ต</span>
                    </button>
                    {pending && (
                      <Loader2 size={12} className="animate-spin text-textMuted" />
                    )}
                  </div>
                </td>
              </tr>
            )
          })}
        </tbody>
      </table>
    </div>
  )
}

function RoleBadge({ role }: { role: Role }) {
  const isAdmin = role === 'admin'
  return (
    <span
      className={`inline-flex items-center px-1.5 py-0.5 text-[10px] font-medium uppercase tracking-wide ${
        isAdmin
          ? 'bg-brand-50 text-brand-700 border border-brand-500/30'
          : 'bg-headerBar text-textMuted border border-border'
      }`}
    >
      {role}
    </span>
  )
}

function ActiveBadge({ active }: { active: boolean }) {
  return active ? (
    <span className="inline-flex items-center gap-1 text-[11px] text-emerald-600">
      <CheckCircle2 size={12} />
      <span>active</span>
    </span>
  ) : (
    <span className="inline-flex items-center gap-1 text-[11px] text-textMuted">
      <XCircle size={12} />
      <span>disabled</span>
    </span>
  )
}

function LoadingState() {
  return (
    <div className="flex items-center justify-center gap-2 py-12 text-textMuted text-[12px]">
      <Loader2 size={14} className="animate-spin" />
      <span>กำลังโหลดข้อมูลผู้ใช้...</span>
    </div>
  )
}

function ErrorState({
  message,
  onRetry,
}: {
  message: string
  onRetry: () => void
}) {
  return (
    <div className="border border-error/40 bg-error/10 p-3 space-y-2">
      <div className="flex items-center gap-2 text-error text-[12px]">
        <AlertCircle size={14} />
        <span>โหลดข้อมูลไม่สำเร็จ: {message}</span>
      </div>
      <button
        type="button"
        onClick={onRetry}
        className="h-7 px-2 border border-border text-[11px] text-text hover:bg-headerBar transition-colors"
      >
        ลองอีกครั้ง
      </button>
    </div>
  )
}

function EmptyState({ onCreate }: { onCreate: () => void }) {
  return (
    <div className="border border-dashed border-border bg-panel p-6 text-center space-y-2">
      <UsersIcon size={20} className="mx-auto text-textMuted" />
      <p className="text-[12px] text-textMuted">ยังไม่มีผู้ใช้งานในระบบ</p>
      <button
        type="button"
        onClick={onCreate}
        className="h-8 px-3 inline-flex items-center gap-1 bg-brand-600 text-white text-[12px] hover:bg-brand-700 transition-colors"
      >
        <UserPlus size={13} />
        สร้างผู้ใช้คนแรก
      </button>
    </div>
  )
}

/**
 * Shown to authenticated users whose role isn't `admin`. Provides a
 * polite explanation + a link back to the home page so they don't get
 * stuck.
 */
function ForbiddenView() {
  return (
    <div className="min-h-screen bg-shell flex items-center justify-center p-4">
      <div className="bg-panel border border-border w-full max-w-sm">
        <div className="bg-headerBar border-b border-border px-3 py-2">
          <h1 className="text-[14px] font-semibold text-text">
            ไม่มีสิทธิ์เข้าถึงหน้านี้
          </h1>
        </div>
        <div className="p-3 space-y-3">
          <p className="text-[12px] text-text">
            หน้าจัดการผู้ใช้สามารถเข้าใช้ได้เฉพาะผู้ดูแลระบบเท่านั้น
            กรุณาติดต่อผู้ดูแลของท่านหากต้องการสิทธิ์เพิ่มเติม
          </p>
          <Link
            href="/"
            className="h-8 px-3 inline-flex items-center gap-1 bg-brand-600 text-white text-[12px] hover:bg-brand-700 transition-colors"
          >
            <Home size={13} />
            กลับสู่หน้าหลัก
          </Link>
        </div>
      </div>
    </div>
  )
}

function formatDate(value: string | null): string {
  if (!value) return '—'
  if (Number.isNaN(new Date(value).getTime())) return value
  return formatStoredDateTime(value)
}

function messageForActionError(err: unknown): string {
  if (err instanceof ApiError) {
    if (err.status === 404) return 'ไม่พบผู้ใช้รายนี้'
    if (err.status === 403) return 'คุณไม่มีสิทธิ์ในการแก้ไขผู้ใช้'
    if (err.status === 400) return 'ข้อมูลไม่ถูกต้อง'
  }
  return 'การอัปเดตล้มเหลว กรุณาลองอีกครั้ง'
}
