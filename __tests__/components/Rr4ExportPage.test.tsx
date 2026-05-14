/**
 * @jest-environment jsdom
 *
 * Track G8 — frontend test for the RR.4 export page.
 *
 * Covers the load-bearing behaviour:
 * 1. Clicking the Export button issues a `GET /api/new/reports/rr4`
 *    with the form's `from`, `to`, `site`, and `format` query params.
 * 2. The mocked branchFetch is invoked exactly once per click.
 * 3. The default format is `xlsx` (regulator delivery format).
 */

import type React from 'react'
import { fireEvent, render, screen, waitFor } from '@testing-library/react'

// Mock react-datepicker (jsdom doesn't render it cleanly; we only need
// the controlled-value pathway, which the parent state already drives).
jest.mock('react-datepicker', () => {
  return function MockDatePicker(props: {
    startDate?: Date | null
    endDate?: Date | null
    onChange: (update: [Date | null, Date | null]) => void
  }) {
    return (
      <div data-testid="datepicker-mock">
        <button
          type="button"
          data-testid="set-range"
          onClick={() =>
            props.onChange([
              new Date('2026-05-01T00:00:00'),
              new Date('2026-05-13T00:00:00'),
            ])
          }
        >
          set range
        </button>
      </div>
    )
  }
})

// Capture the fetch invoked under the hood. We mock `useBranchFetch`
// (the page's only outbound dependency) so the test exercises the
// real URL/format-construction logic.
const mockFetch = jest.fn()
jest.mock('@/lib/use-branch-fetch', () => ({
  useBranchFetch: () => mockFetch,
}))

// Mock URL.createObjectURL — jsdom doesn't ship it by default and
// the page calls it to trigger the browser download.
beforeAll(() => {
  ;(global.URL as unknown as { createObjectURL: () => string }).createObjectURL =
    jest.fn(() => 'blob:mock-url')
  ;(global.URL as unknown as { revokeObjectURL: () => void }).revokeObjectURL =
    jest.fn()
})

beforeEach(() => {
  mockFetch.mockReset()
})

// Use require so the mock above is in effect when the module first
// evaluates. The page imports its `useBranchFetch` hook at module
// load, so a top-level ESM import would race ahead of the jest.mock.
const Rr4ExportPage: React.ComponentType = require('@/app/reports/rr4/page').default

function mockSuccessfulXlsxResponse() {
  mockFetch.mockResolvedValueOnce({
    ok: true,
    status: 200,
    blob: () => Promise.resolve(new Blob([new Uint8Array([0x50, 0x4b, 0x03, 0x04])])),
    text: () => Promise.resolve(''),
    headers: {
      get: (name: string) => {
        const map: Record<string, string> = {
          'content-disposition':
            'attachment; filename="RR4_hfhotel_20260501_20260513.xlsx"',
          'x-rr4-row-count': '2',
          'x-rr4-file-hash':
            'abc1234567890def1234567890abcdef1234567890abcdef1234567890abcdef',
        }
        return map[name.toLowerCase()] ?? null
      },
    },
  })
}

describe('Rr4ExportPage', () => {
  test('renders Thai header and form controls', () => {
    render(<Rr4ExportPage />)
    expect(screen.getByText('รายงาน รร.4 / ตม.30')).toBeInTheDocument()
    expect(screen.getByText(/RR\.4/)).toBeInTheDocument()
    expect(screen.getByLabelText('สาขา (site)')).toBeInTheDocument()
    expect(screen.getByLabelText('รูปแบบไฟล์')).toBeInTheDocument()
  })

  test('default format selector is xlsx (regulator delivery)', () => {
    render(<Rr4ExportPage />)
    const formatSelect = screen.getByLabelText('รูปแบบไฟล์') as HTMLSelectElement
    expect(formatSelect.value).toBe('xlsx')
  })

  test('clicking Export fires GET /api/new/reports/rr4 with the form params', async () => {
    mockSuccessfulXlsxResponse()
    render(<Rr4ExportPage />)

    // Configure date range via the mocked datepicker.
    fireEvent.click(screen.getByTestId('set-range'))

    // Submit.
    const exportButton = screen.getByRole('button', { name: /ส่งออกรายงาน/ })
    fireEvent.click(exportButton)

    await waitFor(() => {
      expect(mockFetch).toHaveBeenCalledTimes(1)
    })

    const calledUrl = mockFetch.mock.calls[0][0] as string
    expect(calledUrl.startsWith('/api/new/reports/rr4?')).toBe(true)
    expect(calledUrl).toContain('from=2026-05-01')
    expect(calledUrl).toContain('to=2026-05-13')
    expect(calledUrl).toContain('site=hfhotel')
    expect(calledUrl).toContain('format=xlsx')
  })

  test('format switch to csv flows into the request URL', async () => {
    mockSuccessfulXlsxResponse()
    render(<Rr4ExportPage />)

    fireEvent.click(screen.getByTestId('set-range'))
    fireEvent.change(screen.getByLabelText('รูปแบบไฟล์'), {
      target: { value: 'csv' },
    })

    fireEvent.click(screen.getByRole('button', { name: /ส่งออกรายงาน/ }))

    await waitFor(() => {
      expect(mockFetch).toHaveBeenCalledTimes(1)
    })
    const calledUrl = mockFetch.mock.calls[0][0] as string
    expect(calledUrl).toContain('format=csv')
  })

  test('export button is enabled by default (component seeds a last-30-days range)', () => {
    render(<Rr4ExportPage />)
    const exportButton = screen.getByRole('button', { name: /ส่งออกรายงาน/ })
    // The component pre-seeds startDate/endDate with the trailing 30
    // days so the receptionist can click Export immediately. If that
    // initial state were null, the button would be disabled — this
    // test pins the UX contract.
    expect(exportButton).not.toBeDisabled()
  })

  test('site dropdown switches between hfhotel and hfville', async () => {
    mockSuccessfulXlsxResponse()
    render(<Rr4ExportPage />)

    fireEvent.click(screen.getByTestId('set-range'))
    fireEvent.change(screen.getByLabelText('สาขา (site)'), {
      target: { value: 'hfville' },
    })

    fireEvent.click(screen.getByRole('button', { name: /ส่งออกรายงาน/ }))

    await waitFor(() => {
      expect(mockFetch).toHaveBeenCalledTimes(1)
    })
    const calledUrl = mockFetch.mock.calls[0][0] as string
    expect(calledUrl).toContain('site=hfville')
  })
})
