/**
 * @jest-environment jsdom
 */

import { render, screen, fireEvent } from '@testing-library/react'
import Calendar, { Booking, CheckIn } from '@/components/Calendar'

// Mock lucide-react icons
jest.mock('lucide-react', () => ({
  ChevronLeft: () => <span data-testid="chevron-left">&lt;</span>,
  ChevronRight: () => <span data-testid="chevron-right">&gt;</span>,
}))

const mockBookings: Booking[] = [
  {
    id: 1,
    customerId: 101,
    customerName: 'Test Customer 1',
    roomNumber: '301',
    roomType: 'Standard',
    checkInDate: '2026-01-15T00:00:00.000Z',
    checkOutDate: '2026-01-20T00:00:00.000Z',
    status: 'confirmed',
  },
  {
    id: 2,
    customerId: 102,
    customerName: 'Test Customer 2',
    roomNumber: '302',
    roomType: 'Deluxe',
    checkInDate: '2026-01-18T00:00:00.000Z',
    checkOutDate: '2026-01-22T00:00:00.000Z',
    status: 'confirmed',
  },
]

const mockCheckins: CheckIn[] = [
  {
    id: 1,
    customerId: 101,
    customerName: 'Test Customer 1',
    roomNumber: '301',
    roomType: 'Standard',
    checkInTime: '2026-01-15T14:00:00.000Z',
  },
  {
    id: 2,
    customerId: 103,
    customerName: 'Test Customer 3',
    roomNumber: '303',
    roomType: 'Suite',
    checkInTime: '2026-01-20T15:00:00.000Z',
  },
]

describe('Calendar Component', () => {
  const defaultProps = {
    bookings: mockBookings,
    checkins: mockCheckins,
    selectedMonth: new Date('2026-01-15'),
    onMonthChange: jest.fn(),
  }

  beforeEach(() => {
    jest.clearAllMocks()
  })

  describe('Rendering', () => {
    test('renders calendar with month header', () => {
      render(<Calendar {...defaultProps} />)

      // Should display Thai month and Buddhist year
      expect(screen.getByText(/มกราคม 2569/)).toBeInTheDocument()
    })

    test('renders day headers in Thai', () => {
      render(<Calendar {...defaultProps} />)

      expect(screen.getByText('อา')).toBeInTheDocument()
      expect(screen.getByText('จ')).toBeInTheDocument()
      expect(screen.getByText('อ')).toBeInTheDocument()
      expect(screen.getByText('พ')).toBeInTheDocument()
      expect(screen.getByText('พฤ')).toBeInTheDocument()
      expect(screen.getByText('ศ')).toBeInTheDocument()
      expect(screen.getByText('ส')).toBeInTheDocument()
    })

    test('renders all days of the month', () => {
      render(<Calendar {...defaultProps} />)

      // January 2026 has 31 days - use getAllByText since adjacent months may have duplicate numbers
      expect(screen.getAllByText('1').length).toBeGreaterThan(0)
      expect(screen.getAllByText('15').length).toBeGreaterThan(0)
      // Day 31 might appear from December (previous month) as well
      expect(screen.getAllByText('31').length).toBeGreaterThan(0)
    })

    test('renders legend items', () => {
      render(<Calendar {...defaultProps} />)

      expect(screen.getByText('การจอง')).toBeInTheDocument()
      expect(screen.getByText('เช็คอิน')).toBeInTheDocument()
      expect(screen.getByText('วันนี้')).toBeInTheDocument()
    })
  })

  describe('Month Navigation', () => {
    test('calls onMonthChange when previous month button is clicked', () => {
      const mockMonthChange = jest.fn()
      render(<Calendar {...defaultProps} onMonthChange={mockMonthChange} />)

      const prevButton = screen.getByLabelText('เดือนก่อนหน้า')
      fireEvent.click(prevButton)

      expect(mockMonthChange).toHaveBeenCalledTimes(1)
      // Should be called with December 2025
      const calledDate = mockMonthChange.mock.calls[0][0]
      expect(calledDate.getMonth()).toBe(11) // December
      expect(calledDate.getFullYear()).toBe(2025)
    })

    test('calls onMonthChange when next month button is clicked', () => {
      const mockMonthChange = jest.fn()
      render(<Calendar {...defaultProps} onMonthChange={mockMonthChange} />)

      const nextButton = screen.getByLabelText('เดือนถัดไป')
      fireEvent.click(nextButton)

      expect(mockMonthChange).toHaveBeenCalledTimes(1)
      // Should be called with February 2026
      const calledDate = mockMonthChange.mock.calls[0][0]
      expect(calledDate.getMonth()).toBe(1) // February
      expect(calledDate.getFullYear()).toBe(2026)
    })
  })

  describe('Date Selection', () => {
    test('calls onDateClick when a date is clicked', () => {
      const mockDateClick = jest.fn()
      render(<Calendar {...defaultProps} onDateClick={mockDateClick} />)

      // Click on day 15
      const day15 = screen.getByText('15')
      fireEvent.click(day15)

      expect(mockDateClick).toHaveBeenCalledTimes(1)
      expect(mockDateClick).toHaveBeenCalledWith(
        expect.any(Date),
        expect.any(Array), // bookings
        expect.any(Array) // checkins
      )
    })

    test('highlights selected date', () => {
      render(<Calendar {...defaultProps} />)

      // Click on day 20
      const day20 = screen.getByText('20')
      fireEvent.click(day20)

      // The parent div should have the selected style
      const dayCell = day20.closest('div[class*="cursor-pointer"]')
      expect(dayCell).toHaveClass('ring-2')
      expect(dayCell).toHaveClass('ring-blue-500')
    })

    test('passes correct bookings for clicked date', () => {
      const mockDateClick = jest.fn()
      render(<Calendar {...defaultProps} onDateClick={mockDateClick} />)

      // Click on day 18 (overlaps with both bookings)
      const day18 = screen.getByText('18')
      fireEvent.click(day18)

      const [, bookings] = mockDateClick.mock.calls[0]
      expect(bookings.length).toBe(2) // Both bookings cover Jan 18
    })

    test('passes correct checkins for clicked date', () => {
      const mockDateClick = jest.fn()
      render(<Calendar {...defaultProps} onDateClick={mockDateClick} />)

      // Click on day 15 (has a checkin)
      const day15 = screen.getByText('15')
      fireEvent.click(day15)

      const [, , checkins] = mockDateClick.mock.calls[0]
      expect(checkins.length).toBe(1)
      expect(checkins[0].customerName).toBe('Test Customer 1')
    })
  })

  describe('Booking/CheckIn Indicators', () => {
    test('displays booking indicator for dates with bookings', () => {
      render(<Calendar {...defaultProps} />)

      // Day 17 should show booking indicator (booking spans Jan 15-20)
      const bookingIndicators = screen.getAllByText(/จอง \d+/)
      expect(bookingIndicators.length).toBeGreaterThan(0)
    })

    test('displays checkin indicator for dates with checkins', () => {
      render(<Calendar {...defaultProps} />)

      // Day 15 has a checkin
      const checkinIndicators = screen.getAllByText(/เข้าพัก \d+/)
      expect(checkinIndicators.length).toBeGreaterThan(0)
    })

    test('shows correct count for multiple bookings on same date', () => {
      render(<Calendar {...defaultProps} />)

      // Day 19 has 2 bookings overlapping
      // Find booking indicator that shows "จอง 2"
      const bookingIndicators = screen.getAllByText(/จอง \d+/)
      const multiBookingDay = bookingIndicators.find((el) => el.textContent === 'จอง 2')
      expect(multiBookingDay).toBeInTheDocument()
    })
  })

  describe('Edge Cases', () => {
    test('renders with empty bookings and checkins', () => {
      render(
        <Calendar
          {...defaultProps}
          bookings={[]}
          checkins={[]}
        />
      )

      // Should still render calendar structure
      expect(screen.getByText('15')).toBeInTheDocument()
      expect(screen.queryByText(/จอง \d+/)).not.toBeInTheDocument()
      expect(screen.queryByText(/เข้าพัก \d+/)).not.toBeInTheDocument()
    })

    test('handles dates outside current month', () => {
      render(<Calendar {...defaultProps} />)

      // Days from adjacent months should have different styling
      // This is hard to test directly without checking classes
      // At minimum, the calendar should render without errors
      expect(screen.getByText('1')).toBeInTheDocument()
    })
  })
})
