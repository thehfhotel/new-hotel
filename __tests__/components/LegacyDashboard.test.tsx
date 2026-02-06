/**
 * @jest-environment jsdom
 */

import { render, screen, waitFor } from '@testing-library/react'
import Dashboard from '@/app/(legacy)/page'
import { mockFetch, mockFetchError } from '../utils/commonMocks'

// Mock lucide-react icons (use require to avoid hoisting issue)
jest.mock('lucide-react', () => require('../utils/commonMocks').mockLucideIcons)

// Mock RoomGrid (legacy-only visual component)
jest.mock('@/components/RoomGrid', () => {
  const MockRoomGrid = ({ rooms }: { rooms: unknown[] }) => (
    <div data-testid="room-grid">RoomGrid ({rooms.length} rooms)</div>
  )
  MockRoomGrid.displayName = 'MockRoomGrid'
  return {
    __esModule: true,
    default: MockRoomGrid,
  }
})

// Mock Charts (recharts doesn't work in jsdom)
jest.mock('@/components/Charts', () => ({
  OccupancyChart: ({ data, title }: { data: unknown[]; title?: string }) => (
    <div data-testid="occupancy-chart">{title || 'OccupancyChart'} ({data.length} points)</div>
  ),
}))

// Mock API response data
const mockStatsData = {
  totalRooms: 40,
  occupiedRooms: 15,
  checkoutRooms: 3,
  bookedRooms: 5,
  todayCheckIns: 4,
  activeBookings: 8,
}

const mockRoomsData = [
  { Room_no: '101', Room_Type: 'Standard', Room_Details: 'Single bed', Room_Clean: 'yes', Room_Use: 'yes', Room_Book: '', Room_Manternace: '' },
  { Room_no: '102', Room_Type: 'Deluxe', Room_Details: 'Double bed', Room_Clean: 'yes', Room_Use: '', Room_Book: '', Room_Manternace: '' },
  { Room_no: '103', Room_Type: 'Suite', Room_Details: 'King bed', Room_Clean: 'yes', Room_Use: '', Room_Book: 'yes', Room_Manternace: '' },
]

const mockCheckInsData = [
  { Cin_no: 'CI-001', Cin_Room_No: '101', Cin_Room_In: '2026-02-05T10:00:00.000Z', Cin_Room_Out: '2026-02-07T12:00:00.000Z', Cin_cust_name: 'สมชาย ใจดี', Cin_status: 'เช็คอิน' },
  { Cin_no: 'CI-002', Cin_Room_No: '201', Cin_Room_In: '2026-02-04T14:00:00.000Z', Cin_Room_Out: '2026-02-06T12:00:00.000Z', Cin_cust_name: 'สมหญิง รักสุข', Cin_status: 'เช็คอิน' },
]

const mockOccupancyData = [
  { date: '2026-02-01', occupiedRooms: 10 },
  { date: '2026-02-02', occupiedRooms: 12 },
  { date: '2026-02-03', occupiedRooms: 8 },
]

function setupSuccessfulFetch() {
  global.fetch = mockFetch({
    '/api/stats': { data: mockStatsData, success: true },
    '/api/rooms/checkouts-today': { data: ['101'], success: true },
    '/api/rooms': { data: mockRoomsData, success: true },
    '/api/checkins': { data: mockCheckInsData, success: true },
    '/api/occupancy': { data: mockOccupancyData, success: true },
  })
}

describe('Legacy Dashboard Page', () => {
  beforeEach(() => {
    jest.clearAllMocks()
  })

  afterEach(() => {
    jest.restoreAllMocks()
  })

  test('renders loading state initially', () => {
    // Use a fetch that never resolves to keep loading state
    global.fetch = jest.fn().mockImplementation(() => new Promise(() => {}))

    render(<Dashboard />)

    expect(screen.getByText('กำลังโหลดข้อมูล...')).toBeInTheDocument()
  })

  test('renders page title after loading', async () => {
    setupSuccessfulFetch()

    render(<Dashboard />)

    await waitFor(() => {
      expect(screen.getByText('หน้าหลัก')).toBeInTheDocument()
    })
  })

  test('renders 3 stats cards with Thai labels', async () => {
    setupSuccessfulFetch()

    render(<Dashboard />)

    await waitFor(() => {
      expect(screen.getByText('ห้องที่มีผู้เข้าพัก')).toBeInTheDocument()
    })
    expect(screen.getByText('ห้องที่จองแล้ว')).toBeInTheDocument()
    expect(screen.getByText('ห้องรอเช็คเอาท์')).toBeInTheDocument()
  })

  test('renders stats card values from API', async () => {
    setupSuccessfulFetch()

    render(<Dashboard />)

    await waitFor(() => {
      expect(screen.getByText('15')).toBeInTheDocument() // occupiedRooms
    })
    expect(screen.getByText('5')).toBeInTheDocument() // bookedRooms
    expect(screen.getByText('3')).toBeInTheDocument() // checkoutRooms
  })

  test('renders room grid section', async () => {
    setupSuccessfulFetch()

    render(<Dashboard />)

    await waitFor(() => {
      expect(screen.getByText('สถานะห้องพัก')).toBeInTheDocument()
    })
    expect(screen.getByTestId('room-grid')).toBeInTheDocument()
  })

  test('renders occupancy chart', async () => {
    setupSuccessfulFetch()

    render(<Dashboard />)

    await waitFor(() => {
      expect(screen.getByTestId('occupancy-chart')).toBeInTheDocument()
    })
  })

  test('renders recent activity section with check-in entries', async () => {
    setupSuccessfulFetch()

    render(<Dashboard />)

    await waitFor(() => {
      expect(screen.getByText('กิจกรรมล่าสุด')).toBeInTheDocument()
    })
    expect(screen.getByText('สมชาย ใจดี')).toBeInTheDocument()
    expect(screen.getByText('สมหญิง รักสุข')).toBeInTheDocument()
    expect(screen.getByText('ห้อง 101')).toBeInTheDocument()
    expect(screen.getByText('ห้อง 201')).toBeInTheDocument()
  })

  test('shows empty state when no check-ins', async () => {
    global.fetch = mockFetch({
      '/api/stats': { data: mockStatsData, success: true },
      '/api/rooms/checkouts-today': { data: [], success: true },
      '/api/rooms': { data: mockRoomsData, success: true },
      '/api/checkins': { data: [], success: true },
      '/api/occupancy': { data: mockOccupancyData, success: true },
    })

    render(<Dashboard />)

    await waitFor(() => {
      expect(screen.getByText('ไม่มีกิจกรรมล่าสุด')).toBeInTheDocument()
    })
  })

  test('handles API errors gracefully', async () => {
    global.fetch = mockFetchError('Server error', 500)

    render(<Dashboard />)

    // Page should still render with zeroed stats
    await waitFor(() => {
      expect(screen.getByText('หน้าหลัก')).toBeInTheDocument()
    })
    // Stats should remain at 0 defaults
    expect(screen.getByText('ห้องที่มีผู้เข้าพัก')).toBeInTheDocument()
  })

  test('fetches all 5 API endpoints', async () => {
    setupSuccessfulFetch()

    render(<Dashboard />)

    await waitFor(() => {
      expect(screen.getByText('หน้าหลัก')).toBeInTheDocument()
    })

    const fetchCalls = (global.fetch as jest.Mock).mock.calls.map((c: [string]) => c[0])
    expect(fetchCalls).toContainEqual('/api/stats')
    expect(fetchCalls).toContainEqual('/api/rooms')
    expect(fetchCalls).toContainEqual('/api/rooms/checkouts-today')
    expect(fetchCalls).toContainEqual('/api/checkins?limit=10')
    expect(fetchCalls).toContainEqual('/api/occupancy?days=7')
  })
})
