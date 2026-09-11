/**
 * lib/v2/status — the HOUSEKEEPING AXIS on `roomStatusView`.
 *
 * The invariant this file exists to pin: housekeeping is a SECOND AXIS, not a
 * status tier. `hkStatus` refines ONLY the vacant (`available`) branch and can
 * never win over ซ่อมบำรุง / มีผู้เข้าพัก / จองแล้ว / รอเช็คเอาท์ — a room with a
 * guest in it is not "รอทำความสะอาด" on reception's board no matter what the
 * housekeeping axis says about its cleanliness. Collapsing the two axes into
 * one ordered list is the mistake this test makes expensive.
 *
 * Deliberately under `__tests__/components/` rather than `__tests__/utils/`:
 * CI and the pre-push hook run `pnpm test:components` only, so a copy of these
 * assertions under utils/ would gate nothing. `__tests__/utils/status.test.ts`
 * keeps owning the tone↔v2.css pin.
 */

import { HK_STATUS_LABELS, roomStatusView, type HkStatus } from '@/lib/v2/status'

const HK_STATUSES: HkStatus[] = ['clean', 'cleaning', 'dirty']

/** Every tone the v2 room-state palette already ships (pinned against
 *  app/v2/v2.css by `__tests__/utils/status.test.ts`). The housekeeping axis
 *  must REUSE these: a new tone would emit `.s-*`/`.d-*` classes with no CSS
 *  behind them and render as an invisible pill. */
const EXISTING_ROOM_TONES = ['vac', 'stay', 'res', 'out', 'dirt', 'mtn', 'mut']

/** The availability tiers that outrank the housekeeping axis, with the view
 *  they must keep producing regardless of `hkStatus`. */
const AVAILABILITY_TIERS = [
  ['maintenance', 'mtn', 'ซ่อมบำรุง'],
  ['occupied', 'stay', 'มีผู้เข้าพัก'],
  ['booked', 'res', 'จองแล้ว'],
  ['checkout_pending', 'out', 'รอเช็คเอาท์'],
] as const

describe('HK_STATUS_LABELS — locked Thai copy', () => {
  it('spells the three housekeeping states exactly as the design fixed them', () => {
    expect(HK_STATUS_LABELS).toEqual({
      clean: 'สะอาด',
      cleaning: 'กำลังทำความสะอาด',
      dirty: 'รอทำความสะอาด',
    })
  })
})

describe('roomStatusView — hkStatus refines the vacant branch', () => {
  it.each([
    ['dirty', 'รอทำความสะอาด', 'dirt'],
    ['cleaning', 'กำลังทำความสะอาด', 'dirt'],
    ['clean', 'ว่าง', 'vac'],
  ] as const)('available + hkStatus %s → label %s, tone %s', (hkStatus, label, tone) => {
    const view = roomStatusView('available', { hkStatus })
    expect(view.label).toBe(label)
    expect(view.tone).toBe(tone)
    expect(view.cls).toBe(`s-${tone}`)
    expect(view.dot).toBe(`d-${tone}`)
  })

  it.each(HK_STATUSES)('reuses an existing tone for hkStatus %s — never invents one', (hkStatus) => {
    expect(EXISTING_ROOM_TONES).toContain(roomStatusView('available', { hkStatus }).tone)
  })

  it('shares the dirt tone between cleaning and dirty (not yet sellable-clean)', () => {
    expect(roomStatusView('available', { hkStatus: 'cleaning' }).tone).toBe(
      roomStatusView('available', { hkStatus: 'dirty' }).tone,
    )
  })

  it('outranks isClean=true, because hkStatus is the merged iHOTEL-wins truth', () => {
    const view = roomStatusView('available', { isClean: true, hkStatus: 'dirty' })
    expect(view.label).toBe('รอทำความสะอาด')
    expect(view.tone).toBe('dirt')
  })

  it('outranks isClean=false too — the merged truth wins in both directions', () => {
    const view = roomStatusView('available', { isClean: false, hkStatus: 'clean' })
    expect(view.label).toBe('ว่าง')
    expect(view.tone).toBe('vac')
  })
})

describe('roomStatusView — hkStatus is a SECOND AXIS, never a status tier', () => {
  const tierCases = AVAILABILITY_TIERS.flatMap(([status, tone, label]) =>
    HK_STATUSES.map((hkStatus) => [status, hkStatus, tone, label] as const),
  )

  it.each(tierCases)('%s is unchanged by hkStatus %s → tone %s, label %s', (status, hkStatus, tone, label) => {
    const view = roomStatusView(status, { hkStatus })
    expect(view.tone).toBe(tone)
    expect(view.label).toBe(label)
  })

  it.each(HK_STATUSES)('the isMaintenance flag still wins over hkStatus %s', (hkStatus) => {
    expect(roomStatusView('available', { isMaintenance: true, hkStatus }).tone).toBe('mtn')
    expect(roomStatusView('occupied', { isMaintenance: true, hkStatus }).tone).toBe('mtn')
  })

  it('leaves an unknown status on the neutral tone', () => {
    expect(roomStatusView('something_else', { hkStatus: 'cleaning' }).tone).toBe('mut')
    expect(roomStatusView('', { hkStatus: 'dirty' }).label).toBe('ไม่ทราบ')
  })
})

describe('roomStatusView — omitting hkStatus preserves the isClean behaviour exactly', () => {
  it.each([
    [{}, 'vac', 'ว่าง'],
    [{ isClean: true }, 'vac', 'ว่าง'],
    [{ isClean: false }, 'dirt', 'รอทำความสะอาด'],
    [{ hkStatus: undefined }, 'vac', 'ว่าง'],
    [{ isClean: false, hkStatus: undefined }, 'dirt', 'รอทำความสะอาด'],
  ] as const)('available %o → tone %s, label %s', (opts, tone, label) => {
    const view = roomStatusView('available', opts)
    expect(view.tone).toBe(tone)
    expect(view.label).toBe(label)
  })

  it('still compiles and behaves with no opts bag at all', () => {
    expect(roomStatusView('available')).toEqual({
      label: 'ว่าง',
      tone: 'vac',
      cls: 's-vac',
      dot: 'd-vac',
    })
  })
})
