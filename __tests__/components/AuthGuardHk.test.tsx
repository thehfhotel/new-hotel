/**
 * @jest-environment jsdom
 *
 * AuthGuard × /hk exemption (V11 maid white-blank regression).
 *
 * The /hk maid surface authenticates via the Cloudflare Access assertion +
 * the backend's hk_access middleware — maids hold NO PMS user account. With
 * NEXT_PUBLIC_AUTH_REQUIRED=true the cookie-session AuthGuard used to treat
 * /hk like any protected page: user null → render null (white blank) →
 * bounce to /login, which in production sits behind the Google-only root
 * Access app the maid can never satisfy. These tests pin the exemption.
 *
 * They also pin its corollary — /hk issues NO /api/auth/* request at all.
 * That probe was dead by construction (a maid has no session to find) and in
 * production it rode the ROOT Access app's `aud`, so the edge answered with a
 * cross-origin login redirect that connect-src refused: a red CSP error on
 * every maid pageload. The zero-request assertions below are two-way — they
 * fail if the probe is reintroduced on /hk specifically OR by a refactor that
 * moves it somewhere unconditional.
 */

import { act, render, screen, waitFor } from '@testing-library/react'

// next/navigation ties into Next's router runtime, which isn't bootstrapped
// in jsdom — same mock shape as __tests__/components/AuthContext.test.tsx.
const mockReplace = jest.fn()
const mockPathname = jest.fn(() => '/')

jest.mock('next/navigation', () => ({
  useRouter: () => ({ replace: mockReplace, push: jest.fn() }),
  usePathname: () => mockPathname(),
  useSearchParams: () => new URLSearchParams(),
}))

// AUTH_REQUIRED is a module-load const read from process.env, so the flag has
// to be set BEFORE the module under test is evaluated. Static `import`
// bindings are hoisted above plain assignments, so the provider comes in via
// `require` afterwards instead (same trick as Rr4ExportPage.test.tsx). Note
// this must NOT be done with jest.isolateModules(): an isolated registry hands
// the provider a *second* copy of react whose hook dispatcher is null, and
// every render dies on `Cannot read properties of null (reading 'useState')`.
const ORIGINAL_AUTH_REQUIRED = process.env.NEXT_PUBLIC_AUTH_REQUIRED
process.env.NEXT_PUBLIC_AUTH_REQUIRED = 'true'
const { AuthProvider, useAuth } =
  require('@/contexts/AuthContext') as typeof import('@/contexts/AuthContext')

/** Surfaces the provider's own state so tests can assert it settles. */
function AuthStateProbe() {
  const { loading } = useAuth()
  return <span data-testid="auth-loading">{String(loading)}</span>
}

/** A 401 from every auth endpoint — exactly a maid's state on /hk. */
function unauthorized(): Response {
  return {
    ok: false,
    status: 401,
    statusText: 'Unauthorized',
    text: () => Promise.resolve(JSON.stringify({ error: 'unauthenticated' })),
  } as unknown as Response
}

describe('AuthGuard with NEXT_PUBLIC_AUTH_REQUIRED=true and no session', () => {
  let fetchMock: jest.Mock

  beforeEach(() => {
    fetchMock = jest.fn().mockResolvedValue(unauthorized())
    global.fetch = fetchMock as unknown as typeof fetch
    mockReplace.mockReset()
    mockPathname.mockReturnValue('/')
  })

  afterAll(() => {
    // Restore, or the next test file loaded in this worker re-evaluates
    // AuthContext with auth enforced and its own cases render nothing.
    if (ORIGINAL_AUTH_REQUIRED === undefined) {
      delete process.env.NEXT_PUBLIC_AUTH_REQUIRED
    } else {
      process.env.NEXT_PUBLIC_AUTH_REQUIRED = ORIGINAL_AUTH_REQUIRED
    }
  })

  test.each(['/hk', '/hk/rooms/12'])(
    '%s renders children, issues NO /api/auth/* request, and never bounces to /login',
    async (path) => {
      mockPathname.mockReturnValue(path)

      render(
        <AuthProvider>
          <div data-testid="hk-content">maid surface</div>
        </AuthProvider>,
      )

      expect(screen.getByTestId('hk-content')).toBeInTheDocument()

      // Drain the mount effects and the microtask queue. The control case
      // below fires its /me → cf-login chain inside exactly this window, so
      // "still zero" here means suppressed — not merely not-yet.
      await act(async () => {
        await Promise.resolve()
      })

      expect(
        fetchMock.mock.calls
          .map((c) => String(c[0]))
          .filter((url) => url.startsWith('/api/auth/')),
      ).toEqual([])
      // Nothing else should be reaching the network from this tree either.
      expect(fetchMock).not.toHaveBeenCalled()
      expect(screen.getByTestId('hk-content')).toBeInTheDocument()
      expect(mockReplace).not.toHaveBeenCalled()
    },
  )

  test('/hk settles the provider out of `loading` without probing', async () => {
    // The /hk pages read no session state, but the provider's state must
    // still be coherent — a permanently-`loading` provider would re-arm the
    // AuthGuard blank the moment anyone removed its /hk opt-out.
    mockPathname.mockReturnValue('/hk')

    render(
      <AuthProvider>
        <AuthStateProbe />
      </AuthProvider>,
    )

    await waitFor(() =>
      expect(screen.getByTestId('auth-loading')).toHaveTextContent('false'),
    )
    expect(fetchMock).not.toHaveBeenCalled()
  })

  test('/hk paints even when an auth round-trip would never settle', async () => {
    // Belt-and-braces on the AuthGuard's own contract: /hk owns its identity,
    // so its paint must never be gated on a session probe — including one a
    // future change might reintroduce that hangs (a maid's /api/auth/me can
    // be intercepted by the root Access app on her device).
    mockPathname.mockReturnValue('/hk')
    fetchMock.mockReturnValue(new Promise<Response>(() => {}))

    render(
      <AuthProvider>
        <div data-testid="hk-content">maid surface</div>
      </AuthProvider>,
    )

    expect(screen.getByTestId('hk-content')).toBeInTheDocument()
    expect(mockReplace).not.toHaveBeenCalled()
  })

  test('control: a protected path still probes, then redirects and renders nothing', async () => {
    mockPathname.mockReturnValue('/v2/rooms')

    render(
      <AuthProvider>
        <div data-testid="protected-content">front desk</div>
      </AuthProvider>,
    )

    // The probe is removed on /hk ONLY — everywhere else it must still run,
    // including the cf-login healing attempt after the anonymous 401.
    await waitFor(() => expect(fetchMock).toHaveBeenCalledTimes(2))
    expect(fetchMock.mock.calls.map((c) => String(c[0]))).toEqual([
      '/api/auth/me',
      '/api/auth/cf-login',
    ])

    await waitFor(() =>
      expect(mockReplace).toHaveBeenCalledWith(
        `/login?redirect=${encodeURIComponent('/v2/rooms')}`,
      ),
    )
    expect(screen.queryByTestId('protected-content')).not.toBeInTheDocument()
  })
})
