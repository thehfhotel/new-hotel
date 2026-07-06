'use client'

import { Suspense, useEffect, useState, type FormEvent } from 'react'
import { useRouter, useSearchParams } from 'next/navigation'
import { Loader2, LogIn, AlertCircle, CreditCard, KeyRound } from 'lucide-react'
import { useAuth } from '@/contexts/AuthContext'
import CardTapLogin from './CardTapLogin'

/**
 * /login — the only public page in the app.
 *
 * Flow:
 *  - if the AuthContext already has a user, immediately redirect to ?redirect
 *    (or `/`) — covers the "user opens /login by mistake" case.
 *  - submit posts {username, password} via AuthContext.login(); on success
 *    we replace history with the redirect target so back-button doesn't
 *    return to the login form.
 *  - any error from login() is already mirrored on AuthContext.error.
 */
export default function LoginPage() {
  return (
    <Suspense fallback={<LoginShell><LoginPlaceholder /></LoginShell>}>
      <LoginPageInner />
    </Suspense>
  )
}

function LoginPageInner() {
  const router = useRouter()
  const searchParams = useSearchParams()
  const { user, login, error: contextError } = useAuth()

  const [username, setUsername] = useState('')
  const [password, setPassword] = useState('')
  const [submitting, setSubmitting] = useState(false)
  const [localError, setLocalError] = useState<string | null>(null)
  const [mode, setMode] = useState<'password' | 'card'>('password')

  // Open-redirect guard: only same-origin paths are allowed. Reject
  // anything that doesn't start with `/`, AND reject `//evil.com` (which
  // is protocol-relative and would navigate the browser to evil.com).
  const rawRedirect = searchParams?.get('redirect') ?? '/'
  const redirectTarget =
    rawRedirect.startsWith('/') && !rawRedirect.startsWith('//')
      ? rawRedirect
      : '/'
  const displayedError = localError ?? contextError

  // Already logged in? Bounce out of /login.
  useEffect(() => {
    if (user !== null) {
      router.replace(redirectTarget)
    }
  }, [user, redirectTarget, router])

  const handleSubmit = async (event: FormEvent<HTMLFormElement>) => {
    event.preventDefault()
    if (submitting) return
    setLocalError(null)
    setSubmitting(true)
    try {
      await login(username.trim(), password)
      router.replace(redirectTarget)
    } catch (err) {
      setLocalError(err instanceof Error ? err.message : 'login_failed')
    } finally {
      setSubmitting(false)
    }
  }

  const isSubmitDisabled =
    submitting || username.trim().length === 0 || password.length === 0

  return (
    <LoginShell>
      {/* Mode switch — password form (default) vs. tap-your-card. */}
      <div className="flex mb-3 border border-border" role="tablist">
        <button
          type="button"
          role="tab"
          aria-selected={mode === 'password'}
          onClick={() => setMode('password')}
          className={`flex-1 h-8 flex items-center justify-center gap-1.5 text-[12px] font-medium transition-colors ${
            mode === 'password'
              ? 'bg-brand-600 text-white'
              : 'bg-panel text-textMuted hover:text-text'
          }`}
        >
          <KeyRound size={13} />
          รหัสผ่าน
        </button>
        <button
          type="button"
          role="tab"
          aria-selected={mode === 'card'}
          onClick={() => setMode('card')}
          className={`flex-1 h-8 flex items-center justify-center gap-1.5 text-[12px] font-medium transition-colors ${
            mode === 'card'
              ? 'bg-brand-600 text-white'
              : 'bg-panel text-textMuted hover:text-text'
          }`}
        >
          <CreditCard size={13} />
          แตะบัตร
        </button>
      </div>

      {mode === 'card' ? (
        <CardTapLogin onLoggedIn={() => router.replace(redirectTarget)} />
      ) : (
      <form onSubmit={handleSubmit} className="space-y-3" noValidate>
        <div className="space-y-1">
          <label
            htmlFor="login-username"
            className="block text-[12px] font-medium text-text"
          >
            ชื่อผู้ใช้
          </label>
          <input
            id="login-username"
            type="text"
            value={username}
            onChange={(event) => setUsername(event.target.value)}
            autoComplete="username"
            autoFocus
            required
            className="w-full h-9 px-2 bg-panel border border-border focus:border-brand-500 focus:outline-none text-[13px] text-text"
          />
        </div>

        <div className="space-y-1">
          <label
            htmlFor="login-password"
            className="block text-[12px] font-medium text-text"
          >
            รหัสผ่าน
          </label>
          <input
            id="login-password"
            type="password"
            value={password}
            onChange={(event) => setPassword(event.target.value)}
            autoComplete="current-password"
            required
            className="w-full h-9 px-2 bg-panel border border-border focus:border-brand-500 focus:outline-none text-[13px] text-text"
          />
        </div>

        {displayedError && (
          <div
            role="alert"
            className="flex items-center gap-2 p-2 bg-error/10 border border-error/40 text-error text-[12px]"
          >
            <AlertCircle className="w-4 h-4 shrink-0" />
            <span>{displayedError}</span>
          </div>
        )}

        <button
          type="submit"
          disabled={isSubmitDisabled}
          className="w-full h-9 flex items-center justify-center gap-1.5 bg-brand-600 text-white hover:bg-brand-700 disabled:opacity-50 transition-colors text-[13px] font-medium"
        >
          {submitting ? (
            <>
              <Loader2 size={14} className="animate-spin" />
              กำลังเข้าสู่ระบบ...
            </>
          ) : (
            <>
              <LogIn size={14} />
              เข้าสู่ระบบ
            </>
          )}
        </button>
      </form>
      )}
    </LoginShell>
  )
}

/**
 * Centered card chrome — kept as its own component so the Suspense fallback
 * can render the same frame while useSearchParams() resolves on the client.
 */
function LoginShell({ children }: { children: React.ReactNode }) {
  return (
    <div className="min-h-screen flex items-center justify-center bg-shell px-4">
      <div className="w-full max-w-sm bg-panel border border-border">
        <div className="bg-headerBar border-b border-border px-3 py-2">
          <h1 className="text-[14px] font-semibold text-text">
            เข้าสู่ระบบจัดการโรงแรม
          </h1>
          <p className="text-[11px] text-textMuted mt-0.5">
            กรุณาใช้บัญชีที่ผู้ดูแลระบบมอบให้
          </p>
        </div>
        <div className="p-3">{children}</div>
      </div>
    </div>
  )
}

function LoginPlaceholder() {
  return (
    <div className="flex items-center justify-center py-6">
      <Loader2 className="animate-spin h-5 w-5 text-brand-500" />
    </div>
  )
}
