/**
 * @jest-environment jsdom
 */

/**
 * HF Ville gating in the REAL shells — both the classic Sidebar and the new
 * V2Shell must disable the "HF Ville" branch control when villeAvailable is
 * false, and enable it when true. This guards the user-visible "HF Ville not
 * loading" behavior in both UIs at once.
 */

import { render, screen, waitFor } from '@testing-library/react'
import { BranchProvider } from '@/contexts/BranchContext'
import Sidebar from '@/components/Sidebar'
import V2Shell from '@/components/v2/V2Shell'

jest.mock('next/navigation', () => ({
  usePathname: () => '/',
  useRouter: () => ({ replace: jest.fn(), push: jest.fn() }),
}))

// Auth chrome is irrelevant to branch gating; stub it so we don't need /api/auth/me.
jest.mock('@/contexts/AuthContext', () => ({
  useAuth: () => ({ user: null, logout: jest.fn() }),
}))

// SkinToggle pulls in SkinContext; not under test here.
jest.mock('@/components/SkinToggle', () => ({
  __esModule: true,
  default: () => null,
}))

function modeResponse(villeAvailable: boolean): Response {
  return {
    ok: true,
    status: 200,
    json: () => Promise.resolve({ success: true, villeAvailable }),
    text: () => Promise.resolve(JSON.stringify({ success: true, villeAvailable })),
  } as unknown as Response
}

describe('HF Ville gating — classic Sidebar', () => {
  let fetchMock: jest.Mock
  beforeEach(() => {
    fetchMock = jest.fn()
    global.fetch = fetchMock as unknown as typeof fetch
    localStorage.clear()
    localStorage.setItem('sidebar-collapsed', 'false') // keep branch labels visible
  })

  test('HF Ville button is disabled when villeAvailable=false', async () => {
    fetchMock.mockResolvedValue(modeResponse(false))
    render(
      <BranchProvider>
        <Sidebar />
      </BranchProvider>,
    )
    await waitFor(() => expect(fetchMock).toHaveBeenCalledWith('/api/mode'))
    expect(screen.getByRole('button', { name: 'HF Ville' })).toBeDisabled()
  })

  test('HF Ville button is enabled when villeAvailable=true', async () => {
    fetchMock.mockResolvedValue(modeResponse(true))
    render(
      <BranchProvider>
        <Sidebar />
      </BranchProvider>,
    )
    await waitFor(() => expect(screen.getByRole('button', { name: 'HF Ville' })).not.toBeDisabled())
  })

  test('still renders the new "/v2" doorway (no classic regression)', async () => {
    fetchMock.mockResolvedValue(modeResponse(true))
    render(
      <BranchProvider>
        <Sidebar />
      </BranchProvider>,
    )
    expect(screen.getByRole('link', { name: /ดีไซน์ใหม่/ })).toHaveAttribute('href', '/v2')
  })
})

describe('HF Ville gating — new V2Shell', () => {
  let fetchMock: jest.Mock
  beforeEach(() => {
    fetchMock = jest.fn()
    global.fetch = fetchMock as unknown as typeof fetch
    localStorage.clear()
  })

  test('all HF Ville controls are disabled when villeAvailable=false', async () => {
    fetchMock.mockResolvedValue(modeResponse(false))
    render(
      <BranchProvider>
        <V2Shell>
          <div>content</div>
        </V2Shell>
      </BranchProvider>,
    )
    await waitFor(() => expect(fetchMock).toHaveBeenCalledWith('/api/mode'))
    const villeButtons = screen.getAllByRole('button', { name: 'HF Ville' })
    expect(villeButtons.length).toBeGreaterThan(0)
    villeButtons.forEach((b) => expect(b).toBeDisabled())
  })

  test('HF Ville controls become enabled when villeAvailable=true', async () => {
    fetchMock.mockResolvedValue(modeResponse(true))
    render(
      <BranchProvider>
        <V2Shell>
          <div>content</div>
        </V2Shell>
      </BranchProvider>,
    )
    await waitFor(() => {
      const villeButtons = screen.getAllByRole('button', { name: 'HF Ville' })
      villeButtons.forEach((b) => expect(b).not.toBeDisabled())
    })
  })
})
