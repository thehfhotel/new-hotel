/**
 * @jest-environment jsdom
 *
 * Wave-5 R2 — the maid's room screen (`app/hk/rooms/[roomId]/page.tsx`).
 *
 * Two behaviours, both of which cost a real person real work when they break:
 *
 * 1. **The mark-dirty confirm (R2b).** แจ้งห้องไม่สะอาด used to POST on a
 *    single tap. A mis-tap flips the room dirty in iHOTEL, which puts it back
 *    on reception's board and sends someone to look at a room nobody asked
 *    about. The first tap must issue NO request; only ยืนยัน may. เสร็จแล้ว
 *    and เริ่มทำความสะอาด must stay single-tap — a confirm on every tap is a
 *    confirm nobody reads.
 * 2. **The iHOTEL-unavailable note (R2a / CR-1).** When the backend could not
 *    reach iHOTEL it serves the PMS mirror and flags it. The maid must SEE
 *    that, and must still get a fully working screen — stale-but-shown beats a
 *    dead screen on a stairwell.
 * 3. **แจ้งขาดผ้า (linen shortage).** The same "first tap files nothing" shape,
 *    plus a payload that has to be exactly right: the linen room acts on the
 *    kinds and quantities in that body, and a report that fails must leave the
 *    counts on screen — retyping six steppers in a corridor is how a report
 *    stops getting filed at all.
 *
 * `hkFetch` / `hkFetchMe` are mocked at the module boundary: this suite is
 * about what the page DOES with an answer, and `hk-lib.test.ts` already owns
 * the URL construction and the pure helpers.
 */

import { fireEvent, render, screen, waitFor, within } from '@testing-library/react'

const mockHkFetch = jest.fn()
const mockHkFetchMe = jest.fn()

jest.mock('next/navigation', () => ({
  useParams: () => ({ roomId: '7' }),
}))

jest.mock('@/app/hk/hk-lib', () => {
  const actual = jest.requireActual('@/app/hk/hk-lib')
  return {
    ...actual,
    hkFetch: (...args: unknown[]) => mockHkFetch(...args),
    hkFetchMe: (...args: unknown[]) => mockHkFetchMe(...args),
  }
})

import HkRoomPage from '@/app/hk/rooms/[roomId]/page'
import { LEGACY_STATUS_STALE_NOTE, LINEN_KINDS } from '@/app/hk/hk-lib'
import { HK_STATUS_LABELS } from '@/lib/v2/status'

const ROOM = {
  roomId: 7,
  roomNo: '104',
  floor: 1,
  building: null,
  roomClean: false,
  cleaning: null,
  occupancy: 'occupied',
}

function jsonResponse(body: unknown, status = 200) {
  return {
    ok: status >= 200 && status < 300,
    status,
    json: async () => body,
  }
}

/** `/me` with the mark-dirty button enabled and a single branch (so the page
 * auto-selects it and never blocks on the picker). `overrides` is how the
 * viewer-mode suite below flips `canReport` — the default payload deliberately
 * OMITS the field, which is the deploy-skew shape (an older backend) and must
 * keep behaving exactly like a maid's. */
function meResponse(overrides: Record<string, unknown> = {}) {
  return jsonResponse({
    success: true,
    badge: 'Q1001',
    displayName: null,
    branches: [{ id: 'hfhotel', labelTh: 'ฮาร์เบอร์ฟร้อนท์' }],
    markDirtyEnabled: true,
    branchesUnavailableReason: null,
    ...overrides,
  })
}

/** Render the page with a scripted room-detail payload and wait for it. */
async function renderRoom(detail: Record<string, unknown>) {
  mockHkFetchMe.mockResolvedValue(meResponse())
  mockHkFetch.mockResolvedValue(
    jsonResponse({ success: true, room: ROOM, events: [], ...detail })
  )
  render(<HkRoomPage />)
  await screen.findByText('ห้อง 104')
}

beforeEach(() => {
  jest.clearAllMocks()
  localStorage.clear()
})

describe('mark-dirty confirm step (R2b)', () => {
  // THE regression this exists for: tapping แจ้งห้องไม่สะอาด must not file
  // anything. Asserted on the POST count, not on the DOM, because that is the
  // thing that reaches iHOTEL.
  it('the first tap issues NO request — it only arms the confirm', async () => {
    await renderRoom({})
    const readCalls = mockHkFetch.mock.calls.length

    fireEvent.click(screen.getByRole('button', { name: /แจ้งห้องไม่สะอาด/ }))

    await screen.findByText(/ยืนยันแจ้งว่า ห้อง 104 ยังไม่สะอาด/)
    expect(mockHkFetch.mock.calls.length).toBe(readCalls)
    expect(
      mockHkFetch.mock.calls.some(([, , init]) => (init as RequestInit | undefined)?.method === 'POST')
    ).toBe(false)
  })

  it('ยืนยัน files the dirty report', async () => {
    await renderRoom({})
    fireEvent.click(screen.getByRole('button', { name: /แจ้งห้องไม่สะอาด/ }))
    await screen.findByText(/ยืนยันแจ้งว่า ห้อง 104/)

    fireEvent.click(screen.getByRole('button', { name: 'ยืนยัน' }))

    await waitFor(() => {
      const post = mockHkFetch.mock.calls.find(
        ([, , init]) => (init as RequestInit | undefined)?.method === 'POST'
      )
      expect(post).toBeDefined()
      expect(post?.[0]).toBe('/rooms/7/cleaning')
      expect(JSON.parse((post?.[2] as RequestInit).body as string)).toEqual({ status: 'dirty' })
    })
  })

  // The mis-tap path, end to end: arm it, change your mind, walk away with
  // nothing filed and the original button back.
  it('ยกเลิก files nothing and restores the button', async () => {
    await renderRoom({})
    fireEvent.click(screen.getByRole('button', { name: /แจ้งห้องไม่สะอาด/ }))
    await screen.findByText(/ยืนยันแจ้งว่า ห้อง 104/)

    fireEvent.click(screen.getByRole('button', { name: 'ยกเลิก' }))

    await waitFor(() =>
      expect(screen.queryByText(/ยืนยันแจ้งว่า ห้อง 104/)).not.toBeInTheDocument()
    )
    expect(screen.getByRole('button', { name: /แจ้งห้องไม่สะอาด/ })).toBeInTheDocument()
    expect(
      mockHkFetch.mock.calls.some(([, , init]) => (init as RequestInit | undefined)?.method === 'POST')
    ).toBe(false)
  })

  // The normal flow must not grow friction. If these ever start needing a
  // confirm, the confirm on the destructive action stops meaning anything.
  it.each(['เสร็จแล้ว', 'เริ่มทำความสะอาด'])('%s stays single-tap', async (label) => {
    await renderRoom({})
    fireEvent.click(screen.getByRole('button', { name: label }))

    await waitFor(() =>
      expect(
        mockHkFetch.mock.calls.some(
          ([, , init]) => (init as RequestInit | undefined)?.method === 'POST'
        )
      ).toBe(true)
    )
  })

  // The button is server-gated (HK_MARK_DIRTY_ENABLED via /me). With it off
  // there must be no button AND no way to reach the confirm.
  it('renders neither button nor confirm while markDirtyEnabled is false', async () => {
    mockHkFetchMe.mockResolvedValue(
      jsonResponse({
        success: true,
        badge: 'Q1001',
        displayName: null,
        branches: [{ id: 'hfhotel', labelTh: 'ฮาร์เบอร์ฟร้อนท์' }],
        markDirtyEnabled: false,
        branchesUnavailableReason: null,
      })
    )
    mockHkFetch.mockResolvedValue(jsonResponse({ success: true, room: ROOM, events: [] }))
    render(<HkRoomPage />)
    await screen.findByText('ห้อง 104')

    expect(screen.queryByRole('button', { name: /แจ้งห้องไม่สะอาด/ })).not.toBeInTheDocument()
    expect(screen.queryByRole('button', { name: 'ยืนยัน' })).not.toBeInTheDocument()
  })
})

describe('แจ้งขาดผ้า — linen shortage', () => {
  /** Every POST the page has issued, in order. The GETs (initial load, polls)
   * are noise here — what reaches the linen room is a POST. */
  function postCalls() {
    return mockHkFetch.mock.calls.filter(
      ([, , init]) => (init as RequestInit | undefined)?.method === 'POST'
    )
  }

  /** Every READ the page has issued (no `init` ⇒ a GET), in order — the
   * reload after a landed report is one of these. */
  function getCalls() {
    return mockHkFetch.mock.calls.filter(([, , init]) => init === undefined)
  }

  /** The one POST the page must have issued, decoded. */
  function soleLinenPost() {
    const posts = postCalls()
    expect(posts).toHaveLength(1)
    const init = posts[0][2] as RequestInit
    return { path: posts[0][0] as string, init, body: JSON.parse(init.body as string) }
  }

  async function openPanel() {
    await renderRoom({})
    fireEvent.click(screen.getByRole('button', { name: 'แจ้งขาดผ้า' }))
    await screen.findByRole('button', { name: 'ส่งแจ้ง' })
  }

  /** Tap a kind's + button `times` times. The node identity survives the
   * re-renders, so one lookup is enough. */
  function step(label: string, times: number) {
    const plus = screen.getByRole('button', { name: `เพิ่ม ${label}` })
    for (let i = 0; i < times; i += 1) fireEvent.click(plus)
  }

  // Same regression as the mark-dirty confirm: opening a form is not filing a
  // report. Asserted on the POST count, because that is what the linen room acts on.
  it('the first tap issues NO request — it only opens the form', async () => {
    await renderRoom({})
    const readCalls = mockHkFetch.mock.calls.length

    fireEvent.click(screen.getByRole('button', { name: 'แจ้งขาดผ้า' }))

    await screen.findByRole('button', { name: 'ส่งแจ้ง' })
    expect(mockHkFetch.mock.calls.length).toBe(readCalls)
    expect(postCalls()).toHaveLength(0)
    // Every row starts at zero, so every − starts dead.
    expect(screen.getByRole('button', { name: 'ลด ปลอกหมอน' })).toBeDisabled()
  })

  it('ส่งแจ้ง is dead until at least one kind is above zero', async () => {
    await openPanel()
    expect(screen.getByRole('button', { name: 'ส่งแจ้ง' })).toBeDisabled()

    step('ผ้าเช็ดตัว', 1)

    expect(screen.getByRole('button', { name: 'ส่งแจ้ง' })).toBeEnabled()
  })

  // The payload IS the feature: wrong codes or wrong quantities send someone
  // up with the wrong armful of linen. Zero rows must not ship at all.
  it('POSTs exactly the non-zero rows, then confirms and collapses', async () => {
    await openPanel()
    step('ปลอกหมอน', 2)
    step('ผ้าเช็ดเท้า', 3)

    mockHkFetch.mockResolvedValueOnce(jsonResponse({ success: true, roomId: 7, reported: 2 }))
    fireEvent.click(screen.getByRole('button', { name: 'ส่งแจ้ง' }))

    await screen.findByText('บันทึกแล้ว: แจ้งขาดผ้า')
    const post = soleLinenPost()
    expect(post.path).toBe('/rooms/7/linen-shortage')
    expect(post.init.method).toBe('POST')
    expect(post.init.headers).toEqual({ 'Content-Type': 'application/json' })
    expect(post.body).toEqual({
      items: [
        { kind: 'pillowcase', qty: 2 },
        { kind: 'foot_towel', qty: 3 },
      ],
    })

    // Panel gone, trigger back, and the form is empty again on reopen.
    expect(screen.queryByRole('button', { name: 'ส่งแจ้ง' })).not.toBeInTheDocument()
    fireEvent.click(screen.getByRole('button', { name: 'แจ้งขาดผ้า' }))
    expect(await screen.findByRole('button', { name: 'ส่งแจ้ง' })).toBeDisabled()
  })

  // The retry path. Losing the counts on a failure is how a report stops
  // getting filed at all — she is standing in a corridor, not at a desk.
  it('keeps the panel open and the counts intact when the report fails', async () => {
    await openPanel()
    step('ผ้าเช็ดหน้า', 2)

    mockHkFetch.mockResolvedValueOnce(jsonResponse({ success: false }, 500))
    fireEvent.click(screen.getByRole('button', { name: 'ส่งแจ้ง' }))

    expect(await screen.findByText('บันทึกไม่สำเร็จ กรุณาลองใหม่')).toBeInTheDocument()
    expect(screen.getByRole('button', { name: 'ส่งแจ้ง' })).toBeInTheDocument()
    expect(screen.queryByText('บันทึกแล้ว: แจ้งขาดผ้า')).not.toBeInTheDocument()

    // Counts survived: the retry sends the same body, and the green banner
    // replaces the red one rather than hiding behind it.
    mockHkFetch.mockResolvedValueOnce(jsonResponse({ success: true, roomId: 7, reported: 1 }))
    fireEvent.click(screen.getByRole('button', { name: 'ส่งแจ้ง' }))

    await screen.findByText('บันทึกแล้ว: แจ้งขาดผ้า')
    const posts = postCalls()
    expect(posts).toHaveLength(2)
    expect(JSON.parse((posts[1][2] as RequestInit).body as string)).toEqual({
      items: [{ kind: 'face_towel', qty: 2 }],
    })
  })

  // A 200 is not the answer — the body's `success` is. A green banner over a
  // report that never landed is worse than an error.
  it('treats a 200 carrying success: false as a failure', async () => {
    await openPanel()
    step('ปลอกผ้านวม', 1)

    mockHkFetch.mockResolvedValueOnce(jsonResponse({ success: false, roomId: 7, reported: 0 }))
    fireEvent.click(screen.getByRole('button', { name: 'ส่งแจ้ง' }))

    expect(await screen.findByText('บันทึกไม่สำเร็จ กรุณาลองใหม่')).toBeInTheDocument()
    expect(screen.queryByText('บันทึกแล้ว: แจ้งขาดผ้า')).not.toBeInTheDocument()
    expect(screen.getByRole('button', { name: 'ส่งแจ้ง' })).toBeInTheDocument()
  })

  // The kinds ARE the vocabulary: the stepper rows come from LINEN_KINDS, and
  // that list is what the linen room reads. A sixth kind that renders in the
  // wrong place (or not at all) is a maid who cannot report a missing bed
  // sheet — so both the count and the leading row are asserted.
  it('offers every LINEN_KINDS row, ผ้าปูที่นอน first', async () => {
    await openPanel()
    const plusButtons = screen.getAllByRole('button', { name: /^เพิ่ม / })
    expect(plusButtons).toHaveLength(LINEN_KINDS.length)
    expect(plusButtons).toHaveLength(6)
    expect(plusButtons.map((b) => b.getAttribute('aria-label'))).toEqual(
      LINEN_KINDS.map(({ label }) => `เพิ่ม ${label}`)
    )
    // Bed linen largest-first: ผ้าปูที่นอน leads, ปลอกหมอน follows.
    expect(plusButtons[0]).toHaveAttribute('aria-label', 'เพิ่ม ผ้าปูที่นอน')
    expect(plusButtons[1]).toHaveAttribute('aria-label', 'เพิ่ม ปลอกหมอน')
  })

  // The new kind must reach the wire under the code the backend allowlists.
  it('sends the bed_sheet code for the ผ้าปูที่นอน row', async () => {
    await openPanel()
    step('ผ้าปูที่นอน', 2)

    mockHkFetch.mockResolvedValueOnce(jsonResponse({ success: true, roomId: 7, reported: 1 }))
    fireEvent.click(screen.getByRole('button', { name: 'ส่งแจ้ง' }))

    await screen.findByText('บันทึกแล้ว: แจ้งขาดผ้า')
    expect(soleLinenPost().body).toEqual({ items: [{ kind: 'bed_sheet', qty: 2 }] })
  })

  // A landed report changes what this room IS (the ขาดผ้า chip, today's
  // totals). Without the reload the maid is left looking at the pre-report
  // room and may well file the same shortage twice.
  it('re-fetches the room after a successful report', async () => {
    await openPanel()
    step('ปลอกหมอน', 1)
    const getsBefore = getCalls().length

    mockHkFetch.mockResolvedValueOnce(jsonResponse({ success: true, roomId: 7, reported: 1 }))
    fireEvent.click(screen.getByRole('button', { name: 'ส่งแจ้ง' }))

    await screen.findByText('บันทึกแล้ว: แจ้งขาดผ้า')
    await waitFor(() => expect(getCalls().length).toBe(getsBefore + 1))
    expect(getCalls().at(-1)?.[0]).toBe('/rooms/7')
  })

  // ...and only after a successful one. A failed report leaves the screen
  // exactly as she left it, counts included (asserted above).
  it('does NOT re-fetch when the report fails', async () => {
    await openPanel()
    step('ปลอกหมอน', 1)
    const getsBefore = getCalls().length

    mockHkFetch.mockResolvedValueOnce(jsonResponse({ success: false }, 500))
    fireEvent.click(screen.getByRole('button', { name: 'ส่งแจ้ง' }))

    await screen.findByText('บันทึกไม่สำเร็จ กรุณาลองใหม่')
    expect(getCalls().length).toBe(getsBefore)
  })

  it('ยกเลิก files nothing and resets the counts', async () => {
    await openPanel()
    step('ผ้าเช็ดตัว', 2)

    fireEvent.click(screen.getByRole('button', { name: 'ยกเลิก' }))

    await waitFor(() =>
      expect(screen.queryByRole('button', { name: 'ส่งแจ้ง' })).not.toBeInTheDocument()
    )
    expect(postCalls()).toHaveLength(0)
    // Reopening gives a blank form, not the abandoned one.
    fireEvent.click(screen.getByRole('button', { name: 'แจ้งขาดผ้า' }))
    expect(await screen.findByRole('button', { name: 'ส่งแจ้ง' })).toBeDisabled()
  })

  // The ceiling is the contract (qty ≤ 20), so it is enforced in the reducer,
  // not only by the disabled +: a bound that lives on an attribute is one
  // double-tap away from not existing.
  it('clamps a runaway stepper at 20 and never sends more', async () => {
    await openPanel()
    step('ปลอกหมอน', 25)

    expect(screen.getByRole('button', { name: 'เพิ่ม ปลอกหมอน' })).toBeDisabled()
    mockHkFetch.mockResolvedValueOnce(jsonResponse({ success: true, roomId: 7, reported: 1 }))
    fireEvent.click(screen.getByRole('button', { name: 'ส่งแจ้ง' }))

    await screen.findByText('บันทึกแล้ว: แจ้งขาดผ้า')
    expect(soleLinenPost().body).toEqual({ items: [{ kind: 'pillowcase', qty: 20 }] })
  })

  // One in-flight report at a time, whichever kind it is — two reports on one
  // room from one thumb is never what she meant.
  it('locks every other action while the linen report is in flight', async () => {
    await openPanel()
    step('ผ้าเช็ดตัว', 1)

    let settle: (value: unknown) => void = () => {}
    mockHkFetch.mockReturnValueOnce(
      new Promise((resolve) => {
        settle = resolve
      })
    )
    fireEvent.click(screen.getByRole('button', { name: 'ส่งแจ้ง' }))

    await waitFor(() => expect(screen.getByRole('button', { name: 'เสร็จแล้ว' })).toBeDisabled())
    expect(screen.getByRole('button', { name: 'เริ่มทำความสะอาด' })).toBeDisabled()
    expect(screen.getByRole('button', { name: /แจ้งห้องไม่สะอาด/ })).toBeDisabled()

    settle(jsonResponse({ success: true, roomId: 7, reported: 1 }))
    await screen.findByText('บันทึกแล้ว: แจ้งขาดผ้า')
    expect(screen.getByRole('button', { name: 'เสร็จแล้ว' })).toBeEnabled()
  })
})

describe('clean/dirty chip on the detail header (owner feedback, wave-5)', () => {
  // ROOM fixture is roomClean: false — the merged-dirty state.
  it('shows the explicit รอทำความสะอาด chip for a merged-dirty room', async () => {
    await renderRoom({})
    expect(screen.getByText(HK_STATUS_LABELS.dirty)).toBeInTheDocument()
  })

  it('shows the explicit สะอาด chip for a merged-clean room, not just silence', async () => {
    mockHkFetchMe.mockResolvedValue(meResponse())
    mockHkFetch.mockResolvedValue(
      jsonResponse({ success: true, room: { ...ROOM, roomClean: true }, events: [] })
    )
    render(<HkRoomPage />)
    await screen.findByText('ห้อง 104')
    expect(screen.getByText(HK_STATUS_LABELS.clean)).toBeInTheDocument()
    expect(screen.queryByText(HK_STATUS_LABELS.dirty)).not.toBeInTheDocument()
  })

  it('keeps the progress chip visible as a secondary label alongside the clean/dirty chip', async () => {
    await renderRoom({})
    expect(screen.getByText(HK_STATUS_LABELS.dirty)).toBeInTheDocument()
    // ROOM.cleaning is null → progressLabel default, "ยังไม่เริ่ม".
    expect(screen.getByText('ยังไม่เริ่ม')).toBeInTheDocument()
  })

  // When the backend fell back to the PMS mirror, the chip still renders the
  // canonical fallback value — the stale note explains provenance, not
  // whether there is a chip at all.
  it('still renders the chip while legacyStatusStale is true', async () => {
    await renderRoom({ legacyStatusStale: true })
    expect(screen.getByText(HK_STATUS_LABELS.dirty)).toBeInTheDocument()
  })

  it('uses the exact same words as reception (lib/v2/status.ts HK labels)', async () => {
    await renderRoom({})
    expect(screen.getByText(HK_STATUS_LABELS.dirty).textContent).toBe(HK_STATUS_LABELS.dirty)
  })
})

describe('guest occupancy indicator (header, next to the roomNo heading)', () => {
  // ROOM fixture is occupancy: 'occupied'.
  it('shows มีแขกพัก for an occupied room', async () => {
    await renderRoom({})
    expect(screen.getByText('มีแขกพัก')).toBeInTheDocument()
  })

  it('shows ว่าง for a vacant room', async () => {
    mockHkFetchMe.mockResolvedValue(meResponse())
    mockHkFetch.mockResolvedValue(
      jsonResponse({ success: true, room: { ...ROOM, occupancy: 'vacant' }, events: [] })
    )
    render(<HkRoomPage />)
    await screen.findByText('ห้อง 104')
    expect(screen.getByText('ว่าง')).toBeInTheDocument()
    expect(screen.queryByText('มีแขกพัก')).not.toBeInTheDocument()
  })

  // Deploy skew: an older backend has not shipped `occupancy` yet.
  it('shows nothing when the backend omits occupancy entirely', async () => {
    const { occupancy: _occupancy, ...roomWithoutOccupancy } = ROOM
    mockHkFetchMe.mockResolvedValue(meResponse())
    mockHkFetch.mockResolvedValue(
      jsonResponse({ success: true, room: roomWithoutOccupancy, events: [] })
    )
    render(<HkRoomPage />)
    await screen.findByText('ห้อง 104')
    expect(screen.queryByText('มีแขกพัก')).not.toBeInTheDocument()
    expect(screen.queryByText('ว่าง')).not.toBeInTheDocument()
  })
})

describe('movement tags (phase 2 delta: arrivals/departures today)', () => {
  it('renders both tags, departure first, for a back-to-back room', async () => {
    mockHkFetchMe.mockResolvedValue(meResponse())
    mockHkFetch.mockResolvedValue(
      jsonResponse({
        success: true,
        room: { ...ROOM, expectedArrival: true, expectedDeparture: true },
        events: [],
      })
    )
    render(<HkRoomPage />)
    await screen.findByText('ห้อง 104')

    const departure = screen.getByText('แขกออกวันนี้')
    const arrival = screen.getByText('แขกเข้าวันนี้')
    expect(departure).toBeInTheDocument()
    expect(arrival).toBeInTheDocument()
    expect(
      departure.compareDocumentPosition(arrival) & Node.DOCUMENT_POSITION_FOLLOWING
    ).toBeTruthy()
  })

  it('renders only the departure tag for a departure-only room', async () => {
    mockHkFetchMe.mockResolvedValue(meResponse())
    mockHkFetch.mockResolvedValue(
      jsonResponse({ success: true, room: { ...ROOM, expectedDeparture: true }, events: [] })
    )
    render(<HkRoomPage />)
    await screen.findByText('ห้อง 104')
    expect(screen.getByText('แขกออกวันนี้')).toBeInTheDocument()
    expect(screen.queryByText('แขกเข้าวันนี้')).not.toBeInTheDocument()
  })

  it('renders only the arrival tag for an arrival-only room', async () => {
    mockHkFetchMe.mockResolvedValue(meResponse())
    mockHkFetch.mockResolvedValue(
      jsonResponse({ success: true, room: { ...ROOM, expectedArrival: true }, events: [] })
    )
    render(<HkRoomPage />)
    await screen.findByText('ห้อง 104')
    expect(screen.getByText('แขกเข้าวันนี้')).toBeInTheDocument()
    expect(screen.queryByText('แขกออกวันนี้')).not.toBeInTheDocument()
  })

  // ROOM fixture carries no expectedArrival/expectedDeparture — skew case.
  it('renders no tag row when the fields are absent, chip row still exactly two chips', async () => {
    await renderRoom({})
    expect(screen.queryByText('แขกออกวันนี้')).not.toBeInTheDocument()
    expect(screen.queryByText('แขกเข้าวันนี้')).not.toBeInTheDocument()
    expect(screen.getByText(HK_STATUS_LABELS.dirty)).toBeInTheDocument()
    expect(screen.getByText('ยังไม่เริ่ม')).toBeInTheDocument()
  })
})

// ---------------------------------------------------------------------------
// ขาดผ้า tag + today's totals. The tag is the same one the room list carries
// (one helper, two screens); the totals line is the detail behind it — what a
// maid needs before she counts anything, so she does not file this morning's
// shortage a second time.
// ---------------------------------------------------------------------------

describe('ขาดผ้า tag and today\'s totals on the detail screen', () => {
  it('shows the chip beside the cleaning chips when today carries a report', async () => {
    await renderRoom({ room: { ...ROOM, linenShortageToday: true } })
    expect(screen.getByText('ขาดผ้า')).toBeInTheDocument()
    // The cleaning chips are untouched — a linen shortage is a different axis,
    // not a replacement status.
    expect(screen.getByText(HK_STATUS_LABELS.dirty)).toBeInTheDocument()
    expect(screen.getByText('ยังไม่เริ่ม')).toBeInTheDocument()
  })

  // The case the whole tag exists for, on this screen too.
  it('shows the chip on a room already reported เสร็จแล้ว', async () => {
    await renderRoom({
      room: {
        ...ROOM,
        roomClean: true,
        cleaning: { status: 'done', badge: 'Q1001', name: null, at: '2026-09-01T03:00:00.000Z' },
        linenShortageToday: true,
      },
    })
    // Scoped to the header: "เสร็จแล้ว" is also the label of the progress
    // BUTTON further down, and the chip is the one under test.
    const header = screen.getByText('ห้อง 104').closest('header') as HTMLElement
    expect(within(header).getByText('เสร็จแล้ว')).toBeInTheDocument()
    expect(within(header).getByText('ขาดผ้า')).toBeInTheDocument()
  })

  it('shows no chip when the room reported no shortage today', async () => {
    await renderRoom({ room: { ...ROOM, linenShortageToday: false } })
    expect(screen.queryByText('ขาดผ้า')).not.toBeInTheDocument()
  })

  // Deploy skew — the ROOM fixture carries no such field.
  it('shows no chip when the backend omits the field entirely', async () => {
    await renderRoom({})
    expect(screen.queryByText('ขาดผ้า')).not.toBeInTheDocument()
  })

  // The totals line: Thai labels from LINEN_KINDS, quantities as delivered,
  // in the order delivered.
  it('renders today\'s totals with Thai labels, in the delivered order', async () => {
    await renderRoom({
      room: { ...ROOM, linenShortageToday: true },
      linenShortages: [
        { kind: 'pillowcase', qty: 2 },
        { kind: 'bath_towel', qty: 1 },
      ],
    })
    expect(screen.getByText('วันนี้แจ้งขาดผ้า: ปลอกหมอน 2, ผ้าเช็ดตัว 1')).toBeInTheDocument()
  })

  it('renders the new ผ้าปูที่นอน label in the totals line', async () => {
    await renderRoom({
      room: { ...ROOM, linenShortageToday: true },
      linenShortages: [{ kind: 'bed_sheet', qty: 3 }],
    })
    expect(screen.getByText('วันนี้แจ้งขาดผ้า: ผ้าปูที่นอน 3')).toBeInTheDocument()
  })

  it('renders no totals line for an empty list', async () => {
    await renderRoom({ linenShortages: [] })
    expect(screen.queryByText(/วันนี้แจ้งขาดผ้า/)).not.toBeInTheDocument()
  })

  it('renders no totals line when the backend omits the field entirely', async () => {
    await renderRoom({})
    expect(screen.queryByText(/วันนี้แจ้งขาดผ้า/)).not.toBeInTheDocument()
  })

  // The reload after a landed report is what makes both appear without a
  // manual refresh: the second GET answers with the room as it now is.
  it('shows the chip and totals as soon as the reload answers', async () => {
    mockHkFetchMe.mockResolvedValue(meResponse())
    mockHkFetch.mockResolvedValue(jsonResponse({ success: true, room: ROOM, events: [] }))
    render(<HkRoomPage />)
    await screen.findByText('ห้อง 104')
    expect(screen.queryByText('ขาดผ้า')).not.toBeInTheDocument()

    fireEvent.click(screen.getByRole('button', { name: 'แจ้งขาดผ้า' }))
    await screen.findByRole('button', { name: 'ส่งแจ้ง' })
    fireEvent.click(screen.getByRole('button', { name: 'เพิ่ม ปลอกหมอน' }))

    // The POST lands, and the reload behind it returns the updated room.
    mockHkFetch
      .mockResolvedValueOnce(jsonResponse({ success: true, roomId: 7, reported: 1 }))
      .mockResolvedValueOnce(
        jsonResponse({
          success: true,
          room: { ...ROOM, linenShortageToday: true },
          events: [],
          linenShortages: [{ kind: 'pillowcase', qty: 1 }],
        })
      )
    fireEvent.click(screen.getByRole('button', { name: 'ส่งแจ้ง' }))

    expect(await screen.findByText('ขาดผ้า')).toBeInTheDocument()
    expect(screen.getByText('วันนี้แจ้งขาดผ้า: ปลอกหมอน 1')).toBeInTheDocument()
    // The success banner is unchanged by any of this.
    expect(screen.getByText('บันทึกแล้ว: แจ้งขาดผ้า')).toBeInTheDocument()
  })
})

describe('iHOTEL-unavailable note (R2a / CR-1)', () => {
  it('shows the Thai note when the backend fell back to the PMS mirror', async () => {
    await renderRoom({ legacyStatusStale: true })
    expect(screen.getByText(LEGACY_STATUS_STALE_NOTE)).toBeInTheDocument()
  })

  it('shows nothing when iHOTEL answered', async () => {
    await renderRoom({ legacyStatusStale: false })
    expect(screen.queryByText(LEGACY_STATUS_STALE_NOTE)).not.toBeInTheDocument()
  })

  // A rollback (or an older backend) omits the field. Silence must not be read
  // as "stale" — a banner that is always on is a banner nobody reads.
  it('shows nothing when the backend omits the flag entirely', async () => {
    await renderRoom({})
    expect(screen.queryByText(LEGACY_STATUS_STALE_NOTE)).not.toBeInTheDocument()
  })

  // The whole point of the fallback: the screen still WORKS. A stale note must
  // never come with a disabled or missing set of buttons.
  it('leaves the screen fully usable while stale', async () => {
    await renderRoom({ legacyStatusStale: true })
    expect(screen.getByText(LEGACY_STATUS_STALE_NOTE)).toBeInTheDocument()
    expect(screen.getByRole('button', { name: 'เสร็จแล้ว' })).toBeEnabled()
    expect(screen.getByRole('button', { name: /แจ้งห้องไม่สะอาด/ })).toBeEnabled()
  })
})

// ---------------------------------------------------------------------------
// Viewer mode — the `reception` grant. `/hk` now admits two identities: a maid
// (`housekeeping`) who reports, and reception (`reception`) who only READS the
// same screen. The whole action surface disappears for the viewer; every
// informational part stays, because reading them is the entire reason
// reception opened the room.
//
// The hiding is UX, not enforcement — the server 403s a viewer's POST either
// way — so these assertions are about what a receptionist can SEE and TAP, and
// the "no POST" assertion is the one that says she cannot tap her way to a
// write at all.
// ---------------------------------------------------------------------------

describe('viewer mode (reception grant — canReport: false)', () => {
  /** A room carrying one of everything informational: both header chips, the
   * ขาดผ้า chip, today's totals, an event, and the stale notice. */
  const RICH_DETAIL = {
    room: { ...ROOM, linenShortageToday: true },
    events: [
      {
        eventId: 11,
        status: 'started',
        badge: 'Q1001',
        name: 'สมศรี',
        at: '2026-09-01T03:00:00.000Z',
      },
    ],
    linenShortages: [{ kind: 'pillowcase', qty: 2 }],
    legacyStatusStale: true,
  }

  /** Render the room screen for an identity with the given `/me` extras. */
  async function renderAs(meExtras: Record<string, unknown>) {
    mockHkFetchMe.mockResolvedValue(meResponse(meExtras))
    mockHkFetch.mockResolvedValue(jsonResponse({ success: true, ...RICH_DETAIL }))
    render(<HkRoomPage />)
    await screen.findByText('ห้อง 104')
  }

  /** Every POST the page has issued — the thing that reaches the backend. */
  function postCalls() {
    return mockHkFetch.mock.calls.filter(
      ([, , init]) => (init as RequestInit | undefined)?.method === 'POST'
    )
  }

  const ACTION_BUTTONS = [
    'เริ่มทำความสะอาด',
    'เสร็จแล้ว',
    /แจ้งห้องไม่สะอาด/,
    'แจ้งขาดผ้า',
  ] as const

  it.each(ACTION_BUTTONS)('does not render the %s control', async (name) => {
    await renderAs({ canReport: false })
    expect(screen.queryByRole('button', { name })).not.toBeInTheDocument()
  })

  // The heading labels the action surface; with nothing under it, it would read
  // as a section that failed to load.
  it('does not render the รายงานความคืบหน้า heading', async () => {
    await renderAs({ canReport: false })
    expect(screen.queryByText('รายงานความคืบหน้า')).not.toBeInTheDocument()
  })

  // THE assertion: there is no control on this screen a receptionist could tap
  // to file anything — not a disabled one, not a hidden one. `markDirtyEnabled`
  // is TRUE in this `/me` payload, so this also proves the viewer gate wins
  // over the mark-dirty flag rather than merely agreeing with it.
  it('offers no tappable control at all, and fires no POST', async () => {
    await renderAs({ canReport: false })
    expect(screen.queryAllByRole('button')).toHaveLength(0)
    expect(postCalls()).toHaveLength(0)
  })

  it('still shows both header chips', async () => {
    await renderAs({ canReport: false })
    expect(screen.getByText(HK_STATUS_LABELS.dirty)).toBeInTheDocument()
    expect(screen.getByText('ยังไม่เริ่ม')).toBeInTheDocument()
  })

  // The ขาดผ้า chip is the fact reception most needs off this screen, and it
  // must not vanish with the button that files one.
  it('still shows the ขาดผ้า chip', async () => {
    await renderAs({ canReport: false })
    expect(screen.getByText('ขาดผ้า')).toBeInTheDocument()
  })

  it("still shows today's linen totals line", async () => {
    await renderAs({ canReport: false })
    expect(screen.getByText('วันนี้แจ้งขาดผ้า: ปลอกหมอน 2')).toBeInTheDocument()
  })

  it("still shows today's cleaning events", async () => {
    await renderAs({ canReport: false })
    expect(screen.getByText(/กำลังทำความสะอาด โดย สมศรี/)).toBeInTheDocument()
  })

  it('still shows the occupancy indicator and the legacy-stale notice', async () => {
    await renderAs({ canReport: false })
    expect(screen.getByText('มีแขกพัก')).toBeInTheDocument()
    expect(screen.getByText(LEGACY_STATUS_STALE_NOTE)).toBeInTheDocument()
  })

  // The other two identities are unchanged, and both must stay that way: an
  // explicit maid, and the deploy-skew case where an older backend sends no
  // `canReport` at all (it only ever admitted maids, so absent means maid).
  describe.each([
    ['canReport: true', { canReport: true }],
    ['an absent canReport (older backend)', {}],
  ])('%s renders the action surface exactly as before', (_label, meExtras) => {
    it.each(ACTION_BUTTONS)('renders the %s control', async (name) => {
      await renderAs(meExtras)
      expect(screen.getByRole('button', { name })).toBeEnabled()
    })

    it('renders the รายงานความคืบหน้า heading', async () => {
      await renderAs(meExtras)
      expect(screen.getByText('รายงานความคืบหน้า')).toBeInTheDocument()
    })

    // And the buttons still WORK — hiding them for a viewer must not have made
    // them inert for a maid.
    it('still files a report on เสร็จแล้ว', async () => {
      await renderAs(meExtras)
      fireEvent.click(screen.getByRole('button', { name: 'เสร็จแล้ว' }))

      await waitFor(() => expect(postCalls()).toHaveLength(1))
      expect(postCalls()[0][0]).toBe('/rooms/7/cleaning')
    })
  })
})
