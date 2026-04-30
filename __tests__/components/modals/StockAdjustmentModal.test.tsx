/**
 * @jest-environment jsdom
 */

import { render, screen, fireEvent, waitFor, act } from '@testing-library/react'
import userEvent from '@testing-library/user-event'
import StockAdjustmentModal from '@/components/modals/StockAdjustmentModal'
import { InventoryItem } from '@/types/inventory'

// Mock lucide-react icons
jest.mock('lucide-react', () => ({
  X: () => <span data-testid="x-icon">X</span>,
  Search: () => <span data-testid="search-icon">Search</span>,
  Loader2: () => <span data-testid="loader-icon">Loader</span>,
  Package: () => <span data-testid="package-icon">Package</span>,
  Plus: () => <span data-testid="plus-icon">Plus</span>,
  Minus: () => <span data-testid="minus-icon">Minus</span>,
  RefreshCw: () => <span data-testid="refresh-icon">RefreshCw</span>,
  AlertCircle: () => <span data-testid="alert-circle-icon">AlertCircle</span>,
  FileText: () => <span data-testid="file-text-icon">FileText</span>,
}))

describe('StockAdjustmentModal Component', () => {
  const mockOnClose = jest.fn()
  const mockOnSuccess = jest.fn()

  const mockInventoryItem: InventoryItem = {
    id: 1,
    itemCode: 'MB001',
    itemName: 'น้ำดื่ม',
    category: 'Minibar',
    unit: 'ขวด',
    minStock: 50,
    currentStock: 100,
    costPerUnit: 10,
    active: true,
    createdAt: '2026-01-01T00:00:00.000Z',
    updatedAt: '2026-01-01T00:00:00.000Z',
  }

  const mockSearchResults: InventoryItem[] = [
    mockInventoryItem,
    {
      id: 2,
      itemCode: 'MB002',
      itemName: 'น้ำอัดลม',
      category: 'Minibar',
      unit: 'กระป๋อง',
      minStock: 30,
      currentStock: 25,
      costPerUnit: 15,
      active: true,
      createdAt: '2026-01-01T00:00:00.000Z',
      updatedAt: '2026-01-01T00:00:00.000Z',
    },
  ]

  beforeEach(() => {
    jest.clearAllMocks()
    jest.useFakeTimers()
    global.fetch = jest.fn()
  })

  afterEach(() => {
    jest.useRealTimers()
  })

  describe('Rendering', () => {
    test('does not render when isOpen is false', () => {
      render(<StockAdjustmentModal isOpen={false} onClose={mockOnClose} onSuccess={mockOnSuccess} />)

      expect(screen.queryByText('ปรับสต็อก')).not.toBeInTheDocument()
    })

    test('renders when isOpen is true', () => {
      render(<StockAdjustmentModal isOpen={true} onClose={mockOnClose} onSuccess={mockOnSuccess} />)

      expect(screen.getByText('ปรับสต็อก')).toBeInTheDocument()
      expect(screen.getByText('เพิ่ม, ลด หรือตั้งค่าจำนวนสต็อกใหม่')).toBeInTheDocument()
    })

    test('displays all three adjustment type buttons', () => {
      render(<StockAdjustmentModal isOpen={true} onClose={mockOnClose} onSuccess={mockOnSuccess} />)

      expect(screen.getByText('รับเข้า')).toBeInTheDocument()
      expect(screen.getByText('เบิกออก')).toBeInTheDocument()
      expect(screen.getByText('ตั้งค่าใหม่')).toBeInTheDocument()
    })

    test('displays search input', () => {
      render(<StockAdjustmentModal isOpen={true} onClose={mockOnClose} onSuccess={mockOnSuccess} />)

      expect(screen.getByPlaceholderText('ค้นหาด้วยชื่อหรือรหัสสินค้า...')).toBeInTheDocument()
    })

    test('displays quantity input', () => {
      render(<StockAdjustmentModal isOpen={true} onClose={mockOnClose} onSuccess={mockOnSuccess} />)

      expect(screen.getByText('จำนวน *')).toBeInTheDocument()
    })

    test('displays notes textarea', () => {
      render(<StockAdjustmentModal isOpen={true} onClose={mockOnClose} onSuccess={mockOnSuccess} />)

      expect(screen.getByPlaceholderText('เหตุผลในการปรับสต็อก...')).toBeInTheDocument()
    })
  })

  describe('Item Search Functionality', () => {
    test('searches items when typing in search input', async () => {
      ;(global.fetch as jest.Mock).mockResolvedValue({
        json: () => Promise.resolve({ success: true, data: mockSearchResults }),
      })

      render(<StockAdjustmentModal isOpen={true} onClose={mockOnClose} onSuccess={mockOnSuccess} />)

      const searchInput = screen.getByPlaceholderText('ค้นหาด้วยชื่อหรือรหัสสินค้า...')
      fireEvent.change(searchInput, { target: { value: 'น้ำ' } })

      // Wait for debounce
      await act(async () => {
        jest.advanceTimersByTime(300)
      })

      await waitFor(() => {
        expect(global.fetch).toHaveBeenCalledWith('/api/new/inventory/items?search=%E0%B8%99%E0%B9%89%E0%B8%B3&limit=10&branch=hfhotel', undefined)
      })
    })

    test('displays search results in dropdown', async () => {
      ;(global.fetch as jest.Mock).mockResolvedValue({
        json: () => Promise.resolve({ success: true, data: mockSearchResults }),
      })

      render(<StockAdjustmentModal isOpen={true} onClose={mockOnClose} onSuccess={mockOnSuccess} />)

      const searchInput = screen.getByPlaceholderText('ค้นหาด้วยชื่อหรือรหัสสินค้า...')
      fireEvent.change(searchInput, { target: { value: 'น้ำ' } })

      await act(async () => {
        jest.advanceTimersByTime(300)
      })

      await waitFor(() => {
        expect(screen.getByText('น้ำดื่ม')).toBeInTheDocument()
        expect(screen.getByText('น้ำอัดลม')).toBeInTheDocument()
      })
    })

    test('shows no results message when search returns empty', async () => {
      ;(global.fetch as jest.Mock).mockResolvedValue({
        json: () => Promise.resolve({ success: true, data: [] }),
      })

      render(<StockAdjustmentModal isOpen={true} onClose={mockOnClose} onSuccess={mockOnSuccess} />)

      const searchInput = screen.getByPlaceholderText('ค้นหาด้วยชื่อหรือรหัสสินค้า...')
      fireEvent.change(searchInput, { target: { value: 'xyz' } })

      await act(async () => {
        jest.advanceTimersByTime(300)
      })

      await waitFor(() => {
        expect(screen.getByText('ไม่พบสินค้า')).toBeInTheDocument()
      })
    })

    test('selects item when clicked from dropdown', async () => {
      ;(global.fetch as jest.Mock).mockResolvedValue({
        json: () => Promise.resolve({ success: true, data: mockSearchResults }),
      })

      render(<StockAdjustmentModal isOpen={true} onClose={mockOnClose} onSuccess={mockOnSuccess} />)

      const searchInput = screen.getByPlaceholderText('ค้นหาด้วยชื่อหรือรหัสสินค้า...')
      fireEvent.change(searchInput, { target: { value: 'น้ำ' } })

      await act(async () => {
        jest.advanceTimersByTime(300)
      })

      await waitFor(() => {
        expect(screen.getByText('น้ำดื่ม')).toBeInTheDocument()
      })

      // Click on the first item
      fireEvent.click(screen.getByText('น้ำดื่ม'))

      // Should show selected item info
      await waitFor(() => {
        expect(screen.getByText('คงเหลือปัจจุบัน')).toBeInTheDocument()
        expect(screen.getByText('100 ขวด')).toBeInTheDocument()
      })
    })
  })

  describe('Pre-selected Item Support', () => {
    test('pre-populates with preselectedItem', async () => {
      render(
        <StockAdjustmentModal
          isOpen={true}
          onClose={mockOnClose}
          onSuccess={mockOnSuccess}
          preselectedItem={mockInventoryItem}
        />
      )

      await waitFor(() => {
        expect(screen.getByText('คงเหลือปัจจุบัน')).toBeInTheDocument()
        expect(screen.getByText('100 ขวด')).toBeInTheDocument()
      })
    })

    test('search input shows preselected item name', () => {
      render(
        <StockAdjustmentModal
          isOpen={true}
          onClose={mockOnClose}
          onSuccess={mockOnSuccess}
          preselectedItem={mockInventoryItem}
        />
      )

      const searchInput = screen.getByPlaceholderText('ค้นหาด้วยชื่อหรือรหัสสินค้า...')
      expect(searchInput).toHaveValue('น้ำดื่ม')
    })
  })

  describe('Three Adjustment Types', () => {
    test('add type is selected by default', () => {
      render(<StockAdjustmentModal isOpen={true} onClose={mockOnClose} onSuccess={mockOnSuccess} />)

      const addButton = screen.getByText('รับเข้า').closest('button')
      expect(addButton).toHaveClass('border-green-500')
    })

    test('clicking remove changes adjustment type', () => {
      render(<StockAdjustmentModal isOpen={true} onClose={mockOnClose} onSuccess={mockOnSuccess} />)

      const removeButton = screen.getByText('เบิกออก')
      fireEvent.click(removeButton)

      expect(removeButton.closest('button')).toHaveClass('border-red-500')
    })

    test('clicking set changes adjustment type', () => {
      render(<StockAdjustmentModal isOpen={true} onClose={mockOnClose} onSuccess={mockOnSuccess} />)

      const setButton = screen.getByText('ตั้งค่าใหม่')
      fireEvent.click(setButton)

      expect(setButton.closest('button')).toHaveClass('border-blue-500')
    })
  })

  describe('New Stock Calculation Preview', () => {
    test('shows new stock for add adjustment', async () => {
      render(
        <StockAdjustmentModal
          isOpen={true}
          onClose={mockOnClose}
          onSuccess={mockOnSuccess}
          preselectedItem={mockInventoryItem}
        />
      )

      // Current stock is 100, add 20
      const quantityInput = screen.getByPlaceholderText('จำนวนที่ต้องการ')
      fireEvent.change(quantityInput, { target: { value: '20' } })

      await waitFor(() => {
        expect(screen.getByText('สต็อกใหม่:')).toBeInTheDocument()
        expect(screen.getByText(/120 ขวด/)).toBeInTheDocument()
      })
    })

    test('shows new stock for remove adjustment', async () => {
      render(
        <StockAdjustmentModal
          isOpen={true}
          onClose={mockOnClose}
          onSuccess={mockOnSuccess}
          preselectedItem={mockInventoryItem}
        />
      )

      // Click remove
      fireEvent.click(screen.getByText('เบิกออก'))

      // Current stock is 100, remove 30
      const quantityInput = screen.getByPlaceholderText('จำนวนที่ต้องการ')
      fireEvent.change(quantityInput, { target: { value: '30' } })

      await waitFor(() => {
        expect(screen.getByText(/70 ขวด/)).toBeInTheDocument()
      })
    })

    test('shows new stock for set adjustment', async () => {
      render(
        <StockAdjustmentModal
          isOpen={true}
          onClose={mockOnClose}
          onSuccess={mockOnSuccess}
          preselectedItem={mockInventoryItem}
        />
      )

      // Click set
      fireEvent.click(screen.getByText('ตั้งค่าใหม่'))

      // Set to 50
      const quantityInput = screen.getByPlaceholderText('จำนวนใหม่')
      fireEvent.change(quantityInput, { target: { value: '50' } })

      await waitFor(() => {
        expect(screen.getByText(/50 ขวด/)).toBeInTheDocument()
      })
    })

    test('remove calculation does not go below 0', async () => {
      render(
        <StockAdjustmentModal
          isOpen={true}
          onClose={mockOnClose}
          onSuccess={mockOnSuccess}
          preselectedItem={mockInventoryItem}
        />
      )

      // First verify preselected item is shown
      expect(screen.getByText('คงเหลือปัจจุบัน')).toBeInTheDocument()

      // Click remove adjustment type
      const removeButton = screen.getByText('เบิกออก')
      fireEvent.click(removeButton)

      // Wait for adjustment type to be selected (red styling)
      await waitFor(() => {
        expect(removeButton.closest('button')).toHaveClass('border-red-500')
      })

      // Try to remove more than current stock (100)
      const quantityInput = screen.getByPlaceholderText('จำนวนที่ต้องการ')
      fireEvent.change(quantityInput, { target: { value: '150' } })

      // Wait for preview to update
      await waitFor(() => {
        expect(screen.getByText('สต็อกใหม่:')).toBeInTheDocument()
      })

      // Should show 0, not negative (use exact match to avoid matching "100 ขวด")
      await waitFor(() => {
        expect(screen.getByText(/^0 ขวด$/)).toBeInTheDocument()
      })
    })
  })

  describe('Validation', () => {
    test('submit button is disabled when no item is selected even with quantity', () => {
      render(<StockAdjustmentModal isOpen={true} onClose={mockOnClose} onSuccess={mockOnSuccess} />)

      const quantityInput = screen.getByPlaceholderText('จำนวนที่ต้องการ')
      fireEvent.change(quantityInput, { target: { value: '10' } })

      const submitButton = screen.getByText('ยืนยันการปรับสต็อก')
      expect(submitButton).toBeDisabled()
    })

    test('submit button is disabled when quantity is empty even with item selected', () => {
      render(
        <StockAdjustmentModal
          isOpen={true}
          onClose={mockOnClose}
          onSuccess={mockOnSuccess}
          preselectedItem={mockInventoryItem}
        />
      )

      // Quantity is empty by default
      const submitButton = screen.getByText('ยืนยันการปรับสต็อก')
      expect(submitButton).toBeDisabled()
    })

    test('input has min=0 attribute for HTML5 validation', () => {
      // Note: HTML5 form validation (min="0") prevents negative values at browser level
      // This is not easily testable in JSDOM, so we verify the attribute is present
      render(
        <StockAdjustmentModal
          isOpen={true}
          onClose={mockOnClose}
          onSuccess={mockOnSuccess}
          preselectedItem={mockInventoryItem}
        />
      )

      const quantityInput = screen.getByPlaceholderText('จำนวนที่ต้องการ')
      expect(quantityInput).toHaveAttribute('min', '0')
      expect(quantityInput).toHaveAttribute('type', 'number')
    })

    test('shows error when removing more than available stock', async () => {
      render(
        <StockAdjustmentModal
          isOpen={true}
          onClose={mockOnClose}
          onSuccess={mockOnSuccess}
          preselectedItem={mockInventoryItem}
        />
      )

      fireEvent.click(screen.getByText('เบิกออก'))

      // Try to remove 150 when only 100 available
      const quantityInput = screen.getByPlaceholderText('จำนวนที่ต้องการ')
      fireEvent.change(quantityInput, { target: { value: '150' } })

      const submitButton = screen.getByText('ยืนยันการปรับสต็อก')
      fireEvent.click(submitButton)

      await waitFor(() => {
        expect(screen.getByText('จำนวนที่เบิกออกมากกว่าจำนวนคงเหลือ')).toBeInTheDocument()
      })
    })

    test('allows removing exact amount of available stock', async () => {
      ;(global.fetch as jest.Mock).mockResolvedValue({
        ok: true,
        json: () => Promise.resolve({ success: true }),
      })

      render(
        <StockAdjustmentModal
          isOpen={true}
          onClose={mockOnClose}
          onSuccess={mockOnSuccess}
          preselectedItem={mockInventoryItem}
        />
      )

      fireEvent.click(screen.getByText('เบิกออก'))

      // Remove exactly 100 (all available)
      const quantityInput = screen.getByPlaceholderText('จำนวนที่ต้องการ')
      fireEvent.change(quantityInput, { target: { value: '100' } })

      const submitButton = screen.getByText('ยืนยันการปรับสต็อก')
      fireEvent.click(submitButton)

      await waitFor(() => {
        expect(global.fetch).toHaveBeenCalledWith('/api/new/inventory/adjustments?branch=hfhotel', expect.any(Object))
      })
    })
  })

  describe('API Submission', () => {
    test('submits correct payload for add adjustment', async () => {
      ;(global.fetch as jest.Mock).mockResolvedValue({
        ok: true,
        json: () => Promise.resolve({ success: true }),
      })

      render(
        <StockAdjustmentModal
          isOpen={true}
          onClose={mockOnClose}
          onSuccess={mockOnSuccess}
          preselectedItem={mockInventoryItem}
        />
      )

      const quantityInput = screen.getByPlaceholderText('จำนวนที่ต้องการ')
      fireEvent.change(quantityInput, { target: { value: '25' } })

      const notesInput = screen.getByPlaceholderText('เหตุผลในการปรับสต็อก...')
      fireEvent.change(notesInput, { target: { value: 'รับของจากซัพพลายเออร์' } })

      const submitButton = screen.getByText('ยืนยันการปรับสต็อก')
      fireEvent.click(submitButton)

      await waitFor(() => {
        expect(global.fetch).toHaveBeenCalledWith('/api/new/inventory/adjustments?branch=hfhotel', {
          method: 'POST',
          headers: { 'Content-Type': 'application/json' },
          body: JSON.stringify({
            itemId: 1,
            adjustmentType: 'add',
            quantity: 25,
            notes: 'รับของจากซัพพลายเออร์',
          }),
        })
      })
    })

    test('submits correct payload for remove adjustment', async () => {
      ;(global.fetch as jest.Mock).mockResolvedValue({
        ok: true,
        json: () => Promise.resolve({ success: true }),
      })

      render(
        <StockAdjustmentModal
          isOpen={true}
          onClose={mockOnClose}
          onSuccess={mockOnSuccess}
          preselectedItem={mockInventoryItem}
        />
      )

      fireEvent.click(screen.getByText('เบิกออก'))

      const quantityInput = screen.getByPlaceholderText('จำนวนที่ต้องการ')
      fireEvent.change(quantityInput, { target: { value: '10' } })

      const submitButton = screen.getByText('ยืนยันการปรับสต็อก')
      fireEvent.click(submitButton)

      await waitFor(() => {
        expect(global.fetch).toHaveBeenCalledWith('/api/new/inventory/adjustments?branch=hfhotel', {
          method: 'POST',
          headers: { 'Content-Type': 'application/json' },
          body: JSON.stringify({
            itemId: 1,
            adjustmentType: 'remove',
            quantity: 10,
            notes: undefined,
          }),
        })
      })
    })

    test('submits correct payload for set adjustment', async () => {
      ;(global.fetch as jest.Mock).mockResolvedValue({
        ok: true,
        json: () => Promise.resolve({ success: true }),
      })

      render(
        <StockAdjustmentModal
          isOpen={true}
          onClose={mockOnClose}
          onSuccess={mockOnSuccess}
          preselectedItem={mockInventoryItem}
        />
      )

      fireEvent.click(screen.getByText('ตั้งค่าใหม่'))

      const quantityInput = screen.getByPlaceholderText('จำนวนใหม่')
      fireEvent.change(quantityInput, { target: { value: '75' } })

      const submitButton = screen.getByText('ยืนยันการปรับสต็อก')
      fireEvent.click(submitButton)

      await waitFor(() => {
        expect(global.fetch).toHaveBeenCalledWith('/api/new/inventory/adjustments?branch=hfhotel', {
          method: 'POST',
          headers: { 'Content-Type': 'application/json' },
          body: JSON.stringify({
            itemId: 1,
            adjustmentType: 'set',
            quantity: 75,
            notes: undefined,
          }),
        })
      })
    })

    test('calls onSuccess after successful submission', async () => {
      ;(global.fetch as jest.Mock).mockResolvedValue({
        ok: true,
        json: () => Promise.resolve({ success: true }),
      })

      render(
        <StockAdjustmentModal
          isOpen={true}
          onClose={mockOnClose}
          onSuccess={mockOnSuccess}
          preselectedItem={mockInventoryItem}
        />
      )

      const quantityInput = screen.getByPlaceholderText('จำนวนที่ต้องการ')
      fireEvent.change(quantityInput, { target: { value: '10' } })

      const submitButton = screen.getByText('ยืนยันการปรับสต็อก')
      fireEvent.click(submitButton)

      await waitFor(() => {
        expect(mockOnSuccess).toHaveBeenCalled()
        expect(mockOnClose).toHaveBeenCalled()
      })
    })

    test('shows error message on API failure', async () => {
      ;(global.fetch as jest.Mock).mockResolvedValue({
        ok: false,
        json: () => Promise.resolve({ success: false, error: 'Server error' }),
      })

      render(
        <StockAdjustmentModal
          isOpen={true}
          onClose={mockOnClose}
          onSuccess={mockOnSuccess}
          preselectedItem={mockInventoryItem}
        />
      )

      const quantityInput = screen.getByPlaceholderText('จำนวนที่ต้องการ')
      fireEvent.change(quantityInput, { target: { value: '10' } })

      const submitButton = screen.getByText('ยืนยันการปรับสต็อก')
      fireEvent.click(submitButton)

      await waitFor(() => {
        expect(screen.getByText('Server error')).toBeInTheDocument()
      })
    })
  })

  describe('Modal Interactions', () => {
    test('closes modal when close button is clicked', () => {
      render(<StockAdjustmentModal isOpen={true} onClose={mockOnClose} onSuccess={mockOnSuccess} />)

      const closeButton = screen.getByTestId('x-icon').closest('button')
      fireEvent.click(closeButton!)

      expect(mockOnClose).toHaveBeenCalled()
    })

    test('closes modal when cancel button is clicked', () => {
      render(<StockAdjustmentModal isOpen={true} onClose={mockOnClose} onSuccess={mockOnSuccess} />)

      const cancelButton = screen.getByText('ยกเลิก')
      fireEvent.click(cancelButton)

      expect(mockOnClose).toHaveBeenCalled()
    })

    test('closes modal when clicking backdrop', () => {
      render(<StockAdjustmentModal isOpen={true} onClose={mockOnClose} onSuccess={mockOnSuccess} />)

      // Click on the backdrop (outer div)
      const backdrop = screen.getByText('ปรับสต็อก').closest('.fixed')
      fireEvent.click(backdrop!)

      expect(mockOnClose).toHaveBeenCalled()
    })

    test('does not close modal when clicking inside content', () => {
      render(<StockAdjustmentModal isOpen={true} onClose={mockOnClose} onSuccess={mockOnSuccess} />)

      // Click on the modal content
      fireEvent.click(screen.getByText('ปรับสต็อก'))

      expect(mockOnClose).not.toHaveBeenCalled()
    })

    test('submit button is disabled when no item selected', () => {
      render(<StockAdjustmentModal isOpen={true} onClose={mockOnClose} onSuccess={mockOnSuccess} />)

      const submitButton = screen.getByText('ยืนยันการปรับสต็อก')
      expect(submitButton).toBeDisabled()
    })

    test('submit button is disabled when quantity is empty', () => {
      render(
        <StockAdjustmentModal
          isOpen={true}
          onClose={mockOnClose}
          onSuccess={mockOnSuccess}
          preselectedItem={mockInventoryItem}
        />
      )

      const submitButton = screen.getByText('ยืนยันการปรับสต็อก')
      expect(submitButton).toBeDisabled()
    })

    test('submit button is enabled when item and quantity are provided', () => {
      render(
        <StockAdjustmentModal
          isOpen={true}
          onClose={mockOnClose}
          onSuccess={mockOnSuccess}
          preselectedItem={mockInventoryItem}
        />
      )

      const quantityInput = screen.getByPlaceholderText('จำนวนที่ต้องการ')
      fireEvent.change(quantityInput, { target: { value: '10' } })

      const submitButton = screen.getByText('ยืนยันการปรับสต็อก')
      expect(submitButton).not.toBeDisabled()
    })
  })
})
