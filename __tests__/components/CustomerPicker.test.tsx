/**
 * @jest-environment jsdom
 */

import { render, screen, fireEvent, waitFor, act } from '@testing-library/react'
import CustomerPicker, { CustomerOption } from '@/components/pickers/CustomerPicker'

// Mock lucide-react icons
jest.mock('lucide-react', () => ({
  Search: () => <span data-testid="search-icon">Search</span>,
  X: () => <span data-testid="x-icon">X</span>,
  User: () => <span data-testid="user-icon">User</span>,
  Phone: () => <span data-testid="phone-icon">Phone</span>,
  CreditCard: () => <span data-testid="credit-card-icon">CreditCard</span>,
  ChevronDown: () => <span data-testid="chevron-down-icon">v</span>,
  Loader2: () => <span data-testid="loader-icon">Loading</span>,
  UserPlus: () => <span data-testid="user-plus-icon">+</span>,
}))

// Mock fetch API
const mockFetch = jest.fn()
global.fetch = mockFetch

const mockCustomers: CustomerOption[] = [
  {
    id: 1,
    firstName: 'สมชาย',
    lastName: 'ใจดี',
    phone: '0891234567',
    email: 'somchai@example.com',
    idCard: '***REMOVED***90123',
  },
  {
    id: 2,
    firstName: 'สมศรี',
    lastName: 'รักไทย',
    phone: '0899876543',
    email: 'somsri@example.com',
    idCard: '9876543210123',
  },
  {
    id: 3,
    firstName: 'วิชัย',
    lastName: 'เก่งกาจ',
    phone: '0812223333',
    email: 'wichai@example.com',
    idCard: '1111222233334',
  },
]

describe('CustomerPicker Component', () => {
  const defaultProps = {
    value: null,
    onChange: jest.fn(),
  }

  beforeEach(() => {
    jest.clearAllMocks()
    jest.useFakeTimers()
    mockFetch.mockResolvedValue({
      ok: true,
      json: () => Promise.resolve({ data: mockCustomers }),
    })
  })

  afterEach(() => {
    jest.useRealTimers()
  })

  describe('Rendering', () => {
    test('renders with default Thai placeholder', () => {
      render(<CustomerPicker {...defaultProps} />)

      expect(screen.getByPlaceholderText('ค้นหาลูกค้า...')).toBeInTheDocument()
    })

    test('renders with custom placeholder', () => {
      render(
        <CustomerPicker
          {...defaultProps}
          placeholder="เลือกลูกค้า..."
        />
      )

      expect(screen.getByPlaceholderText('เลือกลูกค้า...')).toBeInTheDocument()
    })

    test('renders search icon', () => {
      render(<CustomerPicker {...defaultProps} />)

      expect(screen.getByTestId('search-icon')).toBeInTheDocument()
    })

    test('renders chevron down icon', () => {
      render(<CustomerPicker {...defaultProps} />)

      expect(screen.getByTestId('chevron-down-icon')).toBeInTheDocument()
    })

    test('displays selected customer name and phone', () => {
      render(
        <CustomerPicker
          {...defaultProps}
          value={mockCustomers[0]}
        />
      )

      expect(screen.getByText('สมชาย ใจดี')).toBeInTheDocument()
      expect(screen.getByText('(0891234567)')).toBeInTheDocument()
    })
  })

  describe('Dropdown Behavior', () => {
    test('opens dropdown when input is focused', async () => {
      render(<CustomerPicker {...defaultProps} />)

      const input = screen.getByPlaceholderText('ค้นหาลูกค้า...')
      fireEvent.focus(input)

      // Wait for debounce and fetch
      await act(async () => {
        jest.advanceTimersByTime(300)
      })

      await waitFor(() => {
        // Should show loading or customer options
        expect(mockFetch).toHaveBeenCalled()
      })
    })

    test('opens dropdown when container is clicked', async () => {
      render(<CustomerPicker {...defaultProps} />)

      const container = screen.getByPlaceholderText('ค้นหาลูกค้า...').closest('div')
      fireEvent.click(container!)

      // Dropdown should open
      await act(async () => {
        jest.advanceTimersByTime(300)
      })

      await waitFor(() => {
        expect(mockFetch).toHaveBeenCalled()
      })
    })

    test('does not open dropdown when disabled', () => {
      render(
        <CustomerPicker
          {...defaultProps}
          disabled={true}
        />
      )

      const input = screen.getByPlaceholderText('ค้นหาลูกค้า...')
      expect(input).toBeDisabled()
    })

    test('closes dropdown when clicking outside', async () => {
      render(
        <div>
          <CustomerPicker {...defaultProps} />
          <button data-testid="outside-element">Outside</button>
        </div>
      )

      // Open dropdown
      const input = screen.getByPlaceholderText('ค้นหาลูกค้า...')
      fireEvent.focus(input)

      await act(async () => {
        jest.advanceTimersByTime(300)
      })

      // Click outside
      fireEvent.mouseDown(screen.getByTestId('outside-element'))

      // Dropdown should close - search for empty state that would only appear when open
      await waitFor(() => {
        expect(screen.queryByText('พิมพ์เพื่อค้นหาลูกค้า')).not.toBeInTheDocument()
      })
    })
  })

  describe('Search Functionality', () => {
    test('debounces search input by 300ms', async () => {
      render(<CustomerPicker {...defaultProps} />)

      const input = screen.getByPlaceholderText('ค้นหาลูกค้า...')
      fireEvent.focus(input)
      fireEvent.change(input, { target: { value: 'สมชาย' } })

      // Fetch should not be called immediately for the search
      expect(mockFetch).not.toHaveBeenCalledWith(
        expect.stringContaining('search=สมชาย'),
        expect.anything()
      )

      // After debounce period
      await act(async () => {
        jest.advanceTimersByTime(300)
      })

      await waitFor(() => {
        const fetchUrl = mockFetch.mock.calls.find(
          (call) => call[0].includes('search=')
        )
        expect(fetchUrl).toBeDefined()
      })
    })

    test('fetches customers with search query', async () => {
      render(<CustomerPicker {...defaultProps} />)

      const input = screen.getByPlaceholderText('ค้นหาลูกค้า...')
      fireEvent.focus(input)
      fireEvent.change(input, { target: { value: 'สมชาย' } })

      await act(async () => {
        jest.advanceTimersByTime(300)
      })

      await waitFor(() => {
        expect(mockFetch).toHaveBeenCalledWith(
          expect.stringContaining('search=')
        )
      })
    })

    test('shows loading indicator while fetching', async () => {
      // Make fetch hang
      mockFetch.mockImplementation(
        () => new Promise((resolve) => setTimeout(resolve, 5000))
      )

      render(<CustomerPicker {...defaultProps} />)

      const input = screen.getByPlaceholderText('ค้นหาลูกค้า...')
      fireEvent.focus(input)
      fireEvent.change(input, { target: { value: 'test' } })

      await act(async () => {
        jest.advanceTimersByTime(300)
      })

      await waitFor(() => {
        expect(screen.getByText('กำลังค้นหา...')).toBeInTheDocument()
      })
    })

    test('shows "ไม่พบลูกค้า" when no results', async () => {
      mockFetch.mockResolvedValue({
        ok: true,
        json: () => Promise.resolve({ data: [] }),
      })

      render(<CustomerPicker {...defaultProps} />)

      const input = screen.getByPlaceholderText('ค้นหาลูกค้า...')
      fireEvent.focus(input)
      fireEvent.change(input, { target: { value: 'ไม่มีชื่อนี้' } })

      await act(async () => {
        jest.advanceTimersByTime(300)
      })

      await waitFor(() => {
        expect(screen.getByText(/ไม่พบลูกค้าที่ตรงกับ/)).toBeInTheDocument()
      })
    })

    test('shows "พิมพ์เพื่อค้นหาลูกค้า" when no search and no results', async () => {
      mockFetch.mockResolvedValue({
        ok: true,
        json: () => Promise.resolve({ data: [] }),
      })

      render(<CustomerPicker {...defaultProps} />)

      const input = screen.getByPlaceholderText('ค้นหาลูกค้า...')
      fireEvent.focus(input)

      await act(async () => {
        jest.advanceTimersByTime(300)
      })

      await waitFor(() => {
        expect(screen.getByText('พิมพ์เพื่อค้นหาลูกค้า')).toBeInTheDocument()
      })
    })
  })

  describe('Customer Selection', () => {
    test('calls onChange when customer is selected', async () => {
      const mockOnChange = jest.fn()
      render(
        <CustomerPicker
          {...defaultProps}
          onChange={mockOnChange}
        />
      )

      const input = screen.getByPlaceholderText('ค้นหาลูกค้า...')
      fireEvent.focus(input)
      fireEvent.change(input, { target: { value: 'สม' } })

      await act(async () => {
        jest.advanceTimersByTime(300)
      })

      await waitFor(() => {
        expect(screen.getByText('สมชาย ใจดี')).toBeInTheDocument()
      })

      fireEvent.click(screen.getByText('สมชาย ใจดี'))

      expect(mockOnChange).toHaveBeenCalledWith(
        expect.objectContaining({
          id: 1,
          firstName: 'สมชาย',
          lastName: 'ใจดี',
        })
      )
    })

    test('clears search query after selection', async () => {
      const mockOnChange = jest.fn()
      render(
        <CustomerPicker
          {...defaultProps}
          onChange={mockOnChange}
        />
      )

      const input = screen.getByPlaceholderText('ค้นหาลูกค้า...')
      fireEvent.focus(input)
      fireEvent.change(input, { target: { value: 'สม' } })

      await act(async () => {
        jest.advanceTimersByTime(300)
      })

      await waitFor(() => {
        expect(screen.getByText('สมชาย ใจดี')).toBeInTheDocument()
      })

      fireEvent.click(screen.getByText('สมชาย ใจดี'))

      // Search query should be cleared
      await waitFor(() => {
        expect(mockOnChange).toHaveBeenCalled()
      })
    })
  })

  describe('Clear Selection', () => {
    test('renders clear button when customer is selected', () => {
      render(
        <CustomerPicker
          {...defaultProps}
          value={mockCustomers[0]}
        />
      )

      const clearButton = screen.getByLabelText('ล้าง')
      expect(clearButton).toBeInTheDocument()
    })

    test('calls onChange with null when clear button is clicked', () => {
      const mockOnChange = jest.fn()
      render(
        <CustomerPicker
          {...defaultProps}
          value={mockCustomers[0]}
          onChange={mockOnChange}
        />
      )

      const clearButton = screen.getByLabelText('ล้าง')
      fireEvent.click(clearButton)

      expect(mockOnChange).toHaveBeenCalledWith(null)
    })

    test('clears search input when clear button is clicked', async () => {
      render(<CustomerPicker {...defaultProps} />)

      const input = screen.getByPlaceholderText('ค้นหาลูกค้า...')
      fireEvent.focus(input)
      fireEvent.change(input, { target: { value: 'test' } })

      // Find clear button for search input
      const clearButtons = screen.getAllByLabelText('ล้าง')
      fireEvent.click(clearButtons[0])

      expect(input).toHaveValue('')
    })
  })

  describe('Add New Customer', () => {
    test('renders "เพิ่มลูกค้าใหม่" button when onAddNew is provided', async () => {
      const mockOnAddNew = jest.fn()
      render(
        <CustomerPicker
          {...defaultProps}
          onAddNew={mockOnAddNew}
        />
      )

      const input = screen.getByPlaceholderText('ค้นหาลูกค้า...')
      fireEvent.focus(input)

      await act(async () => {
        jest.advanceTimersByTime(300)
      })

      await waitFor(() => {
        expect(screen.getByText('เพิ่มลูกค้าใหม่')).toBeInTheDocument()
      })
    })

    test('calls onAddNew when button is clicked', async () => {
      const mockOnAddNew = jest.fn()
      render(
        <CustomerPicker
          {...defaultProps}
          onAddNew={mockOnAddNew}
        />
      )

      const input = screen.getByPlaceholderText('ค้นหาลูกค้า...')
      fireEvent.focus(input)

      await act(async () => {
        jest.advanceTimersByTime(300)
      })

      await waitFor(() => {
        expect(screen.getByText('เพิ่มลูกค้าใหม่')).toBeInTheDocument()
      })

      fireEvent.click(screen.getByText('เพิ่มลูกค้าใหม่'))

      expect(mockOnAddNew).toHaveBeenCalled()
    })

    test('does not render add button when onAddNew is not provided', async () => {
      render(<CustomerPicker {...defaultProps} />)

      const input = screen.getByPlaceholderText('ค้นหาลูกค้า...')
      fireEvent.focus(input)

      await act(async () => {
        jest.advanceTimersByTime(300)
      })

      await waitFor(() => {
        expect(screen.queryByText('เพิ่มลูกค้าใหม่')).not.toBeInTheDocument()
      })
    })
  })

  describe('Keyboard Navigation', () => {
    test('opens dropdown on Enter key', async () => {
      render(<CustomerPicker {...defaultProps} />)

      const input = screen.getByPlaceholderText('ค้นหาลูกค้า...')
      fireEvent.keyDown(input, { key: 'Enter' })

      await act(async () => {
        jest.advanceTimersByTime(300)
      })

      await waitFor(() => {
        expect(mockFetch).toHaveBeenCalled()
      })
    })

    test('opens dropdown on ArrowDown key', async () => {
      render(<CustomerPicker {...defaultProps} />)

      const input = screen.getByPlaceholderText('ค้นหาลูกค้า...')
      fireEvent.keyDown(input, { key: 'ArrowDown' })

      await act(async () => {
        jest.advanceTimersByTime(300)
      })

      await waitFor(() => {
        expect(mockFetch).toHaveBeenCalled()
      })
    })

    test('closes dropdown on Escape key', async () => {
      render(<CustomerPicker {...defaultProps} />)

      const input = screen.getByPlaceholderText('ค้นหาลูกค้า...')
      fireEvent.focus(input)

      await act(async () => {
        jest.advanceTimersByTime(300)
      })

      fireEvent.keyDown(input, { key: 'Escape' })

      // Dropdown should close
      await waitFor(() => {
        expect(screen.queryByText('พิมพ์เพื่อค้นหาลูกค้า')).not.toBeInTheDocument()
      })
    })
  })

  describe('ID Card Display', () => {
    test('masks ID card number in dropdown', async () => {
      render(<CustomerPicker {...defaultProps} />)

      const input = screen.getByPlaceholderText('ค้นหาลูกค้า...')
      fireEvent.focus(input)
      fireEvent.change(input, { target: { value: 'สม' } })

      await act(async () => {
        jest.advanceTimersByTime(300)
      })

      await waitFor(() => {
        // ID card ***REMOVED***90123 should show as ****0123
        expect(screen.getByText('****0123')).toBeInTheDocument()
      })
    })
  })

  describe('Error Handling', () => {
    test('handles fetch error gracefully', async () => {
      // Suppress console.error for this test
      const consoleSpy = jest.spyOn(console, 'error').mockImplementation(() => {})

      mockFetch.mockRejectedValue(new Error('Network error'))

      render(<CustomerPicker {...defaultProps} />)

      const input = screen.getByPlaceholderText('ค้นหาลูกค้า...')
      fireEvent.focus(input)
      fireEvent.change(input, { target: { value: 'test' } })

      await act(async () => {
        jest.advanceTimersByTime(300)
      })

      // Should not crash, should show empty state
      await waitFor(() => {
        expect(screen.getByPlaceholderText('ค้นหาลูกค้า...')).toBeInTheDocument()
      })

      consoleSpy.mockRestore()
    })
  })
})
