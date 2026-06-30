'use client'

import { useEffect, useState } from 'react'
import { createPortal } from 'react-dom'
import { HotelInfo } from '@/types/invoice'

/**
 * Data for a check-in registration slip (ใบลงทะเบียนเข้าพัก). Mirrors the
 * shape the check-in modals already hold after a successful POST /api/checkins
 * (form fields + the returned `cin_no`). All amounts are baht.
 */
export interface RegistrationSlipData {
  /** Legacy-style check-in number (e.g. "CIN-20260627-0001"). */
  registrationNo: string
  checkInId?: number
  guestName: string
  guestIdCard?: string
  guestContact?: string
  roomNumber: string
  roomType?: string
  /** ISO date string. */
  checkInDate: string
  /** ISO date string. */
  checkOutDate: string
  nights: number
  ratePerNight?: number
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

/** Gregorian → Thai Buddhist Era. */
function toBuddhistYear(date: Date): number {
  return date.getFullYear() + 543
}

/** "5 กุมภาพันธ์ 2569" */
function formatThaiDate(dateStr: string): string {
  if (!dateStr) return '-'
  const date = new Date(dateStr)
  if (Number.isNaN(date.getTime())) return '-'
  const thaiMonths = [
    'มกราคม', 'กุมภาพันธ์', 'มีนาคม', 'เมษายน', 'พฤษภาคม', 'มิถุนายน',
    'กรกฎาคม', 'สิงหาคม', 'กันยายน', 'ตุลาคม', 'พฤศจิกายน', 'ธันวาคม',
  ]
  return `${date.getDate()} ${thaiMonths[date.getMonth()]} ${toBuddhistYear(date)}`
}

/** "05/02/2569" */
function formatThaiDateShort(dateStr: string): string {
  if (!dateStr) return '-'
  const date = new Date(dateStr)
  if (Number.isNaN(date.getTime())) return '-'
  const day = String(date.getDate()).padStart(2, '0')
  const month = String(date.getMonth() + 1).padStart(2, '0')
  return `${day}/${month}/${toBuddhistYear(date)}`
}

function formatCurrency(amount: number): string {
  return new Intl.NumberFormat('th-TH', {
    minimumFractionDigits: 2,
    maximumFractionDigits: 2,
  }).format(amount)
}

/**
 * Print-only registration slip. Renders into a portal at `document.body` so a
 * `window.print()` (e.g. via `PrintButton`) prints ONLY this sheet — the
 * `@media print` rules below hide every other top-level node (the modal, the
 * dashboard behind it). On screen the slip is `display:none`.
 */
export default function RegistrationSlipTemplate({
  data,
  hotelInfo,
}: RegistrationSlipTemplateProps) {
  // Portals require the DOM; guard SSR / first paint.
  const [mounted, setMounted] = useState(false)
  useEffect(() => setMounted(true), [])
  if (!mounted) return null

  const today = new Date().toISOString()
  const hasDeposit = (data.deposit ?? 0) > 0

  return createPortal(
    <div className="registration-slip-print-root">
      <style jsx global>{`
        .registration-slip-print-root {
          display: none;
        }
        @media print {
          @page {
            size: A4;
            margin: 15mm;
          }
          body {
            print-color-adjust: exact;
            -webkit-print-color-adjust: exact;
          }
          /* Hide everything except this slip (the modal + dashboard live in
             the Next.js root, a sibling of this portal node). */
          body > *:not(.registration-slip-print-root) {
            display: none !important;
          }
          .registration-slip-print-root {
            display: block !important;
          }
          /* The v2 layout's print CSS hides everything via a body-wide
             visibility:hidden and only un-hides its own .v2-print-active region.
             This portal isn't that, so make it visibility-robust too, or the
             slip prints blank under /v2 even though display is block. No-op in
             the classic app. */
          .registration-slip-print-root,
          .registration-slip-print-root * {
            visibility: visible !important;
          }
        }
      `}</style>

      <div className="registration-slip bg-white p-8 max-w-[210mm] mx-auto font-sans text-gray-900">
        {/* Header — Hotel Information */}
        <div className="border-b-2 border-gray-800 pb-6 mb-6">
          <div className="flex justify-between items-start">
            <div className="flex items-center gap-4">
              {hotelInfo.logo && (
                <img src={hotelInfo.logo} alt={hotelInfo.name} className="w-16 h-16 object-contain" />
              )}
              <div>
                <h1 className="text-2xl font-bold text-gray-900">{hotelInfo.name}</h1>
                <p className="text-sm text-gray-600 mt-1">{hotelInfo.address}</p>
                <p className="text-sm text-gray-600">โทร: {hotelInfo.phone}</p>
                <p className="text-sm text-gray-600">เลขประจำตัวผู้เสียภาษี: {hotelInfo.taxId}</p>
              </div>
            </div>
            <div className="text-right">
              <h2 className="text-xl font-bold text-gray-900">ใบลงทะเบียนเข้าพัก</h2>
              <p className="text-sm text-gray-600">Registration Form</p>
              <p className="text-sm text-gray-600 mt-2">
                เลขที่ / No: <span className="font-semibold">{data.registrationNo}</span>
              </p>
              <p className="text-sm text-gray-600">
                วันที่ / Date:{' '}
                <span className="font-semibold">{formatThaiDate(data.createdAt || today)}</span>
              </p>
            </div>
          </div>
        </div>

        {/* Guest Information */}
        <div className="mb-6 bg-gray-50 p-4 rounded-lg border border-gray-200">
          <h3 className="text-sm font-semibold text-gray-700 uppercase mb-3">
            ข้อมูลผู้เข้าพัก / Guest Information
          </h3>
          <div className="grid grid-cols-2 gap-4 text-sm">
            <div>
              <span className="text-gray-500">ชื่อ-สกุล / Name:</span>
              <span className="ml-2 font-medium">{data.guestName || '-'}</span>
            </div>
            <div>
              <span className="text-gray-500">เลขบัตรประชาชน / ID Card:</span>
              <span className="ml-2 font-medium">{data.guestIdCard || '-'}</span>
            </div>
            <div>
              <span className="text-gray-500">ติดต่อ / Contact:</span>
              <span className="ml-2 font-medium">{data.guestContact || '-'}</span>
            </div>
            <div>
              <span className="text-gray-500">จำนวนผู้เข้าพัก / Guests:</span>
              <span className="ml-2 font-medium">
                {data.adults ?? 1} ผู้ใหญ่ / Adults
                {(data.children ?? 0) > 0 ? `, ${data.children} เด็ก / Children` : ''}
              </span>
            </div>
          </div>
        </div>

        {/* Stay Period */}
        <div className="mb-6 flex justify-between text-sm">
          <div>
            <span className="text-gray-500">วันที่เข้าพัก / Check-in:</span>
            <span className="ml-2 font-medium">{formatThaiDateShort(data.checkInDate)}</span>
          </div>
          <div>
            <span className="text-gray-500">วันที่ออก / Check-out:</span>
            <span className="ml-2 font-medium">{formatThaiDateShort(data.checkOutDate)}</span>
          </div>
          <div>
            <span className="text-gray-500">จำนวนคืน / Nights:</span>
            <span className="ml-2 font-medium">{data.nights}</span>
          </div>
        </div>

        {/* Room + charges */}
        <div className="mb-6">
          <h3 className="text-sm font-semibold text-gray-700 uppercase mb-3">
            รายละเอียดห้องพัก / Room Details
          </h3>
          <table className="w-full border-collapse">
            <thead>
              <tr className="bg-gray-100">
                <th className="border border-gray-300 px-3 py-2 text-left text-sm font-semibold">
                  ห้อง / Room
                </th>
                <th className="border border-gray-300 px-3 py-2 text-left text-sm font-semibold">
                  ประเภท / Type
                </th>
                <th className="border border-gray-300 px-3 py-2 text-center text-sm font-semibold">
                  จำนวนคืน / Nights
                </th>
                <th className="border border-gray-300 px-3 py-2 text-right text-sm font-semibold">
                  ราคา/คืน / Rate
                </th>
              </tr>
            </thead>
            <tbody>
              <tr>
                <td className="border border-gray-300 px-3 py-2 text-sm">{data.roomNumber}</td>
                <td className="border border-gray-300 px-3 py-2 text-sm">{data.roomType || '-'}</td>
                <td className="border border-gray-300 px-3 py-2 text-sm text-center">{data.nights}</td>
                <td className="border border-gray-300 px-3 py-2 text-sm text-right">
                  {data.ratePerNight != null ? `${formatCurrency(data.ratePerNight)} บาท` : '-'}
                </td>
              </tr>
            </tbody>
          </table>
        </div>

        {/* Deposit */}
        {hasDeposit && (
          <div className="mb-6 flex justify-end">
            <div className="w-72">
              <div className="flex justify-between py-2 text-sm border-t border-gray-300">
                <span className="text-gray-600">เงินมัดจำ / Deposit:</span>
                <span className="font-medium">{formatCurrency(data.deposit as number)} บาท</span>
              </div>
            </div>
          </div>
        )}

        {/* Consent / acknowledgement */}
        <div className="mb-8 text-sm text-gray-600">
          <p>
            ข้าพเจ้าขอรับรองว่าข้อมูลข้างต้นเป็นความจริง และยอมรับเงื่อนไขการเข้าพักของโรงแรม
          </p>
          <p className="mt-1">
            I certify that the information above is correct and accept the hotel&apos;s terms of stay.
          </p>
        </div>

        {/* Signatures */}
        <div className="mb-8 mt-12">
          <div className="flex justify-between">
            <div className="w-48 text-center">
              <div className="border-b border-gray-400 h-16 mb-2"></div>
              <p className="text-sm text-gray-600">ผู้เข้าพัก / Guest</p>
            </div>
            <div className="w-48 text-center">
              <div className="border-b border-gray-400 h-16 mb-2"></div>
              <p className="text-sm text-gray-600">เจ้าหน้าที่ / Receptionist</p>
            </div>
          </div>
        </div>

        {/* Footer */}
        <div className="mt-8 pt-4 border-t border-gray-200 text-center text-xs text-gray-400">
          <p>เอกสารนี้ออกโดยระบบคอมพิวเตอร์ / This document is computer generated.</p>
          <p className="mt-1">
            {hotelInfo.name} - {hotelInfo.phone}
          </p>
        </div>
      </div>
    </div>,
    document.body,
  )
}
