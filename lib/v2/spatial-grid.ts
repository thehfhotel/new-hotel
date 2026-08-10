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
 *  values within 40px of their sorted neighbour belong to the same band.
 *  Exported so `deriveBoardPixels` callers/tests share the same constant. */
export const CLUSTER_TOLERANCE = 40

/** Pixel pitch used when a layout-edit drop must SYNTHESIZE a coordinate
 *  (new band beyond the board's extent, or an empty board). ~the iHOTEL tile
 *  pitch; must be STRICTLY greater than CLUSTER_TOLERANCE so the synthesized
 *  value forms its own band instead of merging into a neighbour. */
const BOARD_PITCH = 110

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

/** A 1-based board cell (same coordinate space as `PlacedRoom.col/row`). */
export interface BoardCell {
  col: number
  row: number
}

/** Per-axis band table derived the same way `computeSpatialLayout` does:
 *  band index (1-based) → the sorted raw pixel values in that band. */
function bandValues(values: number[], tolerance: number): number[][] {
  const bands = quantize(values, tolerance)
  const byBand: number[][] = []
  for (const [raw, band] of bands) {
    ;(byBand[band - 1] ||= []).push(raw)
  }
  for (const list of byBand) list.sort((a, b) => a - b)
  return byBand
}

/** Resolve ONE axis of a layout-edit drop (#236 จัดผัง).
 *
 *  - Band exists → reuse an existing raw value VERBATIM (exact-value reuse
 *    maps to exactly that band via quantize's Set-dedup, so the round-trip is
 *    tautological and can never chain-merge neighbouring bands — the
 *    "neighbor-derived" rule from the governing decision. Never midpoint!).
 *  - Band one-or-more beyond the extent → extrapolate outermost + n·pitch
 *    (pitch > tolerance strictly, so the new value forms its own band).
 *  - Empty axis (no placed tiles) → seed at 10 + (idx−1)·pitch, iHOTEL-like
 *    origin, never 0 so a (1,1) seed can't produce the (0,0) sentinel.
 */
function axisValue(bands: number[][], index: number): number {
  if (bands.length === 0) return 10 + (index - 1) * BOARD_PITCH
  if (index <= bands.length) return bands[index - 1][0]
  const outermost = bands[bands.length - 1]
  return outermost[outermost.length - 1] + (index - bands.length) * BOARD_PITCH
}

/** True when nudging a 0-valued band member to 1 cannot merge band 1 into
 *  band 2 (quantize merges on gap ≤ tolerance, so the next band's smallest
 *  member must sit STRICTLY more than tolerance+1 away from 0). */
function nudgeIsBandSafe(bands: number[][], tolerance: number): boolean {
  if (bands.length < 2) return true
  return bands[1][0] - 1 > tolerance
}

/** Derive the raw legacy pixel pair to WRITE so that a layout-edit drop
 *  lands the room in `target` — and ONLY the moved room changes cell — when
 *  the board is re-run through `computeSpatialLayout` (#236 decision 2).
 *
 *  MUST be fed the UNFILTERED room list: bands computed from a filtered
 *  subset can differ, and pixels derived from them may land in the wrong
 *  cell once the filter clears.
 *
 *  Swaps do NOT go through here — exchange the two rooms' existing pairs
 *  verbatim (both values already band correctly by exact-value reuse).
 */
export function deriveBoardPixels<T extends SpatialRoom>(
  rooms: T[],
  target: BoardCell,
  tolerance: number = CLUSTER_TOLERANCE,
): { x: number; y: number } {
  const placed = rooms.filter((r) => !isUnplaced(r))
  const xBands = bandValues(placed.map((r) => r.roomX as number), tolerance)
  const yBands = bandValues(placed.map((r) => r.roomY as number), tolerance)

  let x = axisValue(xBands, target.col)
  let y = axisValue(yBands, target.row)

  // (0,0) is the legacy "unplaced" sentinel (`isUnplaced`) — a derived pair
  // must never hit it. Only possible when both target bands contain a raw 0
  // (a single-axis 0 is a real placement). Prefer another exemplar from the
  // same band; as a last resort nudge one axis to 1 — inside the band
  // (gap 1 ≤ tolerance) — picking the axis where the nudge provably cannot
  // chain-merge band 1 into band 2.
  if (x === 0 && y === 0) {
    const altX = target.col <= xBands.length ? xBands[target.col - 1].find((v) => v !== 0) : undefined
    const altY = target.row <= yBands.length ? yBands[target.row - 1].find((v) => v !== 0) : undefined
    if (altX !== undefined) x = altX
    else if (altY !== undefined) y = altY
    else if (nudgeIsBandSafe(yBands, tolerance)) y = 1
    else x = 1 // xBands nudge — see nudgeIsBandSafe; both-unsafe is unreachable
    // with integer pixels unless band 2 starts at exactly tolerance+1 on BOTH
    // axes while band 1 is exactly {0} on both; accept the x-nudge then.
  }

  return { x, y }
}

/** Where a จัดผัง drop landed: on another placed tile (swap) or on an empty
 *  lattice cell (move/place). Mirrors `LayoutDropTarget` in
 *  `components/v2/SpatialRoomGrid.tsx`, minus the React coupling. */
export type LayoutDropLanding<T extends SpatialRoom> =
  | { type: 'swap'; room: T }
  | { type: 'cell'; col: number; row: number }

/** One entry of the `PUT /api/rooms/layout` body (#236 wire contract). */
export interface LayoutMove {
  id: number
  roomX: number
  roomY: number
}

/** Build the `PUT /api/rooms/layout` payload for one จัดผัง drop.
 *
 *  - **cell** (move/place): one move, pixels neighbour-derived via
 *    `deriveBoardPixels` so they round-trip through `computeSpatialLayout`
 *    into exactly the intended cell.
 *  - **swap**: two moves exchanging the two rooms' existing pairs VERBATIM —
 *    no derivation, because both values already band correctly by exact-value
 *    reuse. One request, so the backend commits both halves in one intent and
 *    the shared board is never left half-swapped.
 *
 *  Returns `null` when the drop is a no-op the caller must ignore: a swap
 *  partner without coordinates (an unplaced-row tile has no pair to give
 *  away) or a self-swap. `rooms` MUST be the UNFILTERED list — see
 *  `deriveBoardPixels`.
 */
export function buildLayoutMoves<T extends SpatialRoom>(
  rooms: T[],
  source: T,
  target: LayoutDropLanding<T>,
  tolerance: number = CLUSTER_TOLERANCE,
): LayoutMove[] | null {
  if (target.type === 'swap') {
    const other = target.room
    if (other.id === source.id) return null
    if (source.roomX == null || source.roomY == null) return null
    if (other.roomX == null || other.roomY == null) return null
    return [
      { id: source.id, roomX: other.roomX, roomY: other.roomY },
      { id: other.id, roomX: source.roomX, roomY: source.roomY },
    ]
  }
  const { x, y } = deriveBoardPixels(rooms, { col: target.col, row: target.row }, tolerance)
  return [{ id: source.id, roomX: x, roomY: y }]
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
