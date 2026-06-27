/**
 * @jest-environment jsdom
 */

/**
 * Task #58 — in-app reception verification form (app/v2/verification/page.tsx).
 * Verifies the form renders all five checklist sections, blocks submit until the
 * §5 readiness verdict is chosen, and POSTs the answers (as JSON) to
 * /api/verification on a valid submit.
 */

import { render, screen, fireEvent, waitFor } from '@testing-library/react'
import V2Verification from '@/app/v2/verification/page'

// --- mock the contexts + branch-aware fetch the page depends on ---
const fetchMock = jest.fn()

jest.mock('@/lib/use-branch-fetch', () => ({
  useBranchFetch: () => fetchMock,
}))

jest.mock('@/contexts/BranchContext', () => ({
  useBranch: () => ({ branch: 'hfhotel', canWrite: true }),
}))

jest.mock('@/contexts/AuthContext', () => ({
  useAuth: () => ({ user: { username: 'reception_a' } }),
}))

describe('V2Verification — reception verification form', () => {
  beforeEach(() => {
    fetchMock.mockReset()
    // jsdom has no real scroll; the success path calls window.scrollTo.
    window.scrollTo = jest.fn()
  })

  test('renders all five checklist sections and the submit button', () => {
    render(<V2Verification />)
    expect(screen.getByRole('heading', { name: /เทียบหน้าจอกับ iHOTEL/ })).toBeInTheDocument()
    expect(screen.getByRole('heading', { name: /คำถามเรื่องนโยบาย/ })).toBeInTheDocument()
    expect(screen.getByRole('heading', { name: /ตรวจยอดบิล 5 รายการล่าสุด/ })).toBeInTheDocument()
    expect(screen.getByRole('heading', { name: /ทดสอบสด/ })).toBeInTheDocument()
    expect(screen.getByRole('heading', { name: /ความพร้อมโดยรวม/ })).toBeInTheDocument()
    expect(screen.getByRole('button', { name: /บันทึกผลการตรวจสอบ/ })).toBeInTheDocument()
  })

  test('blocks submit until the §5 readiness verdict is chosen', async () => {
    render(<V2Verification />)
    fireEvent.click(screen.getByRole('button', { name: /บันทึกผลการตรวจสอบ/ }))
    expect(await screen.findByText(/กรุณาเลือกความพร้อมโดยรวม/)).toBeInTheDocument()
    expect(fetchMock).not.toHaveBeenCalled()
  })

  test('POSTs the answers as JSON to /api/verification on a valid submit', async () => {
    fetchMock.mockResolvedValue({
      ok: true,
      json: () => Promise.resolve({ success: true, id: 7, submittedAt: '2026-06-28T00:00:00Z' }),
    })

    render(<V2Verification />)

    // Choose the §5 verdict (the only required answer).
    fireEvent.click(screen.getByRole('radio', { name: /ดี ข้อมูลตรง พร้อมทดลองใช้จริง/ }))
    fireEvent.click(screen.getByRole('button', { name: /บันทึกผลการตรวจสอบ/ }))

    await waitFor(() => expect(fetchMock).toHaveBeenCalledTimes(1))

    const [url, init] = fetchMock.mock.calls[0]
    expect(url).toBe('/api/verification')
    expect(init.method).toBe('POST')
    const body = JSON.parse(init.body)
    expect(body.overall).toBe('a')
    expect(body.answers.q5).toBe('a')
    // inspector falls back to the session username when the field is blank.
    expect(body.inspector).toBe('reception_a')

    // Success confirmation replaces the form.
    expect(await screen.findByText(/บันทึกผลการตรวจสอบเรียบร้อยแล้ว/)).toBeInTheDocument()
  })
})
