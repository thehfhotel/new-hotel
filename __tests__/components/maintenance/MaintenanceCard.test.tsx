/**
 * @jest-environment jsdom
 */

import { render, screen, fireEvent, waitFor } from '@testing-library/react'
import MaintenanceCard from '@/components/maintenance/MaintenanceCard'
import { MaintenanceRequest } from '@/types/reports'

// Mock lucide-react icons
jest.mock('lucide-react', () => ({
  Clock: () => <span data-testid="clock-icon">Clock</span>,
  AlertTriangle: () => <span data-testid="alert-icon">AlertTriangle</span>,
  Play: () => <span data-testid="play-icon">Play</span>,
  CheckCircle: () => <span data-testid="check-icon">CheckCircle</span>,
  User: () => <span data-testid="user-icon">User</span>,
  Tag: () => <span data-testid="tag-icon">Tag</span>,
  DoorOpen: () => <span data-testid="door-icon">DoorOpen</span>,
  Edit2: () => <span data-testid="edit-icon">Edit</span>,
}))

describe('MaintenanceCard Component', () => {
  const mockOnStatusChange = jest.fn()
  const mockOnEdit = jest.fn()

  const baseRequest: MaintenanceRequest = {
    id: 1,
    requestNo: 'MR-2602-0001',
    roomId: 1,
    roomNo: '101',
    categoryId: 1,
    categoryName: 'ไฟฟ้า',
    title: 'แอร์ไม่เย็น',
    description: 'แอร์เปิดแล้วไม่เย็น',
    priority: 2,
    status: 'open',
    assignedTo: 'ช่างสมชาย',
    startedAt: null,
    completedAt: null,
    resolution: null,
    cost: null,
    createdAt: new Date().toISOString(),
    updatedAt: new Date().toISOString(),
  }

  beforeEach(() => {
    jest.clearAllMocks()
    mockOnStatusChange.mockResolvedValue(undefined)
  })

  describe('Rendering', () => {
    test('renders request number', () => {
      render(
        <MaintenanceCard
          request={baseRequest}
          onStatusChange={mockOnStatusChange}
          onEdit={mockOnEdit}
        />
      )

      expect(screen.getByText('MR-2602-0001')).toBeInTheDocument()
    })

    test('renders title', () => {
      render(
        <MaintenanceCard
          request={baseRequest}
          onStatusChange={mockOnStatusChange}
          onEdit={mockOnEdit}
        />
      )

      expect(screen.getByText('แอร์ไม่เย็น')).toBeInTheDocument()
    })

    test('renders room number', () => {
      render(
        <MaintenanceCard
          request={baseRequest}
          onStatusChange={mockOnStatusChange}
          onEdit={mockOnEdit}
        />
      )

      expect(screen.getByText('101')).toBeInTheDocument()
    })

    test('renders category name', () => {
      render(
        <MaintenanceCard
          request={baseRequest}
          onStatusChange={mockOnStatusChange}
          onEdit={mockOnEdit}
        />
      )

      expect(screen.getByText('ไฟฟ้า')).toBeInTheDocument()
    })

    test('renders assigned person', () => {
      render(
        <MaintenanceCard
          request={baseRequest}
          onStatusChange={mockOnStatusChange}
          onEdit={mockOnEdit}
        />
      )

      expect(screen.getByText('ช่างสมชาย')).toBeInTheDocument()
    })

    test('renders description', () => {
      render(
        <MaintenanceCard
          request={baseRequest}
          onStatusChange={mockOnStatusChange}
          onEdit={mockOnEdit}
        />
      )

      expect(screen.getByText('แอร์เปิดแล้วไม่เย็น')).toBeInTheDocument()
    })
  })

  describe('Priority Display', () => {
    test('displays high priority badge', () => {
      const highPriorityRequest = { ...baseRequest, priority: 3 as const }
      render(
        <MaintenanceCard
          request={highPriorityRequest}
          onStatusChange={mockOnStatusChange}
          onEdit={mockOnEdit}
        />
      )

      expect(screen.getByText('ด่วน')).toBeInTheDocument()
    })

    test('displays medium priority badge', () => {
      render(
        <MaintenanceCard
          request={baseRequest}
          onStatusChange={mockOnStatusChange}
          onEdit={mockOnEdit}
        />
      )

      expect(screen.getByText('ปานกลาง')).toBeInTheDocument()
    })

    test('displays low priority badge', () => {
      const lowPriorityRequest = { ...baseRequest, priority: 1 as const }
      render(
        <MaintenanceCard
          request={lowPriorityRequest}
          onStatusChange={mockOnStatusChange}
          onEdit={mockOnEdit}
        />
      )

      expect(screen.getByText('ต่ำ')).toBeInTheDocument()
    })
  })

  describe('Status-based Action Buttons', () => {
    test('shows "เริ่มซ่อม" button for open status', () => {
      render(
        <MaintenanceCard
          request={baseRequest}
          onStatusChange={mockOnStatusChange}
          onEdit={mockOnEdit}
        />
      )

      expect(screen.getByText('เริ่มซ่อม')).toBeInTheDocument()
    })

    test('shows "ซ่อมเสร็จ" button for in_progress status', () => {
      const inProgressRequest = {
        ...baseRequest,
        status: 'in_progress' as const,
        startedAt: new Date().toISOString(),
      }
      render(
        <MaintenanceCard
          request={inProgressRequest}
          onStatusChange={mockOnStatusChange}
          onEdit={mockOnEdit}
        />
      )

      expect(screen.getByText('ซ่อมเสร็จ')).toBeInTheDocument()
    })

    test('shows completed message for completed status', () => {
      const completedRequest = {
        ...baseRequest,
        status: 'completed' as const,
        completedAt: new Date().toISOString(),
      }
      render(
        <MaintenanceCard
          request={completedRequest}
          onStatusChange={mockOnStatusChange}
          onEdit={mockOnEdit}
        />
      )

      expect(screen.getByText('ซ่อมเรียบร้อยแล้ว')).toBeInTheDocument()
    })
  })

  describe('Status Change', () => {
    test('clicking "เริ่มซ่อม" calls onStatusChange with in_progress', async () => {
      render(
        <MaintenanceCard
          request={baseRequest}
          onStatusChange={mockOnStatusChange}
          onEdit={mockOnEdit}
        />
      )

      fireEvent.click(screen.getByText('เริ่มซ่อม'))

      await waitFor(() => {
        expect(mockOnStatusChange).toHaveBeenCalledWith(1, 'in_progress')
      })
    })

    test('clicking "ซ่อมเสร็จ" calls onStatusChange with completed', async () => {
      const inProgressRequest = {
        ...baseRequest,
        status: 'in_progress' as const,
        startedAt: new Date().toISOString(),
      }
      render(
        <MaintenanceCard
          request={inProgressRequest}
          onStatusChange={mockOnStatusChange}
          onEdit={mockOnEdit}
        />
      )

      fireEvent.click(screen.getByText('ซ่อมเสร็จ'))

      await waitFor(() => {
        expect(mockOnStatusChange).toHaveBeenCalledWith(1, 'completed')
      })
    })
  })

  describe('Edit Button', () => {
    test('clicking edit button calls onEdit', () => {
      render(
        <MaintenanceCard
          request={baseRequest}
          onStatusChange={mockOnStatusChange}
          onEdit={mockOnEdit}
        />
      )

      const editButton = screen.getByTestId('edit-icon').closest('button')
      fireEvent.click(editButton!)

      expect(mockOnEdit).toHaveBeenCalledWith(baseRequest)
    })
  })

  describe('Disabled State', () => {
    test('action button is disabled when disabled prop is true', () => {
      render(
        <MaintenanceCard
          request={baseRequest}
          onStatusChange={mockOnStatusChange}
          onEdit={mockOnEdit}
          disabled={true}
        />
      )

      const startButton = screen.getByText('เริ่มซ่อม').closest('button')
      expect(startButton).toBeDisabled()
    })

    test('edit button is disabled when disabled prop is true', () => {
      render(
        <MaintenanceCard
          request={baseRequest}
          onStatusChange={mockOnStatusChange}
          onEdit={mockOnEdit}
          disabled={true}
        />
      )

      const editButton = screen.getByTestId('edit-icon').closest('button')
      expect(editButton).toBeDisabled()
    })
  })

  describe('Overdue Indicator', () => {
    test('shows overdue badge when open and older than 2 hours', () => {
      const oldRequest = {
        ...baseRequest,
        createdAt: new Date(Date.now() - 3 * 60 * 60 * 1000).toISOString(), // 3 hours ago
      }
      render(
        <MaintenanceCard
          request={oldRequest}
          onStatusChange={mockOnStatusChange}
          onEdit={mockOnEdit}
        />
      )

      expect(screen.getByText('ล่าช้า')).toBeInTheDocument()
    })

    test('does not show overdue badge when status is not open', () => {
      const oldCompletedRequest = {
        ...baseRequest,
        status: 'completed' as const,
        createdAt: new Date(Date.now() - 3 * 60 * 60 * 1000).toISOString(),
      }
      render(
        <MaintenanceCard
          request={oldCompletedRequest}
          onStatusChange={mockOnStatusChange}
          onEdit={mockOnEdit}
        />
      )

      expect(screen.queryByText('ล่าช้า')).not.toBeInTheDocument()
    })
  })

  describe('Completed Request Display', () => {
    test('shows cost when completed and cost is set', () => {
      const completedWithCost = {
        ...baseRequest,
        status: 'completed' as const,
        cost: 1500,
      }
      render(
        <MaintenanceCard
          request={completedWithCost}
          onStatusChange={mockOnStatusChange}
          onEdit={mockOnEdit}
        />
      )

      expect(screen.getByText(/ค่าใช้จ่าย.*1,500 บาท/)).toBeInTheDocument()
    })

    test('shows resolution when completed and resolution is set', () => {
      const completedWithResolution = {
        ...baseRequest,
        status: 'completed' as const,
        resolution: 'เปลี่ยนคอมเพรสเซอร์แล้ว',
      }
      render(
        <MaintenanceCard
          request={completedWithResolution}
          onStatusChange={mockOnStatusChange}
          onEdit={mockOnEdit}
        />
      )

      expect(screen.getByText('เปลี่ยนคอมเพรสเซอร์แล้ว')).toBeInTheDocument()
    })
  })
})
