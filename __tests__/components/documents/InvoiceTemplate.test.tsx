/**
 * @jest-environment jsdom
 */

import { render, screen } from '@testing-library/react'
import InvoiceTemplate from '@/components/documents/InvoiceTemplate'
import { InvoiceData, HotelInfo } from '@/types/invoice'

// Helper to create mock invoice data
const createMockInvoiceData = (overrides: Partial<InvoiceData> = {}): InvoiceData => ({
  invoiceNumber: 'INV-2569-0001',
  checkInId: 1,
  guestName: 'John Doe',
  guestIdCard: 'REDACTED-sa-pw90123',
  guestContact: '08REDACTED-sa-pw',
  checkInDate: '2026-01-20T07:00:00.000Z',
  checkOutDate: '2026-01-25T05:00:00.000Z',
  rooms: [
    {
      roomNumber: '301',
      roomType: 'Standard',
      ratePerNight: 1500,
      nights: 5,
      subtotal: 7500,
    },
  ],
  subtotal: 7500,
  discount: 0,
  vatAmount: 0,
  grandTotal: 7500,
  paymentMethod: 'cash',
  paidAmount: 7500,
  createdAt: '2026-01-25T10:00:00.000Z',
  ...overrides,
})

// Helper to create mock hotel info
const createMockHotelInfo = (overrides: Partial<HotelInfo> = {}): HotelInfo => ({
  name: 'The HF Hotel',
  address: '123 Beach Road, Pattaya, Chonburi 20150',
  phone: '038-123-4567',
  taxId: 'REDACTED-sa-pw90123',
  ...overrides,
})

describe('InvoiceTemplate Component', () => {
  const defaultProps = {
    checkInData: createMockInvoiceData(),
    hotelInfo: createMockHotelInfo(),
  }

  describe('Hotel Info Display', () => {
    test('displays hotel name', () => {
      render(<InvoiceTemplate {...defaultProps} />)

      expect(screen.getByText('The HF Hotel')).toBeInTheDocument()
    })

    test('displays hotel address', () => {
      render(<InvoiceTemplate {...defaultProps} />)

      expect(screen.getByText('123 Beach Road, Pattaya, Chonburi 20150')).toBeInTheDocument()
    })

    test('displays hotel phone number', () => {
      render(<InvoiceTemplate {...defaultProps} />)

      expect(screen.getByText(/โทร: 038-123-4567/)).toBeInTheDocument()
    })

    test('displays hotel tax ID', () => {
      render(<InvoiceTemplate {...defaultProps} />)

      expect(screen.getByText(/เลขประจำตัวผู้เสียภาษี: REDACTED-sa-pw90123/)).toBeInTheDocument()
    })

    test('displays hotel logo when provided', () => {
      const hotelInfoWithLogo = createMockHotelInfo({ logo: '/images/logo.png' })
      render(<InvoiceTemplate {...defaultProps} hotelInfo={hotelInfoWithLogo} />)

      const logo = screen.getByAltText('The HF Hotel')
      expect(logo).toBeInTheDocument()
      expect(logo).toHaveAttribute('src', '/images/logo.png')
    })

    test('does not display logo image when logo is not provided', () => {
      const hotelInfoNoLogo = createMockHotelInfo({ logo: undefined })
      render(<InvoiceTemplate {...defaultProps} hotelInfo={hotelInfoNoLogo} />)

      expect(screen.queryByRole('img')).not.toBeInTheDocument()
    })

    test('displays invoice title in Thai and English', () => {
      render(<InvoiceTemplate {...defaultProps} />)

      expect(screen.getByText('ใบแจ้งหนี้ / Invoice')).toBeInTheDocument()
    })

    test('displays invoice number', () => {
      render(<InvoiceTemplate {...defaultProps} />)

      expect(screen.getByText('INV-2569-0001')).toBeInTheDocument()
    })
  })

  describe('Guest Details', () => {
    test('displays guest name', () => {
      render(<InvoiceTemplate {...defaultProps} />)

      expect(screen.getByText('ข้อมูลผู้เข้าพัก / Guest Information')).toBeInTheDocument()
      expect(screen.getByText('John Doe')).toBeInTheDocument()
    })

    test('displays guest ID card number', () => {
      render(<InvoiceTemplate {...defaultProps} />)

      expect(screen.getByText('REDACTED-sa-pw90123')).toBeInTheDocument()
    })

    test('displays guest contact information', () => {
      render(<InvoiceTemplate {...defaultProps} />)

      expect(screen.getByText('08REDACTED-sa-pw')).toBeInTheDocument()
    })

    test('displays check-in ID', () => {
      render(<InvoiceTemplate {...defaultProps} />)

      const checkInIdLabel = screen.getByText('รหัสเช็คอิน / Check-in ID:')
      expect(checkInIdLabel.nextElementSibling).toHaveTextContent('1')
    })

    test('displays dash for missing guest name', () => {
      const invoiceNoName = createMockInvoiceData({ guestName: '' })
      render(<InvoiceTemplate {...defaultProps} checkInData={invoiceNoName} />)

      // Look for the dash in the guest name field
      const nameLabel = screen.getByText('ชื่อ-สกุล / Name:')
      expect(nameLabel.nextElementSibling).toHaveTextContent('-')
    })

    test('displays dash for missing ID card', () => {
      const invoiceNoIdCard = createMockInvoiceData({ guestIdCard: '' })
      render(<InvoiceTemplate {...defaultProps} checkInData={invoiceNoIdCard} />)

      const idCardLabel = screen.getByText('เลขบัตรประชาชน / ID Card:')
      expect(idCardLabel.nextElementSibling).toHaveTextContent('-')
    })

    test('displays dash for missing contact', () => {
      const invoiceNoContact = createMockInvoiceData({ guestContact: '' })
      render(<InvoiceTemplate {...defaultProps} checkInData={invoiceNoContact} />)

      const contactLabel = screen.getByText('ติดต่อ / Contact:')
      expect(contactLabel.nextElementSibling).toHaveTextContent('-')
    })
  })

  describe('Stay Period Display', () => {
    test('displays check-in date in Thai short format with Buddhist Era', () => {
      render(<InvoiceTemplate {...defaultProps} />)

      // 20 Jan 2026 = 20/01/2569 (Buddhist Era)
      expect(screen.getByText('20/01/2569')).toBeInTheDocument()
    })

    test('displays check-out date in Thai short format with Buddhist Era', () => {
      render(<InvoiceTemplate {...defaultProps} />)

      // 25 Jan 2026 = 25/01/2569 (Buddhist Era)
      expect(screen.getByText('25/01/2569')).toBeInTheDocument()
    })

    test('displays stay period labels', () => {
      render(<InvoiceTemplate {...defaultProps} />)

      expect(screen.getByText('วันที่เข้าพัก / Check-in:')).toBeInTheDocument()
      expect(screen.getByText('วันที่ออก / Check-out:')).toBeInTheDocument()
    })
  })

  describe('Room Charges Table', () => {
    test('displays room charges header', () => {
      render(<InvoiceTemplate {...defaultProps} />)

      expect(screen.getByText('รายละเอียดค่าห้องพัก / Room Charges')).toBeInTheDocument()
    })

    test('displays table headers', () => {
      render(<InvoiceTemplate {...defaultProps} />)

      expect(screen.getByText('ห้อง / Room')).toBeInTheDocument()
      expect(screen.getByText('ประเภท / Type')).toBeInTheDocument()
      expect(screen.getByText('จำนวนคืน / Nights')).toBeInTheDocument()
      expect(screen.getByText('ราคา/คืน / Rate')).toBeInTheDocument()
      expect(screen.getByText('รวม / Subtotal')).toBeInTheDocument()
    })

    test('displays room number', () => {
      render(<InvoiceTemplate {...defaultProps} />)

      expect(screen.getByText('301')).toBeInTheDocument()
    })

    test('displays room type', () => {
      render(<InvoiceTemplate {...defaultProps} />)

      // The room type appears in the table
      expect(screen.getAllByText('Standard').length).toBeGreaterThan(0)
    })

    test('displays number of nights', () => {
      render(<InvoiceTemplate {...defaultProps} />)

      expect(screen.getByText('5')).toBeInTheDocument()
    })

    test('displays rate per night formatted', () => {
      render(<InvoiceTemplate {...defaultProps} />)

      // 1500 formatted as Thai number
      expect(screen.getByText('1,500.00')).toBeInTheDocument()
    })

    test('displays room subtotal formatted', () => {
      render(<InvoiceTemplate {...defaultProps} />)

      // 7500 formatted as Thai number (appears twice: once in table, once in summary)
      const subtotalElements = screen.getAllByText('7,500.00')
      expect(subtotalElements.length).toBeGreaterThan(0)
    })

    test('renders multiple rooms correctly', () => {
      const invoiceMultipleRooms = createMockInvoiceData({
        rooms: [
          { roomNumber: '301', roomType: 'Standard', ratePerNight: 1500, nights: 3, subtotal: 4500 },
          { roomNumber: '302', roomType: 'Deluxe', ratePerNight: 2500, nights: 3, subtotal: 7500 },
          { roomNumber: '501', roomType: 'Suite', ratePerNight: 5000, nights: 2, subtotal: 10000 },
        ],
        subtotal: 22000,
        grandTotal: 22000,
      })

      render(<InvoiceTemplate {...defaultProps} checkInData={invoiceMultipleRooms} />)

      expect(screen.getByText('301')).toBeInTheDocument()
      expect(screen.getByText('302')).toBeInTheDocument()
      expect(screen.getByText('501')).toBeInTheDocument()
      expect(screen.getByText('Deluxe')).toBeInTheDocument()
      expect(screen.getByText('Suite')).toBeInTheDocument()
    })
  })

  describe('Summary Calculations', () => {
    test('displays subtotal correctly', () => {
      render(<InvoiceTemplate {...defaultProps} />)

      expect(screen.getByText('ยอดรวมค่าห้อง / Subtotal:')).toBeInTheDocument()
      // Multiple elements may have this text (subtotal and grand total when same)
      const subtotalElements = screen.getAllByText('7,500.00 บาท')
      expect(subtotalElements.length).toBeGreaterThan(0)
    })

    test('displays discount when greater than 0', () => {
      const invoiceWithDiscount = createMockInvoiceData({
        subtotal: 7500,
        discount: 500,
        grandTotal: 7000,
      })

      render(<InvoiceTemplate {...defaultProps} checkInData={invoiceWithDiscount} />)

      expect(screen.getByText('ส่วนลด / Discount:')).toBeInTheDocument()
      expect(screen.getByText('-500.00 บาท')).toBeInTheDocument()
    })

    test('does not display discount when it is 0', () => {
      const invoiceNoDiscount = createMockInvoiceData({ discount: 0 })

      render(<InvoiceTemplate {...defaultProps} checkInData={invoiceNoDiscount} />)

      expect(screen.queryByText('ส่วนลด / Discount:')).not.toBeInTheDocument()
    })

    test('displays VAT when showVat is true and vatAmount > 0', () => {
      const invoiceWithVat = createMockInvoiceData({
        subtotal: 7500,
        vatAmount: 525, // 7% of 7500
        grandTotal: 8025,
      })

      render(<InvoiceTemplate {...defaultProps} checkInData={invoiceWithVat} showVat={true} />)

      expect(screen.getByText('ภาษีมูลค่าเพิ่ม 7% / VAT 7%:')).toBeInTheDocument()
      expect(screen.getByText('525.00 บาท')).toBeInTheDocument()
    })

    test('does not display VAT when showVat is false', () => {
      const invoiceWithVat = createMockInvoiceData({
        vatAmount: 525,
        grandTotal: 8025,
      })

      render(<InvoiceTemplate {...defaultProps} checkInData={invoiceWithVat} showVat={false} />)

      expect(screen.queryByText('ภาษีมูลค่าเพิ่ม 7% / VAT 7%:')).not.toBeInTheDocument()
    })

    test('does not display VAT when vatAmount is 0 even if showVat is true', () => {
      const invoiceNoVat = createMockInvoiceData({ vatAmount: 0 })

      render(<InvoiceTemplate {...defaultProps} checkInData={invoiceNoVat} showVat={true} />)

      expect(screen.queryByText('ภาษีมูลค่าเพิ่ม 7% / VAT 7%:')).not.toBeInTheDocument()
    })

    test('displays grand total correctly', () => {
      render(<InvoiceTemplate {...defaultProps} />)

      expect(screen.getByText('ยอดรวมทั้งสิ้น / Grand Total:')).toBeInTheDocument()
      // Grand total appears in the summary section
      const grandTotalElements = screen.getAllByText(/7,500\.00 บาท/)
      expect(grandTotalElements.length).toBeGreaterThanOrEqual(1)
    })

    test('calculates correctly with discount and VAT', () => {
      const complexInvoice = createMockInvoiceData({
        subtotal: 10000,
        discount: 1000,
        vatAmount: 630, // 7% of (10000 - 1000)
        grandTotal: 9630, // 10000 - 1000 + 630
      })

      render(<InvoiceTemplate {...defaultProps} checkInData={complexInvoice} showVat={true} />)

      expect(screen.getByText('10,000.00 บาท')).toBeInTheDocument() // Subtotal
      expect(screen.getByText('-1,000.00 บาท')).toBeInTheDocument() // Discount
      expect(screen.getByText('630.00 บาท')).toBeInTheDocument() // VAT
      expect(screen.getByText('9,630.00 บาท')).toBeInTheDocument() // Grand Total
    })
  })

  describe('Thai Date Formatting (Buddhist Era +543)', () => {
    test('formats invoice date in full Thai format with Buddhist Era', () => {
      const invoiceJanuary = createMockInvoiceData({
        createdAt: '2026-01-25T10:00:00.000Z',
      })

      render(<InvoiceTemplate {...defaultProps} checkInData={invoiceJanuary} />)

      // January 25, 2026 = 25 มกราคม 2569
      expect(screen.getByText('25 มกราคม 2569')).toBeInTheDocument()
    })

    test('formats February date correctly', () => {
      const invoiceFebruary = createMockInvoiceData({
        createdAt: '2026-02-14T10:00:00.000Z',
        checkInDate: '2026-02-10T07:00:00.000Z',
        checkOutDate: '2026-02-14T05:00:00.000Z',
      })

      render(<InvoiceTemplate {...defaultProps} checkInData={invoiceFebruary} />)

      expect(screen.getByText('14 กุมภาพันธ์ 2569')).toBeInTheDocument()
    })

    test('formats December date with year transition', () => {
      const invoiceDecember = createMockInvoiceData({
        createdAt: '2025-12-31T10:00:00.000Z',
        checkInDate: '2025-12-28T07:00:00.000Z',
        checkOutDate: '2025-12-31T05:00:00.000Z',
      })

      render(<InvoiceTemplate {...defaultProps} checkInData={invoiceDecember} />)

      // December 31, 2025 = 31 ธันวาคม 2568
      expect(screen.getByText('31 ธันวาคม 2568')).toBeInTheDocument()
    })

    test('uses current date when createdAt is not provided', () => {
      // Mock the Date constructor
      const mockDate = new Date('2026-03-15T10:00:00.000Z')
      jest.useFakeTimers()
      jest.setSystemTime(mockDate)

      const invoiceNoCreatedAt = createMockInvoiceData({
        createdAt: '',
      })

      // Note: The component uses `checkInData.createdAt || today.toISOString()`
      // Since createdAt is empty string (falsy), it should use today's date
      render(<InvoiceTemplate {...defaultProps} checkInData={invoiceNoCreatedAt} />)

      // March 15, 2026 = 15 มีนาคม 2569
      expect(screen.getByText('15 มีนาคม 2569')).toBeInTheDocument()

      jest.useRealTimers()
    })
  })

  describe('Notes Section', () => {
    test('displays notes section', () => {
      render(<InvoiceTemplate {...defaultProps} />)

      expect(screen.getByText('หมายเหตุ / Notes:')).toBeInTheDocument()
    })

    test('displays Thai note text', () => {
      render(<InvoiceTemplate {...defaultProps} />)

      expect(screen.getByText('กรุณาตรวจสอบความถูกต้องของใบแจ้งหนี้')).toBeInTheDocument()
    })

    test('displays English note text', () => {
      render(<InvoiceTemplate {...defaultProps} />)

      expect(screen.getByText('Please verify the accuracy of this invoice.')).toBeInTheDocument()
    })
  })

  describe('Footer', () => {
    test('displays computer-generated message', () => {
      render(<InvoiceTemplate {...defaultProps} />)

      expect(screen.getByText('เอกสารนี้ออกโดยระบบคอมพิวเตอร์ / This document is computer generated.')).toBeInTheDocument()
    })

    test('displays hotel name and phone in footer', () => {
      render(<InvoiceTemplate {...defaultProps} />)

      expect(screen.getByText('The HF Hotel - 038-123-4567')).toBeInTheDocument()
    })
  })

  describe('Edge Cases', () => {
    test('handles empty rooms array', () => {
      const invoiceNoRooms = createMockInvoiceData({
        rooms: [],
        subtotal: 0,
        grandTotal: 0,
      })

      render(<InvoiceTemplate {...defaultProps} checkInData={invoiceNoRooms} />)

      // Should still render the table headers
      expect(screen.getByText('ห้อง / Room')).toBeInTheDocument()
      // But no room rows
      expect(screen.queryByText('301')).not.toBeInTheDocument()
    })

    test('handles very large amounts', () => {
      const expensiveInvoice = createMockInvoiceData({
        rooms: [
          { roomNumber: '1001', roomType: 'Presidential Suite', ratePerNight: 100000, nights: 30, subtotal: 3000000 },
        ],
        subtotal: 3000000,
        grandTotal: 3000000,
      })

      render(<InvoiceTemplate {...defaultProps} checkInData={expensiveInvoice} />)

      // Multiple elements may have the same text (subtotal and grand total when same)
      const largeAmountElements = screen.getAllByText('3,000,000.00 บาท')
      expect(largeAmountElements.length).toBeGreaterThan(0)
    })

    test('handles decimal amounts correctly', () => {
      const decimalInvoice = createMockInvoiceData({
        rooms: [
          { roomNumber: '301', roomType: 'Standard', ratePerNight: 1499.99, nights: 2, subtotal: 2999.98 },
        ],
        subtotal: 2999.98,
        grandTotal: 2999.98,
      })

      render(<InvoiceTemplate {...defaultProps} checkInData={decimalInvoice} />)

      expect(screen.getByText('1,499.99')).toBeInTheDocument()
      // Multiple elements may have the same text (subtotal and grand total when same)
      const decimalAmountElements = screen.getAllByText('2,999.98 บาท')
      expect(decimalAmountElements.length).toBeGreaterThan(0)
    })

    test('handles missing createdAt by using current date', () => {
      jest.useFakeTimers()
      jest.setSystemTime(new Date('2026-06-15T10:00:00.000Z'))

      const invoiceNoCreatedAt = {
        ...createMockInvoiceData(),
        createdAt: '',
      }

      render(<InvoiceTemplate {...defaultProps} checkInData={invoiceNoCreatedAt} />)

      // Should use current date: June 15, 2026 = 15 มิถุนายน 2569
      expect(screen.getByText('15 มิถุนายน 2569')).toBeInTheDocument()

      jest.useRealTimers()
    })

    test('renders with showVat defaulting to false', () => {
      const invoiceWithVat = createMockInvoiceData({
        vatAmount: 525,
      })

      // Don't pass showVat prop - should default to false
      render(<InvoiceTemplate checkInData={invoiceWithVat} hotelInfo={createMockHotelInfo()} />)

      // VAT should not be displayed
      expect(screen.queryByText('ภาษีมูลค่าเพิ่ม 7% / VAT 7%:')).not.toBeInTheDocument()
    })
  })

  // Track G3 / T4 HIGH-7 (`audit-2026-05-13.md`): VAT-inclusive split,
  // buyer tax-id, and PG-only invoice number. These tests pin the
  // render-time contract — server-side math is owned by
  // `hotel-backend/src/routes/new_invoice.rs` test module.
  describe('Track G3 — VAT invoice fields', () => {
    test('renders buyer tax-id in header when guestTaxId is present', () => {
      const corporateInvoice = createMockInvoiceData({
        guestTaxId: 'REDACTED-tax-id',
      })

      render(<InvoiceTemplate {...defaultProps} checkInData={corporateInvoice} />)

      // Label appears in the header alongside the seller's tax-id.
      expect(
        screen.getByText(/เลขประจำตัวผู้เสียภาษีของผู้ซื้อ \/ Buyer Tax ID:/)
      ).toBeInTheDocument()
      expect(screen.getByText('REDACTED-tax-id')).toBeInTheDocument()
    })

    test('hides buyer tax-id row when guestTaxId is absent (individual walk-in)', () => {
      const walkInInvoice = createMockInvoiceData({ guestTaxId: undefined })

      render(<InvoiceTemplate {...defaultProps} checkInData={walkInInvoice} />)

      expect(
        screen.queryByText(/เลขประจำตัวผู้เสียภาษีของผู้ซื้อ \/ Buyer Tax ID:/)
      ).not.toBeInTheDocument()
    })

    test('renders VAT-inclusive split (before-VAT line) when beforeVat is supplied', () => {
      // Server-side math (banker's rounding): total=801.00 → before=748.60, vat=52.40
      const vatInvoice = createMockInvoiceData({
        subtotal: 801,
        beforeVat: 748.6,
        vatAmount: 52.4,
        vatPercent: 7,
        grandTotal: 801,
      })

      render(
        <InvoiceTemplate {...defaultProps} checkInData={vatInvoice} showVat={true} />
      )

      // Before-VAT line uses the new label.
      expect(screen.getByText('มูลค่าก่อนภาษี / Before VAT:')).toBeInTheDocument()
      expect(screen.getByText('748.60 บาท')).toBeInTheDocument()
      // VAT line still rendered with the dynamic percentage.
      expect(screen.getByText('ภาษีมูลค่าเพิ่ม 7% / VAT 7%:')).toBeInTheDocument()
      expect(screen.getByText('52.40 บาท')).toBeInTheDocument()
    })

    test('renders dynamic VAT percentage from server when not 7%', () => {
      // Hypothetical 10% — the template MUST NOT hardcode "7%".
      const tenPercentInvoice = createMockInvoiceData({
        beforeVat: 1000,
        vatAmount: 100,
        vatPercent: 10,
        grandTotal: 1100,
      })

      render(
        <InvoiceTemplate {...defaultProps} checkInData={tenPercentInvoice} showVat={true} />
      )

      expect(screen.getByText('ภาษีมูลค่าเพิ่ม 10% / VAT 10%:')).toBeInTheDocument()
    })

    test('skips before-VAT line for legacy responses missing beforeVat', () => {
      // Older API responses (pre-G3) ship vatAmount only — no beforeVat.
      // The template must continue to render the VAT line by itself so
      // the regression test from earlier in this file still passes.
      const legacyInvoice = createMockInvoiceData({
        subtotal: 7500,
        vatAmount: 525,
        grandTotal: 8025,
        // beforeVat intentionally undefined.
      })

      render(
        <InvoiceTemplate {...defaultProps} checkInData={legacyInvoice} showVat={true} />
      )

      expect(screen.queryByText('มูลค่าก่อนภาษี / Before VAT:')).not.toBeInTheDocument()
      expect(screen.getByText('ภาษีมูลค่าเพิ่ม 7% / VAT 7%:')).toBeInTheDocument()
    })
  })

  // Task #44 (ใบกำกับภาษี): tax-invoice title relabel + POS / other-charge
  // line itemisation. The server math (VAT split over room+products) is
  // owned by `hotel-backend/src/routes/new_invoice.rs`; these pin the
  // render-time contract.
  describe('Task #44 — tax invoice title + product lines', () => {
    test('defaults to plain-invoice title when taxInvoice is not set', () => {
      render(<InvoiceTemplate {...defaultProps} />)
      expect(screen.getByText('ใบแจ้งหนี้ / Invoice')).toBeInTheDocument()
      expect(screen.queryByText('ใบกำกับภาษี / Tax Invoice')).not.toBeInTheDocument()
    })

    test('renders tax-invoice title when taxInvoice is true', () => {
      render(<InvoiceTemplate {...defaultProps} taxInvoice={true} />)
      expect(screen.getByText('ใบกำกับภาษี / Tax Invoice')).toBeInTheDocument()
      expect(screen.queryByText('ใบแจ้งหนี้ / Invoice')).not.toBeInTheDocument()
    })

    test('hides the products section for a room-only stay', () => {
      render(<InvoiceTemplate {...defaultProps} />)
      expect(screen.queryByText('สินค้าและบริการ / Products & Services')).not.toBeInTheDocument()
      expect(
        screen.queryByText('ยอดรวมสินค้าและบริการ / Products Subtotal:'),
      ).not.toBeInTheDocument()
    })

    test('renders product lines + products subtotal when present', () => {
      const withProducts = createMockInvoiceData({
        subtotal: 7500,
        products: [
          { name: 'น้ำดื่ม', unit: 'ขวด', qty: 2, unitPrice: 20, total: 40 },
          { name: 'เบียร์', unit: 'กระป๋อง', qty: 3, unitPrice: 60, total: 180 },
        ],
        productsSubtotal: 220,
        grandTotal: 7720,
      })

      render(<InvoiceTemplate {...defaultProps} checkInData={withProducts} taxInvoice={true} />)

      expect(screen.getByText('สินค้าและบริการ / Products & Services')).toBeInTheDocument()
      expect(screen.getByText('น้ำดื่ม')).toBeInTheDocument()
      expect(screen.getByText('เบียร์')).toBeInTheDocument()
      // Products subtotal line + the reconciling grand total.
      expect(screen.getByText('ยอดรวมสินค้าและบริการ / Products Subtotal:')).toBeInTheDocument()
      expect(screen.getByText('220.00 บาท')).toBeInTheDocument()
      expect(screen.getByText('7,720.00 บาท')).toBeInTheDocument()
    })
  })

  // Task #62 (itemise the tax invoice per room): a multi-room stay must
  // list one line per room (matching iHOTEL's per-room HT_CheckIn_Ds view)
  // with the room subtotal = sum of the per-room totals. The per-room data
  // is sourced from the ht_checkin_rooms junction server-side and mapped to
  // InvoiceData.rooms[] by mapInvoiceResponse; these pin the render-time
  // contract (single-room folios still fall back to one row).
  describe('Task #62 — per-room itemisation (multi-room bills)', () => {
    test('renders one line item per room with the summed subtotal (INV2606-019832 case)', () => {
      // Real reception case: cin 19832 (CH26-005923) = 2-room stay
      // (rooms 5 & 6), 890/night × 2 nights = 1,780 each → 3,560 total.
      const twoRoomInvoice = createMockInvoiceData({
        rooms: [
          { roomNumber: '5', roomType: 'Standard', ratePerNight: 890, nights: 2, subtotal: 1780 },
          { roomNumber: '6', roomType: 'Standard', ratePerNight: 890, nights: 2, subtotal: 1780 },
        ],
        subtotal: 3560,
        grandTotal: 3560,
      })

      render(<InvoiceTemplate {...defaultProps} checkInData={twoRoomInvoice} taxInvoice={true} />)

      // Two itemised room rows (one per room).
      expect(screen.getByText('5')).toBeInTheDocument()
      expect(screen.getByText('6')).toBeInTheDocument()
      // Each room line carries its own 1,780.00 subtotal (one per room).
      expect(screen.getAllByText('1,780.00')).toHaveLength(2)
      // The summary room subtotal is the SUM of both rooms (3,560), which
      // also equals the grand total here (no products) — appears on both
      // the subtotal and grand-total lines.
      expect(screen.getByText('ยอดรวมค่าห้อง / Subtotal:')).toBeInTheDocument()
      expect(screen.getAllByText('3,560.00 บาท').length).toBeGreaterThanOrEqual(2)
    })

    test('falls back to a single room row for a single-room stay', () => {
      // Single-room folio (or a pre-junction folio whose mapper fallback
      // produced one room): exactly one data row in the room-charges table.
      render(<InvoiceTemplate {...defaultProps} />)

      // Default mock = room 301 only.
      expect(screen.getByText('301')).toBeInTheDocument()
      const roomChargesSection =
        screen.getByText('รายละเอียดค่าห้องพัก / Room Charges').parentElement
      const roomRows = roomChargesSection!.querySelectorAll('tbody tr')
      expect(roomRows).toHaveLength(1)
    })
  })

  describe('Print Styles', () => {
    test('renders invoice container with print-friendly class', () => {
      const { container } = render(<InvoiceTemplate {...defaultProps} />)

      // Check that the invoice container exists with the proper class
      const invoiceContainer = container.querySelector('.invoice-container')
      expect(invoiceContainer).toBeInTheDocument()
      expect(invoiceContainer).toHaveClass('bg-white')
    })
  })
})
