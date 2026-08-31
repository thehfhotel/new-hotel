/**
 * @jest-environment jsdom
 *
 * The desk's open-signal panel (ADR 0008).
 *
 * What this pins is the ROLE contract as it reaches a receptionist's thumb:
 * she acks and completes what the maids sent her, she cancels what she sent
 * them while it is still open, and she is never shown a เสร็จสิ้น on a
 * ขอเช็คห้อง — that one is completed by the maid's answer and a bare done is a
 * 400 by contract.
 */

import { fireEvent, render, screen, within } from '@testing-library/react'
import SignalsPanel, {
  EMPTY_TITLE,
  INBOX_TITLE,
  OUTBOX_TITLE,
} from '@/components/v2/signals/SignalsPanel'
import type { RoomSignal } from '@/components/v2/signals/signal-lib'

function signal(partial: Partial<RoomSignal> & { signalId: number }): RoomSignal {
  return {
    roomId: 1,
    roomNo: '101',
    direction: 'maid_to_desk',
    type: 'guest_in_room',
    status: 'open',
    createdBy: { badge: 'Q1001', name: 'สมหญิง' },
    createdAt: '2026-09-01T03:00:00Z',
    ...partial,
  }
}

function renderPanel(signals: RoomSignal[], onAct = jest.fn()) {
  render(<SignalsPanel signals={signals} busySignalId={null} onAct={onAct} />)
  return onAct
}

/** The row containing a given signal label — actions are asserted within it so
 *  a second signal's buttons can never satisfy the assertion. */
function rowFor(label: string): HTMLElement {
  return screen.getByText(label).parentElement as HTMLElement
}

describe('SignalsPanel — the two directions read differently', () => {
  it('splits the maid inbox from what the desk has asked for', () => {
    renderPanel([
      signal({ signalId: 1, type: 'item_missing' }),
      signal({ signalId: 2, direction: 'desk_to_maid', type: 'priority_clean' }),
    ])
    expect(screen.getByText(INBOX_TITLE)).toBeInTheDocument()
    expect(screen.getByText(OUTBOX_TITLE)).toBeInTheDocument()
  })

  it('omits a heading with nothing under it', () => {
    renderPanel([signal({ signalId: 1 })])
    expect(screen.getByText(INBOX_TITLE)).toBeInTheDocument()
    expect(screen.queryByText(OUTBOX_TITLE)).not.toBeInTheDocument()
  })

  it('says so plainly when nothing is outstanding', () => {
    renderPanel([])
    expect(screen.getByText(EMPTY_TITLE)).toBeInTheDocument()
    expect(screen.queryAllByRole('button')).toHaveLength(0)
  })

  it('does NOT claim "nothing outstanding" before the first read lands', () => {
    render(<SignalsPanel signals={[]} busySignalId={null} loading onAct={jest.fn()} />)
    expect(screen.queryByText(EMPTY_TITLE)).not.toBeInTheDocument()
  })

  it('shows the type, the room, and who raised it', () => {
    renderPanel([signal({ signalId: 1, type: 'item_damaged', roomNo: '204' })])
    expect(screen.getByText('มีของเสียหาย')).toBeInTheDocument()
    expect(screen.getByText('204')).toBeInTheDocument()
    expect(screen.getByText(/สมหญิง/)).toBeInTheDocument()
  })

  it('falls back to the badge when the IdP forwarded no name', () => {
    renderPanel([signal({ signalId: 1, createdBy: { badge: 'Q1009', name: null } })])
    expect(screen.getByText(/Q1009/)).toBeInTheDocument()
  })

  it('names who acked, so reception knows the job has an owner', () => {
    renderPanel([
      signal({
        signalId: 1,
        status: 'acked',
        ackedBy: { badge: 'R2001', name: 'มานี' },
        ackedAt: '2026-09-01T03:05:00Z',
      }),
    ])
    expect(screen.getByText(/รับแล้ว · มานี/)).toBeInTheDocument()
  })
})

describe('SignalsPanel — which tap is offered', () => {
  it('offers รับทราบ and เสร็จสิ้น on an open maid signal', () => {
    renderPanel([signal({ signalId: 1, type: 'found_belongings' })])
    const row = rowFor('พบของลืมในห้อง')
    expect(within(row).getByText('รับทราบ')).toBeInTheDocument()
    expect(within(row).getByText('เสร็จสิ้น')).toBeInTheDocument()
    expect(within(row).queryByText('ยกเลิก')).not.toBeInTheDocument()
  })

  it('drops รับทราบ once the signal is acked, keeping เสร็จสิ้น', () => {
    renderPanel([signal({ signalId: 1, type: 'found_belongings', status: 'acked' })])
    const row = rowFor('พบของลืมในห้อง')
    expect(within(row).queryByText('รับทราบ')).not.toBeInTheDocument()
    expect(within(row).getByText('เสร็จสิ้น')).toBeInTheDocument()
  })

  it('offers only ยกเลิก on the desk’s own open signal', () => {
    renderPanel([signal({ signalId: 1, direction: 'desk_to_maid', type: 'priority_clean' })])
    const row = rowFor('ทำห้องนี้ก่อน')
    expect(within(row).getByText('ยกเลิก')).toBeInTheDocument()
    expect(within(row).queryByText('รับทราบ')).not.toBeInTheDocument()
    expect(within(row).queryByText('เสร็จสิ้น')).not.toBeInTheDocument()
  })

  it('offers NOTHING once a maid has acked the desk’s signal — cancel is open-only', () => {
    renderPanel([
      signal({ signalId: 1, direction: 'desk_to_maid', type: 'priority_clean', status: 'acked' }),
    ])
    expect(screen.queryAllByRole('button')).toHaveLength(0)
  })

  it('never offers เสร็จสิ้น on ขอเช็คห้อง — the maid’s answer completes it', () => {
    renderPanel([
      signal({ signalId: 1, direction: 'desk_to_maid', type: 'room_check', status: 'acked' }),
    ])
    expect(screen.queryByText('เสร็จสิ้น')).not.toBeInTheDocument()
  })
})

describe('SignalsPanel — the taps reach the right endpoint action', () => {
  it('acks by signal id', () => {
    const onAct = renderPanel([signal({ signalId: 55 })])
    fireEvent.click(screen.getByText('รับทราบ'))
    expect(onAct).toHaveBeenCalledWith(55, 'ack')
  })

  it('completes by signal id', () => {
    const onAct = renderPanel([signal({ signalId: 55 })])
    fireEvent.click(screen.getByText('เสร็จสิ้น'))
    expect(onAct).toHaveBeenCalledWith(55, 'done')
  })

  it('cancels by signal id', () => {
    const onAct = renderPanel([
      signal({ signalId: 77, direction: 'desk_to_maid', type: 'skip_room' }),
    ])
    fireEvent.click(screen.getByText('ยกเลิก'))
    expect(onAct).toHaveBeenCalledWith(77, 'cancel')
  })

  it('disables only the row whose tap is in flight', () => {
    render(
      <SignalsPanel
        signals={[signal({ signalId: 1 }), signal({ signalId: 2, roomNo: '102' })]}
        busySignalId={1}
        onAct={jest.fn()}
      />,
    )
    const busy = within(screen.getByText('101').parentElement as HTMLElement)
    const idle = within(screen.getByText('102').parentElement as HTMLElement)
    expect(busy.getByText('รับทราบ').closest('button')).toBeDisabled()
    expect(idle.getByText('รับทราบ').closest('button')).not.toBeDisabled()
  })
})
