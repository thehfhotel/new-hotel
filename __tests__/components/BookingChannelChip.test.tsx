/**
 * @jest-environment jsdom
 *
 * Task B6 — reception cannot tell an app booking from a walk-in. The chip is
 * the tell, so its rule (which bookings get a chip, and what it says) is
 * pinned here rather than left to a visual check.
 */

import { render, screen } from '@testing-library/react'
import BookingChannelChip, { bookingChannelView } from '@/components/v2/BookingChannelChip'

describe('BookingChannelChip', () => {
  test('shows the Thai "แอป" chip for a loyalty-channel booking', () => {
    render(<BookingChannelChip bookChannel="loyalty" bookSource="loyalty" />)

    const chip = screen.getByTestId('booking-channel-chip')
    expect(chip).toHaveTextContent('แอป')
    expect(chip).toHaveAttribute('data-channel-kind', 'app')
  })

  test('renders nothing for a walk-in booking (null channel, null source)', () => {
    const { container } = render(<BookingChannelChip bookChannel={null} bookSource={null} />)
    expect(container).toBeEmptyDOMElement()
  })

  test('renders nothing for a phone booking', () => {
    const { container } = render(<BookingChannelChip bookChannel={null} bookSource="phone" />)
    expect(container).toBeEmptyDOMElement()
  })

  test('is null-safe when the field is missing entirely', () => {
    const { container } = render(<BookingChannelChip bookChannel={undefined} />)
    expect(container).toBeEmptyDOMElement()
  })

  test('names the OTA when the channel is a known slug', () => {
    render(<BookingChannelChip bookChannel="bookingcom" bookSource="ota" />)

    const chip = screen.getByTestId('booking-channel-chip')
    expect(chip).toHaveTextContent('Booking.com')
    expect(chip).toHaveAttribute('data-channel-kind', 'ota')
  })

  test('falls back to a generic OTA chip for a pre-076 row with no channel', () => {
    render(<BookingChannelChip bookChannel={null} bookSource="ota" />)
    expect(screen.getByTestId('booking-channel-chip')).toHaveTextContent('OTA')
  })

  test('shows the raw channel for an OTA slug we have no pretty name for', () => {
    render(<BookingChannelChip bookChannel="someagency" bookSource="ota" />)
    expect(screen.getByTestId('booking-channel-chip')).toHaveTextContent('someagency')
  })
})

describe('bookingChannelView', () => {
  test('matches the canonical literal case-insensitively and ignores padding', () => {
    expect(bookingChannelView('  Loyalty ')).toEqual({ label: 'แอป', kind: 'app' })
  })

  test('treats an empty-string channel as no channel', () => {
    expect(bookingChannelView('', null)).toBeNull()
  })

  test('the channel wins over a stale source', () => {
    // A loyalty booking also carries book_source='loyalty'; a desk-corrected
    // source must never downgrade the app chip.
    expect(bookingChannelView('loyalty', 'ota')).toEqual({ label: 'แอป', kind: 'app' })
  })
})
