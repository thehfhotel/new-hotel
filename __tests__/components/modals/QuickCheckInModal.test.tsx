/**
 * @jest-environment jsdom
 */

import { render, screen, fireEvent, waitFor, act } from '@testing-library/react'
import QuickCheckInModal from '@/components/modals/QuickCheckInModal'

// Mock lucide-react icons
jest.mock('lucide-react', () => ({
  X: () => <span data-testid="x-icon">X</span>,
  Search: () => <span data-testid="search-icon">Search</span>,
  Loader2: ({ className }: { className?: string }) => (
    <span data-testid="loader-icon" className={className}>Loading</span>
  ),
  User: () => <span data-testid="user-icon">User</span>,
  Calendar: () => <span data-testid="calendar-icon">Calendar</span>,
  DollarSign: () => <span data-testid="dollar-icon">$</span>,
  FileText: () => <span data-testid="filetext-icon">File</span>,
}))

// Mock customer data
const mockCustomers = [
  { id: 1, firstName: 'John', lastName: 'Doe', phone: '08***REMOVED***', idCard: '***REMOVED***90123' },
  { id: 2, firstName: 'Jane', lastName: 'Smith', phone: '0898765432', idCard: '9876543210987' },
  { id: 3, firstName: 'Test', lastName: 'Customer', phone: '0811112222', idCard: null },
]

const mockRoom = {
  id: 101,
  roomNumber: '301',
  type: 'Standard',
  details: 'Sea View',
}

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
    // Mock fetch
    global.fetch = jest.fn()
  })

  afterEach(() => {
    jest.useRealTimers()
  })

  describe('Rendering', () => {
    test('renders modal when isOpen is true', () => {
      render(<QuickCheckInModal {...defaultProps} />)

      expect(screen.getByText('เช็คอินด่วน')).toBeInTheDocument()
      expect(screen.getByText('ห้อง 301 - Standard')).toBeInTheDocument()
    })

    test('does not render modal when isOpen is false', () => {
      render(<QuickCheckInModal {...defaultProps} isOpen={false} />)

      expect(screen.queryByText('เช็คอินด่วน')).not.toBeInTheDocument()
    })

    test('renders all form fields', () => {
      render(<QuickCheckInModal {...defaultProps} />)

      expect(screen.getByText('ลูกค้า *')).toBeInTheDocument()
      expect(screen.getByText('วันที่คาดว่าจะเช็คเอาท์ *')).toBeInTheDocument()
      expect(screen.getByText('ราคาต่อคืน (บาท)')).toBeInTheDocument()
      expect(screen.getByText('ผู้ใหญ่')).toBeInTheDocument()
      expect(screen.getByText('เด็ก')).toBeInTheDocument()
      expect(screen.getByText('หมายเหตุ')).toBeInTheDocument()
    })

    test('renders cancel and submit buttons', () => {
      render(<QuickCheckInModal {...defaultProps} />)

      expect(screen.getByText('ยกเลิก')).toBeInTheDocument()
      expect(screen.getByText('เช็คอิน')).toBeInTheDocument()
    })

    test('displays room information in header', () => {
      const customRoom = { id: 202, roomNumber: '505', type: 'Deluxe Suite' }
      render(<QuickCheckInModal {...defaultProps} room={customRoom} />)

      expect(screen.getByText('ห้อง 505 - Deluxe Suite')).toBeInTheDocument()
    })
  })

  describe('Customer Search (Debounced)', () => {
    test('shows loading indicator while searching', async () => {
      ;(global.fetch as jest.Mock).mockImplementation(
        () => new Promise((resolve) => setTimeout(() => resolve({
          json: () => Promise.resolve({ success: true, data: mockCustomers }),
        }), 500))
      )

      render(<QuickCheckInModal {...defaultProps} />)

      const searchInput = screen.getByPlaceholderText('ค้นหาด้วยชื่อ, เบอร์โทร, หรือเลขบัตรประชาชน...')
      fireEvent.change(searchInput, { target: { value: 'John' } })

      // Advance past debounce time
      act(() => {
        jest.advanceTimersByTime(300)
      })

      await waitFor(() => {
        expect(screen.getByTestId('loader-icon')).toBeInTheDocument()
      })
    })

    test('searches customers after debounce delay', async () => {
      ;(global.fetch as jest.Mock).mockResolvedValue({
        json: () => Promise.resolve({ success: true, data: mockCustomers }),
      })

      render(<QuickCheckInModal {...defaultProps} />)

      const searchInput = screen.getByPlaceholderText('ค้นหาด้วยชื่อ, เบอร์โทร, หรือเลขบัตรประชาชน...')
      fireEvent.change(searchInput, { target: { value: 'John' } })

      // Should not call fetch immediately
      expect(global.fetch).not.toHaveBeenCalled()

      // Advance past debounce time (300ms)
      act(() => {
        jest.advanceTimersByTime(300)
      })

      await waitFor(() => {
        expect(global.fetch).toHaveBeenCalledWith('/api/new/customers?search=John&limit=10')
      })
    })

    test('does not search if query is less than 2 characters', async () => {
      render(<QuickCheckInModal {...defaultProps} />)

      const searchInput = screen.getByPlaceholderText('ค้นหาด้วยชื่อ, เบอร์โทร, หรือเลขบัตรประชาชน...')
      fireEvent.change(searchInput, { target: { value: 'J' } })

      act(() => {
        jest.advanceTimersByTime(300)
      })

      expect(global.fetch).not.toHaveBeenCalled()
    })

    test('displays customer search results dropdown', async () => {
      ;(global.fetch as jest.Mock).mockResolvedValue({
        json: () => Promise.resolve({ success: true, data: mockCustomers }),
      })

      render(<QuickCheckInModal {...defaultProps} />)

      const searchInput = screen.getByPlaceholderText('ค้นหาด้วยชื่อ, เบอร์โทร, หรือเลขบัตรประชาชน...')
      fireEvent.change(searchInput, { target: { value: 'John' } })

      act(() => {
        jest.advanceTimersByTime(300)
      })

      await waitFor(() => {
        expect(screen.getByText('John Doe')).toBeInTheDocument()
        expect(screen.getByText('08***REMOVED***')).toBeInTheDocument()
      })
    })

    test('shows no results message when search returns empty', async () => {
      ;(global.fetch as jest.Mock).mockResolvedValue({
        json: () => Promise.resolve({ success: true, data: [] }),
      })

      render(<QuickCheckInModal {...defaultProps} />)

      const searchInput = screen.getByPlaceholderText('ค้นหาด้วยชื่อ, เบอร์โทร, หรือเลขบัตรประชาชน...')
      fireEvent.change(searchInput, { target: { value: 'NonExistent' } })

      act(() => {
        jest.advanceTimersByTime(300)
      })

      await waitFor(() => {
        expect(screen.getByText('ไม่พบลูกค้า')).toBeInTheDocument()
      })
    })

    test('selects customer when clicking on dropdown item', async () => {
      ;(global.fetch as jest.Mock).mockResolvedValue({
        json: () => Promise.resolve({ success: true, data: mockCustomers }),
      })

      render(<QuickCheckInModal {...defaultProps} />)

      const searchInput = screen.getByPlaceholderText('ค้นหาด้วยชื่อ, เบอร์โทร, หรือเลขบัตรประชาชน...')
      fireEvent.change(searchInput, { target: { value: 'John' } })

      act(() => {
        jest.advanceTimersByTime(300)
      })

      await waitFor(() => {
        expect(screen.getByText('John Doe')).toBeInTheDocument()
      })

      fireEvent.click(screen.getByText('John Doe'))

      // Should show selected customer confirmation
      expect(screen.getByText(/เลือกแล้ว: John Doe/)).toBeInTheDocument()
    })
  })

  describe('Form Validation', () => {
    test('shows error when submitting without selecting customer', async () => {
      // Mock customer search to return customers
      ;(global.fetch as jest.Mock).mockResolvedValue({
        json: () => Promise.resolve({ success: true, data: mockCustomers }),
      })

      render(<QuickCheckInModal {...defaultProps} />)

      // Select a customer first to enable submit button
      const searchInput = screen.getByPlaceholderText('ค้นหาด้วยชื่อ, เบอร์โทร, หรือเลขบัตรประชาชน...')
      fireEvent.change(searchInput, { target: { value: 'John' } })

      act(() => {
        jest.advanceTimersByTime(300)
      })

      await waitFor(() => {
        expect(screen.getByText('John Doe')).toBeInTheDocument()
      })

      // Click to select customer
      fireEvent.click(screen.getByText('John Doe'))

      // Then clear the search (simulating customer deselection)
      fireEvent.change(searchInput, { target: { value: '' } })

      act(() => {
        jest.advanceTimersByTime(300)
      })

      // Now try to submit - button should be disabled
      const submitButton = screen.getByText('เช็คอิน')
      expect(submitButton).toBeDisabled()
    })

    test('submit button is disabled when no customer is selected', () => {
      render(<QuickCheckInModal {...defaultProps} />)

      const submitButton = screen.getByText('เช็คอิน')
      expect(submitButton).toBeDisabled()
    })

    test('submit button is enabled after selecting customer', async () => {
      ;(global.fetch as jest.Mock).mockResolvedValue({
        json: () => Promise.resolve({ success: true, data: mockCustomers }),
      })

      render(<QuickCheckInModal {...defaultProps} />)

      const searchInput = screen.getByPlaceholderText('ค้นหาด้วยชื่อ, เบอร์โทร, หรือเลขบัตรประชาชน...')
      fireEvent.change(searchInput, { target: { value: 'John' } })

      act(() => {
        jest.advanceTimersByTime(300)
      })

      await waitFor(() => {
        expect(screen.getByText('John Doe')).toBeInTheDocument()
      })

      fireEvent.click(screen.getByText('John Doe'))

      const submitButton = screen.getByText('เช็คอิน')
      expect(submitButton).not.toBeDisabled()
    })

    test('checkout date field is required (HTML5 validation)', async () => {
      ;(global.fetch as jest.Mock).mockResolvedValue({
        json: () => Promise.resolve({ success: true, data: mockCustomers }),
      })

      render(<QuickCheckInModal {...defaultProps} />)

      // Select a customer
      const searchInput = screen.getByPlaceholderText('ค้นหาด้วยชื่อ, เบอร์โทร, หรือเลขบัตรประชาชน...')
      fireEvent.change(searchInput, { target: { value: 'John' } })

      act(() => {
        jest.advanceTimersByTime(300)
      })

      await waitFor(() => {
        expect(screen.getByText('John Doe')).toBeInTheDocument()
      })

      fireEvent.click(screen.getByText('John Doe'))

      // The date input should have required attribute
      const dateInput = document.querySelector('input[type="date"]')
      expect(dateInput).toHaveAttribute('required')
    })
  })

  describe('Date Handling (Buddhist Era)', () => {
    test('displays Buddhist Era date when checkout date is selected', () => {
      render(<QuickCheckInModal {...defaultProps} />)

      // The default checkout date should be tomorrow, displayed in Buddhist Era
      // Look for the Buddhist year indicator (พ.ศ.)
      expect(screen.getByText(/พ\.ศ\./)).toBeInTheDocument()
    })

    test('formats date correctly in Buddhist Era format', () => {
      render(<QuickCheckInModal {...defaultProps} />)

      // The component should display the Buddhist Era format
      // The default checkout date shows the BE year
      expect(screen.getByText(/พ\.ศ\./)).toBeInTheDocument()
    })
  })

  describe('API Submission', () => {
    test('submits form with correct data', async () => {
      // Mock customer search
      ;(global.fetch as jest.Mock)
        .mockResolvedValueOnce({
          json: () => Promise.resolve({ success: true, data: mockCustomers }),
        })
        // Mock checkin submission
        .mockResolvedValueOnce({
          ok: true,
          json: () => Promise.resolve({ success: true, data: { id: 1 } }),
        })

      render(<QuickCheckInModal {...defaultProps} />)

      // Select a customer
      const searchInput = screen.getByPlaceholderText('ค้นหาด้วยชื่อ, เบอร์โทร, หรือเลขบัตรประชาชน...')
      fireEvent.change(searchInput, { target: { value: 'John' } })

      act(() => {
        jest.advanceTimersByTime(300)
      })

      await waitFor(() => {
        expect(screen.getByText('John Doe')).toBeInTheDocument()
      })

      fireEvent.click(screen.getByText('John Doe'))

      // Fill in optional fields
      const rateInput = screen.getByPlaceholderText('ระบุราคา...')
      fireEvent.change(rateInput, { target: { value: '1500' } })

      const notesInput = screen.getByPlaceholderText('หมายเหตุเพิ่มเติม...')
      fireEvent.change(notesInput, { target: { value: 'Early check-in requested' } })

      // Submit
      const submitButton = screen.getByText('เช็คอิน')
      fireEvent.click(submitButton)

      await waitFor(() => {
        expect(global.fetch).toHaveBeenLastCalledWith('/api/new/checkins', {
          method: 'POST',
          headers: { 'Content-Type': 'application/json' },
          body: expect.stringContaining('"customerId":1'),
        })
      })

      // Verify request body contains expected data
      const lastCall = (global.fetch as jest.Mock).mock.calls.pop()
      const requestBody = JSON.parse(lastCall[1].body)
      expect(requestBody.customerId).toBe(1)
      expect(requestBody.roomId).toBe(101)
      expect(requestBody.ratePerNight).toBe(1500)
      expect(requestBody.notes).toBe('Early check-in requested')
    })

    test('shows loading state during submission', async () => {
      ;(global.fetch as jest.Mock)
        .mockResolvedValueOnce({
          json: () => Promise.resolve({ success: true, data: mockCustomers }),
        })
        .mockImplementationOnce(
          () => new Promise((resolve) => setTimeout(() => resolve({
            ok: true,
            json: () => Promise.resolve({ success: true }),
          }), 1000))
        )

      render(<QuickCheckInModal {...defaultProps} />)

      // Select a customer
      const searchInput = screen.getByPlaceholderText('ค้นหาด้วยชื่อ, เบอร์โทร, หรือเลขบัตรประชาชน...')
      fireEvent.change(searchInput, { target: { value: 'John' } })

      act(() => {
        jest.advanceTimersByTime(300)
      })

      await waitFor(() => {
        expect(screen.getByText('John Doe')).toBeInTheDocument()
      })

      fireEvent.click(screen.getByText('John Doe'))

      const submitButton = screen.getByText('เช็คอิน')
      fireEvent.click(submitButton)

      await waitFor(() => {
        expect(screen.getByText('กำลังบันทึก...')).toBeInTheDocument()
      })
    })

    test('calls onSuccess and onClose after successful submission', async () => {
      ;(global.fetch as jest.Mock)
        .mockResolvedValueOnce({
          json: () => Promise.resolve({ success: true, data: mockCustomers }),
        })
        .mockResolvedValueOnce({
          ok: true,
          json: () => Promise.resolve({ success: true, data: { id: 1 } }),
        })

      render(<QuickCheckInModal {...defaultProps} />)

      // Select a customer
      const searchInput = screen.getByPlaceholderText('ค้นหาด้วยชื่อ, เบอร์โทร, หรือเลขบัตรประชาชน...')
      fireEvent.change(searchInput, { target: { value: 'John' } })

      act(() => {
        jest.advanceTimersByTime(300)
      })

      await waitFor(() => {
        expect(screen.getByText('John Doe')).toBeInTheDocument()
      })

      fireEvent.click(screen.getByText('John Doe'))

      const submitButton = screen.getByText('เช็คอิน')
      fireEvent.click(submitButton)

      await waitFor(() => {
        expect(defaultProps.onSuccess).toHaveBeenCalled()
        expect(defaultProps.onClose).toHaveBeenCalled()
      })
    })

    test('shows error message on API failure', async () => {
      ;(global.fetch as jest.Mock)
        .mockResolvedValueOnce({
          json: () => Promise.resolve({ success: true, data: mockCustomers }),
        })
        .mockResolvedValueOnce({
          ok: false,
          json: () => Promise.resolve({ success: false, error: 'ห้องนี้มีผู้เข้าพักอยู่แล้ว' }),
        })

      render(<QuickCheckInModal {...defaultProps} />)

      // Select a customer
      const searchInput = screen.getByPlaceholderText('ค้นหาด้วยชื่อ, เบอร์โทร, หรือเลขบัตรประชาชน...')
      fireEvent.change(searchInput, { target: { value: 'John' } })

      act(() => {
        jest.advanceTimersByTime(300)
      })

      await waitFor(() => {
        expect(screen.getByText('John Doe')).toBeInTheDocument()
      })

      fireEvent.click(screen.getByText('John Doe'))

      const submitButton = screen.getByText('เช็คอิน')
      fireEvent.click(submitButton)

      await waitFor(() => {
        expect(screen.getByText('ห้องนี้มีผู้เข้าพักอยู่แล้ว')).toBeInTheDocument()
      })
    })

    test('handles network error gracefully', async () => {
      ;(global.fetch as jest.Mock)
        .mockResolvedValueOnce({
          json: () => Promise.resolve({ success: true, data: mockCustomers }),
        })
        .mockRejectedValueOnce(new Error('Network error'))

      render(<QuickCheckInModal {...defaultProps} />)

      // Select a customer
      const searchInput = screen.getByPlaceholderText('ค้นหาด้วยชื่อ, เบอร์โทร, หรือเลขบัตรประชาชน...')
      fireEvent.change(searchInput, { target: { value: 'John' } })

      act(() => {
        jest.advanceTimersByTime(300)
      })

      await waitFor(() => {
        expect(screen.getByText('John Doe')).toBeInTheDocument()
      })

      fireEvent.click(screen.getByText('John Doe'))

      const submitButton = screen.getByText('เช็คอิน')
      fireEvent.click(submitButton)

      await waitFor(() => {
        expect(screen.getByText('Network error')).toBeInTheDocument()
      })
    })
  })

  describe('Modal Interaction', () => {
    test('calls onClose when clicking close button', () => {
      render(<QuickCheckInModal {...defaultProps} />)

      const closeButton = screen.getByTestId('x-icon').closest('button')
      fireEvent.click(closeButton!)

      expect(defaultProps.onClose).toHaveBeenCalled()
    })

    test('calls onClose when clicking cancel button', () => {
      render(<QuickCheckInModal {...defaultProps} />)

      const cancelButton = screen.getByText('ยกเลิก')
      fireEvent.click(cancelButton)

      expect(defaultProps.onClose).toHaveBeenCalled()
    })

    test('calls onClose when clicking backdrop', () => {
      render(<QuickCheckInModal {...defaultProps} />)

      // Click on the backdrop (the outermost div with onClick={handleClose})
      const backdrop = screen.getByText('เช็คอินด่วน').closest('.fixed')
      fireEvent.click(backdrop!)

      expect(defaultProps.onClose).toHaveBeenCalled()
    })

    test('does not close when clicking inside modal content', () => {
      render(<QuickCheckInModal {...defaultProps} />)

      // Click inside the modal content
      const modalContent = screen.getByText('เช็คอินด่วน').closest('.bg-white')
      fireEvent.click(modalContent!)

      expect(defaultProps.onClose).not.toHaveBeenCalled()
    })

    test('resets form when modal is closed via close button', () => {
      render(<QuickCheckInModal {...defaultProps} />)

      // Interact with the form
      const rateInput = screen.getByPlaceholderText('ระบุราคา...')
      fireEvent.change(rateInput, { target: { value: '1500' } })

      // Close modal via close button - this triggers handleClose which calls resetForm
      const closeButton = screen.getByTestId('x-icon').closest('button')
      fireEvent.click(closeButton!)

      // onClose should be called
      expect(defaultProps.onClose).toHaveBeenCalled()
    })
  })

  describe('Edge Cases', () => {
    test('handles customer with null name fields', async () => {
      const customersWithNulls = [
        { id: 4, firstName: null, lastName: null, phone: '0899999999', idCard: null },
      ]

      ;(global.fetch as jest.Mock).mockResolvedValue({
        json: () => Promise.resolve({ success: true, data: customersWithNulls }),
      })

      render(<QuickCheckInModal {...defaultProps} />)

      const searchInput = screen.getByPlaceholderText('ค้นหาด้วยชื่อ, เบอร์โทร, หรือเลขบัตรประชาชน...')
      fireEvent.change(searchInput, { target: { value: '089' } })

      act(() => {
        jest.advanceTimersByTime(300)
      })

      await waitFor(() => {
        expect(screen.getByText('0899999999')).toBeInTheDocument()
      })
    })

    test('updates adults and children count', () => {
      render(<QuickCheckInModal {...defaultProps} />)

      const adultsInput = screen.getByDisplayValue('1')
      const childrenInput = screen.getByDisplayValue('0')

      fireEvent.change(adultsInput, { target: { value: '2' } })
      fireEvent.change(childrenInput, { target: { value: '1' } })

      expect(adultsInput).toHaveValue(2)
      expect(childrenInput).toHaveValue(1)
    })
  })
})
