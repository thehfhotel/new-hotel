'use client'

import { StandaloneReceiptData, HotelInfo } from '@/types/invoice'

interface StandaloneReceiptTemplateProps {
  receiptData: StandaloneReceiptData
  hotelInfo: HotelInfo
  /** Show the VAT breakdown line. Defaults to `vatAmount > 0`. */
  showVat?: boolean
}

/**
 * Task #45 — standalone (walk-up / roomless) sale receipt.
 *
 * Unlike `ReceiptTemplate` (which requires a `checkInId` + `rooms[]` from a
 * folio), this template renders a pure product-line receipt — the mirror of
 * legacy `HT_Receipt_H` / `HT_Receipt_Ds`. Used by the walk-up POS UI.
 */

/** Gregorian → Buddhist Era year. */
function toBuddhistYear(date: Date): number {
  return date.getFullYear() + 543
}

/** "5 กุมภาพันธ์ 2569" */
function formatThaiDate(dateStr: string): string {
  if (!dateStr) return '-'
  const date = new Date(dateStr)
  const thaiMonths = [
    'มกราคม', 'กุมภาพันธ์', 'มีนาคม', 'เมษายน', 'พฤษภาคม', 'มิถุนายน',
    'กรกฎาคม', 'สิงหาคม', 'กันยายน', 'ตุลาคม', 'พฤศจิกายน', 'ธันวาคม',
  ]
  const day = date.getDate()
  const month = thaiMonths[date.getMonth()]
  const year = toBuddhistYear(date)
  return `${day} ${month} ${year}`
}

function formatCurrency(amount: number): string {
  return new Intl.NumberFormat('th-TH', {
    minimumFractionDigits: 2,
    maximumFractionDigits: 2,
  }).format(amount)
}

function getPaymentMethodLabel(method: string): string {
  const labels: Record<string, string> = {
    cash: 'เงินสด / Cash',
    credit: 'บัตรเครดิต / Credit Card',
    transfer: 'โอนเงิน / Bank Transfer',
    qr: 'QR Code / PromptPay',
  }
  return labels[method.toLowerCase()] || method
}

export default function StandaloneReceiptTemplate({
  receiptData,
  hotelInfo,
  showVat,
}: StandaloneReceiptTemplateProps) {
  const today = new Date()
  const displayNumber =
    receiptData.receiptNumber || `#${receiptData.receiptId}`
  const showVatLine = showVat ?? receiptData.vatAmount > 0

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
          /* Self-isolate against the v2 layout's body-wide visibility:hidden
             print CSS (it only un-hides .v2-print-active) — without this the
             receipt prints blank under /v2. Hide everything, show only the
             receipt. Improves the classic app too (drops modal chrome). */
          body * {
            visibility: hidden !important;
          }
          .receipt-container,
          .receipt-container * {
            visibility: visible !important;
          }
          .receipt-container {
            position: absolute;
            left: 0;
            top: 0;
            width: 100%;
            padding: 0;
            margin: 0;
            max-width: 100%;
          }
          .no-print {
            display: none !important;
          }
        }
      `}</style>

      {/* Header */}
      <div className="border-b-2 border-gray-800 pb-6 mb-6">
        <div className="flex justify-between items-start">
          <div className="flex items-center gap-4">
            {hotelInfo.logo && (
              // eslint-disable-next-line @next/next/no-img-element
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
              <p className="text-sm text-gray-600">
                เลขประจำตัวผู้เสียภาษี: {hotelInfo.taxId}
              </p>
            </div>
          </div>
          <div className="text-right">
            <h2 className="text-xl font-bold text-gray-900">ใบเสร็จรับเงิน / Receipt</h2>
            <p className="text-sm text-gray-600 mt-2">
              เลขที่ / No: <span className="font-semibold">{displayNumber}</span>
            </p>
            <p className="text-sm text-gray-600">
              วันที่ / Date:{' '}
              <span className="font-semibold">
                {formatThaiDate(receiptData.createdAt || today.toISOString())}
              </span>
            </p>
          </div>
        </div>
      </div>

      {/* Customer (optional — hidden for an anonymous walk-up) */}
      {(receiptData.customerName ||
        receiptData.customerTaxId ||
        receiptData.customerTel) && (
        <div className="mb-6 bg-gray-50 p-4 rounded-lg border border-gray-200">
          <h3 className="text-sm font-semibold text-gray-700 uppercase mb-3">
            ข้อมูลลูกค้า / Customer
          </h3>
          <div className="grid grid-cols-2 gap-4 text-sm">
            <div>
              <span className="text-gray-500">ชื่อ / Name:</span>
              <span className="ml-2 font-medium">{receiptData.customerName || '-'}</span>
            </div>
            {receiptData.customerTaxId && (
              <div>
                <span className="text-gray-500">เลขผู้เสียภาษี / Tax ID:</span>
                <span className="ml-2 font-medium">{receiptData.customerTaxId}</span>
              </div>
            )}
            {receiptData.customerTel && (
              <div>
                <span className="text-gray-500">ติดต่อ / Contact:</span>
                <span className="ml-2 font-medium">{receiptData.customerTel}</span>
              </div>
            )}
            {receiptData.customerAddress && (
              <div>
                <span className="text-gray-500">ที่อยู่ / Address:</span>
                <span className="ml-2 font-medium">{receiptData.customerAddress}</span>
              </div>
            )}
          </div>
        </div>
      )}

      {/* Line items */}
      <div className="mb-6">
        <h3 className="text-sm font-semibold text-gray-700 uppercase mb-3">
          รายการสินค้า / Items
        </h3>
        <table className="w-full border-collapse">
          <thead>
            <tr className="bg-gray-100">
              <th className="border border-gray-300 px-3 py-2 text-left text-sm font-semibold">
                รายการ / Item
              </th>
              <th className="border border-gray-300 px-3 py-2 text-center text-sm font-semibold">
                จำนวน / Qty
              </th>
              <th className="border border-gray-300 px-3 py-2 text-right text-sm font-semibold">
                ราคา/หน่วย / Unit Price
              </th>
              <th className="border border-gray-300 px-3 py-2 text-right text-sm font-semibold">
                รวม / Total
              </th>
            </tr>
          </thead>
          <tbody>
            {receiptData.lines.map((line, index) => (
              <tr key={index} className="hover:bg-gray-50">
                <td className="border border-gray-300 px-3 py-2 text-sm">
                  {line.name}
                  {line.productNo && (
                    <span className="ml-2 text-xs text-gray-400">#{line.productNo}</span>
                  )}
                </td>
                <td className="border border-gray-300 px-3 py-2 text-sm text-center">
                  {line.qty}
                  {line.unit ? ` ${line.unit}` : ''}
                </td>
                <td className="border border-gray-300 px-3 py-2 text-sm text-right">
                  {formatCurrency(line.unitPrice)}
                </td>
                <td className="border border-gray-300 px-3 py-2 text-sm text-right font-medium">
                  {formatCurrency(line.total)}
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
              <span className="text-gray-600">ยอดรวม / Subtotal:</span>
              <span className="font-medium">{formatCurrency(receiptData.subtotal)} บาท</span>
            </div>

            {receiptData.discount > 0 && (
              <div className="flex justify-between py-2 text-sm text-green-600">
                <span>ส่วนลด / Discount:</span>
                <span className="font-medium">-{formatCurrency(receiptData.discount)} บาท</span>
              </div>
            )}

            {showVatLine && (
              <div className="flex justify-between py-2 text-sm">
                <span className="text-gray-600">
                  ภาษีมูลค่าเพิ่ม {receiptData.vatPercent ?? 7}% / VAT:
                </span>
                <span className="font-medium">{formatCurrency(receiptData.vatAmount)} บาท</span>
              </div>
            )}

            <div className="flex justify-between py-3 border-t-2 border-gray-800 mt-2">
              <span className="text-lg font-bold">ยอดรวมทั้งสิ้น / Grand Total:</span>
              <span className="text-lg font-bold text-blue-600">
                {formatCurrency(receiptData.grandTotal)} บาท
              </span>
            </div>
          </div>
        </div>
      </div>

      {/* Payment */}
      <div className="mb-6 bg-green-50 p-4 rounded-lg border border-green-200">
        <h3 className="text-sm font-semibold text-green-800 uppercase mb-3">
          ข้อมูลการชำระเงิน / Payment Information
        </h3>
        <div className="grid grid-cols-2 gap-4 text-sm">
          <div>
            <span className="text-gray-600">วิธีการชำระเงิน / Payment Method:</span>
            <span className="ml-2 font-medium">
              {getPaymentMethodLabel(receiptData.paymentMethod)}
            </span>
          </div>
          <div>
            <span className="text-gray-600">จำนวนเงินที่ชำระ / Amount Paid:</span>
            <span className="ml-2 font-medium text-green-700">
              {formatCurrency(receiptData.paidAmount)} บาท
            </span>
          </div>
        </div>

        {receiptData.paidAmount >= receiptData.grandTotal && (
          <div className="mt-4 flex items-center justify-center">
            <div className="px-6 py-2 bg-green-600 text-white font-bold rounded-lg text-center">
              ชำระเงินครบถ้วน / PAID IN FULL
            </div>
          </div>
        )}
      </div>

      {/* Signature */}
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
            <p className="text-sm text-gray-600">ผู้ซื้อ / Customer</p>
          </div>
        </div>
      </div>

      {/* Footer */}
      <div className="mt-8 pt-4 border-t border-gray-200 text-center text-xs text-gray-400">
        <p>เอกสารนี้ออกโดยระบบคอมพิวเตอร์ / This document is computer generated.</p>
        <p className="mt-1">{hotelInfo.name} - {hotelInfo.phone}</p>
      </div>
    </div>
  )
}
