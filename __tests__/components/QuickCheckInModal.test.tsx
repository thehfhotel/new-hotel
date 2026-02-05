/**
 * @jest-environment jsdom
 */

import { render, screen, fireEvent, waitFor, act } from '@testing-library/react'
import QuickCheckInModal from '@/components/modals/QuickCheckInModal'

// Mock lucide-react icons
jest.mock('lucide-react', () => ({
  X: () => <span data-testid="x-icon">X</span>,
  Search: () => <span data-testid="search-icon">Search</span>,
  Loader2: () => <span data-testid="loader-icon">Loading</span>,
  User: () => <span data-testid="user-icon">User</span>,
  Calendar: () => <span data-testid="calendar-icon">Calendar</span>,
  DollarSign: () => <span data-testid="dollar-icon">$</span>,
  FileText: () => <span data-testid="file-text-icon">Notes</span>,
}))

// Mock fetch API
const mockFetch = jest.fn()
global.fetch = mockFetch

const mockRoom = {
  id: 101,
  roomNumber: '301',
  type: 'Deluxe',
  details: 'Sea View',
}

const mockCustomers = [
  {
    id: 1,
    firstName: 'สมชาย',
    lastName: 'ใจดี',
    phone: '0891234567',
    idCard: '***REMOVED***90123',
  },
  {
    id: 2,
    firstName: 'สมศรี',
    lastName: 'รักไทย',
    phone: '0899876543',
    idCard: '9876543210123',
  },
]

describe('QuickCheckInModal Component', () => {
  const defaultProps = {
    isOpen: true,
    onClose: jest.fn(),
    room: mockRoom,
    onSuccess: jest.fn(),
  }

  beforeEach(() => {
    jest.clearAllMocks()
    jest.useFakeTimers()
    jest.setSystemTime(new Date('2026-01-20T10:00:00'))

    // Default fetch implementation
    mockFetch.mockImplementation((url: string) => {
      if (url.includes('/api/new/customers')) {
        return Promise.resolve({
          ok: true,
          json: () => Promise.resolve({ success: true, data: mockCustomers }),
        })
      }
      if (url.includes('/api/new/checkins') && !url.includes('/checkout')) {
        return Promise.resolve({
          ok: true,
          json: () => Promise.resolve({ success: true, data: { id: 1 } }),
        })
      }
      return Promise.reject(new Error('Unknown endpoint'))
    })
  })

  afterEach(() => {
    jest.useRealTimers()
  })

  describe('Rendering', () => {
    test('renders modal with Thai title', () => {
      render(<QuickCheckInModal {...defaultProps} />)

      expect(screen.getByText('เช็คอินด่วน')).toBeInTheDocument()
    })

    test('renders room information in subtitle', () => {
      render(<QuickCheckInModal {...defaultProps} />)

      expect(screen.getByText('ห้อง 301 - Deluxe')).toBeInTheDocument()
    })

    test('renders customer search field with Thai label', () => {
      render(<QuickCheckInModal {...defaultProps} />)

      expect(screen.getByText('ลูกค้า *')).toBeInTheDocument()
      expect(
        screen.getByPlaceholderText('ค้นหาด้วยชื่อ, เบอร์โทร, หรือเลขบัตรประชาชน...')
      ).toBeInTheDocument()
    })

    test('renders expected checkout field with Thai label', () => {
      render(<QuickCheckInModal {...defaultProps} />)

      expect(screen.getByText('วันที่คาดว่าจะเช็คเอาท์ *')).toBeInTheDocument()
    })

    test('renders rate per night field with Thai label', () => {
      render(<QuickCheckInModal {...defaultProps} />)

      expect(screen.getByText('ราคาต่อคืน (บาท)')).toBeInTheDocument()
    })

    test('renders adults and children fields', () => {
      render(<QuickCheckInModal {...defaultProps} />)

      expect(screen.getByText('ผู้ใหญ่')).toBeInTheDocument()
      expect(screen.getByText('เด็ก')).toBeInTheDocument()
    })

    test('renders notes field with Thai label', () => {
      render(<QuickCheckInModal {...defaultProps} />)

      expect(screen.getByText('หมายเหตุ')).toBeInTheDocument()
      expect(screen.getByPlaceholderText('หมายเหตุเพิ่มเติม...')).toBeInTheDocument()
    })

    test('renders action buttons with Thai text', () => {
      render(<QuickCheckInModal {...defaultProps} />)

      expect(screen.getByText('ยกเลิก')).toBeInTheDocument()
      expect(screen.getByText('เช็คอิน')).toBeInTheDocument()
    })

    test('does not render when isOpen is false', () => {
      render(<QuickCheckInModal {...defaultProps} isOpen={false} />)

      expect(screen.queryByText('เช็คอินด่วน')).not.toBeInTheDocument()
    })
  })

  describe('Form Initialization', () => {
    test('sets default expected checkout to tomorrow', () => {
      render(<QuickCheckInModal {...defaultProps} />)

      // System time is 2026-01-20, so tomorrow is 2026-01-21
      const dateInput = screen.getByRole('textbox', { hidden: true }) ||
        document.querySelector('input[type="date"]')

      // Check that the date input exists and has tomorrow's date
      expect(dateInput).toHaveValue('2026-01-21')
    })

    test('sets default adults to 1', () => {
      render(<QuickCheckInModal {...defaultProps} />)

      const adultsInput = screen.getAllByRole('spinbutton').find(
        (input) => (input as HTMLInputElement).value === '1'
      )
      expect(adultsInput).toBeInTheDocument()
    })

    test('sets default children to 0', () => {
      render(<QuickCheckInModal {...defaultProps} />)

      const childrenInput = screen.getAllByRole('spinbutton').find(
        (input) => (input as HTMLInputElement).value === '0'
      )
      expect(childrenInput).toBeInTheDocument()
    })
  })

  describe('Customer Search', () => {
    test('shows dropdown when search query is entered', async () => {
      render(<QuickCheckInModal {...defaultProps} />)

      const searchInput = screen.getByPlaceholderText(
        'ค้นหาด้วยชื่อ, เบอร์โทร, หรือเลขบัตรประชาชน...'
      )
      fireEvent.change(searchInput, { target: { value: 'สม' } })

      await act(async () => {
        jest.advanceTimersByTime(300)
      })

      await waitFor(() => {
        expect(mockFetch).toHaveBeenCalledWith(
          expect.stringContaining('/api/new/customers')
        )
      })
    })

    test('does not search with query less than 2 characters', async () => {
      render(<QuickCheckInModal {...defaultProps} />)

      const searchInput = screen.getByPlaceholderText(
        'ค้นหาด้วยชื่อ, เบอร์โทร, หรือเลขบัตรประชาชน...'
      )
      fireEvent.change(searchInput, { target: { value: 'ส' } })

      await act(async () => {
        jest.advanceTimersByTime(300)
      })

      // Should not have made any API calls for customers
      const customerCalls = mockFetch.mock.calls.filter(
        (call) => call[0].includes('/api/new/customers')
      )
      expect(customerCalls.length).toBe(0)
    })

    test('displays customer results in dropdown', async () => {
      render(<QuickCheckInModal {...defaultProps} />)

      const searchInput = screen.getByPlaceholderText(
        'ค้นหาด้วยชื่อ, เบอร์โทร, หรือเลขบัตรประชาชน...'
      )
      fireEvent.change(searchInput, { target: { value: 'สม' } })

      await act(async () => {
        jest.advanceTimersByTime(300)
      })

      await waitFor(() => {
        expect(screen.getByText('สมชาย')).toBeInTheDocument()
        expect(screen.getByText('สมศรี')).toBeInTheDocument()
      })
    })

    test('shows "ไม่พบลูกค้า" when no results', async () => {
      mockFetch.mockImplementation((url: string) => {
        if (url.includes('/api/new/customers')) {
          return Promise.resolve({
            ok: true,
            json: () => Promise.resolve({ success: true, data: [] }),
          })
        }
        return Promise.reject(new Error('Unknown endpoint'))
      })

      render(<QuickCheckInModal {...defaultProps} />)

      const searchInput = screen.getByPlaceholderText(
        'ค้นหาด้วยชื่อ, เบอร์โทร, หรือเลขบัตรประชาชน...'
      )
      fireEvent.change(searchInput, { target: { value: 'ไม่มี' } })

      await act(async () => {
        jest.advanceTimersByTime(300)
      })

      await waitFor(() => {
        expect(screen.getByText('ไม่พบลูกค้า')).toBeInTheDocument()
      })
    })

    test('selects customer when clicked', async () => {
      render(<QuickCheckInModal {...defaultProps} />)

      const searchInput = screen.getByPlaceholderText(
        'ค้นหาด้วยชื่อ, เบอร์โทร, หรือเลขบัตรประชาชน...'
      )
      fireEvent.change(searchInput, { target: { value: 'สม' } })

      await act(async () => {
        jest.advanceTimersByTime(300)
      })

      await waitFor(() => {
        expect(screen.getByText('สมชาย')).toBeInTheDocument()
      })

      fireEvent.click(screen.getByText('สมชาย'))

      // Should show selected customer confirmation
      await waitFor(() => {
        expect(screen.getByText(/เลือกแล้ว:/)).toBeInTheDocument()
      })
    })
  })

  describe('Form Validation', () => {
    test('shows error when submitting without customer', async () => {
      render(<QuickCheckInModal {...defaultProps} />)

      const submitButton = screen.getByText('เช็คอิน')
      fireEvent.click(submitButton)

      await waitFor(() => {
        expect(screen.getByText('กรุณาเลือกลูกค้า')).toBeInTheDocument()
      })
    })

    test('shows error when submitting without checkout date', async () => {
      render(<QuickCheckInModal {...defaultProps} />)

      // Select a customer first
      const searchInput = screen.getByPlaceholderText(
        'ค้นหาด้วยชื่อ, เบอร์โทร, หรือเลขบัตรประชาชน...'
      )
      fireEvent.change(searchInput, { target: { value: 'สม' } })

      await act(async () => {
        jest.advanceTimersByTime(300)
      })

      await waitFor(() => {
        expect(screen.getByText('สมชาย')).toBeInTheDocument()
      })

      fireEvent.click(screen.getByText('สมชาย'))

      // Clear the checkout date
      const dateInput = document.querySelector('input[type="date"]') as HTMLInputElement
      fireEvent.change(dateInput, { target: { value: '' } })

      const submitButton = screen.getByText('เช็คอิน')
      fireEvent.click(submitButton)

      await waitFor(() => {
        expect(screen.getByText('กรุณาเลือกวันที่คาดว่าจะเช็คเอาท์')).toBeInTheDocument()
      })
    })
  })

  describe('Form Submission', () => {
    test('submits check-in with correct data', async () => {
      const mockOnSuccess = jest.fn()
      render(
        <QuickCheckInModal {...defaultProps} onSuccess={mockOnSuccess} />
      )

      // Select a customer
      const searchInput = screen.getByPlaceholderText(
        'ค้นหาด้วยชื่อ, เบอร์โทร, หรือเลขบัตรประชาชน...'
      )
      fireEvent.change(searchInput, { target: { value: 'สม' } })

      await act(async () => {
        jest.advanceTimersByTime(300)
      })

      await waitFor(() => {
        expect(screen.getByText('สมชาย')).toBeInTheDocument()
      })

      fireEvent.click(screen.getByText('สมชาย'))

      // Fill in rate per night
      const rateInput = screen.getByPlaceholderText('ระบุราคา...')
      fireEvent.change(rateInput, { target: { value: '1500' } })

      // Submit
      const submitButton = screen.getByText('เช็คอิน')
      fireEvent.click(submitButton)

      await waitFor(() => {
        expect(mockFetch).toHaveBeenCalledWith(
          '/api/new/checkins',
          expect.objectContaining({
            method: 'POST',
            headers: { 'Content-Type': 'application/json' },
          })
        )
      })

      // Verify the request body
      const checkinCall = mockFetch.mock.calls.find(
        (call) => call[0] === '/api/new/checkins'
      )
      const requestBody = JSON.parse(checkinCall![1].body)
      expect(requestBody.customerId).toBe(1)
      expect(requestBody.roomId).toBe(101)
      expect(requestBody.ratePerNight).toBe(1500)
    })

    test('calls onSuccess after successful submission', async () => {
      const mockOnSuccess = jest.fn()
      render(
        <QuickCheckInModal {...defaultProps} onSuccess={mockOnSuccess} />
      )

      // Select a customer
      const searchInput = screen.getByPlaceholderText(
        'ค้นหาด้วยชื่อ, เบอร์โทร, หรือเลขบัตรประชาชน...'
      )
      fireEvent.change(searchInput, { target: { value: 'สม' } })

      await act(async () => {
        jest.advanceTimersByTime(300)
      })

      await waitFor(() => {
        expect(screen.getByText('สมชาย')).toBeInTheDocument()
      })

      fireEvent.click(screen.getByText('สมชาย'))

      const submitButton = screen.getByText('เช็คอิน')
      fireEvent.click(submitButton)

      await waitFor(() => {
        expect(mockOnSuccess).toHaveBeenCalled()
      })
    })

    test('shows error on submission failure', async () => {
      mockFetch.mockImplementation((url: string) => {
        if (url.includes('/api/new/customers')) {
          return Promise.resolve({
            ok: true,
            json: () => Promise.resolve({ success: true, data: mockCustomers }),
          })
        }
        if (url.includes('/api/new/checkins')) {
          return Promise.resolve({
            ok: false,
            json: () => Promise.resolve({
              success: false,
              error: 'ห้องนี้มีผู้เข้าพักอยู่แล้ว'
            }),
          })
        }
        return Promise.reject(new Error('Unknown endpoint'))
      })

      render(<QuickCheckInModal {...defaultProps} />)

      // Select a customer
      const searchInput = screen.getByPlaceholderText(
        'ค้นหาด้วยชื่อ, เบอร์โทร, หรือเลขบัตรประชาชน...'
      )
      fireEvent.change(searchInput, { target: { value: 'สม' } })

      await act(async () => {
        jest.advanceTimersByTime(300)
      })

      await waitFor(() => {
        expect(screen.getByText('สมชาย')).toBeInTheDocument()
      })

      fireEvent.click(screen.getByText('สมชาย'))

      const submitButton = screen.getByText('เช็คอิน')
      fireEvent.click(submitButton)

      await waitFor(() => {
        expect(screen.getByText('ห้องนี้มีผู้เข้าพักอยู่แล้ว')).toBeInTheDocument()
      })
    })

    test('disables submit button when no customer selected', () => {
      render(<QuickCheckInModal {...defaultProps} />)

      const submitButton = screen.getByText('เช็คอิน')
      expect(submitButton).toBeDisabled()
    })

    test('disables submit button while submitting', async () => {
      mockFetch.mockImplementation((url: string) => {
        if (url.includes('/api/new/customers')) {
          return Promise.resolve({
            ok: true,
            json: () => Promise.resolve({ success: true, data: mockCustomers }),
          })
        }
        if (url.includes('/api/new/checkins')) {
          return new Promise((resolve) =>
            setTimeout(() => resolve({
              ok: true,
              json: () => Promise.resolve({ success: true }),
            }), 5000)
          )
        }
        return Promise.reject(new Error('Unknown endpoint'))
      })

      render(<QuickCheckInModal {...defaultProps} />)

      // Select a customer
      const searchInput = screen.getByPlaceholderText(
        'ค้นหาด้วยชื่อ, เบอร์โทร, หรือเลขบัตรประชาชน...'
      )
      fireEvent.change(searchInput, { target: { value: 'สม' } })

      await act(async () => {
        jest.advanceTimersByTime(300)
      })

      await waitFor(() => {
        expect(screen.getByText('สมชาย')).toBeInTheDocument()
      })

      fireEvent.click(screen.getByText('สมชาย'))

      const submitButton = screen.getByText('เช็คอิน')
      fireEvent.click(submitButton)

      await waitFor(() => {
        expect(screen.getByText('กำลังบันทึก...')).toBeInTheDocument()
      })
    })
  })

  describe('Modal Behavior', () => {
    test('calls onClose when cancel button is clicked', () => {
      const mockOnClose = jest.fn()
      render(<QuickCheckInModal {...defaultProps} onClose={mockOnClose} />)

      const cancelButton = screen.getByText('ยกเลิก')
      fireEvent.click(cancelButton)

      expect(mockOnClose).toHaveBeenCalled()
    })

    test('calls onClose when close (X) button is clicked', () => {
      const mockOnClose = jest.fn()
      render(<QuickCheckInModal {...defaultProps} onClose={mockOnClose} />)

      const closeButton = screen.getByTestId('x-icon').closest('button')
      fireEvent.click(closeButton!)

      expect(mockOnClose).toHaveBeenCalled()
    })

    test('calls onClose when backdrop is clicked', () => {
      const mockOnClose = jest.fn()
      render(<QuickCheckInModal {...defaultProps} onClose={mockOnClose} />)

      // Click on the backdrop
      const backdrop = document.querySelector('.fixed.inset-0.bg-black')
      fireEvent.click(backdrop!)

      expect(mockOnClose).toHaveBeenCalled()
    })

    test('does not close when modal content is clicked', () => {
      const mockOnClose = jest.fn()
      render(<QuickCheckInModal {...defaultProps} onClose={mockOnClose} />)

      // Click on the modal content
      const modalContent = screen.getByText('เช็คอินด่วน').closest('.bg-white')
      fireEvent.click(modalContent!)

      expect(mockOnClose).not.toHaveBeenCalled()
    })

    test('resets form when modal is closed and reopened', async () => {
      const { rerender } = render(<QuickCheckInModal {...defaultProps} />)

      // Enter some data
      const searchInput = screen.getByPlaceholderText(
        'ค้นหาด้วยชื่อ, เบอร์โทร, หรือเลขบัตรประชาชน...'
      )
      fireEvent.change(searchInput, { target: { value: 'test' } })

      const notesInput = screen.getByPlaceholderText('หมายเหตุเพิ่มเติม...')
      fireEvent.change(notesInput, { target: { value: 'test notes' } })

      // Close modal
      rerender(<QuickCheckInModal {...defaultProps} isOpen={false} />)

      // Reopen modal
      rerender(<QuickCheckInModal {...defaultProps} isOpen={true} />)

      // Form should be reset
      const newSearchInput = screen.getByPlaceholderText(
        'ค้นหาด้วยชื่อ, เบอร์โทร, หรือเลขบัตรประชาชน...'
      )
      expect(newSearchInput).toHaveValue('')

      const newNotesInput = screen.getByPlaceholderText('หมายเหตุเพิ่มเติม...')
      expect(newNotesInput).toHaveValue('')
    })
  })

  describe('Buddhist Era Date Display', () => {
    test('shows Buddhist Era year in date helper text', async () => {
      render(<QuickCheckInModal {...defaultProps} />)

      // Default date is tomorrow (2026-01-21 in Gregorian = 2569 in Buddhist Era)
      // The helper text should show พ.ศ. 21/01/2569
      await waitFor(() => {
        expect(screen.getByText(/พ\.ศ\./)).toBeInTheDocument()
        expect(screen.getByText(/2569/)).toBeInTheDocument()
      })
    })
  })
})
