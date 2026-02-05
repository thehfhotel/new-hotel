/**
 * @jest-environment jsdom
 */

import { render, screen } from '@testing-library/react'
import HousekeepingStats, { HousekeepingStatsData } from '@/components/housekeeping/HousekeepingStats'

// Mock lucide-react icons
jest.mock('lucide-react', () => ({
  Sparkles: () => <span data-testid="sparkles-icon">Sparkles</span>,
  Clock: () => <span data-testid="clock-icon">Clock</span>,
  CheckCircle: () => <span data-testid="check-circle-icon">CheckCircle</span>,
  Timer: () => <span data-testid="timer-icon">Timer</span>,
}))

describe('HousekeepingStats Component', () => {
  const mockStats: HousekeepingStatsData = {
    dirtyCount: 5,
    cleaningCount: 3,
    cleanedTodayCount: 12,
    avgCleaningTimeMinutes: 25,
  }

  describe('Rendering', () => {
    test('displays all 4 stat cards', () => {
      render(<HousekeepingStats stats={mockStats} />)

      // Check all Thai labels are present
      expect(screen.getByText('รอทำความสะอาด')).toBeInTheDocument()
      expect(screen.getByText('กำลังทำความสะอาด')).toBeInTheDocument()
      expect(screen.getByText('ทำความสะอาดแล้ววันนี้')).toBeInTheDocument()
      expect(screen.getByText('เวลาเฉลี่ยต่อห้อง')).toBeInTheDocument()
    })

    test('displays correct values from props', () => {
      render(<HousekeepingStats stats={mockStats} />)

      // Check values are displayed
      expect(screen.getByText('5')).toBeInTheDocument() // dirtyCount
      expect(screen.getByText('3')).toBeInTheDocument() // cleaningCount
      expect(screen.getByText('12')).toBeInTheDocument() // cleanedTodayCount
      expect(screen.getByText('25 นาที')).toBeInTheDocument() // avgCleaningTimeMinutes
    })

    test('displays dash when avgCleaningTimeMinutes is null', () => {
      const statsWithNullAvg: HousekeepingStatsData = {
        ...mockStats,
        avgCleaningTimeMinutes: null,
      }
      render(<HousekeepingStats stats={statsWithNullAvg} />)

      expect(screen.getByText('-')).toBeInTheDocument()
    })

    test('displays all icons', () => {
      render(<HousekeepingStats stats={mockStats} />)

      expect(screen.getByTestId('clock-icon')).toBeInTheDocument()
      expect(screen.getByTestId('sparkles-icon')).toBeInTheDocument()
      expect(screen.getByTestId('check-circle-icon')).toBeInTheDocument()
      expect(screen.getByTestId('timer-icon')).toBeInTheDocument()
    })
  })

  describe('Loading State', () => {
    test('shows loading placeholder when loading is true', () => {
      const { container } = render(<HousekeepingStats stats={mockStats} loading={true} />)

      // Check for loading skeleton elements (animate-pulse)
      const skeletons = container.querySelectorAll('.animate-pulse')
      expect(skeletons.length).toBe(4) // One for each stat card
    })

    test('does not show values when loading', () => {
      render(<HousekeepingStats stats={mockStats} loading={true} />)

      // Values should not be visible during loading
      expect(screen.queryByText('5')).not.toBeInTheDocument()
      expect(screen.queryByText('3')).not.toBeInTheDocument()
    })

    test('shows values when loading is false', () => {
      render(<HousekeepingStats stats={mockStats} loading={false} />)

      expect(screen.getByText('5')).toBeInTheDocument()
      expect(screen.getByText('3')).toBeInTheDocument()
    })
  })

  describe('Color Coding', () => {
    test('dirty count card has red background', () => {
      const { container } = render(<HousekeepingStats stats={mockStats} />)

      const redCard = container.querySelector('.bg-red-50')
      expect(redCard).toBeInTheDocument()
      expect(redCard).toHaveTextContent('รอทำความสะอาด')
    })

    test('cleaning count card has yellow background', () => {
      const { container } = render(<HousekeepingStats stats={mockStats} />)

      const yellowCard = container.querySelector('.bg-yellow-50')
      expect(yellowCard).toBeInTheDocument()
      expect(yellowCard).toHaveTextContent('กำลังทำความสะอาด')
    })

    test('cleaned today count card has green background', () => {
      const { container } = render(<HousekeepingStats stats={mockStats} />)

      const greenCard = container.querySelector('.bg-green-50')
      expect(greenCard).toBeInTheDocument()
      expect(greenCard).toHaveTextContent('ทำความสะอาดแล้ววันนี้')
    })

    test('average time card has blue background', () => {
      const { container } = render(<HousekeepingStats stats={mockStats} />)

      const blueCard = container.querySelector('.bg-blue-50')
      expect(blueCard).toBeInTheDocument()
      expect(blueCard).toHaveTextContent('เวลาเฉลี่ยต่อห้อง')
    })
  })

  describe('Edge Cases', () => {
    test('handles zero values correctly', () => {
      const zeroStats: HousekeepingStatsData = {
        dirtyCount: 0,
        cleaningCount: 0,
        cleanedTodayCount: 0,
        avgCleaningTimeMinutes: 0,
      }
      render(<HousekeepingStats stats={zeroStats} />)

      // Should display 0 values
      const zeros = screen.getAllByText('0')
      expect(zeros.length).toBe(3) // Three zeros for counts
      expect(screen.getByText('0 นาที')).toBeInTheDocument() // avg time
    })

    test('handles large numbers correctly', () => {
      const largeStats: HousekeepingStatsData = {
        dirtyCount: 999,
        cleaningCount: 888,
        cleanedTodayCount: 777,
        avgCleaningTimeMinutes: 120,
      }
      render(<HousekeepingStats stats={largeStats} />)

      expect(screen.getByText('999')).toBeInTheDocument()
      expect(screen.getByText('888')).toBeInTheDocument()
      expect(screen.getByText('777')).toBeInTheDocument()
      expect(screen.getByText('120 นาที')).toBeInTheDocument()
    })
  })
})
