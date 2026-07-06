/**
 * @jest-environment jsdom
 */

import { render, screen, waitFor } from '@testing-library/react'
import CardTapLogin from '@/app/login/CardTapLogin'

// AuthContext is mocked — CardTapLogin only needs `refresh()` from it, and we
// don't want to bootstrap the full provider (which fires its own /me + cf-login
// fetches and would pollute the fetch mock sequence).
const mockRefresh = jest.fn().mockResolvedValue(undefined)
jest.mock('@/contexts/AuthContext', () => ({
  useAuth: () => ({ refresh: mockRefresh }),
}))

/** Minimal Response stub matching what `lib/api.apiFetch` reads (text() + ok). */
function makeResponse(status: number, body: unknown): Response {
  return {
    ok: status >= 200 && status < 300,
    status,
    statusText: status === 200 ? 'OK' : 'Error',
    text: () => Promise.resolve(body === null ? '' : JSON.stringify(body)),
  } as unknown as Response
}

describe('CardTapLogin', () => {
  let fetchMock: jest.Mock

  beforeEach(() => {
    fetchMock = jest.fn()
    global.fetch = fetchMock as unknown as typeof fetch
    mockRefresh.mockClear()
    window.localStorage.clear()
  })

  test('claims, waits for a tap, card-logs-in, then calls onLoggedIn', async () => {
    window.localStorage.setItem('reader_id', 'desk-1')
    // claim → wait(deliver) → card-login
    fetchMock
      .mockResolvedValueOnce(makeResponse(200, {})) // POST /api/reader/claim
      .mockResolvedValueOnce(makeResponse(200, { login_token: 'tok-abc' })) // GET /api/reader/wait
      .mockResolvedValueOnce(makeResponse(200, { user: { username: 'nok' } })) // POST /api/auth/card-login

    const onLoggedIn = jest.fn()
    render(<CardTapLogin onLoggedIn={onLoggedIn} />)

    await waitFor(() => expect(onLoggedIn).toHaveBeenCalledTimes(1))

    // Endpoint sequence + credentialed requests.
    expect(fetchMock).toHaveBeenNthCalledWith(
      1,
      '/api/reader/claim',
      expect.objectContaining({ method: 'POST', credentials: 'include' }),
    )
    expect(fetchMock).toHaveBeenNthCalledWith(
      2,
      '/api/reader/wait',
      expect.objectContaining({ credentials: 'include' }),
    )
    expect(fetchMock).toHaveBeenNthCalledWith(
      3,
      '/api/auth/card-login',
      expect.objectContaining({ method: 'POST', credentials: 'include' }),
    )
    // The claim carried the paired reader_id.
    const claimInit = fetchMock.mock.calls[0][1] as RequestInit
    expect(JSON.parse(claimInit.body as string)).toEqual({ reader_id: 'desk-1' })
    // The card-login carried the delivered one-time token.
    const cardInit = fetchMock.mock.calls[2][1] as RequestInit
    expect(JSON.parse(cardInit.body as string)).toEqual({ login_token: 'tok-abc' })
    // AuthContext was rehydrated after the session was minted.
    expect(mockRefresh).toHaveBeenCalledTimes(1)
  })

  test('re-polls after a 204 timeout before the tap lands', async () => {
    window.localStorage.setItem('reader_id', 'desk-2')
    fetchMock
      .mockResolvedValueOnce(makeResponse(200, {})) // claim
      .mockResolvedValueOnce(makeResponse(204, null)) // wait: no tap yet
      .mockResolvedValueOnce(makeResponse(200, { login_token: 'tok-xyz' })) // wait: delivered
      .mockResolvedValueOnce(makeResponse(200, { user: { username: 'nok' } })) // card-login

    const onLoggedIn = jest.fn()
    render(<CardTapLogin onLoggedIn={onLoggedIn} />)

    await waitFor(() => expect(onLoggedIn).toHaveBeenCalledTimes(1))

    // /api/reader/wait was polled twice (204 then the delivering 200).
    const waitCalls = fetchMock.mock.calls.filter((c) => c[0] === '/api/reader/wait')
    expect(waitCalls).toHaveLength(2)
  })

  test('re-claims and keeps polling when wait returns 401 (stale claim)', async () => {
    window.localStorage.setItem('reader_id', 'desk-3')
    fetchMock
      .mockResolvedValueOnce(makeResponse(200, {})) // claim
      .mockResolvedValueOnce(makeResponse(401, { error: 'unauthenticated' })) // wait: stale claim
      .mockResolvedValueOnce(makeResponse(200, {})) // re-claim
      .mockResolvedValueOnce(makeResponse(200, { login_token: 'tok-2' })) // wait: delivered
      .mockResolvedValueOnce(makeResponse(200, { user: { username: 'nok' } })) // card-login

    const onLoggedIn = jest.fn()
    render(<CardTapLogin onLoggedIn={onLoggedIn} />)

    await waitFor(() => expect(onLoggedIn).toHaveBeenCalledTimes(1))

    // Claim fired twice (initial + re-claim after the 401).
    const claimCalls = fetchMock.mock.calls.filter((c) => c[0] === '/api/reader/claim')
    expect(claimCalls).toHaveLength(2)
  })

  test('shows the pairing input and polls nothing when no reader_id is set', async () => {
    // No localStorage reader_id, no NEXT_PUBLIC_READER_ID.
    const onLoggedIn = jest.fn()
    render(<CardTapLogin onLoggedIn={onLoggedIn} />)

    expect(screen.getByLabelText(/เครื่องอ่าน/)).toBeInTheDocument()
    expect(fetchMock).not.toHaveBeenCalled()
    expect(onLoggedIn).not.toHaveBeenCalled()
  })
})
