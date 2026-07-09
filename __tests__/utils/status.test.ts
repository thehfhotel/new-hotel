/**
 * lib/v2/status — iHOTEL-anchored room-state tones (#227, ADR 0003 U2).
 *
 * Pinned here:
 * - Every room state maps to its iHOTEL-hue tone (`vac`/`stay`/`res`/`out`/
 *   `dirt`/`mtn`), NOT the v2-native tones — the hue is legacy vocabulary
 *   (FormRoomMain.cs palette).
 * - Thai labels are verbatim and unchanged by the color migration.
 * - Booking / check-in views (not room state) keep the v2-native tones.
 */

import { readFileSync } from 'fs'
import { join } from 'path'
import { bookingStatusView, checkinStatusView, roomStatusView } from '@/lib/v2/status'

describe('roomStatusView — iHOTEL room-state hues (#227)', () => {
  it.each([
    // [status, opts, tone, label]
    ['available', { isClean: true }, 'vac', 'ว่าง'],
    ['available', {}, 'vac', 'ว่าง'], // isClean undefined → treated clean
    ['available', { isClean: false }, 'dirt', 'รอทำความสะอาด'],
    ['occupied', {}, 'stay', 'มีผู้เข้าพัก'],
    ['booked', {}, 'res', 'จองแล้ว'],
    ['checkout_pending', {}, 'out', 'รอเช็คเอาท์'],
    ['maintenance', {}, 'mtn', 'ซ่อมบำรุง'],
  ] as const)('%s %o → tone %s, label %s', (status, opts, tone, label) => {
    const view = roomStatusView(status, opts)
    expect(view.tone).toBe(tone)
    expect(view.label).toBe(label)
    // The rooms page interpolates the tone into CSS var names AND uses the
    // paired classes — both derive from the tone.
    expect(view.cls).toBe(`s-${tone}`)
    expect(view.dot).toBe(`d-${tone}`)
  })

  it('isMaintenance flag overrides any status (iHOTEL: ซ่อม is the LAST assignment)', () => {
    expect(roomStatusView('available', { isMaintenance: true }).tone).toBe('mtn')
    expect(roomStatusView('occupied', { isMaintenance: true }).tone).toBe('mtn')
  })

  it('unknown status stays on the neutral tone', () => {
    expect(roomStatusView('something_else').tone).toBe('mut')
    expect(roomStatusView('').label).toBe('ไม่ทราบ')
  })
})

describe('v2.css ships the var pair + classes for every room tone', () => {
  // The rooms page interpolates `var(--v2-${tone}-bg)` / `var(--v2-${tone})`
  // inline, so the CSS var pair is mandatory — not just the .s-*/.d-* classes.
  const css = readFileSync(join(process.cwd(), 'app/v2/v2.css'), 'utf8')

  it.each(['vac', 'stay', 'res', 'out', 'dirt', 'mtn', 'mut'])('tone %s', (tone) => {
    expect(css).toContain(`--v2-${tone}:`)
    expect(css).toContain(`--v2-${tone}-bg:`)
    expect(css).toContain(`.s-${tone}`)
    expect(css).toContain(`.d-${tone}`)
  })
})

describe('booking / check-in views keep v2-native tones (not room state)', () => {
  it('bookingStatusView is unchanged', () => {
    expect(bookingStatusView('pending').tone).toBe('arr')
    expect(bookingStatusView('confirmed').tone).toBe('ok')
    expect(bookingStatusView('checkedin').tone).toBe('occ')
    expect(bookingStatusView('cancelled').tone).toBe('fix')
  })

  it('checkinStatusView is unchanged', () => {
    expect(checkinStatusView('active').tone).toBe('occ')
    expect(checkinStatusView('checkedout').tone).toBe('mut')
    expect(checkinStatusView('cancelled').tone).toBe('fix')
  })
})
