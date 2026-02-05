/**
 * @jest-environment jsdom
 */

import { render, screen, fireEvent, waitFor } from '@testing-library/react'
import RoomTypeForm, { RoomTypeFormData } from '@/components/forms/RoomTypeForm'

// Mock lucide-react icons
jest.mock('lucide-react', () => ({
  X: () => <span data-testid="icon-x">X</span>,
  Home: () => <span data-testid="icon-home">Home</span>,
  Loader2: () => <span data-testid="icon-loader">Loading</span>,
  AlertCircle: () => <span data-testid="icon-alert">Alert</span>,
  Save: () => <span data-testid="icon-save">Save</span>,
  Trash2: () => <span data-testid="icon-trash">Trash</span>,
  Hash: () => <span data-testid="icon-hash">Hash</span>,
  DollarSign: () => <span data-testid="icon-dollar">Dollar</span>,
  Users: () => <span data-testid="icon-users">Users</span>,
  Bed: () => <span data-testid="icon-bed">Bed</span>,
  Maximize: () => <span data-testid="icon-maximize">Maximize</span>,
  FileText: () => <span data-testid="icon-file-text">FileText</span>,
  Globe: () => <span data-testid="icon-globe">Globe</span>,
}))

const mockRoomTypeData: RoomTypeFormData = {
  id: 1,
  typeCode: 'DLX',
  typeName: 'ห้องดีลักซ์',
  typeNameEn: 'Deluxe Room',
  description: 'ห้องพักขนาดใหญ่พร้อมวิวสวน',
  basePrice: 2500,
  maxGuests: 3,
  bedType: 'King',
  sizeSqm: 35,
  active: true,
}

describe('RoomTypeForm Component', () => {
  const mockOnClose = jest.fn()
  const mockOnSave = jest.fn()
  const mockOnDelete = jest.fn()

  beforeEach(() => {
    jest.clearAllMocks()
  })

  describe('Modal Behavior', () => {
    test('does not render when isOpen is false', () => {
      render(
        <RoomTypeForm
          isOpen={false}
          onClose={mockOnClose}
          onSave={mockOnSave}
          mode="create"
        />
      )

      expect(screen.queryByText('เพิ่มประเภทห้องใหม่')).not.toBeInTheDocument()
    })

    test('renders when isOpen is true', () => {
      render(
        <RoomTypeForm
          isOpen={true}
          onClose={mockOnClose}
          onSave={mockOnSave}
          mode="create"
        />
      )

      expect(screen.getByText('เพิ่มประเภทห้องใหม่')).toBeInTheDocument()
    })

    test('calls onClose when close button is clicked', () => {
      render(
        <RoomTypeForm
          isOpen={true}
          onClose={mockOnClose}
          onSave={mockOnSave}
          mode="create"
        />
      )

      const closeButton = screen.getByLabelText('ปิด')
      fireEvent.click(closeButton)

      expect(mockOnClose).toHaveBeenCalledTimes(1)
    })

    test('calls onClose when cancel button is clicked', () => {
      render(
        <RoomTypeForm
          isOpen={true}
          onClose={mockOnClose}
          onSave={mockOnSave}
          mode="create"
        />
      )

      const cancelButton = screen.getByText('ยกเลิก')
      fireEvent.click(cancelButton)

      expect(mockOnClose).toHaveBeenCalledTimes(1)
    })
  })

  describe('Create Mode', () => {
    test('displays create mode title', () => {
      render(
        <RoomTypeForm
          isOpen={true}
          onClose={mockOnClose}
          onSave={mockOnSave}
          mode="create"
        />
      )

      expect(screen.getByText('เพิ่มประเภทห้องใหม่')).toBeInTheDocument()
    })

    test('displays create button text', () => {
      render(
        <RoomTypeForm
          isOpen={true}
          onClose={mockOnClose}
          onSave={mockOnSave}
          mode="create"
        />
      )

      expect(screen.getByText('เพิ่มประเภทห้อง')).toBeInTheDocument()
    })

    test('does not show delete button in create mode', () => {
      render(
        <RoomTypeForm
          isOpen={true}
          onClose={mockOnClose}
          onSave={mockOnSave}
          onDelete={mockOnDelete}
          mode="create"
        />
      )

      expect(screen.queryByText('ลบ')).not.toBeInTheDocument()
    })
  })

  describe('Edit Mode', () => {
    test('displays edit mode title', () => {
      render(
        <RoomTypeForm
          isOpen={true}
          onClose={mockOnClose}
          onSave={mockOnSave}
          initialData={mockRoomTypeData}
          mode="edit"
        />
      )

      expect(screen.getByText('แก้ไขประเภทห้อง')).toBeInTheDocument()
    })

    test('displays save button text', () => {
      render(
        <RoomTypeForm
          isOpen={true}
          onClose={mockOnClose}
          onSave={mockOnSave}
          initialData={mockRoomTypeData}
          mode="edit"
        />
      )

      expect(screen.getByText('บันทึก')).toBeInTheDocument()
    })

    test('populates form with initial data', () => {
      render(
        <RoomTypeForm
          isOpen={true}
          onClose={mockOnClose}
          onSave={mockOnSave}
          initialData={mockRoomTypeData}
          mode="edit"
        />
      )

      expect(screen.getByDisplayValue('DLX')).toBeInTheDocument()
      expect(screen.getByDisplayValue('ห้องดีลักซ์')).toBeInTheDocument()
      expect(screen.getByDisplayValue('Deluxe Room')).toBeInTheDocument()
      expect(screen.getByDisplayValue('ห้องพักขนาดใหญ่พร้อมวิวสวน')).toBeInTheDocument()
      expect(screen.getByDisplayValue('2500')).toBeInTheDocument()
      expect(screen.getByDisplayValue('3')).toBeInTheDocument()
      expect(screen.getByDisplayValue('35')).toBeInTheDocument()
    })

    test('shows delete button in edit mode with onDelete', () => {
      render(
        <RoomTypeForm
          isOpen={true}
          onClose={mockOnClose}
          onSave={mockOnSave}
          onDelete={mockOnDelete}
          initialData={mockRoomTypeData}
          mode="edit"
        />
      )

      expect(screen.getByText('ลบ')).toBeInTheDocument()
    })
  })

  describe('Form Field Rendering', () => {
    test('renders all form fields', () => {
      render(
        <RoomTypeForm
          isOpen={true}
          onClose={mockOnClose}
          onSave={mockOnSave}
          mode="create"
        />
      )

      expect(screen.getByPlaceholderText('เช่น STD, DLX, SUI')).toBeInTheDocument()
      expect(screen.getByPlaceholderText('กรอกชื่อประเภทห้อง')).toBeInTheDocument()
      expect(screen.getByPlaceholderText('Enter English name')).toBeInTheDocument()
      expect(screen.getByPlaceholderText('กรอกรายละเอียดประเภทห้อง')).toBeInTheDocument()
    })

    test('renders bed type dropdown with options', () => {
      render(
        <RoomTypeForm
          isOpen={true}
          onClose={mockOnClose}
          onSave={mockOnSave}
          mode="create"
        />
      )

      // Find the bed type select by its name attribute
      const bedTypeSelect = document.querySelector('select[name="bedType"]')
      expect(bedTypeSelect).toBeInTheDocument()

      expect(screen.getByText('-- เลือกประเภทเตียง --')).toBeInTheDocument()
      expect(screen.getByText('Single (เตียงเดี่ยว)')).toBeInTheDocument()
      expect(screen.getByText('Double (เตียงคู่)')).toBeInTheDocument()
      expect(screen.getByText('Twin (เตียงแฝด)')).toBeInTheDocument()
      expect(screen.getByText('King (เตียงคิงไซส์)')).toBeInTheDocument()
    })

    test('renders active toggle', () => {
      render(
        <RoomTypeForm
          isOpen={true}
          onClose={mockOnClose}
          onSave={mockOnSave}
          mode="create"
        />
      )

      expect(screen.getByText('เปิดใช้งาน')).toBeInTheDocument()
      const activeCheckbox = screen.getByRole('checkbox')
      expect(activeCheckbox).toBeChecked() // Default is true
    })
  })

  describe('Form Input Handling', () => {
    test('updates text inputs', () => {
      render(
        <RoomTypeForm
          isOpen={true}
          onClose={mockOnClose}
          onSave={mockOnSave}
          mode="create"
        />
      )

      const typeCodeInput = screen.getByPlaceholderText('เช่น STD, DLX, SUI')
      fireEvent.change(typeCodeInput, { target: { value: 'SUP', name: 'typeCode' } })
      expect(typeCodeInput).toHaveValue('SUP')

      const typeNameInput = screen.getByPlaceholderText('กรอกชื่อประเภทห้อง')
      fireEvent.change(typeNameInput, { target: { value: 'ห้องสูพีเรียร์', name: 'typeName' } })
      expect(typeNameInput).toHaveValue('ห้องสูพีเรียร์')
    })

    test('updates numeric inputs', () => {
      render(
        <RoomTypeForm
          isOpen={true}
          onClose={mockOnClose}
          onSave={mockOnSave}
          mode="create"
        />
      )

      const basePriceInput = document.querySelector('input[name="basePrice"]') as HTMLInputElement
      fireEvent.change(basePriceInput, { target: { value: '3000', name: 'basePrice', type: 'number' } })
      expect(basePriceInput).toHaveValue(3000)

      const maxGuestsInput = document.querySelector('input[name="maxGuests"]') as HTMLInputElement
      fireEvent.change(maxGuestsInput, { target: { value: '4', name: 'maxGuests', type: 'number' } })
      expect(maxGuestsInput).toHaveValue(4)
    })

    test('updates select inputs', () => {
      render(
        <RoomTypeForm
          isOpen={true}
          onClose={mockOnClose}
          onSave={mockOnSave}
          mode="create"
        />
      )

      const bedTypeSelect = document.querySelector('select[name="bedType"]') as HTMLSelectElement
      fireEvent.change(bedTypeSelect, { target: { value: 'King', name: 'bedType' } })
      expect(bedTypeSelect).toHaveValue('King')
    })

    test('updates checkbox inputs', () => {
      render(
        <RoomTypeForm
          isOpen={true}
          onClose={mockOnClose}
          onSave={mockOnSave}
          mode="create"
        />
      )

      const activeCheckbox = screen.getByRole('checkbox')
      expect(activeCheckbox).toBeChecked()

      fireEvent.click(activeCheckbox)
      expect(activeCheckbox).not.toBeChecked()
    })
  })

  describe('Form Validation', () => {
    test('shows error when type code is whitespace only', async () => {
      render(
        <RoomTypeForm
          isOpen={true}
          onClose={mockOnClose}
          onSave={mockOnSave}
          mode="create"
        />
      )

      // Fill with whitespace to bypass HTML5 required validation
      fireEvent.change(screen.getByPlaceholderText('เช่น STD, DLX, SUI'), {
        target: { value: '   ', name: 'typeCode' },
      })

      const form = document.querySelector('form')!
      fireEvent.submit(form)

      await waitFor(() => {
        expect(screen.getByText('กรุณากรอกรหัสประเภทห้อง')).toBeInTheDocument()
      })

      expect(mockOnSave).not.toHaveBeenCalled()
    })

    test('shows error when type name is whitespace only', async () => {
      render(
        <RoomTypeForm
          isOpen={true}
          onClose={mockOnClose}
          onSave={mockOnSave}
          mode="create"
        />
      )

      fireEvent.change(screen.getByPlaceholderText('เช่น STD, DLX, SUI'), {
        target: { value: 'STD', name: 'typeCode' },
      })
      fireEvent.change(screen.getByPlaceholderText('กรอกชื่อประเภทห้อง'), {
        target: { value: '   ', name: 'typeName' },
      })

      const form = document.querySelector('form')!
      fireEvent.submit(form)

      await waitFor(() => {
        expect(screen.getByText('กรุณากรอกชื่อประเภทห้อง')).toBeInTheDocument()
      })
    })

    test('shows error when base price is negative', async () => {
      render(
        <RoomTypeForm
          isOpen={true}
          onClose={mockOnClose}
          onSave={mockOnSave}
          mode="create"
        />
      )

      fireEvent.change(screen.getByPlaceholderText('เช่น STD, DLX, SUI'), {
        target: { value: 'STD', name: 'typeCode' },
      })
      fireEvent.change(screen.getByPlaceholderText('กรอกชื่อประเภทห้อง'), {
        target: { value: 'Standard', name: 'typeName' },
      })

      const basePriceInput = document.querySelector('input[name="basePrice"]') as HTMLInputElement
      fireEvent.change(basePriceInput, { target: { value: '-100', name: 'basePrice', type: 'number' } })

      const form = document.querySelector('form')!
      fireEvent.submit(form)

      await waitFor(() => {
        expect(screen.getByText('ราคาพื้นฐานต้องไม่ติดลบ')).toBeInTheDocument()
      })
    })

    test('shows error when max guests is less than 1', async () => {
      render(
        <RoomTypeForm
          isOpen={true}
          onClose={mockOnClose}
          onSave={mockOnSave}
          mode="create"
        />
      )

      fireEvent.change(screen.getByPlaceholderText('เช่น STD, DLX, SUI'), {
        target: { value: 'STD', name: 'typeCode' },
      })
      fireEvent.change(screen.getByPlaceholderText('กรอกชื่อประเภทห้อง'), {
        target: { value: 'Standard', name: 'typeName' },
      })

      const maxGuestsInput = document.querySelector('input[name="maxGuests"]') as HTMLInputElement
      fireEvent.change(maxGuestsInput, { target: { value: '0', name: 'maxGuests', type: 'number' } })

      const form = document.querySelector('form')!
      fireEvent.submit(form)

      await waitFor(() => {
        expect(screen.getByText('จำนวนผู้เข้าพักสูงสุดต้องมากกว่า 0')).toBeInTheDocument()
      })
    })

    test('required fields have required attribute', () => {
      render(
        <RoomTypeForm
          isOpen={true}
          onClose={mockOnClose}
          onSave={mockOnSave}
          mode="create"
        />
      )

      expect(screen.getByPlaceholderText('เช่น STD, DLX, SUI')).toHaveAttribute('required')
      expect(screen.getByPlaceholderText('กรอกชื่อประเภทห้อง')).toHaveAttribute('required')
    })
  })

  describe('Form Submission', () => {
    test('calls onSave with correct data on valid submission', async () => {
      mockOnSave.mockResolvedValueOnce(undefined)

      render(
        <RoomTypeForm
          isOpen={true}
          onClose={mockOnClose}
          onSave={mockOnSave}
          mode="create"
        />
      )

      fireEvent.change(screen.getByPlaceholderText('เช่น STD, DLX, SUI'), {
        target: { value: 'STD', name: 'typeCode' },
      })
      fireEvent.change(screen.getByPlaceholderText('กรอกชื่อประเภทห้อง'), {
        target: { value: 'Standard Room', name: 'typeName' },
      })

      const submitButton = screen.getByText('เพิ่มประเภทห้อง')
      fireEvent.click(submitButton)

      await waitFor(() => {
        expect(mockOnSave).toHaveBeenCalledWith(
          expect.objectContaining({
            typeCode: 'STD',
            typeName: 'Standard Room',
          })
        )
      })
    })

    test('calls onClose after successful save', async () => {
      mockOnSave.mockResolvedValueOnce(undefined)

      render(
        <RoomTypeForm
          isOpen={true}
          onClose={mockOnClose}
          onSave={mockOnSave}
          mode="create"
        />
      )

      fireEvent.change(screen.getByPlaceholderText('เช่น STD, DLX, SUI'), {
        target: { value: 'STD', name: 'typeCode' },
      })
      fireEvent.change(screen.getByPlaceholderText('กรอกชื่อประเภทห้อง'), {
        target: { value: 'Standard', name: 'typeName' },
      })

      const submitButton = screen.getByText('เพิ่มประเภทห้อง')
      fireEvent.click(submitButton)

      await waitFor(() => {
        expect(mockOnClose).toHaveBeenCalled()
      })
    })

    test('shows error message on save failure', async () => {
      mockOnSave.mockRejectedValueOnce(new Error('Database error'))

      render(
        <RoomTypeForm
          isOpen={true}
          onClose={mockOnClose}
          onSave={mockOnSave}
          mode="create"
        />
      )

      fireEvent.change(screen.getByPlaceholderText('เช่น STD, DLX, SUI'), {
        target: { value: 'STD', name: 'typeCode' },
      })
      fireEvent.change(screen.getByPlaceholderText('กรอกชื่อประเภทห้อง'), {
        target: { value: 'Standard', name: 'typeName' },
      })

      const submitButton = screen.getByText('เพิ่มประเภทห้อง')
      fireEvent.click(submitButton)

      await waitFor(() => {
        expect(screen.getByText('Database error')).toBeInTheDocument()
      })

      expect(mockOnClose).not.toHaveBeenCalled()
    })
  })

  describe('Loading States', () => {
    test('disables submit button while saving', async () => {
      let resolvePromise: () => void
      mockOnSave.mockImplementationOnce(
        () =>
          new Promise<void>((resolve) => {
            resolvePromise = resolve
          })
      )

      render(
        <RoomTypeForm
          isOpen={true}
          onClose={mockOnClose}
          onSave={mockOnSave}
          mode="create"
        />
      )

      fireEvent.change(screen.getByPlaceholderText('เช่น STD, DLX, SUI'), {
        target: { value: 'STD', name: 'typeCode' },
      })
      fireEvent.change(screen.getByPlaceholderText('กรอกชื่อประเภทห้อง'), {
        target: { value: 'Standard', name: 'typeName' },
      })

      const submitButton = screen.getByRole('button', { name: /เพิ่มประเภทห้อง/i })
      fireEvent.click(submitButton)

      await waitFor(() => {
        expect(submitButton).toBeDisabled()
      })

      resolvePromise!()
    })

    test('shows loading indicator while saving', async () => {
      let resolvePromise: () => void
      mockOnSave.mockImplementationOnce(
        () =>
          new Promise<void>((resolve) => {
            resolvePromise = resolve
          })
      )

      render(
        <RoomTypeForm
          isOpen={true}
          onClose={mockOnClose}
          onSave={mockOnSave}
          mode="create"
        />
      )

      fireEvent.change(screen.getByPlaceholderText('เช่น STD, DLX, SUI'), {
        target: { value: 'STD', name: 'typeCode' },
      })
      fireEvent.change(screen.getByPlaceholderText('กรอกชื่อประเภทห้อง'), {
        target: { value: 'Standard', name: 'typeName' },
      })

      const submitButton = screen.getByText('เพิ่มประเภทห้อง')
      fireEvent.click(submitButton)

      await waitFor(() => {
        expect(screen.getByTestId('icon-loader')).toBeInTheDocument()
      })

      resolvePromise!()
    })
  })

  describe('Delete Functionality', () => {
    test('shows delete confirmation dialog', () => {
      render(
        <RoomTypeForm
          isOpen={true}
          onClose={mockOnClose}
          onSave={mockOnSave}
          onDelete={mockOnDelete}
          initialData={mockRoomTypeData}
          mode="edit"
        />
      )

      fireEvent.click(screen.getByText('ลบ'))

      expect(screen.getByText('ยืนยันการลบ?')).toBeInTheDocument()
      expect(screen.getByText('ใช่')).toBeInTheDocument()
      expect(screen.getByText('ไม่')).toBeInTheDocument()
    })

    test('calls onDelete when confirmed', async () => {
      mockOnDelete.mockResolvedValueOnce(undefined)

      render(
        <RoomTypeForm
          isOpen={true}
          onClose={mockOnClose}
          onSave={mockOnSave}
          onDelete={mockOnDelete}
          initialData={mockRoomTypeData}
          mode="edit"
        />
      )

      fireEvent.click(screen.getByText('ลบ'))
      fireEvent.click(screen.getByText('ใช่'))

      await waitFor(() => {
        expect(mockOnDelete).toHaveBeenCalledWith(1)
      })
    })

    test('hides confirmation when No is clicked', () => {
      render(
        <RoomTypeForm
          isOpen={true}
          onClose={mockOnClose}
          onSave={mockOnSave}
          onDelete={mockOnDelete}
          initialData={mockRoomTypeData}
          mode="edit"
        />
      )

      fireEvent.click(screen.getByText('ลบ'))
      fireEvent.click(screen.getByText('ไม่'))

      expect(screen.queryByText('ยืนยันการลบ?')).not.toBeInTheDocument()
    })

    test('shows error on delete failure', async () => {
      mockOnDelete.mockRejectedValueOnce(new Error('Cannot delete'))

      render(
        <RoomTypeForm
          isOpen={true}
          onClose={mockOnClose}
          onSave={mockOnSave}
          onDelete={mockOnDelete}
          initialData={mockRoomTypeData}
          mode="edit"
        />
      )

      fireEvent.click(screen.getByText('ลบ'))
      fireEvent.click(screen.getByText('ใช่'))

      await waitFor(() => {
        expect(screen.getByText('Cannot delete')).toBeInTheDocument()
      })
    })
  })

  describe('Size SQM Handling', () => {
    test('handles empty size sqm as null', () => {
      render(
        <RoomTypeForm
          isOpen={true}
          onClose={mockOnClose}
          onSave={mockOnSave}
          initialData={{ ...mockRoomTypeData, sizeSqm: null }}
          mode="edit"
        />
      )

      const sizeInput = document.querySelector('input[name="sizeSqm"]') as HTMLInputElement
      expect(sizeInput).toHaveValue(null)
    })

    test('updates size sqm correctly', () => {
      render(
        <RoomTypeForm
          isOpen={true}
          onClose={mockOnClose}
          onSave={mockOnSave}
          mode="create"
        />
      )

      const sizeInput = document.querySelector('input[name="sizeSqm"]') as HTMLInputElement
      fireEvent.change(sizeInput, { target: { value: '42', name: 'sizeSqm', type: 'number' } })
      expect(sizeInput).toHaveValue(42)
    })
  })
})
