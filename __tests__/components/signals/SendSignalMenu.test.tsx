/**
 * @jest-environment jsdom
 *
 * The per-room send action on the desk board (ADR 0008).
 *
 * Two properties matter here and both are structural, not cosmetic:
 * canned-only — the menu offers exactly the five desk→maid types from
 * `app/hk/signal-vocab.ts`, never a maid type and never a text field — and one
 * room per signal, so a tap sends about the room whose row it sits on and then
 * closes.
 */

import { fireEvent, render, screen } from '@testing-library/react'
import SendSignalMenu from '@/components/v2/signals/SendSignalMenu'
import { DESK_SIGNALS, MAID_SIGNALS } from '@/app/hk/signal-vocab'

function open(onSend = jest.fn()) {
  render(<SendSignalMenu roomId={12} roomNo="304" onSend={onSend} />)
  fireEvent.click(screen.getByRole('button', { name: 'แจ้งแม่บ้าน ห้อง 304' }))
  return onSend
}

describe('SendSignalMenu — canned-only, five types', () => {
  it('shows nothing until it is opened', () => {
    render(<SendSignalMenu roomId={12} roomNo="304" onSend={jest.fn()} />)
    expect(screen.queryByRole('menu')).not.toBeInTheDocument()
  })

  it('offers exactly the five desk→maid types, in vocabulary order', () => {
    open()
    const items = screen.getAllByRole('menuitem').map((item) => item.textContent)
    expect(items).toEqual(DESK_SIGNALS.map((s) => s.label))
    expect(items).toHaveLength(5)
  })

  it('offers no maid→desk type — the desk cannot speak in the maid’s direction', () => {
    open()
    for (const { label } of MAID_SIGNALS) {
      expect(screen.queryByRole('menuitem', { name: label })).not.toBeInTheDocument()
    }
  })

  it('offers no free-text input anywhere — canned-only is the decision', () => {
    open()
    expect(screen.queryByRole('textbox')).not.toBeInTheDocument()
    expect(document.querySelectorAll('input, textarea')).toHaveLength(0)
  })
})

describe('SendSignalMenu — one room per signal', () => {
  it('sends the tapped type about this row’s room', () => {
    const onSend = open()
    fireEvent.click(screen.getByRole('menuitem', { name: 'ทำห้องนี้ก่อน' }))
    expect(onSend).toHaveBeenCalledWith(12, 'priority_clean')
    expect(onSend).toHaveBeenCalledTimes(1)
  })

  it('sends ขอเช็คห้อง by its code, not its label', () => {
    const onSend = open()
    fireEvent.click(screen.getByRole('menuitem', { name: 'ขอเช็คห้อง' }))
    expect(onSend).toHaveBeenCalledWith(12, 'room_check')
  })

  it('closes on the tap, so a second signal cannot go out by accident', () => {
    open()
    fireEvent.click(screen.getByRole('menuitem', { name: 'งดทำห้องนี้' }))
    expect(screen.queryByRole('menu')).not.toBeInTheDocument()
  })

  it('closes on Escape without sending anything', () => {
    const onSend = open()
    fireEvent.keyDown(document, { key: 'Escape' })
    expect(screen.queryByRole('menu')).not.toBeInTheDocument()
    expect(onSend).not.toHaveBeenCalled()
  })

  it('closes on an outside click without sending anything', () => {
    const onSend = open()
    fireEvent.mouseDown(document.body)
    expect(screen.queryByRole('menu')).not.toBeInTheDocument()
    expect(onSend).not.toHaveBeenCalled()
  })

  it('cannot be opened while a send for this room is in flight', () => {
    render(<SendSignalMenu roomId={12} roomNo="304" onSend={jest.fn()} busy />)
    expect(screen.getByRole('button', { name: 'แจ้งแม่บ้าน ห้อง 304' })).toBeDisabled()
  })
})
