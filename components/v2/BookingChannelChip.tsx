/**
 * Where a booking came from, as a small Thai-first chip.
 *
 * Reception cannot otherwise tell an app booking from a walk-in: both land in
 * `ht_bookings` and read identically on the reservations list. This chip is the
 * visual tell.
 *
 * Source of truth is `ht_bookings.book_channel` (migration 076), surfaced by the
 * backend as `bookChannel` on the reservations LIST and DETAIL responses:
 *   - `'loyalty'` — booked in the guest app (`service::channel::LOYALTY_CHANNEL`)
 *   - an OTA slug (`'bookingcom'`, `'agoda'`, …) — written back by OTA Desk
 *   - `null` — walk-in / phone / manual desk booking
 *
 * `bookSource` (`ht_bookings.book_source`, already on the wire as `source`) is
 * the older, coarser field and is used ONLY as a fallback for historical OTA
 * rows written before `book_channel` was wired: `source === 'ota'` with no
 * channel still deserves an "OTA" chip.
 *
 * Read-only. Renders nothing for a walk-in or phone booking — a chip on every
 * row would carry no signal.
 */

/** Pretty names for the OTA slugs `book_channel` actually carries. Anything
 *  unmapped falls through to the raw slug, which is still more useful at the
 *  desk than a generic "OTA". */
const OTA_LABELS: Record<string, string> = {
  bookingcom: 'Booking.com',
  'booking.com': 'Booking.com',
  agoda: 'Agoda',
  expedia: 'Expedia',
  traveloka: 'Traveloka',
  trip: 'Trip.com',
  'trip.com': 'Trip.com',
  ctrip: 'Trip.com',
  airbnb: 'Airbnb',
  ota: 'OTA',
}

/** Channels that are NOT a distinct booking origin worth a chip. */
const NO_CHIP_CHANNELS = new Set(['walkin', 'walk-in', 'walk_in', 'phone', 'direct', 'desk', ''])

export type BookingChannelView = {
  label: string
  /** `app` = booked in the guest app; `ota` = came from an OTA. */
  kind: 'app' | 'ota'
}

/**
 * Resolve the chip to show, or `null` for "no chip".
 * Exported so the list, the detail header and tests all agree on one rule.
 */
export function bookingChannelView(
  bookChannel: string | null | undefined,
  bookSource?: string | null,
): BookingChannelView | null {
  const channel = (bookChannel ?? '').trim().toLowerCase()

  if (channel === 'loyalty') return { label: 'แอป', kind: 'app' }

  if (channel && !NO_CHIP_CHANNELS.has(channel)) {
    return { label: OTA_LABELS[channel] ?? bookChannel!.trim(), kind: 'ota' }
  }

  // No channel recorded — fall back to the coarse legacy `book_source` so
  // pre-076 OTA rows still read as OTA.
  const source = (bookSource ?? '').trim().toLowerCase()
  if (source === 'ota') return { label: 'OTA', kind: 'ota' }
  if (source && OTA_LABELS[source] && !NO_CHIP_CHANNELS.has(source)) {
    return { label: OTA_LABELS[source], kind: 'ota' }
  }

  return null
}

export default function BookingChannelChip({
  bookChannel,
  bookSource,
}: {
  bookChannel: string | null | undefined
  bookSource?: string | null
}) {
  const view = bookingChannelView(bookChannel, bookSource)
  if (!view) return null

  // v2 tokens (app/v2/v2.css) with literal fallbacks — the tokens are scoped
  // under `.v2`, and BookingForm (where the detail chip lives) is also mounted
  // by the classic /bookings page, outside that scope.
  //   app -> `dep` teal: distinct from the wine status pill and from the
  //          wine-family `occ` tint the balance-due chip already uses.
  //   OTA -> `mut` neutral: present but quiet next to the status pill.
  const style =
    view.kind === 'app'
      ? { background: 'var(--v2-dep-bg, #e2eff1)', color: 'var(--v2-dep, #2a7585)' }
      : { background: 'var(--v2-mut-bg, #ede9e3)', color: 'var(--v2-mut, #877e77)' }

  return (
    <span
      data-testid="booking-channel-chip"
      data-channel-kind={view.kind}
      title={view.kind === 'app' ? 'จองผ่านแอป' : `จองผ่าน ${view.label}`}
      className="inline-flex items-center text-[11px] leading-none px-1.5 py-0.5 rounded font-medium whitespace-nowrap"
      style={style}
    >
      {view.label}
    </span>
  )
}
