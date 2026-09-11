/**
 * The desk half of the room-signal contract (ADR 0008), as pure functions.
 *
 * The load-bearing rule pinned here is the ROLE rule: "nobody acts on their
 * own direction's signals except cancel-own-while-open". The desk is the
 * creator side of `desk_to_maid` and the audience for `maid_to_desk`, so the
 * UI must never offer a tap the backend is going to reject — most importantly
 * it must never offer เสร็จสิ้น on a ขอเช็คห้อง, which is completed by the
 * maid's ANSWER and returns 400 to a bare done.
 */

import {
  actorName,
  canDeskAck,
  canDeskCancel,
  canDeskDone,
  deskActionEndpoint,
  deskSendEndpoint,
  formatSignalTime,
  isGuestAccountability,
  signalStatusLabel,
  signalTone,
  signalsByRoomNo,
  sortSignals,
  type RoomSignal,
} from '@/components/v2/signals/signal-lib'
import { DESK_SIGNALS, MAID_SIGNALS } from '@/app/hk/signal-vocab'

function signal(partial: Partial<RoomSignal> & { signalId: number }): RoomSignal {
  return {
    roomId: 1,
    roomNo: '101',
    direction: 'maid_to_desk',
    type: 'guest_in_room',
    status: 'open',
    createdBy: { badge: 'Q1001', name: 'สมหญิง' },
    createdAt: '2026-09-01T03:00:00Z',
    ...partial,
  }
}

describe('endpoints', () => {
  it('sends a signal about exactly one room', () => {
    expect(deskSendEndpoint(42)).toBe('/api/housekeeping/rooms/42/signals')
  })

  it('addresses each lifecycle tap by signal id', () => {
    expect(deskActionEndpoint(7, 'ack')).toBe('/api/housekeeping/signals/7/ack')
    expect(deskActionEndpoint(7, 'done')).toBe('/api/housekeeping/signals/7/done')
    expect(deskActionEndpoint(7, 'cancel')).toBe('/api/housekeeping/signals/7/cancel')
  })
})

describe('what the desk may act on', () => {
  it('acks a maid signal only while it is still open', () => {
    expect(canDeskAck(signal({ signalId: 1, status: 'open' }))).toBe(true)
    expect(canDeskAck(signal({ signalId: 1, status: 'acked' }))).toBe(false)
  })

  it('completes a maid signal whether or not it was acked first', () => {
    expect(canDeskDone(signal({ signalId: 1, status: 'open' }))).toBe(true)
    expect(canDeskDone(signal({ signalId: 1, status: 'acked' }))).toBe(true)
  })

  it('never acks or completes a signal the desk itself sent', () => {
    for (const status of ['open', 'acked'] as const) {
      const own = signal({ signalId: 1, direction: 'desk_to_maid', type: 'priority_clean', status })
      expect(canDeskAck(own)).toBe(false)
      expect(canDeskDone(own)).toBe(false)
    }
  })

  it('offers no เสร็จสิ้น on ขอเช็คห้อง — completion is the maid ANSWER, and a bare done is a 400', () => {
    const check = signal({
      signalId: 1,
      direction: 'desk_to_maid',
      type: 'room_check',
      status: 'acked',
    })
    expect(canDeskDone(check)).toBe(false)
  })

  it('cancels its own signal only while it is still open', () => {
    expect(
      canDeskCancel(signal({ signalId: 1, direction: 'desk_to_maid', type: 'room_check' })),
    ).toBe(true)
    expect(
      canDeskCancel(
        signal({ signalId: 1, direction: 'desk_to_maid', type: 'room_check', status: 'acked' }),
      ),
    ).toBe(false)
  })

  it('never cancels a maid signal — the creator side owns the cancel', () => {
    expect(canDeskCancel(signal({ signalId: 1, direction: 'maid_to_desk' }))).toBe(false)
  })

  it('knows the two guest-accountability types and nothing else', () => {
    expect(isGuestAccountability({ type: 'item_missing' })).toBe(true)
    expect(isGuestAccountability({ type: 'item_damaged' })).toBe(true)
    for (const { type } of [...DESK_SIGNALS, ...MAID_SIGNALS]) {
      if (type === 'item_missing' || type === 'item_damaged') continue
      expect(isGuestAccountability({ type })).toBe(false)
    }
  })
})

describe('ordering — what needs reception now, not a chronology', () => {
  it('puts ขอเช็คห้อง first, then the guest-accountability pair', () => {
    const ordered = sortSignals([
      signal({ signalId: 1, type: 'deliver_linen', direction: 'desk_to_maid' }),
      signal({ signalId: 2, type: 'item_damaged' }),
      signal({ signalId: 3, type: 'room_check', direction: 'desk_to_maid' }),
    ])
    expect(ordered.map((s) => s.signalId)).toEqual([3, 2, 1])
  })

  it('breaks ties by longest-waiting first', () => {
    const ordered = sortSignals([
      signal({ signalId: 1, createdAt: '2026-09-01T04:00:00Z' }),
      signal({ signalId: 2, createdAt: '2026-09-01T02:00:00Z' }),
    ])
    expect(ordered.map((s) => s.signalId)).toEqual([2, 1])
  })

  it('does not mutate the input', () => {
    const input = [signal({ signalId: 1, type: 'skip_room' }), signal({ signalId: 2, type: 'room_check' })]
    sortSignals(input)
    expect(input.map((s) => s.signalId)).toEqual([1, 2])
  })

  it('groups by room number — the only key the cleaning board has', () => {
    const byRoom = signalsByRoomNo([
      signal({ signalId: 1, roomNo: '101' }),
      signal({ signalId: 2, roomNo: '102' }),
      signal({ signalId: 3, roomNo: '101', type: 'room_check', direction: 'desk_to_maid' }),
    ])
    expect(byRoom.get('101')?.map((s) => s.signalId)).toEqual([3, 1])
    expect(byRoom.get('102')?.map((s) => s.signalId)).toEqual([2])
    expect(byRoom.get('999')).toBeUndefined()
  })
})

describe('display vocabulary', () => {
  it('spells the four statuses in Thai', () => {
    expect(signalStatusLabel('open')).toBe('รอรับ')
    expect(signalStatusLabel('acked')).toBe('รับแล้ว')
    expect(signalStatusLabel('done')).toBe('เสร็จสิ้น')
    expect(signalStatusLabel('cancelled')).toBe('ยกเลิกแล้ว')
  })

  it('falls back to the raw code for a status this bundle predates', () => {
    expect(signalStatusLabel('escalated')).toBe('escalated')
  })

  it('tints the guest-accountability pair with the alarm tone in every status', () => {
    expect(signalTone({ type: 'item_missing', status: 'open' })).toBe('fix')
    expect(signalTone({ type: 'item_damaged', status: 'acked' })).toBe('fix')
  })

  it('tints the checkout-axis signals with the checkout tone', () => {
    expect(signalTone({ type: 'room_check', status: 'open' })).toBe('out')
    expect(signalTone({ type: 'checked_out', status: 'acked' })).toBe('out')
  })

  it('cools an ordinary signal down once somebody has put a name on it', () => {
    expect(signalTone({ type: 'priority_clean', status: 'open' })).toBe('arr')
    expect(signalTone({ type: 'priority_clean', status: 'acked' })).toBe('mut')
  })

  it('shows the name, falling back to the badge the IdP always forwards', () => {
    expect(actorName({ badge: 'Q1001', name: 'สมหญิง' })).toBe('สมหญิง')
    expect(actorName({ badge: 'Q1001', name: null })).toBe('Q1001')
    expect(actorName(null)).toBe('')
    expect(actorName(undefined)).toBe('')
  })

  it('formats a timestamptz in the browser zone, and survives junk', () => {
    const at = '2026-09-01T03:00:00Z'
    expect(formatSignalTime(at)).toBe(
      new Date(at).toLocaleTimeString('th-TH', { hour: '2-digit', minute: '2-digit' }),
    )
    expect(formatSignalTime('not-a-date')).toBe('')
    expect(formatSignalTime(null)).toBe('')
  })
})
