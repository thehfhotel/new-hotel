/**
 * @jest-environment jsdom
 */

import { render, screen, fireEvent, waitFor } from '@testing-library/react'
import CustomerForm, { CustomerFormData } from '@/components/forms/CustomerForm'

// Mock lucide-react icons
jest.mock('lucide-react', () => ({
  X: () => <span data-testid="icon-x">X</span>,
  User: () => <span data-testid="icon-user">User</span>,
  Phone: () => <span data-testid="icon-phone">Phone</span>,
  Mail: () => <span data-testid="icon-mail">Mail</span>,
  CreditCard: () => <span data-testid="icon-credit-card">CreditCard</span>,
  MapPin: () => <span data-testid="icon-map-pin">MapPin</span>,
  FileText: () => <span data-testid="icon-file-text">FileText</span>,
  Loader2: () => <span data-testid="icon-loader">Loading</span>,
  AlertCircle: () => <span data-testid="icon-alert">Alert</span>,
  Save: () => <span data-testid="icon-save">Save</span>,
  Trash2: () => <span data-testid="icon-trash">Trash</span>,
}))

const mockCustomerData: CustomerFormData = {
  id: 1,
  firstName: 'John',
  lastName: 'Doe',
  phone: '0812345678',
  email: 'john@example.com',
  idCard: '1234567890123',
  address: '123 Main Street',
  notes: 'VIP customer',
}

describe('CustomerForm Component', () => {
  const mockOnClose = jest.fn()
  const mockOnSave = jest.fn()
  const mockOnDelete = jest.fn()

  beforeEach(() => {
    jest.clearAllMocks()
  })

  describe('Modal Behavior', () => {
    test('does not render when isOpen is false', () => {
      render(
        <CustomerForm
          isOpen={false}
          onClose={mockOnClose}
          onSave={mockOnSave}
          mode="create"
        />
      )

      expect(screen.queryByText('เพิ่มลูกค้าใหม่')).not.toBeInTheDocument()
    })

    test('renders when isOpen is true', () => {
      render(
        <CustomerForm
          isOpen={true}
          onClose={mockOnClose}
          onSave={mockOnSave}
          mode="create"
        />
      )

      expect(screen.getByText('เพิ่มลูกค้าใหม่')).toBeInTheDocument()
    })

    test('calls onClose when close button is clicked', () => {
      render(
        <CustomerForm
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
        <CustomerForm
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

    test('calls onClose when backdrop is clicked', () => {
      render(
        <CustomerForm
          isOpen={true}
          onClose={mockOnClose}
          onSave={mockOnSave}
          mode="create"
        />
      )

      // Find the backdrop (first fixed div with bg-black)
      const backdrop = document.querySelector('.bg-black.bg-opacity-50')
      if (backdrop) {
        fireEvent.click(backdrop)
      }

      expect(mockOnClose).toHaveBeenCalledTimes(1)
    })
  })

  describe('Create Mode', () => {
    test('displays create mode title', () => {
      render(
        <CustomerForm
          isOpen={true}
          onClose={mockOnClose}
          onSave={mockOnSave}
          mode="create"
        />
      )

      expect(screen.getByText('เพิ่มลูกค้าใหม่')).toBeInTheDocument()
    })

    test('displays create button text', () => {
      render(
        <CustomerForm
          isOpen={true}
          onClose={mockOnClose}
          onSave={mockOnSave}
          mode="create"
        />
      )

      expect(screen.getByText('เพิ่มลูกค้า')).toBeInTheDocument()
    })

    test('does not show delete button in create mode', () => {
      render(
        <CustomerForm
          isOpen={true}
          onClose={mockOnClose}
          onSave={mockOnSave}
          onDelete={mockOnDelete}
          mode="create"
        />
      )

      expect(screen.queryByText('ลบ')).not.toBeInTheDocument()
    })

    test('renders with empty form fields', () => {
      render(
        <CustomerForm
          isOpen={true}
          onClose={mockOnClose}
          onSave={mockOnSave}
          mode="create"
        />
      )

      const firstNameInput = screen.getByPlaceholderText('กรอกชื่อ')
      const lastNameInput = screen.getByPlaceholderText('กรอกนามสกุล')
      const phoneInput = screen.getByPlaceholderText('กรอกเบอร์โทรศัพท์')
      const emailInput = screen.getByPlaceholderText('กรอกอีเมล')

      expect(firstNameInput).toHaveValue('')
      expect(lastNameInput).toHaveValue('')
      expect(phoneInput).toHaveValue('')
      expect(emailInput).toHaveValue('')
    })
  })

  describe('Edit Mode', () => {
    test('displays edit mode title', () => {
      render(
        <CustomerForm
          isOpen={true}
          onClose={mockOnClose}
          onSave={mockOnSave}
          initialData={mockCustomerData}
          mode="edit"
        />
      )

      expect(screen.getByText('แก้ไขข้อมูลลูกค้า')).toBeInTheDocument()
    })

    test('displays save button text', () => {
      render(
        <CustomerForm
          isOpen={true}
          onClose={mockOnClose}
          onSave={mockOnSave}
          initialData={mockCustomerData}
          mode="edit"
        />
      )

      expect(screen.getByText('บันทึก')).toBeInTheDocument()
    })

    test('populates form with initial data', () => {
      render(
        <CustomerForm
          isOpen={true}
          onClose={mockOnClose}
          onSave={mockOnSave}
          initialData={mockCustomerData}
          mode="edit"
        />
      )

      expect(screen.getByDisplayValue('John')).toBeInTheDocument()
      expect(screen.getByDisplayValue('Doe')).toBeInTheDocument()
      expect(screen.getByDisplayValue('0812345678')).toBeInTheDocument()
      expect(screen.getByDisplayValue('john@example.com')).toBeInTheDocument()
      expect(screen.getByDisplayValue('1234567890123')).toBeInTheDocument()
      expect(screen.getByDisplayValue('123 Main Street')).toBeInTheDocument()
      expect(screen.getByDisplayValue('VIP customer')).toBeInTheDocument()
    })

    test('shows delete button in edit mode with onDelete', () => {
      render(
        <CustomerForm
          isOpen={true}
          onClose={mockOnClose}
          onSave={mockOnSave}
          onDelete={mockOnDelete}
          initialData={mockCustomerData}
          mode="edit"
        />
      )

      expect(screen.getByText('ลบ')).toBeInTheDocument()
    })
  })

  describe('Form Field Rendering', () => {
    test('renders all form fields', () => {
      render(
        <CustomerForm
          isOpen={true}
          onClose={mockOnClose}
          onSave={mockOnSave}
          mode="create"
        />
      )

      expect(screen.getByPlaceholderText('กรอกชื่อ')).toBeInTheDocument()
      expect(screen.getByPlaceholderText('กรอกนามสกุล')).toBeInTheDocument()
      expect(screen.getByPlaceholderText('กรอกเบอร์โทรศัพท์')).toBeInTheDocument()
      expect(screen.getByPlaceholderText('กรอกอีเมล')).toBeInTheDocument()
      expect(screen.getByPlaceholderText('กรอกเลขบัตรประชาชน')).toBeInTheDocument()
      expect(screen.getByPlaceholderText('กรอกที่อยู่')).toBeInTheDocument()
      expect(screen.getByPlaceholderText('กรอกหมายเหตุ')).toBeInTheDocument()
    })

    test('marks first name as required', () => {
      render(
        <CustomerForm
          isOpen={true}
          onClose={mockOnClose}
          onSave={mockOnSave}
          mode="create"
        />
      )

      const firstNameInput = screen.getByPlaceholderText('กรอกชื่อ')
      expect(firstNameInput).toHaveAttribute('required')
    })
  })

  describe('Form Input Handling', () => {
    test('updates form data on input change', () => {
      render(
        <CustomerForm
          isOpen={true}
          onClose={mockOnClose}
          onSave={mockOnSave}
          mode="create"
        />
      )

      const firstNameInput = screen.getByPlaceholderText('กรอกชื่อ')
      fireEvent.change(firstNameInput, { target: { value: 'Jane' } })

      expect(firstNameInput).toHaveValue('Jane')
    })

    test('updates all form fields', () => {
      render(
        <CustomerForm
          isOpen={true}
          onClose={mockOnClose}
          onSave={mockOnSave}
          mode="create"
        />
      )

      fireEvent.change(screen.getByPlaceholderText('กรอกชื่อ'), {
        target: { value: 'Jane', name: 'firstName' },
      })
      fireEvent.change(screen.getByPlaceholderText('กรอกนามสกุล'), {
        target: { value: 'Smith', name: 'lastName' },
      })
      fireEvent.change(screen.getByPlaceholderText('กรอกเบอร์โทรศัพท์'), {
        target: { value: '0891234567', name: 'phone' },
      })
      fireEvent.change(screen.getByPlaceholderText('กรอกอีเมล'), {
        target: { value: 'jane@test.com', name: 'email' },
      })

      expect(screen.getByPlaceholderText('กรอกชื่อ')).toHaveValue('Jane')
      expect(screen.getByPlaceholderText('กรอกนามสกุล')).toHaveValue('Smith')
      expect(screen.getByPlaceholderText('กรอกเบอร์โทรศัพท์')).toHaveValue('0891234567')
      expect(screen.getByPlaceholderText('กรอกอีเมล')).toHaveValue('jane@test.com')
    })
  })

  describe('Form Validation', () => {
    test('shows error when first name is only whitespace', async () => {
      render(
        <CustomerForm
          isOpen={true}
          onClose={mockOnClose}
          onSave={mockOnSave}
          mode="create"
        />
      )

      // Set whitespace value - bypasses HTML5 required validation
      fireEvent.change(screen.getByPlaceholderText('กรอกชื่อ'), {
        target: { value: '   ', name: 'firstName' },
      })

      const form = document.querySelector('form')!
      fireEvent.submit(form)

      await waitFor(() => {
        expect(screen.getByText('กรุณากรอกชื่อ')).toBeInTheDocument()
      })

      expect(mockOnSave).not.toHaveBeenCalled()
    })

    test('first name field is required', () => {
      render(
        <CustomerForm
          isOpen={true}
          onClose={mockOnClose}
          onSave={mockOnSave}
          mode="create"
        />
      )

      const firstNameInput = screen.getByPlaceholderText('กรอกชื่อ')
      expect(firstNameInput).toHaveAttribute('required')
    })
  })

  describe('Form Submission', () => {
    test('calls onSave with correct data on valid submission', async () => {
      mockOnSave.mockResolvedValueOnce(undefined)

      render(
        <CustomerForm
          isOpen={true}
          onClose={mockOnClose}
          onSave={mockOnSave}
          mode="create"
        />
      )

      fireEvent.change(screen.getByPlaceholderText('กรอกชื่อ'), {
        target: { value: 'Jane', name: 'firstName' },
      })
      fireEvent.change(screen.getByPlaceholderText('กรอกนามสกุล'), {
        target: { value: 'Smith', name: 'lastName' },
      })

      const submitButton = screen.getByText('เพิ่มลูกค้า')
      fireEvent.click(submitButton)

      await waitFor(() => {
        expect(mockOnSave).toHaveBeenCalledWith(
          expect.objectContaining({
            firstName: 'Jane',
            lastName: 'Smith',
          })
        )
      })
    })

    test('calls onClose after successful save', async () => {
      mockOnSave.mockResolvedValueOnce(undefined)

      render(
        <CustomerForm
          isOpen={true}
          onClose={mockOnClose}
          onSave={mockOnSave}
          mode="create"
        />
      )

      fireEvent.change(screen.getByPlaceholderText('กรอกชื่อ'), {
        target: { value: 'Jane', name: 'firstName' },
      })

      const submitButton = screen.getByText('เพิ่มลูกค้า')
      fireEvent.click(submitButton)

      await waitFor(() => {
        expect(mockOnClose).toHaveBeenCalled()
      })
    })

    test('shows error message on save failure', async () => {
      mockOnSave.mockRejectedValueOnce(new Error('Database error'))

      render(
        <CustomerForm
          isOpen={true}
          onClose={mockOnClose}
          onSave={mockOnSave}
          mode="create"
        />
      )

      fireEvent.change(screen.getByPlaceholderText('กรอกชื่อ'), {
        target: { value: 'Jane', name: 'firstName' },
      })

      const submitButton = screen.getByText('เพิ่มลูกค้า')
      fireEvent.click(submitButton)

      await waitFor(() => {
        expect(screen.getByText('Database error')).toBeInTheDocument()
      })

      expect(mockOnClose).not.toHaveBeenCalled()
    })

    test('shows generic error message when error is not an Error instance', async () => {
      mockOnSave.mockRejectedValueOnce('Unknown error')

      render(
        <CustomerForm
          isOpen={true}
          onClose={mockOnClose}
          onSave={mockOnSave}
          mode="create"
        />
      )

      fireEvent.change(screen.getByPlaceholderText('กรอกชื่อ'), {
        target: { value: 'Jane', name: 'firstName' },
      })

      const submitButton = screen.getByText('เพิ่มลูกค้า')
      fireEvent.click(submitButton)

      await waitFor(() => {
        expect(screen.getByText('เกิดข้อผิดพลาดในการบันทึก')).toBeInTheDocument()
      })
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
        <CustomerForm
          isOpen={true}
          onClose={mockOnClose}
          onSave={mockOnSave}
          mode="create"
        />
      )

      fireEvent.change(screen.getByPlaceholderText('กรอกชื่อ'), {
        target: { value: 'Jane', name: 'firstName' },
      })

      const submitButton = screen.getByRole('button', { name: /เพิ่มลูกค้า/i })
      fireEvent.click(submitButton)

      await waitFor(() => {
        expect(submitButton).toBeDisabled()
      })

      // Resolve the promise to clean up
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
        <CustomerForm
          isOpen={true}
          onClose={mockOnClose}
          onSave={mockOnSave}
          mode="create"
        />
      )

      fireEvent.change(screen.getByPlaceholderText('กรอกชื่อ'), {
        target: { value: 'Jane', name: 'firstName' },
      })

      const submitButton = screen.getByText('เพิ่มลูกค้า')
      fireEvent.click(submitButton)

      await waitFor(() => {
        expect(screen.getByTestId('icon-loader')).toBeInTheDocument()
      })

      // Resolve the promise to clean up
      resolvePromise!()
    })
  })

  describe('Delete Functionality', () => {
    test('shows delete button in edit mode with onDelete and id', () => {
      render(
        <CustomerForm
          isOpen={true}
          onClose={mockOnClose}
          onSave={mockOnSave}
          onDelete={mockOnDelete}
          initialData={mockCustomerData}
          mode="edit"
        />
      )

      expect(screen.getByText('ลบ')).toBeInTheDocument()
    })

    test('does not show delete button without onDelete', () => {
      render(
        <CustomerForm
          isOpen={true}
          onClose={mockOnClose}
          onSave={mockOnSave}
          initialData={mockCustomerData}
          mode="edit"
        />
      )

      expect(screen.queryByText('ลบ')).not.toBeInTheDocument()
    })

    test('shows delete confirmation when delete button is clicked', () => {
      render(
        <CustomerForm
          isOpen={true}
          onClose={mockOnClose}
          onSave={mockOnSave}
          onDelete={mockOnDelete}
          initialData={mockCustomerData}
          mode="edit"
        />
      )

      const deleteButton = screen.getByText('ลบ')
      fireEvent.click(deleteButton)

      expect(screen.getByText('ยืนยันการลบ?')).toBeInTheDocument()
      expect(screen.getByText('ใช่')).toBeInTheDocument()
      expect(screen.getByText('ไม่')).toBeInTheDocument()
    })

    test('hides delete confirmation when No is clicked', () => {
      render(
        <CustomerForm
          isOpen={true}
          onClose={mockOnClose}
          onSave={mockOnSave}
          onDelete={mockOnDelete}
          initialData={mockCustomerData}
          mode="edit"
        />
      )

      const deleteButton = screen.getByText('ลบ')
      fireEvent.click(deleteButton)

      const noButton = screen.getByText('ไม่')
      fireEvent.click(noButton)

      expect(screen.queryByText('ยืนยันการลบ?')).not.toBeInTheDocument()
      expect(screen.getByText('ลบ')).toBeInTheDocument()
    })

    test('calls onDelete when confirmed', async () => {
      mockOnDelete.mockResolvedValueOnce(undefined)

      render(
        <CustomerForm
          isOpen={true}
          onClose={mockOnClose}
          onSave={mockOnSave}
          onDelete={mockOnDelete}
          initialData={mockCustomerData}
          mode="edit"
        />
      )

      const deleteButton = screen.getByText('ลบ')
      fireEvent.click(deleteButton)

      const confirmButton = screen.getByText('ใช่')
      fireEvent.click(confirmButton)

      await waitFor(() => {
        expect(mockOnDelete).toHaveBeenCalledWith(1)
      })
    })

    test('calls onClose after successful delete', async () => {
      mockOnDelete.mockResolvedValueOnce(undefined)

      render(
        <CustomerForm
          isOpen={true}
          onClose={mockOnClose}
          onSave={mockOnSave}
          onDelete={mockOnDelete}
          initialData={mockCustomerData}
          mode="edit"
        />
      )

      fireEvent.click(screen.getByText('ลบ'))
      fireEvent.click(screen.getByText('ใช่'))

      await waitFor(() => {
        expect(mockOnClose).toHaveBeenCalled()
      })
    })

    test('shows error on delete failure', async () => {
      mockOnDelete.mockRejectedValueOnce(new Error('Delete failed'))

      render(
        <CustomerForm
          isOpen={true}
          onClose={mockOnClose}
          onSave={mockOnSave}
          onDelete={mockOnDelete}
          initialData={mockCustomerData}
          mode="edit"
        />
      )

      fireEvent.click(screen.getByText('ลบ'))
      fireEvent.click(screen.getByText('ใช่'))

      await waitFor(() => {
        expect(screen.getByText('Delete failed')).toBeInTheDocument()
      })

      expect(mockOnClose).not.toHaveBeenCalled()
    })

    test('disables confirm button while deleting', async () => {
      let resolvePromise: () => void
      mockOnDelete.mockImplementationOnce(
        () =>
          new Promise<void>((resolve) => {
            resolvePromise = resolve
          })
      )

      render(
        <CustomerForm
          isOpen={true}
          onClose={mockOnClose}
          onSave={mockOnSave}
          onDelete={mockOnDelete}
          initialData={mockCustomerData}
          mode="edit"
        />
      )

      fireEvent.click(screen.getByText('ลบ'))
      const confirmButton = screen.getByText('ใช่')
      fireEvent.click(confirmButton)

      await waitFor(() => {
        expect(confirmButton).toBeDisabled()
      })

      // Resolve to clean up
      resolvePromise!()
    })
  })

  describe('Form Reset', () => {
    test('resets form when modal closes and reopens', () => {
      const { rerender } = render(
        <CustomerForm
          isOpen={true}
          onClose={mockOnClose}
          onSave={mockOnSave}
          mode="create"
        />
      )

      fireEvent.change(screen.getByPlaceholderText('กรอกชื่อ'), {
        target: { value: 'Temporary', name: 'firstName' },
      })

      // Close the modal
      rerender(
        <CustomerForm
          isOpen={false}
          onClose={mockOnClose}
          onSave={mockOnSave}
          mode="create"
        />
      )

      // Reopen the modal
      rerender(
        <CustomerForm
          isOpen={true}
          onClose={mockOnClose}
          onSave={mockOnSave}
          mode="create"
        />
      )

      expect(screen.getByPlaceholderText('กรอกชื่อ')).toHaveValue('')
    })

    test('resets error when modal reopens', async () => {
      mockOnSave.mockRejectedValueOnce(new Error('Error'))

      const { rerender } = render(
        <CustomerForm
          isOpen={true}
          onClose={mockOnClose}
          onSave={mockOnSave}
          mode="create"
        />
      )

      fireEvent.change(screen.getByPlaceholderText('กรอกชื่อ'), {
        target: { value: 'Jane', name: 'firstName' },
      })

      fireEvent.click(screen.getByText('เพิ่มลูกค้า'))

      await waitFor(() => {
        expect(screen.getByText('Error')).toBeInTheDocument()
      })

      // Close and reopen
      rerender(
        <CustomerForm
          isOpen={false}
          onClose={mockOnClose}
          onSave={mockOnSave}
          mode="create"
        />
      )

      rerender(
        <CustomerForm
          isOpen={true}
          onClose={mockOnClose}
          onSave={mockOnSave}
          mode="create"
        />
      )

      expect(screen.queryByText('Error')).not.toBeInTheDocument()
    })
  })
})
