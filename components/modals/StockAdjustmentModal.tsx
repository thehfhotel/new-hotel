'use client'

import { useState, useEffect, useCallback } from 'react'
import {
  X,
  Search,
  Loader2,
  Package,
  Plus,
  Minus,
  RefreshCw,
  AlertCircle,
  FileText,
} from 'lucide-react'
import { InventoryItem, getStockStatus, getStockStatusColor, getStockStatusLabel } from '@/types/inventory'
import { useBranchFetch } from '@/lib/use-branch-fetch'

interface StockAdjustmentModalProps {
  isOpen: boolean
  onClose: () => void
  onSuccess: () => void
  preselectedItem?: InventoryItem | null
}

type AdjustmentType = 'add' | 'remove' | 'set'

const adjustmentTypes: { value: AdjustmentType; label: string; icon: React.ReactNode }[] = [
  { value: 'add', label: 'รับเข้า', icon: <Plus className="w-4 h-4" /> },
  { value: 'remove', label: 'เบิกออก', icon: <Minus className="w-4 h-4" /> },
  { value: 'set', label: 'ตั้งค่าใหม่', icon: <RefreshCw className="w-4 h-4" /> },
]

export default function StockAdjustmentModal({
  isOpen,
  onClose,
  onSuccess,
  preselectedItem = null,
}: StockAdjustmentModalProps) {
  const branchFetch = useBranchFetch()

  // Item search state
  const [searchQuery, setSearchQuery] = useState('')
  const [items, setItems] = useState<InventoryItem[]>([])
  const [selectedItem, setSelectedItem] = useState<InventoryItem | null>(null)
  const [isSearching, setIsSearching] = useState(false)
  const [showDropdown, setShowDropdown] = useState(false)

  // Form state
  const [adjustmentType, setAdjustmentType] = useState<AdjustmentType>('add')
  const [quantity, setQuantity] = useState('')
  const [notes, setNotes] = useState('')

  // Submission state
  const [isSubmitting, setIsSubmitting] = useState(false)
  const [error, setError] = useState<string | null>(null)

  // Set preselected item when modal opens
  useEffect(() => {
    if (isOpen) {
      if (preselectedItem) {
        setSelectedItem(preselectedItem)
        setSearchQuery(preselectedItem.itemName)
      } else {
        resetForm()
      }
    }
  }, [isOpen, preselectedItem])

  // Search items
  const searchItems = useCallback(async (query: string) => {
    if (query.length < 1) {
      setItems([])
      return
    }

    setIsSearching(true)
    try {
      const res = await branchFetch(`/api/new/inventory/items?search=${encodeURIComponent(query)}&limit=10`)
      const data = await res.json()
      if (data.success) {
        setItems(data.data || [])
      }
    } catch (err) {
      console.error('Error searching items:', err)
    } finally {
      setIsSearching(false)
    }
  }, [branchFetch])

  // Debounced search
  useEffect(() => {
    const timer = setTimeout(() => {
      if (searchQuery && !selectedItem) {
        searchItems(searchQuery)
        setShowDropdown(true)
      } else {
        setItems([])
        setShowDropdown(false)
      }
    }, 300)

    return () => clearTimeout(timer)
  }, [searchQuery, selectedItem, searchItems])

  // Handle item selection
  const handleSelectItem = (item: InventoryItem) => {
    setSelectedItem(item)
    setSearchQuery(item.itemName)
    setShowDropdown(false)
  }

  // Reset form
  const resetForm = () => {
    setSearchQuery('')
    setSelectedItem(null)
    setItems([])
    setAdjustmentType('add')
    setQuantity('')
    setNotes('')
    setError(null)
  }

  // Handle close
  const handleClose = () => {
    resetForm()
    onClose()
  }

  // Calculate new stock
  const calculateNewStock = (): number => {
    if (!selectedItem) return 0
    const qty = parseFloat(quantity) || 0

    switch (adjustmentType) {
      case 'add':
        return selectedItem.currentStock + qty
      case 'remove':
        return Math.max(0, selectedItem.currentStock - qty)
      case 'set':
        return qty
    }
  }

  // Handle submit
  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault()

    if (!selectedItem) {
      setError('กรุณาเลือกสินค้า')
      return
    }

    const qty = parseFloat(quantity)
    if (isNaN(qty) || qty <= 0) {
      setError('กรุณาระบุจำนวนที่ถูกต้อง')
      return
    }

    if (adjustmentType === 'remove' && qty > selectedItem.currentStock) {
      setError('จำนวนที่เบิกออกมากกว่าจำนวนคงเหลือ')
      return
    }

    setIsSubmitting(true)
    setError(null)

    try {
      const res = await branchFetch('/api/new/inventory/adjustments', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({
          itemId: selectedItem.id,
          adjustmentType,
          quantity: qty,
          notes: notes || undefined,
        }),
      })

      const data = await res.json()

      if (!res.ok || !data.success) {
        throw new Error(data.error || data.message || 'เกิดข้อผิดพลาดในการปรับสต็อก')
      }

      onSuccess()
      handleClose()
    } catch (err) {
      setError(err instanceof Error ? err.message : 'เกิดข้อผิดพลาดในการปรับสต็อก')
    } finally {
      setIsSubmitting(false)
    }
  }

  if (!isOpen) return null

  const newStock = calculateNewStock()
  const newStockStatus = selectedItem ? getStockStatus(newStock, selectedItem.minStock) : 'good'

  return (
    <div
      className="fixed inset-0 bg-black/30 flex items-center justify-center z-50"
      onClick={handleClose}
    >
      <div
        className="bg-white border border-gray-200 rounded-xl shadow-2xl w-full max-w-lg mx-4 max-h-[90vh] overflow-y-auto"
        onClick={(e) => e.stopPropagation()}
      >
        {/* Header */}
        <div className="flex items-center justify-between p-4 border-b border-gray-200">
          <div>
            <h2 className="text-xl font-bold text-gray-900">ปรับสต็อก</h2>
            <p className="text-sm text-gray-500">เพิ่ม, ลด หรือตั้งค่าจำนวนสต็อกใหม่</p>
          </div>
          <button
            onClick={handleClose}
            className="p-2 hover:bg-gray-100 rounded-lg transition-colors"
          >
            <X size={20} />
          </button>
        </div>

        {/* Form */}
        <form onSubmit={handleSubmit} className="p-4 space-y-4">
          {/* Error Message */}
          {error && (
            <div className="flex items-center gap-2 p-3 bg-red-50 border border-red-200 rounded-lg text-red-600 text-sm">
              <AlertCircle className="w-4 h-4 flex-shrink-0" />
              {error}
            </div>
          )}

          {/* Item Search */}
          <div className="relative">
            <label className="block text-sm font-medium text-gray-700 mb-1">
              <Package size={16} className="inline mr-1" />
              สินค้า *
            </label>
            <div className="relative">
              <input
                type="text"
                value={searchQuery}
                onChange={(e) => {
                  setSearchQuery(e.target.value)
                  setSelectedItem(null)
                }}
                onFocus={() => items.length > 0 && setShowDropdown(true)}
                placeholder="ค้นหาด้วยชื่อหรือรหัสสินค้า..."
                className="w-full px-3 py-2 pr-10 bg-gray-100 border border-gray-300 text-gray-800 rounded-lg focus:ring-2 focus:ring-red-500 focus:border-red-500"
              />
              {isSearching && (
                <Loader2
                  className="absolute right-3 top-1/2 -translate-y-1/2 animate-spin text-gray-500"
                  size={18}
                />
              )}
              {!isSearching && (
                <Search
                  className="absolute right-3 top-1/2 -translate-y-1/2 text-gray-500"
                  size={18}
                />
              )}
            </div>

            {/* Item Dropdown */}
            {showDropdown && items.length > 0 && (
              <div className="absolute z-10 w-full mt-1 bg-white border border-gray-300 rounded-lg shadow-lg max-h-48 overflow-y-auto">
                {items.map((item) => {
                  const status = getStockStatus(item.currentStock, item.minStock)
                  return (
                    <button
                      key={item.id}
                      type="button"
                      onClick={() => handleSelectItem(item)}
                      className="w-full px-3 py-2 text-left hover:bg-gray-100 flex items-center justify-between"
                    >
                      <div className="flex items-center gap-2">
                        <div className="w-8 h-8 bg-red-500/10 rounded-full flex items-center justify-center">
                          <Package size={14} className="text-red-600" />
                        </div>
                        <div>
                          <div className="font-medium text-gray-800">{item.itemName}</div>
                          <div className="text-xs text-gray-500">{item.itemCode}</div>
                        </div>
                      </div>
                      <div className="text-right">
                        <div className="font-medium">{item.currentStock} {item.unit}</div>
                        <span className={`text-xs px-1.5 py-0.5 rounded ${getStockStatusColor(status)}`}>
                          {getStockStatusLabel(status)}
                        </span>
                      </div>
                    </button>
                  )
                })}
              </div>
            )}

            {showDropdown && items.length === 0 && searchQuery.length >= 1 && !isSearching && (
              <div className="absolute z-10 w-full mt-1 bg-white border border-gray-300 rounded-lg shadow-lg p-3 text-center text-gray-500 text-sm">
                ไม่พบสินค้า
              </div>
            )}

            {/* Selected Item Display */}
            {selectedItem && (
              <div className="mt-2 p-3 bg-red-500/10 border border-gray-300 rounded-lg">
                <div className="flex items-center justify-between">
                  <div>
                    <div className="font-medium text-red-600">{selectedItem.itemName}</div>
                    <div className="text-sm text-red-600">{selectedItem.itemCode}</div>
                  </div>
                  <div className="text-right">
                    <div className="text-sm text-gray-500">คงเหลือปัจจุบัน</div>
                    <div className="font-bold text-lg">{selectedItem.currentStock} {selectedItem.unit}</div>
                  </div>
                </div>
              </div>
            )}
          </div>

          {/* Adjustment Type */}
          <div>
            <label className="block text-sm font-medium text-gray-700 mb-2">
              ประเภทการปรับ *
            </label>
            <div className="grid grid-cols-3 gap-2">
              {adjustmentTypes.map((type) => (
                <button
                  key={type.value}
                  type="button"
                  onClick={() => setAdjustmentType(type.value)}
                  className={`flex items-center justify-center gap-2 px-3 py-2 rounded-lg border transition-colors ${
                    adjustmentType === type.value
                      ? type.value === 'add'
                        ? 'border-green-500 bg-green-500/10 text-green-600'
                        : type.value === 'remove'
                        ? 'border-red-500 bg-red-500/10 text-red-600'
                        : 'border-blue-500 bg-blue-500/10 text-blue-600'
                      : 'border-gray-300 text-gray-500 hover:bg-gray-100'
                  }`}
                >
                  {type.icon}
                  {type.label}
                </button>
              ))}
            </div>
          </div>

          {/* Quantity */}
          <div>
            <label className="block text-sm font-medium text-gray-700 mb-1">
              จำนวน *
            </label>
            <input
              type="number"
              value={quantity}
              onChange={(e) => setQuantity(e.target.value)}
              placeholder={adjustmentType === 'set' ? 'จำนวนใหม่' : 'จำนวนที่ต้องการ'}
              min="0"
              step="1"
              className="w-full px-3 py-2 bg-gray-100 border border-gray-300 text-gray-800 rounded-lg focus:ring-2 focus:ring-red-500 focus:border-red-500"
              required
            />
            {selectedItem && quantity && (
              <div className="mt-2 flex items-center gap-2 text-sm">
                <span className="text-gray-500">สต็อกใหม่:</span>
                <span className={`font-bold px-2 py-0.5 rounded ${getStockStatusColor(newStockStatus)}`}>
                  {newStock} {selectedItem.unit}
                </span>
              </div>
            )}
          </div>

          {/* Notes */}
          <div>
            <label className="block text-sm font-medium text-gray-700 mb-1">
              <FileText size={16} className="inline mr-1" />
              หมายเหตุ
            </label>
            <textarea
              value={notes}
              onChange={(e) => setNotes(e.target.value)}
              placeholder="เหตุผลในการปรับสต็อก..."
              rows={2}
              className="w-full px-3 py-2 bg-gray-100 border border-gray-300 text-gray-800 rounded-lg focus:ring-2 focus:ring-red-500 focus:border-red-500 resize-none"
            />
          </div>

          {/* Submit Button */}
          <div className="flex gap-3 pt-2">
            <button
              type="button"
              onClick={handleClose}
              className="flex-1 px-4 py-2 border border-gray-300 text-gray-700 rounded-lg hover:bg-gray-100"
              disabled={isSubmitting}
            >
              ยกเลิก
            </button>
            <button
              type="submit"
              disabled={isSubmitting || !selectedItem || !quantity}
              className="flex-1 px-4 py-2 bg-red-600 text-white rounded-lg hover:bg-red-700 disabled:opacity-50 disabled:cursor-not-allowed flex items-center justify-center gap-2"
            >
              {isSubmitting ? (
                <>
                  <Loader2 className="animate-spin" size={18} />
                  กำลังบันทึก...
                </>
              ) : (
                'ยืนยันการปรับสต็อก'
              )}
            </button>
          </div>
        </form>
      </div>
    </div>
  )
}
