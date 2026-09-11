'use client'

import { useEffect, useRef, useState } from 'react'
import { Loader2, Send } from 'lucide-react'
import { DESK_SIGNALS } from './signal-lib'

/**
 * The per-room send action on the desk board: one button that opens the FIVE
 * desk→maid signal types and nothing else.
 *
 * Canned-only (ADR 0008) is enforced structurally here — the menu is built by
 * mapping `DESK_SIGNALS` from `app/hk/signal-vocab.ts`, so there is no place a
 * free-text field could be added without editing the vocabulary itself, and a
 * type added there appears here with no change to this file.
 *
 * One room per signal: the caller passes exactly one `roomId`, and the menu
 * closes on the tap so a slip of the finger cannot fan a second signal out to
 * the branch.
 */

export default function SendSignalMenu({
  roomId,
  roomNo,
  busy,
  disabled,
  onSend,
}: {
  roomId: number
  roomNo: string
  busy?: boolean
  disabled?: boolean
  onSend: (roomId: number, type: string) => void
}) {
  const [open, setOpen] = useState(false)
  const wrapRef = useRef<HTMLDivElement | null>(null)

  // Close on an outside click or Escape — the menu sits inside a scrolling
  // room list, so leaving one open while reception scans elsewhere would put a
  // send button under an unrelated room.
  useEffect(() => {
    if (!open) return
    const onDown = (event: MouseEvent) => {
      if (wrapRef.current && !wrapRef.current.contains(event.target as Node)) setOpen(false)
    }
    const onKey = (event: KeyboardEvent) => {
      if (event.key === 'Escape') setOpen(false)
    }
    document.addEventListener('mousedown', onDown)
    document.addEventListener('keydown', onKey)
    return () => {
      document.removeEventListener('mousedown', onDown)
      document.removeEventListener('keydown', onKey)
    }
  }, [open])

  return (
    <div className="relative" ref={wrapRef}>
      <button
        type="button"
        className="v2-btn v2-btn-ghost v2-btn-sm"
        aria-haspopup="menu"
        aria-expanded={open}
        aria-label={`แจ้งแม่บ้าน ห้อง ${roomNo}`}
        disabled={disabled || busy}
        onClick={() => setOpen((v) => !v)}
      >
        {busy ? <Loader2 className="animate-spin" size={14} /> : <Send size={14} />}
        แจ้งแม่บ้าน
      </button>

      {open && (
        <div
          role="menu"
          aria-label={`เลือกสัญญาณสำหรับห้อง ${roomNo}`}
          className="absolute right-0 z-30 mt-1 w-[190px] overflow-hidden rounded-[10px] p-1"
          style={{
            background: 'var(--v2-surface)',
            border: '1px solid var(--v2-line-2)',
            boxShadow: 'var(--v2-shadow-md)',
          }}
        >
          {DESK_SIGNALS.map(({ type, label }) => (
            <button
              key={type}
              type="button"
              role="menuitem"
              className="w-full rounded-[7px] px-3 py-2 text-left text-[13.5px]"
              style={{ color: 'var(--v2-ink)' }}
              onClick={() => {
                setOpen(false)
                onSend(roomId, type)
              }}
            >
              {label}
            </button>
          ))}
        </div>
      )}
    </div>
  )
}
