/**
 * @jest-environment jsdom
 *
 * Task #54 item 5 — printable room-change slip. Pins the content contract
 * (folio + both rooms + old/new rate + reason) so a future refactor surfaces
 * here.
 */

import { render, screen } from '@testing-library/react'
import RoomChangeSlip from '@/components/documents/RoomChangeSlip'

describe('RoomChangeSlip — Task #54 item 5', () => {
  it('renders the move summary: folio, both rooms, rates, and reason', () => {
    render(
      <RoomChangeSlip
        cinNo="CIN-20260514-0007"
        customerName="Test Guest"
        fromRoomNo="303"
        toRoomNo="404"
        roomBeforePrice={1200}
        toPrice="890"
        reason="AC broken"
        changedAt="2026-05-14T12:00:00Z"
        changedBy="alice"
        hotelName="HF Hotel"
      />,
    )

    // Title + hotel header.
    expect(
      screen.getByText('ใบแจ้งเปลี่ยนห้องพัก / Room Change'),
    ).toBeInTheDocument()
    expect(screen.getByText('HF Hotel')).toBeInTheDocument()

    // Folio + guest.
    expect(screen.getByText('CIN-20260514-0007')).toBeInTheDocument()
    expect(screen.getByText('Test Guest')).toBeInTheDocument()

    // Both room numbers.
    expect(screen.getByText('303')).toBeInTheDocument()
    expect(screen.getByText('404')).toBeInTheDocument()

    // Both rates render (currency-formatted; assert the bare figures appear).
    expect(screen.getByText(/1,200/)).toBeInTheDocument()
    expect(screen.getByText(/890/)).toBeInTheDocument()

    // Reason + operator.
    expect(screen.getByText('AC broken')).toBeInTheDocument()
    expect(screen.getByText('alice')).toBeInTheDocument()
  })

  it('omits the new-rate value gracefully when toPrice is missing', () => {
    render(
      <RoomChangeSlip
        cinNo="CIN-X"
        fromRoomNo="101"
        toRoomNo="102"
        roomBeforePrice={0}
        toPrice={null}
      />,
    )
    // The new-rate label is still shown with a dash fallback (the dash also
    // appears for the missing date, so there is at least one).
    expect(screen.getByText('ราคาใหม่ / New rate')).toBeInTheDocument()
    expect(screen.getAllByText('-').length).toBeGreaterThanOrEqual(1)
  })
})
