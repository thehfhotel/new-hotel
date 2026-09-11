/**
 * @jest-environment jsdom
 *
 * Task B7a — the room board's action sheet for a `booked` room.
 *
 * `booked` (จองแล้ว) was the one room state with a reservation attached and no
 * way to act on it: the sheet offered housekeeping actions only, so the desk
 * either went to iHOTEL or checked the guest in as a walk-in from the
 * `available` path — which creates an unlinked stay and silences every B7
 * app-deposit signpost. This pins the action's presence, and that the other
 * states did not move.
 */

import { render, screen, fireEvent } from '@testing-library/react'
import RoomActionSheet, { type RoomItem } from '@/components/v2/RoomActionSheet'

const base: RoomItem = {
  id: 7,
  roomNo: '301',
  roomTypeName: 'Standard',
  floor: 3,
  status: 'booked',
  isClean: true,
  isMaintenance: false,
}

function renderSheet(room: Partial<RoomItem> = {}, props: Record<string, unknown> = {}) {
  const onAction = jest.fn()
  render(
    <RoomActionSheet
      room={{ ...base, ...room }}
      onClose={jest.fn()}
      onAction={onAction}
      {...props}
    />,
  )
  return onAction
}

describe('RoomActionSheet — booked room', () => {
  test('offers a from-reservation check-in', () => {
    const onAction = renderSheet()
    const btn = screen.getByRole('button', { name: /เช็คอิน \(จากการจอง\)/ })
    fireEvent.click(btn)
    expect(onAction).toHaveBeenCalledWith('checkin')
  })

  test('does not offer checkout / extend / change — nobody is in the room yet', () => {
    renderSheet()
    expect(screen.queryByRole('button', { name: 'เช็คเอาท์' })).not.toBeInTheDocument()
    expect(screen.queryByRole('button', { name: 'ขยายเวลาเข้าพัก' })).not.toBeInTheDocument()
    expect(screen.queryByRole('button', { name: 'เปลี่ยนห้อง' })).not.toBeInTheDocument()
  })

  test('housekeeping stays available on a booked room', () => {
    renderSheet()
    expect(screen.getByRole('button', { name: 'แจ้งทำความสะอาด' })).toBeInTheDocument()
    expect(screen.getByRole('button', { name: 'ปิดห้องซ่อม' })).toBeInTheDocument()
  })

  test('a resolve in flight disables the action, so one tap cannot become two stays', () => {
    const onAction = renderSheet({}, { busy: true })
    const btn = screen.getByRole('button', { name: /เช็คอิน \(จากการจอง\)/ })
    expect(btn).toBeDisabled()
    fireEvent.click(btn)
    expect(onAction).not.toHaveBeenCalled()
  })

  test('surfaces the caller’s notice when no reservation could be resolved', () => {
    renderSheet({}, { notice: 'ไม่พบการจองของห้องนี้ในระบบใหม่สำหรับวันนี้' })
    expect(screen.getByTestId('room-action-notice')).toHaveTextContent('ไม่พบการจอง')
  })

  test('read-only branch gets no check-in action at all', () => {
    renderSheet({}, { readOnly: true })
    expect(screen.queryByRole('button', { name: /เช็คอิน/ })).not.toBeInTheDocument()
    expect(screen.getByText(/โหมดดูอย่างเดียว/)).toBeInTheDocument()
  })
})

describe('RoomActionSheet — the states B7a must not have moved', () => {
  test('available still offers the plain walk-in check-in', () => {
    const onAction = renderSheet({ status: 'available' })
    const btn = screen.getByRole('button', { name: 'เช็คอิน' })
    fireEvent.click(btn)
    expect(onAction).toHaveBeenCalledWith('checkin')
    // Not the from-reservation label.
    expect(screen.queryByRole('button', { name: /จากการจอง/ })).not.toBeInTheDocument()
  })

  test('occupied still offers the full stay lifecycle and no check-in', () => {
    renderSheet({ status: 'occupied' })
    expect(screen.getByRole('button', { name: 'เช็คเอาท์' })).toBeInTheDocument()
    expect(screen.getByRole('button', { name: 'ขยายเวลาเข้าพัก' })).toBeInTheDocument()
    expect(screen.getByRole('button', { name: 'เปลี่ยนห้อง' })).toBeInTheDocument()
    expect(screen.getByRole('button', { name: 'เพิ่มรายการในบิล' })).toBeInTheDocument()
    expect(screen.queryByRole('button', { name: /เช็คอิน/ })).not.toBeInTheDocument()
  })

  test('maintenance still offers only เปิดห้องขาย', () => {
    renderSheet({ status: 'maintenance', isMaintenance: true })
    expect(screen.getByRole('button', { name: 'เปิดห้องขาย' })).toBeInTheDocument()
    expect(screen.queryByRole('button', { name: /เช็คอิน/ })).not.toBeInTheDocument()
  })
})
