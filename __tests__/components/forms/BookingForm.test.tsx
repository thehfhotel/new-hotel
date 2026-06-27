/**
 * @jest-environment jsdom
 */

import { render, screen, fireEvent, waitFor } from '@testing-library/react'
import BookingForm, { BookingFormState } from '@/components/forms/BookingForm'

// lucide-react: stub every icon to a no-op component via a Proxy so we don't
// have to enumerate the (growing) icon set this form imports.
jest.mock('lucide-react', () =>
  new Proxy(
    {},
    {
      get: (_t, prop) => (prop === '__esModule' ? true : () => null),
    }
  )
)

// Heavy children are exercised in their own suites; stub them so these tests
// stay focused on BookingForm's waitlist / pre-order / confirmation-slip logic.
jest.mock('@/components/pickers/CustomerPicker', () => ({
  __esModule: true,
  default: ({ onChange }: { onChange: (c: unknown) => void }) => (
    <button
      type="button"
      data-testid="pick-customer"
      onClick={() =>
        onChange({
          id: 7,
          firstName: 'สมชาย',
          lastName: 'ใจดี',
          phone: '0812345678',
          email: '',
          idCard: '',
        })
      }
    >
      pick-customer
    </button>
  ),
}))

jest.mock('@/components/pickers/RoomPicker', () => ({
  __esModule: true,
  default: ({ onChange }: { onChange: (r: unknown[]) => void }) => (
    <button
      type="button"
      data-testid="pick-room"
      onClick={() =>
        onChange([
          {
            id: 11,
            roomNo: '101',
            roomTypeId: 1,
            roomTypeName: 'Standard',
            floor: 1,
            status: 'available',
            priceWeekday: 800,
            priceWeekend: 1000,
          },
        ])
      }
    >
      pick-room
    </button>
  ),
}))

jest.mock('@/components/forms/CustomerForm', () => ({
  __esModule: true,
  default: () => null,
}))

jest.mock('@/components/documents/BookingConfirmationSlip', () => ({
  __esModule: true,
  default: ({ data }: { data: { bookingNo: string } }) => (
    <div data-testid="confirmation-slip">slip:{data.bookingNo}</div>
  ),
}))

jest.mock('@/components/ui/PrintButton', () => ({
  __esModule: true,
  default: () => <button type="button">พิมพ์</button>,
}))

const mockProducts = [
  {
    id: 5,
    legacyNo: 'P005',
    name: 'น้ำดื่ม',
    unit: 'ขวด',
    price: 20,
    currentStock: 100,
    active: true,
  },
]

describe('BookingForm Component', () => {
  const defaultProps = {
    isOpen: true,
    onClose: jest.fn(),
    onSave: jest.fn(),
    onCancel: jest.fn(),
    initialData: null,
    mode: 'create' as const,
  }

  beforeEach(() => {
    jest.clearAllMocks()
    // The form fetches the product catalog on open; everything else is mocked.
    global.fetch = jest.fn((input: RequestInfo | URL) => {
      const url = String(input)
      if (url.includes('/api/products')) {
        return Promise.resolve({
          ok: true,
          json: () => Promise.resolve({ success: true, data: mockProducts }),
        }) as unknown as Promise<Response>
      }
      return Promise.resolve({
        ok: true,
        json: () => Promise.resolve({ success: true }),
      }) as unknown as Promise<Response>
    }) as unknown as typeof fetch
  })

  test('does not render when closed', () => {
    render(<BookingForm {...defaultProps} isOpen={false} />)
    expect(screen.queryByText('สร้างการจองใหม่')).not.toBeInTheDocument()
  })

  test('renders create form with a waitlist hint when no room is chosen', async () => {
    render(<BookingForm {...defaultProps} />)
    expect(screen.getByText('สร้างการจองใหม่')).toBeInTheDocument()
    // Rooms are optional now — the waitlist hint is shown up-front.
    expect(screen.getByText(/ยังไม่ระบุห้องพัก/)).toBeInTheDocument()
    // Let the async product-catalog load settle inside act().
    await screen.findByText('น้ำดื่ม (20 บาท)')
  })

  test('waitlist hint disappears once a room is selected', async () => {
    render(<BookingForm {...defaultProps} />)
    expect(screen.getByText(/ยังไม่ระบุห้องพัก/)).toBeInTheDocument()
    fireEvent.click(screen.getByTestId('pick-room'))
    expect(screen.queryByText(/ยังไม่ระบุห้องพัก/)).not.toBeInTheDocument()
    await screen.findByText('น้ำดื่ม (20 บาท)')
  })

  test('loads the product catalog and adds a pre-order line', async () => {
    render(<BookingForm {...defaultProps} />)

    const select = await screen.findByLabelText('เลือกสินค้าสั่งล่วงหน้า')
    // The catalog option must have loaded from GET /api/products.
    await waitFor(() => {
      expect(screen.getByText('น้ำดื่ม (20 บาท)')).toBeInTheDocument()
    })

    fireEvent.change(select, { target: { value: '5' } })
    fireEvent.click(screen.getByRole('button', { name: /เพิ่ม/ }))

    // The added line renders its name in its own element (qty shown separately).
    expect(screen.getByText('น้ำดื่ม')).toBeInTheDocument()
  })

  test('creates a waitlist booking (no rooms) and shows the confirmation slip', async () => {
    const onSave = jest
      .fn()
      .mockResolvedValue({ id: 99, bookNo: 'BK-20260627-0001' })
    const onClose = jest.fn()

    const { container } = render(
      <BookingForm {...defaultProps} onSave={onSave} onClose={onClose} />
    )

    // Pick a customer + set the stay dates; leave rooms empty (waitlist).
    fireEvent.click(screen.getByTestId('pick-customer'))
    fireEvent.change(container.querySelector('input[name="checkIn"]')!, {
      target: { value: '2026-07-01' },
    })
    fireEvent.change(container.querySelector('input[name="checkOut"]')!, {
      target: { value: '2026-07-03' },
    })
    fireEvent.click(screen.getByRole('button', { name: 'สร้างการจอง' }))

    await waitFor(() => {
      expect(screen.getByText('สร้างการจองสำเร็จ')).toBeInTheDocument()
    })
    expect(screen.getByTestId('confirmation-slip')).toHaveTextContent(
      'BK-20260627-0001'
    )
    // Create-success keeps the modal open on the slip panel; it must NOT close.
    expect(onClose).not.toHaveBeenCalled()

    // onSave received a zero-room (waitlist) booking.
    const saved = onSave.mock.calls[0][0] as BookingFormState
    expect(saved.customerId).toBe(7)
    expect(saved.rooms).toEqual([])

    // "เสร็จสิ้น" closes the modal.
    fireEvent.click(screen.getByText('เสร็จสิ้น'))
    expect(onClose).toHaveBeenCalled()
  })
})
