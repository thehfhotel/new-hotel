/** Spatial room grid placement + guest-move drag eligibility (ADR 0003 U1 / #225 #226).
 *
 *  iHOTEL's FormRoomMain lets reception drag room tiles anywhere on the board
 *  and persists the pixel position in `HT_Rooms.Room_X`/`Room_y` (mirrored into
 *  canonical `ht_rooms_new.room_x`/`room_y`, exposed as `roomX`/`roomY` on
 *  /api/rooms). Those are free pixel coordinates of ~fixed-size WinForms
 *  panels, so tiles in the same visual column/row carry nearly — but not
 *  exactly — equal values. We quantize: cluster the sorted coordinate values
 *  (new cluster when the gap to the previous value exceeds a tolerance) and
 *  map each cluster to a CSS-grid column/row index. This preserves the
 *  receptionist-arranged arrangement without inheriting WinForms pixel pitch.
 *
 *  Rooms with `roomX`/`roomY` missing or both 0 are "unplaced" and render in a
 *  separate row below the board.
 */

import type { RoomItem } from '@/components/v2/RoomActionSheet'

/** RoomItem plus the fields the spatial grid / guest-move flow reads off the
 *  /api/rooms payload (present on the wire, just untyped on RoomItem). */
export interface SpatialRoom extends RoomItem {
  roomTypeId?: number | null
  priceWeekday?: number | null
  roomX?: number | null
  roomY?: number | null
}

export interface PlacedRoom<T extends SpatialRoom = SpatialRoom> {
  room: T
  /** 1-based CSS grid indexes. */
  col: number
  row: number
}

export interface SpatialLayout<T extends SpatialRoom = SpatialRoom> {
  placed: PlacedRoom<T>[]
  unplaced: T[]
  cols: number
  rows: number
}

/** Cluster tolerance in legacy pixels — iHOTEL tiles are ~100px wide, so
 *  values within 40px of their sorted neighbour belong to the same band. */
const CLUSTER_TOLERANCE = 40

/** True when a room has no receptionist-arranged position. */
export function isUnplaced(room: Pick<SpatialRoom, 'roomX' | 'roomY'>): boolean {
  const x = room.roomX
  const y = room.roomY
  if (x == null || y == null) return true
  return x === 0 && y === 0
}

/** Quantize raw coordinate values into 1-based band indexes.
 *  Returns a map from raw value → band index. */
function quantize(values: number[], tolerance: number): Map<number, number> {
  const sorted = [...new Set(values)].sort((a, b) => a - b)
  const bands = new Map<number, number>()
  let band = 0
  let prev: number | null = null
  for (const v of sorted) {
    if (prev === null || v - prev > tolerance) band += 1
    bands.set(v, band)
    prev = v
  }
  return bands
}

/** Compute the spatial layout for a set of rooms. */
export function computeSpatialLayout<T extends SpatialRoom>(
  rooms: T[],
  tolerance: number = CLUSTER_TOLERANCE,
): SpatialLayout<T> {
  const placedRooms = rooms.filter((r) => !isUnplaced(r))
  const unplaced = rooms.filter((r) => isUnplaced(r))

  const colBands = quantize(placedRooms.map((r) => r.roomX as number), tolerance)
  const rowBands = quantize(placedRooms.map((r) => r.roomY as number), tolerance)

  const placed = placedRooms.map((room) => ({
    room,
    col: colBands.get(room.roomX as number) ?? 1,
    row: rowBands.get(room.roomY as number) ?? 1,
  }))

  return {
    placed,
    unplaced,
    cols: placed.reduce((m, p) => Math.max(m, p.col), 0),
    rows: placed.reduce((m, p) => Math.max(m, p.row), 0),
  }
}

/** Guest-move drag eligibility (#225 decision comment):
 *  - source must be an occupied room;
 *  - target vacant (ว่าง) → ok;
 *  - target vacant-dirty (รอทำความสะอาด) → allowed with a visual warning;
 *  - maintenance / occupied / booked / checkout_pending → blocked.
 */
export type MoveEligibility = 'ok' | 'warn' | 'blocked'

export function moveEligibility(
  source: Pick<SpatialRoom, 'id' | 'status'>,
  target: Pick<SpatialRoom, 'id' | 'status' | 'isClean' | 'isMaintenance'>,
): MoveEligibility {
  if (source.status !== 'occupied') return 'blocked'
  if (target.id === source.id) return 'blocked'
  if (target.isMaintenance || target.status === 'maintenance') return 'blocked'
  if (target.status !== 'available') return 'blocked'
  return target.isClean === false ? 'warn' : 'ok'
}
