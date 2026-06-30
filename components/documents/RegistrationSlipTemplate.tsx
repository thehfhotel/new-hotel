'use client'

import { useEffect, useState, type ReactNode } from 'react'
import { createPortal } from 'react-dom'
import { HotelInfo } from '@/types/invoice'

/**
 * Data for a check-in registration card (ใบลงทะเบียนผู้เข้าพัก / GUEST
 * REGISTRATION CARD). Mirrors iHOTEL's `ReportReg_1.rpt` — a bilingual
 * (Thai/English) fill-in form. Fields we hold are pre-printed; the rest are
 * left as blank ruled lines for the guest to complete by hand. All amounts baht.
 */
export interface RegistrationSlipData {
  /** Check-in / registration number (NO. on the card). */
  registrationNo: string
  checkInId?: number
  /** Customer account no (ACC NO.) — legacy Cust_no. */
  accNo?: string
  guestName: string
  guestIdCard?: string
  guestContact?: string
  guestAddress?: string
  vehiclePlate?: string
  /** Defaults to ไทย (matches iHOTEL, which pre-fills Thai). */
  nationality?: string
  roomNumber: string
  roomType?: string
  /** ISO datetime string. */
  checkInDate: string
  /** ISO datetime string (actual checkout if known, else expected). */
  checkOutDate: string
  nights: number
  ratePerNight?: number
  totalAmount?: number
  /** Advance paid against the booking (จ่ายล่วงหน้า) — booking deposit, not the
   *  total paid at check-in. Undefined/0 for walk-ins. */
  bookingAdvance?: number
  /** Originating booking number (จากการจองเลขที่ …). */
  bookingNo?: string
  /** Room deposit collected (เงินมัดจำ), baht. 0 / undefined ⇒ none. */
  deposit?: number
  adults?: number
  children?: number
  /** ISO datetime the slip was created. Defaults to now. */
  createdAt?: string
}

interface RegistrationSlipTemplateProps {
  data: RegistrationSlipData
  hotelInfo: HotelInfo
}

function formatCurrency(amount: number): string {
  return new Intl.NumberFormat('th-TH', {
    minimumFractionDigits: 2,
    maximumFractionDigits: 2,
  }).format(amount)
}

/** "29/06/2026" — Gregorian DD/MM/YYYY (no time), stored value as-is. */
function formatDate(dateStr?: string): string {
  if (!dateStr) return ''
  const d = new Date(dateStr)
  if (Number.isNaN(d.getTime())) return ''
  const p = (n: number) => String(n).padStart(2, '0')
  return `${p(d.getDate())}/${p(d.getMonth() + 1)}/${d.getFullYear()}`
}

/** A labelled fill-in field: value sits on a ruled line; Thai over English. */
function Field({
  thai,
  eng,
  value,
  className = '',
}: {
  thai: string
  eng: string
  value?: ReactNode
  className?: string
}) {
  return (
    <div className={`flex flex-col ${className}`}>
      <div className="flex items-end gap-1">
        <span className="whitespace-nowrap">{thai}</span>
        <span className="flex-1 border-b border-gray-500 px-1 min-h-[1.1em] leading-tight">
          {value || ' '}
        </span>
      </div>
      <span className="text-[9px] text-gray-500 leading-none mt-0.5">{eng}</span>
    </div>
  )
}

/** Empty checkbox square. */
function Box({ label, eng }: { label: string; eng?: string }) {
  return (
    <span className="inline-flex items-center gap-1 whitespace-nowrap">
      <span className="inline-block w-3 h-3 border border-gray-700" />
      {label}
      {eng && <span className="text-[9px] text-gray-500">{eng}</span>}
    </span>
  )
}

/**
 * Print-only registration card. Renders into a portal at `document.body` so a
 * `window.print()` prints ONLY this sheet. Self-isolating against BOTH the
 * classic app (display) and the /v2 layout (which does `body * {visibility:
 * hidden}` and only un-hides `.v2-print-active`).
 */
export default function RegistrationSlipTemplate({
  data,
  hotelInfo,
}: RegistrationSlipTemplateProps) {
  const [mounted, setMounted] = useState(false)
  useEffect(() => setMounted(true), [])
  if (!mounted) return null

  const nights = data.nights || 1
  const total =
    data.totalAmount != null && data.totalAmount > 0
      ? data.totalAmount
      : (data.ratePerNight ?? 0) * nights
  // cin_rate_per_night is unreliable (often 0); derive from total when needed.
  const rate =
    data.ratePerNight != null && data.ratePerNight > 0
      ? data.ratePerNight
      : nights > 0
        ? total / nights
        : total
  const other = Math.max(0, total - rate * nights)

  return createPortal(
    <div className="registration-slip-print-root">
      <style jsx global>{`
        .registration-slip-print-root {
          display: none;
        }
        @media print {
          @page {
            size: A4;
            margin: 12mm;
          }
          body {
            print-color-adjust: exact;
            -webkit-print-color-adjust: exact;
          }
          body > *:not(.registration-slip-print-root) {
            display: none !important;
          }
          .registration-slip-print-root {
            display: block !important;
          }
          /* The v2 layout's print CSS hides everything via a body-wide
             visibility:hidden and only un-hides .v2-print-active; make this
             portal visibility-robust too or it prints blank under /v2. No-op
             in the classic app. */
          .registration-slip-print-root,
          .registration-slip-print-root * {
            visibility: visible !important;
          }
        }
      `}</style>

      <div className="registration-slip bg-white mx-auto font-sans text-gray-900 text-[12px] leading-snug max-w-[210mm] p-6">
        {/* Header */}
        <div className="flex justify-between items-start">
          <div className="font-bold text-[14px] max-w-[60%]">{hotelInfo.name}</div>
          <div className="text-right">
            NO. <span className="font-semibold">{data.registrationNo}</span>
          </div>
        </div>
        <div className="text-center -mt-4 mb-1">
          <div className="font-bold text-[15px]">ใบลงทะเบียนผู้เข้าพัก</div>
          <div className="text-[10px] tracking-wide">GUEST REGISTRATION CARD</div>
        </div>
        <div className="flex justify-between items-end mb-3">
          <div className="whitespace-nowrap">
            ACC NO. : <span className="font-medium">{data.accNo || ' '}</span>
          </div>
          <div className="flex items-end gap-1">
            <span>โทร TELEPHONE</span>
            <span className="border-b border-gray-500 px-1 min-w-[120px] text-center">
              {data.guestContact || ' '}
            </span>
          </div>
        </div>

        {/* Guest fields — most blank for hand-fill, matching iHOTEL */}
        <div className="space-y-2">
          <div className="grid grid-cols-[2fr_1fr] gap-4">
            <Field thai="ชื่อตัว-สกุล" eng="FIRST NAME - FAMILY NAME" value={data.guestName} />
            <Field thai="ทะเบียนรถ" eng="CAR ID." value={data.vehiclePlate} />
          </div>
          <div className="grid grid-cols-4 gap-4">
            <Field thai="วัน เดือน ปี เกิด" eng="DATE OF BIRTH" />
            <Field thai="อายุ ปี" eng="AGE / YEAR" />
            <Field thai="สัญชาติ" eng="NATIONALITY" value={data.nationality} />
            <Field thai="อาชีพ" eng="OCCUPATION" />
          </div>
          <div className="grid grid-cols-[3fr_1fr] gap-4">
            <Field thai="ที่อยู่" eng="ADDRESS" value={data.guestAddress} />
            <Field thai="ประเทศ" eng="COUNTRY" />
          </div>
          <Field
            thai="บัตรประจำตัวประชาชน/หนังสือเดินทาง เลขที่"
            eng="IDENTITY CARD / PASSPORT NO."
            value={data.guestIdCard}
          />
          <div className="grid grid-cols-2 gap-4">
            <Field thai="วันที่ออกบัตร" eng="DATE OF ISSUE" />
            <Field thai="สถานที่ออกบัตร" eng="PLACE OF ISSUE" />
          </div>
          <div className="grid grid-cols-2 gap-4">
            <Field thai="มาจาก" eng="COMING FROM" />
            <Field thai="จะไปที่" eng="DEPARTURE TO" />
          </div>
        </div>

        {/* Stay table */}
        <table className="w-full border-collapse mt-4 text-[11px]">
          <thead>
            <tr>
              {['C-IN DATE', 'C-OUT DATE', 'ROOM NO', 'RATE', 'OTHER', 'TOTAL'].map((h) => (
                <th key={h} className="border border-gray-600 px-2 py-1 font-semibold">
                  {h}
                </th>
              ))}
            </tr>
          </thead>
          <tbody>
            <tr className="text-center">
              <td className="border border-gray-600 px-2 py-1">{formatDate(data.checkInDate)}</td>
              <td className="border border-gray-600 px-2 py-1">{formatDate(data.checkOutDate)}</td>
              <td className="border border-gray-600 px-2 py-1">{data.roomNumber}</td>
              <td className="border border-gray-600 px-2 py-1 text-right">{formatCurrency(rate)}</td>
              <td className="border border-gray-600 px-2 py-1 text-right">{formatCurrency(other)}</td>
              <td className="border border-gray-600 px-2 py-1 text-right">{formatCurrency(total)}</td>
            </tr>
          </tbody>
        </table>

        {/* Terms of payment */}
        <div className="mt-3">
          <div className="flex items-center justify-between">
            <span className="font-medium">เงื่อนไขการชำระเงิน TERM OF PAYMENT</span>
            <span className="flex gap-4">
              <Box label="WALK IN" />
              <Box label="FIT" />
              <Box label="GROUP" />
            </span>
          </div>
          <div className="grid grid-cols-2 gap-x-8 gap-y-1 mt-1">
            <Box label="เงินสด" eng="CASH" />
            {/* fill blanks use items-end + a sized span so the border sits on
                the baseline (underscore), not the line centre (strikethrough). */}
            <div className="flex items-end gap-1">
              <Box label="บัตรเครดิต" eng="CREDIT CARD" />
              <span className="border-b border-gray-500 flex-1 min-h-[1.1em]">&nbsp;</span>
              <span className="text-[9px] text-gray-500">EXP. DATE</span>
              <span className="border-b border-gray-500 w-12 min-h-[1.1em]">&nbsp;</span>
            </div>
            <div className="flex items-end gap-1">
              <Box label="บัญชี" eng="ACCOUNT" />
              <span className="border-b border-gray-500 flex-1 min-h-[1.1em]">&nbsp;</span>
            </div>
            <div className="flex items-end gap-1">
              <Box label="ใบเสร็จรับเงิน" eng="VOUCHER" />
              <span className="border-b border-gray-500 flex-1 min-h-[1.1em]">&nbsp;</span>
            </div>
            <Box label="รับรอง" eng="COMPLIMENTARY" />
            <Box label="ใบ Folio" />
            <Box label="ใบกำกับภาษี" />
          </div>
        </div>

        {/* Advance paid against the reservation (booking deposit). iHOTEL only
            prints this line when the stay came from a booking (Print_Report.cs:
            `if Cin_book_no != ''`), so we omit it entirely for walk-ins. */}
        {data.bookingNo && (
          <div className="mt-2">
            จ่ายล่วงหน้า จากการจองเลขที่{' '}
            <span className="font-medium">{data.bookingNo}</span> จำนวนเงิน{' '}
            <span className="font-medium">{formatCurrency(data.bookingAdvance ?? 0)}</span> บาท
          </div>
        )}

        {/* Signatures */}
        <div className="flex justify-between mt-12">
          <div className="w-56 text-center">
            <div className="border-b border-gray-500 h-10" />
            <div className="mt-1">ลงชื่อพนักงานต้อนรับ</div>
            <div className="text-[9px] text-gray-500">RECEPTION</div>
          </div>
          <div className="w-56 text-center">
            <div className="border-b border-gray-500 h-10" />
            <div className="mt-1">ลายมือชื่อผู้เข้าพัก</div>
            <div className="text-[9px] text-gray-500">GUEST SIGNATURE</div>
          </div>
        </div>
      </div>
    </div>,
    document.body,
  )
}
