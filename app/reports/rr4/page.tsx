'use client'

/**
 * Track G8 — RR.4 Thai immigration foreign-guest export UI.
 *
 * The receptionist picks (from, to, site, format), clicks Export,
 * and the browser downloads the generated xlsx / csv attachment.
 *
 * Per `docs/coexistence/audit-2026-05-13.md` §T4 CRIT-2, this is a
 * legal-critical workflow — the regulator will re-request prior
 * filings, so every export attempt is logged to `ht_rr4_exports`
 * with a SHA-256 of the bytes for non-repudiation.
 */

import { useCallback, useState } from 'react'
import { AlertCircle, Download, FileSpreadsheet, Loader2 } from 'lucide-react'
import DatePicker from 'react-datepicker'
import 'react-datepicker/dist/react-datepicker.css'
import { useBranchFetch } from '@/lib/use-branch-fetch'

type Rr4Format = 'xlsx' | 'csv'

function formatDateForApi(date: Date): string {
  const year = date.getFullYear()
  const month = String(date.getMonth() + 1).padStart(2, '0')
  const day = String(date.getDate()).padStart(2, '0')
  return `${year}-${month}-${day}`
}

function getDefaultDateRange(): [Date, Date] {
  const end = new Date()
  const start = new Date()
  start.setMonth(start.getMonth() - 1)
  return [start, end]
}

/**
 * Read the `filename="X"` token out of a Content-Disposition header.
 * Falls back to the generated default so the download still works
 * when the header is stripped by a misbehaving proxy.
 */
function extractFilename(headerValue: string | null, fallback: string): string {
  if (!headerValue) return fallback
  const match = /filename="?([^";]+)"?/i.exec(headerValue)
  return match ? match[1] : fallback
}

export default function Rr4ExportPage() {
  const branchFetch = useBranchFetch()

  const [dateRange, setDateRange] = useState<[Date | null, Date | null]>(getDefaultDateRange)
  const [startDate, endDate] = dateRange
  const [site, setSite] = useState<string>('hfhotel')
  const [format, setFormat] = useState<Rr4Format>('xlsx')
  const [isExporting, setIsExporting] = useState(false)
  const [error, setError] = useState<string | null>(null)
  const [lastExport, setLastExport] = useState<{
    filename: string
    rowCount: number | null
    fileHash: string | null
  } | null>(null)

  const handleExport = useCallback(async () => {
    if (!startDate || !endDate) {
      setError('กรุณาเลือกช่วงวันที่')
      return
    }
    setIsExporting(true)
    setError(null)

    try {
      const from = formatDateForApi(startDate)
      const to = formatDateForApi(endDate)
      const params = new URLSearchParams({ from, to, site, format })
      const response = await branchFetch(`/api/new/reports/rr4?${params.toString()}`)

      if (!response.ok) {
        const text = await response.text()
        throw new Error(text || `Export failed: HTTP ${response.status}`)
      }

      const blob = await response.blob()
      const fallback = `RR4_${site}_${from.replaceAll('-', '')}_${to.replaceAll('-', '')}.${format}`
      const filename = extractFilename(response.headers.get('content-disposition'), fallback)
      const rowCountHeader = response.headers.get('x-rr4-row-count')
      const fileHashHeader = response.headers.get('x-rr4-file-hash')

      const url = URL.createObjectURL(blob)
      const link = document.createElement('a')
      link.href = url
      link.download = filename
      document.body.appendChild(link)
      link.click()
      document.body.removeChild(link)
      URL.revokeObjectURL(url)

      setLastExport({
        filename,
        rowCount: rowCountHeader ? Number(rowCountHeader) : null,
        fileHash: fileHashHeader,
      })
    } catch (err) {
      setError(err instanceof Error ? err.message : 'เกิดข้อผิดพลาดในการส่งออก')
    } finally {
      setIsExporting(false)
    }
  }, [startDate, endDate, site, format, branchFetch])

  return (
    <div className="space-y-6">
      <div className="flex items-center gap-3">
        <FileSpreadsheet className="w-8 h-8 text-red-600" />
        <div>
          <h1 className="text-2xl font-bold text-gray-900">รายงาน รร.4 / ตม.30</h1>
          <p className="text-gray-500">RR.4 — Thai immigration foreign-guest export</p>
        </div>
      </div>

      <div className="bg-white rounded-lg p-6 space-y-4">
        <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
          <div>
            <label className="block text-sm font-medium text-gray-700 mb-1">
              ช่วงวันที่เข้าพัก
            </label>
            <DatePicker
              selectsRange
              startDate={startDate}
              endDate={endDate}
              onChange={(update) => setDateRange(update)}
              placeholderText="เลือกช่วงวันที่"
              className="w-full px-3 py-2 border border-gray-300 rounded-lg text-sm focus:outline-none focus:ring-2 focus:ring-red-500"
              dateFormat="dd/MM/yyyy"
              isClearable
            />
          </div>

          <div>
            <label htmlFor="rr4-site" className="block text-sm font-medium text-gray-700 mb-1">
              สาขา (site)
            </label>
            <select
              id="rr4-site"
              value={site}
              onChange={(e) => setSite(e.target.value)}
              className="w-full px-3 py-2 border border-gray-300 rounded-lg text-sm focus:outline-none focus:ring-2 focus:ring-red-500 bg-white"
            >
              <option value="hfhotel">HF Hotel</option>
              <option value="hfville">HF Ville</option>
            </select>
          </div>

          <div>
            <label htmlFor="rr4-format" className="block text-sm font-medium text-gray-700 mb-1">
              รูปแบบไฟล์
            </label>
            <select
              id="rr4-format"
              value={format}
              onChange={(e) => setFormat(e.target.value as Rr4Format)}
              className="w-full px-3 py-2 border border-gray-300 rounded-lg text-sm focus:outline-none focus:ring-2 focus:ring-red-500 bg-white"
            >
              <option value="xlsx">Excel (.xlsx) — ส่งทางการ</option>
              <option value="csv">CSV (.csv) — สำหรับนักพัฒนา</option>
            </select>
          </div>
        </div>

        <div className="pt-2">
          <button
            type="button"
            onClick={handleExport}
            disabled={isExporting || !startDate || !endDate}
            className="inline-flex items-center gap-2 px-4 py-2 bg-red-600 text-white rounded-lg hover:bg-red-700 disabled:opacity-50 disabled:cursor-not-allowed transition-colors"
          >
            {isExporting ? (
              <Loader2 className="w-4 h-4 animate-spin" />
            ) : (
              <Download className="w-4 h-4" />
            )}
            {isExporting ? 'กำลังส่งออก...' : 'ส่งออกรายงาน'}
          </button>
        </div>
      </div>

      {error && (
        <div className="flex items-start gap-2 p-4 bg-red-50 border border-red-200 rounded-lg text-red-700">
          <AlertCircle className="w-5 h-5 shrink-0 mt-0.5" />
          <span className="text-sm">{error}</span>
        </div>
      )}

      {lastExport && !error && (
        <div className="bg-emerald-50 border border-emerald-200 rounded-lg p-4 text-sm text-emerald-800 space-y-1">
          <div>
            <span className="font-medium">ดาวน์โหลดแล้ว:</span> {lastExport.filename}
          </div>
          {lastExport.rowCount !== null && (
            <div>
              <span className="font-medium">จำนวนแถวต่างชาติ:</span> {lastExport.rowCount}
            </div>
          )}
          {lastExport.fileHash && (
            <div className="break-all">
              <span className="font-medium">SHA-256:</span>{' '}
              <code className="text-xs">{lastExport.fileHash}</code>
            </div>
          )}
        </div>
      )}

      <div className="bg-gray-50 border border-gray-200 rounded-lg p-4 text-xs text-gray-600 space-y-1">
        <div className="font-medium text-gray-700">หมายเหตุ</div>
        <ul className="list-disc list-inside space-y-0.5">
          <li>รายงานนี้กรองเฉพาะผู้เข้าพักต่างชาติ (สัญชาติไม่ใช่ไทย)</li>
          <li>ครอบคลุมเฉพาะผู้ที่มี <em>วันเข้าพัก</em> อยู่ในช่วงที่เลือก (รวมขอบเขต)</li>
          <li>ยกเว้นรายการที่ถูกยกเลิก (<code>cin_status = &apos;cancelled&apos;</code>)</li>
          <li>ทุกครั้งที่ส่งออกจะถูกบันทึกใน <code>ht_rr4_exports</code> เพื่อการตรวจสอบ</li>
        </ul>
      </div>
    </div>
  )
}
