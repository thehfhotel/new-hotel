'use client'

import { useEffect, useRef, useState } from 'react'
import { Loader2, CreditCard, AlertCircle, CheckCircle2 } from 'lucide-react'
import { ApiError, apiFetch } from '@/lib/api'
import { useAuth } from '@/contexts/AuthContext'

/**
 * CardTapLogin — "tap your NFC staff card to log in" panel.
 *
 * Flow (backend: routes::reader + routes::auth::card_login):
 *  1. POST /api/reader/claim {reader_id}  → sets the HttpOnly reader_claim
 *     cookie binding THIS browser to a reader_id (closes the wait/hijack race).
 *  2. GET  /api/reader/wait               → long-polls (~25s). 204 = no tap yet
 *     (re-poll); 200 {login_token} = a tap resolved and a one-time login_token
 *     is ready; 401 = claim cookie missing/expired (re-claim).
 *  3. POST /api/auth/card-login {login_token} → mints the normal session cookie.
 *  4. refresh() to hydrate AuthContext, then onLoggedIn() to redirect.
 *
 * The per-terminal reader_id comes from localStorage (`reader_id`) or
 * `NEXT_PUBLIC_READER_ID`. When neither is set the panel shows a one-time
 * "pair this terminal" input instead of polling.
 */

interface WaitResponse {
  login_token?: string
}

/** Initial reader_id: localStorage wins, then the build-time env default. */
function initialReaderId(): string {
  if (typeof window !== 'undefined') {
    const stored = window.localStorage.getItem('reader_id')
    if (stored && stored.trim()) return stored.trim()
  }
  return (process.env.NEXT_PUBLIC_READER_ID ?? '').trim()
}

export default function CardTapLogin({ onLoggedIn }: { onLoggedIn: () => void }) {
  const { refresh } = useAuth()
  const [readerId, setReaderId] = useState<string>(() => initialReaderId())
  const [pairInput, setPairInput] = useState('')
  const [status, setStatus] = useState<'waiting' | 'success'>('waiting')
  const [error, setError] = useState<string | null>(null)

  // Stash the redirect callback in a ref so a fresh identity each render does
  // not tear down and restart the polling effect (mirrors useIdleLogout).
  const onLoggedInRef = useRef(onLoggedIn)
  onLoggedInRef.current = onLoggedIn

  const savePairing = () => {
    const id = pairInput.trim()
    if (!id) return
    if (typeof window !== 'undefined') window.localStorage.setItem('reader_id', id)
    setReaderId(id)
    setError(null)
  }

  const forgetPairing = () => {
    if (typeof window !== 'undefined') window.localStorage.removeItem('reader_id')
    setReaderId('')
  }

  useEffect(() => {
    if (!readerId) return

    let cancelled = false
    setStatus('waiting')
    setError(null)

    const claim = () =>
      apiFetch('/api/reader/claim', {
        method: 'POST',
        json: { reader_id: readerId },
      })

    const run = async () => {
      try {
        await claim()
      } catch {
        if (!cancelled) setError('ไม่สามารถเชื่อมต่อเครื่องอ่านบัตรได้')
      }

      while (!cancelled) {
        try {
          const res = await apiFetch<WaitResponse | null>('/api/reader/wait')
          if (cancelled) return
          if (res && res.login_token) {
            await apiFetch('/api/auth/card-login', {
              method: 'POST',
              json: { login_token: res.login_token },
            })
            if (cancelled) return
            await refresh()
            if (cancelled) return
            setStatus('success')
            setError(null)
            onLoggedInRef.current()
            return
          }
          // 204 → null body → the long-poll timed out with no tap; re-poll.
          setError(null)
        } catch (err) {
          if (cancelled) return
          if (err instanceof ApiError && err.status === 401) {
            // reader_claim cookie missing/expired → re-pair the browser and
            // keep polling. Not surfaced as an error (it self-heals).
            try {
              await claim()
            } catch {
              /* retry on the next tick */
            }
          } else {
            setError('เกิดข้อผิดพลาด กรุณาแตะบัตรอีกครั้ง')
            // Back off briefly so a persistent failure doesn't hot-loop.
            await new Promise((resolve) => setTimeout(resolve, 1500))
          }
        }
      }
    }

    void run()
    return () => {
      cancelled = true
    }
  }, [readerId, refresh])

  // ── No reader paired yet: show the one-time pairing input. ──
  if (!readerId) {
    return (
      <div className="space-y-3">
        <p className="text-[12px] text-textMuted">
          จับคู่เครื่องนี้กับเครื่องอ่านบัตรก่อนใช้งาน (ตั้งค่าครั้งเดียวต่อเครื่อง)
        </p>
        <div className="space-y-1">
          <label
            htmlFor="reader-pair"
            className="block text-[12px] font-medium text-text"
          >
            รหัสเครื่องอ่าน (reader id)
          </label>
          <input
            id="reader-pair"
            type="text"
            value={pairInput}
            onChange={(event) => setPairInput(event.target.value)}
            autoComplete="off"
            className="w-full h-9 px-2 bg-panel border border-border focus:border-brand-500 focus:outline-none text-[13px] text-text"
          />
        </div>
        <button
          type="button"
          onClick={savePairing}
          disabled={pairInput.trim().length === 0}
          className="w-full h-9 flex items-center justify-center gap-1.5 bg-brand-600 text-white hover:bg-brand-700 disabled:opacity-50 transition-colors text-[13px] font-medium"
        >
          จับคู่เครื่องอ่าน
        </button>
      </div>
    )
  }

  // ── Paired: waiting for a tap (or success). ──
  return (
    <div className="space-y-3">
      <div className="flex flex-col items-center justify-center gap-2 py-4 text-center">
        {status === 'success' ? (
          <>
            <CheckCircle2 className="w-8 h-8 text-success" />
            <p className="text-[13px] font-medium text-text">เข้าสู่ระบบสำเร็จ</p>
          </>
        ) : (
          <>
            <div className="relative">
              <CreditCard className="w-8 h-8 text-brand-500" />
              <Loader2 className="w-4 h-4 animate-spin text-brand-500 absolute -right-2 -bottom-1" />
            </div>
            <p className="text-[13px] font-medium text-text">แตะบัตรของคุณ…</p>
            <p className="text-[11px] text-textMuted">
              เครื่องอ่าน: <span className="font-mono">{readerId}</span>
            </p>
          </>
        )}
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

      <button
        type="button"
        onClick={forgetPairing}
        className="w-full text-[11px] text-textMuted hover:text-text underline underline-offset-2"
      >
        เปลี่ยนเครื่องอ่าน
      </button>
    </div>
  )
}
