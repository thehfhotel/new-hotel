/**
 * @jest-environment jsdom
 */

import { render, screen, fireEvent, waitFor, act } from '@testing-library/react'
import userEvent from '@testing-library/user-event'
import RoomInventoryChecklist from '@/components/inventory/RoomInventoryChecklist'
import { RoomInventory, RoomInventoryItem } from '@/types/inventory'

// Mock lucide-react icons
jest.mock('lucide-react', () => ({
  X: () => <span data-testid="x-icon">X</span>,
  Check: () => <span data-testid="check-icon">Check</span>,
  AlertTriangle: () => <span data-testid="alert-triangle-icon">AlertTriangle</span>,
  Package: () => <span data-testid="package-icon">Package</span>,
  Loader2: () => <span data-testid="loader-icon">Loader</span>,
  RefreshCw: () => <span data-testid="refresh-icon">RefreshCw</span>,
  FileText: () => <span data-testid="file-text-icon">FileText</span>,
  CheckCircle: () => <span data-testid="check-circle-icon">CheckCircle</span>,
  XCircle: () => <span data-testid="x-circle-icon">XCircle</span>,
}))

describe('RoomInventoryChecklist Component', () => {
  const mockOnClose = jest.fn()
  const mockOnSuccess = jest.fn()

  const mockRoomInventoryItems: RoomInventoryItem[] = [
    {
      itemId: 1,
      itemCode: 'MB001',
      itemName: 'น้ำดื่ม',
      category: 'Minibar',
      unit: 'ขวด',
      assignedQuantity: 4,
      verifiedQuantity: 4,
      lastVerified: '2026-01-19T10:00:00.000Z',
    },
    {
      itemId: 2,
      itemCode: 'MB002',
      itemName: 'ขนมปัง',
      category: 'Minibar',
      unit: 'ชิ้น',
      assignedQuantity: 2,
      verifiedQuantity: 2,
      lastVerified: '2026-01-19T10:00:00.000Z',
    },
    {
      itemId: 3,
      itemCode: 'AM001',
      itemName: 'สบู่',
      category: 'Amenities',
      unit: 'ก้อน',
      assignedQuantity: 3,
      verifiedQuantity: 3,
      lastVerified: '2026-01-19T10:00:00.000Z',
    },
    {
      itemId: 4,
      itemCode: 'LN001',
      itemName: 'ผ้าขนหนู',
      category: 'Linens',
      unit: 'ผืน',
      assignedQuantity: 2,
      verifiedQuantity: 2,
      lastVerified: '2026-01-19T10:00:00.000Z',
    },
  ]

  const mockRoomInventory: RoomInventory = {
    roomId: 301,
    roomNumber: '301',
    roomType: 'Standard',
    items: mockRoomInventoryItems,
  }

  beforeEach(() => {
    jest.clearAllMocks()
    global.fetch = jest.fn()
  })

  describe('Rendering', () => {
    test('does not render when isOpen is false', () => {
      render(
        <RoomInventoryChecklist
          isOpen={false}
          onClose={mockOnClose}
          roomId={301}
          roomNumber="301"
          onSuccess={mockOnSuccess}
        />
      )

      expect(screen.queryByText('ตรวจสอบสินค้าในห้อง')).not.toBeInTheDocument()
    })

    test('renders when isOpen is true', async () => {
      ;(global.fetch as jest.Mock).mockResolvedValue({
        json: () => Promise.resolve({ success: true, data: mockRoomInventory }),
      })

      render(
        <RoomInventoryChecklist
          isOpen={true}
          onClose={mockOnClose}
          roomId={301}
          roomNumber="301"
          onSuccess={mockOnSuccess}
        />
      )

      expect(screen.getByText('ตรวจสอบสินค้าในห้อง')).toBeInTheDocument()
      expect(screen.getByText('ห้อง 301')).toBeInTheDocument()
    })

    test('shows loading state initially', () => {
      ;(global.fetch as jest.Mock).mockImplementation(
        () => new Promise(() => {}) // Never resolves
      )

      render(
        <RoomInventoryChecklist
          isOpen={true}
          onClose={mockOnClose}
          roomId={301}
          roomNumber="301"
          onSuccess={mockOnSuccess}
        />
      )

      expect(screen.getByText('กำลังโหลด...')).toBeInTheDocument()
    })
  })

  describe('Displays Room Inventory Items', () => {
    test('displays all inventory items after loading', async () => {
      ;(global.fetch as jest.Mock).mockResolvedValue({
        json: () => Promise.resolve({ success: true, data: mockRoomInventory }),
      })

      render(
        <RoomInventoryChecklist
          isOpen={true}
          onClose={mockOnClose}
          roomId={301}
          roomNumber="301"
          onSuccess={mockOnSuccess}
        />
      )

      await waitFor(() => {
        expect(screen.getByText('น้ำดื่ม')).toBeInTheDocument()
        expect(screen.getByText('ขนมปัง')).toBeInTheDocument()
        expect(screen.getByText('สบู่')).toBeInTheDocument()
        expect(screen.getByText('ผ้าขนหนู')).toBeInTheDocument()
      })
    })

    test('displays item codes', async () => {
      ;(global.fetch as jest.Mock).mockResolvedValue({
        json: () => Promise.resolve({ success: true, data: mockRoomInventory }),
      })

      render(
        <RoomInventoryChecklist
          isOpen={true}
          onClose={mockOnClose}
          roomId={301}
          roomNumber="301"
          onSuccess={mockOnSuccess}
        />
      )

      await waitFor(() => {
        expect(screen.getByText(/MB001/)).toBeInTheDocument()
        expect(screen.getByText(/AM001/)).toBeInTheDocument()
        expect(screen.getByText(/LN001/)).toBeInTheDocument()
      })
    })

    test('displays total items count', async () => {
      ;(global.fetch as jest.Mock).mockResolvedValue({
        json: () => Promise.resolve({ success: true, data: mockRoomInventory }),
      })

      render(
        <RoomInventoryChecklist
          isOpen={true}
          onClose={mockOnClose}
          roomId={301}
          roomNumber="301"
          onSuccess={mockOnSuccess}
        />
      )

      await waitFor(() => {
        expect(screen.getByText(/สินค้าทั้งหมด: 4 รายการ/)).toBeInTheDocument()
      })
    })
  })

  describe('Checkbox for Each Item', () => {
    test('each item has a checkbox', async () => {
      ;(global.fetch as jest.Mock).mockResolvedValue({
        json: () => Promise.resolve({ success: true, data: mockRoomInventory }),
      })

      const { container } = render(
        <RoomInventoryChecklist
          isOpen={true}
          onClose={mockOnClose}
          roomId={301}
          roomNumber="301"
          onSuccess={mockOnSuccess}
        />
      )

      await waitFor(() => {
        // Find checkbox buttons (the ones with border-2)
        const checkboxes = container.querySelectorAll('button.w-6.h-6.rounded.border-2')
        expect(checkboxes.length).toBe(4)
      })
    })

    test('checkboxes are initially checked (verified)', async () => {
      ;(global.fetch as jest.Mock).mockResolvedValue({
        json: () => Promise.resolve({ success: true, data: mockRoomInventory }),
      })

      const { container } = render(
        <RoomInventoryChecklist
          isOpen={true}
          onClose={mockOnClose}
          roomId={301}
          roomNumber="301"
          onSuccess={mockOnSuccess}
        />
      )

      await waitFor(() => {
        // Checked checkboxes have green background and border
        const checkedBoxes = container.querySelectorAll('button.bg-green-500')
        expect(checkedBoxes.length).toBeGreaterThanOrEqual(4)
      })
    })

    test('unchecking checkbox marks item as not verified', async () => {
      ;(global.fetch as jest.Mock).mockResolvedValue({
        json: () => Promise.resolve({ success: true, data: mockRoomInventory }),
      })

      const { container } = render(
        <RoomInventoryChecklist
          isOpen={true}
          onClose={mockOnClose}
          roomId={301}
          roomNumber="301"
          onSuccess={mockOnSuccess}
        />
      )

      await waitFor(() => {
        expect(screen.getByText('น้ำดื่ม')).toBeInTheDocument()
      })

      // Find and click the first checkbox
      const checkbox = container.querySelector('button.w-6.h-6.rounded.border-2')
      fireEvent.click(checkbox!)

      await waitFor(() => {
        // Should now show unchecked state (gray border, no green bg)
        expect(checkbox).toHaveClass('border-gray-300')
        expect(checkbox).not.toHaveClass('bg-green-500')
      })
    })
  })

  describe('Quantity Inputs', () => {
    test('each item has a quantity input', async () => {
      ;(global.fetch as jest.Mock).mockResolvedValue({
        json: () => Promise.resolve({ success: true, data: mockRoomInventory }),
      })

      render(
        <RoomInventoryChecklist
          isOpen={true}
          onClose={mockOnClose}
          roomId={301}
          roomNumber="301"
          onSuccess={mockOnSuccess}
        />
      )

      await waitFor(() => {
        const inputs = screen.getAllByRole('spinbutton')
        expect(inputs.length).toBe(4)
      })
    })

    test('quantity inputs show assigned quantities initially', async () => {
      ;(global.fetch as jest.Mock).mockResolvedValue({
        json: () => Promise.resolve({ success: true, data: mockRoomInventory }),
      })

      render(
        <RoomInventoryChecklist
          isOpen={true}
          onClose={mockOnClose}
          roomId={301}
          roomNumber="301"
          onSuccess={mockOnSuccess}
        />
      )

      await waitFor(() => {
        const inputs = screen.getAllByRole('spinbutton')
        expect(inputs[0]).toHaveValue(4) // น้ำดื่ม
        expect(inputs[1]).toHaveValue(2) // ขนมปัง
        expect(inputs[2]).toHaveValue(3) // สบู่
        expect(inputs[3]).toHaveValue(2) // ผ้าขนหนู
      })
    })

    test('changing quantity updates verified status', async () => {
      ;(global.fetch as jest.Mock).mockResolvedValue({
        json: () => Promise.resolve({ success: true, data: mockRoomInventory }),
      })

      const { container } = render(
        <RoomInventoryChecklist
          isOpen={true}
          onClose={mockOnClose}
          roomId={301}
          roomNumber="301"
          onSuccess={mockOnSuccess}
        />
      )

      await waitFor(() => {
        expect(screen.getByText('น้ำดื่ม')).toBeInTheDocument()
      })

      // Change quantity to less than assigned
      const inputs = screen.getAllByRole('spinbutton')
      fireEvent.change(inputs[0], { target: { value: '2' } })

      await waitFor(() => {
        // Item should now show as missing (amber styling)
        const firstItemContainer = screen.getByText('น้ำดื่ม').closest('div[class*="rounded-lg border"]')
        expect(firstItemContainer).toHaveClass('border-amber-500')
      })
    })
  })

  describe('Items Grouped by Category', () => {
    test('displays category headers', async () => {
      ;(global.fetch as jest.Mock).mockResolvedValue({
        json: () => Promise.resolve({ success: true, data: mockRoomInventory }),
      })

      render(
        <RoomInventoryChecklist
          isOpen={true}
          onClose={mockOnClose}
          roomId={301}
          roomNumber="301"
          onSuccess={mockOnSuccess}
        />
      )

      await waitFor(() => {
        expect(screen.getByText('เครื่องดื่ม/ของว่าง')).toBeInTheDocument() // Minibar
        expect(screen.getByText('อุปกรณ์อำนวยความสะดวก')).toBeInTheDocument() // Amenities
        expect(screen.getByText('ผ้าและเครื่องนอน')).toBeInTheDocument() // Linens
      })
    })

    test('displays category item count', async () => {
      ;(global.fetch as jest.Mock).mockResolvedValue({
        json: () => Promise.resolve({ success: true, data: mockRoomInventory }),
      })

      render(
        <RoomInventoryChecklist
          isOpen={true}
          onClose={mockOnClose}
          roomId={301}
          roomNumber="301"
          onSuccess={mockOnSuccess}
        />
      )

      await waitFor(() => {
        // Minibar has 2 items
        expect(screen.getByText('(2)')).toBeInTheDocument()
        // Amenities has 1 item, Linens has 1 item
        const oneItemCounts = screen.getAllByText('(1)')
        expect(oneItemCounts.length).toBe(2)
      })
    })
  })

  describe('Missing Items Highlighted', () => {
    test('missing items have amber highlighting', async () => {
      ;(global.fetch as jest.Mock).mockResolvedValue({
        json: () => Promise.resolve({ success: true, data: mockRoomInventory }),
      })

      render(
        <RoomInventoryChecklist
          isOpen={true}
          onClose={mockOnClose}
          roomId={301}
          roomNumber="301"
          onSuccess={mockOnSuccess}
        />
      )

      await waitFor(() => {
        expect(screen.getByText('น้ำดื่ม')).toBeInTheDocument()
      })

      // Set quantity less than assigned
      const inputs = screen.getAllByRole('spinbutton')
      fireEvent.change(inputs[0], { target: { value: '1' } })

      await waitFor(() => {
        // Check for amber/missing styling
        const itemRow = screen.getByText('น้ำดื่ม').closest('.rounded-lg')
        expect(itemRow).toHaveClass('bg-amber-500/10')
        expect(itemRow).toHaveClass('border-amber-500')
      })
    })

    test('displays missing items count in summary', async () => {
      ;(global.fetch as jest.Mock).mockResolvedValue({
        json: () => Promise.resolve({ success: true, data: mockRoomInventory }),
      })

      render(
        <RoomInventoryChecklist
          isOpen={true}
          onClose={mockOnClose}
          roomId={301}
          roomNumber="301"
          onSuccess={mockOnSuccess}
        />
      )

      await waitFor(() => {
        expect(screen.getByText('น้ำดื่ม')).toBeInTheDocument()
      })

      // Uncheck first item
      const { container } = render(
        <RoomInventoryChecklist
          isOpen={true}
          onClose={mockOnClose}
          roomId={301}
          roomNumber="301"
          onSuccess={mockOnSuccess}
        />
      )

      // Make an item missing by changing quantity
      const inputs = screen.getAllByRole('spinbutton')
      fireEvent.change(inputs[0], { target: { value: '0' } })

      await waitFor(() => {
        expect(screen.getByText(/ขาด 1 รายการ/)).toBeInTheDocument()
      })
    })

    test('verified items have green styling', async () => {
      ;(global.fetch as jest.Mock).mockResolvedValue({
        json: () => Promise.resolve({ success: true, data: mockRoomInventory }),
      })

      render(
        <RoomInventoryChecklist
          isOpen={true}
          onClose={mockOnClose}
          roomId={301}
          roomNumber="301"
          onSuccess={mockOnSuccess}
        />
      )

      await waitFor(() => {
        // All items are verified initially, should have green styling
        const itemRows = screen
          .getAllByText(/MB00|AM00|LN00/)
          .map((el) => el.closest('.rounded-lg'))
        itemRows.forEach((row) => {
          expect(row).toHaveClass('bg-green-500/10')
          expect(row).toHaveClass('border-green-500')
        })
      })
    })
  })

  describe('Replenish Button Functionality', () => {
    test('replenish button appears when items are missing', async () => {
      ;(global.fetch as jest.Mock).mockResolvedValue({
        json: () => Promise.resolve({ success: true, data: mockRoomInventory }),
      })

      render(
        <RoomInventoryChecklist
          isOpen={true}
          onClose={mockOnClose}
          roomId={301}
          roomNumber="301"
          onSuccess={mockOnSuccess}
        />
      )

      await waitFor(() => {
        expect(screen.getByText('น้ำดื่ม')).toBeInTheDocument()
      })

      // Make an item missing
      const inputs = screen.getAllByRole('spinbutton')
      fireEvent.change(inputs[0], { target: { value: '1' } })

      await waitFor(() => {
        expect(screen.getByText('เติมสินค้าที่ขาด')).toBeInTheDocument()
      })
    })

    test('replenish button is not visible when all items are verified', async () => {
      ;(global.fetch as jest.Mock).mockResolvedValue({
        json: () => Promise.resolve({ success: true, data: mockRoomInventory }),
      })

      render(
        <RoomInventoryChecklist
          isOpen={true}
          onClose={mockOnClose}
          roomId={301}
          roomNumber="301"
          onSuccess={mockOnSuccess}
        />
      )

      await waitFor(() => {
        expect(screen.getByText('น้ำดื่ม')).toBeInTheDocument()
      })

      // All items are verified by default
      expect(screen.queryByText('เติมสินค้าที่ขาด')).not.toBeInTheDocument()
    })

    test('clicking replenish sends correct API payload', async () => {
      ;(global.fetch as jest.Mock)
        .mockResolvedValueOnce({
          json: () => Promise.resolve({ success: true, data: mockRoomInventory }),
        })
        .mockResolvedValueOnce({
          ok: true,
          json: () => Promise.resolve({ success: true }),
        })

      render(
        <RoomInventoryChecklist
          isOpen={true}
          onClose={mockOnClose}
          roomId={301}
          roomNumber="301"
          onSuccess={mockOnSuccess}
        />
      )

      await waitFor(() => {
        expect(screen.getByText('น้ำดื่ม')).toBeInTheDocument()
      })

      // Make item 1 missing (assigned 4, actual 1 = need 3)
      const inputs = screen.getAllByRole('spinbutton')
      fireEvent.change(inputs[0], { target: { value: '1' } })

      await waitFor(() => {
        expect(screen.getByText('เติมสินค้าที่ขาด')).toBeInTheDocument()
      })

      fireEvent.click(screen.getByText('เติมสินค้าที่ขาด'))

      await waitFor(() => {
        expect(global.fetch).toHaveBeenCalledWith('/api/new/inventory/rooms/301/replenish', {
          method: 'POST',
          headers: { 'Content-Type': 'application/json' },
          body: JSON.stringify({
            items: [{ itemId: 1, quantity: 3 }],
            notes: 'เติมสินค้าหลังตรวจสอบ',
          }),
        })
      })
    })

    test('shows error when replenish fails', async () => {
      ;(global.fetch as jest.Mock)
        .mockResolvedValueOnce({
          json: () => Promise.resolve({ success: true, data: mockRoomInventory }),
        })
        .mockResolvedValueOnce({
          ok: false,
          json: () => Promise.resolve({ success: false, error: 'สต็อกไม่เพียงพอ' }),
        })

      render(
        <RoomInventoryChecklist
          isOpen={true}
          onClose={mockOnClose}
          roomId={301}
          roomNumber="301"
          onSuccess={mockOnSuccess}
        />
      )

      await waitFor(() => {
        expect(screen.getByText('น้ำดื่ม')).toBeInTheDocument()
      })

      const inputs = screen.getAllByRole('spinbutton')
      fireEvent.change(inputs[0], { target: { value: '1' } })

      await waitFor(() => {
        expect(screen.getByText('เติมสินค้าที่ขาด')).toBeInTheDocument()
      })

      fireEvent.click(screen.getByText('เติมสินค้าที่ขาด'))

      await waitFor(() => {
        expect(screen.getByText('สต็อกไม่เพียงพอ')).toBeInTheDocument()
      })
    })
  })

  describe('Notes Field', () => {
    test('displays notes textarea', async () => {
      ;(global.fetch as jest.Mock).mockResolvedValue({
        json: () => Promise.resolve({ success: true, data: mockRoomInventory }),
      })

      render(
        <RoomInventoryChecklist
          isOpen={true}
          onClose={mockOnClose}
          roomId={301}
          roomNumber="301"
          onSuccess={mockOnSuccess}
        />
      )

      await waitFor(() => {
        expect(screen.getByPlaceholderText('หมายเหตุเพิ่มเติม...')).toBeInTheDocument()
      })
    })

    test('notes are included in check submission', async () => {
      ;(global.fetch as jest.Mock)
        .mockResolvedValueOnce({
          json: () => Promise.resolve({ success: true, data: mockRoomInventory }),
        })
        .mockResolvedValueOnce({
          ok: true,
          json: () => Promise.resolve({ success: true }),
        })

      render(
        <RoomInventoryChecklist
          isOpen={true}
          onClose={mockOnClose}
          roomId={301}
          roomNumber="301"
          onSuccess={mockOnSuccess}
        />
      )

      await waitFor(() => {
        expect(screen.getByText('น้ำดื่ม')).toBeInTheDocument()
      })

      const notesInput = screen.getByPlaceholderText('หมายเหตุเพิ่มเติม...')
      fireEvent.change(notesInput, { target: { value: 'ตรวจเรียบร้อย' } })

      fireEvent.click(screen.getByText('บันทึกการตรวจสอบ'))

      await waitFor(() => {
        expect(global.fetch).toHaveBeenCalledWith('/api/new/inventory/rooms/301/check', {
          method: 'POST',
          headers: { 'Content-Type': 'application/json' },
          body: expect.stringContaining('"notes":"ตรวจเรียบร้อย"'),
        })
      })
    })
  })

  describe('Submit Check', () => {
    test('submits all items verification status', async () => {
      ;(global.fetch as jest.Mock)
        .mockResolvedValueOnce({
          json: () => Promise.resolve({ success: true, data: mockRoomInventory }),
        })
        .mockResolvedValueOnce({
          ok: true,
          json: () => Promise.resolve({ success: true }),
        })

      render(
        <RoomInventoryChecklist
          isOpen={true}
          onClose={mockOnClose}
          roomId={301}
          roomNumber="301"
          onSuccess={mockOnSuccess}
        />
      )

      await waitFor(() => {
        expect(screen.getByText('น้ำดื่ม')).toBeInTheDocument()
      })

      fireEvent.click(screen.getByText('บันทึกการตรวจสอบ'))

      await waitFor(() => {
        expect(global.fetch).toHaveBeenCalledWith('/api/new/inventory/rooms/301/check', {
          method: 'POST',
          headers: { 'Content-Type': 'application/json' },
          body: JSON.stringify({
            items: [
              { itemId: 1, verified: true, actualQuantity: 4 },
              { itemId: 2, verified: true, actualQuantity: 2 },
              { itemId: 3, verified: true, actualQuantity: 3 },
              { itemId: 4, verified: true, actualQuantity: 2 },
            ],
            notes: undefined,
          }),
        })
      })
    })

    test('calls onSuccess and onClose after successful submission', async () => {
      ;(global.fetch as jest.Mock)
        .mockResolvedValueOnce({
          json: () => Promise.resolve({ success: true, data: mockRoomInventory }),
        })
        .mockResolvedValueOnce({
          ok: true,
          json: () => Promise.resolve({ success: true }),
        })

      render(
        <RoomInventoryChecklist
          isOpen={true}
          onClose={mockOnClose}
          roomId={301}
          roomNumber="301"
          onSuccess={mockOnSuccess}
        />
      )

      await waitFor(() => {
        expect(screen.getByText('น้ำดื่ม')).toBeInTheDocument()
      })

      fireEvent.click(screen.getByText('บันทึกการตรวจสอบ'))

      await waitFor(() => {
        expect(mockOnSuccess).toHaveBeenCalled()
        expect(mockOnClose).toHaveBeenCalled()
      })
    })
  })

  describe('Empty State', () => {
    test('shows empty message when no items assigned', async () => {
      const emptyRoomInventory: RoomInventory = {
        roomId: 301,
        roomNumber: '301',
        roomType: 'Standard',
        items: [],
      }

      ;(global.fetch as jest.Mock).mockResolvedValue({
        json: () => Promise.resolve({ success: true, data: emptyRoomInventory }),
      })

      render(
        <RoomInventoryChecklist
          isOpen={true}
          onClose={mockOnClose}
          roomId={301}
          roomNumber="301"
          onSuccess={mockOnSuccess}
        />
      )

      await waitFor(() => {
        expect(screen.getByText('ไม่มีสินค้าที่กำหนดให้ห้องนี้')).toBeInTheDocument()
      })
    })

    test('does not show footer buttons when no items', async () => {
      const emptyRoomInventory: RoomInventory = {
        roomId: 301,
        roomNumber: '301',
        roomType: 'Standard',
        items: [],
      }

      ;(global.fetch as jest.Mock).mockResolvedValue({
        json: () => Promise.resolve({ success: true, data: emptyRoomInventory }),
      })

      render(
        <RoomInventoryChecklist
          isOpen={true}
          onClose={mockOnClose}
          roomId={301}
          roomNumber="301"
          onSuccess={mockOnSuccess}
        />
      )

      await waitFor(() => {
        expect(screen.getByText('ไม่มีสินค้าที่กำหนดให้ห้องนี้')).toBeInTheDocument()
      })

      expect(screen.queryByText('บันทึกการตรวจสอบ')).not.toBeInTheDocument()
    })
  })

  describe('Error Handling', () => {
    test('shows error message on fetch failure', async () => {
      ;(global.fetch as jest.Mock).mockRejectedValue(new Error('Network error'))

      render(
        <RoomInventoryChecklist
          isOpen={true}
          onClose={mockOnClose}
          roomId={301}
          roomNumber="301"
          onSuccess={mockOnSuccess}
        />
      )

      await waitFor(() => {
        expect(screen.getByText('ไม่สามารถโหลดข้อมูลสินค้าคงคลังของห้องได้')).toBeInTheDocument()
      })
    })

    test('shows retry button on error', async () => {
      ;(global.fetch as jest.Mock).mockRejectedValue(new Error('Network error'))

      render(
        <RoomInventoryChecklist
          isOpen={true}
          onClose={mockOnClose}
          roomId={301}
          roomNumber="301"
          onSuccess={mockOnSuccess}
        />
      )

      await waitFor(() => {
        expect(screen.getByText('ลองใหม่')).toBeInTheDocument()
      })
    })

    test('retry button refetches data', async () => {
      ;(global.fetch as jest.Mock)
        .mockRejectedValueOnce(new Error('Network error'))
        .mockResolvedValueOnce({
          json: () => Promise.resolve({ success: true, data: mockRoomInventory }),
        })

      render(
        <RoomInventoryChecklist
          isOpen={true}
          onClose={mockOnClose}
          roomId={301}
          roomNumber="301"
          onSuccess={mockOnSuccess}
        />
      )

      await waitFor(() => {
        expect(screen.getByText('ลองใหม่')).toBeInTheDocument()
      })

      fireEvent.click(screen.getByText('ลองใหม่'))

      await waitFor(() => {
        expect(screen.getByText('น้ำดื่ม')).toBeInTheDocument()
      })
    })
  })

  describe('Modal Interactions', () => {
    test('closes when X button is clicked', async () => {
      ;(global.fetch as jest.Mock).mockResolvedValue({
        json: () => Promise.resolve({ success: true, data: mockRoomInventory }),
      })

      render(
        <RoomInventoryChecklist
          isOpen={true}
          onClose={mockOnClose}
          roomId={301}
          roomNumber="301"
          onSuccess={mockOnSuccess}
        />
      )

      await waitFor(() => {
        expect(screen.getByText('น้ำดื่ม')).toBeInTheDocument()
      })

      const closeButton = screen.getByTestId('x-icon').closest('button')
      fireEvent.click(closeButton!)

      expect(mockOnClose).toHaveBeenCalled()
    })

    test('closes when cancel button is clicked', async () => {
      ;(global.fetch as jest.Mock).mockResolvedValue({
        json: () => Promise.resolve({ success: true, data: mockRoomInventory }),
      })

      render(
        <RoomInventoryChecklist
          isOpen={true}
          onClose={mockOnClose}
          roomId={301}
          roomNumber="301"
          onSuccess={mockOnSuccess}
        />
      )

      await waitFor(() => {
        expect(screen.getByText('น้ำดื่ม')).toBeInTheDocument()
      })

      fireEvent.click(screen.getByText('ยกเลิก'))

      expect(mockOnClose).toHaveBeenCalled()
    })

    test('closes when clicking backdrop', async () => {
      ;(global.fetch as jest.Mock).mockResolvedValue({
        json: () => Promise.resolve({ success: true, data: mockRoomInventory }),
      })

      render(
        <RoomInventoryChecklist
          isOpen={true}
          onClose={mockOnClose}
          roomId={301}
          roomNumber="301"
          onSuccess={mockOnSuccess}
        />
      )

      await waitFor(() => {
        expect(screen.getByText('น้ำดื่ม')).toBeInTheDocument()
      })

      const backdrop = screen.getByText('ตรวจสอบสินค้าในห้อง').closest('.fixed')
      fireEvent.click(backdrop!)

      expect(mockOnClose).toHaveBeenCalled()
    })

    test('fetches data when modal opens', async () => {
      ;(global.fetch as jest.Mock).mockResolvedValue({
        json: () => Promise.resolve({ success: true, data: mockRoomInventory }),
      })

      render(
        <RoomInventoryChecklist
          isOpen={true}
          onClose={mockOnClose}
          roomId={301}
          roomNumber="301"
          onSuccess={mockOnSuccess}
        />
      )

      await waitFor(() => {
        expect(global.fetch).toHaveBeenCalledWith('/api/new/inventory/rooms/301')
      })
    })
  })
})
