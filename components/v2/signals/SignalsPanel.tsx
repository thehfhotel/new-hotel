'use client'

import { Loader2, User } from 'lucide-react'
import { V2Empty, V2Section } from '@/components/v2/primitives'
import {
  actorName,
  canDeskAck,
  canDeskCancel,
  canDeskDone,
  formatSignalTime,
  signalLabel,
  signalStatusLabel,
  signalTone,
  type DeskSignalAction,
  type RoomSignal,
} from './signal-lib'

/**
 * The desk's open-signal panel (ADR 0008): every signal at the active branch
 * that is still `open` or `acked`, and the taps reception is allowed to make
 * on each one.
 *
 * Presentational on purpose — it holds no fetch and no state, so the contract
 * it encodes (which button appears on which signal) is testable on its own.
 * The two lists are split by DIRECTION rather than merged, because they mean
 * opposite things to a receptionist: the first is her inbox, the second is
 * what she has asked of the maids and is still waiting on.
 *
 * A signal stays here until it is done, whatever the day — the list is not a
 * feed and never scrolls away.
 */

export const INBOX_TITLE = 'จากแม่บ้าน'
export const OUTBOX_TITLE = 'ส่งถึงแม่บ้าน'
export const EMPTY_TITLE = 'ไม่มีสัญญาณค้างอยู่'

function ActionButton({
  label,
  onClick,
  busy,
  disabled,
  variant = 'ghost',
}: {
  label: string
  onClick: () => void
  busy?: boolean
  disabled?: boolean
  variant?: 'primary' | 'ghost' | 'soft'
}) {
  return (
    <button
      type="button"
      onClick={onClick}
      disabled={busy || disabled}
      className={`v2-btn v2-btn-${variant} v2-btn-sm`}
    >
      {busy && <Loader2 className="animate-spin" size={13} />}
      {label}
    </button>
  )
}

function SignalRow({
  signal,
  busy,
  disabled,
  onAct,
}: {
  signal: RoomSignal
  busy: boolean
  disabled?: boolean
  onAct: (signalId: number, action: DeskSignalAction) => void
}) {
  const created = formatSignalTime(signal.createdAt)
  const by = actorName(signal.createdBy)
  const ackedBy = actorName(signal.ackedBy)

  return (
    <div className="flex flex-wrap items-center gap-x-3 gap-y-2 px-4 lg:px-5 py-3">
      <span className={`v2-pill s-${signalTone(signal)}`}>{signalLabel(signal.type)}</span>

      <span className="v2-num text-[15px] font-bold" style={{ color: 'var(--v2-ink)' }}>
        {signal.roomNo}
      </span>

      <span
        className="inline-flex items-center gap-1.5 text-[12px]"
        style={{ color: 'var(--v2-ink-3)' }}
      >
        <User size={13} />
        {by}
        {created && ` · ${created}`}
      </span>

      {signal.status === 'acked' ? (
        <span className="text-[12px]" style={{ color: 'var(--v2-ink-2)' }}>
          {signalStatusLabel(signal.status)}
          {ackedBy && ` · ${ackedBy}`}
          {formatSignalTime(signal.ackedAt) && ` · ${formatSignalTime(signal.ackedAt)}`}
        </span>
      ) : (
        <span className="text-[12px]" style={{ color: 'var(--v2-ink-3)' }}>
          {signalStatusLabel(signal.status)}
        </span>
      )}

      <div className="flex-1" />

      <div className="flex items-center gap-2">
        {canDeskAck(signal) && (
          <ActionButton
            label="รับทราบ"
            variant="soft"
            busy={busy}
            disabled={disabled}
            onClick={() => onAct(signal.signalId, 'ack')}
          />
        )}
        {canDeskDone(signal) && (
          <ActionButton
            label="เสร็จสิ้น"
            variant="primary"
            busy={busy}
            disabled={disabled}
            onClick={() => onAct(signal.signalId, 'done')}
          />
        )}
        {canDeskCancel(signal) && (
          <ActionButton
            label="ยกเลิก"
            busy={busy}
            disabled={disabled}
            onClick={() => onAct(signal.signalId, 'cancel')}
          />
        )}
      </div>
    </div>
  )
}

function SignalList({
  title,
  signals,
  busySignalId,
  disabled,
  onAct,
}: {
  title: string
  signals: RoomSignal[]
  busySignalId: number | null
  disabled?: boolean
  onAct: (signalId: number, action: DeskSignalAction) => void
}) {
  if (signals.length === 0) return null
  return (
    <div>
      <div
        className="v2-eyebrow px-4 lg:px-5 pt-3 pb-1"
        style={{ color: 'var(--v2-ink-3)' }}
      >
        {title}
      </div>
      <div className="divide-y" style={{ borderColor: 'var(--v2-line)' }}>
        {signals.map((signal) => (
          <SignalRow
            key={signal.signalId}
            signal={signal}
            busy={busySignalId === signal.signalId}
            disabled={disabled}
            onAct={onAct}
          />
        ))}
      </div>
    </div>
  )
}

export default function SignalsPanel({
  signals,
  busySignalId,
  loading,
  disabled,
  onAct,
}: {
  /** Already in `sortSignals` order — the hook returns them that way. */
  signals: RoomSignal[]
  busySignalId: number | null
  /** True until the first read lands. "Nothing outstanding" is a real answer a
   *  receptionist acts on, so it must never be shown before it is known. */
  loading?: boolean
  /** True while the surface cannot write (e.g. the aggregate branch view). */
  disabled?: boolean
  onAct: (signalId: number, action: DeskSignalAction) => void
}) {
  const inbox = signals.filter((s) => s.direction === 'maid_to_desk')
  const outbox = signals.filter((s) => s.direction === 'desk_to_maid')

  return (
    <V2Section
      title="สัญญาณห้องพัก"
      count={loading && signals.length === 0 ? undefined : signals.length}
      tone="s-arr"
    >
      {signals.length === 0 ? (
        loading ? (
          <V2Empty title="กำลังโหลดสัญญาณ…" />
        ) : (
          <V2Empty title={EMPTY_TITLE} hint="สัญญาณจะแสดงอยู่ที่นี่จนกว่าจะเสร็จสิ้น" />
        )
      ) : (
        <div className="divide-y" style={{ borderColor: 'var(--v2-line)' }}>
          <SignalList
            title={INBOX_TITLE}
            signals={inbox}
            busySignalId={busySignalId}
            disabled={disabled}
            onAct={onAct}
          />
          <SignalList
            title={OUTBOX_TITLE}
            signals={outbox}
            busySignalId={busySignalId}
            disabled={disabled}
            onAct={onAct}
          />
        </div>
      )}
    </V2Section>
  )
}
