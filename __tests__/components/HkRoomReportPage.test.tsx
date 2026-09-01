/**
 * @jest-environment jsdom
 *
 * Report HK v2 — ONE ROOM's daily report (`app/hk/rooms/[roomId]/report/page.tsx`).
 *
 * The screen is a CAPTURE-ZONE STEPPER, and what this suite owns is the
 * behaviour that makes it fast enough to use against the clock:
 *
 * 1. **One tap per zone pre-ticks that zone.** The camera lands a thumbnail and
 *    every item of the zone appears ครบ against it; she touches only what is
 *    wrong. A perfect room is four shots, four ถัดไป, ส่งรายงาน, ยืนยัน.
 * 2. **Every tick is photo-backed, and the submit says so.** The body carries
 *    all 22 ticks with the id of the picture that vouches for each, quantities
 *    on problems only, and it cannot fire while an upload is still in flight.
 * 3. **Nothing is lost.** Removing a photo unbinds its ticks instead of
 *    deleting them; a reload restores the draft and re-checks every photo id it
 *    remembered.
 * 4. **The two-sided evidence.** Reception sees the maid's photos grouped the
 *    way they were shot, with the items each one backs, then verifies with her
 *    OWN photos or returns with one of exactly three canned reasons.
 * 5. **Legacy still renders.** A report filed before the ticks existed has none;
 *    its v1 exception list is what the screen draws.
 *
 * The hk-lib helpers are mocked at the module boundary (the repo's established
 * pattern) — only the ones that touch the WIRE. The pure tick/queue/draft
 * helpers are the real ones, because what this suite asserts is what the PAGE
 * does with them; `hk-lib.test.ts` owns their arithmetic.
 */

import { act, fireEvent, render, screen, waitFor, within } from '@testing-library/react'

const mockHkFetch = jest.fn()
const mockHkFetchMe = jest.fn()
const mockFetchHkRoomReport = jest.fn()
const mockUploadHkReportPhoto = jest.fn()
const mockDeleteHkReportPhoto = jest.fn()
const mockFetchHkReportPhotoMeta = jest.fn()
const mockSubmitHkReport = jest.fn()
const mockVerifyHkReport = jest.fn()
const mockReturnHkReport = jest.fn()
// The canvas downscale needs a real browser (`createImageBitmap`); mocked here
// so this suite is about the report, not about jsdom's missing image codecs.
// `hk-lib.test.ts` owns the arithmetic (`downscaleDimensions`).
const mockDownscalePhoto = jest.fn()
const mockPush = jest.fn()

jest.mock('next/navigation', () => ({
  useParams: () => ({ roomId: '7' }),
  useRouter: () => ({ push: (...args: unknown[]) => mockPush(...args) }),
}))

jest.mock('@/app/hk/hk-lib', () => {
  const actual = jest.requireActual('@/app/hk/hk-lib')
  return {
    ...actual,
    hkFetch: (...args: unknown[]) => mockHkFetch(...args),
    hkFetchMe: (...args: unknown[]) => mockHkFetchMe(...args),
    fetchHkRoomReport: (...args: unknown[]) => mockFetchHkRoomReport(...args),
    uploadHkReportPhoto: (...args: unknown[]) => mockUploadHkReportPhoto(...args),
    deleteHkReportPhoto: (...args: unknown[]) => mockDeleteHkReportPhoto(...args),
    fetchHkReportPhotoMeta: (...args: unknown[]) => mockFetchHkReportPhotoMeta(...args),
    submitHkReport: (...args: unknown[]) => mockSubmitHkReport(...args),
    verifyHkReport: (...args: unknown[]) => mockVerifyHkReport(...args),
    returnHkReport: (...args: unknown[]) => mockReturnHkReport(...args),
    downscalePhoto: (...args: unknown[]) => mockDownscalePhoto(...args),
  }
})

import HkRoomReportPage from '@/app/hk/rooms/[roomId]/report/page'
import {
  applyZoneCapture,
  REPORT_ITEMS,
  REPORT_SUBMITTED_NOTICE,
  REPORT_ZONES,
  reportDraftKey,
  reportItemZone,
} from '@/app/hk/hk-lib'

const ROOM = {
  roomId: 7,
  roomNo: '104',
  floor: 1,
  building: null,
  roomClean: false,
  cleaning: null,
  occupancy: 'occupied',
}

/** The four shots of a walked room, one per zone, in shooting order. */
const ZONE_PHOTO_IDS: Record<string, number> = {
  bed: 101,
  desk: 102,
  bathroom: 103,
  general: 104,
}

function jsonResponse(body: unknown, status = 200) {
  return { ok: status >= 200 && status < 300, status, json: async () => body }
}

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

function photoFile() {
  return new File(['x'], 'photo.jpg', { type: 'image/jpeg' })
}

/** Let every queued promise (downscale → upload → setState) settle. */
async function flush() {
  await act(async () => {
    await new Promise((resolve) => setTimeout(resolve, 0))
  })
}

/** Render the screen for one room + one day's report. `report: null` is the
 *  ยังไม่ส่ง case. */
async function renderReport({
  report = null,
  me = {},
  room = {},
}: {
  report?: Record<string, unknown> | null
  me?: Record<string, unknown>
  room?: Record<string, unknown>
} = {}) {
  mockHkFetchMe.mockResolvedValue(meResponse(me))
  mockHkFetch.mockResolvedValue(
    jsonResponse({ success: true, room: { ...ROOM, ...room }, events: [] })
  )
  mockFetchHkRoomReport.mockResolvedValue({
    date: '2026-09-02',
    room: { roomId: 7, roomNo: '104', floor: 1, building: null, report },
    report,
  })
  render(<HkRoomReportPage />)
  await screen.findByText('ห้อง 104')
  await flush()
}

/** Take ONE shot in the zone the stepper is currently on. */
async function shoot(zoneLabel: string, photoId: number) {
  mockUploadHkReportPhoto.mockResolvedValue({ photoId, bytes: 1024 })
  await act(async () => {
    fireEvent.change(screen.getByLabelText(`ถ่ายรูป${zoneLabel}`), {
      target: { files: [photoFile()] },
    })
  })
  await flush()
}

/** Shoot the current zone and step forward. */
async function shootAndAdvance(index: number) {
  const zone = REPORT_ZONES[index]
  await shoot(zone.label, ZONE_PHOTO_IDS[zone.zone])
  fireEvent.click(
    screen.getByRole('button', {
      name: index === REPORT_ZONES.length - 1 ? 'ตรวจทาน' : 'ถัดไป',
    })
  )
  await flush()
}

/** The whole room, zone by zone, ending on the review step. */
async function walkTheRoom() {
  for (let i = 0; i < REPORT_ZONES.length; i += 1) await shootAndAdvance(i)
}

/** The body a walked room should send: 22 ticks in the paper form's order,
 *  each carrying the id of its zone's shot. */
function expectedTicks(
  problems: Record<string, { state: string; qty: number }> = {},
  photoIds: Record<string, number> = ZONE_PHOTO_IDS
) {
  return REPORT_ITEMS.map(({ item }) => {
    const photoId = photoIds[reportItemZone(item) as string]
    const problem = problems[item]
    return problem
      ? { item, state: problem.state, qty: problem.qty, photoId }
      : { item, state: 'ok', photoId }
  })
}

const MAID_PHOTOS = [
  { photoId: 31, side: 'maid', zone: 'bed', bytes: 910 },
  { photoId: 32, side: 'maid', zone: 'bathroom', bytes: 880 },
]

const TICKS = [
  { item: 'pillow', state: 'ok', qty: null, photoId: 31 },
  { item: 'duvet', state: 'ok', qty: null, photoId: 31 },
  { item: 'bath_towel', state: 'missing', qty: 2, photoId: 32 },
]

const SUBMITTED_REPORT = {
  reportId: 55,
  roomId: 7,
  date: '2026-09-02',
  status: 'submitted',
  roomStatus: 'co',
  allItemsOk: false,
  problemCount: 1,
  ticks: TICKS,
  photos: MAID_PHOTOS,
  // Kept populated for bundles that predate the ticks.
  items: [{ item: 'bath_towel', problem: 'missing', qty: 2 }],
  returnReason: null,
  parentReportId: null,
  submittedBy: { badge: 'Q1001', name: 'สมศรี' },
  submittedAt: '2026-09-02T03:00:00.000Z',
  verifiedBy: null,
  verifiedAt: null,
  maidPhotoIds: [31, 32],
  receptionPhotoIds: [],
}

const RETURNED_REPORT = {
  ...SUBMITTED_REPORT,
  status: 'returned',
  returnReason: 'photos_unclear',
  verifiedBy: { badge: 'R2002', name: 'มานี' },
  verifiedAt: '2026-09-02T04:00:00.000Z',
}

const VERIFIED_REPORT = {
  ...SUBMITTED_REPORT,
  status: 'verified',
  verifiedBy: { badge: 'R2002', name: 'มานี' },
  verifiedAt: '2026-09-02T04:00:00.000Z',
  photos: [...MAID_PHOTOS, { photoId: 41, side: 'reception', zone: null, bytes: 700 }],
  receptionPhotoIds: [41],
}

/** A report filed before v2: no ticks, no photo metadata, just exceptions. */
const LEGACY_REPORT = {
  reportId: 40,
  roomId: 7,
  date: '2026-09-02',
  status: 'submitted',
  roomStatus: 'co',
  allItemsOk: false,
  items: [{ item: 'tv_remote', problem: 'damaged', qty: 2 }],
  ticks: [],
  returnReason: null,
  parentReportId: null,
  submittedBy: { badge: 'Q1001', name: 'สมศรี' },
  submittedAt: '2026-09-02T03:00:00.000Z',
  verifiedBy: null,
  verifiedAt: null,
  maidPhotoIds: [31, 32],
  receptionPhotoIds: [],
}

beforeEach(() => {
  jest.clearAllMocks()
  localStorage.clear()
  sessionStorage.clear()
  mockDownscalePhoto.mockImplementation(async (file: Blob) => file)
  mockUploadHkReportPhoto.mockResolvedValue({ photoId: 1, bytes: 1024 })
  mockDeleteHkReportPhoto.mockResolvedValue(true)
  mockFetchHkReportPhotoMeta.mockResolvedValue(null)
  mockSubmitHkReport.mockResolvedValue({ ...SUBMITTED_REPORT, reportId: 56 })
  mockVerifyHkReport.mockResolvedValue(VERIFIED_REPORT)
  mockReturnHkReport.mockResolvedValue(RETURNED_REPORT)
})

// ---------------------------------------------------------------------------
// THE STEPPER — the shooting order, and the pre-tick that is the whole speed of
// the flow.
// ---------------------------------------------------------------------------

describe('the zone stepper', () => {
  it('opens on เตียง and walks the zones in the vocabulary’s order', async () => {
    await renderReport()
    expect(screen.getByTestId('hk-report-form')).toBeInTheDocument()
    for (let i = 0; i < REPORT_ZONES.length; i += 1) {
      expect(
        within(screen.getByTestId('hk-zone-step')).getByText(
          new RegExp(`โซน ${i + 1}/4 · ${REPORT_ZONES[i].label}`)
        )
      ).toBeInTheDocument()
      expect(screen.getByLabelText(`ถ่ายรูป${REPORT_ZONES[i].label}`)).toBeInTheDocument()
      fireEvent.click(
        screen.getByRole('button', {
          name: i === REPORT_ZONES.length - 1 ? 'ตรวจทาน' : 'ถัดไป',
        })
      )
    }
    expect(screen.getByTestId('hk-report-review')).toBeInTheDocument()
  })

  it('shows only the current zone’s items', async () => {
    await renderReport()
    const items = screen.getByTestId('hk-zone-items')
    // เตียง: ผ้าปูที่นอน is here, กาน้ำร้อน is a desk item and is not.
    expect(within(items).getByText('ผ้าปูที่นอน')).toBeInTheDocument()
    expect(within(items).queryByText('กาน้ำร้อน')).not.toBeInTheDocument()
  })

  // THE pre-tick: one tap, and the zone is attested.
  it('pre-ticks the zone ครบ the moment the shot lands', async () => {
    await renderReport()
    expect(screen.getAllByText('ถ่ายรูปโซนนี้ก่อน').length).toBe(REPORT_ZONES[0].items.length)

    await shoot('เตียง', 101)

    for (const item of REPORT_ZONES[0].items) {
      const row = screen.getByTestId(`hk-tick-${item}`)
      expect(within(row).getByRole('button', { name: /ครบ$/ })).toBeInTheDocument()
    }
    expect(screen.queryByText('ถ่ายรูปโซนนี้ก่อน')).not.toBeInTheDocument()
    expect(screen.getByTestId('hk-upload-status')).toHaveTextContent('อัปโหลดแล้ว 1/1')
  })

  it('leaves the other zones untouched — a shot attests what it shows', async () => {
    await renderReport()
    await shoot('เตียง', 101)
    fireEvent.click(screen.getByRole('button', { name: 'ถัดไป' }))
    expect(screen.getAllByText('ถ่ายรูปโซนนี้ก่อน').length).toBe(REPORT_ZONES[1].items.length)
  })
})

// ---------------------------------------------------------------------------
// Ticking — one tap cycles, quantities appear only on problems.
// ---------------------------------------------------------------------------

describe('ticking an item', () => {
  it('cycles ครบ → หาย → ชำรุด → ครบ on one tap each', async () => {
    await renderReport()
    await shoot('เตียง', 101)

    fireEvent.click(screen.getByRole('button', { name: 'หมอน ครบ' }))
    expect(screen.getByRole('button', { name: 'หมอน หาย' })).toBeInTheDocument()
    fireEvent.click(screen.getByRole('button', { name: 'หมอน หาย' }))
    expect(screen.getByRole('button', { name: 'หมอน ชำรุด' })).toBeInTheDocument()
    fireEvent.click(screen.getByRole('button', { name: 'หมอน ชำรุด' }))
    expect(screen.getByRole('button', { name: 'หมอน ครบ' })).toBeInTheDocument()
  })

  it('opens the quantity stepper on a problem and nowhere else', async () => {
    await renderReport()
    await shoot('เตียง', 101)
    expect(screen.queryByRole('button', { name: 'เพิ่ม หมอน' })).not.toBeInTheDocument()

    fireEvent.click(screen.getByRole('button', { name: 'หมอน ครบ' }))
    fireEvent.click(screen.getByRole('button', { name: 'เพิ่ม หมอน' }))
    fireEvent.click(screen.getByRole('button', { name: 'เพิ่ม หมอน' }))
    expect(within(screen.getByTestId('hk-tick-pillow')).getByText('3')).toBeInTheDocument()

    // Back to ครบ — two more taps, and the quantity is gone with it. The row's
    // name is always the state it is IN, which is what she reads.
    fireEvent.click(screen.getByRole('button', { name: 'หมอน หาย' }))
    fireEvent.click(screen.getByRole('button', { name: 'หมอน ชำรุด' }))
    expect(screen.getByRole('button', { name: 'หมอน ครบ' })).toBeInTheDocument()
    expect(screen.queryByRole('button', { name: 'เพิ่ม หมอน' })).not.toBeInTheDocument()
  })

  // Allowed, never demanded — the server does not enforce close-ups.
  it('rebinds one tick to its close-up, leaving its neighbours on the zone shot', async () => {
    await renderReport()
    await shoot('เตียง', 101)
    fireEvent.click(screen.getByRole('button', { name: 'หมอน ครบ' }))

    mockUploadHkReportPhoto.mockResolvedValue({ photoId: 999, bytes: 512 })
    await act(async () => {
      fireEvent.change(screen.getByLabelText('ถ่ายรูปใกล้ หมอน'), {
        target: { files: [photoFile()] },
      })
    })
    await flush()

    // Two shots in the zone now, and the pillow is on the second one.
    expect(screen.getByTestId('hk-upload-status')).toHaveTextContent('อัปโหลดแล้ว 2/2')
    expect(
      within(screen.getByTestId('hk-tick-pillow')).getByRole('button', {
        name: 'เปลี่ยนรูปของ หมอน',
      })
    ).toHaveTextContent('รูปที่ 2/2')
    expect(
      within(screen.getByTestId('hk-tick-duvet')).getByRole('button', {
        name: 'เปลี่ยนรูปของ ผ้านวม',
      })
    ).toHaveTextContent('รูปที่ 1/2')
  })
})

// ---------------------------------------------------------------------------
// Photo management — the rule that a removal never silently un-attests a room.
// ---------------------------------------------------------------------------

describe('managing the photos', () => {
  it('unbinds the ticks a removed photo backed, and keeps every one of them', async () => {
    await renderReport()
    await shoot('เตียง', 101)
    fireEvent.click(screen.getByRole('button', { name: 'หมอน ครบ' }))

    fireEvent.click(screen.getByRole('button', { name: 'ลบรูปที่ 1 โซนเตียง' }))
    await flush()

    // The tick survives — with its state and its count — and says what it owes.
    expect(screen.getByRole('button', { name: 'หมอน หาย' })).toBeInTheDocument()
    expect(screen.getAllByText('ต้องถ่ายรูปใหม่').length).toBe(REPORT_ZONES[0].items.length)
    // Best effort, and never in the maid's way: the row is deleted behind her.
    expect(mockDeleteHkReportPhoto).toHaveBeenCalledWith('hfhotel', 101)
  })

  it('re-binds those ticks when she shoots the zone again', async () => {
    await renderReport()
    await shoot('เตียง', 101)
    fireEvent.click(screen.getByRole('button', { name: 'หมอน ครบ' }))
    fireEvent.click(screen.getByRole('button', { name: 'ลบรูปที่ 1 โซนเตียง' }))
    await flush()

    await shoot('เตียง', 111)
    expect(screen.queryByText('ต้องถ่ายรูปใหม่')).not.toBeInTheDocument()
    // Her judgement is still hers — the retake restored evidence, not answers.
    expect(screen.getByRole('button', { name: 'หมอน หาย' })).toBeInTheDocument()
  })

  // Remove-and-shoot-again in one tap, from the screen where she decided the
  // picture would not do — and her answers are untouched by it.
  it('retakes a shot from the viewer, re-binding the ticks it backed', async () => {
    await renderReport()
    await shoot('เตียง', 101)
    fireEvent.click(screen.getByRole('button', { name: 'หมอน ครบ' }))
    fireEvent.click(screen.getByRole('button', { name: 'ดูรูปที่ 1 โซนเตียง' }))

    mockUploadHkReportPhoto.mockResolvedValue({ photoId: 202, bytes: 1024 })
    await act(async () => {
      fireEvent.change(screen.getByLabelText('ถ่ายรูปนี้ใหม่'), {
        target: { files: [photoFile()] },
      })
    })
    await flush()

    expect(mockDeleteHkReportPhoto).toHaveBeenCalledWith('hfhotel', 101)
    expect(screen.getByTestId('hk-upload-status')).toHaveTextContent('อัปโหลดแล้ว 1/1')
    expect(screen.queryByText('ต้องถ่ายรูปใหม่')).not.toBeInTheDocument()
    expect(screen.getByRole('button', { name: 'หมอน หาย' })).toBeInTheDocument()
  })

  it('opens one shot full-screen with the list of items it backs', async () => {
    await renderReport()
    await shoot('เตียง', 101)
    fireEvent.click(screen.getByRole('button', { name: 'ดูรูปที่ 1 โซนเตียง' }))
    const viewer = screen.getByTestId('hk-photo-viewer')
    expect(within(viewer).getByText(/หมอน/)).toBeInTheDocument()
    expect(within(viewer).getByText(/ผ้าปูที่นอน/)).toBeInTheDocument()
  })
})

// ---------------------------------------------------------------------------
// Filing it — the blocked submit, then the exact body.
// ---------------------------------------------------------------------------

describe('filing the report', () => {
  it('will not submit while an upload is still in flight, and says how far along it is', async () => {
    await renderReport()
    await shootAndAdvance(0)
    await shootAndAdvance(1)
    await shootAndAdvance(2)
    // The last zone's upload never lands.
    mockUploadHkReportPhoto.mockReturnValue(new Promise(() => {}))
    await act(async () => {
      fireEvent.change(screen.getByLabelText('ถ่ายรูปทั่วไป'), {
        target: { files: [photoFile()] },
      })
    })
    await flush()
    fireEvent.click(screen.getByRole('button', { name: 'ตรวจทาน' }))

    expect(screen.getByRole('button', { name: /ส่งรายงาน/ })).toBeDisabled()
    expect(screen.getByTestId('hk-submit-blocked')).toHaveTextContent('อัปโหลดแล้ว 3/4')
  })

  it('will not submit a room whose zones are not all shot', async () => {
    await renderReport()
    await shootAndAdvance(0)
    for (let i = 1; i < REPORT_ZONES.length; i += 1) {
      fireEvent.click(
        screen.getByRole('button', {
          name: i === REPORT_ZONES.length - 1 ? 'ตรวจทาน' : 'ถัดไป',
        })
      )
    }
    expect(screen.getByRole('button', { name: /ส่งรายงาน/ })).toBeDisabled()
    expect(screen.getByTestId('hk-submit-blocked')).toHaveTextContent('ยังติ๊กไม่ครบ')
  })

  it('arms the submit once every zone is shot and every upload has landed', async () => {
    await renderReport()
    await walkTheRoom()
    expect(screen.getByRole('button', { name: /ส่งรายงาน/ })).toBeEnabled()
  })

  it('the first tap issues NO request — it only arms the confirm', async () => {
    await renderReport()
    await walkTheRoom()
    fireEvent.click(screen.getByRole('button', { name: /ส่งรายงาน/ }))
    await screen.findByText('ยืนยันส่งรายงาน ห้อง 104?')
    expect(mockSubmitHkReport).not.toHaveBeenCalled()
  })

  // THE contract: 22 photo-backed ticks, quantities on problems only, and the
  // room status the form was already prefilled with.
  it('posts all 22 ticks, each naming the photo that backs it', async () => {
    await renderReport({ room: { occupancy: 'vacant', expectedDeparture: true } })
    await shoot('เตียง', ZONE_PHOTO_IDS.bed)
    fireEvent.click(screen.getByRole('button', { name: 'ถัดไป' }))
    await shoot('โต๊ะและมินิบาร์', ZONE_PHOTO_IDS.desk)
    // One problem, counted twice.
    fireEvent.click(screen.getByRole('button', { name: 'กาน้ำร้อน ครบ' }))
    fireEvent.click(screen.getByRole('button', { name: 'เพิ่ม กาน้ำร้อน' }))
    fireEvent.click(screen.getByRole('button', { name: 'ถัดไป' }))
    await shootAndAdvance(2)
    await shootAndAdvance(3)

    fireEvent.click(screen.getByRole('button', { name: /ส่งรายงาน/ }))
    fireEvent.click(await screen.findByRole('button', { name: 'ยืนยัน' }))

    await waitFor(() => {
      expect(mockSubmitHkReport).toHaveBeenCalledWith('hfhotel', 7, {
        roomStatus: 'co',
        ticks: expectedTicks({ kettle: { state: 'missing', qty: 2 } }),
      })
    })
    const body = mockSubmitHkReport.mock.calls[0][2]
    expect(body.ticks).toHaveLength(22)
    // ok ticks carry no quantity at all — one that does is a 400.
    expect(body.ticks.filter((t: { qty?: number }) => t.qty !== undefined)).toHaveLength(1)
    expect(body).not.toHaveProperty('extraPhotoIds')
    expect(body).not.toHaveProperty('parentReportId')
  })

  it('sends a shot that backs no tick as an extra', async () => {
    await renderReport()
    await shoot('เตียง', ZONE_PHOTO_IDS.bed)
    // A second bed shot nothing is rebound to.
    await shoot('เตียง', 900)
    fireEvent.click(screen.getByRole('button', { name: 'ถัดไป' }))
    await shootAndAdvance(1)
    await shootAndAdvance(2)
    await shootAndAdvance(3)

    fireEvent.click(screen.getByRole('button', { name: /ส่งรายงาน/ }))
    fireEvent.click(await screen.findByRole('button', { name: 'ยืนยัน' }))
    await waitFor(() => expect(mockSubmitHkReport).toHaveBeenCalled())
    expect(mockSubmitHkReport.mock.calls[0][2].extraPhotoIds).toEqual([900])
  })

  it('hands the banner to the day overview, goes there, and clears the draft', async () => {
    await renderReport()
    await walkTheRoom()
    expect(sessionStorage.getItem(reportDraftKey('hfhotel', 7, '2026-09-02'))).toBeTruthy()

    fireEvent.click(screen.getByRole('button', { name: /ส่งรายงาน/ }))
    fireEvent.click(await screen.findByRole('button', { name: 'ยืนยัน' }))

    await waitFor(() => expect(mockPush).toHaveBeenCalledWith('/hk/report'))
    expect(sessionStorage.getItem('hk.reportNotice')).toBe(REPORT_SUBMITTED_NOTICE)
    expect(sessionStorage.getItem(reportDraftKey('hfhotel', 7, '2026-09-02'))).toBeNull()
  })

  // A failed write must keep her on the stepper with everything she entered —
  // a maid who has to re-photograph a room stops filing reports.
  it('keeps the review and the photos when the write fails', async () => {
    mockSubmitHkReport.mockRejectedValue(new Error('บันทึกไม่สำเร็จ กรุณาลองใหม่'))
    await renderReport()
    await walkTheRoom()
    fireEvent.click(screen.getByRole('button', { name: /ส่งรายงาน/ }))
    fireEvent.click(await screen.findByRole('button', { name: 'ยืนยัน' }))

    expect(await screen.findByText('บันทึกไม่สำเร็จ กรุณาลองใหม่')).toBeInTheDocument()
    expect(mockPush).not.toHaveBeenCalled()
    expect(screen.getByTestId('hk-report-review')).toBeInTheDocument()
    expect(screen.getByTestId('hk-upload-status')).toHaveTextContent('อัปโหลดแล้ว 4/4')
  })
})

// ---------------------------------------------------------------------------
// The review step — every tick, grouped as it was shot, with the room status.
// ---------------------------------------------------------------------------

describe('the review step', () => {
  it('lists all 22 ticks with the picture that backs each', async () => {
    await renderReport()
    await walkTheRoom()
    const review = screen.getByTestId('hk-report-review')
    for (const { item } of REPORT_ITEMS) {
      expect(within(review).getByTestId(`hk-review-${item}`)).toBeInTheDocument()
    }
    expect(within(review).getByText('เตียง')).toBeInTheDocument()
    expect(within(review).getByText('ห้องน้ำ')).toBeInTheDocument()
  })

  // Prefilled, never locked: the perfect room costs her no taps here at all.
  it('prefills the room status from the room’s own facts', async () => {
    await renderReport({ room: { occupancy: 'occupied' } })
    await walkTheRoom()
    expect(screen.getByRole('button', { name: 'SO พักต่อ' })).toHaveAttribute(
      'aria-pressed',
      'true'
    )
    fireEvent.click(screen.getByRole('button', { name: 'OO รอซ่อม' }))
    expect(screen.getByRole('button', { name: 'OO รอซ่อม' })).toHaveAttribute(
      'aria-pressed',
      'true'
    )
  })
})

// ---------------------------------------------------------------------------
// THE DRAFT — a phone that locks mid-room is the design case.
// ---------------------------------------------------------------------------

describe('the draft survives a reload', () => {
  /** What the page would have written after shooting เตียง and โต๊ะ. */
  function storedDraft() {
    let ticks = applyZoneCapture({}, 'bed', 'photo-0')
    ticks = applyZoneCapture(ticks, 'desk', 'photo-1')
    return JSON.stringify({
      roomStatus: 'vc',
      step: 0,
      ticks,
      photos: [
        {
          key: 'photo-0',
          zone: 'bed',
          photoId: 900,
          bytes: 10,
          attempts: 1,
          status: 'uploaded',
          failedAt: null,
        },
        {
          key: 'photo-1',
          zone: 'desk',
          photoId: 901,
          bytes: 10,
          attempts: 1,
          status: 'uploaded',
          failedAt: null,
        },
      ],
      seq: 2,
    })
  }

  it('writes the draft as she taps', async () => {
    await renderReport()
    await shoot('เตียง', 101)
    fireEvent.click(screen.getByRole('button', { name: 'หมอน ครบ' }))
    await flush()
    const raw = sessionStorage.getItem(reportDraftKey('hfhotel', 7, '2026-09-02'))
    expect(raw).toBeTruthy()
    const draft = JSON.parse(raw as string)
    expect(draft.ticks.pillow).toEqual({ state: 'missing', qty: 1, photo: 'photo-0' })
    expect(draft.photos[0]).toMatchObject({ photoId: 101, status: 'uploaded' })
  })

  it('restores the ticks and re-checks every photo id it remembered', async () => {
    sessionStorage.setItem(reportDraftKey('hfhotel', 7, '2026-09-02'), storedDraft())
    // 900 is still hers and unattached; 901 is gone.
    mockFetchHkReportPhotoMeta.mockImplementation(async (_branch: string, photoId: number) =>
      photoId === 900 ? { photoId: 900, side: 'maid', zone: 'bed', attached: false } : null
    )
    await renderReport()

    expect(mockFetchHkReportPhotoMeta).toHaveBeenCalledWith('hfhotel', 900)
    expect(mockFetchHkReportPhotoMeta).toHaveBeenCalledWith('hfhotel', 901)
    // เตียง came back whole.
    expect(screen.getByRole('button', { name: 'หมอน ครบ' })).toBeInTheDocument()
    expect(screen.queryByText('ต้องถ่ายรูปใหม่')).not.toBeInTheDocument()
    expect(screen.getByTestId('hk-upload-status')).toHaveTextContent('อัปโหลดแล้ว 1/1')
    expect(
      screen.getByText('รูปบางรูปใช้ไม่ได้แล้ว กรุณาถ่ายใหม่ในโซนที่ยังไม่มีรูป')
    ).toBeInTheDocument()
  })

  // The ticks of the dropped photo are still there — she keeps her judgements
  // and owes only the pictures.
  it('keeps the ticks whose photo did not survive, marked as owing one', async () => {
    sessionStorage.setItem(reportDraftKey('hfhotel', 7, '2026-09-02'), storedDraft())
    mockFetchHkReportPhotoMeta.mockImplementation(async (_branch: string, photoId: number) =>
      photoId === 900 ? { photoId: 900, side: 'maid', zone: 'bed', attached: false } : null
    )
    await renderReport()
    fireEvent.click(screen.getByRole('button', { name: 'ถัดไป' }))

    expect(screen.getByRole('button', { name: 'กาน้ำร้อน ครบ' })).toBeInTheDocument()
    expect(screen.getAllByText('ต้องถ่ายรูปใหม่').length).toBe(REPORT_ZONES[1].items.length)
  })

  // A photo the server says is already attached cannot back a new report.
  it('drops a photo that has since been attached to a submission', async () => {
    sessionStorage.setItem(reportDraftKey('hfhotel', 7, '2026-09-02'), storedDraft())
    mockFetchHkReportPhotoMeta.mockImplementation(async (_branch: string, photoId: number) => ({
      photoId,
      side: 'maid',
      zone: 'bed',
      attached: true,
    }))
    await renderReport()
    expect(screen.getAllByText('ต้องถ่ายรูปใหม่').length).toBe(REPORT_ZONES[0].items.length)
  })
})

// ---------------------------------------------------------------------------
// RETURNED — the reason, the tick prefill, and the parent link.
// ---------------------------------------------------------------------------

describe('a returned report', () => {
  it('names the canned reason before the stepper', async () => {
    await renderReport({ report: RETURNED_REPORT })
    expect(
      within(screen.getByTestId('hk-report-returned-banner')).getByText(
        /ส่งกลับให้แก้ไข: รูปไม่ชัดเจน/
      )
    ).toBeInTheDocument()
  })

  // Her ticks come back; her PHOTOS do not — reception rejected the evidence,
  // and re-sending it is not a fix.
  it('prefills her ticks and starts again at zone 1 with no photos', async () => {
    await renderReport({ report: RETURNED_REPORT })
    expect(screen.getByTestId('hk-report-form')).toBeInTheDocument()
    expect(within(screen.getByTestId('hk-zone-step')).getByText(/โซน 1\/4/)).toBeInTheDocument()
    expect(screen.queryByTestId('hk-upload-status')).not.toBeInTheDocument()
    // pillow was ok, and it is owed a photo like everything else.
    expect(screen.getByRole('button', { name: 'หมอน ครบ' })).toBeInTheDocument()
    expect(screen.getAllByText('ต้องถ่ายรูปใหม่').length).toBeGreaterThan(0)
  })

  it('resubmits as a NEW report carrying parentReportId', async () => {
    await renderReport({ report: RETURNED_REPORT })
    await walkTheRoom()
    fireEvent.click(screen.getByRole('button', { name: /ส่งรายงาน/ }))
    fireEvent.click(await screen.findByRole('button', { name: 'ยืนยัน' }))

    await waitFor(() => expect(mockSubmitHkReport).toHaveBeenCalled())
    const body = mockSubmitHkReport.mock.calls[0][2]
    expect(body.parentReportId).toBe(55)
    expect(body.roomStatus).toBe('co')
    // The หาย she reported is still หาย, now backed by a fresh bathroom shot.
    expect(body.ticks).toEqual(expectedTicks({ bath_towel: { state: 'missing', qty: 2 } }))
  })
})

// ---------------------------------------------------------------------------
// RECEPTION'S VERIFY. The desk side, and only on a submitted report.
// ---------------------------------------------------------------------------

describe('reception’s verify view', () => {
  const receptionMe = { canReport: false }

  /** Capture one photo through reception's strip. */
  async function attachReceptionPhoto(photoId: number) {
    mockUploadHkReportPhoto.mockResolvedValue({ photoId, bytes: 1024 })
    await act(async () => {
      fireEvent.change(screen.getByLabelText('ถ่ายรูปการตรวจ'), {
        target: { files: [photoFile()] },
      })
    })
    await screen.findByAltText(`รูปที่แนบ ${photoId}`)
  }

  it('never shows reception the maid’s stepper', async () => {
    await renderReport({ report: null, me: receptionMe })
    expect(screen.queryByTestId('hk-report-form')).not.toBeInTheDocument()
    expect(screen.getByTestId('hk-report-empty')).toBeInTheDocument()
  })

  // The layout IS the feature: reception judges each picture against the ticks
  // it vouches for.
  it('groups the maid’s photos by capture zone, each with the items it backs', async () => {
    await renderReport({ report: SUBMITTED_REPORT, me: receptionMe })
    const evidence = screen.getByTestId('hk-report-zone-evidence')
    const bed = within(evidence).getByTestId('hk-report-zone-bed')
    expect(within(bed).getByText('เตียง')).toBeInTheDocument()
    expect(within(bed).getByText(/หมอน · ครบ/)).toBeInTheDocument()
    expect(within(bed).getByText(/ผ้านวม · ครบ/)).toBeInTheDocument()
    expect(within(bed).getByAltText('รูปโซนเตียง 31')).toHaveAttribute(
      'src',
      '/hk/api/report-photos/31?branch=hfhotel'
    )
    // The problem, with its quantity, under the picture that shows it.
    const bathroom = within(evidence).getByTestId('hk-report-zone-bathroom')
    expect(within(bathroom).getByText(/ผ้าขนหนู \(รวมสีฟ้า\) · หาย 2/)).toBeInTheDocument()
  })

  it('opens one photo full-screen and walks to the next', async () => {
    await renderReport({ report: SUBMITTED_REPORT, me: receptionMe })
    fireEvent.click(screen.getByRole('button', { name: 'ดูรูป 31 โซนเตียง' }))
    const viewer = screen.getByTestId('hk-photo-viewer')
    expect(within(viewer).getByText('รูปที่ 1/2')).toBeInTheDocument()
    fireEvent.click(within(viewer).getByRole('button', { name: 'รูปถัดไป' }))
    expect(
      within(screen.getByTestId('hk-photo-viewer')).getByText('รูปที่ 2/2')
    ).toBeInTheDocument()
  })

  it('also lists what she reported, grouped by zone', async () => {
    await renderReport({ report: SUBMITTED_REPORT, me: receptionMe })
    const summary = screen.getByTestId('hk-report-summary')
    expect(within(summary).getByText('CO เช็คเอาท์')).toBeInTheDocument()
    const ticks = within(summary).getByTestId('hk-report-ticks')
    expect(within(ticks).getByText('ผ้าขนหนู (รวมสีฟ้า)')).toBeInTheDocument()
    expect(within(ticks).getByText('2')).toBeInTheDocument()
  })

  // Reception's OWN photos are what make a verify a walk-up rather than a desk
  // stamp — the two-sided evidence IS the feature.
  it('will not let her verify without a photo of her own', async () => {
    await renderReport({ report: SUBMITTED_REPORT, me: receptionMe })
    expect(screen.getByRole('button', { name: /ยืนยันการตรวจ/ })).toBeDisabled()
  })

  it('verifies with her own photo ids, behind the confirm', async () => {
    await renderReport({ report: SUBMITTED_REPORT, me: receptionMe })
    await attachReceptionPhoto(41)

    fireEvent.click(screen.getByRole('button', { name: /ยืนยันการตรวจ/ }))
    await screen.findByText('ยืนยันว่าตรวจ ห้อง 104 แล้ว?')
    expect(mockVerifyHkReport).not.toHaveBeenCalled()

    fireEvent.click(screen.getByRole('button', { name: 'ยืนยัน' }))
    await waitFor(() => expect(mockVerifyHkReport).toHaveBeenCalledWith('hfhotel', 55, [41]))
    expect(mockPush).toHaveBeenCalledWith('/hk/report')
  })

  // Canned only — the picker IS the whole rejection vocabulary, and there is
  // no free-text field anywhere on this surface.
  it('will not let her send a report back without choosing a reason', async () => {
    await renderReport({ report: SUBMITTED_REPORT, me: receptionMe })
    expect(screen.getByRole('button', { name: /ส่งกลับ$/ })).toBeDisabled()
  })

  it('returns with the canned reason and NO photos, behind the confirm', async () => {
    await renderReport({ report: SUBMITTED_REPORT, me: receptionMe })
    fireEvent.click(screen.getByRole('button', { name: 'รูปไม่ชัดเจน' }))

    fireEvent.click(screen.getByRole('button', { name: /ส่งกลับ$/ }))
    await screen.findByText('ส่งกลับ ห้อง 104 เพราะ รูปไม่ชัดเจน?')
    expect(mockReturnHkReport).not.toHaveBeenCalled()

    fireEvent.click(screen.getByRole('button', { name: 'ยืนยัน' }))
    await waitFor(() =>
      expect(mockReturnHkReport).toHaveBeenCalledWith('hfhotel', 55, 'photos_unclear')
    )
    expect(mockPush).toHaveBeenCalledWith('/hk/report')
  })

  it('offers exactly the three canned reasons', async () => {
    await renderReport({ report: SUBMITTED_REPORT, me: receptionMe })
    const verify = screen.getByTestId('hk-report-verify')
    expect(within(verify).getByRole('button', { name: 'ยังไม่สะอาด' })).toBeInTheDocument()
    expect(
      within(verify).getByRole('button', { name: 'อุปกรณ์ไม่ตรงกับที่รายงาน' })
    ).toBeInTheDocument()
    expect(within(verify).getByRole('button', { name: 'รูปไม่ชัดเจน' })).toBeInTheDocument()
  })
})

// ---------------------------------------------------------------------------
// The role wall, from the other side.
// ---------------------------------------------------------------------------

describe('a maid never verifies', () => {
  // Including one who also holds the reception grant — `canReport` has already
  // resolved that identity to the maid side, and this screen honours it.
  it('gives a maid looking at her own submitted report no verify controls at all', async () => {
    await renderReport({ report: SUBMITTED_REPORT })
    expect(screen.queryByTestId('hk-report-verify')).not.toBeInTheDocument()
    expect(screen.queryByRole('button', { name: /ยืนยันการตรวจ/ })).not.toBeInTheDocument()
    expect(screen.queryByRole('button', { name: /ส่งกลับ$/ })).not.toBeInTheDocument()
  })

  // And no second stepper either: her report is filed, and the screen says what
  // it is waiting for rather than inviting a duplicate.
  it('gives her the read-only summary and a word about what happens next', async () => {
    await renderReport({ report: SUBMITTED_REPORT })
    expect(screen.queryByTestId('hk-report-form')).not.toBeInTheDocument()
    expect(screen.getByTestId('hk-report-summary')).toBeInTheDocument()
    expect(screen.getByText('ส่งแล้ว รอแผนกต้อนรับตรวจ')).toBeInTheDocument()
  })
})

// ---------------------------------------------------------------------------
// A finished report — read-only for everybody, both photo sets.
// ---------------------------------------------------------------------------

describe('a verified report', () => {
  it.each([
    ['the maid', {}],
    ['reception', { canReport: false }],
  ])('is read-only for %s, with both sides’ evidence', async (_who, me) => {
    await renderReport({ report: VERIFIED_REPORT, me })
    expect(screen.queryByTestId('hk-report-form')).not.toBeInTheDocument()
    expect(screen.queryByTestId('hk-report-verify')).not.toBeInTheDocument()
    expect(screen.getByTestId('hk-report-zone-evidence')).toBeInTheDocument()
    expect(screen.getByTestId('hk-report-reception-gallery')).toBeInTheDocument()
  })

  it('names who verified it', async () => {
    await renderReport({ report: VERIFIED_REPORT })
    expect(screen.getByText(/ตรวจโดย มานี/)).toBeInTheDocument()
    expect(screen.getByText('ตรวจแล้ว')).toBeInTheDocument()
  })
})

// ---------------------------------------------------------------------------
// LEGACY. A report filed before the ticks existed is permanent, and the bundle
// that reads it will outlive the bundle that wrote it.
// ---------------------------------------------------------------------------

describe('a v1 report, filed before the ticks', () => {
  it('renders exactly as it always did — the exception list and a flat gallery', async () => {
    await renderReport({ report: LEGACY_REPORT, me: { canReport: false } })
    const summary = screen.getByTestId('hk-report-summary')
    expect(within(summary).getByTestId('hk-report-exceptions')).toBeInTheDocument()
    expect(within(summary).getByText('รีโมทโทรทัศน์ · ชำรุด')).toBeInTheDocument()
    expect(within(summary).getByText('2')).toBeInTheDocument()
    expect(screen.queryByTestId('hk-report-ticks')).not.toBeInTheDocument()
    // No `photos` metadata to group by, so the flat gallery is what it gets.
    expect(screen.queryByTestId('hk-report-zone-evidence')).not.toBeInTheDocument()
    expect(screen.getByAltText('รูปจากแม่บ้าน 31')).toHaveAttribute(
      'src',
      '/hk/api/report-photos/31?branch=hfhotel'
    )
  })

  // Showing "ครบทุกรายการ" over a list of missing items is the one failure here
  // that could cost a guest a charge nobody can explain.
  it('lets the ROWS win when a v1 report arrives without its flag', async () => {
    await renderReport({
      report: { ...LEGACY_REPORT, allItemsOk: undefined },
      me: { canReport: false },
    })
    const summary = screen.getByTestId('hk-report-summary')
    expect(within(summary).getByText('รีโมทโทรทัศน์ · ชำรุด')).toBeInTheDocument()
    expect(within(summary).queryByText('ครบทุกรายการ')).not.toBeInTheDocument()
  })

  it('prefills a returned v1 report’s exceptions as ticks', async () => {
    await renderReport({ report: { ...LEGACY_REPORT, status: 'returned', returnReason: 'not_clean' } })
    fireEvent.click(screen.getByRole('button', { name: 'ถัดไป' }))
    fireEvent.click(screen.getByRole('button', { name: 'ถัดไป' }))
    fireEvent.click(screen.getByRole('button', { name: 'ถัดไป' }))
    // tv_remote is a ทั่วไป item, and it comes back ชำรุด.
    expect(screen.getByRole('button', { name: 'รีโมทโทรทัศน์ ชำรุด' })).toBeInTheDocument()
  })
})
