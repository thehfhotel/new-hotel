/**
 * @jest-environment jsdom
 */

import { render, screen, fireEvent, waitFor } from '@testing-library/react'
import CheckOutModal from '@/components/modals/CheckOutModal'

// Mock lucide-react icons
jest.mock('lucide-react', () => ({
  X: () => <span data-testid="x-icon">X</span>,
  Loader2: () => <span data-testid="loader-icon">Loading</span>,
  User: () => <span data-testid="user-icon">User</span>,
  Calendar: () => <span data-testid="calendar-icon">Calendar</span>,
  DollarSign: () => <span data-testid="dollar-icon">$</span>,
  FileText: () => <span data-testid="file-text-icon">Notes</span>,
  CreditCard: () => <span data-testid="credit-card-icon">Card</span>,
}))

// Mock fetch API
const mockFetch = jest.fn()
global.fetch = mockFetch

const mockCheckIn = {
  id: 1,
  cinNo: 'CIN-001',
  customerId: 101,
  customerName: 'สมชาย ใจดี',
  roomId: 301,
  roomNo: '301',
  checkInTime: '2026-01-18T14:00:00.000Z',
  expectedCheckout: '2026-01-21T12:00:00.000Z',
  ratePerNight: 1500,
  status: 'active',
}

describe('CheckOutModal Component', () => {
  const defaultProps = {
    isOpen: true,
    onClose: jest.fn(),
    checkIn: mockCheckIn,
    onSuccess: jest.fn(),
  }

  beforeEach(() => {
    jest.clearAllMocks()
    jest.useFakeTimers()
    jest.setSystemTime(new Date('2026-01-21T10:00:00'))

    mockFetch.mockResolvedValue({
      ok: true,
      json: () => Promise.resolve({ success: true }),
    })
  })

  afterEach(() => {
    jest.useRealTimers()
  })

  describe('Rendering', () => {
    test('renders modal with Thai title', () => {
      render(<CheckOutModal {...defaultProps} />)

      expect(screen.getByText('เช็คเอาท์')).toBeInTheDocument()
    })

    test('renders room and check-in number in subtitle', () => {
      render(<CheckOutModal {...defaultProps} />)

      expect(screen.getByText('ห้อง 301 - CIN-001')).toBeInTheDocument()
    })

    test('renders customer name', () => {
      render(<CheckOutModal {...defaultProps} />)

      expect(screen.getByText('ชื่อผู้เข้าพัก')).toBeInTheDocument()
      expect(screen.getByText('สมชาย ใจดี')).toBeInTheDocument()
    })

    test('renders check-in date section with Thai labels', () => {
      render(<CheckOutModal {...defaultProps} />)

      expect(screen.getByText('วันที่เช็คอิน')).toBeInTheDocument()
    })

    test('renders check-out date section with Thai labels', () => {
      render(<CheckOutModal {...defaultProps} />)

      expect(screen.getByText('วันที่เช็คเอาท์ (วันนี้)')).toBeInTheDocument()
    })

    test('renders billing summary', () => {
      render(<CheckOutModal {...defaultProps} />)

      expect(screen.getByText('จำนวนคืน')).toBeInTheDocument()
      expect(screen.getByText('ราคาต่อคืน')).toBeInTheDocument()
      expect(screen.getByText('รวมทั้งหมด')).toBeInTheDocument()
    })

    test('renders payment method dropdown with Thai label', () => {
      render(<CheckOutModal {...defaultProps} />)

      expect(screen.getByText('วิธีการชำระเงิน')).toBeInTheDocument()
      expect(screen.getByText('เงินสด / Cash')).toBeInTheDocument()
    })

    test('renders notes field with Thai label', () => {
      render(<CheckOutModal {...defaultProps} />)

      expect(screen.getByText('หมายเหตุ')).toBeInTheDocument()
      expect(screen.getByPlaceholderText('หมายเหตุเพิ่มเติม...')).toBeInTheDocument()
    })

    test('renders action buttons with Thai text', () => {
      render(<CheckOutModal {...defaultProps} />)

      expect(screen.getByText('ยกเลิก')).toBeInTheDocument()
      expect(screen.getByText('ยืนยันเช็คเอาท์')).toBeInTheDocument()
    })

    test('does not render when isOpen is false', () => {
      render(<CheckOutModal {...defaultProps} isOpen={false} />)

      expect(screen.queryByText('เช็คเอาท์')).not.toBeInTheDocument()
    })

    test('does not render when checkIn is null', () => {
      render(<CheckOutModal {...defaultProps} checkIn={null} />)

      expect(screen.queryByText('เช็คเอาท์')).not.toBeInTheDocument()
    })
  })

  describe('Billing Calculations', () => {
    test('calculates correct number of nights', () => {
      // Check-in: Jan 18, System time (checkout): Jan 21 = 3 nights
      render(<CheckOutModal {...defaultProps} />)

      expect(screen.getByText('3 คืน')).toBeInTheDocument()
    })

    test('calculates correct total amount', () => {
      // 3 nights x 1500 = 4500 THB
      render(<CheckOutModal {...defaultProps} />)

      // Total should be formatted as Thai Baht
      expect(screen.getByText(/฿4,500/)).toBeInTheDocument()
    })

    test('shows minimum 1 night for same day checkout', () => {
      const sameDay = {
        ...mockCheckIn,
        checkInTime: '2026-01-21T08:00:00.000Z', // Same day as system time
      }

      render(<CheckOutModal {...defaultProps} checkIn={sameDay} />)

      expect(screen.getByText('1 คืน')).toBeInTheDocument()
    })

    test('shows "ไม่ระบุ" when rate is 0', () => {
      const noRate = {
        ...mockCheckIn,
        ratePerNight: 0,
      }

      render(<CheckOutModal {...defaultProps} checkIn={noRate} />)

      const rateLabels = screen.getAllByText('ไม่ระบุ')
      expect(rateLabels.length).toBeGreaterThan(0)
    })

    test('shows "ไม่ระบุ" when rate is null', () => {
      const nullRate = {
        ...mockCheckIn,
        ratePerNight: null,
      }

      render(<CheckOutModal {...defaultProps} checkIn={nullRate} />)

      const rateLabels = screen.getAllByText('ไม่ระบุ')
      expect(rateLabels.length).toBeGreaterThan(0)
    })
  })

  describe('Date Formatting', () => {
    test('displays check-in date in Thai Buddhist Era format', () => {
      render(<CheckOutModal {...defaultProps} />)

      // Jan 18, 2026 = 18 ม.ค. 2569
      expect(screen.getByText('18 ม.ค. 2569')).toBeInTheDocument()
    })

    test('displays checkout date (today) in Buddhist Era format', () => {
      render(<CheckOutModal {...defaultProps} />)

      // Jan 21, 2026 = 21/01/2569
      expect(screen.getByText('21/01/2569')).toBeInTheDocument()
    })
  })

  describe('Payment Methods', () => {
    test('renders all payment method options', () => {
      render(<CheckOutModal {...defaultProps} />)

      const select = screen.getByRole('combobox')

      expect(select).toContainHTML('เงินสด / Cash')
      expect(select).toContainHTML('บัตรเครดิต / Credit Card')
      expect(select).toContainHTML('โอนเงิน / Transfer')
    })

    test('allows changing payment method', () => {
      render(<CheckOutModal {...defaultProps} />)

      const select = screen.getByRole('combobox') as HTMLSelectElement
      fireEvent.change(select, { target: { value: 'credit' } })

      expect(select.value).toBe('credit')
    })
  })

  describe('Form Submission', () => {
    test('submits checkout with correct data', async () => {
      const mockOnSuccess = jest.fn()
      render(
        <CheckOutModal {...defaultProps} onSuccess={mockOnSuccess} />
      )

      // Change payment method
      const select = screen.getByRole('combobox')
      fireEvent.change(select, { target: { value: 'transfer' } })

      // Add notes
      const notesInput = screen.getByPlaceholderText('หมายเหตุเพิ่มเติม...')
      fireEvent.change(notesInput, { target: { value: 'ลูกค้าชำระเงินเรียบร้อย' } })

      // Submit
      const submitButton = screen.getByText('ยืนยันเช็คเอาท์')
      fireEvent.click(submitButton)

      await waitFor(() => {
        expect(mockFetch).toHaveBeenCalledWith(
          '/api/new/checkins/1/checkout',
          expect.objectContaining({
            method: 'PUT',
            headers: { 'Content-Type': 'application/json' },
          })
        )
      })

      // Verify request body
      const checkoutCall = mockFetch.mock.calls.find(
        (call) => call[0].includes('/checkout')
      )
      const requestBody = JSON.parse(checkoutCall![1].body)
      expect(requestBody.paymentStatus).toBe('transfer')
      expect(requestBody.notes).toBe('ลูกค้าชำระเงินเรียบร้อย')
      expect(requestBody.totalAmount).toBe(4500) // 3 nights x 1500
    })

    test('calls onSuccess after successful submission', async () => {
      const mockOnSuccess = jest.fn()
      render(
        <CheckOutModal {...defaultProps} onSuccess={mockOnSuccess} />
      )

      const submitButton = screen.getByText('ยืนยันเช็คเอาท์')
      fireEvent.click(submitButton)

      await waitFor(() => {
        expect(mockOnSuccess).toHaveBeenCalled()
      })
    })

    test('calls onClose after successful submission', async () => {
      const mockOnClose = jest.fn()
      render(
        <CheckOutModal {...defaultProps} onClose={mockOnClose} />
      )

      const submitButton = screen.getByText('ยืนยันเช็คเอาท์')
      fireEvent.click(submitButton)

      await waitFor(() => {
        expect(mockOnClose).toHaveBeenCalled()
      })
    })

    test('shows error on submission failure', async () => {
      mockFetch.mockResolvedValue({
        ok: false,
        json: () => Promise.resolve({
          success: false,
          error: 'ไม่สามารถเช็คเอาท์ได้',
        }),
      })

      render(<CheckOutModal {...defaultProps} />)

      const submitButton = screen.getByText('ยืนยันเช็คเอาท์')
      fireEvent.click(submitButton)

      await waitFor(() => {
        expect(screen.getByText('ไม่สามารถเช็คเอาท์ได้')).toBeInTheDocument()
      })
    })

    test('shows generic error on network failure', async () => {
      mockFetch.mockRejectedValue(new Error('Network error'))

      render(<CheckOutModal {...defaultProps} />)

      const submitButton = screen.getByText('ยืนยันเช็คเอาท์')
      fireEvent.click(submitButton)

      await waitFor(() => {
        expect(screen.getByText('เกิดข้อผิดพลาดในการเช็คเอาท์')).toBeInTheDocument()
      })
    })

    test('disables submit button while submitting', async () => {
      mockFetch.mockImplementation(
        () => new Promise((resolve) =>
          setTimeout(() => resolve({
            ok: true,
            json: () => Promise.resolve({ success: true }),
          }), 5000)
        )
      )

      render(<CheckOutModal {...defaultProps} />)

      const submitButton = screen.getByText('ยืนยันเช็คเอาท์')
      fireEvent.click(submitButton)

      await waitFor(() => {
        expect(screen.getByText('กำลังบันทึก...')).toBeInTheDocument()
      })
    })

    test('does not send undefined values in request', async () => {
      const noRate = {
        ...mockCheckIn,
        ratePerNight: null,
      }

      render(<CheckOutModal {...defaultProps} checkIn={noRate} />)

      const submitButton = screen.getByText('ยืนยันเช็คเอาท์')
      fireEvent.click(submitButton)

      await waitFor(() => {
        expect(mockFetch).toHaveBeenCalled()
      })

      const checkoutCall = mockFetch.mock.calls.find(
        (call) => call[0].includes('/checkout')
      )
      const requestBody = JSON.parse(checkoutCall![1].body)

      // When total is 0, it should send undefined
      expect(requestBody.totalAmount).toBeUndefined()
    })
  })

  describe('Modal Behavior', () => {
    test('calls onClose when cancel button is clicked', () => {
      const mockOnClose = jest.fn()
      render(<CheckOutModal {...defaultProps} onClose={mockOnClose} />)

      const cancelButton = screen.getByText('ยกเลิก')
      fireEvent.click(cancelButton)

      expect(mockOnClose).toHaveBeenCalled()
    })

    test('calls onClose when close (X) button is clicked', () => {
      const mockOnClose = jest.fn()
      render(<CheckOutModal {...defaultProps} onClose={mockOnClose} />)

      const closeButton = screen.getByTestId('x-icon').closest('button')
      fireEvent.click(closeButton!)

      expect(mockOnClose).toHaveBeenCalled()
    })

    test('calls onClose when backdrop is clicked', () => {
      const mockOnClose = jest.fn()
      render(<CheckOutModal {...defaultProps} onClose={mockOnClose} />)

      // Click on the backdrop
      const backdrop = document.querySelector('.fixed.inset-0.bg-black')
      fireEvent.click(backdrop!)

      expect(mockOnClose).toHaveBeenCalled()
    })

    test('does not close when modal content is clicked', () => {
      const mockOnClose = jest.fn()
      render(<CheckOutModal {...defaultProps} onClose={mockOnClose} />)

      // Click on the modal content
      const modalContent = screen.getByText('เช็คเอาท์').closest('.bg-white')
      fireEvent.click(modalContent!)

      expect(mockOnClose).not.toHaveBeenCalled()
    })

    test('resets form when modal is reopened', async () => {
      const { rerender } = render(<CheckOutModal {...defaultProps} />)

      // Change payment method and add notes
      const select = screen.getByRole('combobox') as HTMLSelectElement
      fireEvent.change(select, { target: { value: 'credit' } })

      const notesInput = screen.getByPlaceholderText('หมายเหตุเพิ่มเติม...')
      fireEvent.change(notesInput, { target: { value: 'test' } })

      // Close modal
      rerender(<CheckOutModal {...defaultProps} isOpen={false} />)

      // Reopen modal
      rerender(<CheckOutModal {...defaultProps} isOpen={true} />)

      // Form should be reset
      const newSelect = screen.getByRole('combobox') as HTMLSelectElement
      expect(newSelect.value).toBe('cash')

      const newNotesInput = screen.getByPlaceholderText('หมายเหตุเพิ่มเติม...')
      expect(newNotesInput).toHaveValue('')
    })
  })

  describe('Edge Cases', () => {
    test('handles missing customer name gracefully', () => {
      const noName = {
        ...mockCheckIn,
        customerName: null,
      }

      render(<CheckOutModal {...defaultProps} checkIn={noName} />)

      expect(screen.getByText('ไม่ระบุ')).toBeInTheDocument()
    })

    test('handles missing check-in time gracefully', () => {
      const noCheckInTime = {
        ...mockCheckIn,
        checkInTime: null,
      }

      render(<CheckOutModal {...defaultProps} checkIn={noCheckInTime} />)

      // Should show "-" for missing date
      expect(screen.getByText('-')).toBeInTheDocument()
    })
  })
})
