/**
 * @jest-environment jsdom
 *
 * Report HK — the DAY OVERVIEW (`app/hk/report/page.tsx`).
 *
 * This screen is the paper day-sheet's heir and the landing page of its own
 * LINE tile, so it has exactly two jobs and both of them are visual:
 *
 * 1. **Every room's state is legible at a glance** — ยังไม่ส่ง / ส่งแล้ว รอตรวจ /
 *    ตรวจแล้ว / ส่งกลับแก้ไข, and a returned room must NAME the canned reason on
 *    the row itself. A maid who has to open a room to find out why it came back
 *    is a maid who walks the corridor twice.
 * 2. **Each role's own work is at the top.** The rooms are the same for both;
 *    the ORDER is the whole difference between a maid's queue and reception's.
 *
 * `hkFetchMe` / `fetchHkReports` are mocked at the module boundary — the
 * repo's established pattern. `hk-lib.test.ts` owns the URL construction, the
 * chip vocabulary and the sort itself.
 */

import { render, screen, within } from '@testing-library/react'

const mockHkFetchMe = jest.fn()
const mockFetchHkReports = jest.fn()

jest.mock('@/app/hk/hk-lib', () => {
  const actual = jest.requireActual('@/app/hk/hk-lib')
  return {
    ...actual,
    hkFetchMe: (...args: unknown[]) => mockHkFetchMe(...args),
    fetchHkReports: (...args: unknown[]) => mockFetchHkReports(...args),
  }
})

import HkReportOverviewPage from '@/app/hk/report/page'
import { stashHkReportNotice } from '@/app/hk/hk-lib'

function jsonResponse(body: unknown, status = 200) {
  return { ok: status >= 200 && status < 300, status, json: async () => body }
}

/** `/me` with a single branch, so the page auto-selects it and never blocks on
 *  the picker. `overrides` is how the reception cases flip `canReport`; the
 *  default payload OMITS the field, which is the deploy-skew shape and must
 *  behave exactly like a maid's. */
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

function row(
  roomId: number,
  roomNo: string,
  report: Record<string, unknown> | null,
  floor = 1
) {
  return { roomId, roomNo, floor, building: null, report }
}

const UNSENT = row(1, '101', null)
const SUBMITTED = row(2, '102', { reportId: 12, roomId: 2, status: 'submitted' })
const VERIFIED = row(3, '103', { reportId: 13, roomId: 3, status: 'verified' })
const RETURNED = row(4, '104', {
  reportId: 14,
  roomId: 4,
  status: 'returned',
  returnReason: 'photos_unclear',
})

/** Render with a scripted day sheet and wait for the first room. */
async function renderOverview(
  rooms: Array<Record<string, unknown>>,
  meOverrides: Record<string, unknown> = {}
) {
  mockHkFetchMe.mockResolvedValue(meResponse(meOverrides))
  mockFetchHkReports.mockResolvedValue({ date: '2026-09-02', rooms })
  render(<HkReportOverviewPage />)
  await screen.findByText(`ห้อง ${(rooms[0] as { roomNo: string }).roomNo}`)
}

beforeEach(() => {
  jest.clearAllMocks()
  localStorage.clear()
  sessionStorage.clear()
})

// ---------------------------------------------------------------------------
// State chips — one per room, four states, and the returned one carries WHY.
// ---------------------------------------------------------------------------

describe('state chips', () => {
  it('marks a room with no report ยังไม่ส่ง', async () => {
    await renderOverview([UNSENT])
    const card = screen.getByTestId('hk-report-row-1')
    expect(within(card).getByText('ยังไม่ส่ง')).toBeInTheDocument()
  })

  it('marks a filed report ส่งแล้ว รอตรวจ', async () => {
    await renderOverview([SUBMITTED])
    expect(within(screen.getByTestId('hk-report-row-2')).getByText('ส่งแล้ว รอตรวจ')).toBeInTheDocument()
  })

  it('marks a countersigned report ตรวจแล้ว', async () => {
    await renderOverview([VERIFIED])
    expect(within(screen.getByTestId('hk-report-row-3')).getByText('ตรวจแล้ว')).toBeInTheDocument()
  })

  // THE one that saves a walk: the reason is on the row, not behind a tap.
  it('names the canned reason on a returned room', async () => {
    await renderOverview([RETURNED])
    expect(
      within(screen.getByTestId('hk-report-row-4')).getByText('ส่งกลับแก้ไข: รูปไม่ชัดเจน')
    ).toBeInTheDocument()
  })

  it('gives each room its own chip rather than one state for the sheet', async () => {
    await renderOverview([UNSENT, SUBMITTED, VERIFIED, RETURNED])
    expect(within(screen.getByTestId('hk-report-row-1')).getByText('ยังไม่ส่ง')).toBeInTheDocument()
    expect(within(screen.getByTestId('hk-report-row-2')).getByText('ส่งแล้ว รอตรวจ')).toBeInTheDocument()
    expect(within(screen.getByTestId('hk-report-row-3')).getByText('ตรวจแล้ว')).toBeInTheDocument()
    expect(
      within(screen.getByTestId('hk-report-row-4')).getByText('ส่งกลับแก้ไข: รูปไม่ชัดเจน')
    ).toBeInTheDocument()
  })
})

// ---------------------------------------------------------------------------
// Each role's queue. Same rooms, different top of the list.
// ---------------------------------------------------------------------------

describe('queue order', () => {
  function orderOnScreen() {
    return screen
      .getAllByRole('link')
      .map((a) => a.getAttribute('href'))
      .filter((href): href is string => Boolean(href?.includes('/report')) && href !== '/hk/report')
  }

  it('leads a maid with the room that came back, then what she has not filed', async () => {
    await renderOverview([VERIFIED, SUBMITTED, UNSENT, RETURNED])
    expect(orderOnScreen()).toEqual([
      '/hk/rooms/4/report',
      '/hk/rooms/1/report',
      '/hk/rooms/2/report',
      '/hk/rooms/3/report',
    ])
  })

  // canReport: false — the reception viewer. The only state she can act on is
  // submitted, so it leads.
  it('leads reception with what is waiting to be checked', async () => {
    await renderOverview([VERIFIED, UNSENT, SUBMITTED, RETURNED], { canReport: false })
    expect(orderOnScreen()[0]).toBe('/hk/rooms/2/report')
  })

  // The grouping is the room list's, so the sheet reads as a walking order.
  it('groups the rooms by floor', async () => {
    await renderOverview([row(1, '101', null, 1), row(9, '301', null, 3)])
    expect(screen.getByText('ชั้น 1')).toBeInTheDocument()
    expect(screen.getByText('ชั้น 3')).toBeInTheDocument()
  })

  it('sorts within a floor, not across the sheet', async () => {
    await renderOverview([
      row(1, '101', null, 1),
      row(9, '301', { reportId: 31, roomId: 9, status: 'returned' }, 3),
    ])
    // The returned room leads its OWN floor group; it does not jump above ชั้น 1.
    const floor1 = screen.getByText('ชั้น 1')
    const returnedRow = screen.getByTestId('hk-report-row-9')
    expect(
      floor1.compareDocumentPosition(returnedRow) & Node.DOCUMENT_POSITION_FOLLOWING
    ).toBeTruthy()
  })
})

// ---------------------------------------------------------------------------
// Everything else the screen owes its two audiences.
// ---------------------------------------------------------------------------

describe('the sheet around the rows', () => {
  it('links every room to that room’s report screen', async () => {
    await renderOverview([UNSENT])
    expect(screen.getByTestId('hk-report-row-1').getAttribute('href')).toBe('/hk/rooms/1/report')
  })

  it('renders the day the SERVER answered for, never one it assumed', async () => {
    await renderOverview([UNSENT])
    // 2026-09-02, rendered through the Thai date helper.
    expect(screen.getByText(/2569|2026/)).toBeInTheDocument()
  })

  // A maid plans by what is still unsent; reception by what is waiting.
  it('leads the summary bar with each role’s own number', async () => {
    await renderOverview([UNSENT, SUBMITTED, VERIFIED, RETURNED])
    const summary = screen.getByTestId('hk-report-summary')
    expect(within(summary).getByText('ยังไม่ส่ง')).toBeInTheDocument()
    expect(within(summary).queryByText('รอตรวจ')).not.toBeInTheDocument()
    expect(within(summary).getByText('ทั้งหมด')).toBeInTheDocument()
  })

  it('leads reception’s summary bar with รอตรวจ instead', async () => {
    await renderOverview([UNSENT, SUBMITTED], { canReport: false })
    const summary = screen.getByTestId('hk-report-summary')
    expect(within(summary).getByText('รอตรวจ')).toBeInTheDocument()
    expect(within(summary).queryByText('ยังไม่ส่ง')).not.toBeInTheDocument()
  })

  // The banner a landed submit/verify/return stashed before navigating here.
  it('shows the hand-off banner once, and clears it', async () => {
    stashHkReportNotice('ส่งรายงานแล้ว')
    await renderOverview([UNSENT])
    expect(within(screen.getByTestId('hk-report-notice')).getByText('ส่งรายงานแล้ว')).toBeInTheDocument()
    expect(sessionStorage.getItem('hk.reportNotice')).toBeNull()
  })

  it('shows no banner when nothing was stashed', async () => {
    await renderOverview([UNSENT])
    expect(screen.queryByTestId('hk-report-notice')).not.toBeInTheDocument()
  })

  // A read that fails must say so rather than leaving a blank sheet that reads
  // as "no rooms".
  it('surfaces a failed read', async () => {
    mockHkFetchMe.mockResolvedValue(meResponse())
    mockFetchHkReports.mockRejectedValue(new Error('ไม่สามารถดึงรายงานได้ กรุณาลองใหม่'))
    render(<HkReportOverviewPage />)
    expect(await screen.findByText('ไม่สามารถดึงรายงานได้ กรุณาลองใหม่')).toBeInTheDocument()
  })

  // Same fail-closed rule as every other /hk screen: identity first, and no
  // fallback branch.
  it('never asks for a day sheet when /me fails', async () => {
    mockHkFetchMe.mockRejectedValue(new Error('ไม่สามารถดึงข้อมูลผู้ใช้ได้ กรุณาลองใหม่'))
    render(<HkReportOverviewPage />)
    expect(await screen.findByText('ไม่สามารถดึงข้อมูลผู้ใช้ได้ กรุณาลองใหม่')).toBeInTheDocument()
    expect(mockFetchHkReports).not.toHaveBeenCalled()
  })

  it('asks for the day sheet with the resolved branch and no date — v1 has no picker', async () => {
    await renderOverview([UNSENT])
    expect(mockFetchHkReports).toHaveBeenCalledWith('hfhotel')
  })
})
