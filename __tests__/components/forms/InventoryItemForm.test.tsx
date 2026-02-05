/**
 * @jest-environment jsdom
 */

import { render, screen, fireEvent, waitFor } from '@testing-library/react'
import InventoryItemForm from '@/components/forms/InventoryItemForm'
import { InventoryItemFormData } from '@/types/inventory'

// Mock lucide-react icons
jest.mock('lucide-react', () => ({
  X: () => <span data-testid="icon-x">X</span>,
  Package: () => <span data-testid="icon-package">Package</span>,
  Loader2: () => <span data-testid="icon-loader">Loading</span>,
  AlertCircle: () => <span data-testid="icon-alert">Alert</span>,
  Save: () => <span data-testid="icon-save">Save</span>,
  Trash2: () => <span data-testid="icon-trash">Trash</span>,
  Hash: () => <span data-testid="icon-hash">Hash</span>,
  Tag: () => <span data-testid="icon-tag">Tag</span>,
  Layers: () => <span data-testid="icon-layers">Layers</span>,
  DollarSign: () => <span data-testid="icon-dollar">Dollar</span>,
}))

const mockInventoryItemData: InventoryItemFormData = {
  id: 1,
  itemCode: 'AMT-001',
  itemName: 'แชมพู',
  category: 'Amenities',
  unit: 'ขวด',
  minStock: 50,
  currentStock: 100,
  costPerUnit: 25.5,
  active: true,
}

describe('InventoryItemForm Component', () => {
  const mockOnClose = jest.fn()
  const mockOnSave = jest.fn()
  const mockOnDelete = jest.fn()

  beforeEach(() => {
    jest.clearAllMocks()
  })

  describe('Modal Behavior', () => {
    test('does not render when isOpen is false', () => {
      render(
        <InventoryItemForm
          isOpen={false}
          onClose={mockOnClose}
          onSave={mockOnSave}
          mode="create"
        />
      )

      expect(screen.queryByText('เพิ่มสินค้าใหม่')).not.toBeInTheDocument()
    })

    test('renders when isOpen is true', () => {
      render(
        <InventoryItemForm
          isOpen={true}
          onClose={mockOnClose}
          onSave={mockOnSave}
          mode="create"
        />
      )

      expect(screen.getByText('เพิ่มสินค้าใหม่')).toBeInTheDocument()
    })

    test('calls onClose when close button is clicked', () => {
      render(
        <InventoryItemForm
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
        <InventoryItemForm
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
        <InventoryItemForm
          isOpen={true}
          onClose={mockOnClose}
          onSave={mockOnSave}
          mode="create"
        />
      )

      expect(screen.getByText('เพิ่มสินค้าใหม่')).toBeInTheDocument()
    })

    test('displays create button text', () => {
      render(
        <InventoryItemForm
          isOpen={true}
          onClose={mockOnClose}
          onSave={mockOnSave}
          mode="create"
        />
      )

      expect(screen.getByText('เพิ่มสินค้า')).toBeInTheDocument()
    })

    test('does not show delete button in create mode', () => {
      render(
        <InventoryItemForm
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
        <InventoryItemForm
          isOpen={true}
          onClose={mockOnClose}
          onSave={mockOnSave}
          initialData={mockInventoryItemData}
          mode="edit"
        />
      )

      expect(screen.getByText('แก้ไขสินค้า')).toBeInTheDocument()
    })

    test('displays save button text', () => {
      render(
        <InventoryItemForm
          isOpen={true}
          onClose={mockOnClose}
          onSave={mockOnSave}
          initialData={mockInventoryItemData}
          mode="edit"
        />
      )

      expect(screen.getByText('บันทึก')).toBeInTheDocument()
    })

    test('populates form with initial data', () => {
      render(
        <InventoryItemForm
          isOpen={true}
          onClose={mockOnClose}
          onSave={mockOnSave}
          initialData={mockInventoryItemData}
          mode="edit"
        />
      )

      expect(screen.getByDisplayValue('AMT-001')).toBeInTheDocument()
      expect(screen.getByDisplayValue('แชมพู')).toBeInTheDocument()
      expect(screen.getByDisplayValue('50')).toBeInTheDocument()
      expect(screen.getByDisplayValue('100')).toBeInTheDocument()
      expect(screen.getByDisplayValue('25.5')).toBeInTheDocument()
    })

    test('shows delete button in edit mode with onDelete', () => {
      render(
        <InventoryItemForm
          isOpen={true}
          onClose={mockOnClose}
          onSave={mockOnSave}
          onDelete={mockOnDelete}
          initialData={mockInventoryItemData}
          mode="edit"
        />
      )

      expect(screen.getByText('ลบ')).toBeInTheDocument()
    })
  })

  describe('Form Field Rendering', () => {
    test('renders all form fields', () => {
      render(
        <InventoryItemForm
          isOpen={true}
          onClose={mockOnClose}
          onSave={mockOnSave}
          mode="create"
        />
      )

      expect(screen.getByPlaceholderText('เช่น AMT-001, MNB-001')).toBeInTheDocument()
      expect(screen.getByPlaceholderText('กรอกชื่อสินค้า')).toBeInTheDocument()
    })

    test('renders category dropdown with all options', () => {
      render(
        <InventoryItemForm
          isOpen={true}
          onClose={mockOnClose}
          onSave={mockOnSave}
          mode="create"
        />
      )

      const categorySelect = document.querySelector('select[name="category"]')
      expect(categorySelect).toBeInTheDocument()

      expect(screen.getByText('Minibar (เครื่องดื่ม/ของว่าง)')).toBeInTheDocument()
      expect(screen.getByText('Amenities (อุปกรณ์อำนวยความสะดวก)')).toBeInTheDocument()
      expect(screen.getByText('Linens (ผ้าและเครื่องนอน)')).toBeInTheDocument()
      expect(screen.getByText('Equipment (อุปกรณ์ในห้อง)')).toBeInTheDocument()
    })

    test('renders unit dropdown with all options', () => {
      render(
        <InventoryItemForm
          isOpen={true}
          onClose={mockOnClose}
          onSave={mockOnSave}
          mode="create"
        />
      )

      const unitSelect = document.querySelector('select[name="unit"]')
      expect(unitSelect).toBeInTheDocument()

      expect(screen.getByText('-- เลือกหน่วยนับ --')).toBeInTheDocument()
      expect(screen.getByRole('option', { name: 'ชิ้น' })).toBeInTheDocument()
      expect(screen.getByRole('option', { name: 'ขวด' })).toBeInTheDocument()
      expect(screen.getByRole('option', { name: 'กล่อง' })).toBeInTheDocument()
      expect(screen.getByRole('option', { name: 'ชุด' })).toBeInTheDocument()
      expect(screen.getByRole('option', { name: 'ผืน' })).toBeInTheDocument()
      expect(screen.getByRole('option', { name: 'ม้วน' })).toBeInTheDocument()
      expect(screen.getByRole('option', { name: 'แพ็ค' })).toBeInTheDocument()
      expect(screen.getByRole('option', { name: 'อัน' })).toBeInTheDocument()
    })

    test('renders active toggle', () => {
      render(
        <InventoryItemForm
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
        <InventoryItemForm
          isOpen={true}
          onClose={mockOnClose}
          onSave={mockOnSave}
          mode="create"
        />
      )

      const itemCodeInput = screen.getByPlaceholderText('เช่น AMT-001, MNB-001')
      fireEvent.change(itemCodeInput, { target: { value: 'MNB-002', name: 'itemCode' } })
      expect(itemCodeInput).toHaveValue('MNB-002')

      const itemNameInput = screen.getByPlaceholderText('กรอกชื่อสินค้า')
      fireEvent.change(itemNameInput, { target: { value: 'โค้ก', name: 'itemName' } })
      expect(itemNameInput).toHaveValue('โค้ก')
    })

    test('updates category select', () => {
      render(
        <InventoryItemForm
          isOpen={true}
          onClose={mockOnClose}
          onSave={mockOnSave}
          mode="create"
        />
      )

      const categorySelect = document.querySelector('select[name="category"]') as HTMLSelectElement
      fireEvent.change(categorySelect, { target: { value: 'Minibar', name: 'category' } })
      expect(categorySelect).toHaveValue('Minibar')
    })

    test('updates unit select', () => {
      render(
        <InventoryItemForm
          isOpen={true}
          onClose={mockOnClose}
          onSave={mockOnSave}
          mode="create"
        />
      )

      const unitSelect = document.querySelector('select[name="unit"]') as HTMLSelectElement
      fireEvent.change(unitSelect, { target: { value: 'กล่อง', name: 'unit' } })
      expect(unitSelect).toHaveValue('กล่อง')
    })

    test('updates numeric inputs', () => {
      render(
        <InventoryItemForm
          isOpen={true}
          onClose={mockOnClose}
          onSave={mockOnSave}
          mode="create"
        />
      )

      const minStockInputs = screen.getAllByRole('spinbutton')
      const minStockInput = minStockInputs[0]
      fireEvent.change(minStockInput, { target: { value: '30', name: 'minStock', type: 'number' } })
      expect(minStockInput).toHaveValue(30)
    })

    test('updates checkbox inputs', () => {
      render(
        <InventoryItemForm
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
    test('shows error when item code is whitespace only', async () => {
      render(
        <InventoryItemForm
          isOpen={true}
          onClose={mockOnClose}
          onSave={mockOnSave}
          mode="create"
        />
      )

      // Use whitespace to bypass HTML5 required validation
      fireEvent.change(screen.getByPlaceholderText('เช่น AMT-001, MNB-001'), {
        target: { value: '   ', name: 'itemCode' },
      })

      const form = document.querySelector('form')!
      fireEvent.submit(form)

      await waitFor(() => {
        expect(screen.getByText('กรุณากรอกรหัสสินค้า')).toBeInTheDocument()
      })

      expect(mockOnSave).not.toHaveBeenCalled()
    })

    test('shows error when item name is whitespace only', async () => {
      render(
        <InventoryItemForm
          isOpen={true}
          onClose={mockOnClose}
          onSave={mockOnSave}
          mode="create"
        />
      )

      fireEvent.change(screen.getByPlaceholderText('เช่น AMT-001, MNB-001'), {
        target: { value: 'TEST-001', name: 'itemCode' },
      })
      fireEvent.change(screen.getByPlaceholderText('กรอกชื่อสินค้า'), {
        target: { value: '   ', name: 'itemName' },
      })

      const form = document.querySelector('form')!
      fireEvent.submit(form)

      await waitFor(() => {
        expect(screen.getByText('กรุณากรอกชื่อสินค้า')).toBeInTheDocument()
      })
    })

    test('shows error when unit is not selected', async () => {
      render(
        <InventoryItemForm
          isOpen={true}
          onClose={mockOnClose}
          onSave={mockOnSave}
          mode="create"
        />
      )

      fireEvent.change(screen.getByPlaceholderText('เช่น AMT-001, MNB-001'), {
        target: { value: 'TEST-001', name: 'itemCode' },
      })
      fireEvent.change(screen.getByPlaceholderText('กรอกชื่อสินค้า'), {
        target: { value: 'Test Item', name: 'itemName' },
      })

      // Clear unit selection
      const unitSelect = document.querySelector('select[name="unit"]') as HTMLSelectElement
      fireEvent.change(unitSelect, { target: { value: '', name: 'unit' } })

      const form = document.querySelector('form')!
      fireEvent.submit(form)

      await waitFor(() => {
        expect(screen.getByText('กรุณาเลือกหน่วยนับ')).toBeInTheDocument()
      })
    })

    test('shows error when min stock is negative', async () => {
      render(
        <InventoryItemForm
          isOpen={true}
          onClose={mockOnClose}
          onSave={mockOnSave}
          mode="create"
        />
      )

      fireEvent.change(screen.getByPlaceholderText('เช่น AMT-001, MNB-001'), {
        target: { value: 'TEST-001', name: 'itemCode' },
      })
      fireEvent.change(screen.getByPlaceholderText('กรอกชื่อสินค้า'), {
        target: { value: 'Test Item', name: 'itemName' },
      })

      const unitSelect = document.querySelector('select[name="unit"]') as HTMLSelectElement
      fireEvent.change(unitSelect, { target: { value: 'ชิ้น', name: 'unit' } })

      const minStockInput = document.querySelector('input[name="minStock"]') as HTMLInputElement
      fireEvent.change(minStockInput, { target: { value: '-10', name: 'minStock', type: 'number' } })

      const form = document.querySelector('form')!
      fireEvent.submit(form)

      await waitFor(() => {
        expect(screen.getByText('จำนวนขั้นต่ำต้องไม่ติดลบ')).toBeInTheDocument()
      })
    })

    test('shows error when current stock is negative', async () => {
      render(
        <InventoryItemForm
          isOpen={true}
          onClose={mockOnClose}
          onSave={mockOnSave}
          mode="create"
        />
      )

      fireEvent.change(screen.getByPlaceholderText('เช่น AMT-001, MNB-001'), {
        target: { value: 'TEST-001', name: 'itemCode' },
      })
      fireEvent.change(screen.getByPlaceholderText('กรอกชื่อสินค้า'), {
        target: { value: 'Test Item', name: 'itemName' },
      })

      const unitSelect = document.querySelector('select[name="unit"]') as HTMLSelectElement
      fireEvent.change(unitSelect, { target: { value: 'ชิ้น', name: 'unit' } })

      const currentStockInput = document.querySelector('input[name="currentStock"]') as HTMLInputElement
      fireEvent.change(currentStockInput, { target: { value: '-5', name: 'currentStock', type: 'number' } })

      const form = document.querySelector('form')!
      fireEvent.submit(form)

      await waitFor(() => {
        expect(screen.getByText('จำนวนคงเหลือต้องไม่ติดลบ')).toBeInTheDocument()
      })
    })

    test('required fields have required attribute', () => {
      render(
        <InventoryItemForm
          isOpen={true}
          onClose={mockOnClose}
          onSave={mockOnSave}
          mode="create"
        />
      )

      expect(screen.getByPlaceholderText('เช่น AMT-001, MNB-001')).toHaveAttribute('required')
      expect(screen.getByPlaceholderText('กรอกชื่อสินค้า')).toHaveAttribute('required')
    })
  })

  describe('Form Submission', () => {
    test('calls onSave with correct data on valid submission', async () => {
      mockOnSave.mockResolvedValueOnce(undefined)

      render(
        <InventoryItemForm
          isOpen={true}
          onClose={mockOnClose}
          onSave={mockOnSave}
          mode="create"
        />
      )

      fireEvent.change(screen.getByPlaceholderText('เช่น AMT-001, MNB-001'), {
        target: { value: 'LIN-001', name: 'itemCode' },
      })
      fireEvent.change(screen.getByPlaceholderText('กรอกชื่อสินค้า'), {
        target: { value: 'ผ้าขนหนู', name: 'itemName' },
      })

      const categorySelect = document.querySelector('select[name="category"]') as HTMLSelectElement
      fireEvent.change(categorySelect, { target: { value: 'Linens', name: 'category' } })

      const unitSelect = document.querySelector('select[name="unit"]') as HTMLSelectElement
      fireEvent.change(unitSelect, { target: { value: 'ผืน', name: 'unit' } })

      const submitButton = screen.getByText('เพิ่มสินค้า')
      fireEvent.click(submitButton)

      await waitFor(() => {
        expect(mockOnSave).toHaveBeenCalledWith(
          expect.objectContaining({
            itemCode: 'LIN-001',
            itemName: 'ผ้าขนหนู',
            category: 'Linens',
            unit: 'ผืน',
          })
        )
      })
    })

    test('calls onClose after successful save', async () => {
      mockOnSave.mockResolvedValueOnce(undefined)

      render(
        <InventoryItemForm
          isOpen={true}
          onClose={mockOnClose}
          onSave={mockOnSave}
          mode="create"
        />
      )

      fireEvent.change(screen.getByPlaceholderText('เช่น AMT-001, MNB-001'), {
        target: { value: 'TEST-001', name: 'itemCode' },
      })
      fireEvent.change(screen.getByPlaceholderText('กรอกชื่อสินค้า'), {
        target: { value: 'Test Item', name: 'itemName' },
      })

      const unitSelect = document.querySelector('select[name="unit"]') as HTMLSelectElement
      fireEvent.change(unitSelect, { target: { value: 'ชิ้น', name: 'unit' } })

      const submitButton = screen.getByText('เพิ่มสินค้า')
      fireEvent.click(submitButton)

      await waitFor(() => {
        expect(mockOnClose).toHaveBeenCalled()
      })
    })

    test('shows error message on save failure', async () => {
      mockOnSave.mockRejectedValueOnce(new Error('Item code already exists'))

      render(
        <InventoryItemForm
          isOpen={true}
          onClose={mockOnClose}
          onSave={mockOnSave}
          mode="create"
        />
      )

      fireEvent.change(screen.getByPlaceholderText('เช่น AMT-001, MNB-001'), {
        target: { value: 'TEST-001', name: 'itemCode' },
      })
      fireEvent.change(screen.getByPlaceholderText('กรอกชื่อสินค้า'), {
        target: { value: 'Test Item', name: 'itemName' },
      })

      const unitSelect = document.querySelector('select[name="unit"]') as HTMLSelectElement
      fireEvent.change(unitSelect, { target: { value: 'ชิ้น', name: 'unit' } })

      const submitButton = screen.getByText('เพิ่มสินค้า')
      fireEvent.click(submitButton)

      await waitFor(() => {
        expect(screen.getByText('Item code already exists')).toBeInTheDocument()
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
        <InventoryItemForm
          isOpen={true}
          onClose={mockOnClose}
          onSave={mockOnSave}
          mode="create"
        />
      )

      fireEvent.change(screen.getByPlaceholderText('เช่น AMT-001, MNB-001'), {
        target: { value: 'TEST-001', name: 'itemCode' },
      })
      fireEvent.change(screen.getByPlaceholderText('กรอกชื่อสินค้า'), {
        target: { value: 'Test Item', name: 'itemName' },
      })

      const unitSelect = document.querySelector('select[name="unit"]') as HTMLSelectElement
      fireEvent.change(unitSelect, { target: { value: 'ชิ้น', name: 'unit' } })

      const submitButton = screen.getByRole('button', { name: /เพิ่มสินค้า/i })
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
        <InventoryItemForm
          isOpen={true}
          onClose={mockOnClose}
          onSave={mockOnSave}
          mode="create"
        />
      )

      fireEvent.change(screen.getByPlaceholderText('เช่น AMT-001, MNB-001'), {
        target: { value: 'TEST-001', name: 'itemCode' },
      })
      fireEvent.change(screen.getByPlaceholderText('กรอกชื่อสินค้า'), {
        target: { value: 'Test Item', name: 'itemName' },
      })

      const unitSelect = document.querySelector('select[name="unit"]') as HTMLSelectElement
      fireEvent.change(unitSelect, { target: { value: 'ชิ้น', name: 'unit' } })

      const submitButton = screen.getByText('เพิ่มสินค้า')
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
        <InventoryItemForm
          isOpen={true}
          onClose={mockOnClose}
          onSave={mockOnSave}
          onDelete={mockOnDelete}
          initialData={mockInventoryItemData}
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
        <InventoryItemForm
          isOpen={true}
          onClose={mockOnClose}
          onSave={mockOnSave}
          onDelete={mockOnDelete}
          initialData={mockInventoryItemData}
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
        <InventoryItemForm
          isOpen={true}
          onClose={mockOnClose}
          onSave={mockOnSave}
          onDelete={mockOnDelete}
          initialData={mockInventoryItemData}
          mode="edit"
        />
      )

      fireEvent.click(screen.getByText('ลบ'))
      fireEvent.click(screen.getByText('ไม่'))

      expect(screen.queryByText('ยืนยันการลบ?')).not.toBeInTheDocument()
    })

    test('shows error on delete failure', async () => {
      mockOnDelete.mockRejectedValueOnce(new Error('Cannot delete item in use'))

      render(
        <InventoryItemForm
          isOpen={true}
          onClose={mockOnClose}
          onSave={mockOnSave}
          onDelete={mockOnDelete}
          initialData={mockInventoryItemData}
          mode="edit"
        />
      )

      fireEvent.click(screen.getByText('ลบ'))
      fireEvent.click(screen.getByText('ใช่'))

      await waitFor(() => {
        expect(screen.getByText('Cannot delete item in use')).toBeInTheDocument()
      })
    })
  })

  describe('Cost Per Unit Handling', () => {
    test('handles null cost per unit', () => {
      render(
        <InventoryItemForm
          isOpen={true}
          onClose={mockOnClose}
          onSave={mockOnSave}
          initialData={{ ...mockInventoryItemData, costPerUnit: null }}
          mode="edit"
        />
      )

      const costInputs = screen.getAllByRole('spinbutton')
      const costInput = costInputs[costInputs.length - 1] // Cost is last spinbutton
      expect(costInput).toHaveValue(null)
    })

    test('updates cost per unit correctly', () => {
      render(
        <InventoryItemForm
          isOpen={true}
          onClose={mockOnClose}
          onSave={mockOnSave}
          mode="create"
        />
      )

      const costInputs = screen.getAllByRole('spinbutton')
      const costInput = costInputs[costInputs.length - 1]
      fireEvent.change(costInput, { target: { value: '15.75', name: 'costPerUnit', type: 'number' } })
      expect(costInput).toHaveValue(15.75)
    })
  })
})
