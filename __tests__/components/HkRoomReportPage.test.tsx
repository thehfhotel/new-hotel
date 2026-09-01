/**
 * @jest-environment jsdom
 *
 * Report HK — ONE ROOM's daily report (`app/hk/rooms/[roomId]/report/page.tsx`).
 *
 * Three screens live in this route and the switch between them is the feature:
 *
 * 1. **The maid's form**, which must not be fileable without evidence. No
 *    photo, or a claim that something is wrong that names nothing, is a report
 *    reception cannot act on — refused here so she is never told "no" after
 *    walking away from the room.
 * 2. **The two-sided evidence.** Reception verifies with her OWN photos or
 *    returns with one of exactly three canned reasons; a maid NEVER verifies,
 *    including one who also holds the reception grant.
 * 3. **Append-only history.** A returned report is fixed by a NEW submission
 *    that carries `parentReportId` — the old row is never edited.
 *
 * The hk-lib helpers are mocked at the module boundary (the repo's established
 * pattern): what this suite owns is what the PAGE does — which body it hands
 * those helpers, and which controls each role is shown. `hk-lib.test.ts` owns
 * what the helpers then put on the wire.
 */

import { act, fireEvent, render, screen, waitFor, within } from '@testing-library/react'

const mockHkFetch = jest.fn()
const mockHkFetchMe = jest.fn()
const mockFetchHkRoomReport = jest.fn()
const mockUploadHkReportPhoto = jest.fn()
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
    submitHkReport: (...args: unknown[]) => mockSubmitHkReport(...args),
    verifyHkReport: (...args: unknown[]) => mockVerifyHkReport(...args),
    returnHkReport: (...args: unknown[]) => mockReturnHkReport(...args),
    downscalePhoto: (...args: unknown[]) => mockDownscalePhoto(...args),
  }
})

import HkRoomReportPage from '@/app/hk/rooms/[roomId]/report/page'
import { REPORT_SUBMITTED_NOTICE } from '@/app/hk/hk-lib'

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
}

/** Capture one photo through the strip labelled `label`, and wait for its
 *  thumbnail — which only appears once the upload has actually landed. */
async function attachPhoto(label: string, photoId: number) {
  mockUploadHkReportPhoto.mockResolvedValue(photoId)
  const input = screen.getByLabelText(label)
  await act(async () => {
    fireEvent.change(input, {
      target: { files: [new File(['x'], 'photo.jpg', { type: 'image/jpeg' })] },
    })
  })
  await screen.findByAltText(`รูปที่แนบ ${photoId}`)
}

const SUBMITTED_REPORT = {
  reportId: 55,
  roomId: 7,
  date: '2026-09-02',
  status: 'submitted',
  roomStatus: 'co',
  allItemsOk: false,
  items: [{ item: 'tv_remote', problem: 'damaged', qty: 2 }],
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
  receptionPhotoIds: [41],
}

beforeEach(() => {
  jest.clearAllMocks()
  localStorage.clear()
  sessionStorage.clear()
  mockDownscalePhoto.mockImplementation(async (file: Blob) => file)
  mockSubmitHkReport.mockResolvedValue({ ...SUBMITTED_REPORT, reportId: 56 })
  mockVerifyHkReport.mockResolvedValue(VERIFIED_REPORT)
  mockReturnHkReport.mockResolvedValue(RETURNED_REPORT)
})

// ---------------------------------------------------------------------------
// THE MAID'S FORM — prefill, then the two rules that keep an unusable report
// off reception's screen.
// ---------------------------------------------------------------------------

describe('the maid’s form', () => {
  it('renders the form when nothing has been filed for this room today', async () => {
    await renderReport()
    expect(screen.getByTestId('hk-report-form')).toBeInTheDocument()
  })

  // Prefilled, never locked: what she leaves selected is what is stored, so a
  // wrong guess costs one tap.
  it('prefills SO for a room the guest is still in', async () => {
    await renderReport({ room: { occupancy: 'occupied' } })
    expect(screen.getByRole('button', { name: 'SO พักต่อ' })).toHaveAttribute(
      'aria-pressed',
      'true'
    )
  })

  it('prefills CO for a vacated room that was due out today', async () => {
    await renderReport({ room: { occupancy: 'vacant', expectedDeparture: true } })
    expect(screen.getByRole('button', { name: 'CO เช็คเอาท์' })).toHaveAttribute(
      'aria-pressed',
      'true'
    )
  })

  it('lets the maid override the prefill', async () => {
    await renderReport({ room: { occupancy: 'occupied' } })
    fireEvent.click(screen.getByRole('button', { name: 'OO รอซ่อม' }))
    expect(screen.getByRole('button', { name: 'OO รอซ่อม' })).toHaveAttribute(
      'aria-pressed',
      'true'
    )
    expect(screen.getByRole('button', { name: 'SO พักต่อ' })).toHaveAttribute(
      'aria-pressed',
      'false'
    )
  })

  // THE rule: evidence is not optional.
  it('will not let her submit without a photo', async () => {
    await renderReport()
    expect(screen.getByRole('button', { name: /ส่งรายงาน/ })).toBeDisabled()
  })

  it('arms the submit once one photo has landed', async () => {
    await renderReport()
    await attachPhoto('ถ่ายรูปห้อง', 31)
    expect(screen.getByRole('button', { name: /ส่งรายงาน/ })).toBeEnabled()
  })

  it('drops a photo she removes, and disarms the submit with it', async () => {
    await renderReport()
    await attachPhoto('ถ่ายรูปห้อง', 31)
    fireEvent.click(screen.getByRole('button', { name: 'ลบรูป 31' }))
    expect(screen.queryByAltText('รูปที่แนบ 31')).not.toBeInTheDocument()
    expect(screen.getByRole('button', { name: /ส่งรายงาน/ })).toBeDisabled()
  })

  // The toggle and the list are ONE claim: "something is wrong" that names
  // nothing is a report reception cannot act on.
  it('will not let her claim an exception without naming one', async () => {
    await renderReport()
    await attachPhoto('ถ่ายรูปห้อง', 31)
    fireEvent.click(screen.getByRole('button', { name: 'มีรายการผิดปกติ' }))
    expect(screen.getByRole('button', { name: /ส่งรายงาน/ })).toBeDisabled()
  })

  it('arms the submit again once an item is named', async () => {
    await renderReport()
    await attachPhoto('ถ่ายรูปห้อง', 31)
    fireEvent.click(screen.getByRole('button', { name: 'มีรายการผิดปกติ' }))
    fireEvent.click(screen.getByRole('button', { name: 'กาน้ำร้อน หาย' }))
    expect(screen.getByRole('button', { name: /ส่งรายงาน/ })).toBeEnabled()
  })

  // The checklist only appears when she says something is wrong — the common
  // room stays one tap.
  it('shows no item list at all while ครบทุกรายการ is selected', async () => {
    await renderReport()
    expect(screen.queryByTestId('hk-report-items')).not.toBeInTheDocument()
    fireEvent.click(screen.getByRole('button', { name: 'มีรายการผิดปกติ' }))
    expect(screen.getByTestId('hk-report-items')).toBeInTheDocument()
  })

  it('steps a named item’s quantity', async () => {
    await renderReport()
    fireEvent.click(screen.getByRole('button', { name: 'มีรายการผิดปกติ' }))
    fireEvent.click(screen.getByRole('button', { name: 'หมอน หาย' }))
    fireEvent.click(screen.getByRole('button', { name: 'เพิ่ม หมอน หาย' }))
    fireEvent.click(screen.getByRole('button', { name: 'เพิ่ม หมอน หาย' }))
    expect(screen.getByText('3')).toBeInTheDocument()
  })
})

// ---------------------------------------------------------------------------
// Filing it — the two-step confirm, then the exact body.
// ---------------------------------------------------------------------------

describe('filing the report', () => {
  it('the first tap issues NO request — it only arms the confirm', async () => {
    await renderReport()
    await attachPhoto('ถ่ายรูปห้อง', 31)

    fireEvent.click(screen.getByRole('button', { name: /ส่งรายงาน/ }))

    await screen.findByText('ยืนยันส่งรายงาน ห้อง 104?')
    expect(mockSubmitHkReport).not.toHaveBeenCalled()
  })

  it('posts exactly what the form says, for this room', async () => {
    await renderReport({ room: { occupancy: 'vacant', expectedDeparture: true } })
    await attachPhoto('ถ่ายรูปห้อง', 31)
    fireEvent.click(screen.getByRole('button', { name: 'มีรายการผิดปกติ' }))
    fireEvent.click(screen.getByRole('button', { name: 'กาน้ำร้อน หาย' }))
    fireEvent.click(screen.getByRole('button', { name: 'เพิ่ม กาน้ำร้อน หาย' }))

    fireEvent.click(screen.getByRole('button', { name: /ส่งรายงาน/ }))
    fireEvent.click(await screen.findByRole('button', { name: 'ยืนยัน' }))

    await waitFor(() => {
      expect(mockSubmitHkReport).toHaveBeenCalledWith('hfhotel', 7, {
        roomStatus: 'co',
        allItemsOk: false,
        items: [{ item: 'kettle', problem: 'missing', qty: 2 }],
        photoIds: [31],
      })
    })
  })

  // ครบทุกรายการ ships an EMPTY list, never a list of "fine" rows: the wire
  // says what is wrong and nothing else.
  it('posts an empty item list for a room that is fine', async () => {
    await renderReport()
    await attachPhoto('ถ่ายรูปห้อง', 31)
    fireEvent.click(screen.getByRole('button', { name: /ส่งรายงาน/ }))
    fireEvent.click(await screen.findByRole('button', { name: 'ยืนยัน' }))

    await waitFor(() => {
      expect(mockSubmitHkReport).toHaveBeenCalledWith('hfhotel', 7, {
        roomStatus: 'so',
        allItemsOk: true,
        items: [],
        photoIds: [31],
      })
    })
  })

  // A first submission has no parent: nothing is being superseded.
  it('carries no parentReportId on a first submission', async () => {
    await renderReport()
    await attachPhoto('ถ่ายรูปห้อง', 31)
    fireEvent.click(screen.getByRole('button', { name: /ส่งรายงาน/ }))
    fireEvent.click(await screen.findByRole('button', { name: 'ยืนยัน' }))
    await waitFor(() => expect(mockSubmitHkReport).toHaveBeenCalled())
    expect(mockSubmitHkReport.mock.calls[0][2]).not.toHaveProperty('parentReportId')
  })

  it('hands the banner to the day overview and goes there', async () => {
    await renderReport()
    await attachPhoto('ถ่ายรูปห้อง', 31)
    fireEvent.click(screen.getByRole('button', { name: /ส่งรายงาน/ }))
    fireEvent.click(await screen.findByRole('button', { name: 'ยืนยัน' }))

    await waitFor(() => expect(mockPush).toHaveBeenCalledWith('/hk/report'))
    expect(sessionStorage.getItem('hk.reportNotice')).toBe(REPORT_SUBMITTED_NOTICE)
  })

  // A failed write must keep her on the form with everything she entered — a
  // maid who has to re-photograph a room stops filing reports.
  it('keeps the form and the photo when the write fails', async () => {
    mockSubmitHkReport.mockRejectedValue(new Error('บันทึกไม่สำเร็จ กรุณาลองใหม่'))
    await renderReport()
    await attachPhoto('ถ่ายรูปห้อง', 31)
    fireEvent.click(screen.getByRole('button', { name: /ส่งรายงาน/ }))
    fireEvent.click(await screen.findByRole('button', { name: 'ยืนยัน' }))

    expect(await screen.findByText('บันทึกไม่สำเร็จ กรุณาลองใหม่')).toBeInTheDocument()
    expect(mockPush).not.toHaveBeenCalled()
    expect(screen.getByTestId('hk-report-form')).toBeInTheDocument()
    expect(screen.getByAltText('รูปที่แนบ 31')).toBeInTheDocument()
  })
})

// ---------------------------------------------------------------------------
// RETURNED — the reason, the prefill, and the parent link.
// ---------------------------------------------------------------------------

describe('a returned report', () => {
  it('names the canned reason before the form', async () => {
    await renderReport({ report: RETURNED_REPORT })
    expect(
      within(screen.getByTestId('hk-report-returned-banner')).getByText(
        /ส่งกลับให้แก้ไข: รูปไม่ชัดเจน/
      )
    ).toBeInTheDocument()
  })

  it('gives her the form again, prefilled with what she sent', async () => {
    await renderReport({ report: RETURNED_REPORT })
    expect(screen.getByTestId('hk-report-form')).toBeInTheDocument()
    // roomStatus 'co', allItemsOk false, one damaged tv_remote at qty 2.
    expect(screen.getByRole('button', { name: 'CO เช็คเอาท์' })).toHaveAttribute(
      'aria-pressed',
      'true'
    )
    expect(screen.getByRole('button', { name: 'มีรายการผิดปกติ' })).toHaveAttribute(
      'aria-pressed',
      'true'
    )
    expect(screen.getByRole('button', { name: 'รีโมทโทรทัศน์ ชำรุด' })).toHaveAttribute(
      'aria-pressed',
      'true'
    )
    expect(screen.getByText('2')).toBeInTheDocument()
  })

  // Append-only: the fix is a NEW report that POINTS AT the one it supersedes.
  // Her old photos are NOT carried over — reception returned the report, and
  // re-sending the same evidence is not a fix.
  it('resubmits as a NEW report carrying parentReportId', async () => {
    await renderReport({ report: RETURNED_REPORT })
    await attachPhoto('ถ่ายรูปห้อง', 91)
    fireEvent.click(screen.getByRole('button', { name: /ส่งรายงาน/ }))
    fireEvent.click(await screen.findByRole('button', { name: 'ยืนยัน' }))

    await waitFor(() => {
      expect(mockSubmitHkReport).toHaveBeenCalledWith('hfhotel', 7, {
        roomStatus: 'co',
        allItemsOk: false,
        items: [{ item: 'tv_remote', problem: 'damaged', qty: 2 }],
        photoIds: [91],
        parentReportId: 55,
      })
    })
  })

  it('starts the resubmission with no photos of its own', async () => {
    await renderReport({ report: RETURNED_REPORT })
    expect(screen.queryByAltText('รูปที่แนบ 31')).not.toBeInTheDocument()
    expect(screen.getByRole('button', { name: /ส่งรายงาน/ })).toBeDisabled()
  })
})

// ---------------------------------------------------------------------------
// RECEPTION'S VERIFY. The desk side, and only on a submitted report.
// ---------------------------------------------------------------------------

describe('reception’s verify view', () => {
  const receptionMe = { canReport: false }

  it('never shows reception the maid’s form', async () => {
    await renderReport({ report: null, me: receptionMe })
    expect(screen.queryByTestId('hk-report-form')).not.toBeInTheDocument()
    expect(screen.getByTestId('hk-report-empty')).toBeInTheDocument()
  })

  it('shows her everything the maid reported', async () => {
    await renderReport({ report: SUBMITTED_REPORT, me: receptionMe })
    const summary = screen.getByTestId('hk-report-summary')
    expect(within(summary).getByText('CO เช็คเอาท์')).toBeInTheDocument()
    expect(within(summary).getByText('รีโมทโทรทัศน์ · ชำรุด')).toBeInTheDocument()
    expect(within(summary).getByText('2')).toBeInTheDocument()
    // Her photos, rendered from the branch-scoped photo endpoint.
    expect(screen.getByTestId('hk-report-maid-gallery')).toBeInTheDocument()
    expect(screen.getByAltText('รูปจากแม่บ้าน 31')).toHaveAttribute(
      'src',
      '/hk/api/report-photos/31?branch=hfhotel'
    )
  })

  // Reception's OWN photos are what make a verify a walk-up rather than a desk
  // stamp — the two-sided evidence IS the feature.
  it('will not let her verify without a photo of her own', async () => {
    await renderReport({ report: SUBMITTED_REPORT, me: receptionMe })
    expect(screen.getByRole('button', { name: /ยืนยันการตรวจ/ })).toBeDisabled()
  })

  it('verifies with her own photo ids, behind the confirm', async () => {
    await renderReport({ report: SUBMITTED_REPORT, me: receptionMe })
    await attachPhoto('ถ่ายรูปการตรวจ', 41)

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
    expect(within(verify).getByRole('button', { name: 'อุปกรณ์ไม่ตรงกับที่รายงาน' })).toBeInTheDocument()
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

  // And no second form either: her report is filed, and the screen says what
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
  ])('is read-only for %s, with both photo sets', async (_who, me) => {
    await renderReport({ report: VERIFIED_REPORT, me })
    expect(screen.queryByTestId('hk-report-form')).not.toBeInTheDocument()
    expect(screen.queryByTestId('hk-report-verify')).not.toBeInTheDocument()
    expect(screen.getByTestId('hk-report-maid-gallery')).toBeInTheDocument()
    expect(screen.getByTestId('hk-report-reception-gallery')).toBeInTheDocument()
  })

  it('names who verified it', async () => {
    await renderReport({ report: VERIFIED_REPORT })
    expect(screen.getByText(/ตรวจโดย มานี/)).toBeInTheDocument()
    expect(screen.getByText('ตรวจแล้ว')).toBeInTheDocument()
  })
})

// ---------------------------------------------------------------------------
// Deploy skew. The wire may lose a field under a cached LINE WebView; the one
// disagreement that matters is a report whose exceptions arrive without the
// flag that says there are any.
// ---------------------------------------------------------------------------

describe('when the flag and the rows disagree', () => {
  const noFlag = { ...SUBMITTED_REPORT, allItemsOk: undefined }

  // Showing "ครบทุกรายการ" over a list of missing items is the one failure
  // here that could cost a guest a charge nobody can explain.
  it('lets the ROWS win on the read-only summary', async () => {
    await renderReport({ report: noFlag, me: { canReport: false } })
    const summary = screen.getByTestId('hk-report-summary')
    expect(within(summary).getByText('รีโมทโทรทัศน์ · ชำรุด')).toBeInTheDocument()
    expect(within(summary).queryByText('ครบทุกรายการ')).not.toBeInTheDocument()
  })

  // Same rule on the way back into the form: her list must not be hidden
  // behind a toggle she never chose.
  it('lets the ROWS win when prefilling a returned report', async () => {
    await renderReport({ report: { ...RETURNED_REPORT, allItemsOk: undefined } })
    expect(screen.getByRole('button', { name: 'มีรายการผิดปกติ' })).toHaveAttribute(
      'aria-pressed',
      'true'
    )
    expect(screen.getByRole('button', { name: 'รีโมทโทรทัศน์ ชำรุด' })).toHaveAttribute(
      'aria-pressed',
      'true'
    )
  })
})
