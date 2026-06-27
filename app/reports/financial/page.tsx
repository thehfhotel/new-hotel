'use client'

/**
 * Financial reports (task #55) — VAT / output-tax summary, sales-by-customer,
 * and a printable A4 period-income statement.
 *
 * Branch-aware via `useBranchFetch` (appends ?branch=X) + `useBranch` for the
 * printed hotel identity. Read-only: every figure comes from the report
 * endpoints in `routes/new_reports.rs`; this page never mutates state.
 *
 * Printing reuses the classic `.no-print` + `<style jsx global>` @page idiom
 * (see components/documents/InvoiceTemplate.tsx) so PrintButton's window.print()
 * yields a clean A4 sheet — controls are hidden, the statement tables remain.
 */

import { useCallback, useEffect, useState } from 'react'
import {
  AlertCircle,
  BarChart3,
  Calendar,
  Loader2,
  Percent,
  Receipt,
  RefreshCw,
  Users,
} from 'lucide-react'
import DatePicker from 'react-datepicker'
import 'react-datepicker/dist/react-datepicker.css'
import PrintButton from '@/components/ui/PrintButton'
import { useBranchFetch } from '@/lib/use-branch-fetch'
import { useBranch, BRANCH_LABELS } from '@/contexts/BranchContext'
import { hotelInfoForBranch } from '@/lib/hotel-info'
import {
  formatBaht,
  formatDateForApi,
  sumIncome,
} from '@/lib/reports-finance'
import type {
  CustomerSales,
  GroupBy,
  OccupancyResponse,
  RevenueDataPoint,
  VatPeriodRow,
  VatSummaryResponse,
} from '@/types/reports'

function getDefaultDateRange(): [Date, Date] {
  const end = new Date()
  const start = new Date()
  start.setMonth(start.getMonth() - 1)
  return [start, end]
}

function formatThaiDate(date: Date | null): string {
  if (!date) return '-'
  return date.toLocaleDateString('th-TH', { day: '2-digit', month: 'short', year: 'numeric' })
}

export default function FinancialReportsPage() {
  const branchFetch = useBranchFetch()
  const { branch } = useBranch()
  const hotelInfo = hotelInfoForBranch(branch)

  const [dateRange, setDateRange] = useState<[Date | null, Date | null]>(getDefaultDateRange)
  const [startDate, endDate] = dateRange
  const [groupBy, setGroupBy] = useState<GroupBy>('month')

  const [income, setIncome] = useState<RevenueDataPoint[]>([])
  const [occupancy, setOccupancy] = useState<OccupancyResponse | null>(null)
  const [vat, setVat] = useState<VatSummaryResponse | null>(null)
  const [sales, setSales] = useState<CustomerSales[]>([])

  const [loading, setLoading] = useState(true)
  const [error, setError] = useState<string | null>(null)

  const fetchAll = useCallback(async () => {
    if (!startDate || !endDate) return
    setLoading(true)
    setError(null)
    try {
      const from = formatDateForApi(startDate)
      const to = formatDateForApi(endDate)

      const [revRes, occRes, vatRes, salesRes] = await Promise.all([
        branchFetch(`/api/reports/revenue?from=${from}&to=${to}&groupBy=${groupBy}`),
        branchFetch(`/api/reports/occupancy?from=${from}&to=${to}`),
        branchFetch(`/api/reports/vat-summary?from=${from}&to=${to}&groupBy=${groupBy}`),
        branchFetch(`/api/reports/sales-by-customer?from=${from}&to=${to}&limit=100`),
      ])
      const [revJson, occJson, vatJson, salesJson] = await Promise.all([
        revRes.json(),
        occRes.json(),
        vatRes.json(),
        salesRes.json(),
      ])

      setIncome(revJson.success ? revJson.data ?? [] : [])
      setOccupancy(occJson.success ? occJson : null)
      setVat(vatJson.success ? vatJson : null)
      setSales(salesJson.success ? salesJson.data ?? [] : [])
    } catch (err) {
      setError(err instanceof Error ? err.message : 'เกิดข้อผิดพลาดในการโหลดข้อมูล')
    } finally {
      setLoading(false)
    }
  }, [startDate, endDate, groupBy, branchFetch])

  useEffect(() => {
    fetchAll()
  }, [fetchAll])

  const incomeTotals = sumIncome(income)
  const periodLabel = `${formatThaiDate(startDate)} – ${formatThaiDate(endDate)}`

  return (
    <div className="space-y-6">
      {/* Print rules — hide controls, format A4. Mirrors InvoiceTemplate. */}
      <style jsx global>{`
        @media print {
          @page {
            size: A4 portrait;
            margin: 14mm;
          }
          body {
            print-color-adjust: exact;
            -webkit-print-color-adjust: exact;
          }
          .no-print {
            display: none !important;
          }
          .fin-doc {
            box-shadow: none !important;
          }
          .fin-doc table {
            font-size: 11px;
          }
        }
      `}</style>

      {/* Header (screen only) */}
      <div className="flex items-center justify-between no-print">
        <div className="flex items-center gap-3">
          <Receipt className="w-8 h-8 text-red-600" />
          <div>
            <h1 className="text-2xl font-bold text-gray-900">รายงานการเงิน</h1>
            <p className="text-gray-500">Financial Reports · {BRANCH_LABELS[branch]}</p>
          </div>
        </div>
        <div className="flex items-center gap-2">
          <button
            onClick={fetchAll}
            disabled={loading}
            className="flex items-center space-x-2 px-4 py-2 border border-gray-300 text-gray-700 rounded-lg hover:bg-gray-100 disabled:opacity-50"
          >
            <RefreshCw className={`h-4 w-4 ${loading ? 'animate-spin text-red-500' : ''}`} />
            <span className="hidden sm:inline">รีเฟรช</span>
          </button>
          <PrintButton size="md" showPdfOption />
        </div>
      </div>

      {/* Filters (screen only) */}
      <div className="bg-white rounded-lg p-4 no-print">
        <div className="flex flex-wrap items-center gap-4">
          <div className="flex items-center gap-2">
            <span className="text-sm font-medium text-gray-700">ช่วงวันที่:</span>
            <div className="flex items-center border border-gray-300 rounded-lg overflow-hidden focus-within:ring-2 focus-within:ring-red-500 focus-within:border-red-500">
              <div className="flex items-center px-3 bg-gray-100 border-r border-gray-300 py-2">
                <Calendar className="h-4 w-4 text-gray-500" />
              </div>
              <DatePicker
                selectsRange
                startDate={startDate}
                endDate={endDate}
                onChange={(update) => setDateRange(update)}
                placeholderText="เลือกช่วงวันที่"
                className="w-48 px-3 py-2 border-0 focus:ring-0 text-sm focus:outline-hidden bg-white text-gray-800"
                dateFormat="dd/MM/yyyy"
                isClearable
              />
            </div>
          </div>

          <div className="flex items-center gap-2">
            <span className="text-sm font-medium text-gray-700">แบ่งตาม:</span>
            <div className="flex rounded-lg border border-gray-300 overflow-hidden">
              {(['day', 'week', 'month'] as GroupBy[]).map((g, i) => (
                <button
                  key={g}
                  onClick={() => setGroupBy(g)}
                  className={`px-4 py-2 text-sm font-medium transition-colors ${
                    i > 0 ? 'border-l border-gray-300' : ''
                  } ${
                    groupBy === g
                      ? 'bg-red-600 text-white'
                      : 'bg-white text-gray-700 hover:bg-gray-100'
                  }`}
                >
                  {g === 'day' ? 'วัน' : g === 'week' ? 'สัปดาห์' : 'เดือน'}
                </button>
              ))}
            </div>
          </div>
        </div>
      </div>

      {/* Error */}
      {error && (
        <div className="flex items-center gap-2 p-4 bg-red-50 border border-red-200 rounded-lg text-red-600 no-print">
          <AlertCircle className="w-5 h-5 shrink-0" />
          <span>{error}</span>
        </div>
      )}

      {loading ? (
        <div className="flex items-center justify-center h-64 no-print">
          <Loader2 className="w-8 h-8 animate-spin text-red-500" />
        </div>
      ) : (
        /* Printable statement document */
        <div className="fin-doc bg-white rounded-lg p-6 md:p-8 max-w-[210mm] mx-auto space-y-8 text-gray-900">
          {/* Document header */}
          <div className="border-b-2 border-gray-800 pb-4">
            <div className="flex items-start justify-between gap-4">
              <div>
                <h2 className="text-xl font-bold">{hotelInfo.name}</h2>
                <p className="text-sm text-gray-600">{hotelInfo.address}</p>
                <p className="text-sm text-gray-600">
                  เลขประจำตัวผู้เสียภาษี: {hotelInfo.taxId}
                </p>
              </div>
              <div className="text-right">
                <p className="text-lg font-semibold">สรุปรายได้ตามช่วงเวลา</p>
                <p className="text-sm text-gray-600">Period Income Statement</p>
                <p className="text-sm text-gray-700 mt-1">{periodLabel}</p>
              </div>
            </div>
          </div>

          {/* Section 1 — Income summary */}
          <section>
            <div className="flex items-center gap-2 mb-3">
              <BarChart3 className="w-5 h-5 text-red-600" />
              <h3 className="text-lg font-semibold">รายได้ (Income)</h3>
            </div>

            <div className="grid grid-cols-2 md:grid-cols-4 gap-3 mb-4">
              <KpiBox label="รายได้รวม" hint="Total Revenue" value={formatBaht(incomeTotals.revenue)} />
              <KpiBox label="จำนวนการเข้าพัก" hint="Check-outs" value={`${incomeTotals.bookings}`} />
              <KpiBox
                label="อัตราเข้าพัก"
                hint="Occupancy"
                value={occupancy ? `${occupancy.occupancyRate.toFixed(1)}%` : '-'}
              />
              <KpiBox label="ราคาเฉลี่ย/คืน" hint="ADR" value={occupancy ? formatBaht(occupancy.adr) : '-'} />
            </div>

            <ReportTable
              head={['ช่วงเวลา', 'จำนวน', 'รายได้']}
              empty={income.length === 0}
              rows={income.map((r: RevenueDataPoint) => [
                r.period,
                `${r.bookings}`,
                formatBaht(r.revenue),
              ])}
              alignRight={[false, true, true]}
              footer={['รวม', `${incomeTotals.bookings}`, formatBaht(incomeTotals.revenue)]}
            />
          </section>

          {/* Section 2 — VAT / output tax */}
          <section>
            <div className="flex items-center gap-2 mb-3">
              <Percent className="w-5 h-5 text-red-600" />
              <h3 className="text-lg font-semibold">ภาษีขาย (VAT / Output Tax)</h3>
              <span className="ml-1 text-xs px-2 py-0.5 rounded-full bg-gray-100 text-gray-700">
                VAT {vat ? vat.vatPercent : 0}%
              </span>
            </div>

            <div className="grid grid-cols-1 md:grid-cols-3 gap-3 mb-4">
              <KpiBox label="ยอดรวม (รวมภาษี)" hint="Gross" value={formatBaht(vat?.gross ?? 0)} />
              <KpiBox label="มูลค่าก่อนภาษี" hint="Before VAT" value={formatBaht(vat?.beforeVat ?? 0)} />
              <KpiBox label="ภาษีขาย" hint="Output VAT" value={formatBaht(vat?.vat ?? 0)} />
            </div>

            <ReportTable
              head={['ช่วงเวลา', 'ยอดรวม', 'ก่อนภาษี', 'ภาษีขาย']}
              empty={!vat || vat.data.length === 0}
              rows={(vat?.data ?? []).map((r: VatPeriodRow) => [
                r.period,
                formatBaht(r.gross),
                formatBaht(r.beforeVat),
                formatBaht(r.vat),
              ])}
              alignRight={[false, true, true, true]}
              footer={[
                'รวม',
                formatBaht(vat?.gross ?? 0),
                formatBaht(vat?.beforeVat ?? 0),
                formatBaht(vat?.vat ?? 0),
              ]}
            />
            {vat && vat.vatPercent === 0 && (
              <p className="text-xs text-gray-500 mt-2">
                สาขานี้ตั้งค่า VAT = 0% — มูลค่าก่อนภาษีเท่ากับยอดรวมและไม่มีภาษีขาย
              </p>
            )}
          </section>

          {/* Section 3 — Sales by customer */}
          <section>
            <div className="flex items-center gap-2 mb-3">
              <Users className="w-5 h-5 text-red-600" />
              <h3 className="text-lg font-semibold">ยอดขายตามลูกค้า (Sales by Customer)</h3>
            </div>

            <ReportTable
              head={['#', 'ลูกค้า', 'โทรศัพท์', 'ครั้ง', 'รายได้']}
              empty={sales.length === 0}
              rows={sales.map((c: CustomerSales, idx) => [
                `${idx + 1}`,
                c.customerName,
                c.phone ?? '-',
                `${c.checkins}`,
                formatBaht(c.revenue),
              ])}
              alignRight={[false, false, false, true, true]}
              footer={[
                'รวม',
                '',
                '',
                `${sales.reduce((a, c) => a + c.checkins, 0)}`,
                formatBaht(sales.reduce((a, c) => a + c.revenue, 0)),
              ]}
            />
          </section>

          <p className="text-xs text-gray-400 pt-2">
            ออกรายงานเมื่อ {new Date().toLocaleString('th-TH', { timeZone: 'UTC' })}
          </p>
        </div>
      )}
    </div>
  )
}

function KpiBox({ label, hint, value }: { label: string; hint: string; value: string }) {
  return (
    <div className="border border-gray-200 rounded-lg p-3">
      <p className="text-xs text-gray-500">{label}</p>
      <p className="text-xl font-bold text-gray-900">{value}</p>
      <p className="text-[10px] text-gray-400">{hint}</p>
    </div>
  )
}

function ReportTable({
  head,
  rows,
  footer,
  alignRight,
  empty,
}: {
  head: string[]
  rows: string[][]
  footer?: string[]
  alignRight: boolean[]
  empty: boolean
}) {
  if (empty) {
    return (
      <div className="border border-dashed border-gray-300 rounded-lg p-6 text-center text-gray-500 text-sm">
        ไม่มีข้อมูลในช่วงเวลาที่เลือก
      </div>
    )
  }
  return (
    <div className="overflow-x-auto">
      <table className="w-full text-sm border-collapse">
        <thead>
          <tr className="border-b-2 border-gray-300 bg-gray-50">
            {head.map((h, i) => (
              <th
                key={i}
                className={`px-3 py-2 font-semibold text-gray-700 ${
                  alignRight[i] ? 'text-right' : 'text-left'
                }`}
              >
                {h}
              </th>
            ))}
          </tr>
        </thead>
        <tbody>
          {rows.map((row, ri) => (
            <tr key={ri} className="border-b border-gray-100">
              {row.map((cell, ci) => (
                <td
                  key={ci}
                  className={`px-3 py-1.5 text-gray-800 ${alignRight[ci] ? 'text-right tabular-nums' : 'text-left'}`}
                >
                  {cell}
                </td>
              ))}
            </tr>
          ))}
        </tbody>
        {footer && (
          <tfoot>
            <tr className="border-t-2 border-gray-400 font-semibold">
              {footer.map((cell, ci) => (
                <td
                  key={ci}
                  className={`px-3 py-2 text-gray-900 ${alignRight[ci] ? 'text-right tabular-nums' : 'text-left'}`}
                >
                  {cell}
                </td>
              ))}
            </tr>
          </tfoot>
        )}
      </table>
    </div>
  )
}
