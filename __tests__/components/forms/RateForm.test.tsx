/**
 * @jest-environment jsdom
 */

import { render, screen, fireEvent, waitFor } from '@testing-library/react'
import RateForm, { RateFormData } from '@/components/forms/RateForm'

// Mock lucide-react icons
jest.mock('lucide-react', () => ({
  X: () => <span data-testid="icon-x">X</span>,
  Percent: () => <span data-testid="icon-percent">Percent</span>,
  Loader2: () => <span data-testid="icon-loader">Loading</span>,
  AlertCircle: () => <span data-testid="icon-alert">Alert</span>,
  Save: () => <span data-testid="icon-save">Save</span>,
  Trash2: () => <span data-testid="icon-trash">Trash</span>,
  Tag: () => <span data-testid="icon-tag">Tag</span>,
  Calendar: () => <span data-testid="icon-calendar">Calendar</span>,
  DollarSign: () => <span data-testid="icon-dollar">Dollar</span>,
  ToggleLeft: () => <span data-testid="icon-toggle">Toggle</span>,
  Home: () => <span data-testid="icon-home">Home</span>,
}))

const mockRoomTypes = [
  { id: 1, typeCode: 'STD', typeName: 'Standard', basePrice: 1000 },
  { id: 2, typeCode: 'DLX', typeName: 'Deluxe', basePrice: 2000 },
  { id: 3, typeCode: 'SUI', typeName: 'Suite', basePrice: 3500 },
]

const mockRateData: RateFormData = {
  id: 1,
  rateName: 'Weekend Special',
  roomTypeId: 2,
  rateType: 'multiplier',
  rateValue: 0.8,
  validFrom: '2026-02-01',
  validTo: '2026-02-28',
  daysOfWeek: [0, 6],
  active: true,
}

describe('RateForm Component', () => {
  const mockOnClose = jest.fn()
  const mockOnSave = jest.fn()
  const mockOnDelete = jest.fn()

  beforeEach(() => {
    jest.clearAllMocks()
  })

  describe('Modal Behavior', () => {
    test('does not render when isOpen is false', () => {
      render(
        <RateForm
          isOpen={false}
          onClose={mockOnClose}
          onSave={mockOnSave}
          roomTypes={mockRoomTypes}
        />
      )

      expect(screen.queryByText('เพิ่มอัตราพิเศษ')).not.toBeInTheDocument()
    })

    test('renders when isOpen is true', () => {
      render(
        <RateForm
          isOpen={true}
          onClose={mockOnClose}
          onSave={mockOnSave}
          roomTypes={mockRoomTypes}
        />
      )

      expect(screen.getByText('เพิ่มอัตราพิเศษ')).toBeInTheDocument()
    })

    test('calls onClose when close button is clicked', () => {
      render(
        <RateForm
          isOpen={true}
          onClose={mockOnClose}
          onSave={mockOnSave}
          roomTypes={mockRoomTypes}
        />
      )

      const closeButton = screen.getByLabelText('ปิด')
      fireEvent.click(closeButton)

      expect(mockOnClose).toHaveBeenCalledTimes(1)
    })

    test('calls onClose when cancel button is clicked', () => {
      render(
        <RateForm
          isOpen={true}
          onClose={mockOnClose}
          onSave={mockOnSave}
          roomTypes={mockRoomTypes}
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
        <RateForm
          isOpen={true}
          onClose={mockOnClose}
          onSave={mockOnSave}
          roomTypes={mockRoomTypes}
        />
      )

      expect(screen.getByText('เพิ่มอัตราพิเศษ')).toBeInTheDocument()
    })

    test('displays create button text', () => {
      render(
        <RateForm
          isOpen={true}
          onClose={mockOnClose}
          onSave={mockOnSave}
          roomTypes={mockRoomTypes}
        />
      )

      expect(screen.getByText('เพิ่มอัตรา')).toBeInTheDocument()
    })

    test('does not show delete button in create mode', () => {
      render(
        <RateForm
          isOpen={true}
          onClose={mockOnClose}
          onSave={mockOnSave}
          onDelete={mockOnDelete}
          roomTypes={mockRoomTypes}
        />
      )

      expect(screen.queryByText('ลบ')).not.toBeInTheDocument()
    })

    test('sets default dates for new rate', () => {
      render(
        <RateForm
          isOpen={true}
          onClose={mockOnClose}
          onSave={mockOnSave}
          roomTypes={mockRoomTypes}
        />
      )

      // Find date inputs by name attribute
      const validFromInput = document.querySelector('input[name="validFrom"]') as HTMLInputElement
      const validToInput = document.querySelector('input[name="validTo"]') as HTMLInputElement

      // Verify dates are set and follow YYYY-MM-DD format
      expect(validFromInput.value).toMatch(/^\d{4}-\d{2}-\d{2}$/)
      expect(validToInput.value).toMatch(/^\d{4}-\d{2}-\d{2}$/)

      // validTo should be after validFrom
      expect(new Date(validToInput.value).getTime()).toBeGreaterThan(
        new Date(validFromInput.value).getTime()
      )
    })
  })

  describe('Edit Mode', () => {
    test('displays edit mode title', () => {
      render(
        <RateForm
          isOpen={true}
          onClose={mockOnClose}
          onSave={mockOnSave}
          rate={mockRateData}
          roomTypes={mockRoomTypes}
        />
      )

      expect(screen.getByText('แก้ไขอัตราพิเศษ')).toBeInTheDocument()
    })

    test('displays save button text', () => {
      render(
        <RateForm
          isOpen={true}
          onClose={mockOnClose}
          onSave={mockOnSave}
          rate={mockRateData}
          roomTypes={mockRoomTypes}
        />
      )

      expect(screen.getByText('บันทึก')).toBeInTheDocument()
    })

    test('populates form with initial data', () => {
      render(
        <RateForm
          isOpen={true}
          onClose={mockOnClose}
          onSave={mockOnSave}
          rate={mockRateData}
          roomTypes={mockRoomTypes}
        />
      )

      expect(screen.getByDisplayValue('Weekend Special')).toBeInTheDocument()
      expect(screen.getByDisplayValue('0.8')).toBeInTheDocument()
    })

    test('shows delete button in edit mode with onDelete', () => {
      render(
        <RateForm
          isOpen={true}
          onClose={mockOnClose}
          onSave={mockOnSave}
          onDelete={mockOnDelete}
          rate={mockRateData}
          roomTypes={mockRoomTypes}
        />
      )

      expect(screen.getByText('ลบ')).toBeInTheDocument()
    })
  })

  describe('Form Field Rendering', () => {
    test('renders all form fields', () => {
      render(
        <RateForm
          isOpen={true}
          onClose={mockOnClose}
          onSave={mockOnSave}
          roomTypes={mockRoomTypes}
        />
      )

      expect(screen.getByPlaceholderText('เช่น Weekend Special, Low Season')).toBeInTheDocument()
      expect(screen.getByText('-- เลือกประเภทห้อง --')).toBeInTheDocument()
      expect(screen.getByText('ตัวคูณ (Multiplier)')).toBeInTheDocument()
      expect(screen.getByText('ราคาคงที่ (Fixed Price)')).toBeInTheDocument()
    })

    test('renders room type dropdown with options', () => {
      render(
        <RateForm
          isOpen={true}
          onClose={mockOnClose}
          onSave={mockOnSave}
          roomTypes={mockRoomTypes}
        />
      )

      expect(screen.getByText('Standard (STD) - 1,000 บาท')).toBeInTheDocument()
      expect(screen.getByText('Deluxe (DLX) - 2,000 บาท')).toBeInTheDocument()
      expect(screen.getByText('Suite (SUI) - 3,500 บาท')).toBeInTheDocument()
    })

    test('renders rate type radio buttons', () => {
      render(
        <RateForm
          isOpen={true}
          onClose={mockOnClose}
          onSave={mockOnSave}
          roomTypes={mockRoomTypes}
        />
      )

      const multiplierRadio = screen.getByRole('radio', { name: /ตัวคูณ/i })
      const fixedRadio = screen.getByRole('radio', { name: /ราคาคงที่/i })

      expect(multiplierRadio).toBeInTheDocument()
      expect(fixedRadio).toBeInTheDocument()
      expect(multiplierRadio).toBeChecked() // Default
    })

    test('renders days of week buttons', () => {
      render(
        <RateForm
          isOpen={true}
          onClose={mockOnClose}
          onSave={mockOnSave}
          roomTypes={mockRoomTypes}
        />
      )

      // Use title attribute to find day buttons precisely
      expect(screen.getByTitle('อาทิตย์')).toBeInTheDocument()
      expect(screen.getByTitle('จันทร์')).toBeInTheDocument()
      expect(screen.getByTitle('อังคาร')).toBeInTheDocument()
      expect(screen.getByTitle('พุธ')).toBeInTheDocument()
      expect(screen.getByTitle('พฤหัสบดี')).toBeInTheDocument()
      expect(screen.getByTitle('ศุกร์')).toBeInTheDocument()
      expect(screen.getByTitle('เสาร์')).toBeInTheDocument()
    })

    test('renders quick day selection buttons', () => {
      render(
        <RateForm
          isOpen={true}
          onClose={mockOnClose}
          onSave={mockOnSave}
          roomTypes={mockRoomTypes}
        />
      )

      expect(screen.getByRole('button', { name: 'ทุกวัน' })).toBeInTheDocument()
      expect(screen.getByRole('button', { name: 'วันธรรมดา' })).toBeInTheDocument()
      expect(screen.getByRole('button', { name: 'วันหยุด' })).toBeInTheDocument()
    })

    test('renders active toggle', () => {
      render(
        <RateForm
          isOpen={true}
          onClose={mockOnClose}
          onSave={mockOnSave}
          roomTypes={mockRoomTypes}
        />
      )

      expect(screen.getByText('เปิดใช้งาน')).toBeInTheDocument()
      const activeCheckbox = screen.getByRole('checkbox')
      expect(activeCheckbox).toBeChecked() // Default is true
    })
  })

  describe('Rate Type Toggle', () => {
    test('switches to fixed price mode', () => {
      render(
        <RateForm
          isOpen={true}
          onClose={mockOnClose}
          onSave={mockOnSave}
          roomTypes={mockRoomTypes}
        />
      )

      const fixedRadio = screen.getByRole('radio', { name: /ราคาคงที่/i })
      fireEvent.click(fixedRadio)

      expect(fixedRadio).toBeChecked()
      // Label should change
      expect(screen.getByText(/ราคา \(บาท\)/i)).toBeInTheDocument()
    })

    test('shows multiplier hint when in multiplier mode', () => {
      render(
        <RateForm
          isOpen={true}
          onClose={mockOnClose}
          onSave={mockOnSave}
          roomTypes={mockRoomTypes}
        />
      )

      expect(screen.getByText(/เช่น 0.8 = ลด 20%, 1.2 = เพิ่ม 20%/i)).toBeInTheDocument()
    })

    test('hides multiplier hint when in fixed mode', () => {
      render(
        <RateForm
          isOpen={true}
          onClose={mockOnClose}
          onSave={mockOnSave}
          roomTypes={mockRoomTypes}
        />
      )

      const fixedRadio = screen.getByRole('radio', { name: /ราคาคงที่/i })
      fireEvent.click(fixedRadio)

      expect(screen.queryByText(/เช่น 0.8 = ลด 20%/i)).not.toBeInTheDocument()
    })
  })

  describe('Days of Week Selection', () => {
    test('toggles individual days', () => {
      render(
        <RateForm
          isOpen={true}
          onClose={mockOnClose}
          onSave={mockOnSave}
          roomTypes={mockRoomTypes}
        />
      )

      // By default all days are selected
      const sundayButton = screen.getByTitle('อาทิตย์')
      expect(sundayButton).toHaveClass('bg-amber-500')

      // Click to deselect
      fireEvent.click(sundayButton)
      expect(sundayButton).not.toHaveClass('bg-amber-500')

      // Click to reselect
      fireEvent.click(sundayButton)
      expect(sundayButton).toHaveClass('bg-amber-500')
    })

    test('selects weekdays only when button clicked', () => {
      render(
        <RateForm
          isOpen={true}
          onClose={mockOnClose}
          onSave={mockOnSave}
          roomTypes={mockRoomTypes}
        />
      )

      const weekdaysButton = screen.getByRole('button', { name: 'วันธรรมดา' })
      fireEvent.click(weekdaysButton)

      // Check weekdays are selected (Mon-Fri)
      const mondayButton = screen.getByTitle('จันทร์')
      const fridayButton = screen.getByTitle('ศุกร์')
      expect(mondayButton).toHaveClass('bg-amber-500')
      expect(fridayButton).toHaveClass('bg-amber-500')

      // Check weekends are not selected
      const sundayButton = screen.getByTitle('อาทิตย์')
      const saturdayButton = screen.getByTitle('เสาร์')
      expect(sundayButton).not.toHaveClass('bg-amber-500')
      expect(saturdayButton).not.toHaveClass('bg-amber-500')
    })

    test('selects weekends only when button clicked', () => {
      render(
        <RateForm
          isOpen={true}
          onClose={mockOnClose}
          onSave={mockOnSave}
          roomTypes={mockRoomTypes}
        />
      )

      const weekendsButton = screen.getByRole('button', { name: 'วันหยุด' })
      fireEvent.click(weekendsButton)

      // Check weekends are selected
      const sundayButton = screen.getByTitle('อาทิตย์')
      const saturdayButton = screen.getByTitle('เสาร์')
      expect(sundayButton).toHaveClass('bg-amber-500')
      expect(saturdayButton).toHaveClass('bg-amber-500')

      // Check weekdays are not selected
      const mondayButton = screen.getByTitle('จันทร์')
      expect(mondayButton).not.toHaveClass('bg-amber-500')
    })

    test('selects all days when button clicked', () => {
      render(
        <RateForm
          isOpen={true}
          onClose={mockOnClose}
          onSave={mockOnSave}
          roomTypes={mockRoomTypes}
        />
      )

      // First deselect some
      const weekendsButton = screen.getByRole('button', { name: 'วันหยุด' })
      fireEvent.click(weekendsButton)

      // Then select all
      const allDaysButton = screen.getByRole('button', { name: 'ทุกวัน' })
      fireEvent.click(allDaysButton)

      // All days should be selected
      const sundayButton = screen.getByTitle('อาทิตย์')
      const mondayButton = screen.getByTitle('จันทร์')
      const saturdayButton = screen.getByTitle('เสาร์')
      expect(sundayButton).toHaveClass('bg-amber-500')
      expect(mondayButton).toHaveClass('bg-amber-500')
      expect(saturdayButton).toHaveClass('bg-amber-500')
    })
  })

  describe('Price Preview', () => {
    test('shows price preview when room type is selected', () => {
      render(
        <RateForm
          isOpen={true}
          onClose={mockOnClose}
          onSave={mockOnSave}
          roomTypes={mockRoomTypes}
        />
      )

      const roomTypeSelect = document.querySelector('select[name="roomTypeId"]') as HTMLSelectElement
      fireEvent.change(roomTypeSelect, { target: { value: '2', name: 'roomTypeId' } })

      // Default multiplier is 1.0, so effective = base = 2000
      expect(screen.getByText('ตัวอย่างราคา:')).toBeInTheDocument()
      // Both base price and effective price show 2000 when multiplier is 1.0
      const priceTexts = screen.getAllByText('2,000 บาท')
      expect(priceTexts.length).toBeGreaterThanOrEqual(1)
    })

    test('shows discount badge when multiplier is less than 1', () => {
      render(
        <RateForm
          isOpen={true}
          onClose={mockOnClose}
          onSave={mockOnSave}
          rate={mockRateData} // multiplier 0.8
          roomTypes={mockRoomTypes}
        />
      )

      // Deluxe base price 2000, multiplier 0.8 = 1600
      expect(screen.getByText('1,600 บาท')).toBeInTheDocument()
      expect(screen.getByText('-20%')).toBeInTheDocument()
    })

    test('updates preview when changing rate value', () => {
      render(
        <RateForm
          isOpen={true}
          onClose={mockOnClose}
          onSave={mockOnSave}
          roomTypes={mockRoomTypes}
        />
      )

      // Select room type first
      const roomTypeSelect = document.querySelector('select[name="roomTypeId"]') as HTMLSelectElement
      fireEvent.change(roomTypeSelect, { target: { value: '1', name: 'roomTypeId' } })

      // Change multiplier to 0.9
      const rateValueInput = document.querySelector('input[name="rateValue"]') as HTMLInputElement
      fireEvent.change(rateValueInput, { target: { value: '0.9', name: 'rateValue' } })

      // Standard 1000 * 0.9 = 900
      expect(screen.getByText('900 บาท')).toBeInTheDocument()
    })

    test('shows fixed price in preview when rate type is fixed', () => {
      render(
        <RateForm
          isOpen={true}
          onClose={mockOnClose}
          onSave={mockOnSave}
          roomTypes={mockRoomTypes}
        />
      )

      // Select room type
      const roomTypeSelect = document.querySelector('select[name="roomTypeId"]') as HTMLSelectElement
      fireEvent.change(roomTypeSelect, { target: { value: '1', name: 'roomTypeId' } })

      // Switch to fixed price
      const fixedRadio = screen.getByRole('radio', { name: /ราคาคงที่/i })
      fireEvent.click(fixedRadio)

      // Set fixed price
      const rateValueInput = document.querySelector('input[name="rateValue"]') as HTMLInputElement
      fireEvent.change(rateValueInput, { target: { value: '800', name: 'rateValue' } })

      expect(screen.getByText('800 บาท')).toBeInTheDocument()
    })
  })

  describe('Form Validation', () => {
    test('shows error when rate name is whitespace only', async () => {
      render(
        <RateForm
          isOpen={true}
          onClose={mockOnClose}
          onSave={mockOnSave}
          roomTypes={mockRoomTypes}
        />
      )

      // Use whitespace to bypass HTML5 required validation
      fireEvent.change(screen.getByPlaceholderText('เช่น Weekend Special, Low Season'), {
        target: { value: '   ', name: 'rateName' },
      })

      const form = document.querySelector('form')!
      fireEvent.submit(form)

      await waitFor(() => {
        expect(screen.getByText('กรุณากรอกชื่ออัตรา')).toBeInTheDocument()
      })

      expect(mockOnSave).not.toHaveBeenCalled()
    })

    test('shows error when room type is not selected', async () => {
      render(
        <RateForm
          isOpen={true}
          onClose={mockOnClose}
          onSave={mockOnSave}
          roomTypes={mockRoomTypes}
        />
      )

      fireEvent.change(screen.getByPlaceholderText('เช่น Weekend Special, Low Season'), {
        target: { value: 'Test Rate', name: 'rateName' },
      })

      const form = document.querySelector('form')!
      fireEvent.submit(form)

      await waitFor(() => {
        expect(screen.getByText('กรุณาเลือกประเภทห้อง')).toBeInTheDocument()
      })
    })

    test('shows error when dates are invalid', async () => {
      render(
        <RateForm
          isOpen={true}
          onClose={mockOnClose}
          onSave={mockOnSave}
          roomTypes={mockRoomTypes}
        />
      )

      fireEvent.change(screen.getByPlaceholderText('เช่น Weekend Special, Low Season'), {
        target: { value: 'Test Rate', name: 'rateName' },
      })

      const roomTypeSelect = document.querySelector('select[name="roomTypeId"]') as HTMLSelectElement
      fireEvent.change(roomTypeSelect, { target: { value: '1', name: 'roomTypeId' } })

      // Set invalid date range (from > to)
      const validFromInput = document.querySelector('input[name="validFrom"]') as HTMLInputElement
      const validToInput = document.querySelector('input[name="validTo"]') as HTMLInputElement

      fireEvent.change(validFromInput, { target: { value: '2026-03-01', name: 'validFrom' } })
      fireEvent.change(validToInput, { target: { value: '2026-02-01', name: 'validTo' } })

      const form = document.querySelector('form')!
      fireEvent.submit(form)

      await waitFor(() => {
        expect(screen.getByText('วันที่เริ่มต้นต้องก่อนวันที่สิ้นสุด')).toBeInTheDocument()
      })
    })

    test('shows error when multiplier is out of range', async () => {
      render(
        <RateForm
          isOpen={true}
          onClose={mockOnClose}
          onSave={mockOnSave}
          roomTypes={mockRoomTypes}
        />
      )

      fireEvent.change(screen.getByPlaceholderText('เช่น Weekend Special, Low Season'), {
        target: { value: 'Test Rate', name: 'rateName' },
      })

      const roomTypeSelect = document.querySelector('select[name="roomTypeId"]') as HTMLSelectElement
      fireEvent.change(roomTypeSelect, { target: { value: '1', name: 'roomTypeId' } })

      const rateValueInput = document.querySelector('input[name="rateValue"]') as HTMLInputElement
      fireEvent.change(rateValueInput, { target: { value: '3.0', name: 'rateValue' } })

      const form = document.querySelector('form')!
      fireEvent.submit(form)

      await waitFor(() => {
        expect(screen.getByText('ตัวคูณต้องอยู่ระหว่าง 0.1 ถึง 2.0')).toBeInTheDocument()
      })
    })

    test('shows error when fixed price is zero or negative', async () => {
      render(
        <RateForm
          isOpen={true}
          onClose={mockOnClose}
          onSave={mockOnSave}
          roomTypes={mockRoomTypes}
        />
      )

      fireEvent.change(screen.getByPlaceholderText('เช่น Weekend Special, Low Season'), {
        target: { value: 'Test Rate', name: 'rateName' },
      })

      const roomTypeSelect = document.querySelector('select[name="roomTypeId"]') as HTMLSelectElement
      fireEvent.change(roomTypeSelect, { target: { value: '1', name: 'roomTypeId' } })

      // Switch to fixed price
      const fixedRadio = screen.getByRole('radio', { name: /ราคาคงที่/i })
      fireEvent.click(fixedRadio)

      const rateValueInput = document.querySelector('input[name="rateValue"]') as HTMLInputElement
      fireEvent.change(rateValueInput, { target: { value: '0', name: 'rateValue' } })

      const form = document.querySelector('form')!
      fireEvent.submit(form)

      await waitFor(() => {
        expect(screen.getByText('ราคาคงที่ต้องมากกว่า 0')).toBeInTheDocument()
      })
    })

    test('shows error when no days are selected', async () => {
      render(
        <RateForm
          isOpen={true}
          onClose={mockOnClose}
          onSave={mockOnSave}
          roomTypes={mockRoomTypes}
        />
      )

      fireEvent.change(screen.getByPlaceholderText('เช่น Weekend Special, Low Season'), {
        target: { value: 'Test Rate', name: 'rateName' },
      })

      const roomTypeSelect = document.querySelector('select[name="roomTypeId"]') as HTMLSelectElement
      fireEvent.change(roomTypeSelect, { target: { value: '1', name: 'roomTypeId' } })

      // Deselect all days
      const allDayButtons = [
        screen.getByTitle('อาทิตย์'),
        screen.getByTitle('จันทร์'),
        screen.getByTitle('อังคาร'),
        screen.getByTitle('พุธ'),
        screen.getByTitle('พฤหัสบดี'),
        screen.getByTitle('ศุกร์'),
        screen.getByTitle('เสาร์'),
      ]

      allDayButtons.forEach((button) => fireEvent.click(button))

      const form = document.querySelector('form')!
      fireEvent.submit(form)

      await waitFor(() => {
        expect(screen.getByText('กรุณาเลือกวันที่มีผลอย่างน้อย 1 วัน')).toBeInTheDocument()
      })
    })

    test('required fields have required attribute', () => {
      render(
        <RateForm
          isOpen={true}
          onClose={mockOnClose}
          onSave={mockOnSave}
          roomTypes={mockRoomTypes}
        />
      )

      expect(screen.getByPlaceholderText('เช่น Weekend Special, Low Season')).toHaveAttribute('required')
    })
  })

  describe('Form Submission', () => {
    test('calls onSave with correct data on valid submission', async () => {
      mockOnSave.mockResolvedValueOnce(undefined)

      render(
        <RateForm
          isOpen={true}
          onClose={mockOnClose}
          onSave={mockOnSave}
          roomTypes={mockRoomTypes}
        />
      )

      fireEvent.change(screen.getByPlaceholderText('เช่น Weekend Special, Low Season'), {
        target: { value: 'Test Rate', name: 'rateName' },
      })

      const roomTypeSelect = document.querySelector('select[name="roomTypeId"]') as HTMLSelectElement
      fireEvent.change(roomTypeSelect, { target: { value: '1', name: 'roomTypeId' } })

      const rateValueInput = document.querySelector('input[name="rateValue"]') as HTMLInputElement
      fireEvent.change(rateValueInput, { target: { value: '0.9', name: 'rateValue' } })

      const submitButton = screen.getByText('เพิ่มอัตรา')
      fireEvent.click(submitButton)

      await waitFor(() => {
        expect(mockOnSave).toHaveBeenCalledWith(
          expect.objectContaining({
            rateName: 'Test Rate',
            roomTypeId: 1,
            rateType: 'multiplier',
            rateValue: 0.9,
          })
        )
      })
    })

    test('calls onClose after successful save', async () => {
      mockOnSave.mockResolvedValueOnce(undefined)

      render(
        <RateForm
          isOpen={true}
          onClose={mockOnClose}
          onSave={mockOnSave}
          roomTypes={mockRoomTypes}
        />
      )

      fireEvent.change(screen.getByPlaceholderText('เช่น Weekend Special, Low Season'), {
        target: { value: 'Test Rate', name: 'rateName' },
      })

      const roomTypeSelect = document.querySelector('select[name="roomTypeId"]') as HTMLSelectElement
      fireEvent.change(roomTypeSelect, { target: { value: '1', name: 'roomTypeId' } })

      const submitButton = screen.getByText('เพิ่มอัตรา')
      fireEvent.click(submitButton)

      await waitFor(() => {
        expect(mockOnClose).toHaveBeenCalled()
      })
    })

    test('shows error message on save failure', async () => {
      mockOnSave.mockRejectedValueOnce(new Error('Database error'))

      render(
        <RateForm
          isOpen={true}
          onClose={mockOnClose}
          onSave={mockOnSave}
          roomTypes={mockRoomTypes}
        />
      )

      fireEvent.change(screen.getByPlaceholderText('เช่น Weekend Special, Low Season'), {
        target: { value: 'Test Rate', name: 'rateName' },
      })

      const roomTypeSelect = document.querySelector('select[name="roomTypeId"]') as HTMLSelectElement
      fireEvent.change(roomTypeSelect, { target: { value: '1', name: 'roomTypeId' } })

      const submitButton = screen.getByText('เพิ่มอัตรา')
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
        <RateForm
          isOpen={true}
          onClose={mockOnClose}
          onSave={mockOnSave}
          roomTypes={mockRoomTypes}
        />
      )

      fireEvent.change(screen.getByPlaceholderText('เช่น Weekend Special, Low Season'), {
        target: { value: 'Test Rate', name: 'rateName' },
      })

      const roomTypeSelect = document.querySelector('select[name="roomTypeId"]') as HTMLSelectElement
      fireEvent.change(roomTypeSelect, { target: { value: '1', name: 'roomTypeId' } })

      const submitButton = screen.getByRole('button', { name: /เพิ่มอัตรา/i })
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
        <RateForm
          isOpen={true}
          onClose={mockOnClose}
          onSave={mockOnSave}
          roomTypes={mockRoomTypes}
        />
      )

      fireEvent.change(screen.getByPlaceholderText('เช่น Weekend Special, Low Season'), {
        target: { value: 'Test Rate', name: 'rateName' },
      })

      const roomTypeSelect = document.querySelector('select[name="roomTypeId"]') as HTMLSelectElement
      fireEvent.change(roomTypeSelect, { target: { value: '1', name: 'roomTypeId' } })

      const submitButton = screen.getByText('เพิ่มอัตรา')
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
        <RateForm
          isOpen={true}
          onClose={mockOnClose}
          onSave={mockOnSave}
          onDelete={mockOnDelete}
          rate={mockRateData}
          roomTypes={mockRoomTypes}
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
        <RateForm
          isOpen={true}
          onClose={mockOnClose}
          onSave={mockOnSave}
          onDelete={mockOnDelete}
          rate={mockRateData}
          roomTypes={mockRoomTypes}
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
        <RateForm
          isOpen={true}
          onClose={mockOnClose}
          onSave={mockOnSave}
          onDelete={mockOnDelete}
          rate={mockRateData}
          roomTypes={mockRoomTypes}
        />
      )

      fireEvent.click(screen.getByText('ลบ'))
      fireEvent.click(screen.getByText('ไม่'))

      expect(screen.queryByText('ยืนยันการลบ?')).not.toBeInTheDocument()
    })

    test('shows error on delete failure', async () => {
      mockOnDelete.mockRejectedValueOnce(new Error('Cannot delete active rate'))

      render(
        <RateForm
          isOpen={true}
          onClose={mockOnClose}
          onSave={mockOnSave}
          onDelete={mockOnDelete}
          rate={mockRateData}
          roomTypes={mockRoomTypes}
        />
      )

      fireEvent.click(screen.getByText('ลบ'))
      fireEvent.click(screen.getByText('ใช่'))

      await waitFor(() => {
        expect(screen.getByText('Cannot delete active rate')).toBeInTheDocument()
      })
    })
  })

  describe('Buddhist Year Display', () => {
    test('displays Buddhist year for dates', () => {
      render(
        <RateForm
          isOpen={true}
          onClose={mockOnClose}
          onSave={mockOnSave}
          rate={mockRateData}
          roomTypes={mockRoomTypes}
        />
      )

      // 2026 + 543 = 2569
      expect(screen.getByText(/พ\.ศ\. 01\/02\/2569/)).toBeInTheDocument()
      expect(screen.getByText(/พ\.ศ\. 28\/02\/2569/)).toBeInTheDocument()
    })
  })
})
