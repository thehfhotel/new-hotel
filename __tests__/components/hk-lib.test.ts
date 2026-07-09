/**
 * Pure-helper tests for the maid-facing housekeeping surface (/hk).
 * Covers the display/grouping logic in app/hk/hk-lib.ts — the API-shape
 * mapping is exercised end-to-end by the backend tests in
 * hotel-backend/src/routes/hk.rs.
 */

import {
  groupRoomsByFloor,
  progressLabel,
  timeLabel,
  type HkRoom,
} from '@/app/hk/hk-lib'

function room(overrides: Partial<HkRoom>): HkRoom {
  return {
    roomId: 1,
    roomNo: '101',
    floor: 1,
    building: null,
    roomClean: true,
    cleaning: null,
    openReports: 0,
    ...overrides,
  }
}

describe('progressLabel', () => {
  it('maps started/done/none to the Thai labels', () => {
    expect(progressLabel('started').label).toBe('กำลังทำความสะอาด')
    expect(progressLabel('done').label).toBe('เสร็จแล้ว')
    expect(progressLabel(null).label).toBe('ยังไม่เริ่ม')
    expect(progressLabel(undefined).label).toBe('ยังไม่เริ่ม')
  })

  it('assigns distinct badge styles per status', () => {
    const classes = new Set([
      progressLabel('started').className,
      progressLabel('done').className,
      progressLabel(null).className,
    ])
    expect(classes.size).toBe(3)
  })
})

describe('groupRoomsByFloor', () => {
  it('groups by floor ascending with floorless rooms last', () => {
    const rooms = [
      room({ roomId: 1, roomNo: '301', floor: 3 }),
      room({ roomId: 2, roomNo: '101', floor: 1 }),
      room({ roomId: 3, roomNo: 'ANNEX', floor: null }),
      room({ roomId: 4, roomNo: '102', floor: 1 }),
    ]
    const groups = groupRoomsByFloor(rooms)
    expect(groups.map((g) => g.floor)).toEqual([1, 3, null])
    expect(groups[0].rooms.map((r) => r.roomNo)).toEqual(['101', '102'])
  })

  it('sorts room numbers numerically within a floor', () => {
    const rooms = [
      room({ roomId: 1, roomNo: '110', floor: 1 }),
      room({ roomId: 2, roomNo: '102', floor: 1 }),
      room({ roomId: 3, roomNo: '19', floor: 1 }),
    ]
    const [group] = groupRoomsByFloor(rooms)
    expect(group.rooms.map((r) => r.roomNo)).toEqual(['19', '102', '110'])
  })

  it('returns an empty list for no rooms', () => {
    expect(groupRoomsByFloor([])).toEqual([])
  })
})

describe('timeLabel', () => {
  it('renders a parseable ISO instant as hh:mm', () => {
    // Exact rendering is locale/timezone dependent; assert shape only.
    expect(timeLabel('2026-07-09T03:15:00Z')).toMatch(/\d{2}:\d{2}/)
  })

  it('returns empty string for garbage', () => {
    expect(timeLabel('not-a-date')).toBe('')
  })
})
