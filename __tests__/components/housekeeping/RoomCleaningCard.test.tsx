/**
 * @jest-environment jsdom
 */

import { render, screen, fireEvent, waitFor, act } from '@testing-library/react'
import RoomCleaningCard, { HousekeepingRoom } from '@/components/housekeeping/RoomCleaningCard'

// Mock lucide-react icons
jest.mock('lucide-react', () => ({
  Clock: () => <span data-testid="clock-icon">Clock</span>,
  AlertTriangle: () => <span data-testid="alert-triangle-icon">AlertTriangle</span>,
  Play: () => <span data-testid="play-icon">Play</span>,
  CheckCircle: () => <span data-testid="check-circle-icon">CheckCircle</span>,
  User: () => <span data-testid="user-icon">User</span>,
  MessageSquare: () => <span data-testid="message-square-icon">MessageSquare</span>,
  ChevronDown: () => <span data-testid="chevron-down-icon">ChevronDown</span>,
  ChevronUp: () => <span data-testid="chevron-up-icon">ChevronUp</span>,
}))

describe('RoomCleaningCard Component', () => {
  // Base mock room for tests
  const createMockRoom = (overrides: Partial<HousekeepingRoom> = {}): HousekeepingRoom => ({
    id: 1,
    roomNo: '301',
    roomTypeName: 'Standard',
    floor: 3,
    status: 'dirty',
    lastCheckoutTime: null,
    statusChangedAt: null,
    housekeeperName: null,
    notes: null,
    ...overrides,
  })

  const mockOnStatusChange = jest.fn().mockResolvedValue(undefined)

  beforeEach(() => {
    jest.clearAllMocks()
    // Mock current date/time for consistent testing
    jest.useFakeTimers()
    jest.setSystemTime(new Date('2026-01-20T14:00:00.000Z'))
  })

  afterEach(() => {
    jest.useRealTimers()
  })

  describe('Room Info Display', () => {
    test('displays room number', () => {
      render(<RoomCleaningCard room={createMockRoom()} onStatusChange={mockOnStatusChange} />)

      expect(screen.getByText('301')).toBeInTheDocument()
    })

    test('displays room type', () => {
      render(<RoomCleaningCard room={createMockRoom({ roomTypeName: 'Deluxe' })} onStatusChange={mockOnStatusChange} />)

      expect(screen.getByText('Deluxe')).toBeInTheDocument()
    })

    test('displays default text when room type is null', () => {
      render(<RoomCleaningCard room={createMockRoom({ roomTypeName: null })} onStatusChange={mockOnStatusChange} />)

      expect(screen.getByText('ไม่ระบุประเภท')).toBeInTheDocument()
    })

    test('displays floor number', () => {
      render(<RoomCleaningCard room={createMockRoom({ floor: 5 })} onStatusChange={mockOnStatusChange} />)

      expect(screen.getByText('ชั้น 5')).toBeInTheDocument()
    })

    test('does not display floor when null', () => {
      render(<RoomCleaningCard room={createMockRoom({ floor: null })} onStatusChange={mockOnStatusChange} />)

      expect(screen.queryByText(/ชั้น/)).not.toBeInTheDocument()
    })

    test('displays housekeeper name when assigned', () => {
      render(
        <RoomCleaningCard
          room={createMockRoom({ status: 'cleaning', housekeeperName: 'สมศรี' })}
          onStatusChange={mockOnStatusChange}
        />
      )

      expect(screen.getByText('สมศรี')).toBeInTheDocument()
    })
  })

  describe('Status-based Styling', () => {
    test('dirty room has red border styling', () => {
      const { container } = render(
        <RoomCleaningCard room={createMockRoom({ status: 'dirty' })} onStatusChange={mockOnStatusChange} />
      )

      const card = container.firstChild as HTMLElement
      expect(card.className).toContain('bg-white')
      expect(card.className).toContain('border-red-900')
    })

    test('cleaning room has amber border styling', () => {
      const { container } = render(
        <RoomCleaningCard room={createMockRoom({ status: 'cleaning' })} onStatusChange={mockOnStatusChange} />
      )

      const card = container.firstChild as HTMLElement
      expect(card.className).toContain('bg-white')
      expect(card.className).toContain('border-amber-900')
    })

    test('available room has emerald border styling', () => {
      const { container } = render(
        <RoomCleaningCard room={createMockRoom({ status: 'available' })} onStatusChange={mockOnStatusChange} />
      )

      const card = container.firstChild as HTMLElement
      expect(card.className).toContain('bg-white')
      expect(card.className).toContain('border-emerald-900')
    })
  })

  describe('Priority Badge', () => {
    test('shows priority badge when dirty room is over 2 hours old', () => {
      // Last checkout 3 hours ago
      const threeHoursAgo = new Date('2026-01-20T11:00:00.000Z').toISOString()

      render(
        <RoomCleaningCard
          room={createMockRoom({ status: 'dirty', lastCheckoutTime: threeHoursAgo })}
          onStatusChange={mockOnStatusChange}
        />
      )

      expect(screen.getByText('ด่วน')).toBeInTheDocument()
      expect(screen.getByTestId('alert-triangle-icon')).toBeInTheDocument()
    })

    test('does not show priority badge when dirty room is under 2 hours old', () => {
      // Last checkout 1 hour ago
      const oneHourAgo = new Date('2026-01-20T13:00:00.000Z').toISOString()

      render(
        <RoomCleaningCard
          room={createMockRoom({ status: 'dirty', lastCheckoutTime: oneHourAgo })}
          onStatusChange={mockOnStatusChange}
        />
      )

      expect(screen.queryByText('ด่วน')).not.toBeInTheDocument()
    })

    test('does not show priority badge for cleaning rooms even if over 2 hours', () => {
      const threeHoursAgo = new Date('2026-01-20T11:00:00.000Z').toISOString()

      render(
        <RoomCleaningCard
          room={createMockRoom({ status: 'cleaning', lastCheckoutTime: threeHoursAgo })}
          onStatusChange={mockOnStatusChange}
        />
      )

      expect(screen.queryByText('ด่วน')).not.toBeInTheDocument()
    })

    test('does not show priority badge for available rooms', () => {
      const threeHoursAgo = new Date('2026-01-20T11:00:00.000Z').toISOString()

      render(
        <RoomCleaningCard
          room={createMockRoom({ status: 'available', lastCheckoutTime: threeHoursAgo })}
          onStatusChange={mockOnStatusChange}
        />
      )

      expect(screen.queryByText('ด่วน')).not.toBeInTheDocument()
    })

    test('shows priority badge at exactly 121 minutes (over 2 hours)', () => {
      // 121 minutes = 2 hours 1 minute ago
      const over2Hours = new Date(Date.now() - 121 * 60 * 1000).toISOString()

      render(
        <RoomCleaningCard
          room={createMockRoom({ status: 'dirty', lastCheckoutTime: over2Hours })}
          onStatusChange={mockOnStatusChange}
        />
      )

      expect(screen.getByText('ด่วน')).toBeInTheDocument()
    })
  })

  describe('Action Buttons', () => {
    test('dirty room shows "เริ่มทำความสะอาด" button', () => {
      render(<RoomCleaningCard room={createMockRoom({ status: 'dirty' })} onStatusChange={mockOnStatusChange} />)

      expect(screen.getByText('เริ่มทำความสะอาด')).toBeInTheDocument()
    })

    test('cleaning room shows "เสร็จแล้ว" button', () => {
      render(<RoomCleaningCard room={createMockRoom({ status: 'cleaning' })} onStatusChange={mockOnStatusChange} />)

      expect(screen.getByText('เสร็จแล้ว')).toBeInTheDocument()
    })

    test('available room shows "ทำเครื่องหมายสกปรก" button', () => {
      render(<RoomCleaningCard room={createMockRoom({ status: 'available' })} onStatusChange={mockOnStatusChange} />)

      expect(screen.getByText('ทำเครื่องหมายสกปรก')).toBeInTheDocument()
    })
  })

  describe('onStatusChange Callback', () => {
    test('calls onStatusChange with "cleaning" when start cleaning button clicked', async () => {
      render(<RoomCleaningCard room={createMockRoom({ status: 'dirty' })} onStatusChange={mockOnStatusChange} />)

      const button = screen.getByText('เริ่มทำความสะอาด')
      await act(async () => {
        fireEvent.click(button)
      })

      expect(mockOnStatusChange).toHaveBeenCalledWith(1, 'cleaning', undefined)
    })

    test('calls onStatusChange with "available" when finish button clicked', async () => {
      render(<RoomCleaningCard room={createMockRoom({ status: 'cleaning' })} onStatusChange={mockOnStatusChange} />)

      const button = screen.getByText('เสร็จแล้ว')
      await act(async () => {
        fireEvent.click(button)
      })

      expect(mockOnStatusChange).toHaveBeenCalledWith(1, 'available', undefined)
    })

    test('calls onStatusChange with "dirty" when mark dirty button clicked', async () => {
      render(<RoomCleaningCard room={createMockRoom({ status: 'available' })} onStatusChange={mockOnStatusChange} />)

      const button = screen.getByText('ทำเครื่องหมายสกปรก')
      await act(async () => {
        fireEvent.click(button)
      })

      expect(mockOnStatusChange).toHaveBeenCalledWith(1, 'dirty', undefined)
    })

    test('includes notes in callback when provided', async () => {
      render(<RoomCleaningCard room={createMockRoom({ status: 'dirty' })} onStatusChange={mockOnStatusChange} />)

      // Open notes section
      const notesToggle = screen.getByText('บันทึก')
      fireEvent.click(notesToggle)

      // Enter notes
      const textarea = screen.getByPlaceholderText('เพิ่มบันทึก...')
      fireEvent.change(textarea, { target: { value: 'Test notes' } })

      // Click status change button
      const button = screen.getByText('เริ่มทำความสะอาด')
      await act(async () => {
        fireEvent.click(button)
      })

      expect(mockOnStatusChange).toHaveBeenCalledWith(1, 'cleaning', 'Test notes')
    })
  })

  describe('Notes Section', () => {
    test('notes section is hidden by default', () => {
      render(<RoomCleaningCard room={createMockRoom()} onStatusChange={mockOnStatusChange} />)

      expect(screen.queryByPlaceholderText('เพิ่มบันทึก...')).not.toBeInTheDocument()
    })

    test('clicking notes toggle shows textarea', () => {
      render(<RoomCleaningCard room={createMockRoom()} onStatusChange={mockOnStatusChange} />)

      const notesToggle = screen.getByText('บันทึก')
      fireEvent.click(notesToggle)

      expect(screen.getByPlaceholderText('เพิ่มบันทึก...')).toBeInTheDocument()
    })

    test('clicking notes toggle again hides textarea', () => {
      render(<RoomCleaningCard room={createMockRoom()} onStatusChange={mockOnStatusChange} />)

      const notesToggle = screen.getByText('บันทึก')
      fireEvent.click(notesToggle) // Open
      fireEvent.click(notesToggle) // Close

      expect(screen.queryByPlaceholderText('เพิ่มบันทึก...')).not.toBeInTheDocument()
    })

    test('pre-populates notes from room prop', () => {
      render(
        <RoomCleaningCard room={createMockRoom({ notes: 'Existing notes' })} onStatusChange={mockOnStatusChange} />
      )

      const notesToggle = screen.getByText('บันทึก')
      fireEvent.click(notesToggle)

      const textarea = screen.getByPlaceholderText('เพิ่มบันทึก...')
      expect(textarea).toHaveValue('Existing notes')
    })
  })

  describe('Disabled State', () => {
    test('buttons are disabled when disabled prop is true', () => {
      render(<RoomCleaningCard room={createMockRoom({ status: 'dirty' })} onStatusChange={mockOnStatusChange} disabled />)

      const button = screen.getByText('เริ่มทำความสะอาด').closest('button')
      expect(button).toBeDisabled()
    })

    test('textarea is disabled when disabled prop is true', () => {
      render(<RoomCleaningCard room={createMockRoom({ status: 'dirty' })} onStatusChange={mockOnStatusChange} disabled />)

      const notesToggle = screen.getByText('บันทึก')
      fireEvent.click(notesToggle)

      const textarea = screen.getByPlaceholderText('เพิ่มบันทึก...')
      expect(textarea).toBeDisabled()
    })

    test('clicking disabled button does not call onStatusChange', async () => {
      render(<RoomCleaningCard room={createMockRoom({ status: 'dirty' })} onStatusChange={mockOnStatusChange} disabled />)

      const button = screen.getByText('เริ่มทำความสะอาด')
      fireEvent.click(button)

      expect(mockOnStatusChange).not.toHaveBeenCalled()
    })
  })

  describe('Time Display', () => {
    test('displays checkout time for dirty rooms', () => {
      const checkoutTime = '2026-01-20T10:30:00.000Z'

      render(
        <RoomCleaningCard
          room={createMockRoom({ status: 'dirty', lastCheckoutTime: checkoutTime })}
          onStatusChange={mockOnStatusChange}
        />
      )

      // Should show checkout time formatted (10:30 in UTC)
      expect(screen.getByText(/เช็คเอาท์/)).toBeInTheDocument()
      expect(screen.getByText(/10:30/)).toBeInTheDocument()
    })

    test('displays time elapsed for dirty rooms', () => {
      // 90 minutes ago
      const ninetyMinutesAgo = new Date('2026-01-20T12:30:00.000Z').toISOString()

      render(
        <RoomCleaningCard
          room={createMockRoom({ status: 'dirty', lastCheckoutTime: ninetyMinutesAgo })}
          onStatusChange={mockOnStatusChange}
        />
      )

      // Should show "1 ชม. 30 น.ที่แล้ว"
      expect(screen.getByText(/1 ชม\. 30 น\.ที่แล้ว/)).toBeInTheDocument()
    })

    test('displays cleaning time for cleaning rooms', () => {
      // Started cleaning 45 minutes ago
      const fortyFiveMinutesAgo = new Date('2026-01-20T13:15:00.000Z').toISOString()

      render(
        <RoomCleaningCard
          room={createMockRoom({ status: 'cleaning', statusChangedAt: fortyFiveMinutesAgo })}
          onStatusChange={mockOnStatusChange}
        />
      )

      expect(screen.getByText(/กำลังทำ/)).toBeInTheDocument()
      expect(screen.getByText(/45 นาที/)).toBeInTheDocument()
    })

    test('formats time correctly for hours only', () => {
      // Exactly 2 hours ago
      const twoHoursAgo = new Date('2026-01-20T12:00:00.000Z').toISOString()

      render(
        <RoomCleaningCard
          room={createMockRoom({ status: 'cleaning', statusChangedAt: twoHoursAgo })}
          onStatusChange={mockOnStatusChange}
        />
      )

      expect(screen.getByText(/2 ชม\./)).toBeInTheDocument()
    })
  })

  describe('Loading State During Update', () => {
    test('shows loading state while updating', async () => {
      const slowOnStatusChange = jest.fn(
        () =>
          new Promise<void>((resolve) => {
            setTimeout(resolve, 1000)
          })
      )

      render(
        <RoomCleaningCard room={createMockRoom({ status: 'dirty' })} onStatusChange={slowOnStatusChange} />
      )

      const button = screen.getByText('เริ่มทำความสะอาด').closest('button')

      // Start the update
      act(() => {
        fireEvent.click(button!)
      })

      // Button should be disabled during update
      await waitFor(() => {
        expect(button).toBeDisabled()
      })

      // Complete the update
      await act(async () => {
        jest.advanceTimersByTime(1000)
      })
    })
  })
})
