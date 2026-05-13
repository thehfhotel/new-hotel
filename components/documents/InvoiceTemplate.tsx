'use client'

import { InvoiceData, HotelInfo } from '@/types/invoice'

interface InvoiceTemplateProps {
  checkInData: InvoiceData;
  hotelInfo: HotelInfo;
  showVat?: boolean;
}

/**
 * Convert Gregorian year to Buddhist Era (BE) year
 * Thai Buddhist Era = Gregorian Year + 543
 */
function toBuddhistYear(date: Date): number {
  return date.getFullYear() + 543
}

/**
 * Format date in Thai format with Buddhist Era
 * Example: "5 กุมภาพันธ์ 2569"
 */
function formatThaiDate(dateStr: string): string {
  if (!dateStr) return '-'
  const date = new Date(dateStr)
  const thaiMonths = [
    'มกราคม', 'กุมภาพันธ์', 'มีนาคม', 'เมษายน', 'พฤษภาคม', 'มิถุนายน',
    'กรกฎาคม', 'สิงหาคม', 'กันยายน', 'ตุลาคม', 'พฤศจิกายน', 'ธันวาคม'
  ]
  const day = date.getDate()
  const month = thaiMonths[date.getMonth()]
  const year = toBuddhistYear(date)
  return `${day} ${month} ${year}`
}

/**
 * Format date in short Thai format
 * Example: "05/02/2569"
 */
function formatThaiDateShort(dateStr: string): string {
  if (!dateStr) return '-'
  const date = new Date(dateStr)
  const day = String(date.getDate()).padStart(2, '0')
  const month = String(date.getMonth() + 1).padStart(2, '0')
  const year = toBuddhistYear(date)
  return `${day}/${month}/${year}`
}

/**
 * Format currency in Thai Baht
 */
function formatCurrency(amount: number): string {
  return new Intl.NumberFormat('th-TH', {
    minimumFractionDigits: 2,
    maximumFractionDigits: 2,
  }).format(amount)
}

export default function InvoiceTemplate({
  checkInData,
  hotelInfo,
  showVat = false,
}: InvoiceTemplateProps) {
  const today = new Date()

  return (
    <div className="invoice-container bg-white p-8 max-w-[210mm] mx-auto font-sans text-gray-900">
      {/* Print Styles */}
      <style jsx global>{`
        @media print {
          @page {
            size: A4;
            margin: 15mm;
          }
          body {
            print-color-adjust: exact;
            -webkit-print-color-adjust: exact;
          }
          .invoice-container {
            padding: 0;
            margin: 0;
            max-width: 100%;
          }
          .no-print {
            display: none !important;
          }
        }
      `}</style>

      {/* Header - Hotel Information */}
      <div className="border-b-2 border-gray-800 pb-6 mb-6">
        <div className="flex justify-between items-start">
          <div className="flex items-center gap-4">
            {hotelInfo.logo && (
              <img
                src={hotelInfo.logo}
                alt={hotelInfo.name}
                className="w-16 h-16 object-contain"
              />
            )}
            <div>
              <h1 className="text-2xl font-bold text-gray-900">{hotelInfo.name}</h1>
              <p className="text-sm text-gray-600 mt-1">{hotelInfo.address}</p>
              <p className="text-sm text-gray-600">โทร: {hotelInfo.phone}</p>
              <p className="text-sm text-gray-600">เลขประจำตัวผู้เสียภาษี: {hotelInfo.taxId}</p>
            </div>
          </div>
          <div className="text-right">
            <h2 className="text-xl font-bold text-gray-900">ใบแจ้งหนี้ / Invoice</h2>
            <p className="text-sm text-gray-600 mt-2">
              เลขที่ / No: <span className="font-semibold">{checkInData.invoiceNumber}</span>
            </p>
            <p className="text-sm text-gray-600">
              วันที่ / Date: <span className="font-semibold">{formatThaiDate(checkInData.createdAt || today.toISOString())}</span>
            </p>
            {/* Track G3: corporate buyer tax-id appears in the header so
                Thai-VAT compliance is satisfied without the printer
                having to hunt for it. Hidden when the guest is an
                individual walk-in (cust_work_tax = NULL). */}
            {checkInData.guestTaxId && (
              <p className="text-sm text-gray-600">
                เลขประจำตัวผู้เสียภาษีของผู้ซื้อ / Buyer Tax ID:{' '}
                <span className="font-semibold">{checkInData.guestTaxId}</span>
              </p>
            )}
          </div>
        </div>
      </div>

      {/* Guest Information */}
      <div className="mb-6 bg-gray-50 p-4 rounded-lg border border-gray-200">
        <h3 className="text-sm font-semibold text-gray-700 uppercase mb-3">ข้อมูลผู้เข้าพัก / Guest Information</h3>
        <div className="grid grid-cols-2 gap-4 text-sm">
          <div>
            <span className="text-gray-500">ชื่อ-สกุล / Name:</span>
            <span className="ml-2 font-medium">{checkInData.guestName || '-'}</span>
          </div>
          <div>
            <span className="text-gray-500">เลขบัตรประชาชน / ID Card:</span>
            <span className="ml-2 font-medium">{checkInData.guestIdCard || '-'}</span>
          </div>
          <div>
            <span className="text-gray-500">ติดต่อ / Contact:</span>
            <span className="ml-2 font-medium">{checkInData.guestContact || '-'}</span>
          </div>
          <div>
            <span className="text-gray-500">รหัสเช็คอิน / Check-in ID:</span>
            <span className="ml-2 font-medium">{checkInData.checkInId}</span>
          </div>
        </div>
      </div>

      {/* Stay Period */}
      <div className="mb-6 flex justify-between text-sm">
        <div>
          <span className="text-gray-500">วันที่เข้าพัก / Check-in:</span>
          <span className="ml-2 font-medium">{formatThaiDateShort(checkInData.checkInDate)}</span>
        </div>
        <div>
          <span className="text-gray-500">วันที่ออก / Check-out:</span>
          <span className="ml-2 font-medium">{formatThaiDateShort(checkInData.checkOutDate)}</span>
        </div>
      </div>

      {/* Room Charges Table */}
      <div className="mb-6">
        <h3 className="text-sm font-semibold text-gray-700 uppercase mb-3">รายละเอียดค่าห้องพัก / Room Charges</h3>
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
              <th className="border border-gray-300 px-3 py-2 text-right text-sm font-semibold">
                รวม / Subtotal
              </th>
            </tr>
          </thead>
          <tbody>
            {checkInData.rooms.map((room, index) => (
              <tr key={index} className="hover:bg-gray-50">
                <td className="border border-gray-300 px-3 py-2 text-sm">
                  {room.roomNumber}
                </td>
                <td className="border border-gray-300 px-3 py-2 text-sm">
                  {room.roomType}
                </td>
                <td className="border border-gray-300 px-3 py-2 text-sm text-center">
                  {room.nights}
                </td>
                <td className="border border-gray-300 px-3 py-2 text-sm text-right">
                  {formatCurrency(room.ratePerNight)}
                </td>
                <td className="border border-gray-300 px-3 py-2 text-sm text-right font-medium">
                  {formatCurrency(room.subtotal)}
                </td>
              </tr>
            ))}
          </tbody>
        </table>
      </div>

      {/* Summary */}
      <div className="mb-8">
        <div className="flex justify-end">
          <div className="w-72">
            <div className="flex justify-between py-2 text-sm">
              <span className="text-gray-600">ยอดรวมค่าห้อง / Subtotal:</span>
              <span className="font-medium">{formatCurrency(checkInData.subtotal)} บาท</span>
            </div>

            {checkInData.discount > 0 && (
              <div className="flex justify-between py-2 text-sm text-green-600">
                <span>ส่วนลด / Discount:</span>
                <span className="font-medium">-{formatCurrency(checkInData.discount)} บาท</span>
              </div>
            )}

            {/* Track G3 / T4 HIGH-7: VAT-inclusive split. Render the
                "before VAT" line only when the API supplied a value AND
                showVat is requested AND there is actual VAT to print.
                Older API responses (pre-G3) ship vatAmount=0 and no
                beforeVat — those skip the line entirely (preserved
                behaviour for non-VAT receipts). */}
            {showVat && checkInData.vatAmount > 0 && typeof checkInData.beforeVat === 'number' && (
              <div className="flex justify-between py-2 text-sm">
                <span className="text-gray-600">มูลค่าก่อนภาษี / Before VAT:</span>
                <span className="font-medium">{formatCurrency(checkInData.beforeVat)} บาท</span>
              </div>
            )}

            {showVat && checkInData.vatAmount > 0 && (
              <div className="flex justify-between py-2 text-sm">
                <span className="text-gray-600">
                  ภาษีมูลค่าเพิ่ม {checkInData.vatPercent ?? 7}% / VAT{' '}
                  {checkInData.vatPercent ?? 7}%:
                </span>
                <span className="font-medium">{formatCurrency(checkInData.vatAmount)} บาท</span>
              </div>
            )}

            <div className="flex justify-between py-3 border-t-2 border-gray-800 mt-2">
              <span className="text-lg font-bold">ยอดรวมทั้งสิ้น / Grand Total:</span>
              <span className="text-lg font-bold text-blue-600">{formatCurrency(checkInData.grandTotal)} บาท</span>
            </div>
          </div>
        </div>
      </div>

      {/* Notes */}
      <div className="mb-8 text-sm text-gray-500">
        <p>หมายเหตุ / Notes:</p>
        <ul className="list-disc list-inside mt-1 space-y-1">
          <li>กรุณาตรวจสอบความถูกต้องของใบแจ้งหนี้</li>
          <li>Please verify the accuracy of this invoice.</li>
        </ul>
      </div>

      {/* Footer */}
      <div className="mt-8 pt-4 border-t border-gray-200 text-center text-xs text-gray-400">
        <p>เอกสารนี้ออกโดยระบบคอมพิวเตอร์ / This document is computer generated.</p>
        <p className="mt-1">{hotelInfo.name} - {hotelInfo.phone}</p>
      </div>
    </div>
  )
}
