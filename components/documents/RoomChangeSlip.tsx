'use client'

import { forwardRef } from 'react'

/**
 * Printable room-change slip (ใบแจ้งเปลี่ยนห้องพัก) — Task #54 item 5.
 *
 * Handed to the guest after a mid-stay room move. Sized for a thermal receipt
 * roll (58mm or 80mm). Mirrors the print-isolation pattern of
 * `CouponTemplate.tsx`: the root carries `room-change-print-root`, and
 * `printRoomChange()` (`lib/print.ts`) toggles `room-change-print-active` on it
 * for the duration of the print so a single slip prints cleanly even when
 * opened over a busy folio / room-grid page.
 *
 * Data comes from `GET /api/checkins/:id/room-change-receipt` (the canonical
 * `ht_room_changes` audit row joined to the folio + both rooms). `toPrice` is
 * the destination room's recomputed nightly rate (item 2); `roomBeforePrice`
 * is the rate the guest was on before the move.
 */

export interface RoomChangeSlipProps {
  cinNo: string
  customerName?: string | null
  fromRoomNo: string
  toRoomNo: string
  /** Rate the guest was on before the move (baht/night). */
  roomBeforePrice: number
  /** Destination room's recomputed rate (baht/night, string from the API). */
  toPrice?: string | null
  reason?: string | null
  changedAt?: string | null
  changedBy?: string | null
  hotelName?: string
  hotelPhone?: string
  width?: '58mm' | '80mm'
}

function formatBaht(amount: number): string {
  return new Intl.NumberFormat('th-TH', {
    style: 'currency',
    currency: 'THB',
    minimumFractionDigits: 2,
  }).format(amount)
}

/** Render the destination rate from the API string; falls back to a dash. */
function formatToPrice(toPrice?: string | null): string {
  if (toPrice == null || toPrice.trim() === '') return '-'
  const n = Number(toPrice)
  return Number.isFinite(n) ? formatBaht(n) : toPrice
}

/** Show the stored Thai-local timestamp as-is (no TZ shift — CLAUDE.md). */
function formatChangedAt(s?: string | null): string {
  if (!s) return '-'
  try {
    const d = new Date(s)
    if (Number.isNaN(d.getTime())) return s
    return d.toLocaleString('th-TH', {
      day: '2-digit',
      month: 'short',
      year: 'numeric',
      hour: '2-digit',
      minute: '2-digit',
      timeZone: 'UTC',
    })
  } catch {
    return s
  }
}

const Row = ({ label, value }: { label: string; value: string }) => (
  <div style={{ display: 'flex', justifyContent: 'space-between', gap: '8px' }}>
    <span>{label}</span>
    <span style={{ fontWeight: 600, textAlign: 'right' }}>{value}</span>
  </div>
)

const RoomChangeSlip = forwardRef<HTMLDivElement, RoomChangeSlipProps>(
  function RoomChangeSlip(
    {
      cinNo,
      customerName,
      fromRoomNo,
      toRoomNo,
      roomBeforePrice,
      toPrice,
      reason,
      changedAt,
      changedBy,
      hotelName = '',
      hotelPhone,
      width = '80mm',
    },
    ref,
  ) {
    return (
      <div
        ref={ref}
        className="room-change-print-root"
        style={{
          width,
          maxWidth: '100%',
          margin: '0 auto',
          background: '#fff',
          color: '#000',
          padding: '4mm',
          boxSizing: 'border-box',
          fontFamily: 'ui-monospace, "SFMono-Regular", Menlo, monospace',
        }}
      >
        <style jsx global>{`
          @media print {
            @page {
              margin: 6mm;
            }
            body * {
              visibility: hidden !important;
            }
            .room-change-print-active,
            .room-change-print-active * {
              visibility: visible !important;
            }
            .room-change-print-active {
              position: fixed !important;
              left: 0 !important;
              top: 0 !important;
              margin: 0 !important;
            }
          }
        `}</style>

        <div style={{ textAlign: 'center' }}>
          {hotelName ? (
            <div style={{ fontSize: '14px', fontWeight: 700 }}>{hotelName}</div>
          ) : null}
          <div style={{ fontSize: '12px', fontWeight: 600, marginTop: '2px' }}>
            ใบแจ้งเปลี่ยนห้องพัก / Room Change
          </div>
        </div>

        <div style={{ borderTop: '1px dashed #000', margin: '6px 0' }} />

        <div style={{ fontSize: '11px', lineHeight: 1.6 }}>
          <Row label="การเข้าพัก / Stay" value={cinNo} />
          {customerName ? (
            <Row label="ลูกค้า / Guest" value={customerName} />
          ) : null}
          <Row label="ห้องเดิม / From" value={fromRoomNo} />
          <Row label="ห้องใหม่ / To" value={toRoomNo} />
        </div>

        <div style={{ borderTop: '1px dashed #000', margin: '6px 0' }} />

        <div style={{ fontSize: '11px', lineHeight: 1.6 }}>
          <Row label="ราคาเดิม / Old rate" value={formatBaht(roomBeforePrice)} />
          <Row label="ราคาใหม่ / New rate" value={formatToPrice(toPrice)} />
        </div>

        <div style={{ borderTop: '1px dashed #000', margin: '6px 0' }} />

        <div style={{ fontSize: '11px', lineHeight: 1.6 }}>
          {reason && reason.trim() ? (
            <Row label="เหตุผล / Reason" value={reason} />
          ) : null}
          <Row label="วันที่ / Date" value={formatChangedAt(changedAt)} />
          {changedBy && changedBy.trim() ? (
            <Row label="โดย / By" value={changedBy} />
          ) : null}
        </div>

        <div style={{ borderTop: '1px dashed #000', margin: '6px 0' }} />

        <div style={{ textAlign: 'center', fontSize: '10px' }}>
          <div>ราคาใหม่มีผลกับคืนที่เหลือของการเข้าพัก</div>
          <div>New rate applies to the remaining nights of the stay</div>
          {hotelPhone ? (
            <div style={{ marginTop: '2px' }}>โทร. {hotelPhone}</div>
          ) : null}
        </div>
      </div>
    )
  },
)

export default RoomChangeSlip
