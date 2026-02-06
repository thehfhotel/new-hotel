/**
 * @jest-environment jsdom
 */

import { render, screen, fireEvent, waitFor } from '@testing-library/react'
import PaymentModal from '@/components/modals/PaymentModal'

// Mock lucide-react icons
jest.mock('lucide-react', () => ({
  X: () => <span data-testid="x-icon">X</span>,
  Loader2: () => <span data-testid="loader-icon">Loader</span>,
  CreditCard: () => <span data-testid="credit-card-icon">CreditCard</span>,
  FileText: () => <span data-testid="file-text-icon">FileText</span>,
  DollarSign: () => <span data-testid="dollar-icon">DollarSign</span>,
  Banknote: () => <span data-testid="banknote-icon">Banknote</span>,
  QrCode: () => <span data-testid="qr-icon">QrCode</span>,
}))

describe('PaymentModal Component', () => {
  const mockOnClose = jest.fn()
  const mockOnSuccess = jest.fn()

  const defaultProps = {
    isOpen: true,
    onClose: mockOnClose,
    checkinId: 1,
    totalAmount: 5000,
    totalPaid: 2000,
    onSuccess: mockOnSuccess,
  }

  beforeEach(() => {
    jest.clearAllMocks()
    global.fetch = jest.fn()
  })

  describe('Rendering', () => {
    test('does not render when isOpen is false', () => {
      render(<PaymentModal {...defaultProps} isOpen={false} />)

      expect(screen.queryByText('บันทึกการชำระเงิน')).not.toBeInTheDocument()
    })

    test('renders when isOpen is true', () => {
      render(<PaymentModal {...defaultProps} />)

      expect(screen.getByRole('heading', { name: 'บันทึกการชำระเงิน' })).toBeInTheDocument()
      expect(screen.getByText('Record Payment')).toBeInTheDocument()
    })

    test('displays payment summary with correct amounts', () => {
      render(<PaymentModal {...defaultProps} />)

      expect(screen.getByText('ยอดรวม')).toBeInTheDocument()
      expect(screen.getByText('฿5,000')).toBeInTheDocument()
      expect(screen.getByText('ชำระแล้ว')).toBeInTheDocument()
      expect(screen.getByText('฿2,000')).toBeInTheDocument()
      expect(screen.getByText('คงเหลือ')).toBeInTheDocument()
      expect(screen.getByText('฿3,000')).toBeInTheDocument()
    })

    test('pre-fills amount with balance', () => {
      render(<PaymentModal {...defaultProps} />)

      const amountInput = screen.getByPlaceholderText('0')
      expect(amountInput).toHaveValue(3000) // balance = 5000 - 2000
    })

    test('displays all payment method buttons', () => {
      render(<PaymentModal {...defaultProps} />)

      expect(screen.getByText('เงินสด')).toBeInTheDocument()
      expect(screen.getByText('บัตรเครดิต')).toBeInTheDocument()
      expect(screen.getByText('โอนเงิน')).toBeInTheDocument()
      expect(screen.getByText('QR Code')).toBeInTheDocument()
    })
  })

  describe('Payment Method Selection', () => {
    test('cash is selected by default', () => {
      render(<PaymentModal {...defaultProps} />)

      const cashButton = screen.getByText('เงินสด').closest('button')
      expect(cashButton).toHaveClass('border-red-500')
    })

    test('clicking credit card changes selection and shows reference field', () => {
      render(<PaymentModal {...defaultProps} />)

      fireEvent.click(screen.getByText('บัตรเครดิต'))

      const creditButton = screen.getByText('บัตรเครดิต').closest('button')
      expect(creditButton).toHaveClass('border-red-500')
      expect(screen.getByPlaceholderText('1234')).toBeInTheDocument()
    })

    test('clicking transfer shows reference field', () => {
      render(<PaymentModal {...defaultProps} />)

      fireEvent.click(screen.getByText('โอนเงิน'))

      expect(screen.getByPlaceholderText('เลขอ้างอิงการโอน')).toBeInTheDocument()
    })

    test('cash does not show reference field', () => {
      render(<PaymentModal {...defaultProps} />)

      expect(screen.queryByPlaceholderText('1234')).not.toBeInTheDocument()
      expect(screen.queryByPlaceholderText('เลขอ้างอิงการโอน')).not.toBeInTheDocument()
    })
  })

  describe('Form Interactions', () => {
    test('balance button fills amount with remaining balance', () => {
      render(<PaymentModal {...defaultProps} totalPaid={1000} />)

      const balanceButton = screen.getByText(/ใช้ยอดคงเหลือ/)
      fireEvent.click(balanceButton)

      const amountInput = screen.getByPlaceholderText('0')
      expect(amountInput).toHaveValue(4000) // 5000 - 1000
    })

    test('can enter notes', () => {
      render(<PaymentModal {...defaultProps} />)

      const notesInput = screen.getByPlaceholderText('หมายเหตุเพิ่มเติม...')
      fireEvent.change(notesInput, { target: { value: 'ชำระงวดแรก' } })

      expect(notesInput).toHaveValue('ชำระงวดแรก')
    })
  })

  describe('Form Validation', () => {
    test('amount input is required', () => {
      render(<PaymentModal {...defaultProps} />)

      const amountInput = screen.getByPlaceholderText('0')
      expect(amountInput).toBeRequired()
    })

    test('amount input has min attribute', () => {
      render(<PaymentModal {...defaultProps} />)

      const amountInput = screen.getByPlaceholderText('0')
      expect(amountInput).toHaveAttribute('min', '0')
    })

    test('amount input accepts decimal values', () => {
      render(<PaymentModal {...defaultProps} />)

      const amountInput = screen.getByPlaceholderText('0')
      expect(amountInput).toHaveAttribute('step', '0.01')
    })
  })

  describe('API Submission', () => {
    test('submits correct payload for cash payment', async () => {
      ;(global.fetch as jest.Mock).mockResolvedValue({
        ok: true,
        json: () => Promise.resolve({ success: true }),
      })

      render(<PaymentModal {...defaultProps} />)

      const amountInput = screen.getByPlaceholderText('0')
      fireEvent.change(amountInput, { target: { value: '1500' } })

      const submitButton = screen.getByRole('button', { name: 'บันทึกการชำระเงิน' })
      fireEvent.click(submitButton)

      await waitFor(() => {
        expect(global.fetch).toHaveBeenCalledWith('/api/new/checkins/1/payments', {
          method: 'POST',
          headers: { 'Content-Type': 'application/json' },
          body: JSON.stringify({
            amount: 1500,
            method: 'cash',
            reference: undefined,
            notes: undefined,
          }),
        })
      })
    })

    test('submits correct payload for credit card with reference', async () => {
      ;(global.fetch as jest.Mock).mockResolvedValue({
        ok: true,
        json: () => Promise.resolve({ success: true }),
      })

      render(<PaymentModal {...defaultProps} />)

      // Select credit card
      fireEvent.click(screen.getByText('บัตรเครดิต'))

      // Enter amount
      const amountInput = screen.getByPlaceholderText('0')
      fireEvent.change(amountInput, { target: { value: '2000' } })

      // Enter reference
      const refInput = screen.getByPlaceholderText('1234')
      fireEvent.change(refInput, { target: { value: '5678' } })

      const submitButton = screen.getByRole('button', { name: 'บันทึกการชำระเงิน' })
      fireEvent.click(submitButton)

      await waitFor(() => {
        expect(global.fetch).toHaveBeenCalledWith('/api/new/checkins/1/payments', {
          method: 'POST',
          headers: { 'Content-Type': 'application/json' },
          body: JSON.stringify({
            amount: 2000,
            method: 'credit',
            reference: '5678',
            notes: undefined,
          }),
        })
      })
    })

    test('calls onSuccess and onClose after successful submission', async () => {
      ;(global.fetch as jest.Mock).mockResolvedValue({
        ok: true,
        json: () => Promise.resolve({ success: true }),
      })

      render(<PaymentModal {...defaultProps} />)

      const submitButton = screen.getByRole('button', { name: 'บันทึกการชำระเงิน' })
      fireEvent.click(submitButton)

      await waitFor(() => {
        expect(mockOnSuccess).toHaveBeenCalled()
        expect(mockOnClose).toHaveBeenCalled()
      })
    })

    test('shows error message on API failure', async () => {
      ;(global.fetch as jest.Mock).mockResolvedValue({
        ok: false,
        json: () => Promise.resolve({ success: false, error: 'Payment failed' }),
      })

      render(<PaymentModal {...defaultProps} />)

      const submitButton = screen.getByRole('button', { name: 'บันทึกการชำระเงิน' })
      fireEvent.click(submitButton)

      await waitFor(() => {
        expect(screen.getByText('Payment failed')).toBeInTheDocument()
      })
    })
  })

  describe('Modal Interactions', () => {
    test('closes modal when close button is clicked', () => {
      render(<PaymentModal {...defaultProps} />)

      const closeButton = screen.getByTestId('x-icon').closest('button')
      fireEvent.click(closeButton!)

      expect(mockOnClose).toHaveBeenCalled()
    })

    test('closes modal when cancel button is clicked', () => {
      render(<PaymentModal {...defaultProps} />)

      const cancelButton = screen.getByText('ยกเลิก')
      fireEvent.click(cancelButton)

      expect(mockOnClose).toHaveBeenCalled()
    })

    test('closes modal when clicking backdrop', () => {
      render(<PaymentModal {...defaultProps} />)

      const backdrop = screen.getByRole('heading', { name: 'บันทึกการชำระเงิน' }).closest('.fixed')
      fireEvent.click(backdrop!)

      expect(mockOnClose).toHaveBeenCalled()
    })

    test('does not close modal when clicking inside content', () => {
      render(<PaymentModal {...defaultProps} />)

      fireEvent.click(screen.getByRole('heading', { name: 'บันทึกการชำระเงิน' }))

      expect(mockOnClose).not.toHaveBeenCalled()
    })
  })
})
