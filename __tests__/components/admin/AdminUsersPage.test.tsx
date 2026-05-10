/**
 * @jest-environment jsdom
 */

import { render, screen, waitFor } from '@testing-library/react'
import AdminUsersPage from '@/app/admin/users/page'
import type { UserDto } from '@/contexts/AuthContext'

// Mock next/navigation — required because next/link bottoms out into router APIs
// that aren't bootstrapped in jsdom.
jest.mock('next/navigation', () => ({
  useRouter: () => ({ replace: jest.fn(), push: jest.fn() }),
  usePathname: () => '/admin/users',
  useSearchParams: () => new URLSearchParams(),
}))

// Mock the auth context so each test pins a specific role.
const mockUseAuth = jest.fn()
jest.mock('@/contexts/AuthContext', () => {
  const actual = jest.requireActual('@/contexts/AuthContext')
  return {
    ...actual,
    useAuth: () => mockUseAuth(),
  }
})

// Mock apiFetch so the test never opens a real socket. PR4's admin
// endpoints all flow through this helper.
const mockApiFetch = jest.fn()
jest.mock('@/lib/api', () => {
  const actual = jest.requireActual('@/lib/api')
  return {
    ...actual,
    apiFetch: (path: string, opts?: unknown) => mockApiFetch(path, opts),
  }
})

const ADMIN_USER: UserDto = {
  user_id: 1,
  username: 'admin',
  role: 'admin',
  active: true,
  created_at: '2026-01-01T00:00:00Z',
  last_login_at: '2026-05-10T12:00:00Z',
}

const RECEPTIONIST_USER: UserDto = {
  user_id: 2,
  username: 'reception',
  role: 'receptionist',
  active: true,
  created_at: '2026-01-01T00:00:00Z',
  last_login_at: null,
}

describe('AdminUsersPage', () => {
  beforeEach(() => {
    mockUseAuth.mockReset()
    mockApiFetch.mockReset()
  })

  test('renders ForbiddenView when current user is a receptionist', async () => {
    mockUseAuth.mockReturnValue({
      user: RECEPTIONIST_USER,
      loading: false,
      error: null,
      login: jest.fn(),
      logout: jest.fn(),
      refresh: jest.fn(),
    })

    render(<AdminUsersPage />)

    expect(screen.getByText('ไม่มีสิทธิ์เข้าถึงหน้านี้')).toBeInTheDocument()
    expect(screen.getByText('กลับสู่หน้าหลัก')).toBeInTheDocument()
    // Critical: never call /api/admin/users for a receptionist.
    expect(mockApiFetch).not.toHaveBeenCalled()
  })

  test('admin sees the table populated from /api/admin/users', async () => {
    mockUseAuth.mockReturnValue({
      user: ADMIN_USER,
      loading: false,
      error: null,
      login: jest.fn(),
      logout: jest.fn(),
      refresh: jest.fn(),
    })
    mockApiFetch.mockResolvedValueOnce({
      users: [ADMIN_USER, RECEPTIONIST_USER],
    })

    render(<AdminUsersPage />)

    await waitFor(() => {
      expect(mockApiFetch).toHaveBeenCalledWith(
        '/api/admin/users',
        undefined,
      )
    })

    // Wait for the receptionist row to render (less ambiguous than 'admin'
    // which appears in the badge AND the role-toggle label).
    await waitFor(() => {
      expect(screen.getByText('reception')).toBeInTheDocument()
    })
    // The admin username appears at least once (table cell + role badge).
    expect(screen.getAllByText('admin').length).toBeGreaterThan(0)
    // And the receptionist badge appears in its row.
    expect(screen.getAllByText(/receptionist/i).length).toBeGreaterThan(0)
  })

  test('renders nothing when there is no logged-in user (AuthGuard handles redirect)', () => {
    mockUseAuth.mockReturnValue({
      user: null,
      loading: false,
      error: null,
      login: jest.fn(),
      logout: jest.fn(),
      refresh: jest.fn(),
    })

    const { container } = render(<AdminUsersPage />)
    expect(container).toBeEmptyDOMElement()
    expect(mockApiFetch).not.toHaveBeenCalled()
  })
})
