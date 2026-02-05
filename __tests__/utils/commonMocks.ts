/**
 * Common Mock Implementations for Hotel Management System Tests
 *
 * This file contains reusable mock implementations for:
 * - Lucide React icons
 * - Fetch API
 * - Date utilities
 * - Next.js router and navigation
 */

import React from 'react'

// ============================================================================
// Lucide React Icon Mocks
// ============================================================================

/**
 * Mock implementation for all Lucide React icons used in components.
 * Icons are rendered as spans with data-testid attributes for easy querying.
 *
 * Usage:
 * ```typescript
 * jest.mock('lucide-react', () => mockLucideIcons)
 * ```
 */
export const mockLucideIcons = {
  // Navigation & Actions
  X: () => React.createElement('span', { 'data-testid': 'icon-x' }, '\u00D7'),
  Check: () => React.createElement('span', { 'data-testid': 'icon-check' }, '\u2713'),
  ChevronLeft: () => React.createElement('span', { 'data-testid': 'icon-chevron-left' }, '<'),
  ChevronRight: () => React.createElement('span', { 'data-testid': 'icon-chevron-right' }, '>'),
  ChevronUp: () => React.createElement('span', { 'data-testid': 'icon-chevron-up' }, '^'),
  ChevronDown: () => React.createElement('span', { 'data-testid': 'icon-chevron-down' }, 'v'),

  // UI State
  Loader2: () => React.createElement('span', { 'data-testid': 'icon-loader' }, 'Loading'),
  AlertCircle: () => React.createElement('span', { 'data-testid': 'icon-alert-circle' }, '!'),
  AlertTriangle: () => React.createElement('span', { 'data-testid': 'icon-alert-triangle' }, '!'),

  // Search & Filters
  Search: () => React.createElement('span', { 'data-testid': 'icon-search' }, 'Search'),
  Filter: () => React.createElement('span', { 'data-testid': 'icon-filter' }, 'Filter'),

  // User & People
  User: () => React.createElement('span', { 'data-testid': 'icon-user' }, 'User'),
  Users: () => React.createElement('span', { 'data-testid': 'icon-users' }, 'Users'),
  UserPlus: () => React.createElement('span', { 'data-testid': 'icon-user-plus' }, 'UserPlus'),

  // Accommodation
  Bed: () => React.createElement('span', { 'data-testid': 'icon-bed' }, 'Bed'),
  Building2: () => React.createElement('span', { 'data-testid': 'icon-building' }, 'Building'),
  Home: () => React.createElement('span', { 'data-testid': 'icon-home' }, 'Home'),
  DoorOpen: () => React.createElement('span', { 'data-testid': 'icon-door-open' }, 'DoorOpen'),
  DoorClosed: () => React.createElement('span', { 'data-testid': 'icon-door-closed' }, 'DoorClosed'),

  // Date & Time
  Calendar: () => React.createElement('span', { 'data-testid': 'icon-calendar' }, 'Cal'),
  Clock: () => React.createElement('span', { 'data-testid': 'icon-clock' }, 'Clock'),
  Timer: () => React.createElement('span', { 'data-testid': 'icon-timer' }, 'Timer'),
  Moon: () => React.createElement('span', { 'data-testid': 'icon-moon' }, 'Moon'),

  // Finance & Payment
  CreditCard: () => React.createElement('span', { 'data-testid': 'icon-credit-card' }, 'Card'),
  DollarSign: () => React.createElement('span', { 'data-testid': 'icon-dollar' }, '$'),
  Percent: () => React.createElement('span', { 'data-testid': 'icon-percent' }, '%'),
  Tag: () => React.createElement('span', { 'data-testid': 'icon-tag' }, 'Tag'),

  // Documents & Files
  FileText: () => React.createElement('span', { 'data-testid': 'icon-file-text' }, 'File'),
  FileDown: () => React.createElement('span', { 'data-testid': 'icon-file-down' }, 'Download'),
  Printer: () => React.createElement('span', { 'data-testid': 'icon-printer' }, 'Print'),

  // Actions
  Save: () => React.createElement('span', { 'data-testid': 'icon-save' }, 'Save'),
  Trash2: () => React.createElement('span', { 'data-testid': 'icon-trash' }, 'Trash'),
  Edit: () => React.createElement('span', { 'data-testid': 'icon-edit' }, 'Edit'),
  Plus: () => React.createElement('span', { 'data-testid': 'icon-plus' }, '+'),
  Minus: () => React.createElement('span', { 'data-testid': 'icon-minus' }, '-'),
  MoreVertical: () => React.createElement('span', { 'data-testid': 'icon-more' }, '...'),
  MoreHorizontal: () => React.createElement('span', { 'data-testid': 'icon-more-h' }, '...'),

  // Check-in/Check-out
  LogIn: () => React.createElement('span', { 'data-testid': 'icon-login' }, 'In'),
  LogOut: () => React.createElement('span', { 'data-testid': 'icon-logout' }, 'Out'),

  // Phone & Contact
  Phone: () => React.createElement('span', { 'data-testid': 'icon-phone' }, 'Phone'),
  Mail: () => React.createElement('span', { 'data-testid': 'icon-mail' }, 'Mail'),

  // Housekeeping
  Sparkles: () => React.createElement('span', { 'data-testid': 'icon-sparkles' }, 'Clean'),
  CheckCircle: () => React.createElement('span', { 'data-testid': 'icon-check-circle' }, 'Done'),
  CircleDashed: () => React.createElement('span', { 'data-testid': 'icon-circle-dashed' }, 'Pending'),
  Wrench: () => React.createElement('span', { 'data-testid': 'icon-wrench' }, 'Fix'),

  // Inventory
  Package: () => React.createElement('span', { 'data-testid': 'icon-package' }, 'Pkg'),
  PackagePlus: () => React.createElement('span', { 'data-testid': 'icon-package-plus' }, 'PkgAdd'),
  PackageMinus: () => React.createElement('span', { 'data-testid': 'icon-package-minus' }, 'PkgDel'),
  ArrowLeftRight: () => React.createElement('span', { 'data-testid': 'icon-arrow-lr' }, '<->'),

  // Misc
  RefreshCw: () => React.createElement('span', { 'data-testid': 'icon-refresh' }, 'Refresh'),
  ExternalLink: () => React.createElement('span', { 'data-testid': 'icon-external' }, 'Link'),
  Info: () => React.createElement('span', { 'data-testid': 'icon-info' }, 'i'),
  Settings: () => React.createElement('span', { 'data-testid': 'icon-settings' }, 'Settings'),
  Menu: () => React.createElement('span', { 'data-testid': 'icon-menu' }, 'Menu'),
  Image: () => React.createElement('span', { 'data-testid': 'icon-image' }, 'Img'),
  XCircle: () => React.createElement('span', { 'data-testid': 'icon-x-circle' }, 'XCircle'),
  MinusCircle: () => React.createElement('span', { 'data-testid': 'icon-minus-circle' }, 'MinusCircle'),
  ClipboardList: () => React.createElement('span', { 'data-testid': 'icon-clipboard' }, 'Clipboard'),
  ClipboardCheck: () => React.createElement('span', { 'data-testid': 'icon-clipboard-check' }, 'Checked'),
  Eye: () => React.createElement('span', { 'data-testid': 'icon-eye' }, 'Eye'),
  EyeOff: () => React.createElement('span', { 'data-testid': 'icon-eye-off' }, 'EyeOff'),
}

// ============================================================================
// Fetch Mock Helpers
// ============================================================================

/**
 * Response configuration for mockFetch
 */
export interface MockFetchResponse {
  data?: unknown
  success?: boolean
  error?: string
  status?: number
  ok?: boolean
}

/**
 * Creates a mock fetch function that returns predefined responses
 * based on URL patterns.
 *
 * Usage:
 * ```typescript
 * global.fetch = mockFetch({
 *   '/api/customers': { data: mockCustomers, success: true },
 *   '/api/rooms': { data: mockRooms, success: true },
 * })
 * ```
 *
 * @param responses - A map of URL patterns to response configurations
 * @returns A mock fetch function
 */
export const mockFetch = (
  responses: Record<string, MockFetchResponse>
): jest.Mock => {
  return jest.fn().mockImplementation((url: string) => {
    // Find matching response by URL pattern
    let matchedResponse: MockFetchResponse | undefined

    for (const pattern of Object.keys(responses)) {
      if (url.includes(pattern)) {
        matchedResponse = responses[pattern]
        break
      }
    }

    // Default response if no match found
    if (!matchedResponse) {
      matchedResponse = { data: null, success: false, error: 'Not found', status: 404 }
    }

    const status = matchedResponse.status ?? 200
    const ok = matchedResponse.ok ?? (status >= 200 && status < 300)

    return Promise.resolve({
      ok,
      status,
      json: () =>
        Promise.resolve({
          success: matchedResponse!.success ?? true,
          data: matchedResponse!.data,
          error: matchedResponse!.error,
          message: matchedResponse!.error,
        }),
    })
  })
}

/**
 * Creates a fetch mock that returns an error response
 */
export const mockFetchError = (
  errorMessage = 'Network error',
  status = 500
): jest.Mock => {
  return jest.fn().mockImplementation(() =>
    Promise.resolve({
      ok: false,
      status,
      json: () =>
        Promise.resolve({
          success: false,
          error: errorMessage,
          message: errorMessage,
        }),
    })
  )
}

/**
 * Creates a fetch mock that rejects with a network error
 */
export const mockFetchNetworkError = (errorMessage = 'Network error'): jest.Mock => {
  return jest.fn().mockImplementation(() => Promise.reject(new Error(errorMessage)))
}

// ============================================================================
// Date Mock Helpers
// ============================================================================

/**
 * Standard mock date for testing (February 5, 2026 at 10:00 AM UTC)
 */
export const mockToday = new Date('2026-02-05T10:00:00.000Z')

/**
 * Mock date in Thai Buddhist Era format (2569)
 */
export const mockTodayBuddhistYear = 2569

/**
 * Creates a date string in ISO format for testing
 */
export const createMockDateString = (
  year = 2026,
  month = 1,
  day = 15
): string => {
  return `${year}-${String(month).padStart(2, '0')}-${String(day).padStart(2, '0')}T00:00:00.000Z`
}

/**
 * Creates a date string in API format (YYYY-MM-DD)
 */
export const createMockApiDateString = (
  year = 2026,
  month = 1,
  day = 15
): string => {
  return `${year}-${String(month).padStart(2, '0')}-${String(day).padStart(2, '0')}`
}

/**
 * Converts a date to Buddhist Era year
 */
export const toBuddhistYear = (date: Date): number => {
  return date.getFullYear() + 543
}

/**
 * Creates a mock date range for testing booking scenarios
 */
export const createMockDateRange = (
  startDaysFromNow = 0,
  durationDays = 5
): { checkIn: string; checkOut: string } => {
  const start = new Date(mockToday)
  start.setDate(start.getDate() + startDaysFromNow)

  const end = new Date(start)
  end.setDate(end.getDate() + durationDays)

  return {
    checkIn: createMockApiDateString(
      start.getFullYear(),
      start.getMonth() + 1,
      start.getDate()
    ),
    checkOut: createMockApiDateString(
      end.getFullYear(),
      end.getMonth() + 1,
      end.getDate()
    ),
  }
}

// ============================================================================
// Next.js Router Mocks
// ============================================================================

/**
 * Mock implementation for Next.js useRouter hook
 *
 * Usage:
 * ```typescript
 * jest.mock('next/navigation', () => mockNextNavigation)
 * ```
 */
export const mockNextNavigation = {
  useRouter: () => ({
    push: jest.fn(),
    replace: jest.fn(),
    prefetch: jest.fn(),
    back: jest.fn(),
    forward: jest.fn(),
    refresh: jest.fn(),
  }),
  usePathname: () => '/dashboard',
  useSearchParams: () => new URLSearchParams(),
  useParams: () => ({}),
}

/**
 * Creates a customizable router mock
 */
export const createMockRouter = (overrides?: {
  pathname?: string
  params?: Record<string, string>
  searchParams?: URLSearchParams
}) => ({
  useRouter: () => ({
    push: jest.fn(),
    replace: jest.fn(),
    prefetch: jest.fn(),
    back: jest.fn(),
    forward: jest.fn(),
    refresh: jest.fn(),
  }),
  usePathname: () => overrides?.pathname ?? '/dashboard',
  useSearchParams: () => overrides?.searchParams ?? new URLSearchParams(),
  useParams: () => overrides?.params ?? {},
})

// ============================================================================
// Window & Browser Mocks
// ============================================================================

/**
 * Mock window.print function
 */
export const mockWindowPrint = (): jest.SpyInstance => {
  return jest.spyOn(window, 'print').mockImplementation(() => {})
}

/**
 * Mock window.matchMedia for responsive design testing
 */
export const mockMatchMedia = (matches = false): void => {
  Object.defineProperty(window, 'matchMedia', {
    writable: true,
    value: jest.fn().mockImplementation((query: string) => ({
      matches,
      media: query,
      onchange: null,
      addListener: jest.fn(),
      removeListener: jest.fn(),
      addEventListener: jest.fn(),
      removeEventListener: jest.fn(),
      dispatchEvent: jest.fn(),
    })),
  })
}

/**
 * Mock localStorage
 */
export const mockLocalStorage = (): void => {
  const store: Record<string, string> = {}

  Object.defineProperty(window, 'localStorage', {
    value: {
      getItem: jest.fn((key: string) => store[key] || null),
      setItem: jest.fn((key: string, value: string) => {
        store[key] = value
      }),
      removeItem: jest.fn((key: string) => {
        delete store[key]
      }),
      clear: jest.fn(() => {
        Object.keys(store).forEach((key) => delete store[key])
      }),
    },
    writable: true,
  })
}
