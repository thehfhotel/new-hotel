/**
 * @jest-environment jsdom
 */

import { render, screen, fireEvent } from '@testing-library/react'
import RoomGrid, { Room, RoomStatus } from '@/components/RoomGrid'

// Mock lucide-react icons
jest.mock('lucide-react', () => ({
  X: () => <span data-testid="x-icon">X</span>,
}))

const mockRooms: Room[] = [
  {
    id: 1,
    roomNumber: '301',
    type: 'Standard',
    details: 'Sea View',
    status: 'available',
    floor: 3,
  },
  {
    id: 2,
    roomNumber: '302',
    type: 'Deluxe',
    details: 'City View',
    status: 'occupied',
    floor: 3,
    guestName: 'John Doe',
    checkIn: '2026-01-20',
    checkOut: '2026-01-25',
  },
  {
    id: 3,
    roomNumber: '303',
    type: 'Suite',
    status: 'maintenance',
    floor: 3,
  },
  {
    id: 4,
    roomNumber: '304',
    type: 'Standard',
    status: 'cleaning',
    floor: 3,
  },
  {
    id: 5,
    roomNumber: '305',
    type: 'Standard',
    status: 'checkout',
    floor: 3,
  },
]

describe('RoomGrid Component', () => {
  describe('Rendering', () => {
    test('renders rooms correctly', () => {
      render(<RoomGrid rooms={mockRooms} />)

      // Check that room numbers are displayed
      expect(screen.getByText('301')).toBeInTheDocument()
      expect(screen.getByText('302')).toBeInTheDocument()
      expect(screen.getByText('303')).toBeInTheDocument()
    })

    test('renders room types', () => {
      render(<RoomGrid rooms={mockRooms} />)

      expect(screen.getAllByText('Standard').length).toBeGreaterThan(0)
      expect(screen.getByText('Deluxe')).toBeInTheDocument()
      expect(screen.getByText('Suite')).toBeInTheDocument()
    })

    test('renders legend items', () => {
      render(<RoomGrid rooms={mockRooms} />)

      // Check all legend items are present
      expect(screen.getByText('ว่าง')).toBeInTheDocument()
      expect(screen.getByText('มีผู้เข้าพัก')).toBeInTheDocument()
      expect(screen.getByText('ซ่อมบำรุง')).toBeInTheDocument()
      expect(screen.getByText('ทำความสะอาด')).toBeInTheDocument()
      expect(screen.getByText('รอเช็คเอาท์')).toBeInTheDocument()
    })
  })

  describe('Status Colors', () => {
    test('applies correct styling for available rooms', () => {
      render(<RoomGrid rooms={mockRooms} />)

      const room301 = screen.getByText('301').closest('button')
      expect(room301).toHaveClass('bg-white')
      expect(room301).toHaveClass('border-b-green-500')
    })

    test('applies correct styling for occupied rooms', () => {
      render(<RoomGrid rooms={mockRooms} />)

      const room302 = screen.getByText('302').closest('button')
      expect(room302).toHaveClass('bg-red-50')
      expect(room302).toHaveClass('border-b-red-500')
    })

    test('applies correct styling for maintenance rooms', () => {
      render(<RoomGrid rooms={mockRooms} />)

      const room303 = screen.getByText('303').closest('button')
      expect(room303).toHaveClass('bg-gray-100')
      expect(room303).toHaveClass('border-b-gray-400')
    })
  })

  describe('Modal Interaction', () => {
    test('opens modal when room is clicked', () => {
      render(<RoomGrid rooms={mockRooms} />)

      // Click on room 302 (occupied room)
      const room302 = screen.getByText('302').closest('button')
      fireEvent.click(room302!)

      // Modal should display room details
      expect(screen.getByText('ห้อง 302')).toBeInTheDocument()
      expect(screen.getByText('สถานะ:')).toBeInTheDocument()
    })

    test('displays guest information in modal for occupied room', () => {
      render(<RoomGrid rooms={mockRooms} />)

      const room302 = screen.getByText('302').closest('button')
      fireEvent.click(room302!)

      expect(screen.getByText('ผู้เข้าพัก:')).toBeInTheDocument()
      expect(screen.getByText('John Doe')).toBeInTheDocument()
    })

    test('closes modal when close button is clicked', () => {
      render(<RoomGrid rooms={mockRooms} />)

      // Open modal
      const room301 = screen.getByText('301').closest('button')
      fireEvent.click(room301!)
      expect(screen.getByText('ห้อง 301')).toBeInTheDocument()

      // Close modal via the close button (ปิด)
      const closeButton = screen.getByText('ปิด')
      fireEvent.click(closeButton)

      // Modal should be closed - room details header should not be visible
      expect(screen.queryByText('ห้อง 301')).not.toBeInTheDocument()
    })

    test('calls onRoomClick callback when room is clicked', () => {
      const mockCallback = jest.fn()
      render(<RoomGrid rooms={mockRooms} onRoomClick={mockCallback} />)

      const room301 = screen.getByText('301').closest('button')
      fireEvent.click(room301!)

      expect(mockCallback).toHaveBeenCalledWith(
        expect.objectContaining({
          roomNumber: '301',
          status: 'available',
        })
      )
    })
  })

  describe('Empty State', () => {
    test('handles rooms not in layout gracefully', () => {
      // Rooms not matching layout room numbers will show "ไม่พบ"
      render(<RoomGrid rooms={[]} />)

      // The grid should still render with placeholder text for missing rooms
      const notFoundElements = screen.getAllByText('ไม่พบ')
      expect(notFoundElements.length).toBeGreaterThan(0)
    })
  })
})
