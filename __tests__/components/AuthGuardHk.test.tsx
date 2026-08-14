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
 */

import { render, screen, waitFor } from '@testing-library/react'

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
const { AuthProvider } =
  require('@/contexts/AuthContext') as typeof import('@/contexts/AuthContext')

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
    '%s renders children and never bounces to /login',
    async (path) => {
      mockPathname.mockReturnValue(path)

      render(
        <AuthProvider>
          <div data-testid="hk-content">maid surface</div>
        </AuthProvider>,
      )

      expect(screen.getByTestId('hk-content')).toBeInTheDocument()

      // Let the anonymous /me → cf-login chain settle: the guard must still
      // be showing the maid her page once the session probe has concluded
      // "logged out", and must not have queued a redirect.
      await waitFor(() => expect(fetchMock).toHaveBeenCalledTimes(2))
      expect(fetchMock.mock.calls.map((c) => c[0])).toEqual([
        '/api/auth/me',
        '/api/auth/cf-login',
      ])
      expect(screen.getByTestId('hk-content')).toBeInTheDocument()
      expect(mockReplace).not.toHaveBeenCalled()
    },
  )

  test('/hk paints even while the session probe is still in flight', async () => {
    // The probe never settles (a maid's /api/auth/me can be intercepted by
    // the root Access app on her device). /hk owns its own identity, so its
    // paint must not be gated on the AuthProvider `loading` window.
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

  test('control: a protected path still redirects and renders nothing', async () => {
    mockPathname.mockReturnValue('/v2/rooms')

    render(
      <AuthProvider>
        <div data-testid="protected-content">front desk</div>
      </AuthProvider>,
    )

    await waitFor(() =>
      expect(mockReplace).toHaveBeenCalledWith(
        `/login?redirect=${encodeURIComponent('/v2/rooms')}`,
      ),
    )
    expect(screen.queryByTestId('protected-content')).not.toBeInTheDocument()
  })
})
