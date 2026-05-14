/**
 * @jest-environment jsdom
 *
 * Track G5 — UI smoke tests for the coupon issuing modal. Mirrors
 * the RefundPaymentModal test shape so the same `global.fetch` +
 * lucide-react + AuthContext mocks work without additional wiring.
 */

import { render, screen, fireEvent, waitFor } from '@testing-library/react'
import IssueCouponModal from '@/components/modals/IssueCouponModal'

jest.mock('lucide-react', () => ({
  X: () => <span data-testid="x-icon">X</span>,
  Loader2: () => <span data-testid="loader-icon">Loader</span>,
  Ticket: () => <span data-testid="ticket-icon">Ticket</span>,
  Calendar: () => <span data-testid="calendar-icon">Calendar</span>,
  User: () => <span data-testid="user-icon">User</span>,
  Printer: () => <span data-testid="printer-icon">Printer</span>,
}))

// `useBranchFetch` returns the global fetch by default; the modal
// only needs it to be callable and produce a JSON response.
jest.mock('@/lib/use-branch-fetch', () => ({
  useBranchFetch: () => (...args: [string, RequestInit?]) =>
    (global.fetch as jest.Mock)(...args),
}))

const mockHasPermission = jest.fn(() => true)
jest.mock('@/contexts/AuthContext', () => ({
  useAuth: () => ({
    user: null,
    permissions: [],
    loading: false,
    error: null,
    login: jest.fn(),
    logout: jest.fn(),
    refresh: jest.fn(),
    hasPermission: mockHasPermission,
  }),
}))

describe('IssueCouponModal Component', () => {
  const mockOnClose = jest.fn()
  const mockOnSuccess = jest.fn()

  const defaultProps = {
    isOpen: true,
    onClose: mockOnClose,
    onSuccess: mockOnSuccess,
  }

  beforeEach(() => {
    jest.clearAllMocks()
    global.fetch = jest.fn()
    mockHasPermission.mockReturnValue(true)
  })

  describe('Rendering', () => {
    test('does not render when isOpen is false', () => {
      render(<IssueCouponModal {...defaultProps} isOpen={false} />)
      expect(screen.queryByText('ออกคูปอง')).not.toBeInTheDocument()
    })

    test('renders title and form fields', () => {
      render(<IssueCouponModal {...defaultProps} />)
      expect(
        screen.getByRole('heading', { name: 'ออกคูปอง' }),
      ).toBeInTheDocument()
      // Value input pre-fills with the default (0.00 → 0).
      const valueInput = screen.getByPlaceholderText('0.00')
      expect(valueInput).toHaveValue(0)
      expect(
        screen.getByPlaceholderText('ชื่อพนักงาน (ค่าเริ่มต้น: Admin)'),
      ).toBeInTheDocument()
    })

    test('shows folio context when forCinNo is provided', () => {
      render(
        <IssueCouponModal
          {...defaultProps}
          forCinNo="CH26-005228"
        />,
      )
      expect(screen.getByText('CH26-005228')).toBeInTheDocument()
      expect(screen.getByText('ผูกกับการเข้าพัก')).toBeInTheDocument()
    })

    test('uses defaultValue when supplied', () => {
      render(<IssueCouponModal {...defaultProps} defaultValue={200} />)
      const valueInput = screen.getByPlaceholderText('0.00')
      expect(valueInput).toHaveValue(200)
    })
  })

  describe('Permission gating (Track G7)', () => {
    test('renders nothing when coupon.issue permission is missing', () => {
      mockHasPermission.mockReturnValue(false)
      render(<IssueCouponModal {...defaultProps} />)
      expect(screen.queryByText('ออกคูปอง')).not.toBeInTheDocument()
    })
  })

  describe('Submission', () => {
    test('POSTs to /api/new/coupons and shows the issued code on success', async () => {
      ;(global.fetch as jest.Mock).mockResolvedValueOnce({
        ok: true,
        json: async () => ({
          success: true,
          id: 1,
          aggregateId: '00000000-0000-0000-0000-000000000001',
          code: 'COUP-ABCD1234',
          message: 'Coupon COUP-ABCD1234 issued',
        }),
      })

      render(
        <IssueCouponModal
          {...defaultProps}
          forCinNo="CH26-005228"
          defaultValue={0}
        />,
      )

      fireEvent.click(screen.getByRole('button', { name: /ยืนยันการออกคูปอง/ }))

      await waitFor(() => {
        expect(global.fetch).toHaveBeenCalledWith(
          '/api/new/coupons',
          expect.objectContaining({
            method: 'POST',
            headers: { 'Content-Type': 'application/json' },
          }),
        )
      })

      // The successful response replaces the form with a code +
      // print button. The success callback also fires.
      expect(mockOnSuccess).toHaveBeenCalledWith('COUP-ABCD1234')
      expect(await screen.findByText('COUP-ABCD1234')).toBeInTheDocument()
      expect(
        screen.getByRole('button', { name: /พิมพ์คูปอง/ }),
      ).toBeInTheDocument()
    })

    test('surfaces the server error message on failure', async () => {
      ;(global.fetch as jest.Mock).mockResolvedValueOnce({
        ok: false,
        json: async () => ({
          success: false,
          error: 'value must be a finite non-negative number',
        }),
      })

      // `defaultValue={0}` keeps the input HTML5-valid (`min="0"`)
      // so the click reaches the JS handler; the failure surfaces
      // from the mocked fetch response, exercising the server-error
      // code path the test name advertises.
      render(<IssueCouponModal {...defaultProps} defaultValue={0} />)

      fireEvent.click(screen.getByRole('button', { name: /ยืนยันการออกคูปอง/ }))
      expect(
        await screen.findByText('value must be a finite non-negative number'),
      ).toBeInTheDocument()
      expect(global.fetch).toHaveBeenCalledTimes(1)
    })
  })
})
