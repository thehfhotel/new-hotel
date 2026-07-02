/**
 * @jest-environment jsdom
 */

import { act, render, screen, waitFor } from '@testing-library/react'
import { AuthProvider, useAuth, type UserDto } from '@/contexts/AuthContext'

// next/navigation needs to be mocked because the real module ties into
// Next's router runtime which isn't bootstrapped in jsdom.
const mockReplace = jest.fn()
const mockPathname = jest.fn(() => '/')

jest.mock('next/navigation', () => ({
  useRouter: () => ({ replace: mockReplace, push: jest.fn() }),
  usePathname: () => mockPathname(),
  useSearchParams: () => new URLSearchParams(),
}))

const SAMPLE_USER: UserDto = {
  user_id: 1,
  username: 'admin',
  role: 'admin',
  active: true,
  created_at: '2026-01-01T00:00:00Z',
  last_login_at: null,
}

function makeFetchResponse(status: number, body: unknown): Response {
  return {
    ok: status >= 200 && status < 300,
    status,
    statusText: status === 200 ? 'OK' : 'Error',
    text: () => Promise.resolve(JSON.stringify(body)),
  } as unknown as Response
}

function AuthConsumer() {
  const { user, loading, error, login, logout } = useAuth()
  return (
    <div>
      <div data-testid="loading">{loading ? 'loading' : 'ready'}</div>
      <div data-testid="user">{user ? user.username : 'anon'}</div>
      <div data-testid="error">{error ?? 'no-error'}</div>
      <button onClick={() => login('admin', 'pw').catch(() => {})}>login</button>
      <button onClick={() => logout()}>logout</button>
    </div>
  )
}

describe('AuthContext', () => {
  let fetchMock: jest.Mock

  beforeEach(() => {
    fetchMock = jest.fn()
    global.fetch = fetchMock as unknown as typeof fetch
    mockReplace.mockReset()
    mockPathname.mockReturnValue('/')
  })

  test('hydrates user from /api/auth/me on mount', async () => {
    fetchMock.mockResolvedValueOnce(
      makeFetchResponse(200, { user: SAMPLE_USER }),
    )

    render(
      <AuthProvider>
        <AuthConsumer />
      </AuthProvider>,
    )

    await waitFor(() => {
      expect(screen.getByTestId('loading')).toHaveTextContent('ready')
    })

    expect(screen.getByTestId('user')).toHaveTextContent('admin')
    expect(fetchMock).toHaveBeenCalledWith(
      '/api/auth/me',
      expect.objectContaining({ credentials: 'include' }),
    )
  })

  test('treats 401 from /api/auth/me as anonymous (not error)', async () => {
    fetchMock.mockResolvedValueOnce(
      makeFetchResponse(401, { error: 'unauthenticated' }),
    )
    // CF Access auto-login attempt fires once on 401; reject it so the
    // provider settles on the anonymous state.
    fetchMock.mockResolvedValueOnce(
      makeFetchResponse(401, { error: 'cf_email_not_mapped' }),
    )

    render(
      <AuthProvider>
        <AuthConsumer />
      </AuthProvider>,
    )

    await waitFor(() => {
      expect(screen.getByTestId('loading')).toHaveTextContent('ready')
    })

    expect(screen.getByTestId('user')).toHaveTextContent('anon')
    expect(screen.getByTestId('error')).toHaveTextContent('no-error')
    // The silent CF Access fallback must have been attempted exactly once.
    expect(fetchMock).toHaveBeenCalledWith(
      '/api/auth/cf-login',
      expect.objectContaining({ method: 'POST', credentials: 'include' }),
    )
    expect(fetchMock).toHaveBeenCalledTimes(2)
  })

  test('auto-logs-in via /api/auth/cf-login when CF Access maps the user', async () => {
    // initial /me → 401 (no app session yet)
    fetchMock.mockResolvedValueOnce(
      makeFetchResponse(401, { error: 'unauthenticated' }),
    )
    // cf-login → 200 (backend verified the CF JWT and minted a session)
    fetchMock.mockResolvedValueOnce(
      makeFetchResponse(200, { user: SAMPLE_USER, permissions: [] }),
    )
    // re-fetched /me → 200 with the new session
    fetchMock.mockResolvedValueOnce(
      makeFetchResponse(200, { user: SAMPLE_USER, permissions: [] }),
    )

    render(
      <AuthProvider>
        <AuthConsumer />
      </AuthProvider>,
    )

    await waitFor(() =>
      expect(screen.getByTestId('loading')).toHaveTextContent('ready'),
    )

    expect(screen.getByTestId('user')).toHaveTextContent('admin')
    expect(screen.getByTestId('error')).toHaveTextContent('no-error')
    expect(fetchMock.mock.calls.map((c) => c[0])).toEqual([
      '/api/auth/me',
      '/api/auth/cf-login',
      '/api/auth/me',
    ])
  })

  test('login() POSTs credentials and stores returned user', async () => {
    // initial /me → 401 (no session)
    fetchMock.mockResolvedValueOnce(
      makeFetchResponse(401, { error: 'unauthenticated' }),
    )
    // cf-login auto-attempt → 401 (not mapped; silent fallback)
    fetchMock.mockResolvedValueOnce(
      makeFetchResponse(401, { error: 'cf_email_not_mapped' }),
    )
    // login → 200
    fetchMock.mockResolvedValueOnce(
      makeFetchResponse(200, { user: SAMPLE_USER }),
    )

    render(
      <AuthProvider>
        <AuthConsumer />
      </AuthProvider>,
    )

    await waitFor(() =>
      expect(screen.getByTestId('loading')).toHaveTextContent('ready'),
    )

    await act(async () => {
      screen.getByText('login').click()
    })

    await waitFor(() =>
      expect(screen.getByTestId('user')).toHaveTextContent('admin'),
    )

    const loginCall = fetchMock.mock.calls[2]
    expect(loginCall[0]).toBe('/api/auth/login')
    expect(loginCall[1]).toMatchObject({
      method: 'POST',
      credentials: 'include',
    })
    expect(loginCall[1].headers).toMatchObject({
      'Content-Type': 'application/json',
    })
    expect(JSON.parse(loginCall[1].body)).toEqual({
      username: 'admin',
      password: 'pw',
    })
  })

  test('login() with invalid credentials surfaces a Thai error message', async () => {
    fetchMock.mockResolvedValueOnce(
      makeFetchResponse(401, { error: 'unauthenticated' }),
    )
    // cf-login auto-attempt → 401 (not mapped; silent fallback)
    fetchMock.mockResolvedValueOnce(
      makeFetchResponse(401, { error: 'cf_email_not_mapped' }),
    )
    fetchMock.mockResolvedValueOnce(
      makeFetchResponse(401, { error: 'invalid_credentials' }),
    )

    render(
      <AuthProvider>
        <AuthConsumer />
      </AuthProvider>,
    )

    await waitFor(() =>
      expect(screen.getByTestId('loading')).toHaveTextContent('ready'),
    )

    await act(async () => {
      screen.getByText('login').click()
    })

    await waitFor(() => {
      expect(screen.getByTestId('error')).toHaveTextContent(
        'ชื่อผู้ใช้หรือรหัสผ่านไม่ถูกต้อง',
      )
    })
    expect(screen.getByTestId('user')).toHaveTextContent('anon')
  })

  test('logout() clears user state even if the network call fails', async () => {
    fetchMock.mockResolvedValueOnce(
      makeFetchResponse(200, { user: SAMPLE_USER }),
    )
    fetchMock.mockRejectedValueOnce(new Error('network down'))

    render(
      <AuthProvider>
        <AuthConsumer />
      </AuthProvider>,
    )

    await waitFor(() =>
      expect(screen.getByTestId('user')).toHaveTextContent('admin'),
    )

    await act(async () => {
      screen.getByText('logout').click()
    })

    await waitFor(() =>
      expect(screen.getByTestId('user')).toHaveTextContent('anon'),
    )
  })
})
