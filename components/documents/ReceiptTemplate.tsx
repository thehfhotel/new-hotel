'use client'

import { ReceiptData, HotelInfo } from '@/types/invoice'

interface ReceiptTemplateProps {
  receiptData: ReceiptData;
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

/**
 * Get payment method label in Thai
 */
function getPaymentMethodLabel(method: string): string {
  const labels: Record<string, string> = {
    'cash': 'เงินสด / Cash',
    'credit': 'บัตรเครดิต / Credit Card',
    'transfer': 'โอนเงิน / Bank Transfer',
    'qr': 'QR Code / PromptPay',
  }
  return labels[method.toLowerCase()] || method
}

export default function ReceiptTemplate({
  receiptData,
  hotelInfo,
  showVat = false,
}: ReceiptTemplateProps) {
  const today = new Date()

  return (
    <div className="receipt-container bg-white p-8 max-w-[210mm] mx-auto font-sans text-gray-900">
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
          .receipt-container {
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
            <h2 className="text-xl font-bold text-gray-900">ใบเสร็จรับเงิน / Receipt</h2>
            <p className="text-sm text-gray-600 mt-2">
              เลขที่ / No: <span className="font-semibold">{receiptData.receiptNumber}</span>
            </p>
            <p className="text-sm text-gray-600">
              วันที่ / Date: <span className="font-semibold">{formatThaiDate(receiptData.createdAt || today.toISOString())}</span>
            </p>
          </div>
        </div>
      </div>

      {/* Guest Information */}
      <div className="mb-6 bg-gray-50 p-4 rounded-lg border border-gray-200">
        <h3 className="text-sm font-semibold text-gray-700 uppercase mb-3">ข้อมูลผู้เข้าพัก / Guest Information</h3>
        <div className="grid grid-cols-2 gap-4 text-sm">
          <div>
            <span className="text-gray-500">ชื่อ-สกุล / Name:</span>
            <span className="ml-2 font-medium">{receiptData.guestName || '-'}</span>
          </div>
          <div>
            <span className="text-gray-500">เลขบัตรประชาชน / ID Card:</span>
            <span className="ml-2 font-medium">{receiptData.guestIdCard || '-'}</span>
          </div>
          <div>
            <span className="text-gray-500">ติดต่อ / Contact:</span>
            <span className="ml-2 font-medium">{receiptData.guestContact || '-'}</span>
          </div>
          <div>
            <span className="text-gray-500">รหัสเช็คอิน / Check-in ID:</span>
            <span className="ml-2 font-medium">{receiptData.checkInId}</span>
          </div>
        </div>
      </div>

      {/* Stay Period */}
      <div className="mb-6 flex justify-between text-sm">
        <div>
          <span className="text-gray-500">วันที่เข้าพัก / Check-in:</span>
          <span className="ml-2 font-medium">{formatThaiDateShort(receiptData.checkInDate)}</span>
        </div>
        <div>
          <span className="text-gray-500">วันที่ออก / Check-out:</span>
          <span className="ml-2 font-medium">{formatThaiDateShort(receiptData.checkOutDate)}</span>
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
            {receiptData.rooms.map((room, index) => (
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
      <div className="mb-6">
        <div className="flex justify-end">
          <div className="w-72">
            <div className="flex justify-between py-2 text-sm">
              <span className="text-gray-600">ยอดรวมค่าห้อง / Subtotal:</span>
              <span className="font-medium">{formatCurrency(receiptData.subtotal)} บาท</span>
            </div>

            {receiptData.discount > 0 && (
              <div className="flex justify-between py-2 text-sm text-green-600">
                <span>ส่วนลด / Discount:</span>
                <span className="font-medium">-{formatCurrency(receiptData.discount)} บาท</span>
              </div>
            )}

            {showVat && receiptData.vatAmount > 0 && (
              <div className="flex justify-between py-2 text-sm">
                <span className="text-gray-600">ภาษีมูลค่าเพิ่ม 7% / VAT 7%:</span>
                <span className="font-medium">{formatCurrency(receiptData.vatAmount)} บาท</span>
              </div>
            )}

            <div className="flex justify-between py-3 border-t-2 border-gray-800 mt-2">
              <span className="text-lg font-bold">ยอดรวมทั้งสิ้น / Grand Total:</span>
              <span className="text-lg font-bold text-blue-600">{formatCurrency(receiptData.grandTotal)} บาท</span>
            </div>
          </div>
        </div>
      </div>

      {/* Payment Information */}
      <div className="mb-6 bg-green-50 p-4 rounded-lg border border-green-200">
        <h3 className="text-sm font-semibold text-green-800 uppercase mb-3">ข้อมูลการชำระเงิน / Payment Information</h3>
        <div className="grid grid-cols-2 gap-4 text-sm">
          <div>
            <span className="text-gray-600">วิธีการชำระเงิน / Payment Method:</span>
            <span className="ml-2 font-medium">{getPaymentMethodLabel(receiptData.paymentMethod)}</span>
          </div>
          <div>
            <span className="text-gray-600">จำนวนเงินที่ชำระ / Amount Paid:</span>
            <span className="ml-2 font-medium text-green-700">{formatCurrency(receiptData.paidAmount)} บาท</span>
          </div>
        </div>

        {/* Paid in Full Indicator */}
        {receiptData.paidAmount >= receiptData.grandTotal && (
          <div className="mt-4 flex items-center justify-center">
            <div className="px-6 py-2 bg-green-600 text-white font-bold rounded-lg text-center">
              ชำระเงินครบถ้วน / PAID IN FULL
            </div>
          </div>
        )}
      </div>

      {/* Signature Section */}
      <div className="mb-8 mt-12">
        <div className="flex justify-between">
          <div className="w-48 text-center">
            <div className="border-b border-gray-400 h-16 mb-2"></div>
            <p className="text-sm text-gray-600">ผู้รับเงิน / Cashier</p>
            {receiptData.cashierName && (
              <p className="text-sm font-medium mt-1">{receiptData.cashierName}</p>
            )}
          </div>
          <div className="w-48 text-center">
            <div className="border-b border-gray-400 h-16 mb-2"></div>
            <p className="text-sm text-gray-600">ผู้เข้าพัก / Guest</p>
          </div>
        </div>
      </div>

      {/* Notes */}
      <div className="mb-6 text-sm text-gray-500">
        <p>หมายเหตุ / Notes:</p>
        <ul className="list-disc list-inside mt-1 space-y-1">
          <li>เอกสารฉบับนี้ถือเป็นหลักฐานการชำระเงินที่ถูกต้อง</li>
          <li>This document serves as a valid proof of payment.</li>
          <li>กรุณาเก็บใบเสร็จนี้ไว้เป็นหลักฐาน</li>
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
