/**
 * Wave-4 §B6 — the reception แผนกแม่บ้าน board's "honest columns" derivation:
 * status comes from cleanliness + today's cleaning feed, never the dead
 * `status === 'cleaning'` literal `/api/rooms` never emits.
 */

import {
  deriveRoomStatus,
  housekeeperDisplayName,
  type CleaningFeedEntry,
} from '@/app/housekeeping/page'

function feedEntry(overrides: Partial<CleaningFeedEntry>): CleaningFeedEntry {
  return {
    roomId: 12,
    roomNo: '312',
    status: 'started',
    badge: 'Q1001',
    name: null,
    at: '2026-08-14T03:11:02Z',
    ...overrides,
  }
}

describe('deriveRoomStatus', () => {
  it('a started event today wins over cleanliness — this is what lights up the middle column', () => {
    expect(deriveRoomStatus(true, 'started')).toBe('cleaning')
    expect(deriveRoomStatus(false, 'started')).toBe('cleaning')
  })

  it('no started event today: dirty room ⇒ dirty', () => {
    expect(deriveRoomStatus(false, undefined)).toBe('dirty')
    expect(deriveRoomStatus(false, 'done')).toBe('dirty')
    expect(deriveRoomStatus(false, 'dirty')).toBe('dirty')
  })

  it('no started event today: clean room ⇒ available', () => {
    expect(deriveRoomStatus(true, undefined)).toBe('available')
    expect(deriveRoomStatus(true, 'done')).toBe('available')
  })
})

describe('housekeeperDisplayName', () => {
  it('is null when nothing was reported today (no assignment tracking placeholder)', () => {
    expect(housekeeperDisplayName(undefined)).toBeNull()
  })

  it('prefers the display name when present', () => {
    expect(housekeeperDisplayName(feedEntry({ name: 'นก', badge: 'Q1001' }))).toBe('นก')
  })

  it('falls back to badge when name is null — the CF IdP forwards only apps+badge today', () => {
    expect(housekeeperDisplayName(feedEntry({ name: null, badge: 'Q1001' }))).toBe('Q1001')
  })
})
