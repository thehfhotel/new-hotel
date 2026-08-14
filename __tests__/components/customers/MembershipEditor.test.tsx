/**
 * @jest-environment jsdom
 */

import { render, screen, fireEvent, waitFor } from '@testing-library/react'
import MembershipEditor from '@/components/customers/MembershipEditor'

// Mock lucide-react icons (same idiom as CustomerForm.test.tsx).
jest.mock('lucide-react', () => ({
  BadgeCheck: () => <span data-testid="icon-badge">Badge</span>,
  Loader2: () => <span data-testid="icon-loader">Loading</span>,
  Save: () => <span data-testid="icon-save">Save</span>,
  X: () => <span data-testid="icon-x">X</span>,
}))

describe('MembershipEditor', () => {
  const onSave = jest.fn()

  beforeEach(() => {
    jest.clearAllMocks()
    onSave.mockResolvedValue(undefined)
  })

  const input = () =>
    screen.getByPlaceholderText('พิมพ์หรือสแกนจาก QR สมาชิก')

  it('saves a typed membership id (trimmed)', async () => {
    render(
      <MembershipEditor customerId={42} membershipId={null} onSave={onSave} />
    )

    fireEvent.change(input(), { target: { value: '  M-000123  ' } })
    fireEvent.click(screen.getByRole('button', { name: /บันทึก/ }))

    await waitFor(() => {
      expect(onSave).toHaveBeenCalledWith(42, 'M-000123')
    })
    expect(await screen.findByText('บันทึกแล้ว')).toBeTruthy()
  })

  it('save button is disabled while empty and unlinked', () => {
    render(
      <MembershipEditor customerId={42} membershipId={null} onSave={onSave} />
    )
    const save = screen.getByRole('button', { name: /บันทึก/ })
    expect((save as HTMLButtonElement).disabled).toBe(true)
  })

  it('shows the existing link and clears it with null', async () => {
    render(
      <MembershipEditor
        customerId={7}
        membershipId="M-000999"
        onSave={onSave}
      />
    )

    expect((input() as HTMLInputElement).value).toBe('M-000999')

    fireEvent.click(
      screen.getByRole('button', { name: 'ยกเลิกการเชื่อมสมาชิก' })
    )
    await waitFor(() => {
      expect(onSave).toHaveBeenCalledWith(7, null)
    })
    expect((input() as HTMLInputElement).value).toBe('')
  })

  it('saving an emptied field clears the existing link', async () => {
    render(
      <MembershipEditor
        customerId={7}
        membershipId="M-000999"
        onSave={onSave}
      />
    )

    fireEvent.change(input(), { target: { value: '' } })
    fireEvent.click(screen.getByRole('button', { name: /บันทึก/ }))

    await waitFor(() => {
      expect(onSave).toHaveBeenCalledWith(7, null)
    })
  })

  it('surfaces a save failure without crashing', async () => {
    onSave.mockRejectedValueOnce(new Error('ไม่สามารถบันทึกรหัสสมาชิกได้'))
    render(
      <MembershipEditor customerId={42} membershipId={null} onSave={onSave} />
    )

    fireEvent.change(input(), { target: { value: 'M-1' } })
    fireEvent.click(screen.getByRole('button', { name: /บันทึก/ }))

    expect(
      await screen.findByText('ไม่สามารถบันทึกรหัสสมาชิกได้')
    ).toBeTruthy()
    // Value stays editable for a retry.
    expect((input() as HTMLInputElement).value).toBe('M-1')
  })
})
