'use client'

import { useState, useEffect } from 'react'
import {
  X,
  Check,
  AlertTriangle,
  Package,
  Loader2,
  RefreshCw,
  FileText,
  CheckCircle,
  XCircle,
} from 'lucide-react'
import {
  RoomInventory,
  RoomInventoryItem,
  INVENTORY_CATEGORIES,
} from '@/types/inventory'

interface RoomInventoryChecklistProps {
  isOpen: boolean
  onClose: () => void
  roomId: number
  roomNumber: string
  onSuccess: () => void
}

interface ChecklistItem extends RoomInventoryItem {
  verified: boolean
  actualQuantity: number
}

export default function RoomInventoryChecklist({
  isOpen,
  onClose,
  roomId,
  roomNumber,
  onSuccess,
}: RoomInventoryChecklistProps) {
  const [loading, setLoading] = useState(true)
  const [submitting, setSubmitting] = useState(false)
  const [error, setError] = useState<string | null>(null)
  const [items, setItems] = useState<ChecklistItem[]>([])
  const [notes, setNotes] = useState('')

  // Fetch room inventory
  useEffect(() => {
    if (isOpen) {
      fetchRoomInventory()
    }
  }, [isOpen, roomId])

  const fetchRoomInventory = async () => {
    setLoading(true)
    setError(null)
    try {
      const res = await fetch(`/api/new/inventory/rooms/${roomId}`)
      const data = await res.json()

      if (data.success && data.data) {
        const roomInventory: RoomInventory = data.data
        setItems(
          roomInventory.items.map((item) => ({
            ...item,
            verified: true,
            actualQuantity: item.assignedQuantity,
          }))
        )
      } else {
        setItems([])
      }
    } catch (err) {
      console.error('Error fetching room inventory:', err)
      setError('ไม่สามารถโหลดข้อมูลสินค้าคงคลังของห้องได้')
    } finally {
      setLoading(false)
    }
  }

  const handleVerifiedChange = (itemId: number, verified: boolean) => {
    setItems((prev) =>
      prev.map((item) =>
        item.itemId === itemId
          ? { ...item, verified, actualQuantity: verified ? item.assignedQuantity : 0 }
          : item
      )
    )
  }

  const handleQuantityChange = (itemId: number, quantity: number) => {
    setItems((prev) =>
      prev.map((item) =>
        item.itemId === itemId
          ? {
              ...item,
              actualQuantity: quantity,
              verified: quantity >= item.assignedQuantity,
            }
          : item
      )
    )
  }

  const handleSubmit = async () => {
    setSubmitting(true)
    setError(null)

    try {
      const res = await fetch(`/api/new/inventory/rooms/${roomId}/check`, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({
          items: items.map((item) => ({
            itemId: item.itemId,
            verified: item.verified,
            actualQuantity: item.actualQuantity,
          })),
          notes: notes || undefined,
        }),
      })

      const data = await res.json()

      if (!res.ok || !data.success) {
        throw new Error(data.error || data.message || 'เกิดข้อผิดพลาด')
      }

      onSuccess()
      onClose()
    } catch (err) {
      setError(err instanceof Error ? err.message : 'เกิดข้อผิดพลาดในการบันทึก')
    } finally {
      setSubmitting(false)
    }
  }

  const handleReplenish = async () => {
    setSubmitting(true)
    setError(null)

    try {
      const missingItems = items.filter(
        (item) => !item.verified || item.actualQuantity < item.assignedQuantity
      )

      if (missingItems.length === 0) {
        setError('ไม่มีสินค้าที่ต้องเติม')
        setSubmitting(false)
        return
      }

      const res = await fetch(`/api/new/inventory/rooms/${roomId}/replenish`, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({
          items: missingItems.map((item) => ({
            itemId: item.itemId,
            quantity: item.assignedQuantity - item.actualQuantity,
          })),
          notes: notes || 'เติมสินค้าหลังตรวจสอบ',
        }),
      })

      const data = await res.json()

      if (!res.ok || !data.success) {
        throw new Error(data.error || data.message || 'เกิดข้อผิดพลาด')
      }

      onSuccess()
      onClose()
    } catch (err) {
      setError(err instanceof Error ? err.message : 'เกิดข้อผิดพลาดในการเติมสินค้า')
    } finally {
      setSubmitting(false)
    }
  }

  if (!isOpen) return null

  const missingCount = items.filter(
    (item) => !item.verified || item.actualQuantity < item.assignedQuantity
  ).length

  const getCategoryLabel = (category: string) => {
    const cat = INVENTORY_CATEGORIES.find((c) => c.value === category)
    return cat ? cat.labelTh : category
  }

  // Group items by category
  const groupedItems = items.reduce((acc, item) => {
    const category = item.category
    if (!acc[category]) {
      acc[category] = []
    }
    acc[category].push(item)
    return acc
  }, {} as Record<string, ChecklistItem[]>)

  return (
    <div
      className="fixed inset-0 bg-black/30 flex items-center justify-center z-50"
      onClick={onClose}
    >
      <div
        className="bg-white rounded-xl shadow-2xl border border-gray-200 w-full max-w-2xl mx-4 max-h-[90vh] flex flex-col"
        onClick={(e) => e.stopPropagation()}
      >
        {/* Header */}
        <div className="flex items-center justify-between p-4 border-b border-gray-200">
          <div>
            <h2 className="text-xl font-bold text-gray-900">ตรวจสอบสินค้าในห้อง</h2>
            <p className="text-sm text-gray-500">ห้อง {roomNumber}</p>
          </div>
          <button
            onClick={onClose}
            className="p-2 hover:bg-gray-100 rounded-lg transition-colors"
          >
            <X size={20} />
          </button>
        </div>

        {/* Content */}
        <div className="flex-1 overflow-y-auto p-4">
          {loading ? (
            <div className="flex items-center justify-center py-12">
              <Loader2 className="w-6 h-6 animate-spin text-red-600" />
              <span className="ml-2 text-gray-500">กำลังโหลด...</span>
            </div>
          ) : error && items.length === 0 ? (
            <div className="text-center py-12">
              <AlertTriangle className="w-12 h-12 text-amber-400 mx-auto mb-3" />
              <p className="text-gray-500">{error}</p>
              <button
                onClick={fetchRoomInventory}
                className="mt-4 text-red-600 hover:text-red-300 font-medium"
              >
                ลองใหม่
              </button>
            </div>
          ) : items.length === 0 ? (
            <div className="text-center py-12">
              <Package className="w-12 h-12 text-gray-400 mx-auto mb-3" />
              <p className="text-gray-500">ไม่มีสินค้าที่กำหนดให้ห้องนี้</p>
            </div>
          ) : (
            <div className="space-y-6">
              {/* Error Message */}
              {error && (
                <div className="flex items-center gap-2 p-3 bg-red-500/10 border border-red-500 rounded-lg text-red-600 text-sm">
                  <AlertTriangle className="w-4 h-4 flex-shrink-0" />
                  {error}
                </div>
              )}

              {/* Summary */}
              <div className="flex items-center gap-4 p-3 bg-gray-100 rounded-lg">
                <div className="flex items-center gap-2">
                  <Package className="w-5 h-5 text-gray-500" />
                  <span className="text-gray-500">สินค้าทั้งหมด: {items.length} รายการ</span>
                </div>
                {missingCount > 0 && (
                  <div className="flex items-center gap-2 text-amber-400">
                    <AlertTriangle className="w-4 h-4" />
                    <span>ขาด {missingCount} รายการ</span>
                  </div>
                )}
              </div>

              {/* Items grouped by category */}
              {Object.entries(groupedItems).map(([category, categoryItems]) => (
                <div key={category} className="space-y-2">
                  <h3 className="font-medium text-gray-700 flex items-center gap-2">
                    {getCategoryLabel(category)}
                    <span className="text-sm text-gray-500">({categoryItems.length})</span>
                  </h3>
                  <div className="space-y-2">
                    {categoryItems.map((item) => {
                      const isMissing = !item.verified || item.actualQuantity < item.assignedQuantity
                      return (
                        <div
                          key={item.itemId}
                          className={`flex items-center justify-between p-3 rounded-lg border ${
                            isMissing
                              ? 'border-amber-500 bg-amber-500/10'
                              : 'border-green-500 bg-green-500/10'
                          }`}
                        >
                          <div className="flex items-center gap-3">
                            <button
                              type="button"
                              onClick={() => handleVerifiedChange(item.itemId, !item.verified)}
                              className={`w-6 h-6 rounded border-2 flex items-center justify-center transition-colors ${
                                item.verified
                                  ? 'border-green-500 bg-green-500 text-white'
                                  : 'border-gray-300 bg-white'
                              }`}
                            >
                              {item.verified && <Check className="w-4 h-4" />}
                            </button>
                            <div>
                              <div className="font-medium text-gray-800">{item.itemName}</div>
                              <div className="text-xs text-gray-500">
                                {item.itemCode} - กำหนด: {item.assignedQuantity} {item.unit}
                              </div>
                            </div>
                          </div>
                          <div className="flex items-center gap-2">
                            <input
                              type="number"
                              value={item.actualQuantity}
                              onChange={(e) =>
                                handleQuantityChange(item.itemId, parseInt(e.target.value) || 0)
                              }
                              min="0"
                              max={item.assignedQuantity * 2}
                              className="w-16 px-2 py-1 text-center bg-gray-100 border border-gray-300 text-gray-800 rounded focus:ring-2 focus:ring-red-500 focus:border-red-500"
                            />
                            <span className="text-sm text-gray-500">{item.unit}</span>
                            {isMissing ? (
                              <XCircle className="w-5 h-5 text-amber-400" />
                            ) : (
                              <CheckCircle className="w-5 h-5 text-green-500" />
                            )}
                          </div>
                        </div>
                      )
                    })}
                  </div>
                </div>
              ))}

              {/* Notes */}
              <div>
                <label className="block text-sm font-medium text-gray-700 mb-1">
                  <FileText size={16} className="inline mr-1" />
                  หมายเหตุ
                </label>
                <textarea
                  value={notes}
                  onChange={(e) => setNotes(e.target.value)}
                  placeholder="หมายเหตุเพิ่มเติม..."
                  rows={2}
                  className="w-full px-3 py-2 bg-gray-100 border border-gray-300 text-gray-800 rounded-lg focus:ring-2 focus:ring-red-500 focus:border-red-500 resize-none"
                />
              </div>
            </div>
          )}
        </div>

        {/* Footer */}
        {items.length > 0 && (
          <div className="p-4 border-t border-gray-200 bg-gray-100 flex items-center justify-between gap-3">
            <button
              type="button"
              onClick={onClose}
              className="px-4 py-2 border border-gray-300 text-gray-700 rounded-lg hover:bg-gray-100"
              disabled={submitting}
            >
              ยกเลิก
            </button>
            <div className="flex items-center gap-2">
              {missingCount > 0 && (
                <button
                  type="button"
                  onClick={handleReplenish}
                  disabled={submitting}
                  className="flex items-center gap-2 px-4 py-2 bg-amber-600 text-white rounded-lg hover:bg-amber-700 disabled:opacity-50"
                >
                  {submitting ? (
                    <Loader2 className="w-4 h-4 animate-spin" />
                  ) : (
                    <RefreshCw className="w-4 h-4" />
                  )}
                  เติมสินค้าที่ขาด
                </button>
              )}
              <button
                type="button"
                onClick={handleSubmit}
                disabled={submitting}
                className="flex items-center gap-2 px-4 py-2 bg-red-600 text-white rounded-lg hover:bg-red-700 disabled:opacity-50"
              >
                {submitting ? (
                  <Loader2 className="w-4 h-4 animate-spin" />
                ) : (
                  <Check className="w-4 h-4" />
                )}
                บันทึกการตรวจสอบ
              </button>
            </div>
          </div>
        )}
      </div>
    </div>
  )
}
