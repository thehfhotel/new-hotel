'use client'

import { useState, useEffect, useCallback } from 'react'
import { useSearchParams } from 'next/navigation'
import {
  Package,
  Plus,
  Search,
  X,
  Edit3,
  Loader2,
  AlertCircle,
  Filter,
  RefreshCw,
} from 'lucide-react'
import {
  InventoryItem,
  InventoryItemFormData,
  InventoryCategory,
  INVENTORY_CATEGORIES,
  getStockStatus,
  getStockStatusColor,
  getStockStatusLabel,
} from '@/types/inventory'
import InventoryItemForm from '@/components/forms/InventoryItemForm'
import StockAdjustmentModal from '@/components/modals/StockAdjustmentModal'

type SortField = 'itemCode' | 'itemName' | 'currentStock'
type SortOrder = 'asc' | 'desc'

export default function InventoryItemsPage() {
  const searchParams = useSearchParams()
  const initialFilter = searchParams.get('filter')
  const initialMode = searchParams.get('mode')

  const [items, setItems] = useState<InventoryItem[]>([])
  const [loading, setLoading] = useState(true)
  const [error, setError] = useState<string | null>(null)
  const [searchQuery, setSearchQuery] = useState('')
  const [debouncedSearch, setDebouncedSearch] = useState('')
  const [categoryFilter, setCategoryFilter] = useState<InventoryCategory | ''>('')
  const [showLowStockOnly, setShowLowStockOnly] = useState(initialFilter === 'low')
  const [sortField, setSortField] = useState<SortField>('itemCode')
  const [sortOrder, setSortOrder] = useState<SortOrder>('asc')

  // Form modal state
  const [showForm, setShowForm] = useState(initialMode === 'add')
  const [formMode, setFormMode] = useState<'create' | 'edit'>('create')
  const [selectedItem, setSelectedItem] = useState<InventoryItemFormData | null>(null)

  // Stock adjustment modal state
  const [showAdjustmentModal, setShowAdjustmentModal] = useState(false)
  const [adjustmentItem, setAdjustmentItem] = useState<InventoryItem | null>(null)

  // Debounce search input
  useEffect(() => {
    const timer = setTimeout(() => {
      setDebouncedSearch(searchQuery)
    }, 300)

    return () => clearTimeout(timer)
  }, [searchQuery])

  // Fetch items
  const fetchItems = useCallback(async () => {
    setLoading(true)
    setError(null)
    try {
      const params = new URLSearchParams({
        sortBy: sortField,
        sortOrder: sortOrder,
      })

      if (debouncedSearch) {
        params.append('search', debouncedSearch)
      }

      if (categoryFilter) {
        params.append('category', categoryFilter)
      }

      if (showLowStockOnly) {
        params.append('lowStock', 'true')
      }

      const response = await fetch(`/api/new/inventory/items?${params}`)
      if (!response.ok) {
        throw new Error('Failed to fetch items')
      }

      const data = await response.json()
      if (data.success) {
        setItems(data.data || [])
      } else {
        throw new Error(data.message || 'Failed to fetch items')
      }
    } catch (err) {
      setError(err instanceof Error ? err.message : 'เกิดข้อผิดพลาดในการโหลดข้อมูล')
    } finally {
      setLoading(false)
    }
  }, [debouncedSearch, categoryFilter, showLowStockOnly, sortField, sortOrder])

  useEffect(() => {
    fetchItems()
  }, [fetchItems])

  // Handle sort change
  const handleSortChange = (field: SortField) => {
    if (sortField === field) {
      setSortOrder(sortOrder === 'asc' ? 'desc' : 'asc')
    } else {
      setSortField(field)
      setSortOrder('asc')
    }
  }

  // Handle add new item
  const handleAddItem = () => {
    setSelectedItem(null)
    setFormMode('create')
    setShowForm(true)
  }

  // Handle edit item
  const handleEditItem = (item: InventoryItem) => {
    setSelectedItem({
      id: item.id,
      itemCode: item.itemCode,
      itemName: item.itemName,
      category: item.category,
      unit: item.unit,
      minStock: item.minStock,
      currentStock: item.currentStock,
      costPerUnit: item.costPerUnit,
      active: item.active,
    })
    setFormMode('edit')
    setShowForm(true)
  }

  // Handle adjust stock
  const handleAdjustStock = (item: InventoryItem) => {
    setAdjustmentItem(item)
    setShowAdjustmentModal(true)
  }

  // Handle save item
  const handleSaveItem = async (data: InventoryItemFormData) => {
    const endpoint = data.id
      ? `/api/new/inventory/items/${data.id}`
      : '/api/new/inventory/items'
    const method = data.id ? 'PUT' : 'POST'

    const response = await fetch(endpoint, {
      method,
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({
        itemCode: data.itemCode.toUpperCase(),
        itemName: data.itemName,
        category: data.category,
        unit: data.unit,
        minStock: data.minStock,
        currentStock: data.currentStock,
        costPerUnit: data.costPerUnit,
        active: data.active,
      }),
    })

    const result = await response.json()

    if (!response.ok || !result.success) {
      throw new Error(result.message || 'Failed to save item')
    }

    // Refresh the list
    fetchItems()
  }

  // Handle delete item
  const handleDeleteItem = async (id: number) => {
    const response = await fetch(`/api/new/inventory/items/${id}`, {
      method: 'DELETE',
    })

    const result = await response.json()

    if (!response.ok || !result.success) {
      throw new Error(result.message || 'Failed to delete item')
    }

    // Refresh the list
    fetchItems()
  }

  // Close form
  const handleCloseForm = () => {
    setShowForm(false)
    setSelectedItem(null)
  }

  // Get category label
  const getCategoryLabel = (category: string) => {
    const cat = INVENTORY_CATEGORIES.find((c) => c.value === category)
    return cat ? cat.labelTh : category
  }

  return (
    <div className="space-y-6">
      {/* Page Header */}
      <div className="flex items-center justify-between">
        <div className="flex items-center gap-3">
          <Package className="w-8 h-8 text-blue-600" />
          <div>
            <h1 className="text-2xl font-bold text-gray-800">จัดการสินค้า</h1>
            <p className="text-gray-600">
              จำนวนสินค้าทั้งหมด {items.length} รายการ
            </p>
          </div>
        </div>

        {/* Add Button */}
        <button
          onClick={handleAddItem}
          className="flex items-center gap-2 px-4 py-2 bg-blue-600 text-white rounded-lg hover:bg-blue-700 transition-colors"
        >
          <Plus className="w-5 h-5" />
          เพิ่มสินค้า
        </button>
      </div>

      {/* Search and Filter Bar */}
      <div className="bg-white rounded-lg shadow p-4">
        <div className="flex flex-col lg:flex-row gap-4">
          {/* Search */}
          <div className="relative flex-1">
            <Search className="absolute left-3 top-1/2 -translate-y-1/2 w-5 h-5 text-gray-400" />
            <input
              type="text"
              value={searchQuery}
              onChange={(e) => setSearchQuery(e.target.value)}
              placeholder="ค้นหาด้วยรหัส หรือ ชื่อสินค้า..."
              className="w-full pl-10 pr-4 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-blue-500 outline-none transition-colors"
            />
            {searchQuery && (
              <button
                onClick={() => setSearchQuery('')}
                className="absolute right-3 top-1/2 -translate-y-1/2 text-gray-400 hover:text-gray-600"
                aria-label="ล้างการค้นหา"
              >
                <X className="w-4 h-4" />
              </button>
            )}
          </div>

          {/* Category Filter */}
          <div className="flex items-center gap-2">
            <Filter className="w-5 h-5 text-gray-400" />
            <select
              value={categoryFilter}
              onChange={(e) => setCategoryFilter(e.target.value as InventoryCategory | '')}
              className="px-3 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-blue-500 outline-none bg-white"
            >
              <option value="">ทุกหมวดหมู่</option>
              {INVENTORY_CATEGORIES.map((cat) => (
                <option key={cat.value} value={cat.value}>
                  {cat.labelTh}
                </option>
              ))}
            </select>
          </div>

          {/* Low Stock Toggle */}
          <label className="flex items-center gap-2 cursor-pointer">
            <input
              type="checkbox"
              checked={showLowStockOnly}
              onChange={(e) => setShowLowStockOnly(e.target.checked)}
              className="w-4 h-4 text-blue-600 border-gray-300 rounded focus:ring-blue-500"
            />
            <span className="text-sm text-gray-600">แสดงเฉพาะสินค้าใกล้หมด</span>
          </label>
        </div>

        {/* Sort Buttons */}
        <div className="flex items-center gap-2 mt-4">
          <span className="text-sm text-gray-600">เรียงตาม:</span>
          <button
            onClick={() => handleSortChange('itemCode')}
            className={`px-3 py-1.5 text-sm rounded-lg border transition-colors ${
              sortField === 'itemCode'
                ? 'border-blue-500 bg-blue-50 text-blue-700'
                : 'border-gray-300 text-gray-600 hover:bg-gray-50'
            }`}
          >
            รหัส {sortField === 'itemCode' && (sortOrder === 'asc' ? '(A-Z)' : '(Z-A)')}
          </button>
          <button
            onClick={() => handleSortChange('itemName')}
            className={`px-3 py-1.5 text-sm rounded-lg border transition-colors ${
              sortField === 'itemName'
                ? 'border-blue-500 bg-blue-50 text-blue-700'
                : 'border-gray-300 text-gray-600 hover:bg-gray-50'
            }`}
          >
            ชื่อ {sortField === 'itemName' && (sortOrder === 'asc' ? '(A-Z)' : '(Z-A)')}
          </button>
          <button
            onClick={() => handleSortChange('currentStock')}
            className={`px-3 py-1.5 text-sm rounded-lg border transition-colors ${
              sortField === 'currentStock'
                ? 'border-blue-500 bg-blue-50 text-blue-700'
                : 'border-gray-300 text-gray-600 hover:bg-gray-50'
            }`}
          >
            จำนวน {sortField === 'currentStock' && (sortOrder === 'asc' ? '(น้อย-มาก)' : '(มาก-น้อย)')}
          </button>
        </div>

        {debouncedSearch && (
          <p className="mt-2 text-sm text-gray-600">
            ผลการค้นหา &quot;{debouncedSearch}&quot;: พบ {items.length} รายการ
          </p>
        )}
      </div>

      {/* Error Message */}
      {error && (
        <div className="flex items-center gap-2 p-4 bg-red-50 border border-red-200 rounded-lg text-red-700">
          <AlertCircle className="w-5 h-5 flex-shrink-0" />
          <span>{error}</span>
        </div>
      )}

      {/* Items Table */}
      <div className="bg-white rounded-lg shadow overflow-hidden">
        {loading ? (
          <div className="flex items-center justify-center py-12">
            <Loader2 className="w-6 h-6 animate-spin text-blue-600" />
            <span className="ml-2 text-gray-600">กำลังโหลดข้อมูล...</span>
          </div>
        ) : items.length === 0 ? (
          <div className="text-center py-12 text-gray-500">
            <Package className="w-12 h-12 text-gray-300 mx-auto mb-3" />
            <p>ไม่พบสินค้า</p>
            <button
              onClick={handleAddItem}
              className="mt-4 text-blue-600 hover:text-blue-700 font-medium"
            >
              + เพิ่มสินค้าใหม่
            </button>
          </div>
        ) : (
          <div className="overflow-x-auto">
            <table className="w-full">
              <thead className="bg-gray-50 border-b">
                <tr>
                  <th className="px-4 py-3 text-left text-sm font-medium text-gray-600">รหัส</th>
                  <th className="px-4 py-3 text-left text-sm font-medium text-gray-600">ชื่อสินค้า</th>
                  <th className="px-4 py-3 text-left text-sm font-medium text-gray-600">หมวดหมู่</th>
                  <th className="px-4 py-3 text-left text-sm font-medium text-gray-600">หน่วย</th>
                  <th className="px-4 py-3 text-right text-sm font-medium text-gray-600">ขั้นต่ำ</th>
                  <th className="px-4 py-3 text-right text-sm font-medium text-gray-600">คงเหลือ</th>
                  <th className="px-4 py-3 text-center text-sm font-medium text-gray-600">สถานะ</th>
                  <th className="px-4 py-3 text-center text-sm font-medium text-gray-600">การดำเนินการ</th>
                </tr>
              </thead>
              <tbody className="divide-y divide-gray-200">
                {items.map((item) => {
                  const status = getStockStatus(item.currentStock, item.minStock)
                  return (
                    <tr
                      key={item.id}
                      className={`hover:bg-gray-50 ${!item.active ? 'opacity-50' : ''}`}
                    >
                      <td className="px-4 py-3">
                        <span className="font-mono text-sm bg-gray-100 px-2 py-1 rounded">
                          {item.itemCode}
                        </span>
                      </td>
                      <td className="px-4 py-3">
                        <div className="font-medium text-gray-800">{item.itemName}</div>
                        {!item.active && (
                          <span className="text-xs text-red-600">ปิดใช้งาน</span>
                        )}
                      </td>
                      <td className="px-4 py-3 text-gray-600">
                        {getCategoryLabel(item.category)}
                      </td>
                      <td className="px-4 py-3 text-gray-600">{item.unit}</td>
                      <td className="px-4 py-3 text-right text-gray-600">{item.minStock}</td>
                      <td className="px-4 py-3 text-right">
                        <span className={`font-bold text-lg ${
                          status === 'good' ? 'text-green-600' :
                          status === 'low' ? 'text-yellow-600' :
                          status === 'critical' ? 'text-orange-600' :
                          'text-red-600'
                        }`}>
                          {item.currentStock}
                        </span>
                      </td>
                      <td className="px-4 py-3 text-center">
                        <span className={`px-2 py-1 rounded text-xs font-medium ${getStockStatusColor(status)}`}>
                          {getStockStatusLabel(status)}
                        </span>
                      </td>
                      <td className="px-4 py-3">
                        <div className="flex items-center justify-center gap-1">
                          <button
                            onClick={() => handleAdjustStock(item)}
                            className="p-2 text-green-600 hover:bg-green-50 rounded-lg transition-colors"
                            title="ปรับสต็อก"
                          >
                            <RefreshCw className="w-4 h-4" />
                          </button>
                          <button
                            onClick={() => handleEditItem(item)}
                            className="p-2 text-blue-600 hover:bg-blue-50 rounded-lg transition-colors"
                            title="แก้ไข"
                          >
                            <Edit3 className="w-4 h-4" />
                          </button>
                        </div>
                      </td>
                    </tr>
                  )
                })}
              </tbody>
            </table>
          </div>
        )}
      </div>

      {/* Inventory Item Form Modal */}
      <InventoryItemForm
        isOpen={showForm}
        onClose={handleCloseForm}
        onSave={handleSaveItem}
        onDelete={handleDeleteItem}
        initialData={selectedItem}
        mode={formMode}
      />

      {/* Stock Adjustment Modal */}
      <StockAdjustmentModal
        isOpen={showAdjustmentModal}
        onClose={() => {
          setShowAdjustmentModal(false)
          setAdjustmentItem(null)
        }}
        onSuccess={fetchItems}
        preselectedItem={adjustmentItem}
      />
    </div>
  )
}
