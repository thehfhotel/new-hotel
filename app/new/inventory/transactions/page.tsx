'use client'

import { useState, useEffect, useCallback, useRef } from 'react'
import {
  FileText,
  Search,
  X,
  Loader2,
  AlertCircle,
  Filter,
  Calendar,
  Printer,
  Download,
  Plus,
  Minus,
  RefreshCw,
  ArrowLeftRight,
} from 'lucide-react'
import {
  InventoryTransaction,
  TransactionType,
  TRANSACTION_TYPES,
} from '@/types/inventory'

export default function TransactionHistoryPage() {
  const [transactions, setTransactions] = useState<InventoryTransaction[]>([])
  const [loading, setLoading] = useState(true)
  const [error, setError] = useState<string | null>(null)
  const [searchQuery, setSearchQuery] = useState('')
  const [debouncedSearch, setDebouncedSearch] = useState('')
  const [typeFilter, setTypeFilter] = useState<TransactionType | ''>('')
  const [dateFrom, setDateFrom] = useState('')
  const [dateTo, setDateTo] = useState('')

  const printRef = useRef<HTMLDivElement>(null)

  // Debounce search input
  useEffect(() => {
    const timer = setTimeout(() => {
      setDebouncedSearch(searchQuery)
    }, 300)

    return () => clearTimeout(timer)
  }, [searchQuery])

  // Fetch transactions
  const fetchTransactions = useCallback(async () => {
    setLoading(true)
    setError(null)
    try {
      const params = new URLSearchParams()

      if (debouncedSearch) {
        params.append('search', debouncedSearch)
      }

      if (typeFilter) {
        params.append('type', typeFilter)
      }

      if (dateFrom) {
        params.append('dateFrom', dateFrom)
      }

      if (dateTo) {
        params.append('dateTo', dateTo)
      }

      const response = await fetch(`/api/new/inventory/transactions?${params}`)
      if (!response.ok) {
        throw new Error('Failed to fetch transactions')
      }

      const data = await response.json()
      if (data.success) {
        setTransactions(data.data || [])
      } else {
        throw new Error(data.message || 'Failed to fetch transactions')
      }
    } catch (err) {
      setError(err instanceof Error ? err.message : 'เกิดข้อผิดพลาดในการโหลดข้อมูล')
    } finally {
      setLoading(false)
    }
  }, [debouncedSearch, typeFilter, dateFrom, dateTo])

  useEffect(() => {
    fetchTransactions()
  }, [fetchTransactions])

  // Get transaction type label and color
  const getTransactionTypeInfo = (type: TransactionType) => {
    const found = TRANSACTION_TYPES.find((t) => t.value === type)
    let icon: React.ReactNode
    let color: string

    switch (type) {
      case 'IN':
        icon = <Plus className="w-4 h-4" />
        color = 'text-emerald-400 bg-emerald-500/10'
        break
      case 'OUT':
        icon = <Minus className="w-4 h-4" />
        color = 'text-red-400 bg-red-500/10'
        break
      case 'ADJUST':
        icon = <RefreshCw className="w-4 h-4" />
        color = 'text-red-400 bg-red-500/10'
        break
      case 'MOVE':
        icon = <ArrowLeftRight className="w-4 h-4" />
        color = 'text-violet-400 bg-violet-500/10'
        break
      default:
        icon = <FileText className="w-4 h-4" />
        color = 'text-zinc-400 bg-zinc-800'
    }

    return {
      label: found?.labelTh || type,
      icon,
      color,
    }
  }

  // Format date
  const formatDate = (dateString: string) => {
    const date = new Date(dateString)
    return date.toLocaleDateString('th-TH', {
      day: 'numeric',
      month: 'short',
      year: 'numeric',
      hour: '2-digit',
      minute: '2-digit',
      timeZone: 'UTC',
    })
  }

  // Handle print
  const handlePrint = () => {
    const printContent = printRef.current
    if (!printContent) return

    const printWindow = window.open('', '_blank')
    if (!printWindow) return

    printWindow.document.write(`
      <!DOCTYPE html>
      <html>
        <head>
          <title>ประวัติการเคลื่อนไหวสินค้าคงคลัง</title>
          <style>
            body { font-family: sans-serif; padding: 20px; }
            h1 { font-size: 1.5rem; margin-bottom: 1rem; }
            table { width: 100%; border-collapse: collapse; }
            th, td { padding: 8px; text-align: left; border-bottom: 1px solid #ddd; }
            th { background-color: #f5f5f5; font-weight: 600; }
            .text-right { text-align: right; }
            .text-center { text-align: center; }
            .type-IN { color: #16a34a; }
            .type-OUT { color: #dc2626; }
            .type-ADJUST { color: #2563eb; }
            .type-MOVE { color: #9333ea; }
            .print-date { font-size: 0.875rem; color: #666; margin-bottom: 1rem; }
            @media print {
              body { padding: 0; }
            }
          </style>
        </head>
        <body>
          <h1>ประวัติการเคลื่อนไหวสินค้าคงคลัง</h1>
          <p class="print-date">พิมพ์เมื่อ: ${new Date().toLocaleString('th-TH')}</p>
          <table>
            <thead>
              <tr>
                <th>วันที่</th>
                <th>ประเภท</th>
                <th>สินค้า</th>
                <th class="text-right">จำนวน</th>
                <th>ห้อง</th>
                <th>หมายเหตุ</th>
                <th>ผู้ดำเนินการ</th>
              </tr>
            </thead>
            <tbody>
              ${transactions.map((tx) => {
                const typeInfo = getTransactionTypeInfo(tx.transactionType)
                return `
                  <tr>
                    <td>${formatDate(tx.createdAt)}</td>
                    <td class="type-${tx.transactionType}">${typeInfo.label}</td>
                    <td>${tx.itemName} (${tx.itemCode})</td>
                    <td class="text-right type-${tx.transactionType}">${tx.transactionType === 'IN' ? '+' : tx.transactionType === 'OUT' ? '-' : ''}${tx.quantity}</td>
                    <td>${tx.roomNumber || '-'}</td>
                    <td>${tx.notes || '-'}</td>
                    <td>${tx.createdBy || '-'}</td>
                  </tr>
                `
              }).join('')}
            </tbody>
          </table>
        </body>
      </html>
    `)
    printWindow.document.close()
    printWindow.print()
  }

  // Clear filters
  const clearFilters = () => {
    setSearchQuery('')
    setTypeFilter('')
    setDateFrom('')
    setDateTo('')
  }

  const hasFilters = searchQuery || typeFilter || dateFrom || dateTo

  return (
    <div className="space-y-6">
      {/* Page Header */}
      <div className="flex items-center justify-between">
        <div className="flex items-center gap-3">
          <FileText className="w-8 h-8 text-red-400" />
          <div>
            <h1 className="text-2xl font-bold text-zinc-200">ประวัติการเคลื่อนไหว</h1>
            <p className="text-zinc-400">
              รายการทั้งหมด {transactions.length} รายการ
            </p>
          </div>
        </div>

        {/* Print Button */}
        <button
          onClick={handlePrint}
          disabled={transactions.length === 0}
          className="flex items-center gap-2 px-4 py-2 bg-zinc-700 text-white rounded-lg hover:bg-zinc-600 transition-colors disabled:opacity-50"
        >
          <Printer className="w-5 h-5" />
          พิมพ์
        </button>
      </div>

      {/* Search and Filter Bar */}
      <div className="bg-zinc-900 rounded-lg border border-zinc-800 p-4">
        <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-5 gap-4">
          {/* Search */}
          <div className="relative lg:col-span-2">
            <Search className="absolute left-3 top-1/2 -translate-y-1/2 w-5 h-5 text-zinc-500" />
            <input
              type="text"
              value={searchQuery}
              onChange={(e) => setSearchQuery(e.target.value)}
              placeholder="ค้นหาด้วยรหัส หรือ ชื่อสินค้า..."
              className="w-full pl-10 pr-4 py-2 bg-zinc-800 border border-zinc-700 text-zinc-200 rounded-lg focus:ring-2 focus:ring-red-500 focus:border-red-500 outline-none transition-colors"
            />
            {searchQuery && (
              <button
                onClick={() => setSearchQuery('')}
                className="absolute right-3 top-1/2 -translate-y-1/2 text-zinc-500 hover:text-zinc-400"
                aria-label="ล้างการค้นหา"
              >
                <X className="w-4 h-4" />
              </button>
            )}
          </div>

          {/* Type Filter */}
          <div>
            <select
              value={typeFilter}
              onChange={(e) => setTypeFilter(e.target.value as TransactionType | '')}
              className="w-full px-3 py-2 bg-zinc-800 border border-zinc-700 text-zinc-200 rounded-lg focus:ring-2 focus:ring-red-500 focus:border-red-500 outline-none"
            >
              <option value="">ทุกประเภท</option>
              {TRANSACTION_TYPES.map((type) => (
                <option key={type.value} value={type.value}>
                  {type.labelTh}
                </option>
              ))}
            </select>
          </div>

          {/* Date From */}
          <div className="relative">
            <Calendar className="absolute left-3 top-1/2 -translate-y-1/2 w-5 h-5 text-zinc-500" />
            <input
              type="date"
              value={dateFrom}
              onChange={(e) => setDateFrom(e.target.value)}
              className="w-full pl-10 pr-4 py-2 bg-zinc-800 border border-zinc-700 text-zinc-200 rounded-lg focus:ring-2 focus:ring-red-500 focus:border-red-500 outline-none"
              placeholder="จากวันที่"
            />
          </div>

          {/* Date To */}
          <div className="relative">
            <Calendar className="absolute left-3 top-1/2 -translate-y-1/2 w-5 h-5 text-zinc-500" />
            <input
              type="date"
              value={dateTo}
              onChange={(e) => setDateTo(e.target.value)}
              className="w-full pl-10 pr-4 py-2 bg-zinc-800 border border-zinc-700 text-zinc-200 rounded-lg focus:ring-2 focus:ring-red-500 focus:border-red-500 outline-none"
              placeholder="ถึงวันที่"
            />
          </div>
        </div>

        {/* Clear Filters */}
        {hasFilters && (
          <div className="mt-4 flex items-center justify-between">
            <p className="text-sm text-zinc-400">
              กำลังแสดงผลตามตัวกรอง
            </p>
            <button
              onClick={clearFilters}
              className="text-sm text-red-400 hover:text-red-300"
            >
              ล้างตัวกรองทั้งหมด
            </button>
          </div>
        )}
      </div>

      {/* Error Message */}
      {error && (
        <div className="flex items-center gap-2 p-4 bg-red-950/50 border border-red-900/50 rounded-lg text-red-400">
          <AlertCircle className="w-5 h-5 flex-shrink-0" />
          <span>{error}</span>
        </div>
      )}

      {/* Transactions Table */}
      <div className="bg-zinc-900 rounded-lg border border-zinc-800 overflow-hidden" ref={printRef}>
        {loading ? (
          <div className="flex items-center justify-center py-12">
            <Loader2 className="w-6 h-6 animate-spin text-red-500" />
            <span className="ml-2 text-zinc-400">กำลังโหลดข้อมูล...</span>
          </div>
        ) : transactions.length === 0 ? (
          <div className="text-center py-12 text-zinc-500">
            <FileText className="w-12 h-12 text-zinc-600 mx-auto mb-3" />
            <p>ไม่พบรายการเคลื่อนไหว</p>
          </div>
        ) : (
          <div className="overflow-x-auto">
            <table className="w-full">
              <thead className="bg-zinc-800 border-b border-zinc-800">
                <tr>
                  <th className="px-4 py-3 text-left text-sm font-medium text-zinc-400">วันที่</th>
                  <th className="px-4 py-3 text-left text-sm font-medium text-zinc-400">ประเภท</th>
                  <th className="px-4 py-3 text-left text-sm font-medium text-zinc-400">สินค้า</th>
                  <th className="px-4 py-3 text-right text-sm font-medium text-zinc-400">จำนวน</th>
                  <th className="px-4 py-3 text-center text-sm font-medium text-zinc-400">สต็อก</th>
                  <th className="px-4 py-3 text-left text-sm font-medium text-zinc-400">ห้อง</th>
                  <th className="px-4 py-3 text-left text-sm font-medium text-zinc-400">หมายเหตุ</th>
                  <th className="px-4 py-3 text-left text-sm font-medium text-zinc-400">ผู้ดำเนินการ</th>
                </tr>
              </thead>
              <tbody className="divide-y divide-zinc-800">
                {transactions.map((tx) => {
                  const typeInfo = getTransactionTypeInfo(tx.transactionType)
                  return (
                    <tr key={tx.id} className="hover:bg-zinc-800">
                      <td className="px-4 py-3 text-sm text-zinc-400">
                        {formatDate(tx.createdAt)}
                      </td>
                      <td className="px-4 py-3">
                        <span className={`inline-flex items-center gap-1 px-2 py-1 rounded text-xs font-medium ${typeInfo.color}`}>
                          {typeInfo.icon}
                          {typeInfo.label}
                        </span>
                      </td>
                      <td className="px-4 py-3">
                        <div className="font-medium text-zinc-200">{tx.itemName}</div>
                        <div className="text-xs text-zinc-500">{tx.itemCode}</div>
                      </td>
                      <td className="px-4 py-3 text-right">
                        <span className={`font-bold text-lg ${
                          tx.transactionType === 'IN' ? 'text-emerald-400' :
                          tx.transactionType === 'OUT' ? 'text-red-400' :
                          'text-red-400'
                        }`}>
                          {tx.transactionType === 'IN' ? '+' : tx.transactionType === 'OUT' ? '-' : ''}
                          {tx.quantity}
                        </span>
                      </td>
                      <td className="px-4 py-3 text-center text-sm">
                        <span className="text-zinc-500">{tx.previousStock}</span>
                        <span className="mx-1 text-zinc-600">→</span>
                        <span className="font-medium text-zinc-200">{tx.newStock}</span>
                      </td>
                      <td className="px-4 py-3 text-sm text-zinc-400">
                        {tx.roomNumber ? `ห้อง ${tx.roomNumber}` : '-'}
                      </td>
                      <td className="px-4 py-3 text-sm text-zinc-400 max-w-xs truncate">
                        {tx.notes || '-'}
                      </td>
                      <td className="px-4 py-3 text-sm text-zinc-400">
                        {tx.createdBy || '-'}
                      </td>
                    </tr>
                  )
                })}
              </tbody>
            </table>
          </div>
        )}
      </div>
    </div>
  )
}
