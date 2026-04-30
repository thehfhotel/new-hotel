/**
 * @jest-environment jsdom
 */

import { render, screen, fireEvent, waitFor } from '@testing-library/react'
import MaintenanceRequestModal from '@/components/modals/MaintenanceRequestModal'
import { MaintenanceRequest, MaintenanceCategory } from '@/types/reports'

// Mock lucide-react icons
jest.mock('lucide-react', () => ({
  X: () => <span data-testid="x-icon">X</span>,
  Loader2: () => <span data-testid="loader-icon">Loader</span>,
  AlertCircle: () => <span data-testid="alert-icon">AlertCircle</span>,
  Wrench: () => <span data-testid="wrench-icon">Wrench</span>,
  DoorOpen: () => <span data-testid="door-icon">DoorOpen</span>,
  Tag: () => <span data-testid="tag-icon">Tag</span>,
  FileText: () => <span data-testid="file-text-icon">FileText</span>,
  User: () => <span data-testid="user-icon">User</span>,
  DollarSign: () => <span data-testid="dollar-icon">DollarSign</span>,
}))

describe('MaintenanceRequestModal Component', () => {
  const mockOnClose = jest.fn()
  const mockOnSuccess = jest.fn()

  const mockCategories: MaintenanceCategory[] = [
    { id: 1, name: 'ไฟฟ้า', nameEn: 'Electrical', priority: 3, active: true },
    { id: 2, name: 'ประปา', nameEn: 'Plumbing', priority: 3, active: true },
    { id: 3, name: 'ทั่วไป', nameEn: 'General', priority: 2, active: true },
  ]

  const mockRooms = [
    { id: 1, roomNo: '101', roomTypeName: 'Standard' },
    { id: 2, roomNo: '102', roomTypeName: 'Deluxe' },
    { id: 3, roomNo: '201', roomTypeName: 'Suite' },
  ]

  const mockRequest: MaintenanceRequest = {
    id: 1,
    requestNo: 'MR-2602-0001',
    roomId: 1,
    roomNo: '101',
    categoryId: 1,
    categoryName: 'ไฟฟ้า',
    title: 'แอร์ไม่เย็น',
    description: 'แอร์เปิดแล้วไม่เย็น ลมออกแรงปกติ',
    priority: 3,
    status: 'open',
    assignedTo: 'ช่างสมชาย',
    startedAt: null,
    completedAt: null,
    resolution: null,
    cost: null,
    createdAt: '2026-02-01T10:00:00.000Z',
    updatedAt: '2026-02-01T10:00:00.000Z',
  }

  const defaultProps = {
    isOpen: true,
    onClose: mockOnClose,
    onSuccess: mockOnSuccess,
    categories: mockCategories,
    rooms: mockRooms,
  }

  // Helper to get room and category selects (first two comboboxes)
  const getSelects = () => {
    const selects = screen.getAllByRole('combobox')
    return { roomSelect: selects[0], categorySelect: selects[1] }
  }

  beforeEach(() => {
    jest.clearAllMocks()
    global.fetch = jest.fn()
  })

  describe('Rendering', () => {
    test('does not render when isOpen is false', () => {
      render(<MaintenanceRequestModal {...defaultProps} isOpen={false} />)

      expect(screen.queryByText('แจ้งซ่อมใหม่')).not.toBeInTheDocument()
    })

    test('renders create mode when no editRequest', () => {
      render(<MaintenanceRequestModal {...defaultProps} />)

      expect(screen.getByText('แจ้งซ่อมใหม่')).toBeInTheDocument()
    })

    test('renders edit mode when editRequest is provided', () => {
      render(<MaintenanceRequestModal {...defaultProps} editRequest={mockRequest} />)

      expect(screen.getByText('แก้ไขคำขอซ่อม')).toBeInTheDocument()
      expect(screen.getByText('MR-2602-0001')).toBeInTheDocument()
    })

    test('displays room dropdown with options', () => {
      render(<MaintenanceRequestModal {...defaultProps} />)

      expect(screen.getByText('เลือกห้อง')).toBeInTheDocument()
      expect(screen.getByText('101 - Standard')).toBeInTheDocument()
      expect(screen.getByText('102 - Deluxe')).toBeInTheDocument()
      expect(screen.getByText('201 - Suite')).toBeInTheDocument()
    })

    test('displays category dropdown with options', () => {
      render(<MaintenanceRequestModal {...defaultProps} />)

      expect(screen.getByText('เลือกหมวดหมู่')).toBeInTheDocument()
      expect(screen.getByText('ไฟฟ้า (Electrical)')).toBeInTheDocument()
      expect(screen.getByText('ประปา (Plumbing)')).toBeInTheDocument()
      expect(screen.getByText('ทั่วไป (General)')).toBeInTheDocument()
    })

    test('displays priority buttons', () => {
      render(<MaintenanceRequestModal {...defaultProps} />)

      expect(screen.getByText('สูง (ด่วน)')).toBeInTheDocument()
      expect(screen.getByText('ปานกลาง')).toBeInTheDocument()
      expect(screen.getByText('ต่ำ')).toBeInTheDocument()
    })
  })

  describe('Edit Mode', () => {
    test('pre-populates form with editRequest data', () => {
      render(<MaintenanceRequestModal {...defaultProps} editRequest={mockRequest} />)

      // Check title
      const titleInput = screen.getByPlaceholderText('เช่น แอร์ไม่เย็น, ก๊อกน้ำรั่ว...')
      expect(titleInput).toHaveValue('แอร์ไม่เย็น')

      // Check description
      const descInput = screen.getByPlaceholderText('รายละเอียดเพิ่มเติม...')
      expect(descInput).toHaveValue('แอร์เปิดแล้วไม่เย็น ลมออกแรงปกติ')

      // Check assigned to
      const assignedInput = screen.getByPlaceholderText('ชื่อช่างหรือผู้รับผิดชอบ')
      expect(assignedInput).toHaveValue('ช่างสมชาย')

      // Check priority (high is selected)
      const highPriorityBtn = screen.getByText('สูง (ด่วน)').closest('button')
      expect(highPriorityBtn).toHaveClass('border-red-500')
    })

    test('shows resolution and cost fields only in edit mode', () => {
      render(<MaintenanceRequestModal {...defaultProps} editRequest={mockRequest} />)

      expect(screen.getByText('ผลการซ่อม')).toBeInTheDocument()
      expect(screen.getByText('ค่าใช้จ่าย (บาท)')).toBeInTheDocument()
    })

    test('does not show resolution and cost fields in create mode', () => {
      render(<MaintenanceRequestModal {...defaultProps} />)

      expect(screen.queryByText('ผลการซ่อม')).not.toBeInTheDocument()
      expect(screen.queryByText('ค่าใช้จ่าย (บาท)')).not.toBeInTheDocument()
    })

    test('room and category dropdowns are disabled in edit mode', () => {
      render(<MaintenanceRequestModal {...defaultProps} editRequest={mockRequest} />)

      // Get select elements by their displayed selected option text
      const allSelects = screen.getAllByRole('combobox')
      const roomSelect = allSelects[0] // First select is room
      const categorySelect = allSelects[1] // Second select is category

      expect(roomSelect).toBeDisabled()
      expect(categorySelect).toBeDisabled()
    })
  })

  describe('Priority Selection', () => {
    test('medium priority is selected by default in create mode', () => {
      render(<MaintenanceRequestModal {...defaultProps} />)

      const mediumBtn = screen.getByText('ปานกลาง').closest('button')
      expect(mediumBtn).toHaveClass('border-amber-500')
    })

    test('clicking priority changes selection', () => {
      render(<MaintenanceRequestModal {...defaultProps} />)

      fireEvent.click(screen.getByText('สูง (ด่วน)'))

      const highBtn = screen.getByText('สูง (ด่วน)').closest('button')
      expect(highBtn).toHaveClass('border-red-500')
    })
  })

  describe('Form Validation', () => {
    test('room select is required', () => {
      render(<MaintenanceRequestModal {...defaultProps} />)

      const { roomSelect } = getSelects()
      expect(roomSelect).toBeRequired()
    })

    test('category select is required', () => {
      render(<MaintenanceRequestModal {...defaultProps} />)

      const { categorySelect } = getSelects()
      expect(categorySelect).toBeRequired()
    })

    test('title input is required', () => {
      render(<MaintenanceRequestModal {...defaultProps} />)

      const titleInput = screen.getByPlaceholderText('เช่น แอร์ไม่เย็น, ก๊อกน้ำรั่ว...')
      expect(titleInput).toBeRequired()
    })

    test('can successfully submit when all required fields are filled', async () => {
      ;(global.fetch as jest.Mock).mockResolvedValue({
        ok: true,
        json: () => Promise.resolve({ success: true }),
      })

      render(<MaintenanceRequestModal {...defaultProps} />)

      // Fill all required fields
      const { roomSelect, categorySelect } = getSelects()
      fireEvent.change(roomSelect, { target: { value: '1' } })
      fireEvent.change(categorySelect, { target: { value: '1' } })
      fireEvent.change(screen.getByPlaceholderText('เช่น แอร์ไม่เย็น, ก๊อกน้ำรั่ว...'), { target: { value: 'ทดสอบ' } })

      const submitButton = screen.getByText('แจ้งซ่อม')
      fireEvent.click(submitButton)

      await waitFor(() => {
        expect(global.fetch).toHaveBeenCalled()
      })
    })
  })

  describe('API Submission - Create', () => {
    test('submits correct payload for new request', async () => {
      ;(global.fetch as jest.Mock).mockResolvedValue({
        ok: true,
        json: () => Promise.resolve({ success: true }),
      })

      render(<MaintenanceRequestModal {...defaultProps} />)

      // Fill form
      const { roomSelect, categorySelect } = getSelects()
      fireEvent.change(roomSelect, { target: { value: '1' } })
      fireEvent.change(categorySelect, { target: { value: '2' } })
      fireEvent.change(screen.getByPlaceholderText('เช่น แอร์ไม่เย็น, ก๊อกน้ำรั่ว...'), { target: { value: 'ก๊อกน้ำรั่ว' } })
      fireEvent.change(screen.getByPlaceholderText('รายละเอียดเพิ่มเติม...'), { target: { value: 'รั่วเบาๆ' } })
      fireEvent.click(screen.getByText('สูง (ด่วน)'))
      fireEvent.change(screen.getByPlaceholderText('ชื่อช่างหรือผู้รับผิดชอบ'), { target: { value: 'ช่างแดง' } })

      fireEvent.click(screen.getByText('แจ้งซ่อม'))

      await waitFor(() => {
        expect(global.fetch).toHaveBeenCalledWith('/api/new/maintenance/requests?branch=hfhotel', {
          method: 'POST',
          headers: { 'Content-Type': 'application/json' },
          body: JSON.stringify({
            roomId: 1,
            categoryId: 2,
            title: 'ก๊อกน้ำรั่ว',
            description: 'รั่วเบาๆ',
            priority: 3,
            assignedTo: 'ช่างแดง',
          }),
        })
      })
    })

    test('calls onSuccess and onClose after successful creation', async () => {
      ;(global.fetch as jest.Mock).mockResolvedValue({
        ok: true,
        json: () => Promise.resolve({ success: true }),
      })

      render(<MaintenanceRequestModal {...defaultProps} />)

      // Fill required fields
      const { roomSelect, categorySelect } = getSelects()
      fireEvent.change(roomSelect, { target: { value: '1' } })
      fireEvent.change(categorySelect, { target: { value: '1' } })
      fireEvent.change(screen.getByPlaceholderText('เช่น แอร์ไม่เย็น, ก๊อกน้ำรั่ว...'), { target: { value: 'ทดสอบ' } })

      fireEvent.click(screen.getByText('แจ้งซ่อม'))

      await waitFor(() => {
        expect(mockOnSuccess).toHaveBeenCalled()
        expect(mockOnClose).toHaveBeenCalled()
      })
    })
  })

  describe('API Submission - Update', () => {
    test('submits correct payload for update', async () => {
      ;(global.fetch as jest.Mock).mockResolvedValue({
        ok: true,
        json: () => Promise.resolve({ success: true }),
      })

      render(<MaintenanceRequestModal {...defaultProps} editRequest={mockRequest} />)

      // Modify title
      const titleInput = screen.getByPlaceholderText('เช่น แอร์ไม่เย็น, ก๊อกน้ำรั่ว...')
      fireEvent.change(titleInput, { target: { value: 'แอร์ไม่เย็นและมีเสียงดัง' } })

      // Add resolution
      const resolutionInput = screen.getByPlaceholderText('รายละเอียดผลการซ่อม...')
      fireEvent.change(resolutionInput, { target: { value: 'เปลี่ยนน้ำยาแอร์แล้ว' } })

      // Add cost
      const costInput = screen.getByPlaceholderText('0.00')
      fireEvent.change(costInput, { target: { value: '500' } })

      fireEvent.click(screen.getByText('บันทึกการแก้ไข'))

      await waitFor(() => {
        expect(global.fetch).toHaveBeenCalledWith('/api/new/maintenance/requests/1?branch=hfhotel', {
          method: 'PUT',
          headers: { 'Content-Type': 'application/json' },
          body: expect.stringContaining('แอร์ไม่เย็นและมีเสียงดัง'),
        })
      })
    })

    test('does not make API call when no changes', async () => {
      render(<MaintenanceRequestModal {...defaultProps} editRequest={mockRequest} />)

      fireEvent.click(screen.getByText('บันทึกการแก้ไข'))

      await waitFor(() => {
        expect(mockOnSuccess).toHaveBeenCalled()
        expect(mockOnClose).toHaveBeenCalled()
      })

      // Fetch should not be called
      expect(global.fetch).not.toHaveBeenCalled()
    })
  })

  describe('Error Handling', () => {
    test('shows error message on API failure', async () => {
      ;(global.fetch as jest.Mock).mockResolvedValue({
        ok: false,
        json: () => Promise.resolve({ success: false, error: 'Failed to create request' }),
      })

      render(<MaintenanceRequestModal {...defaultProps} />)

      // Fill required fields
      const { roomSelect, categorySelect } = getSelects()
      fireEvent.change(roomSelect, { target: { value: '1' } })
      fireEvent.change(categorySelect, { target: { value: '1' } })
      fireEvent.change(screen.getByPlaceholderText('เช่น แอร์ไม่เย็น, ก๊อกน้ำรั่ว...'), { target: { value: 'ทดสอบ' } })

      fireEvent.click(screen.getByText('แจ้งซ่อม'))

      await waitFor(() => {
        expect(screen.getByText('Failed to create request')).toBeInTheDocument()
      })
    })
  })

  describe('Modal Interactions', () => {
    test('closes modal when close button is clicked', () => {
      render(<MaintenanceRequestModal {...defaultProps} />)

      const closeButton = screen.getByTestId('x-icon').closest('button')
      fireEvent.click(closeButton!)

      expect(mockOnClose).toHaveBeenCalled()
    })

    test('closes modal when cancel button is clicked', () => {
      render(<MaintenanceRequestModal {...defaultProps} />)

      const cancelButton = screen.getByText('ยกเลิก')
      fireEvent.click(cancelButton)

      expect(mockOnClose).toHaveBeenCalled()
    })

    test('closes modal when clicking backdrop', () => {
      render(<MaintenanceRequestModal {...defaultProps} />)

      const backdrop = screen.getByText('แจ้งซ่อมใหม่').closest('.fixed')
      fireEvent.click(backdrop!)

      expect(mockOnClose).toHaveBeenCalled()
    })
  })
})
