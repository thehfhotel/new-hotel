/**
 * @jest-environment jsdom
 *
 * Track G2 / T4 CRIT-1 — UI smoke tests for the refund modal. Mirrors
 * the PaymentModal test shape so the same `global.fetch` + lucide-react
 * mock pattern works here without additional wiring.
 */

import { render, screen, fireEvent, waitFor } from '@testing-library/react'
import RefundPaymentModal from '@/components/modals/RefundPaymentModal'

jest.mock('lucide-react', () => ({
  X: () => <span data-testid="x-icon">X</span>,
  Loader2: () => <span data-testid="loader-icon">Loader</span>,
  DollarSign: () => <span data-testid="dollar-icon">DollarSign</span>,
  FileText: () => <span data-testid="file-text-icon">FileText</span>,
  Undo2: () => <span data-testid="undo-icon">Undo</span>,
}))

// Track G7 — the modal pulls `hasPermission` from `useAuth`. We mock
// the entire context here so the tests can dial the permission state up
// or down without spinning up a real AuthProvider + /api/auth/me stub.
// `mockHasPermission` is reassigned per-test via `mockReturnValue` —
// see the "Permission gating" describe block below.
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

describe('RefundPaymentModal Component', () => {
  const mockOnClose = jest.fn()
  const mockOnSuccess = jest.fn()

  const defaultProps = {
    isOpen: true,
    onClose: mockOnClose,
    paymentId: 42,
    originalAmount: 1000,
    alreadyRefunded: 0,
    originalMethodLabel: 'เงินสด',
    onSuccess: mockOnSuccess,
  }

  beforeEach(() => {
    jest.clearAllMocks()
    global.fetch = jest.fn()
    // Default the permission gate to "granted" so the pre-G7 tests
    // continue to exercise the happy path. The G7 gate-specific tests
    // override this per-test.
    mockHasPermission.mockReturnValue(true)
  })

  describe('Rendering', () => {
    test('does not render when isOpen is false', () => {
      render(<RefundPaymentModal {...defaultProps} isOpen={false} />)
      expect(screen.queryByText('คืนเงิน')).not.toBeInTheDocument()
    })

    test('renders title and payment id in the header', () => {
      render(<RefundPaymentModal {...defaultProps} />)
      expect(screen.getByRole('heading', { name: 'คืนเงิน' })).toBeInTheDocument()
      expect(screen.getByText('Refund Payment #42')).toBeInTheDocument()
    })

    test('shows original / refunded / remaining summary', () => {
      render(
        <RefundPaymentModal
          {...defaultProps}
          originalAmount={1000}
          alreadyRefunded={250}
        />,
      )
      expect(screen.getByText('ยอดเดิม')).toBeInTheDocument()
      expect(screen.getByText('คืนแล้ว')).toBeInTheDocument()
      expect(screen.getByText('คืนได้อีก')).toBeInTheDocument()
      // Remaining 1000 - 250 = 750
      expect(screen.getByText('฿750.00')).toBeInTheDocument()
    })

    test('pre-fills amount with the remaining refundable balance', () => {
      render(
        <RefundPaymentModal
          {...defaultProps}
          originalAmount={1000}
          alreadyRefunded={300}
        />,
      )
      const amountInput = screen.getByPlaceholderText('0.00')
      // Remaining = 700.00
      expect(amountInput).toHaveValue(700)
    })

    test('disables the submit button when no remaining refundable balance', () => {
      render(
        <RefundPaymentModal
          {...defaultProps}
          originalAmount={1000}
          alreadyRefunded={1000}
        />,
      )
      const submitButton = screen.getByRole('button', { name: 'ยืนยันการคืนเงิน' })
      expect(submitButton).toBeDisabled()
      expect(screen.getByText('ไม่มียอดคงเหลือให้คืน')).toBeInTheDocument()
    })
  })

  describe('Client-side validation', () => {
    test('rejects refund larger than remaining refundable balance', async () => {
      render(
        <RefundPaymentModal
          {...defaultProps}
          originalAmount={500}
          alreadyRefunded={0}
        />,
      )
      const amountInput = screen.getByPlaceholderText('0.00')
      fireEvent.change(amountInput, { target: { value: '750' } })

      const submitButton = screen.getByRole('button', { name: 'ยืนยันการคืนเงิน' })
      fireEvent.click(submitButton)

      await waitFor(() => {
        expect(screen.getByText(/ต้องไม่เกินยอดคงเหลือ/)).toBeInTheDocument()
      })
      expect(global.fetch).not.toHaveBeenCalled()
    })
  })

  describe('API Submission', () => {
    test('POSTs to /api/new/payments/:id/refund with amount and reason', async () => {
      ;(global.fetch as jest.Mock).mockResolvedValue({
        ok: true,
        json: () => Promise.resolve({ success: true }),
      })

      render(<RefundPaymentModal {...defaultProps} />)

      const amountInput = screen.getByPlaceholderText('0.00')
      fireEvent.change(amountInput, { target: { value: '250' } })

      const reasonField = screen.getByPlaceholderText(/ลูกค้าร้องเรียน/)
      fireEvent.change(reasonField, { target: { value: 'wrong room rate' } })

      const submitButton = screen.getByRole('button', { name: 'ยืนยันการคืนเงิน' })
      fireEvent.click(submitButton)

      await waitFor(() => {
        expect(global.fetch).toHaveBeenCalledWith(
          '/api/new/payments/42/refund?branch=hfhotel',
          {
            method: 'POST',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify({ amount: 250, reason: 'wrong room rate' }),
          },
        )
      })
    })

    test('calls onSuccess + onClose after a successful refund', async () => {
      ;(global.fetch as jest.Mock).mockResolvedValue({
        ok: true,
        json: () => Promise.resolve({ success: true }),
      })

      render(<RefundPaymentModal {...defaultProps} />)

      const submitButton = screen.getByRole('button', { name: 'ยืนยันการคืนเงิน' })
      fireEvent.click(submitButton)

      await waitFor(() => {
        expect(mockOnSuccess).toHaveBeenCalled()
        expect(mockOnClose).toHaveBeenCalled()
      })
    })

    test('hides the modal entirely when the user lacks payment.refund', () => {
      // Track G7 — non-cashier users should never see the refund UI.
      mockHasPermission.mockImplementation((key: string) => key !== 'payment.refund')
      const { container } = render(<RefundPaymentModal {...defaultProps} />)

      expect(screen.queryByRole('heading', { name: 'คืนเงิน' })).not.toBeInTheDocument()
      // Render falls through to `null` — wrapper carries no DOM children.
      expect(container.firstChild).toBeNull()
    })

    test('queries hasPermission with the payment.refund key', () => {
      render(<RefundPaymentModal {...defaultProps} />)
      expect(mockHasPermission).toHaveBeenCalledWith('payment.refund')
    })

    test('surfaces the API error message on failure', async () => {
      ;(global.fetch as jest.Mock).mockResolvedValue({
        ok: false,
        json: () =>
          Promise.resolve({ success: false, error: 'payment is already voided' }),
      })

      render(<RefundPaymentModal {...defaultProps} />)

      const submitButton = screen.getByRole('button', { name: 'ยืนยันการคืนเงิน' })
      fireEvent.click(submitButton)

      await waitFor(() => {
        expect(screen.getByText('payment is already voided')).toBeInTheDocument()
      })
      expect(mockOnSuccess).not.toHaveBeenCalled()
    })
  })
})
