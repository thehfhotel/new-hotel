'use client'

import { useEffect, useState } from 'react'
import { createPortal } from 'react-dom'
import {
  appDepositEnglishLine,
  appDepositNoticeView,
  appDepositThaiLine,
} from '@/components/v2/AppDepositNotice'

/**
 * Printable A6 note: "this deposit was paid in the app" (ใบแจ้งมัดจำชำระผ่านแอป)
 * — Task B7.
 *
 * One small block reception can print and clip to the arrival paperwork, so the
 * colleague who later works the stay in iHOTEL — where this booking's deposit
 * reads 0 by design — does not ask the guest to pay it twice.
 * See docs/loyalty-channel.md §"Dual-write policy for holds" "deposit is not mirrored".
 *
 * Print isolation mirrors `RegistrationSlipTemplate` / `BookingConfirmationSlip`:
 * the note renders into a portal at `document.body` and is `display: none` on
 * screen, so a plain `window.print()` prints ONLY this note. The explicit
 * `visibility: visible` re-declaration is what lets it survive the /v2 layout,
 * whose own print block hides `body *` and un-hides only `.v2-print-active`.
 * The on-screen half of this signpost is `AppDepositNotice` — this component is
 * paper only.
 *
 * Renders nothing unless `appDepositNoticeView` says the signpost applies, so a
 * caller can mount it unconditionally next to the notice and the two can never
 * disagree.
 */

export interface AppDepositNoteData {
  /** `ht_bookings.book_channel` — the note only prints for `'loyalty'`. */
  bookChannel: string | null | undefined
  /** Booking-level deposit in BAHT (`ht_bookings.book_deposit_amount`). */
  depositAmount: number | null | undefined
  /** Booking number (เลขที่จอง). */
  bookingNo?: string | null
  guestName?: string | null
  /** Room number(s) if already assigned — free text, e.g. "401" or "401, 402". */
  roomNo?: string | null
  /** ISO `YYYY-MM-DD` (or ISO datetime) — stay dates, shown as stored. */
  checkIn?: string | null
  checkOut?: string | null
  hotelName?: string
}

/**
 * Show a stored Thai-local date as-is. MSSQL/PG datetimes in this tree are
 * naive local Thai time, so formatting with `timeZone: 'UTC'` prints the stored
 * value unshifted — never `Asia/Bangkok` (CLAUDE.md).
 */
function formatStoredDate(value?: string | null): string {
  if (!value || !value.trim()) return '-'
  const d = new Date(value.length <= 10 ? `${value}T00:00:00` : value)
  if (Number.isNaN(d.getTime())) return value
  return d.toLocaleDateString('th-TH', {
    day: '2-digit',
    month: 'short',
    year: 'numeric',
    timeZone: 'UTC',
  })
}

const Row = ({ label, value }: { label: string; value: string }) => (
  <div style={{ display: 'flex', justifyContent: 'space-between', gap: '8px' }}>
    <span style={{ color: '#444' }}>{label}</span>
    <span style={{ fontWeight: 600, textAlign: 'right' }}>{value}</span>
  </div>
)

export default function AppDepositNote({
  bookChannel,
  depositAmount,
  bookingNo,
  guestName,
  roomNo,
  checkIn,
  checkOut,
  hotelName = '',
}: AppDepositNoteData) {
  // Portals need a DOM; render nothing on the server pass.
  const [mounted, setMounted] = useState(false)
  useEffect(() => setMounted(true), [])

  const view = appDepositNoticeView(bookChannel, depositAmount)
  if (!mounted || !view) return null

  return createPortal(
    <div className="app-deposit-note-print-root">
      <style jsx global>{`
        .app-deposit-note-print-root {
          display: none;
        }
        @media print {
          @page {
            size: A6;
            margin: 8mm;
          }
          body {
            print-color-adjust: exact;
            -webkit-print-color-adjust: exact;
          }
          body > *:not(.app-deposit-note-print-root) {
            display: none !important;
          }
          .app-deposit-note-print-root {
            display: block !important;
          }
          .app-deposit-note-print-root,
          .app-deposit-note-print-root * {
            visibility: visible !important;
          }
        }
      `}</style>

      <div
        data-testid="app-deposit-note"
        style={{
          width: '105mm',
          maxWidth: '100%',
          margin: '0 auto',
          background: '#fff',
          color: '#000',
          padding: '4mm',
          boxSizing: 'border-box',
          border: '1.5px solid #000',
          fontFamily: 'inherit',
        }}
      >
        <div style={{ textAlign: 'center' }}>
          {hotelName ? <div style={{ fontSize: '12px', fontWeight: 700 }}>{hotelName}</div> : null}
          <div style={{ fontSize: '13px', fontWeight: 700, marginTop: '2px' }}>
            มัดจำชำระผ่านแอปแล้ว / App deposit paid
          </div>
        </div>

        <div style={{ borderTop: '1px dashed #000', margin: '5px 0' }} />

        <div style={{ fontSize: '10.5px', lineHeight: 1.7 }}>
          {bookingNo ? <Row label="เลขที่จอง / Booking" value={bookingNo} /> : null}
          {guestName ? <Row label="ลูกค้า / Guest" value={guestName} /> : null}
          {roomNo ? <Row label="ห้อง / Room" value={roomNo} /> : null}
          <Row
            label="เข้าพัก / Stay"
            value={`${formatStoredDate(checkIn)} — ${formatStoredDate(checkOut)}`}
          />
        </div>

        <div style={{ borderTop: '1px dashed #000', margin: '5px 0' }} />

        <p style={{ fontSize: '12px', fontWeight: 700, lineHeight: 1.5, margin: 0 }}>
          {appDepositThaiLine(view.amount)}
        </p>
        <p style={{ fontSize: '9.5px', lineHeight: 1.5, margin: '4px 0 0', color: '#333' }}>
          {appDepositEnglishLine(view.amount)}
        </p>

        <div style={{ borderTop: '1px dashed #000', margin: '5px 0' }} />

        <div style={{ textAlign: 'center', fontSize: '10px', fontWeight: 700 }}>
          ห้ามเรียกเก็บมัดจำนี้ซ้ำ / Do not collect this deposit again
        </div>
      </div>
    </div>,
    document.body,
  )
}
