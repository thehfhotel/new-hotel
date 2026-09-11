/**
 * @jest-environment jsdom
 *
 * Task B7 — iHOTEL shows an app booking's deposit as 0 until checkout by
 * design, so a receptionist reading "unpaid" on a guest who already paid is the
 * failure that kills the direct channel quietly. The rule for when this
 * signpost appears (and, just as importantly, when it stays silent) is pinned
 * here rather than left to a visual check on four different screens.
 */

import { render, screen } from '@testing-library/react'
import AppDepositNotice, { appDepositNoticeView } from '@/components/v2/AppDepositNotice'

describe('AppDepositNotice', () => {
  test('shows the Thai-first notice for a loyalty booking with a deposit', () => {
    render(<AppDepositNotice bookChannel="loyalty" depositAmount={600} />)

    const notice = screen.getByTestId('app-deposit-notice')
    expect(notice).toBeInTheDocument()

    // Thai line carries the amount, the "paid in the app" claim, and the
    // iHOTEL-shows-0 warning — the three facts reception needs.
    const th = screen.getByTestId('app-deposit-notice-th')
    expect(th).toHaveTextContent(/600/)
    expect(th).toHaveTextContent('ชำระผ่านแอปแล้ว')
    expect(th).toHaveTextContent('iHOTEL')
    expect(th).toHaveTextContent('จนกว่าจะเช็คเอาต์')

    // English line sits beneath it.
    const en = screen.getByTestId('app-deposit-notice-en')
    expect(en).toHaveTextContent(/600/)
    expect(en).toHaveTextContent(/iHOTEL shows 0/i)
  })

  test('stays silent for a loyalty booking with no deposit recorded yet', () => {
    // An unpaid hold: `book_deposit_amount` is still 0 because
    // `confirm_booking_payment` has not run. Claiming money was collected here
    // would be worse than saying nothing.
    const { container } = render(<AppDepositNotice bookChannel="loyalty" depositAmount={0} />)
    expect(container).toBeEmptyDOMElement()
  })

  test('stays silent for a loyalty booking whose deposit is null', () => {
    const { container } = render(<AppDepositNotice bookChannel="loyalty" depositAmount={null} />)
    expect(container).toBeEmptyDOMElement()
  })

  test('stays silent for an OTA booking even with a deposit', () => {
    // OTA prepayment sits with the agency and is not this divergence.
    const { container } = render(<AppDepositNotice bookChannel="agoda" depositAmount={900} />)
    expect(container).toBeEmptyDOMElement()
  })

  test('stays silent for a walk-in with a desk deposit', () => {
    const { container } = render(<AppDepositNotice bookChannel={null} depositAmount={500} />)
    expect(container).toBeEmptyDOMElement()
  })

  test('is null-safe when both fields are missing entirely', () => {
    const { container } = render(
      <AppDepositNotice bookChannel={undefined} depositAmount={undefined} />,
    )
    expect(container).toBeEmptyDOMElement()
  })

  test('never prints — the paper version is the A6 note, not this banner', () => {
    render(<AppDepositNotice bookChannel="loyalty" depositAmount={600} />)
    const notice = screen.getByTestId('app-deposit-notice')
    expect(notice.className).toContain('no-print')
    expect(notice.className).toContain('v2-no-print')
  })
})

describe('appDepositNoticeView', () => {
  test('matches the canonical channel literal case-insensitively, padding ignored', () => {
    expect(appDepositNoticeView('  Loyalty ', 600)).toEqual({ amount: 600 })
  })

  test('rejects a negative or non-finite deposit rather than rendering nonsense', () => {
    expect(appDepositNoticeView('loyalty', -100)).toBeNull()
    expect(appDepositNoticeView('loyalty', Number.NaN)).toBeNull()
    expect(appDepositNoticeView('loyalty', Number.POSITIVE_INFINITY)).toBeNull()
  })

  test('accepts a sub-baht amount — any collected money is worth signposting', () => {
    expect(appDepositNoticeView('loyalty', 0.5)).toEqual({ amount: 0.5 })
  })

  test('is the single rule every surface shares', () => {
    // Same inputs, same answer — the reservation detail, registration card,
    // folio, payment dialog and printed note all call this one function.
    expect(appDepositNoticeView('loyalty', 1200)).toEqual({ amount: 1200 })
    expect(appDepositNoticeView('bookingcom', 1200)).toBeNull()
  })
})
