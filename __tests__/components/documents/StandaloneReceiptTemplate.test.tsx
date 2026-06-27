/**
 * @jest-environment jsdom
 */

import '@testing-library/jest-dom'
import { render, screen } from '@testing-library/react'
import StandaloneReceiptTemplate from '@/components/documents/StandaloneReceiptTemplate'
import { StandaloneReceiptData, HotelInfo } from '@/types/invoice'

const createMockReceipt = (
  overrides: Partial<StandaloneReceiptData> = {},
): StandaloneReceiptData => ({
  receiptId: 42,
  lines: [
    { productNo: 'B-001', name: 'Coca-Cola 330ml', unit: 'ขวด', qty: 2, unitPrice: 25, discount: 0, total: 50 },
    { productNo: 'S-009', name: "Lay's", unit: 'ถุง', qty: 1, unitPrice: 20, discount: 0, total: 20 },
  ],
  subtotal: 70,
  discount: 0,
  vatAmount: 0,
  vatPercent: 0,
  grandTotal: 70,
  paymentMethod: 'cash',
  paidAmount: 70,
  createdAt: '2026-06-27T05:00:00.000Z',
  ...overrides,
})

const createMockHotelInfo = (overrides: Partial<HotelInfo> = {}): HotelInfo => ({
  name: 'HF Hotel',
  address: '33 ถนนชนเกษม สุราษฎร์ธานี',
  phone: '077313808',
  taxId: '0845557000341',
  ...overrides,
})

describe('StandaloneReceiptTemplate', () => {
  const defaultProps = {
    receiptData: createMockReceipt(),
    hotelInfo: createMockHotelInfo(),
  }

  test('renders the hotel name', () => {
    render(<StandaloneReceiptTemplate {...defaultProps} />)
    expect(screen.getByText('HF Hotel')).toBeInTheDocument()
  })

  test('falls back to the canonical receipt id when no receipt number', () => {
    render(<StandaloneReceiptTemplate {...defaultProps} />)
    expect(screen.getByText('#42')).toBeInTheDocument()
  })

  test('shows the allocated receipt number when present', () => {
    render(
      <StandaloneReceiptTemplate
        receiptData={createMockReceipt({ receiptNumber: 'B2606-0042' })}
        hotelInfo={createMockHotelInfo()}
      />,
    )
    expect(screen.getByText('B2606-0042')).toBeInTheDocument()
  })

  test('renders each product line', () => {
    render(<StandaloneReceiptTemplate {...defaultProps} />)
    expect(screen.getByText('Coca-Cola 330ml')).toBeInTheDocument()
    expect(screen.getByText("Lay's")).toBeInTheDocument()
  })

  test('hides the customer block for an anonymous walk-up', () => {
    render(<StandaloneReceiptTemplate {...defaultProps} />)
    expect(screen.queryByText('ข้อมูลลูกค้า / Customer')).not.toBeInTheDocument()
  })

  test('shows the customer block + tax id when provided', () => {
    render(
      <StandaloneReceiptTemplate
        receiptData={createMockReceipt({ customerName: 'ACME Co', customerTaxId: '0105500000001' })}
        hotelInfo={createMockHotelInfo()}
      />,
    )
    expect(screen.getByText('ACME Co')).toBeInTheDocument()
    expect(screen.getByText('0105500000001')).toBeInTheDocument()
  })

  test('renders the VAT line only when there is VAT', () => {
    const { rerender } = render(<StandaloneReceiptTemplate {...defaultProps} />)
    expect(screen.queryByText(/ภาษีมูลค่าเพิ่ม/)).not.toBeInTheDocument()

    rerender(
      <StandaloneReceiptTemplate
        receiptData={createMockReceipt({ vatAmount: 7, vatPercent: 7, grandTotal: 107, paidAmount: 107, subtotal: 107 })}
        hotelInfo={createMockHotelInfo()}
      />,
    )
    expect(screen.getByText(/ภาษีมูลค่าเพิ่ม 7%/)).toBeInTheDocument()
  })

  test('marks the receipt paid in full when paid >= total', () => {
    render(<StandaloneReceiptTemplate {...defaultProps} />)
    expect(screen.getByText(/PAID IN FULL/)).toBeInTheDocument()
  })
})
