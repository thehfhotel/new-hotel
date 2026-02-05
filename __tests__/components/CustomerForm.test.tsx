/**
 * @jest-environment jsdom
 */

import { render, screen, fireEvent, waitFor } from '@testing-library/react'
import CustomerForm, { CustomerFormData } from '@/components/forms/CustomerForm'

// Mock lucide-react icons
jest.mock('lucide-react', () => ({
  X: () => <span data-testid="x-icon">X</span>,
  User: () => <span data-testid="user-icon">User</span>,
  Phone: () => <span data-testid="phone-icon">Phone</span>,
  Mail: () => <span data-testid="mail-icon">Mail</span>,
  CreditCard: () => <span data-testid="credit-card-icon">CreditCard</span>,
  MapPin: () => <span data-testid="map-pin-icon">MapPin</span>,
  FileText: () => <span data-testid="file-text-icon">FileText</span>,
  Loader2: () => <span data-testid="loader-icon">Loading</span>,
  AlertCircle: () => <span data-testid="alert-icon">Alert</span>,
  Save: () => <span data-testid="save-icon">Save</span>,
  Trash2: () => <span data-testid="trash-icon">Trash</span>,
}))

const mockCustomerData: CustomerFormData = {
  id: 1,
  firstName: 'สมชาย',
  lastName: 'ใจดี',
  phone: '0891234567',
  email: 'somchai@example.com',
  idCard: '***REMOVED***90123',
  address: '123 ถ.สุขุมวิท กรุงเทพฯ',
  notes: 'ลูกค้าประจำ',
}

describe('CustomerForm Component', () => {
  const defaultProps = {
    isOpen: true,
    onClose: jest.fn(),
    onSave: jest.fn(),
    mode: 'create' as const,
  }

  beforeEach(() => {
    jest.clearAllMocks()
  })

  describe('Rendering', () => {
    test('renders create mode with correct Thai title', () => {
      render(<CustomerForm {...defaultProps} />)

      expect(screen.getByText('เพิ่มลูกค้าใหม่')).toBeInTheDocument()
    })

    test('renders edit mode with correct Thai title', () => {
      render(
        <CustomerForm
          {...defaultProps}
          mode="edit"
          initialData={mockCustomerData}
        />
      )

      expect(screen.getByText('แก้ไขข้อมูลลูกค้า')).toBeInTheDocument()
    })

    test('renders all form fields with Thai labels', () => {
      render(<CustomerForm {...defaultProps} />)

      expect(screen.getByText(/ชื่อ/)).toBeInTheDocument()
      expect(screen.getByText('นามสกุล')).toBeInTheDocument()
      expect(screen.getByText('เบอร์โทร')).toBeInTheDocument()
      expect(screen.getByText('อีเมล')).toBeInTheDocument()
      expect(screen.getByText('เลขบัตรประชาชน')).toBeInTheDocument()
      expect(screen.getByText('ที่อยู่')).toBeInTheDocument()
      expect(screen.getByText('หมายเหตุ')).toBeInTheDocument()
    })

    test('renders Thai placeholders', () => {
      render(<CustomerForm {...defaultProps} />)

      expect(screen.getByPlaceholderText('กรอกชื่อ')).toBeInTheDocument()
      expect(screen.getByPlaceholderText('กรอกนามสกุล')).toBeInTheDocument()
      expect(screen.getByPlaceholderText('กรอกเบอร์โทรศัพท์')).toBeInTheDocument()
      expect(screen.getByPlaceholderText('กรอกอีเมล')).toBeInTheDocument()
      expect(screen.getByPlaceholderText('กรอกเลขบัตรประชาชน')).toBeInTheDocument()
      expect(screen.getByPlaceholderText('กรอกที่อยู่')).toBeInTheDocument()
      expect(screen.getByPlaceholderText('กรอกหมายเหตุ')).toBeInTheDocument()
    })

    test('renders required field indicator for firstName', () => {
      render(<CustomerForm {...defaultProps} />)

      // Check for the red asterisk next to "ชื่อ"
      const requiredIndicator = screen.getByText('*')
      expect(requiredIndicator).toHaveClass('text-red-500')
    })

    test('renders action buttons with Thai text', () => {
      render(<CustomerForm {...defaultProps} />)

      expect(screen.getByText('ยกเลิก')).toBeInTheDocument()
      expect(screen.getByText('เพิ่มลูกค้า')).toBeInTheDocument()
    })

    test('renders save button text in edit mode', () => {
      render(
        <CustomerForm
          {...defaultProps}
          mode="edit"
          initialData={mockCustomerData}
        />
      )

      expect(screen.getByText('บันทึก')).toBeInTheDocument()
    })

    test('does not render when isOpen is false', () => {
      render(<CustomerForm {...defaultProps} isOpen={false} />)

      expect(screen.queryByText('เพิ่มลูกค้าใหม่')).not.toBeInTheDocument()
    })
  })

  describe('Form Initialization', () => {
    test('initializes with empty fields in create mode', () => {
      render(<CustomerForm {...defaultProps} />)

      const firstNameInput = screen.getByPlaceholderText('กรอกชื่อ') as HTMLInputElement
      expect(firstNameInput.value).toBe('')
    })

    test('populates fields with initialData in edit mode', () => {
      render(
        <CustomerForm
          {...defaultProps}
          mode="edit"
          initialData={mockCustomerData}
        />
      )

      expect(screen.getByDisplayValue('สมชาย')).toBeInTheDocument()
      expect(screen.getByDisplayValue('ใจดี')).toBeInTheDocument()
      expect(screen.getByDisplayValue('0891234567')).toBeInTheDocument()
      expect(screen.getByDisplayValue('somchai@example.com')).toBeInTheDocument()
      expect(screen.getByDisplayValue('***REMOVED***90123')).toBeInTheDocument()
      expect(screen.getByDisplayValue('123 ถ.สุขุมวิท กรุงเทพฯ')).toBeInTheDocument()
      expect(screen.getByDisplayValue('ลูกค้าประจำ')).toBeInTheDocument()
    })
  })

  describe('Form Input Handling', () => {
    test('updates firstName field on input', () => {
      render(<CustomerForm {...defaultProps} />)

      const firstNameInput = screen.getByPlaceholderText('กรอกชื่อ')
      fireEvent.change(firstNameInput, { target: { value: 'ทดสอบ' } })

      expect(screen.getByDisplayValue('ทดสอบ')).toBeInTheDocument()
    })

    test('updates lastName field on input', () => {
      render(<CustomerForm {...defaultProps} />)

      const lastNameInput = screen.getByPlaceholderText('กรอกนามสกุล')
      fireEvent.change(lastNameInput, { target: { value: 'นามสกุลทดสอบ' } })

      expect(screen.getByDisplayValue('นามสกุลทดสอบ')).toBeInTheDocument()
    })

    test('updates phone field on input', () => {
      render(<CustomerForm {...defaultProps} />)

      const phoneInput = screen.getByPlaceholderText('กรอกเบอร์โทรศัพท์')
      fireEvent.change(phoneInput, { target: { value: '08***REMOVED***' } })

      expect(screen.getByDisplayValue('08***REMOVED***')).toBeInTheDocument()
    })
  })

  describe('Form Validation', () => {
    test('shows error when submitting without firstName', async () => {
      render(<CustomerForm {...defaultProps} />)

      const submitButton = screen.getByText('เพิ่มลูกค้า')
      fireEvent.click(submitButton)

      await waitFor(() => {
        expect(screen.getByText('กรุณากรอกชื่อ')).toBeInTheDocument()
      })

      expect(defaultProps.onSave).not.toHaveBeenCalled()
    })

    test('shows error when firstName is only whitespace', async () => {
      render(<CustomerForm {...defaultProps} />)

      const firstNameInput = screen.getByPlaceholderText('กรอกชื่อ')
      fireEvent.change(firstNameInput, { target: { value: '   ' } })

      const submitButton = screen.getByText('เพิ่มลูกค้า')
      fireEvent.click(submitButton)

      await waitFor(() => {
        expect(screen.getByText('กรุณากรอกชื่อ')).toBeInTheDocument()
      })
    })

    test('allows submission with only firstName filled', async () => {
      const mockOnSave = jest.fn().mockResolvedValue(undefined)
      render(<CustomerForm {...defaultProps} onSave={mockOnSave} />)

      const firstNameInput = screen.getByPlaceholderText('กรอกชื่อ')
      fireEvent.change(firstNameInput, { target: { value: 'ทดสอบ' } })

      const submitButton = screen.getByText('เพิ่มลูกค้า')
      fireEvent.click(submitButton)

      await waitFor(() => {
        expect(mockOnSave).toHaveBeenCalledWith(
          expect.objectContaining({
            firstName: 'ทดสอบ',
          })
        )
      })
    })
  })

  describe('Form Submission', () => {
    test('calls onSave with form data on successful submission', async () => {
      const mockOnSave = jest.fn().mockResolvedValue(undefined)
      render(<CustomerForm {...defaultProps} onSave={mockOnSave} />)

      const firstNameInput = screen.getByPlaceholderText('กรอกชื่อ')
      fireEvent.change(firstNameInput, { target: { value: 'ชื่อทดสอบ' } })

      const lastNameInput = screen.getByPlaceholderText('กรอกนามสกุล')
      fireEvent.change(lastNameInput, { target: { value: 'นามสกุลทดสอบ' } })

      const submitButton = screen.getByText('เพิ่มลูกค้า')
      fireEvent.click(submitButton)

      await waitFor(() => {
        expect(mockOnSave).toHaveBeenCalledWith(
          expect.objectContaining({
            firstName: 'ชื่อทดสอบ',
            lastName: 'นามสกุลทดสอบ',
          })
        )
      })
    })

    test('calls onClose after successful save', async () => {
      const mockOnSave = jest.fn().mockResolvedValue(undefined)
      const mockOnClose = jest.fn()
      render(
        <CustomerForm
          {...defaultProps}
          onSave={mockOnSave}
          onClose={mockOnClose}
        />
      )

      const firstNameInput = screen.getByPlaceholderText('กรอกชื่อ')
      fireEvent.change(firstNameInput, { target: { value: 'ทดสอบ' } })

      const submitButton = screen.getByText('เพิ่มลูกค้า')
      fireEvent.click(submitButton)

      await waitFor(() => {
        expect(mockOnClose).toHaveBeenCalled()
      })
    })

    test('displays error message on save failure', async () => {
      const mockOnSave = jest.fn().mockRejectedValue(new Error('บันทึกไม่สำเร็จ'))
      render(<CustomerForm {...defaultProps} onSave={mockOnSave} />)

      const firstNameInput = screen.getByPlaceholderText('กรอกชื่อ')
      fireEvent.change(firstNameInput, { target: { value: 'ทดสอบ' } })

      const submitButton = screen.getByText('เพิ่มลูกค้า')
      fireEvent.click(submitButton)

      await waitFor(() => {
        expect(screen.getByText('บันทึกไม่สำเร็จ')).toBeInTheDocument()
      })
    })

    test('displays generic error message on non-Error rejection', async () => {
      const mockOnSave = jest.fn().mockRejectedValue('Unknown error')
      render(<CustomerForm {...defaultProps} onSave={mockOnSave} />)

      const firstNameInput = screen.getByPlaceholderText('กรอกชื่อ')
      fireEvent.change(firstNameInput, { target: { value: 'ทดสอบ' } })

      const submitButton = screen.getByText('เพิ่มลูกค้า')
      fireEvent.click(submitButton)

      await waitFor(() => {
        expect(screen.getByText('เกิดข้อผิดพลาดในการบันทึก')).toBeInTheDocument()
      })
    })

    test('disables submit button while saving', async () => {
      const mockOnSave = jest.fn().mockImplementation(
        () => new Promise((resolve) => setTimeout(resolve, 1000))
      )
      render(<CustomerForm {...defaultProps} onSave={mockOnSave} />)

      const firstNameInput = screen.getByPlaceholderText('กรอกชื่อ')
      fireEvent.change(firstNameInput, { target: { value: 'ทดสอบ' } })

      const submitButton = screen.getByText('เพิ่มลูกค้า')
      fireEvent.click(submitButton)

      // After clicking, button text should change and be disabled
      await waitFor(() => {
        const buttons = screen.getAllByRole('button')
        const submitBtn = buttons.find((btn) => btn.getAttribute('type') === 'submit')
        expect(submitBtn).toBeDisabled()
      })
    })
  })

  describe('Delete Functionality', () => {
    test('renders delete button in edit mode with onDelete provided', () => {
      const mockOnDelete = jest.fn()
      render(
        <CustomerForm
          {...defaultProps}
          mode="edit"
          initialData={mockCustomerData}
          onDelete={mockOnDelete}
        />
      )

      expect(screen.getByText('ลบ')).toBeInTheDocument()
    })

    test('does not render delete button in create mode', () => {
      render(<CustomerForm {...defaultProps} />)

      expect(screen.queryByText('ลบ')).not.toBeInTheDocument()
    })

    test('shows delete confirmation when delete button is clicked', () => {
      const mockOnDelete = jest.fn()
      render(
        <CustomerForm
          {...defaultProps}
          mode="edit"
          initialData={mockCustomerData}
          onDelete={mockOnDelete}
        />
      )

      const deleteButton = screen.getByText('ลบ')
      fireEvent.click(deleteButton)

      expect(screen.getByText('ยืนยันการลบ?')).toBeInTheDocument()
      expect(screen.getByText('ใช่')).toBeInTheDocument()
      expect(screen.getByText('ไม่')).toBeInTheDocument()
    })

    test('hides delete confirmation when "ไม่" is clicked', () => {
      const mockOnDelete = jest.fn()
      render(
        <CustomerForm
          {...defaultProps}
          mode="edit"
          initialData={mockCustomerData}
          onDelete={mockOnDelete}
        />
      )

      const deleteButton = screen.getByText('ลบ')
      fireEvent.click(deleteButton)

      const cancelButton = screen.getByText('ไม่')
      fireEvent.click(cancelButton)

      expect(screen.queryByText('ยืนยันการลบ?')).not.toBeInTheDocument()
    })

    test('calls onDelete when delete is confirmed', async () => {
      const mockOnDelete = jest.fn().mockResolvedValue(undefined)
      render(
        <CustomerForm
          {...defaultProps}
          mode="edit"
          initialData={mockCustomerData}
          onDelete={mockOnDelete}
        />
      )

      const deleteButton = screen.getByText('ลบ')
      fireEvent.click(deleteButton)

      const confirmButton = screen.getByText('ใช่')
      fireEvent.click(confirmButton)

      await waitFor(() => {
        expect(mockOnDelete).toHaveBeenCalledWith(mockCustomerData.id)
      })
    })

    test('displays error message on delete failure', async () => {
      const mockOnDelete = jest.fn().mockRejectedValue(new Error('ลบไม่สำเร็จ'))
      render(
        <CustomerForm
          {...defaultProps}
          mode="edit"
          initialData={mockCustomerData}
          onDelete={mockOnDelete}
        />
      )

      const deleteButton = screen.getByText('ลบ')
      fireEvent.click(deleteButton)

      const confirmButton = screen.getByText('ใช่')
      fireEvent.click(confirmButton)

      await waitFor(() => {
        expect(screen.getByText('ลบไม่สำเร็จ')).toBeInTheDocument()
      })
    })
  })

  describe('Modal Behavior', () => {
    test('calls onClose when cancel button is clicked', () => {
      const mockOnClose = jest.fn()
      render(<CustomerForm {...defaultProps} onClose={mockOnClose} />)

      const cancelButton = screen.getByText('ยกเลิก')
      fireEvent.click(cancelButton)

      expect(mockOnClose).toHaveBeenCalled()
    })

    test('calls onClose when close (X) button is clicked', () => {
      const mockOnClose = jest.fn()
      render(<CustomerForm {...defaultProps} onClose={mockOnClose} />)

      const closeButton = screen.getByLabelText('ปิด')
      fireEvent.click(closeButton)

      expect(mockOnClose).toHaveBeenCalled()
    })

    test('calls onClose when backdrop is clicked', () => {
      const mockOnClose = jest.fn()
      render(<CustomerForm {...defaultProps} onClose={mockOnClose} />)

      // Click on the backdrop (the fixed overlay element)
      const backdrop = document.querySelector('.fixed.inset-0.bg-black')
      fireEvent.click(backdrop!)

      expect(mockOnClose).toHaveBeenCalled()
    })
  })
})
