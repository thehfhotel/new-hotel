/**
 * @jest-environment jsdom
 */

import { render, screen } from '@testing-library/react'
import { OccupancyChart, RevenueChart, PieChart } from '@/components/Charts'

// Mock recharts to avoid rendering issues in jsdom
jest.mock('recharts', () => ({
  ResponsiveContainer: ({ children }: { children: React.ReactNode }) => (
    <div data-testid="responsive-container">{children}</div>
  ),
  LineChart: ({ children }: { children: React.ReactNode }) => (
    <div data-testid="line-chart">{children}</div>
  ),
  BarChart: ({ children }: { children: React.ReactNode }) => (
    <div data-testid="bar-chart">{children}</div>
  ),
  PieChart: ({ children }: { children: React.ReactNode }) => (
    <div data-testid="pie-chart">{children}</div>
  ),
  Line: () => <div data-testid="line" />,
  Bar: () => <div data-testid="bar" />,
  Pie: () => <div data-testid="pie" />,
  Cell: () => <div data-testid="cell" />,
  XAxis: () => <div data-testid="x-axis" />,
  YAxis: () => <div data-testid="y-axis" />,
  CartesianGrid: () => <div data-testid="cartesian-grid" />,
  Tooltip: () => <div data-testid="tooltip" />,
  Legend: () => <div data-testid="legend" />,
}))

describe('OccupancyChart Component', () => {
  const mockData = [
    { date: '01 ม.ค.', occupiedRooms: 10 },
    { date: '02 ม.ค.', occupiedRooms: 12 },
    { date: '03 ม.ค.', occupiedRooms: 8 },
  ]

  test('renders with default title', () => {
    render(<OccupancyChart data={mockData} />)

    expect(screen.getByText('จำนวนห้องที่มีผู้เข้าพัก')).toBeInTheDocument()
  })

  test('renders with custom title', () => {
    render(<OccupancyChart data={mockData} title="Custom Title" />)

    expect(screen.getByText('Custom Title')).toBeInTheDocument()
  })

  test('renders bar chart by default', () => {
    render(<OccupancyChart data={mockData} />)

    expect(screen.getByTestId('bar-chart')).toBeInTheDocument()
  })

  test('renders line chart when chartType is line', () => {
    render(<OccupancyChart data={mockData} chartType="line" />)

    expect(screen.getByTestId('line-chart')).toBeInTheDocument()
  })

  test('renders in a container with proper styling', () => {
    render(<OccupancyChart data={mockData} />)

    const container = screen.getByTestId('responsive-container').closest('.bg-white')
    expect(container).toHaveClass('rounded-xl', 'shadow-sm')
  })
})

describe('RevenueChart Component', () => {
  const mockData = [
    { period: 'ม.ค.', revenue: 50000, bookings: 10 },
    { period: 'ก.พ.', revenue: 75000, bookings: 15 },
    { period: 'มี.ค.', revenue: 60000, bookings: 12 },
  ]

  test('renders with default title', () => {
    render(<RevenueChart data={mockData} />)

    expect(screen.getByText('รายได้')).toBeInTheDocument()
  })

  test('renders with custom title', () => {
    render(<RevenueChart data={mockData} title="Revenue Trend" />)

    expect(screen.getByText('Revenue Trend')).toBeInTheDocument()
  })

  test('renders bar chart by default', () => {
    render(<RevenueChart data={mockData} />)

    expect(screen.getByTestId('bar-chart')).toBeInTheDocument()
  })

  test('renders line chart when chartType is line', () => {
    render(<RevenueChart data={mockData} chartType="line" />)

    expect(screen.getByTestId('line-chart')).toBeInTheDocument()
  })

  test('handles empty data', () => {
    render(<RevenueChart data={[]} />)

    expect(screen.getByText('รายได้')).toBeInTheDocument()
    expect(screen.getByTestId('bar-chart')).toBeInTheDocument()
  })
})

describe('PieChart Component', () => {
  const mockData = [
    { roomType: 'Standard', revenue: 30000, percentage: 40 },
    { roomType: 'Deluxe', revenue: 25000, percentage: 33 },
    { roomType: 'Suite', revenue: 20000, percentage: 27 },
  ]

  test('renders with default title', () => {
    render(<PieChart data={mockData} />)

    expect(screen.getByText('รายได้ตามประเภทห้อง')).toBeInTheDocument()
  })

  test('renders with custom title', () => {
    render(<PieChart data={mockData} title="Revenue by Room Type" />)

    expect(screen.getByText('Revenue by Room Type')).toBeInTheDocument()
  })

  test('renders pie chart', () => {
    render(<PieChart data={mockData} />)

    expect(screen.getByTestId('pie-chart')).toBeInTheDocument()
  })

  test('handles empty data', () => {
    render(<PieChart data={[]} />)

    expect(screen.getByText('รายได้ตามประเภทห้อง')).toBeInTheDocument()
    expect(screen.getByTestId('pie-chart')).toBeInTheDocument()
  })

  test('renders in a container with proper styling', () => {
    render(<PieChart data={mockData} />)

    const container = screen.getByTestId('responsive-container').closest('.bg-white')
    expect(container).toHaveClass('rounded-xl', 'shadow-sm')
  })
})
