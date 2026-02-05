'use client'

import { useState, useEffect, useCallback } from 'react'
import Link from 'next/link'
import {
  Package,
  AlertTriangle,
  Layers,
  Plus,
  RefreshCw,
  FileText,
  Loader2,
  AlertCircle,
  ArrowRight,
  TrendingDown,
  Clock,
} from 'lucide-react'
import {
  InventoryItem,
  InventoryTransaction,
  INVENTORY_CATEGORIES,
  TRANSACTION_TYPES,
  getStockStatus,
  getStockStatusColor,
  getStockStatusLabel,
} from '@/types/inventory'
import StockAdjustmentModal from '@/components/modals/StockAdjustmentModal'

interface DashboardStats {
  totalItems: number
  lowStockCount: number
  categoriesCount: number
}

export default function InventoryDashboardPage() {
  const [stats, setStats] = useState<DashboardStats>({
    totalItems: 0,
    lowStockCount: 0,
    categoriesCount: 0,
  })
  const [lowStockItems, setLowStockItems] = useState<InventoryItem[]>([])
  const [recentTransactions, setRecentTransactions] = useState<InventoryTransaction[]>([])
  const [loading, setLoading] = useState(true)
  const [error, setError] = useState<string | null>(null)

  // Modal states
  const [showAdjustmentModal, setShowAdjustmentModal] = useState(false)
  const [selectedItem, setSelectedItem] = useState<InventoryItem | null>(null)

  const fetchDashboardData = useCallback(async () => {
    setLoading(true)
    setError(null)
    try {
      // Fetch all data in parallel
      const [statsRes, lowStockRes, transactionsRes] = await Promise.all([
        fetch('/api/new/inventory/stats'),
        fetch('/api/new/inventory/items?lowStock=true&limit=5'),
        fetch('/api/new/inventory/transactions?limit=10'),
      ])

      const [statsData, lowStockData, transactionsData] = await Promise.all([
        statsRes.json(),
        lowStockRes.json(),
        transactionsRes.json(),
      ])

      if (statsData.success) {
        setStats(statsData.data)
      }
      if (lowStockData.success) {
        setLowStockItems(lowStockData.data || [])
      }
      if (transactionsData.success) {
        setRecentTransactions(transactionsData.data || [])
      }
    } catch (err) {
      setError(err instanceof Error ? err.message : 'เกิดข้อผิดพลาดในการโหลดข้อมูล')
    } finally {
      setLoading(false)
    }
  }, [])

  useEffect(() => {
    fetchDashboardData()
  }, [fetchDashboardData])

  const handleAdjustStock = (item: InventoryItem) => {
    setSelectedItem(item)
    setShowAdjustmentModal(true)
  }

  const handleAdjustmentSuccess = () => {
    fetchDashboardData()
  }

  const getTransactionTypeLabel = (type: string) => {
    const found = TRANSACTION_TYPES.find((t) => t.value === type)
    return found ? found.labelTh : type
  }

  const getTransactionTypeColor = (type: string) => {
    switch (type) {
      case 'IN':
        return 'text-green-600 bg-green-100'
      case 'OUT':
        return 'text-red-600 bg-red-100'
      case 'ADJUST':
        return 'text-blue-600 bg-blue-100'
      case 'MOVE':
        return 'text-purple-600 bg-purple-100'
      default:
        return 'text-gray-600 bg-gray-100'
    }
  }

  const formatDate = (dateString: string) => {
    const date = new Date(dateString)
    return date.toLocaleDateString('th-TH', {
      day: 'numeric',
      month: 'short',
      hour: '2-digit',
      minute: '2-digit',
      timeZone: 'UTC',
    })
  }

  return (
    <div className="space-y-6">
      {/* Page Header */}
      <div className="flex items-center justify-between">
        <div className="flex items-center gap-3">
          <Package className="w-8 h-8 text-blue-600" />
          <div>
            <h1 className="text-2xl font-bold text-gray-800">ระบบจัดการสินค้าคงคลัง</h1>
            <p className="text-gray-600">Inventory Management</p>
          </div>
        </div>
      </div>

      {/* Error Message */}
      {error && (
        <div className="flex items-center gap-2 p-4 bg-red-50 border border-red-200 rounded-lg text-red-700">
          <AlertCircle className="w-5 h-5 flex-shrink-0" />
          <span>{error}</span>
        </div>
      )}

      {/* Stats Cards */}
      <div className="grid grid-cols-1 md:grid-cols-3 gap-4">
        <Link href="/new/inventory/items" className="block">
          <div className="bg-white rounded-lg shadow p-6 hover:shadow-md transition-shadow">
            <div className="flex items-center justify-between">
              <div>
                <p className="text-sm text-gray-600">สินค้าทั้งหมด</p>
                <p className="text-3xl font-bold text-gray-800">
                  {loading ? '-' : stats.totalItems}
                </p>
              </div>
              <div className="w-12 h-12 bg-blue-100 rounded-full flex items-center justify-center">
                <Package className="w-6 h-6 text-blue-600" />
              </div>
            </div>
          </div>
        </Link>

        <Link href="/new/inventory/items?filter=low" className="block">
          <div className="bg-white rounded-lg shadow p-6 hover:shadow-md transition-shadow">
            <div className="flex items-center justify-between">
              <div>
                <p className="text-sm text-gray-600">สินค้าใกล้หมด</p>
                <p className="text-3xl font-bold text-orange-600">
                  {loading ? '-' : stats.lowStockCount}
                </p>
              </div>
              <div className="w-12 h-12 bg-orange-100 rounded-full flex items-center justify-center">
                <AlertTriangle className="w-6 h-6 text-orange-600" />
              </div>
            </div>
          </div>
        </Link>

        <div className="bg-white rounded-lg shadow p-6">
          <div className="flex items-center justify-between">
            <div>
              <p className="text-sm text-gray-600">หมวดหมู่</p>
              <p className="text-3xl font-bold text-gray-800">
                {loading ? '-' : stats.categoriesCount}
              </p>
            </div>
            <div className="w-12 h-12 bg-purple-100 rounded-full flex items-center justify-center">
              <Layers className="w-6 h-6 text-purple-600" />
            </div>
          </div>
        </div>
      </div>

      {/* Quick Actions */}
      <div className="bg-white rounded-lg shadow p-4">
        <h2 className="text-lg font-semibold text-gray-800 mb-4">การดำเนินการด่วน</h2>
        <div className="flex flex-wrap gap-3">
          <Link
            href="/new/inventory/items?mode=add"
            className="flex items-center gap-2 px-4 py-2 bg-blue-600 text-white rounded-lg hover:bg-blue-700 transition-colors"
          >
            <Plus className="w-4 h-4" />
            เพิ่มสินค้าใหม่
          </Link>
          <button
            onClick={() => {
              setSelectedItem(null)
              setShowAdjustmentModal(true)
            }}
            className="flex items-center gap-2 px-4 py-2 bg-green-600 text-white rounded-lg hover:bg-green-700 transition-colors"
          >
            <RefreshCw className="w-4 h-4" />
            ปรับสต็อก
          </button>
          <Link
            href="/new/inventory/transactions"
            className="flex items-center gap-2 px-4 py-2 bg-gray-600 text-white rounded-lg hover:bg-gray-700 transition-colors"
          >
            <FileText className="w-4 h-4" />
            ดูประวัติการเคลื่อนไหว
          </Link>
        </div>
      </div>

      <div className="grid grid-cols-1 lg:grid-cols-2 gap-6">
        {/* Low Stock Alerts */}
        <div className="bg-white rounded-lg shadow">
          <div className="p-4 border-b flex items-center justify-between">
            <h2 className="text-lg font-semibold text-gray-800 flex items-center gap-2">
              <TrendingDown className="w-5 h-5 text-orange-600" />
              สินค้าใกล้หมด
            </h2>
            <Link
              href="/new/inventory/items?filter=low"
              className="text-blue-600 hover:text-blue-700 text-sm flex items-center gap-1"
            >
              ดูทั้งหมด <ArrowRight className="w-4 h-4" />
            </Link>
          </div>
          <div className="p-4">
            {loading ? (
              <div className="flex items-center justify-center py-8">
                <Loader2 className="w-6 h-6 animate-spin text-blue-600" />
              </div>
            ) : lowStockItems.length === 0 ? (
              <div className="text-center py-8 text-gray-500">
                <Package className="w-12 h-12 text-gray-300 mx-auto mb-2" />
                <p>ไม่มีสินค้าใกล้หมด</p>
              </div>
            ) : (
              <div className="space-y-3">
                {lowStockItems.map((item) => {
                  const status = getStockStatus(item.currentStock, item.minStock)
                  return (
                    <div
                      key={item.id}
                      className="flex items-center justify-between p-3 bg-gray-50 rounded-lg hover:bg-gray-100 transition-colors"
                    >
                      <div className="flex items-center gap-3">
                        <div className={`w-2 h-10 rounded-full ${
                          status === 'out' ? 'bg-red-500' :
                          status === 'critical' ? 'bg-orange-500' :
                          'bg-yellow-500'
                        }`} />
                        <div>
                          <div className="font-medium text-gray-800">{item.itemName}</div>
                          <div className="text-xs text-gray-500">{item.itemCode}</div>
                        </div>
                      </div>
                      <div className="flex items-center gap-3">
                        <div className="text-right">
                          <div className="font-bold text-lg">{item.currentStock}</div>
                          <div className="text-xs text-gray-500">ขั้นต่ำ: {item.minStock}</div>
                        </div>
                        <span className={`px-2 py-1 rounded text-xs font-medium ${getStockStatusColor(status)}`}>
                          {getStockStatusLabel(status)}
                        </span>
                        <button
                          onClick={() => handleAdjustStock(item)}
                          className="p-2 text-blue-600 hover:bg-blue-50 rounded-lg transition-colors"
                          title="ปรับสต็อก"
                        >
                          <RefreshCw className="w-4 h-4" />
                        </button>
                      </div>
                    </div>
                  )
                })}
              </div>
            )}
          </div>
        </div>

        {/* Recent Transactions */}
        <div className="bg-white rounded-lg shadow">
          <div className="p-4 border-b flex items-center justify-between">
            <h2 className="text-lg font-semibold text-gray-800 flex items-center gap-2">
              <Clock className="w-5 h-5 text-blue-600" />
              การเคลื่อนไหวล่าสุด
            </h2>
            <Link
              href="/new/inventory/transactions"
              className="text-blue-600 hover:text-blue-700 text-sm flex items-center gap-1"
            >
              ดูทั้งหมด <ArrowRight className="w-4 h-4" />
            </Link>
          </div>
          <div className="p-4">
            {loading ? (
              <div className="flex items-center justify-center py-8">
                <Loader2 className="w-6 h-6 animate-spin text-blue-600" />
              </div>
            ) : recentTransactions.length === 0 ? (
              <div className="text-center py-8 text-gray-500">
                <FileText className="w-12 h-12 text-gray-300 mx-auto mb-2" />
                <p>ยังไม่มีการเคลื่อนไหว</p>
              </div>
            ) : (
              <div className="space-y-3">
                {recentTransactions.map((tx) => (
                  <div
                    key={tx.id}
                    className="flex items-center justify-between p-3 bg-gray-50 rounded-lg"
                  >
                    <div className="flex items-center gap-3">
                      <span className={`px-2 py-1 rounded text-xs font-medium ${getTransactionTypeColor(tx.transactionType)}`}>
                        {getTransactionTypeLabel(tx.transactionType)}
                      </span>
                      <div>
                        <div className="font-medium text-gray-800">{tx.itemName}</div>
                        <div className="text-xs text-gray-500">
                          {tx.roomNumber && `ห้อง ${tx.roomNumber} - `}
                          {formatDate(tx.createdAt)}
                        </div>
                      </div>
                    </div>
                    <div className="text-right">
                      <div className={`font-bold ${
                        tx.transactionType === 'IN' ? 'text-green-600' :
                        tx.transactionType === 'OUT' ? 'text-red-600' :
                        'text-blue-600'
                      }`}>
                        {tx.transactionType === 'IN' ? '+' : tx.transactionType === 'OUT' ? '-' : ''}
                        {tx.quantity}
                      </div>
                      <div className="text-xs text-gray-500">
                        {tx.previousStock} → {tx.newStock}
                      </div>
                    </div>
                  </div>
                ))}
              </div>
            )}
          </div>
        </div>
      </div>

      {/* Stock Adjustment Modal */}
      <StockAdjustmentModal
        isOpen={showAdjustmentModal}
        onClose={() => {
          setShowAdjustmentModal(false)
          setSelectedItem(null)
        }}
        onSuccess={handleAdjustmentSuccess}
        preselectedItem={selectedItem}
      />
    </div>
  )
}
