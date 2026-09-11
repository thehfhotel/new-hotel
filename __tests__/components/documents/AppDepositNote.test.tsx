/**
 * @jest-environment jsdom
 *
 * Task B7 — the paper half of the app-deposit signpost. Reception prints this
 * A6 note and clips it to the arrival paperwork, so the colleague who later
 * works the stay in iHOTEL (where the booking's deposit reads 0 until checkout)
 * does not ask the guest to pay twice.
 *
 * Like `RoomChangeSlip.test.tsx`, this asserts the rendered CONTENT contract
 * only — jsdom has no print behaviour to assert, and the isolation CSS is
 * exercised by the browser, not here.
 */

import { render, screen } from '@testing-library/react'
import AppDepositNote from '@/components/documents/AppDepositNote'

const baseProps = {
  bookChannel: 'loyalty',
  depositAmount: 600,
  bookingNo: 'B000123',
  guestName: 'สมชาย ใจดี',
  roomNo: '401',
  checkIn: '2026-09-12',
  checkOut: '2026-09-14',
  hotelName: 'The Harbour Front Hotel',
}

describe('AppDepositNote', () => {
  test('prints the booking identity, the amount and the double-charge warning', () => {
    render(<AppDepositNote {...baseProps} />)

    const note = screen.getByTestId('app-deposit-note')
    expect(note).toBeInTheDocument()
    expect(note).toHaveTextContent('The Harbour Front Hotel')
    expect(note).toHaveTextContent('มัดจำชำระผ่านแอปแล้ว / App deposit paid')
    expect(note).toHaveTextContent('B000123')
    expect(note).toHaveTextContent('สมชาย ใจดี')
    expect(note).toHaveTextContent('401')
    expect(note).toHaveTextContent(/600/)
    expect(note).toHaveTextContent('iHOTEL')
    expect(note).toHaveTextContent('ห้ามเรียกเก็บมัดจำนี้ซ้ำ / Do not collect this deposit again')
  })

  test('carries both languages — Thai claim first, English beneath', () => {
    render(<AppDepositNote {...baseProps} />)
    const note = screen.getByTestId('app-deposit-note')
    expect(note).toHaveTextContent('ชำระผ่านแอปแล้ว')
    expect(note).toHaveTextContent(/already collected by the app/i)
  })

  test('renders nothing for a walk-in, an OTA booking, or a deposit-less hold', () => {
    // Same gate as the on-screen notice, so the two can never disagree.
    for (const props of [
      { ...baseProps, bookChannel: null },
      { ...baseProps, bookChannel: 'agoda' },
      { ...baseProps, depositAmount: 0 },
      { ...baseProps, depositAmount: null },
    ]) {
      const { unmount } = render(<AppDepositNote {...props} />)
      expect(screen.queryByTestId('app-deposit-note')).not.toBeInTheDocument()
      unmount()
    }
  })

  test('survives missing optional identity fields', () => {
    render(<AppDepositNote bookChannel="loyalty" depositAmount={1200} />)
    const note = screen.getByTestId('app-deposit-note')
    expect(note).toHaveTextContent(/1,?200/)
    // Stay row always renders; unknown dates degrade to a dash, not "Invalid Date".
    expect(note).toHaveTextContent('เข้าพัก / Stay')
    expect(note).not.toHaveTextContent(/Invalid Date/i)
  })
})
