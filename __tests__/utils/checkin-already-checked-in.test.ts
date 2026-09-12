/**
 * Task B7b — decoding the backend's booking-level check-in refusal.
 *
 * The proactive "เช็คอินแล้ว" state (B7a, `openCheckInIdForBooking`) is computed
 * BEFORE the receptionist fills the check-in form, so it cannot see another desk
 * — or iHOTEL — checking the same booking in while she types. The backend's
 * `409` is the only thing that catches that window, and two properties of it are
 * worth pinning rather than checking by eye at the desk:
 *
 *  1. It must be recognised ONLY on the exact `(409, reason)` pair. A looser
 *     test (any 409, or any body carrying `conflictingId`) would swallow other
 *     refusals and show reception the wrong Thai sentence.
 *  2. The message reception sees must never be the backend's own, which is
 *     English and contains `book_id` / `cin_id` values.
 */

import {
  ALREADY_CHECKED_IN_MESSAGE,
  BOOKING_ALREADY_CHECKED_IN_REASON,
  parseAlreadyCheckedIn,
} from '@/lib/v2/checkin-from-booking'

/** The body `POST /api/checkins` actually returns for this refusal. */
const REFUSAL_BODY = {
  success: false,
  reason: BOOKING_ALREADY_CHECKED_IN_REASON,
  error:
    'booking 812 is already checked in (check-in 4242) — open that folio instead of creating a second one',
  conflictingId: 4242,
}

describe('parseAlreadyCheckedIn', () => {
  it('decodes the refusal and surfaces the existing folio id', () => {
    expect(parseAlreadyCheckedIn(409, REFUSAL_BODY)).toEqual({ checkInId: 4242 })
  })

  it('ignores anything that is not this exact refusal', () => {
    // Right status, different refusal (the channel's own 409s share the code).
    expect(parseAlreadyCheckedIn(409, { reason: 'sold_out', conflictingId: 1 })).toBeNull()
    // Right reason, wrong status — a 400 is the room-occupied guard, not this.
    expect(parseAlreadyCheckedIn(400, REFUSAL_BODY)).toBeNull()
    // No reason at all: every pre-B7b error body.
    expect(parseAlreadyCheckedIn(409, { success: false, error: 'boom' })).toBeNull()
    // Defensive: non-object bodies from a proxy or an HTML error page.
    expect(parseAlreadyCheckedIn(409, null)).toBeNull()
    expect(parseAlreadyCheckedIn(409, 'Conflict')).toBeNull()
  })

  it('degrades to a null id rather than linking to /billing/undefined', () => {
    // The guard should always name a stay, but a body that does not must still
    // produce the Thai message — with no link — instead of a broken href.
    for (const conflictingId of [undefined, null, 'four thousand', NaN]) {
      expect(
        parseAlreadyCheckedIn(409, { ...REFUSAL_BODY, conflictingId }),
      ).toEqual({ checkInId: null })
    }
  })
})

describe('ALREADY_CHECKED_IN_MESSAGE', () => {
  it('is Thai and carries no row ids', () => {
    expect(ALREADY_CHECKED_IN_MESSAGE).toMatch(/[฀-๿]/)
    expect(ALREADY_CHECKED_IN_MESSAGE).not.toMatch(/\d/)
    expect(ALREADY_CHECKED_IN_MESSAGE).not.toMatch(/[A-Za-z]/)
  })
})
