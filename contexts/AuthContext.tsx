'use client'

import {
  createContext,
  useCallback,
  useContext,
  useEffect,
  useState,
  type ReactNode,
} from 'react'
import { usePathname, useRouter } from 'next/navigation'
import { ApiError, apiFetch } from '@/lib/api'

// Track G7 widened the role set from binary (admin/receptionist) to the
// four iHOTEL-aligned roles. Existing `Role`-typed call sites still
// accept the legacy union; new code branching on cashier/housekeeper
// should pivot on `hasPermission(...)` instead so we don't have to
// audit every role check when more roles land.
export type Role = 'admin' | 'cashier' | 'housekeeper' | 'receptionist'

export interface UserDto {
  user_id: number
  username: string
  role: Role
  active: boolean
  created_at: string
  last_login_at: string | null
}

export interface AuthState {
  user: UserDto | null
  /**
   * Permission keys (e.g. "payment.refund") granted to `user` via the
   * role grid in `ht_role_permissions`. Empty when logged out OR when
   * the backend permission lookup transiently fails — components that
   * rely on a permission MUST handle the empty case as "no access"
   * rather than "everyone has access".
   */
  permissions: string[]
  loading: boolean
  error: string | null
}

export interface AuthContextValue extends AuthState {
  login: (username: string, password: string) => Promise<void>
  logout: () => Promise<void>
  refresh: () => Promise<void>
  /**
   * Convenience predicate: does the current user hold the given
   * permission key? Returns `false` when logged out. Components
   * SHOULD call this rather than indexing `permissions` directly so
   * the contract stays explicit at every call site.
   */
  hasPermission: (key: string) => boolean
}

const AuthContext = createContext<AuthContextValue | null>(null)

const PUBLIC_PATHS = new Set<string>(['/login'])

/**
 * Read-once flag: when `false` (the default in dev / pre-cutover) the
 * AuthGuard does NOT enforce a logged-in user. This avoids an infinite
 * /login redirect loop while the backend still ships AUTH_ENABLED=false
 * (which makes /api/auth/me return 401 unconditionally). Operator flips
 * BOTH backend AUTH_ENABLED=true AND frontend NEXT_PUBLIC_AUTH_REQUIRED=true
 * at cutover.
 */
const AUTH_REQUIRED = process.env.NEXT_PUBLIC_AUTH_REQUIRED === 'true'

interface LoginResponse {
  user: UserDto
  /**
   * Track G7: backend now ships permissions alongside the user on
   * login too, so the UI is gate-aware from the very first render
   * after authentication (no follow-up /me round-trip required).
   * Optional on the wire for backward compatibility with older
   * backends; treat missing as "empty list".
   */
  permissions?: string[]
}

interface MeResponse {
  user: UserDto
  /**
   * Track G7: backend ships the user's granted permission keys here so
   * the frontend can hide buttons the user cannot exercise. The field
   * is optional on the wire — older backends predate the column and
   * omit it; treat missing as "empty list" (all gated UI hidden, which
   * is the safe default).
   */
  permissions?: string[]
}

/**
 * AuthProvider: hydrates the current user from /api/auth/me on mount and
 * exposes login/logout/refresh actions. Wraps children in <AuthGuard> so
 * that — when NEXT_PUBLIC_AUTH_REQUIRED=true — protected pages redirect
 * unauthenticated users to /login.
 */
export function AuthProvider({ children }: { children: ReactNode }) {
  const [user, setUser] = useState<UserDto | null>(null)
  const [permissions, setPermissions] = useState<string[]>([])
  const [loading, setLoading] = useState(true)
  const [error, setError] = useState<string | null>(null)

  const refresh = useCallback(async () => {
    try {
      const data = await apiFetch<MeResponse>('/api/auth/me')
      setUser(data?.user ?? null)
      setPermissions(data?.permissions ?? [])
      setError(null)
    } catch (err) {
      // 401 simply means "no session" (or auth disabled) — not a UI error.
      if (err instanceof ApiError && err.status === 401) {
        setUser(null)
        setPermissions([])
        setError(null)
        return
      }
      // Anything else (network, 500) is worth surfacing for diagnosis but
      // shouldn't trap the user — leave them logged-out so they can retry.
      setUser(null)
      setPermissions([])
      setError(err instanceof Error ? err.message : 'auth_check_failed')
    } finally {
      setLoading(false)
    }
  }, [])

  const login = useCallback(async (username: string, password: string) => {
    setError(null)
    try {
      const data = await apiFetch<LoginResponse>('/api/auth/login', {
        method: 'POST',
        json: { username, password },
      })
      setUser(data?.user ?? null)
      setPermissions(data?.permissions ?? [])
    } catch (err) {
      if (err instanceof ApiError && err.status === 401) {
        const message = 'ชื่อผู้ใช้หรือรหัสผ่านไม่ถูกต้อง'
        setError(message)
        throw new Error(message)
      }
      const message = err instanceof Error ? err.message : 'login_failed'
      setError(message)
      throw err instanceof Error ? err : new Error(message)
    }
  }, [])

  const logout = useCallback(async () => {
    try {
      await apiFetch('/api/auth/logout', { method: 'POST' })
    } catch {
      // Logout is idempotent — clear local state regardless of network result.
    } finally {
      setUser(null)
      setPermissions([])
      setError(null)
    }
  }, [])

  const hasPermission = useCallback(
    (key: string) => permissions.includes(key),
    [permissions],
  )

  useEffect(() => {
    refresh()
  }, [refresh])

  const value: AuthContextValue = {
    user,
    permissions,
    loading,
    error,
    login,
    logout,
    refresh,
    hasPermission,
  }

  return (
    <AuthContext.Provider value={value}>
      <AuthGuard>{children}</AuthGuard>
    </AuthContext.Provider>
  )
}

/**
 * Guards every page below the provider:
 *  - while the initial /api/auth/me check is in flight, render a tiny
 *    placeholder (avoids flash of "logged out" UI for a logged-in user)
 *  - when AUTH_REQUIRED is false (dev / pre-cutover), pass-through —
 *    every page renders regardless of user state
 *  - when AUTH_REQUIRED is true and user is null and we're not already on
 *    /login, redirect to /login?redirect=<current> and render nothing
 */
function AuthGuard({ children }: { children: ReactNode }) {
  const { user, loading } = useContext(AuthContext) as AuthContextValue
  const pathname = usePathname()
  const router = useRouter()

  const isPublicPath = pathname ? PUBLIC_PATHS.has(pathname) : false
  const shouldRedirect =
    AUTH_REQUIRED && !loading && user === null && !isPublicPath

  useEffect(() => {
    if (!shouldRedirect) return
    const redirectTarget = pathname ?? '/'
    router.replace(`/login?redirect=${encodeURIComponent(redirectTarget)}`)
  }, [shouldRedirect, pathname, router])

  // Hydration shim: while the initial /me check is pending OR a redirect
  // is queued, render an empty fragment so we don't flash protected UI.
  if (loading || shouldRedirect) {
    return null
  }

  return <>{children}</>
}

export function useAuth(): AuthContextValue {
  const ctx = useContext(AuthContext)
  if (ctx === null) {
    throw new Error('useAuth must be used within <AuthProvider>')
  }
  return ctx
}
