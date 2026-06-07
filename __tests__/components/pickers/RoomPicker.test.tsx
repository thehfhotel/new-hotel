/**
 * @jest-environment jsdom
 */

import { render, screen, fireEvent, waitFor } from '@testing-library/react'
import RoomPicker, { RoomOption } from '@/components/pickers/RoomPicker'

// Mock lucide-react icons
jest.mock('lucide-react', () => ({
  Bed: () => <span data-testid="bed-icon">Bed</span>,
  Check: () => <span data-testid="check-icon">Check</span>,
  Loader2: () => <span data-testid="loader-icon">Loader</span>,
  AlertCircle: () => <span data-testid="alert-icon">Alert</span>,
  Filter: () => <span data-testid="filter-icon">Filter</span>,
}))

// Mock fetch
const mockFetch = jest.fn()
global.fetch = mockFetch

const mockRooms: RoomOption[] = [
  {
    id: 1,
    roomNo: '101',
    roomTypeId: 1,
    roomTypeName: 'Standard',
    floor: 1,
    status: 'available',
    priceWeekday: 1000,
    priceWeekend: 1200,
  },
  {
    id: 2,
    roomNo: '102',
    roomTypeId: 1,
    roomTypeName: 'Standard',
    floor: 1,
    status: 'occupied',
    priceWeekday: 1000,
    priceWeekend: 1200,
  },
  {
    id: 3,
    roomNo: '201',
    roomTypeId: 2,
    roomTypeName: 'Deluxe',
    floor: 2,
    status: 'available',
    priceWeekday: 1500,
    priceWeekend: 1800,
  },
  {
    id: 4,
    roomNo: '202',
    roomTypeId: 2,
    roomTypeName: 'Deluxe',
    floor: 2,
    status: 'maintenance',
    priceWeekday: 1500,
    priceWeekend: 1800,
  },
  {
    id: 5,
    roomNo: '301',
    roomTypeId: 3,
    roomTypeName: 'Suite',
    floor: 3,
    status: 'available',
    priceWeekday: 3000,
    priceWeekend: 3500,
  },
  {
    id: 6,
    roomNo: '302',
    roomTypeId: 3,
    roomTypeName: 'Suite',
    floor: 3,
    status: 'cleaning',
    priceWeekday: 3000,
    priceWeekend: 3500,
  },
  {
    id: 7,
    roomNo: 'A01',
    roomTypeId: null,
    roomTypeName: null,
    floor: null,
    status: 'available',
    priceWeekday: null,
    priceWeekend: null,
  },
]

const mockApiResponse = {
  success: true,
  data: mockRooms,
}

// Helper function to wait for data to load
async function waitForRoomsToLoad() {
  await waitFor(() => {
    expect(screen.queryByText('กำลังโหลดห้องพัก...')).not.toBeInTheDocument()
  })
}

describe('RoomPicker Component', () => {
  beforeEach(() => {
    mockFetch.mockClear()
    mockFetch.mockResolvedValue({
      ok: true,
      json: () => Promise.resolve(mockApiResponse),
    })
  })

  describe('Loading State', () => {
    test('shows loading indicator while fetching rooms', () => {
      // Make fetch never resolve
      mockFetch.mockImplementation(() => new Promise(() => {}))

      render(<RoomPicker value={[]} onChange={jest.fn()} />)

      expect(screen.getByText('กำลังโหลดห้องพัก...')).toBeInTheDocument()
      expect(screen.getByTestId('loader-icon')).toBeInTheDocument()
    })

    test('hides loading indicator after data is loaded', async () => {
      render(<RoomPicker value={[]} onChange={jest.fn()} />)

      await waitForRoomsToLoad()
    })
  })

  describe('Error State', () => {
    test('shows error message when fetch fails', async () => {
      mockFetch.mockRejectedValue(new Error('Network error'))

      render(<RoomPicker value={[]} onChange={jest.fn()} />)

      await waitFor(() => {
        expect(screen.getByText('Network error')).toBeInTheDocument()
        expect(screen.getByTestId('alert-icon')).toBeInTheDocument()
      })
    })

    test('shows error message for non-ok response', async () => {
      mockFetch.mockResolvedValue({
        ok: false,
        status: 500,
      })

      render(<RoomPicker value={[]} onChange={jest.fn()} />)

      await waitFor(() => {
        expect(screen.getByText('ไม่สามารถดึงข้อมูลห้องพักได้')).toBeInTheDocument()
      })
    })
  })

  describe('Rendering', () => {
    test('renders rooms grouped by floor', async () => {
      render(<RoomPicker value={[]} onChange={jest.fn()} />)

      await waitForRoomsToLoad()

      expect(screen.getByText('ชั้น 1')).toBeInTheDocument()
      expect(screen.getByText('ชั้น 2')).toBeInTheDocument()
      expect(screen.getByText('ชั้น 3')).toBeInTheDocument()
    })

    test('renders rooms with no floor as "ไม่ระบุชั้น"', async () => {
      render(<RoomPicker value={[]} onChange={jest.fn()} />)

      await waitForRoomsToLoad()

      expect(screen.getByText('ไม่ระบุชั้น')).toBeInTheDocument()
    })

    test('renders room numbers', async () => {
      render(<RoomPicker value={[]} onChange={jest.fn()} />)

      await waitForRoomsToLoad()

      expect(screen.getByText('101')).toBeInTheDocument()
      expect(screen.getByText('102')).toBeInTheDocument()
      expect(screen.getByText('201')).toBeInTheDocument()
      expect(screen.getByText('301')).toBeInTheDocument()
    })

    test('renders room type names in room cards', async () => {
      render(<RoomPicker value={[]} onChange={jest.fn()} />)

      await waitForRoomsToLoad()

      // Room type names appear in room cards
      const standardElements = screen.getAllByText('Standard')
      const deluxeElements = screen.getAllByText('Deluxe')
      const suiteElements = screen.getAllByText('Suite')

      expect(standardElements.length).toBeGreaterThan(0)
      expect(deluxeElements.length).toBeGreaterThan(0)
      expect(suiteElements.length).toBeGreaterThan(0)
    })

    test('renders room prices', async () => {
      render(<RoomPicker value={[]} onChange={jest.fn()} />)

      await waitForRoomsToLoad()

      // Multiple rooms can have same price, so use getAllByText
      expect(screen.getAllByText('1,000 บ./คืน').length).toBeGreaterThan(0)
      expect(screen.getAllByText('1,500 บ./คืน').length).toBeGreaterThan(0)
      expect(screen.getAllByText('3,000 บ./คืน').length).toBeGreaterThan(0)
    })

    test('renders dash for rooms without price', async () => {
      render(<RoomPicker value={[]} onChange={jest.fn()} />)

      await waitForRoomsToLoad()

      // Room A01 has no price
      expect(screen.getByText('- บ./คืน')).toBeInTheDocument()
    })

    test('applies custom className', async () => {
      const { container } = render(
        <RoomPicker value={[]} onChange={jest.fn()} className="custom-class" />
      )

      await waitForRoomsToLoad()

      expect(container.firstChild).toHaveClass('custom-class')
    })

    test('shows empty state when no rooms', async () => {
      mockFetch.mockResolvedValue({
        ok: true,
        json: () => Promise.resolve({ success: true, data: [] }),
      })

      render(<RoomPicker value={[]} onChange={jest.fn()} />)

      await waitFor(() => {
        expect(screen.getByText('ไม่พบห้องพัก')).toBeInTheDocument()
      })
    })
  })

  describe('Room Type Filter', () => {
    test('renders filter label', async () => {
      render(<RoomPicker value={[]} onChange={jest.fn()} />)

      await waitForRoomsToLoad()

      expect(screen.getByText('กรองประเภท:')).toBeInTheDocument()
    })

    test('renders "All" filter button', async () => {
      render(<RoomPicker value={[]} onChange={jest.fn()} />)

      await waitForRoomsToLoad()

      expect(screen.getByText('ทั้งหมด')).toBeInTheDocument()
    })

    test('renders room type filter buttons', async () => {
      render(<RoomPicker value={[]} onChange={jest.fn()} />)

      await waitForRoomsToLoad()

      // Find filter buttons (they are in the filter area with rounded-full class)
      const buttons = screen.getAllByRole('button')
      const filterButtons = buttons.filter(btn => btn.classList.contains('rounded-full'))

      // Should have "All" + 3 room types = 4 buttons
      expect(filterButtons.length).toBe(4)
    })

    test('"All" filter is selected by default', async () => {
      render(<RoomPicker value={[]} onChange={jest.fn()} />)

      await waitForRoomsToLoad()

      const allButton = screen.getByText('ทั้งหมด')
      expect(allButton).toHaveClass('bg-red-600')
      expect(allButton).toHaveClass('text-white')
    })

    test('filters rooms by type when filter is clicked', async () => {
      render(<RoomPicker value={[]} onChange={jest.fn()} />)

      await waitForRoomsToLoad()

      // Find the Deluxe filter button
      const buttons = screen.getAllByRole('button')
      const deluxeFilter = buttons.find(
        (btn) => btn.textContent === 'Deluxe' && btn.classList.contains('rounded-full')
      )

      fireEvent.click(deluxeFilter!)

      // Should only show Deluxe rooms (201, 202)
      expect(screen.getByText('201')).toBeInTheDocument()
      expect(screen.getByText('202')).toBeInTheDocument()
      expect(screen.queryByText('101')).not.toBeInTheDocument()
      expect(screen.queryByText('301')).not.toBeInTheDocument()
    })

    test('shows all rooms when "All" filter is clicked', async () => {
      render(<RoomPicker value={[]} onChange={jest.fn()} />)

      await waitForRoomsToLoad()

      // First filter by type
      const buttons = screen.getAllByRole('button')
      const deluxeFilter = buttons.find(
        (btn) => btn.textContent === 'Deluxe' && btn.classList.contains('rounded-full')
      )
      fireEvent.click(deluxeFilter!)

      // Then click "All"
      fireEvent.click(screen.getByText('ทั้งหมด'))

      expect(screen.getByText('101')).toBeInTheDocument()
      expect(screen.getByText('201')).toBeInTheDocument()
      expect(screen.getByText('301')).toBeInTheDocument()
    })

    test('highlights selected filter', async () => {
      render(<RoomPicker value={[]} onChange={jest.fn()} />)

      await waitForRoomsToLoad()

      const buttons = screen.getAllByRole('button')
      const standardFilter = buttons.find(
        (btn) => btn.textContent === 'Standard' && btn.classList.contains('rounded-full')
      )
      fireEvent.click(standardFilter!)

      expect(standardFilter).toHaveClass('bg-red-600')
      expect(standardFilter).toHaveClass('text-white')
      expect(screen.getByText('ทั้งหมด')).toHaveClass('bg-gray-100')
    })
  })

  describe('Multi-select Toggle', () => {
    test('selects room on click', async () => {
      const mockOnChange = jest.fn()
      render(<RoomPicker value={[]} onChange={mockOnChange} />)

      await waitForRoomsToLoad()

      // Find the room button in the grid
      const gridArea = screen.getByText('ชั้น 1').parentElement
      const roomButtons = gridArea?.querySelectorAll('button')
      const room101Button = Array.from(roomButtons || []).find(
        btn => btn.textContent?.includes('101')
      )
      fireEvent.click(room101Button!)

      expect(mockOnChange).toHaveBeenCalledWith([
        expect.objectContaining({
          id: 1,
          roomNo: '101',
        }),
      ])
    })

    test('deselects room on second click', async () => {
      const mockOnChange = jest.fn()
      render(<RoomPicker value={[mockRooms[0]]} onChange={mockOnChange} />)

      await waitForRoomsToLoad()

      // Find room in grid area
      const gridArea = screen.getByText('ชั้น 1').parentElement
      const roomButtons = gridArea?.querySelectorAll('button')
      const room101Button = Array.from(roomButtons || []).find(
        btn => btn.textContent?.includes('101')
      )
      fireEvent.click(room101Button!)

      expect(mockOnChange).toHaveBeenCalledWith([])
    })

    test('allows selecting multiple rooms', async () => {
      const mockOnChange = jest.fn()
      const { rerender } = render(<RoomPicker value={[]} onChange={mockOnChange} />)

      await waitForRoomsToLoad()

      // Select first room in grid
      const gridArea1 = screen.getByText('ชั้น 1').parentElement
      const roomButtons1 = gridArea1?.querySelectorAll('button')
      const room101Button = Array.from(roomButtons1 || []).find(
        btn => btn.textContent?.includes('101')
      )
      fireEvent.click(room101Button!)

      // Update the value prop
      rerender(<RoomPicker value={[mockRooms[0]]} onChange={mockOnChange} />)

      // Select second room in grid
      const gridArea2 = screen.getByText('ชั้น 2').parentElement
      const roomButtons2 = gridArea2?.querySelectorAll('button')
      const room201Button = Array.from(roomButtons2 || []).find(
        btn => btn.textContent?.includes('201')
      )
      fireEvent.click(room201Button!)

      expect(mockOnChange).toHaveBeenLastCalledWith([
        expect.objectContaining({ roomNo: '101' }),
        expect.objectContaining({ roomNo: '201' }),
      ])
    })
  })

  describe('Selected Rooms Summary', () => {
    test('shows selected rooms count', async () => {
      render(
        <RoomPicker value={[mockRooms[0], mockRooms[2]]} onChange={jest.fn()} />
      )

      await waitForRoomsToLoad()

      expect(screen.getByText('เลือกแล้ว 2 ห้อง')).toBeInTheDocument()
    })

    test('shows selected room badges', async () => {
      render(
        <RoomPicker value={[mockRooms[0], mockRooms[2]]} onChange={jest.fn()} />
      )

      await waitForRoomsToLoad()

      // Find badges in the summary section
      const summary = screen.getByText('เลือกแล้ว 2 ห้อง').closest('div')?.parentElement
      expect(summary).toHaveTextContent('101')
      expect(summary).toHaveTextContent('201')
    })

    test('does not show summary when no rooms selected', async () => {
      render(<RoomPicker value={[]} onChange={jest.fn()} />)

      await waitForRoomsToLoad()

      expect(screen.queryByText(/เลือกแล้ว/)).not.toBeInTheDocument()
    })

    test('can remove room from badge', async () => {
      const mockOnChange = jest.fn()
      render(
        <RoomPicker value={[mockRooms[0], mockRooms[2]]} onChange={mockOnChange} />
      )

      await waitForRoomsToLoad()

      // Find the badges area
      const badges = screen.getByText('เลือกแล้ว 2 ห้อง').closest('div')?.parentElement
      // The × button is the one with textContent containing ×
      const badgeSpans = badges?.querySelectorAll('span.inline-flex')
      const room101Badge = Array.from(badgeSpans || []).find(span => span.textContent?.includes('101'))
      const removeButton = room101Badge?.querySelector('button')

      fireEvent.click(removeButton!)

      expect(mockOnChange).toHaveBeenCalledWith([
        expect.objectContaining({ roomNo: '201' }),
      ])
    })
  })

  describe('Clear All Functionality', () => {
    test('shows "Clear All" button when rooms are selected', async () => {
      render(<RoomPicker value={[mockRooms[0]]} onChange={jest.fn()} />)

      await waitForRoomsToLoad()

      expect(screen.getByText('ล้างทั้งหมด')).toBeInTheDocument()
    })

    test('clears all selected rooms on click', async () => {
      const mockOnChange = jest.fn()
      render(
        <RoomPicker value={[mockRooms[0], mockRooms[2]]} onChange={mockOnChange} />
      )

      await waitForRoomsToLoad()

      fireEvent.click(screen.getByText('ล้างทั้งหมด'))

      expect(mockOnChange).toHaveBeenCalledWith([])
    })
  })

  describe('Room Availability Status', () => {
    test('shows available room without status badge', async () => {
      render(<RoomPicker value={[]} onChange={jest.fn()} />)

      await waitForRoomsToLoad()

      // Find room 101 in grid
      const gridArea = screen.getByText('ชั้น 1').parentElement
      const roomButtons = gridArea?.querySelectorAll('button')
      const room101 = Array.from(roomButtons || []).find(
        btn => btn.textContent?.includes('101')
      )
      expect(room101).not.toHaveTextContent('มีผู้เข้าพัก')
    })

    test('shows occupied status badge', async () => {
      render(<RoomPicker value={[]} onChange={jest.fn()} />)

      await waitForRoomsToLoad()

      expect(screen.getByText('มีผู้เข้าพัก')).toBeInTheDocument()
    })

    test('shows maintenance status badge', async () => {
      render(<RoomPicker value={[]} onChange={jest.fn()} />)

      await waitForRoomsToLoad()

      expect(screen.getByText('ซ่อมบำรุง')).toBeInTheDocument()
    })

    test('shows cleaning status badge', async () => {
      render(<RoomPicker value={[]} onChange={jest.fn()} />)

      await waitForRoomsToLoad()

      expect(screen.getByText('ทำความสะอาด')).toBeInTheDocument()
    })

    test('unavailable rooms cannot be selected', async () => {
      const mockOnChange = jest.fn()
      render(<RoomPicker value={[]} onChange={mockOnChange} />)

      await waitForRoomsToLoad()

      // Room 102 is occupied - find it in grid
      const gridArea = screen.getByText('ชั้น 1').parentElement
      const roomButtons = gridArea?.querySelectorAll('button')
      const room102Button = Array.from(roomButtons || []).find(
        btn => btn.textContent?.includes('102')
      )
      expect(room102Button).toBeDisabled()

      fireEvent.click(room102Button!)
      expect(mockOnChange).not.toHaveBeenCalled()
    })

    test('applies disabled styling to unavailable rooms', async () => {
      render(<RoomPicker value={[]} onChange={jest.fn()} />)

      await waitForRoomsToLoad()

      // Find room 102 in grid
      const gridArea = screen.getByText('ชั้น 1').parentElement
      const roomButtons = gridArea?.querySelectorAll('button')
      const room102Button = Array.from(roomButtons || []).find(
        btn => btn.textContent?.includes('102')
      )
      expect(room102Button).toHaveClass('opacity-60')
      expect(room102Button).toHaveClass('cursor-not-allowed')
    })
  })

  describe('Selected Room Styling', () => {
    test('applies selected styling to chosen rooms', async () => {
      render(<RoomPicker value={[mockRooms[0]]} onChange={jest.fn()} />)

      await waitForRoomsToLoad()

      // Find in the grid area (not the badge area)
      const gridArea = screen.getByText('ชั้น 1').parentElement
      const room101Button = gridArea?.querySelector('button')
      expect(room101Button).toHaveClass('border-red-500')
      expect(room101Button).toHaveClass('bg-red-500/10')
    })

    test('shows checkmark on selected rooms', async () => {
      render(<RoomPicker value={[mockRooms[0]]} onChange={jest.fn()} />)

      await waitForRoomsToLoad()

      // Find in the grid area (not the badge area)
      const gridArea = screen.getByText('ชั้น 1').parentElement
      const room101Button = gridArea?.querySelector('button')
      const checkIcon = room101Button?.querySelector('[data-testid="check-icon"]')
      expect(checkIcon).toBeInTheDocument()
    })
  })

  describe('Disabled State', () => {
    test('all available room buttons are disabled when disabled prop is true', async () => {
      render(<RoomPicker value={[]} onChange={jest.fn()} disabled={true} />)

      await waitForRoomsToLoad()

      // Room 101 is available but should be disabled - find in grid
      const gridArea = screen.getByText('ชั้น 1').parentElement
      const roomButtons = gridArea?.querySelectorAll('button')
      const room101Button = Array.from(roomButtons || []).find(
        btn => btn.textContent?.includes('101')
      )
      expect(room101Button).toBeDisabled()
    })

    test('cannot select rooms when disabled', async () => {
      const mockOnChange = jest.fn()
      render(<RoomPicker value={[]} onChange={mockOnChange} disabled={true} />)

      await waitForRoomsToLoad()

      // Find room in grid
      const gridArea = screen.getByText('ชั้น 1').parentElement
      const roomButtons = gridArea?.querySelectorAll('button')
      const room101Button = Array.from(roomButtons || []).find(
        btn => btn.textContent?.includes('101')
      )
      fireEvent.click(room101Button!)

      expect(mockOnChange).not.toHaveBeenCalled()
    })

    test('applies disabled styling when disabled', async () => {
      render(<RoomPicker value={[]} onChange={jest.fn()} disabled={true} />)

      await waitForRoomsToLoad()

      // Find room in grid
      const gridArea = screen.getByText('ชั้น 1').parentElement
      const roomButtons = gridArea?.querySelectorAll('button')
      const room101Button = Array.from(roomButtons || []).find(
        btn => btn.textContent?.includes('101')
      )
      expect(room101Button).toHaveClass('cursor-not-allowed')
      expect(room101Button).toHaveClass('opacity-60')
    })
  })

  describe('API Parameters', () => {
    test('includes checkIn and checkOut dates in API call', async () => {
      render(
        <RoomPicker
          value={[]}
          onChange={jest.fn()}
          checkInDate="2026-01-20"
          checkOutDate="2026-01-25"
        />
      )

      await waitForRoomsToLoad()

      const apiCall = mockFetch.mock.calls[0][0]
      expect(apiCall).toContain('checkIn=2026-01-20')
      expect(apiCall).toContain('checkOut=2026-01-25')
    })

    test('includes excludeBookingId in API call', async () => {
      render(
        <RoomPicker
          value={[]}
          onChange={jest.fn()}
          checkInDate="2026-01-20"
          checkOutDate="2026-01-25"
          excludeBookingId={123}
        />
      )

      await waitForRoomsToLoad()

      const apiCall = mockFetch.mock.calls[0][0]
      expect(apiCall).toContain('excludeBookingId=123')
    })

    test('does not include excludeBookingId without dates', async () => {
      render(
        <RoomPicker value={[]} onChange={jest.fn()} excludeBookingId={123} />
      )

      await waitForRoomsToLoad()

      const apiCall = mockFetch.mock.calls[0][0]
      expect(apiCall).not.toContain('excludeBookingId')
    })
  })

  describe('Room Sorting', () => {
    test('sorts rooms by room number within each floor', async () => {
      const unsortedRooms = [
        { ...mockRooms[0], id: 8, roomNo: '103', floor: 1 },
        { ...mockRooms[0], roomNo: '101', floor: 1 },
        { ...mockRooms[0], id: 9, roomNo: '102', floor: 1 },
      ]

      mockFetch.mockResolvedValue({
        ok: true,
        json: () => Promise.resolve({ success: true, data: unsortedRooms }),
      })

      render(<RoomPicker value={[]} onChange={jest.fn()} />)

      await waitForRoomsToLoad()

      // Find floor 1 section and check order
      const floor1Section = screen.getByText('ชั้น 1').parentElement
      const roomNumbers = floor1Section?.querySelectorAll('.font-bold')
      const roomTexts = Array.from(roomNumbers || []).map((el) => el.textContent)

      expect(roomTexts).toEqual(['101', '102', '103'])
    })

    test('sorts floors in ascending order', async () => {
      render(<RoomPicker value={[]} onChange={jest.fn()} />)

      await waitForRoomsToLoad()

      const container = screen.getByText('ชั้น 1').closest('.space-y-4')
      const floorHeaders = container?.querySelectorAll('h4')
      const floors = Array.from(floorHeaders || []).map((h) => h.textContent)

      // Floor 0 (ไม่ระบุชั้น) comes first, then 1, 2, 3
      expect(floors.indexOf('ไม่ระบุชั้น')).toBeLessThan(floors.indexOf('ชั้น 1'))
      expect(floors.indexOf('ชั้น 1')).toBeLessThan(floors.indexOf('ชั้น 2'))
      expect(floors.indexOf('ชั้น 2')).toBeLessThan(floors.indexOf('ชั้น 3'))
    })
  })

  describe('Room Type Extraction', () => {
    test('extracts unique room types from rooms', async () => {
      render(<RoomPicker value={[]} onChange={jest.fn()} />)

      await waitForRoomsToLoad()

      // Find filter buttons (they have rounded-full class)
      const buttons = screen.getAllByRole('button')
      const filterButtons = buttons.filter(btn => btn.classList.contains('rounded-full'))

      // 4 buttons: All + 3 room types (Standard, Deluxe, Suite)
      expect(filterButtons.length).toBe(4)
    })

    test('uses fallback name for types without roomTypeName', async () => {
      const roomsWithoutTypeName = [
        {
          ...mockRooms[0],
          roomTypeName: null,
          roomTypeId: 99,
        },
      ]

      mockFetch.mockResolvedValue({
        ok: true,
        json: () => Promise.resolve({ success: true, data: roomsWithoutTypeName }),
      })

      render(<RoomPicker value={[]} onChange={jest.fn()} />)

      await waitForRoomsToLoad()

      expect(screen.getByText('ประเภท 99')).toBeInTheDocument()
    })
  })
})
