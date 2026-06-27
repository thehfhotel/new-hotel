'use client'

import { use, useCallback, useEffect, useRef, useState } from 'react'
import Link from 'next/link'
import { ArrowLeft, Printer, AlertCircle } from 'lucide-react'
import { useBranch } from '@/contexts/BranchContext'
import { useBranchFetch } from '@/lib/use-branch-fetch'
import { hotelInfoForBranch } from '@/lib/hotel-info'
import { mapInvoiceResponse, type InvoiceApiResponse } from '@/lib/invoice'
import { printRegion } from '@/lib/print'
import InvoiceTemplate from '@/components/documents/InvoiceTemplate'
import { V2PageHeader, V2Spinner } from '@/components/v2/primitives'
import type { InvoiceData } from '@/types/invoice'

/**
 * v2 tax invoice (ใบกำกับภาษี) — task #44. View + print a proper Thai tax
 * invoice for one check-in: room line + the folio's POS / other-charge
 * lines + the VAT breakdown (inclusive split, server-derived). Reuses the
 * canonical `InvoiceTemplate` (single source of truth for the document) and
 * the v2 print idiom (`.v2-print` region + `printRegion`, see
 * app/v2/rosters/page.tsx) so the whole UI chrome drops out on paper.
 *
 * Branch-aware: the invoice is read from the active branch's pool via
 * `useBranchFetch`, and the seller identity comes from
 * `hotelInfoForBranch`.
 */
export default function V2InvoicePage({
  params,
}: {
  params: Promise<{ id: string }>
}) {
  const { id } = use(params)
  const branchFetch = useBranchFetch()
  const { branch } = useBranch()
  const hotelInfo = hotelInfoForBranch(branch)

  const [invoice, setInvoice] = useState<InvoiceData | null>(null)
  const [loading, setLoading] = useState(true)
  const [error, setError] = useState<string | null>(null)
  const printRef = useRef<HTMLDivElement>(null)

  const fetchInvoice = useCallback(async () => {
    setLoading(true)
    setError(null)
    try {
      const res = await branchFetch(`/api/checkins/${id}/invoice`)
      const data: InvoiceApiResponse = await res.json()
      if (data.success && data.invoice) {
        setInvoice(mapInvoiceResponse(data.invoice))
      } else {
        setError(data.error || 'ไม่พบข้อมูลใบกำกับภาษี')
      }
    } catch (err) {
      setError(err instanceof Error ? err.message : 'เกิดข้อผิดพลาดในการโหลดข้อมูล')
    } finally {
      setLoading(false)
    }
  }, [id, branchFetch])

  useEffect(() => {
    fetchInvoice()
  }, [fetchInvoice])

  return (
    <div className="space-y-5">
      <div className="flex items-center justify-between gap-3 v2-no-print">
        <Link href="/v2/invoice" className="v2-btn v2-btn-ghost v2-btn-sm">
          <ArrowLeft size={16} /> กลับ
        </Link>
        <V2PageHeader eyebrow="เอกสารภาษี · Tax invoice" title="ใบกำกับภาษี" />
        <button
          onClick={() => printRegion(printRef.current)}
          className="v2-btn v2-btn-primary v2-btn-sm"
          disabled={!invoice}
          aria-label="พิมพ์ใบกำกับภาษี A4"
        >
          <Printer size={16} /> พิมพ์ A4
        </button>
      </div>

      {loading ? (
        <V2Spinner label="กำลังโหลดใบกำกับภาษี…" />
      ) : error ? (
        <div
          className="v2-card p-4 flex items-center gap-2 text-[14px]"
          style={{ color: '#b4232a' }}
        >
          <AlertCircle size={18} /> {error}
        </div>
      ) : invoice ? (
        <div ref={printRef} className="v2-print v2-card overflow-x-auto">
          {/* Reuse the canonical invoice document. `taxInvoice` flips the
              title to ใบกำกับภาษี / Tax Invoice; `showVat` surfaces the VAT
              split when there is VAT (the template hides it when vat=0, e.g.
              HF Hotel). */}
          <InvoiceTemplate checkInData={invoice} hotelInfo={hotelInfo} showVat taxInvoice />
        </div>
      ) : null}
    </div>
  )
}
