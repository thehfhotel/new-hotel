'use client'

import { useState, useEffect, use, useCallback } from 'react'
import { useBranchFetch } from '@/lib/use-branch-fetch'
import { useBranch } from '@/contexts/BranchContext'
import { hotelInfoForBranch } from '@/lib/hotel-info'
import Link from 'next/link'
import { ArrowLeft, Loader2, AlertCircle, Receipt } from 'lucide-react'
import InvoiceTemplate from '@/components/documents/InvoiceTemplate'
import LegacyMirrorPanels from '@/components/LegacyMirrorPanels'
import PrintButton from '@/components/ui/PrintButton'
import { InvoiceData } from '@/types/invoice'

interface InvoiceApiResponse {
  success: boolean
  invoice: {
    checkinId: number
    cinNo: string
    bookingId: number | null
    bookingNo: string | null
    /** Track G3 / T4 HIGH-7: PG-only invoice number (deferred legacy
     *  HT_INVOICE.INV_NO alignment — see new_invoice.rs module doc). */
    invNo: string
    guest: {
      id: number
      firstName: string
      lastName: string | null
      fullName: string
      email: string | null
      phone: string | null
      address: string | null
      idCard: string | null
      passport: string | null
      /** Track G3: corporate buyer tax-id from cust_work_tax. */
      taxId: string | null
    }
    room: {
      roomId: number
      roomNo: string
      roomType: string | null
      floor: number | null
    }
    checkInTime: string | null
    checkOutTime: string | null
    expectedCheckout: string | null
    adults: number
    children: number
    rates: {
      ratePerNight: number
      nights: number
      subtotal: number
    }
    totalAmount: number
    /** Track G3: VAT-inclusive split (banker's rounding server-side). */
    beforeVat: number
    vatAmount: number
    vatPer: number
    paymentStatus: string | null
    notes: string | null
    createdAt: string | null
  }
  error?: string
}

export default function InvoiceDetailPage({
  params,
}: {
  params: Promise<{ id: string }>
}) {
  const resolvedParams = use(params)
  const branchFetch = useBranchFetch()
  const { branch } = useBranch()
  const hotelInfo = hotelInfoForBranch(branch)
  const [invoiceData, setInvoiceData] = useState<InvoiceData | null>(null)
  const [loading, setLoading] = useState(true)
  const [error, setError] = useState<string | null>(null)

  const fetchInvoice = useCallback(async () => {
    setLoading(true)
    setError(null)

    try {
      const response = await branchFetch(`/api/new/checkins/${resolvedParams.id}/invoice`)
      const data: InvoiceApiResponse = await response.json()

      if (data.success && data.invoice) {
        // Transform API response to InvoiceData format
        const invoice = data.invoice
        const transformedData: InvoiceData = {
          // Track G3: prefer the dedicated invoice number (INVyyMM-NNNNNN)
          // when the backend supplied one; fall back to the legacy cin_no
          // for older API responses that pre-date the G3 field.
          invoiceNumber: invoice.invNo || invoice.cinNo,
          cinNo: invoice.cinNo,
          checkInId: invoice.checkinId,
          guestName: invoice.guest.fullName,
          guestIdCard: invoice.guest.idCard || invoice.guest.passport || '',
          guestContact: invoice.guest.phone || invoice.guest.email || '',
          // Track G3: corporate buyer tax-id. `undefined` for individual
          // walk-ins so the InvoiceTemplate header hides the row.
          guestTaxId: invoice.guest.taxId || undefined,
          checkInDate: invoice.checkInTime || '',
          checkOutDate: invoice.checkOutTime || invoice.expectedCheckout || '',
          rooms: [
            {
              roomNumber: invoice.room.roomNo,
              roomType: invoice.room.roomType || 'Standard',
              ratePerNight: invoice.rates.ratePerNight,
              nights: invoice.rates.nights,
              subtotal: invoice.rates.subtotal,
            },
          ],
          subtotal: invoice.rates.subtotal,
          discount: 0,
          // Track G3: VAT-inclusive split — derived server-side via
          // banker's rounding so the printed receipt matches the legacy
          // .NET app's Math.Round behaviour.
          beforeVat: invoice.beforeVat,
          vatAmount: invoice.vatAmount ?? 0,
          vatPercent: invoice.vatPer,
          grandTotal: invoice.totalAmount || invoice.rates.subtotal,
          paymentMethod: invoice.paymentStatus || 'pending',
          paidAmount: invoice.totalAmount || 0,
          createdAt: invoice.createdAt || new Date().toISOString(),
        }
        setInvoiceData(transformedData)
      } else {
        setError(data.error || 'ไม่สามารถโหลดข้อมูลใบแจ้งหนี้ได้')
      }
    } catch (err) {
      setError(err instanceof Error ? err.message : 'เกิดข้อผิดพลาดในการโหลดข้อมูล')
    } finally {
      setLoading(false)
    }
  }, [resolvedParams.id, branchFetch])

  useEffect(() => {
    fetchInvoice()
  }, [fetchInvoice])

  if (loading) {
    return (
      <div className="flex flex-col items-center justify-center min-h-[400px]">
        <Loader2 className="w-12 h-12 animate-spin text-red-500" />
        <p className="mt-4 text-gray-500">กำลังโหลดข้อมูลใบแจ้งหนี้...</p>
      </div>
    )
  }

  if (error) {
    return (
      <div className="space-y-6">
        {/* Back Button */}
        <Link
          href="/billing"
          className="inline-flex items-center gap-2 text-gray-500 hover:text-gray-900"
        >
          <ArrowLeft className="w-5 h-5" />
          กลับไปหน้ารายการ
        </Link>

        {/* Error Message */}
        <div className="flex flex-col items-center justify-center min-h-[300px]">
          <div className="flex items-center gap-2 p-4 bg-red-50 border border-red-200 rounded-lg text-red-600 max-w-md">
            <AlertCircle className="w-6 h-6 shrink-0" />
            <div>
              <p className="font-medium">เกิดข้อผิดพลาด</p>
              <p className="text-sm">{error}</p>
            </div>
          </div>
        </div>
      </div>
    )
  }

  if (!invoiceData) {
    return (
      <div className="space-y-6">
        {/* Back Button */}
        <Link
          href="/billing"
          className="inline-flex items-center gap-2 text-gray-500 hover:text-gray-900"
        >
          <ArrowLeft className="w-5 h-5" />
          กลับไปหน้ารายการ
        </Link>

        {/* Not Found */}
        <div className="flex flex-col items-center justify-center min-h-[300px] text-gray-500">
          <Receipt className="w-16 h-16 text-gray-500 mb-4" />
          <p className="text-lg">ไม่พบข้อมูลใบแจ้งหนี้</p>
        </div>
      </div>
    )
  }

  return (
    <div className="space-y-6">
      {/* Header Actions */}
      <div className="flex items-center justify-between no-print">
        <Link
          href="/billing"
          className="inline-flex items-center gap-2 text-gray-500 hover:text-gray-900"
        >
          <ArrowLeft className="w-5 h-5" />
          กลับไปหน้ารายการ
        </Link>

        <PrintButton size="md" showPdfOption={true} />
      </div>

      {/* Invoice Template — Track G3: show the VAT split whenever the
          guest is a juristic person (has a buyer tax-id). Individual
          walk-ins still print the inclusive total only. */}
      <InvoiceTemplate
        checkInData={invoiceData}
        hotelInfo={hotelInfo}
        showVat={Boolean(invoiceData.guestTaxId)}
      />

      {/* Phase 5.5e — legacy_mirror panels (coupons / minibar / room moves).
          Hidden in print so the receipt itself stays clean. */}
      {invoiceData.cinNo && (
        <div className="no-print">
          <LegacyMirrorPanels cinNo={invoiceData.cinNo} />
        </div>
      )}
    </div>
  )
}
