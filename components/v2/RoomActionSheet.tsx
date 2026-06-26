'use client'

import {
  X,
  LogIn,
  LogOut,
  CalendarPlus,
  ArrowRightLeft,
  Coffee,
  Sparkles,
  Wrench,
  CheckCircle2,
  User,
  Loader2,
} from 'lucide-react'
import { formatStoredDayMonth } from '@/lib/format'
import { roomStatusView } from '@/lib/v2/status'
import { StatusPill } from '@/components/v2/primitives'

export interface RoomItem {
  id: number
  roomNo: string
  roomTypeName: string | null
  floor: number | null
  status: string
  isClean: boolean
  isMaintenance: boolean
  notes?: string | null
  currentGuest?: { name: string; checkIn: string; checkOut: string } | null
}

export type RoomAction =
  | 'checkin'
  | 'checkout'
  | 'extend'
  | 'change'
  | 'pos'
  | 'clean'
  | 'dirty'
  | 'maintenance'
  | 'ready'

function ActionButton({
  onClick,
  icon,
  label,
  variant = 'ghost',
  disabled,
}: {
  onClick: () => void
  icon: React.ReactNode
  label: string
  variant?: 'primary' | 'ghost' | 'soft'
  disabled?: boolean
}) {
  return (
    <button
      onClick={onClick}
      disabled={disabled}
      className={`v2-btn v2-btn-${variant} w-full justify-start`}
    >
      {icon}
      {label}
    </button>
  )
}

export default function RoomActionSheet({
  room,
  onClose,
  onAction,
  busy,
}: {
  room: RoomItem
  onClose: () => void
  onAction: (a: RoomAction) => void
  busy?: boolean
}) {
  const view = roomStatusView(room.status, { isClean: room.isClean, isMaintenance: room.isMaintenance })
  const isDirty = room.status === 'available' && room.isClean === false

  return (
    <div className="v2-sheet-backdrop" onClick={onClose} role="dialog" aria-modal="true">
      {/* Mobile: bottom sheet. Desktop: right panel. */}
      <div
        className="mt-auto lg:mt-0 lg:ml-auto w-full lg:w-[420px] lg:h-full v2-sheet-up lg:[animation:v2-panel-in_.24s_cubic-bezier(.2,.8,.2,1)]"
        style={{ background: 'var(--v2-surface)' }}
        onClick={(e) => e.stopPropagation()}
      >
        <div
          className="flex items-start gap-3 px-5 py-4"
          style={{ borderBottom: '1px solid var(--v2-line)' }}
        >
          <div className="flex-1">
            <div className="v2-eyebrow mb-1">ห้องพัก</div>
            <div className="flex items-center gap-3">
              <span className="v2-num text-[30px] font-bold leading-none">{room.roomNo}</span>
              <StatusPill view={view} />
            </div>
            <div className="text-[13px] mt-1.5" style={{ color: 'var(--v2-ink-3)' }}>
              {room.roomTypeName || 'ไม่ระบุประเภท'}
              {room.floor != null && ` · ชั้น ${room.floor}`}
            </div>
          </div>
          <button onClick={onClose} className="p-2 rounded-full" style={{ color: 'var(--v2-ink-3)' }} aria-label="ปิด">
            <X size={20} />
          </button>
        </div>

        <div className="px-5 py-4 space-y-4 overflow-y-auto" style={{ maxHeight: 'calc(100vh - 110px)' }}>
          {room.currentGuest && (
            <div className="v2-inset px-4 py-3">
              <div className="flex items-center gap-2 text-[14px] font-semibold">
                <User size={15} style={{ color: 'var(--v2-wine-600)' }} />
                {room.currentGuest.name}
              </div>
              <div className="text-[12.5px] mt-1" style={{ color: 'var(--v2-ink-3)' }}>
                {formatStoredDayMonth(room.currentGuest.checkIn)} – {formatStoredDayMonth(room.currentGuest.checkOut)}
              </div>
            </div>
          )}

          {busy && (
            <div className="flex items-center gap-2 text-[13px]" style={{ color: 'var(--v2-ink-3)' }}>
              <Loader2 className="animate-spin" size={15} /> กำลังอัปเดต…
            </div>
          )}

          <div className="space-y-2">
            {/* Available & clean → check in */}
            {room.status === 'available' && room.isClean !== false && (
              <ActionButton variant="primary" icon={<LogIn size={17} />} label="เช็คอิน" onClick={() => onAction('checkin')} />
            )}

            {/* Occupied → stay lifecycle */}
            {room.status === 'occupied' && (
              <>
                <ActionButton variant="primary" icon={<LogOut size={17} />} label="เช็คเอาท์" onClick={() => onAction('checkout')} />
                <ActionButton icon={<CalendarPlus size={17} />} label="ขยายเวลาเข้าพัก" onClick={() => onAction('extend')} />
                <ActionButton icon={<ArrowRightLeft size={17} />} label="เปลี่ยนห้อง" onClick={() => onAction('change')} />
                <ActionButton icon={<Coffee size={17} />} label="เพิ่มรายการในบิล" onClick={() => onAction('pos')} />
              </>
            )}

            {/* Checkout pending */}
            {room.status === 'checkout_pending' && (
              <ActionButton variant="primary" icon={<LogOut size={17} />} label="เช็คเอาท์" onClick={() => onAction('checkout')} />
            )}

            {/* Dirty → mark clean */}
            {isDirty && (
              <ActionButton variant="primary" icon={<CheckCircle2 size={17} />} label="ทำความสะอาดเสร็จ" onClick={() => onAction('clean')} />
            )}

            {/* Housekeeping / maintenance — available to most states */}
            {!isDirty && room.status === 'available' && (
              <ActionButton icon={<Sparkles size={17} />} label="แจ้งทำความสะอาด" onClick={() => onAction('dirty')} />
            )}
            {room.status === 'maintenance' ? (
              <ActionButton icon={<CheckCircle2 size={17} />} label="พร้อมใช้งาน (เลิกแจ้งซ่อม)" onClick={() => onAction('ready')} />
            ) : (
              <ActionButton icon={<Wrench size={17} />} label="แจ้งซ่อม" onClick={() => onAction('maintenance')} />
            )}
          </div>
        </div>
      </div>
    </div>
  )
}
