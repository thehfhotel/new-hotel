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
  ChevronDown: () => <span data-testid="chevron-down-icon">ChevronDown</span>,
  Loader2: () => <span data-testid="loader-icon">Loader</span>,
  UserPlus: () => <span data-testid="user-plus-icon">UserPlus</span>,
}))

// Mock fetch
const mockFetch = jest.fn()
global.fetch = mockFetch

// Mock scrollIntoView which is not available in jsdom
Element.prototype.scrollIntoView = jest.fn()

const mockCustomers: CustomerOption[] = [
  {
    id: 1,
    firstName: 'สมชาย',
    lastName: 'ใจดี',
    phone: '081-234-5678',
    email: 'somchai@email.com',
    idCard: '***REMOVED***90123',
  },
  {
    id: 2,
    firstName: 'สมหญิง',
    lastName: 'รักดี',
    phone: '089-876-5432',
    email: 'somying@email.com',
    idCard: '9876543210987',
  },
  {
    id: 3,
    firstName: 'John',
    lastName: 'Doe',
    phone: '099-999-9999',
    email: 'john@example.com',
    idCard: '',
  },
]

const mockApiResponse = {
  success: true,
  data: mockCustomers,
}

// Helper function to trigger fetch and wait for results
async function openDropdownAndWaitForResults(input: HTMLElement) {
  await act(async () => {
    fireEvent.focus(input)
  })

  // Advance timer past debounce
  await act(async () => {
    jest.advanceTimersByTime(350)
  })

  // Wait for fetch to resolve
  await act(async () => {
    await Promise.resolve()
  })
}

describe('CustomerPicker Component', () => {
  beforeEach(() => {
    jest.useFakeTimers()
    mockFetch.mockClear()
    mockFetch.mockResolvedValue({
      ok: true,
      json: () => Promise.resolve(mockApiResponse),
    })
  })

  afterEach(() => {
    jest.useRealTimers()
  })

  describe('Rendering', () => {
    test('renders placeholder when no value is selected', () => {
      render(<CustomerPicker value={null} onChange={jest.fn()} />)

      expect(screen.getByPlaceholderText('ค้นหาลูกค้า...')).toBeInTheDocument()
    })

    test('renders custom placeholder', () => {
      render(
        <CustomerPicker
          value={null}
          onChange={jest.fn()}
          placeholder="เลือกลูกค้า"
        />
      )

      expect(screen.getByPlaceholderText('เลือกลูกค้า')).toBeInTheDocument()
    })

    test('shows selected customer name and phone', () => {
      const selectedCustomer = mockCustomers[0]
      render(<CustomerPicker value={selectedCustomer} onChange={jest.fn()} />)

      expect(screen.getByText('สมชาย ใจดี')).toBeInTheDocument()
      expect(screen.getByText('(081-234-5678)')).toBeInTheDocument()
    })

    test('shows customer name without phone when phone is empty', () => {
      const customerWithoutPhone: CustomerOption = {
        ...mockCustomers[0],
        phone: '',
      }
      render(<CustomerPicker value={customerWithoutPhone} onChange={jest.fn()} />)

      expect(screen.getByText('สมชาย ใจดี')).toBeInTheDocument()
      expect(screen.queryByText(/\(\)/)).not.toBeInTheDocument()
    })

    test('shows dash for customer with no name', () => {
      const customerNoName: CustomerOption = {
        id: 999,
        firstName: '',
        lastName: '',
        phone: '000-000-0000',
        email: '',
        idCard: '',
      }
      render(<CustomerPicker value={customerNoName} onChange={jest.fn()} />)

      expect(screen.getByText('-')).toBeInTheDocument()
    })

    test('applies custom className', () => {
      const { container } = render(
        <CustomerPicker
          value={null}
          onChange={jest.fn()}
          className="custom-class"
        />
      )

      expect(container.firstChild).toHaveClass('custom-class')
    })
  })

  describe('Search with Debounce', () => {
    test('debounces search input for 300ms', async () => {
      render(<CustomerPicker value={null} onChange={jest.fn()} />)

      const input = screen.getByPlaceholderText('ค้นหาลูกค้า...')

      // Focus to open dropdown
      await act(async () => {
        fireEvent.focus(input)
      })

      // Type search query
      await act(async () => {
        fireEvent.change(input, { target: { value: 'สมชาย' } })
      })

      // API should be called once for initial open
      expect(mockFetch).toHaveBeenCalledTimes(1)

      // Advance time by 200ms - still not debounced
      await act(async () => {
        jest.advanceTimersByTime(200)
      })

      // Still only the initial call
      expect(mockFetch).toHaveBeenCalledTimes(1)

      // Advance time past debounce threshold
      await act(async () => {
        jest.advanceTimersByTime(150)
        await Promise.resolve()
      })

      // Now should have made the search call
      expect(mockFetch).toHaveBeenCalledTimes(2)

      // Check that search param was included
      const lastCall = mockFetch.mock.calls[1][0]
      expect(lastCall).toContain('search=')
    })

    test('clears search query on clear button click', async () => {
      render(<CustomerPicker value={null} onChange={jest.fn()} />)

      const input = screen.getByPlaceholderText('ค้นหาลูกค้า...')

      await act(async () => {
        fireEvent.focus(input)
        fireEvent.change(input, { target: { value: 'test' } })
      })

      expect(input).toHaveValue('test')

      // Find and click clear button
      const clearButtons = screen.getAllByLabelText('ล้าง')
      await act(async () => {
        fireEvent.click(clearButtons[0])
      })

      expect(input).toHaveValue('')
    })
  })

  describe('Dropdown Behavior', () => {
    test('opens dropdown on click', async () => {
      render(<CustomerPicker value={null} onChange={jest.fn()} />)

      const input = screen.getByPlaceholderText('ค้นหาลูกค้า...')
      await openDropdownAndWaitForResults(input)

      expect(screen.getByRole('listbox')).toBeInTheDocument()
    })

    test('opens dropdown on input focus', async () => {
      render(<CustomerPicker value={null} onChange={jest.fn()} />)

      const input = screen.getByPlaceholderText('ค้นหาลูกค้า...')
      await openDropdownAndWaitForResults(input)

      expect(screen.getByRole('listbox')).toBeInTheDocument()
    })

    test('shows search results in dropdown', async () => {
      render(<CustomerPicker value={null} onChange={jest.fn()} />)

      const input = screen.getByPlaceholderText('ค้นหาลูกค้า...')
      await openDropdownAndWaitForResults(input)

      expect(screen.getByText('สมชาย ใจดี')).toBeInTheDocument()
      expect(screen.getByText('สมหญิง รักดี')).toBeInTheDocument()
      expect(screen.getByText('John Doe')).toBeInTheDocument()
    })

    test('shows customer phone in dropdown', async () => {
      render(<CustomerPicker value={null} onChange={jest.fn()} />)

      const input = screen.getByPlaceholderText('ค้นหาลูกค้า...')
      await openDropdownAndWaitForResults(input)

      expect(screen.getByText('081-234-5678')).toBeInTheDocument()
    })

    test('shows masked ID card in dropdown', async () => {
      render(<CustomerPicker value={null} onChange={jest.fn()} />)

      const input = screen.getByPlaceholderText('ค้นหาลูกค้า...')
      await openDropdownAndWaitForResults(input)

      // ID card should be masked: ****0123
      expect(screen.getByText('****0123')).toBeInTheDocument()
    })

    test('shows empty state when no search query and no results', async () => {
      mockFetch.mockResolvedValue({
        ok: true,
        json: () => Promise.resolve({ success: true, data: [] }),
      })

      render(<CustomerPicker value={null} onChange={jest.fn()} />)

      const input = screen.getByPlaceholderText('ค้นหาลูกค้า...')
      await openDropdownAndWaitForResults(input)

      expect(screen.getByText('พิมพ์เพื่อค้นหาลูกค้า')).toBeInTheDocument()
    })

    test('shows no results message when search has no matches', async () => {
      mockFetch.mockResolvedValue({
        ok: true,
        json: () => Promise.resolve({ success: true, data: [] }),
      })

      render(<CustomerPicker value={null} onChange={jest.fn()} />)

      const input = screen.getByPlaceholderText('ค้นหาลูกค้า...')

      await act(async () => {
        fireEvent.focus(input)
        fireEvent.change(input, { target: { value: 'ไม่มีลูกค้านี้' } })
      })

      await act(async () => {
        jest.advanceTimersByTime(350)
        await Promise.resolve()
      })

      expect(screen.getByText(/ไม่พบลูกค้าที่ตรงกับ/)).toBeInTheDocument()
    })

    test('closes dropdown on outside click', async () => {
      render(
        <div>
          <CustomerPicker value={null} onChange={jest.fn()} />
          <button data-testid="outside">Outside</button>
        </div>
      )

      const input = screen.getByPlaceholderText('ค้นหาลูกค้า...')
      await openDropdownAndWaitForResults(input)

      expect(screen.getByRole('listbox')).toBeInTheDocument()

      // Click outside
      await act(async () => {
        fireEvent.mouseDown(screen.getByTestId('outside'))
      })

      expect(screen.queryByRole('listbox')).not.toBeInTheDocument()
    })
  })

  describe('Customer Selection', () => {
    test('calls onChange when customer is selected', async () => {
      const mockOnChange = jest.fn()
      render(<CustomerPicker value={null} onChange={mockOnChange} />)

      const input = screen.getByPlaceholderText('ค้นหาลูกค้า...')
      await openDropdownAndWaitForResults(input)

      await act(async () => {
        fireEvent.click(screen.getByText('สมชาย ใจดี'))
      })

      expect(mockOnChange).toHaveBeenCalledWith(
        expect.objectContaining({
          id: 1,
          firstName: 'สมชาย',
          lastName: 'ใจดี',
        })
      )
    })

    test('closes dropdown after selection', async () => {
      render(<CustomerPicker value={null} onChange={jest.fn()} />)

      const input = screen.getByPlaceholderText('ค้นหาลูกค้า...')
      await openDropdownAndWaitForResults(input)

      expect(screen.getByRole('listbox')).toBeInTheDocument()

      await act(async () => {
        fireEvent.click(screen.getByText('สมชาย ใจดี'))
      })

      expect(screen.queryByRole('listbox')).not.toBeInTheDocument()
    })
  })

  describe('Clear Button Functionality', () => {
    test('calls onChange with null when clear button is clicked', async () => {
      const mockOnChange = jest.fn()
      render(
        <CustomerPicker value={mockCustomers[0]} onChange={mockOnChange} />
      )

      const clearButton = screen.getByLabelText('ล้าง')

      await act(async () => {
        fireEvent.click(clearButton)
      })

      expect(mockOnChange).toHaveBeenCalledWith(null)
    })

    test('clear button is not shown when disabled', () => {
      render(
        <CustomerPicker
          value={mockCustomers[0]}
          onChange={jest.fn()}
          disabled={true}
        />
      )

      expect(screen.queryByLabelText('ล้าง')).not.toBeInTheDocument()
    })
  })

  describe('Keyboard Navigation', () => {
    test('opens dropdown on Enter key when closed', async () => {
      render(<CustomerPicker value={null} onChange={jest.fn()} />)

      const input = screen.getByPlaceholderText('ค้นหาลูกค้า...')

      await act(async () => {
        fireEvent.keyDown(input, { key: 'Enter' })
      })

      await act(async () => {
        jest.advanceTimersByTime(350)
        await Promise.resolve()
      })

      expect(screen.getByRole('listbox')).toBeInTheDocument()
    })

    test('opens dropdown on ArrowDown key when closed', async () => {
      render(<CustomerPicker value={null} onChange={jest.fn()} />)

      const input = screen.getByPlaceholderText('ค้นหาลูกค้า...')

      await act(async () => {
        fireEvent.keyDown(input, { key: 'ArrowDown' })
      })

      await act(async () => {
        jest.advanceTimersByTime(350)
        await Promise.resolve()
      })

      expect(screen.getByRole('listbox')).toBeInTheDocument()
    })

    test('navigates down with ArrowDown key', async () => {
      render(<CustomerPicker value={null} onChange={jest.fn()} />)

      const input = screen.getByPlaceholderText('ค้นหาลูกค้า...')
      await openDropdownAndWaitForResults(input)

      // Navigate down
      await act(async () => {
        fireEvent.keyDown(input, { key: 'ArrowDown' })
      })

      // First item should be highlighted
      const listItems = screen.getAllByRole('option')
      expect(listItems[0]).toHaveClass('bg-blue-50')
    })

    test('navigates up with ArrowUp key', async () => {
      render(<CustomerPicker value={null} onChange={jest.fn()} />)

      const input = screen.getByPlaceholderText('ค้นหาลูกค้า...')
      await openDropdownAndWaitForResults(input)

      // Navigate down twice, then up once
      await act(async () => {
        fireEvent.keyDown(input, { key: 'ArrowDown' })
        fireEvent.keyDown(input, { key: 'ArrowDown' })
        fireEvent.keyDown(input, { key: 'ArrowUp' })
      })

      // First item should be highlighted
      const listItems = screen.getAllByRole('option')
      expect(listItems[0]).toHaveClass('bg-blue-50')
    })

    test('selects highlighted item on Enter key', async () => {
      const mockOnChange = jest.fn()
      render(<CustomerPicker value={null} onChange={mockOnChange} />)

      const input = screen.getByPlaceholderText('ค้นหาลูกค้า...')
      await openDropdownAndWaitForResults(input)

      // Navigate to first item
      await act(async () => {
        fireEvent.keyDown(input, { key: 'ArrowDown' })
      })

      // Then select with Enter
      await act(async () => {
        fireEvent.keyDown(input, { key: 'Enter' })
      })

      expect(mockOnChange).toHaveBeenCalledWith(
        expect.objectContaining({ id: 1 })
      )
    })

    test('closes dropdown on Escape key', async () => {
      render(<CustomerPicker value={null} onChange={jest.fn()} />)

      const input = screen.getByPlaceholderText('ค้นหาลูกค้า...')
      await openDropdownAndWaitForResults(input)

      expect(screen.getByRole('listbox')).toBeInTheDocument()

      await act(async () => {
        fireEvent.keyDown(input, { key: 'Escape' })
      })

      expect(screen.queryByRole('listbox')).not.toBeInTheDocument()
    })

    test('does not navigate past first item with ArrowUp', async () => {
      render(<CustomerPicker value={null} onChange={jest.fn()} />)

      const input = screen.getByPlaceholderText('ค้นหาลูกค้า...')
      await openDropdownAndWaitForResults(input)

      // Navigate to first item
      await act(async () => {
        fireEvent.keyDown(input, { key: 'ArrowDown' })
      })

      // Try to go up (should stay at first item)
      await act(async () => {
        fireEvent.keyDown(input, { key: 'ArrowUp' })
      })

      const listItems = screen.getAllByRole('option')
      expect(listItems[0]).toHaveClass('bg-blue-50')
    })

    test('does not navigate past last item with ArrowDown', async () => {
      render(<CustomerPicker value={null} onChange={jest.fn()} />)

      const input = screen.getByPlaceholderText('ค้นหาลูกค้า...')
      await openDropdownAndWaitForResults(input)

      // Navigate past the last item
      await act(async () => {
        fireEvent.keyDown(input, { key: 'ArrowDown' })
        fireEvent.keyDown(input, { key: 'ArrowDown' })
        fireEvent.keyDown(input, { key: 'ArrowDown' })
        fireEvent.keyDown(input, { key: 'ArrowDown' }) // Should not go past item 3
      })

      const listItems = screen.getAllByRole('option')
      expect(listItems[2]).toHaveClass('bg-blue-50')
    })
  })

  describe('Disabled State', () => {
    test('does not open dropdown when disabled', async () => {
      render(
        <CustomerPicker value={null} onChange={jest.fn()} disabled={true} />
      )

      const container = screen.getByPlaceholderText('ค้นหาลูกค้า...').closest('div')?.parentElement

      await act(async () => {
        fireEvent.click(container!)
      })

      await act(async () => {
        jest.advanceTimersByTime(350)
      })

      expect(screen.queryByRole('listbox')).not.toBeInTheDocument()
    })

    test('input is disabled when disabled prop is true', () => {
      render(
        <CustomerPicker value={null} onChange={jest.fn()} disabled={true} />
      )

      const input = screen.getByPlaceholderText('ค้นหาลูกค้า...')
      expect(input).toBeDisabled()
    })

    test('applies disabled styling', () => {
      render(
        <CustomerPicker value={null} onChange={jest.fn()} disabled={true} />
      )

      const container = screen.getByPlaceholderText('ค้นหาลูกค้า...').closest('div')
      expect(container).toHaveClass('bg-gray-100')
      expect(container).toHaveClass('cursor-not-allowed')
    })

    test('shows selected customer when disabled', () => {
      render(
        <CustomerPicker
          value={mockCustomers[0]}
          onChange={jest.fn()}
          disabled={true}
        />
      )

      expect(screen.getByText('สมชาย ใจดี')).toBeInTheDocument()
    })
  })

  describe('Add New Button', () => {
    test('shows "Add New" button when onAddNew is provided', async () => {
      const mockOnAddNew = jest.fn()
      render(
        <CustomerPicker
          value={null}
          onChange={jest.fn()}
          onAddNew={mockOnAddNew}
        />
      )

      const input = screen.getByPlaceholderText('ค้นหาลูกค้า...')
      await openDropdownAndWaitForResults(input)

      expect(screen.getByText('เพิ่มลูกค้าใหม่')).toBeInTheDocument()
    })

    test('does not show "Add New" button when onAddNew is not provided', async () => {
      render(<CustomerPicker value={null} onChange={jest.fn()} />)

      const input = screen.getByPlaceholderText('ค้นหาลูกค้า...')
      await openDropdownAndWaitForResults(input)

      expect(screen.queryByText('เพิ่มลูกค้าใหม่')).not.toBeInTheDocument()
    })

    test('calls onAddNew when "Add New" button is clicked', async () => {
      const mockOnAddNew = jest.fn()
      render(
        <CustomerPicker
          value={null}
          onChange={jest.fn()}
          onAddNew={mockOnAddNew}
        />
      )

      const input = screen.getByPlaceholderText('ค้นหาลูกค้า...')
      await openDropdownAndWaitForResults(input)

      await act(async () => {
        fireEvent.click(screen.getByText('เพิ่มลูกค้าใหม่'))
      })

      expect(mockOnAddNew).toHaveBeenCalledTimes(1)
    })

    test('closes dropdown when "Add New" button is clicked', async () => {
      const mockOnAddNew = jest.fn()
      render(
        <CustomerPicker
          value={null}
          onChange={jest.fn()}
          onAddNew={mockOnAddNew}
        />
      )

      const input = screen.getByPlaceholderText('ค้นหาลูกค้า...')
      await openDropdownAndWaitForResults(input)

      await act(async () => {
        fireEvent.click(screen.getByText('เพิ่มลูกค้าใหม่'))
      })

      expect(screen.queryByRole('listbox')).not.toBeInTheDocument()
    })
  })

  describe('Error Handling', () => {
    test('handles fetch error gracefully', async () => {
      mockFetch.mockRejectedValue(new Error('Network error'))
      const consoleSpy = jest.spyOn(console, 'error').mockImplementation()

      render(<CustomerPicker value={null} onChange={jest.fn()} />)

      const input = screen.getByPlaceholderText('ค้นหาลูกค้า...')
      await openDropdownAndWaitForResults(input)

      expect(consoleSpy).toHaveBeenCalledWith(
        'Error fetching customers:',
        expect.any(Error)
      )

      consoleSpy.mockRestore()
    })

    test('handles non-ok response gracefully', async () => {
      mockFetch.mockResolvedValue({
        ok: false,
        status: 500,
      })

      render(<CustomerPicker value={null} onChange={jest.fn()} />)

      const input = screen.getByPlaceholderText('ค้นหาลูกค้า...')
      await openDropdownAndWaitForResults(input)

      // Should not crash, just show empty state
      expect(screen.getByText('พิมพ์เพื่อค้นหาลูกค้า')).toBeInTheDocument()
    })
  })

  describe('Mouse Interactions', () => {
    test('highlights item on mouse enter', async () => {
      render(<CustomerPicker value={null} onChange={jest.fn()} />)

      const input = screen.getByPlaceholderText('ค้นหาลูกค้า...')
      await openDropdownAndWaitForResults(input)

      const listItems = screen.getAllByRole('option')

      await act(async () => {
        fireEvent.mouseEnter(listItems[1])
      })

      expect(listItems[1]).toHaveClass('bg-blue-50')
    })

    test('shows current selection with different style', async () => {
      render(
        <CustomerPicker value={mockCustomers[0]} onChange={jest.fn()} />
      )

      // Click to switch to search mode
      const container = screen.getByText('สมชาย ใจดี').closest('div')?.parentElement

      await act(async () => {
        fireEvent.click(container!)
      })

      await act(async () => {
        jest.advanceTimersByTime(350)
        await Promise.resolve()
      })

      const listItems = screen.getAllByRole('option')
      expect(listItems[0]).toHaveAttribute('aria-selected', 'true')
      expect(listItems[0]).toHaveClass('bg-blue-100')
    })
  })
})
