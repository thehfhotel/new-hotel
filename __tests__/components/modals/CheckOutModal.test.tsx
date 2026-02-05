/**
 * @jest-environment jsdom
 */

import { render, screen, fireEvent, waitFor } from '@testing-library/react'
import CheckOutModal from '@/components/modals/CheckOutModal'

// Mock lucide-react icons
jest.mock('lucide-react', () => ({
  X: () => <span data-testid="x-icon">X</span>,
  Loader2: ({ className }: { className?: string }) => (
    <span data-testid="loader-icon" className={className}>Loading</span>
  ),
  User: () => <span data-testid="user-icon">User</span>,
  Calendar: () => <span data-testid="calendar-icon">Calendar</span>,
  DollarSign: () => <span data-testid="dollar-icon">$</span>,
  FileText: () => <span data-testid="filetext-icon">File</span>,
  CreditCard: () => <span data-testid="creditcard-icon">Card</span>,
}))

// Mock check-in data
const createMockCheckIn = (overrides = {}) => ({
  id: 1,
  cinNo: 'CIN-2569-0001',
  customerId: 101,
  customerName: 'John Doe',
  roomId: 201,
  roomNo: '301',
  checkInTime: '2026-01-20T14:00:00.000Z',
  expectedCheckout: '2026-01-25T12:00:00.000Z',
  ratePerNight: 1500,
  status: 'checkin',
  ...overrides,
})

describe('CheckOutModal Component', () => {
  const defaultProps = {
    isOpen: true,
    onClose: jest.fn(),
    checkIn: createMockCheckIn(),
    onSuccess: jest.fn(),
  }

  beforeEach(() => {
    jest.clearAllMocks()
    // Mock fetch
    global.fetch = jest.fn()
    // Mock Date to a fixed date for consistent testing
    jest.useFakeTimers()
    jest.setSystemTime(new Date('2026-01-25T10:00:00.000Z'))
  })

  afterEach(() => {
    jest.useRealTimers()
  })

  describe('Rendering', () => {
    test('renders modal when isOpen is true', () => {
      render(<CheckOutModal {...defaultProps} />)

      expect(screen.getByText('เช็คเอาท์')).toBeInTheDocument()
      expect(screen.getByText('ห้อง 301 - CIN-2569-0001')).toBeInTheDocument()
    })

    test('does not render modal when isOpen is false', () => {
      render(<CheckOutModal {...defaultProps} isOpen={false} />)

      expect(screen.queryByText('เช็คเอาท์')).not.toBeInTheDocument()
    })

    test('does not render modal when checkIn is null', () => {
      render(<CheckOutModal {...defaultProps} checkIn={null} />)

      expect(screen.queryByText('เช็คเอาท์')).not.toBeInTheDocument()
    })

    test('displays guest name', () => {
      render(<CheckOutModal {...defaultProps} />)

      expect(screen.getByText('ชื่อผู้เข้าพัก')).toBeInTheDocument()
      expect(screen.getByText('John Doe')).toBeInTheDocument()
    })

    test('displays "ไม่ระบุ" when customer name is null', () => {
      const checkInWithoutName = createMockCheckIn({ customerName: null })
      render(<CheckOutModal {...defaultProps} checkIn={checkInWithoutName} />)

      expect(screen.getByText('ไม่ระบุ')).toBeInTheDocument()
    })

    test('displays check-in date in Thai Buddhist Era format', () => {
      render(<CheckOutModal {...defaultProps} />)

      expect(screen.getByText('วันที่เช็คอิน')).toBeInTheDocument()
      // Check-in is 20 Jan 2026 = 20 ม.ค. 2569 (Buddhist Era)
      expect(screen.getByText('20 ม.ค. 2569')).toBeInTheDocument()
    })

    test('displays checkout date (today) in Buddhist Era format', () => {
      render(<CheckOutModal {...defaultProps} />)

      expect(screen.getByText('วันที่เช็คเอาท์ (วันนี้)')).toBeInTheDocument()
      // Today is 25 Jan 2026 = 25/01/2569
      expect(screen.getByText('25/01/2569')).toBeInTheDocument()
    })

    test('renders payment method dropdown', () => {
      render(<CheckOutModal {...defaultProps} />)

      expect(screen.getByText('วิธีการชำระเงิน')).toBeInTheDocument()
      expect(screen.getByRole('combobox')).toBeInTheDocument()
    })

    test('renders notes textarea', () => {
      render(<CheckOutModal {...defaultProps} />)

      expect(screen.getByPlaceholderText('หมายเหตุเพิ่มเติม...')).toBeInTheDocument()
    })

    test('renders cancel and submit buttons', () => {
      render(<CheckOutModal {...defaultProps} />)

      expect(screen.getByText('ยกเลิก')).toBeInTheDocument()
      expect(screen.getByText('ยืนยันเช็คเอาท์')).toBeInTheDocument()
    })
  })

  describe('Nights and Total Calculation', () => {
    test('calculates correct number of nights', () => {
      render(<CheckOutModal {...defaultProps} />)

      // Check-in: Jan 20, Checkout: Jan 25 = 5 nights
      expect(screen.getByText('5 คืน')).toBeInTheDocument()
    })

    test('displays rate per night correctly', () => {
      render(<CheckOutModal {...defaultProps} />)

      expect(screen.getByText('ราคาต่อคืน')).toBeInTheDocument()
      // THB 1,500 formatted
      expect(screen.getByText(/฿1,500/)).toBeInTheDocument()
    })

    test('calculates correct total amount', () => {
      render(<CheckOutModal {...defaultProps} />)

      expect(screen.getByText('รวมทั้งหมด')).toBeInTheDocument()
      // 5 nights x 1500 = 7500
      expect(screen.getByText(/฿7,500/)).toBeInTheDocument()
    })

    test('handles minimum 1 night calculation', () => {
      // Check-in today, checkout today = minimum 1 night
      const checkInSameDay = createMockCheckIn({
        checkInTime: '2026-01-25T08:00:00.000Z',
      })
      render(<CheckOutModal {...defaultProps} checkIn={checkInSameDay} />)

      expect(screen.getByText('1 คืน')).toBeInTheDocument()
    })

    test('displays "ไม่ระบุ" when rate is 0', () => {
      const checkInNoRate = createMockCheckIn({ ratePerNight: 0 })
      render(<CheckOutModal {...defaultProps} checkIn={checkInNoRate} />)

      // Find the rate display (there should be "ไม่ระบุ" for rate)
      const rateLabels = screen.getAllByText('ไม่ระบุ')
      expect(rateLabels.length).toBeGreaterThan(0)
    })

    test('displays "ไม่ระบุ" when ratePerNight is null', () => {
      const checkInNullRate = createMockCheckIn({ ratePerNight: null })
      render(<CheckOutModal {...defaultProps} checkIn={checkInNullRate} />)

      const rateLabels = screen.getAllByText('ไม่ระบุ')
      expect(rateLabels.length).toBeGreaterThan(0)
    })

    test('calculates nights correctly for longer stays', () => {
      const longStay = createMockCheckIn({
        checkInTime: '2026-01-10T14:00:00.000Z', // 15 days stay
      })
      render(<CheckOutModal {...defaultProps} checkIn={longStay} />)

      expect(screen.getByText('15 คืน')).toBeInTheDocument()
    })
  })

  describe('Payment Method Selection', () => {
    test('defaults to cash payment method', () => {
      render(<CheckOutModal {...defaultProps} />)

      const select = screen.getByRole('combobox')
      expect(select).toHaveValue('cash')
    })

    test('can select credit card payment method', () => {
      render(<CheckOutModal {...defaultProps} />)

      const select = screen.getByRole('combobox')
      fireEvent.change(select, { target: { value: 'credit' } })

      expect(select).toHaveValue('credit')
    })

    test('can select transfer payment method', () => {
      render(<CheckOutModal {...defaultProps} />)

      const select = screen.getByRole('combobox')
      fireEvent.change(select, { target: { value: 'transfer' } })

      expect(select).toHaveValue('transfer')
    })

    test('displays all payment method options', () => {
      render(<CheckOutModal {...defaultProps} />)

      expect(screen.getByText('เงินสด / Cash')).toBeInTheDocument()
      expect(screen.getByText('บัตรเครดิต / Credit Card')).toBeInTheDocument()
      expect(screen.getByText('โอนเงิน / Transfer')).toBeInTheDocument()
    })
  })

  describe('API Submission', () => {
    test('submits form with correct data', async () => {
      ;(global.fetch as jest.Mock).mockResolvedValue({
        ok: true,
        json: () => Promise.resolve({ success: true }),
      })

      render(<CheckOutModal {...defaultProps} />)

      // Select credit card payment
      const select = screen.getByRole('combobox')
      fireEvent.change(select, { target: { value: 'credit' } })

      // Add notes
      const notesInput = screen.getByPlaceholderText('หมายเหตุเพิ่มเติม...')
      fireEvent.change(notesInput, { target: { value: 'Guest requested late checkout' } })

      // Submit
      const submitButton = screen.getByText('ยืนยันเช็คเอาท์')
      fireEvent.click(submitButton)

      await waitFor(() => {
        expect(global.fetch).toHaveBeenCalledWith('/api/new/checkins/1/checkout', {
          method: 'PUT',
          headers: { 'Content-Type': 'application/json' },
          body: expect.any(String),
        })
      })

      // Verify request body
      const lastCall = (global.fetch as jest.Mock).mock.calls[0]
      const requestBody = JSON.parse(lastCall[1].body)
      expect(requestBody.totalAmount).toBe(7500) // 5 nights x 1500
      expect(requestBody.paymentStatus).toBe('credit')
      expect(requestBody.notes).toBe('Guest requested late checkout')
    })

    test('shows loading state during submission', async () => {
      ;(global.fetch as jest.Mock).mockImplementation(
        () => new Promise((resolve) => setTimeout(() => resolve({
          ok: true,
          json: () => Promise.resolve({ success: true }),
        }), 1000))
      )

      render(<CheckOutModal {...defaultProps} />)

      const submitButton = screen.getByText('ยืนยันเช็คเอาท์')
      fireEvent.click(submitButton)

      await waitFor(() => {
        expect(screen.getByText('กำลังบันทึก...')).toBeInTheDocument()
      })
    })

    test('disables buttons during submission', async () => {
      ;(global.fetch as jest.Mock).mockImplementation(
        () => new Promise((resolve) => setTimeout(() => resolve({
          ok: true,
          json: () => Promise.resolve({ success: true }),
        }), 1000))
      )

      render(<CheckOutModal {...defaultProps} />)

      const submitButton = screen.getByText('ยืนยันเช็คเอาท์')
      const cancelButton = screen.getByText('ยกเลิก')

      fireEvent.click(submitButton)

      await waitFor(() => {
        expect(submitButton).toBeDisabled()
        expect(cancelButton).toBeDisabled()
      })
    })

    test('calls onSuccess and onClose after successful submission', async () => {
      ;(global.fetch as jest.Mock).mockResolvedValue({
        ok: true,
        json: () => Promise.resolve({ success: true }),
      })

      render(<CheckOutModal {...defaultProps} />)

      const submitButton = screen.getByText('ยืนยันเช็คเอาท์')
      fireEvent.click(submitButton)

      await waitFor(() => {
        expect(defaultProps.onSuccess).toHaveBeenCalled()
        expect(defaultProps.onClose).toHaveBeenCalled()
      })
    })

    test('shows error message on API failure', async () => {
      ;(global.fetch as jest.Mock).mockResolvedValue({
        ok: false,
        json: () => Promise.resolve({ success: false, error: 'เกิดข้อผิดพลาดในระบบ' }),
      })

      render(<CheckOutModal {...defaultProps} />)

      const submitButton = screen.getByText('ยืนยันเช็คเอาท์')
      fireEvent.click(submitButton)

      await waitFor(() => {
        expect(screen.getByText('เกิดข้อผิดพลาดในระบบ')).toBeInTheDocument()
      })
    })

    test('handles network error gracefully', async () => {
      ;(global.fetch as jest.Mock).mockRejectedValue(new Error('Network error'))

      render(<CheckOutModal {...defaultProps} />)

      const submitButton = screen.getByText('ยืนยันเช็คเอาท์')
      fireEvent.click(submitButton)

      await waitFor(() => {
        expect(screen.getByText('Network error')).toBeInTheDocument()
      })
    })

    test('does not submit if checkIn is null', async () => {
      const { rerender } = render(<CheckOutModal {...defaultProps} />)

      // Somehow checkIn becomes null during submission (edge case)
      rerender(<CheckOutModal {...defaultProps} checkIn={null} />)

      // The modal should not be visible
      expect(screen.queryByText('ยืนยันเช็คเอาท์')).not.toBeInTheDocument()
      expect(global.fetch).not.toHaveBeenCalled()
    })

    test('omits notes from request if empty', async () => {
      ;(global.fetch as jest.Mock).mockResolvedValue({
        ok: true,
        json: () => Promise.resolve({ success: true }),
      })

      render(<CheckOutModal {...defaultProps} />)

      const submitButton = screen.getByText('ยืนยันเช็คเอาท์')
      fireEvent.click(submitButton)

      await waitFor(() => {
        expect(global.fetch).toHaveBeenCalled()
      })

      const lastCall = (global.fetch as jest.Mock).mock.calls[0]
      const requestBody = JSON.parse(lastCall[1].body)
      expect(requestBody.notes).toBeUndefined()
    })

    test('omits totalAmount if it is 0', async () => {
      const checkInNoRate = createMockCheckIn({ ratePerNight: 0 })
      ;(global.fetch as jest.Mock).mockResolvedValue({
        ok: true,
        json: () => Promise.resolve({ success: true }),
      })

      render(<CheckOutModal {...defaultProps} checkIn={checkInNoRate} />)

      const submitButton = screen.getByText('ยืนยันเช็คเอาท์')
      fireEvent.click(submitButton)

      await waitFor(() => {
        expect(global.fetch).toHaveBeenCalled()
      })

      const lastCall = (global.fetch as jest.Mock).mock.calls[0]
      const requestBody = JSON.parse(lastCall[1].body)
      expect(requestBody.totalAmount).toBeUndefined()
    })
  })

  describe('Modal Interaction', () => {
    test('calls onClose when clicking close button', () => {
      render(<CheckOutModal {...defaultProps} />)

      const closeButton = screen.getByTestId('x-icon').closest('button')
      fireEvent.click(closeButton!)

      expect(defaultProps.onClose).toHaveBeenCalled()
    })

    test('calls onClose when clicking cancel button', () => {
      render(<CheckOutModal {...defaultProps} />)

      const cancelButton = screen.getByText('ยกเลิก')
      fireEvent.click(cancelButton)

      expect(defaultProps.onClose).toHaveBeenCalled()
    })

    test('calls onClose when clicking backdrop', () => {
      render(<CheckOutModal {...defaultProps} />)

      const backdrop = screen.getByText('เช็คเอาท์').closest('.fixed')
      fireEvent.click(backdrop!)

      expect(defaultProps.onClose).toHaveBeenCalled()
    })

    test('does not close when clicking inside modal content', () => {
      render(<CheckOutModal {...defaultProps} />)

      const modalContent = screen.getByText('เช็คเอาท์').closest('.bg-white')
      fireEvent.click(modalContent!)

      expect(defaultProps.onClose).not.toHaveBeenCalled()
    })

    test('resets form when modal is reopened', () => {
      const { rerender } = render(<CheckOutModal {...defaultProps} />)

      // Change payment method
      const select = screen.getByRole('combobox')
      fireEvent.change(select, { target: { value: 'credit' } })

      // Add notes
      const notesInput = screen.getByPlaceholderText('หมายเหตุเพิ่มเติม...')
      fireEvent.change(notesInput, { target: { value: 'Some notes' } })

      // Close modal
      rerender(<CheckOutModal {...defaultProps} isOpen={false} />)

      // Reopen modal
      rerender(<CheckOutModal {...defaultProps} isOpen={true} />)

      // Form should be reset
      const newSelect = screen.getByRole('combobox')
      const newNotesInput = screen.getByPlaceholderText('หมายเหตุเพิ่มเติม...')

      expect(newSelect).toHaveValue('cash')
      expect(newNotesInput).toHaveValue('')
    })

    test('clears error when modal is closed', async () => {
      ;(global.fetch as jest.Mock).mockResolvedValue({
        ok: false,
        json: () => Promise.resolve({ success: false, error: 'Test error' }),
      })

      const { rerender } = render(<CheckOutModal {...defaultProps} />)

      // Trigger an error
      const submitButton = screen.getByText('ยืนยันเช็คเอาท์')
      fireEvent.click(submitButton)

      await waitFor(() => {
        expect(screen.getByText('Test error')).toBeInTheDocument()
      })

      // Close and reopen modal
      rerender(<CheckOutModal {...defaultProps} isOpen={false} />)
      rerender(<CheckOutModal {...defaultProps} isOpen={true} />)

      // Error should be cleared
      expect(screen.queryByText('Test error')).not.toBeInTheDocument()
    })
  })

  describe('Edge Cases', () => {
    test('handles checkInTime with null value', () => {
      const checkInNullTime = createMockCheckIn({ checkInTime: null })
      render(<CheckOutModal {...defaultProps} checkIn={checkInNullTime} />)

      // Should display '-' for the check-in date
      expect(screen.getByText('-')).toBeInTheDocument()
    })

    test('handles very large total amounts', () => {
      const expensiveStay = createMockCheckIn({
        checkInTime: '2026-01-01T14:00:00.000Z', // 24 nights
        ratePerNight: 50000,
      })
      render(<CheckOutModal {...defaultProps} checkIn={expensiveStay} />)

      // 24 nights x 50,000 = 1,200,000
      expect(screen.getByText(/฿1,200,000/)).toBeInTheDocument()
    })

    test('handles decimal rate per night', () => {
      const decimalRate = createMockCheckIn({ ratePerNight: 1499.50 })
      render(<CheckOutModal {...defaultProps} checkIn={decimalRate} />)

      // Rate should be formatted
      expect(screen.getByText(/฿1,499/)).toBeInTheDocument()
    })
  })
})
