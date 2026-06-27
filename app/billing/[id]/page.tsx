'use client'

import { useState, useEffect, use, useCallback } from 'react'
import { useBranchFetch } from '@/lib/use-branch-fetch'
import { useBranch } from '@/contexts/BranchContext'
import { hotelInfoForBranch } from '@/lib/hotel-info'
import Link from 'next/link'
import { ArrowLeft, Loader2, AlertCircle, Receipt, CreditCard, Ticket, Banknote } from 'lucide-react'
import InvoiceTemplate from '@/components/documents/InvoiceTemplate'
import LegacyMirrorPanels from '@/components/LegacyMirrorPanels'
import PrintButton from '@/components/ui/PrintButton'
import PaymentModal from '@/components/modals/PaymentModal'
import IssueCouponModal from '@/components/modals/IssueCouponModal'
import { useAuth } from '@/contexts/AuthContext'
import { InvoiceData } from '@/types/invoice'
import { mapInvoiceResponse, type InvoiceApiResponse } from '@/lib/invoice'

/** Task #49 — one per-room deposit line from
 *  `GET /api/checkins/:id/deposits`. `refunded` drives whether the folio
 *  shows the คืนเงินมัดจำ button or a "refunded" badge. */
interface DepositRow {
  crId: number
  roomNo: string
  amount: number
  status: string | null
  refunded: boolean
  returnedAt: string | null
  returnedBy: string | null
}

interface DepositsApiResponse {
  success: boolean
  deposits: DepositRow[]
}

export default function InvoiceDetailPage({
  params,
}: {
  params: Promise<{ id: string }>
}) {
  const resolvedParams = use(params)
  const branchFetch = useBranchFetch()
  const { branch } = useBranch()
  const { hasPermission } = useAuth()
  const hotelInfo = hotelInfoForBranch(branch)
  const [invoiceData, setInvoiceData] = useState<InvoiceData | null>(null)
  const [loading, setLoading] = useState(true)
  const [error, setError] = useState<string | null>(null)
  // M1 task #36: make PaymentModal reachable from the folio/billing page.
  const [showPayment, setShowPayment] = useState(false)
  // Task #48: issue a food/breakfast coupon (คูปองอาหาร) from the folio.
  // Gated on `coupon.issue` (Track G7) so the launcher mirrors the modal's
  // own permission gate — both stay hidden until an operator holds the grant.
  const [showIssueCoupon, setShowIssueCoupon] = useState(false)
  const [paySummary, setPaySummary] = useState<{ totalAmount: number; totalPaid: number } | null>(
    null,
  )
  // Task #49: per-room deposits + the cr_id currently being refunded (so the
  // button can show a spinner / disable itself during the POST).
  const [deposits, setDeposits] = useState<DepositRow[]>([])
  const [refundingCrId, setRefundingCrId] = useState<number | null>(null)

  // Best-effort folio summary (total billed + already paid) so the payment
  // modal pre-fills the outstanding balance. Falls back to the invoice grand
  // total when the endpoint is unavailable.
  const fetchPaySummary = useCallback(async () => {
    try {
      const res = await branchFetch(`/api/checkins/${resolvedParams.id}/payments`)
      const data = await res.json()
      if (data.success) {
        setPaySummary({
          totalAmount: data.totalAmount ?? 0,
          totalPaid: data.totalPaid ?? 0,
        })
      }
    } catch {
      // ignore — modal still opens with the invoice grand-total fallback
    }
  }, [resolvedParams.id, branchFetch])

  // Task #49: load the folio's per-room deposit lines so the page can offer a
  // refund (คืนเงินมัดจำ) on any room still holding an unrefunded deposit.
  const fetchDeposits = useCallback(async () => {
    try {
      const res = await branchFetch(`/api/checkins/${resolvedParams.id}/deposits`)
      const data: DepositsApiResponse = await res.json()
      setDeposits(data.success ? data.deposits : [])
    } catch {
      // ignore — the deposit panel simply stays hidden on failure
      setDeposits([])
    }
  }, [resolvedParams.id, branchFetch])

  // Task #49: mark one room's deposit refunded, then refetch so the row flips
  // to the "refunded" badge. Mirrors iHOTEL FormShowDEPBack.
  const handleRefundDeposit = useCallback(
    async (crId: number) => {
      setRefundingCrId(crId)
      try {
        const res = await branchFetch(`/api/checkins/${resolvedParams.id}/deposit-refund`, {
          method: 'POST',
          headers: { 'Content-Type': 'application/json' },
          body: JSON.stringify({ crId }),
        })
        const data = await res.json()
        if (!res.ok || !data.success) {
          throw new Error(data.error || data.message || 'คืนเงินมัดจำไม่สำเร็จ')
        }
        await fetchDeposits()
      } catch (err) {
        alert(err instanceof Error ? err.message : 'เกิดข้อผิดพลาดในการคืนเงินมัดจำ')
      } finally {
        setRefundingCrId(null)
      }
    },
    [resolvedParams.id, branchFetch, fetchDeposits],
  )

  const fetchInvoice = useCallback(async () => {
    setLoading(true)
    setError(null)

    try {
      const response = await branchFetch(`/api/checkins/${resolvedParams.id}/invoice`)
      const data: InvoiceApiResponse = await response.json()

      if (data.success && data.invoice) {
        // Transform API response to InvoiceData via the shared mapper
        // (lib/invoice.ts) so the classic folio and the v2 tax-invoice
        // page render identically. Task #44 added POS lines + a reconciling
        // grand total to this transform.
        setInvoiceData(mapInvoiceResponse(data.invoice))
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
    fetchPaySummary()
    fetchDeposits()
  }, [fetchInvoice, fetchPaySummary, fetchDeposits])

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

        <div className="flex items-center gap-2">
          {hasPermission('coupon.issue') && (
            <button
              onClick={() => setShowIssueCoupon(true)}
              className="inline-flex items-center gap-2 px-4 py-2 border border-emerald-600 text-emerald-700 rounded-lg hover:bg-emerald-50 text-sm font-medium"
            >
              <Ticket className="w-4 h-4" />
              ออกคูปอง
            </button>
          )}
          <button
            onClick={() => setShowPayment(true)}
            className="inline-flex items-center gap-2 px-4 py-2 bg-red-600 text-white rounded-lg hover:bg-red-700 text-sm font-medium"
          >
            <CreditCard className="w-4 h-4" />
            บันทึกการชำระเงิน
          </button>
          <PrintButton size="md" showPdfOption={true} />
        </div>
      </div>

      {/* Invoice Template — Track G3: show the VAT split whenever the
          guest is a juristic person (has a buyer tax-id). Individual
          walk-ins still print the inclusive total only. Task #44: render
          as a proper Thai tax invoice (ใบกำกับภาษี) with POS lines. */}
      <InvoiceTemplate
        checkInData={invoiceData}
        hotelInfo={hotelInfo}
        showVat={Boolean(invoiceData.guestTaxId)}
        taxInvoice
      />

      {/* Task #49 — deposit refund (คืนเงินมัดจำ). One row per room that took
          a deposit; an unrefunded room shows the refund button, a refunded one
          shows a badge. Mirrors iHOTEL FormShowDEPBack. Hidden in print. */}
      {deposits.length > 0 && (
        <div className="no-print bg-white border border-gray-200 rounded-lg p-4 space-y-3">
          <div className="flex items-center gap-2 text-gray-900 font-medium">
            <Banknote className="w-5 h-5 text-amber-600" />
            ค่ามัดจำ
          </div>
          <div className="divide-y divide-gray-100">
            {deposits.map((dep) => (
              <div
                key={dep.crId}
                className="flex items-center justify-between gap-4 py-2"
              >
                <div className="text-sm">
                  <span className="font-medium text-gray-900">ห้อง {dep.roomNo}</span>
                  <span className="ml-3 text-gray-600">
                    {dep.amount.toLocaleString('th-TH', {
                      style: 'currency',
                      currency: 'THB',
                    })}
                  </span>
                  {dep.refunded && dep.returnedBy && (
                    <span className="ml-3 text-xs text-gray-400">
                      โดย {dep.returnedBy}
                    </span>
                  )}
                </div>
                {dep.refunded ? (
                  <span className="inline-flex items-center px-2.5 py-1 rounded-full text-xs font-medium bg-green-50 text-green-700 border border-green-200">
                    คืนเงินแล้ว
                  </span>
                ) : (
                  <button
                    onClick={() => handleRefundDeposit(dep.crId)}
                    disabled={refundingCrId === dep.crId}
                    className="inline-flex items-center gap-2 px-3 py-1.5 border border-amber-500 text-amber-700 rounded-lg hover:bg-amber-50 text-sm font-medium disabled:opacity-50 disabled:cursor-not-allowed"
                  >
                    {refundingCrId === dep.crId ? (
                      <Loader2 className="w-4 h-4 animate-spin" />
                    ) : (
                      <Banknote className="w-4 h-4" />
                    )}
                    คืนเงินมัดจำ
                  </button>
                )}
              </div>
            ))}
          </div>
        </div>
      )}

      {/* Phase 5.5e — legacy_mirror panels (coupons / minibar / room moves).
          Hidden in print so the receipt itself stays clean. */}
      {invoiceData.cinNo && (
        <div className="no-print">
          <LegacyMirrorPanels cinNo={invoiceData.cinNo} />
        </div>
      )}

      {/* M1 task #36: record a payment against this folio. Reuses the same
          POST /api/checkins/:id/payments path as the checkout settle flow. */}
      <PaymentModal
        isOpen={showPayment}
        onClose={() => setShowPayment(false)}
        checkinId={invoiceData.checkInId}
        totalAmount={paySummary?.totalAmount ?? invoiceData.grandTotal}
        totalPaid={paySummary?.totalPaid ?? 0}
        onSuccess={() => {
          fetchPaySummary()
          fetchInvoice()
        }}
      />

      {/* Task #48: issue a coupon bound to this folio. The modal handles its
          own success state (shows a print-ready barcoded coupon); the issued
          row surfaces in the legacy panel above after the writeback + sync
          round-trip, so no local refetch is needed here. */}
      <IssueCouponModal
        isOpen={showIssueCoupon}
        onClose={() => setShowIssueCoupon(false)}
        forCinNo={invoiceData.cinNo}
        hotelName={hotelInfo.name}
        room={invoiceData.rooms[0]?.roomNumber}
        defaultValue={0}
        onSuccess={() => {}}
      />
    </div>
  )
}
