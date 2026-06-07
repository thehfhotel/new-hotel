'use client'

import { useState, useEffect } from 'react'
import {
  X,
  Package,
  Loader2,
  AlertCircle,
  Save,
  Trash2,
  Hash,
  Tag,
  Layers,
  DollarSign,
} from 'lucide-react'
import {
  InventoryItemFormData,
  INVENTORY_CATEGORIES,
} from '@/types/inventory'

interface InventoryItemFormProps {
  isOpen: boolean
  onClose: () => void
  onSave: (data: InventoryItemFormData) => Promise<void>
  onDelete?: (id: number) => Promise<void>
  initialData?: InventoryItemFormData | null
  mode: 'create' | 'edit'
}

const emptyFormData: InventoryItemFormData = {
  itemCode: '',
  itemName: '',
  category: 'Amenities',
  unit: '',
  minStock: 0,
  currentStock: 0,
  costPerUnit: null,
  active: true,
}

const unitOptions = [
  { value: 'ชิ้น', label: 'ชิ้น' },
  { value: 'ขวด', label: 'ขวด' },
  { value: 'กล่อง', label: 'กล่อง' },
  { value: 'ชุด', label: 'ชุด' },
  { value: 'ผืน', label: 'ผืน' },
  { value: 'ม้วน', label: 'ม้วน' },
  { value: 'แพ็ค', label: 'แพ็ค' },
  { value: 'อัน', label: 'อัน' },
]

export default function InventoryItemForm({
  isOpen,
  onClose,
  onSave,
  onDelete,
  initialData,
  mode,
}: InventoryItemFormProps) {
  const [formData, setFormData] = useState<InventoryItemFormData>(emptyFormData)
  const [saving, setSaving] = useState(false)
  const [deleting, setDeleting] = useState(false)
  const [error, setError] = useState<string | null>(null)
  const [showDeleteConfirm, setShowDeleteConfirm] = useState(false)

  // Reset form when modal opens/closes or initialData changes
  useEffect(() => {
    if (isOpen) {
      if (initialData) {
        setFormData(initialData)
      } else {
        setFormData(emptyFormData)
      }
      setError(null)
      setShowDeleteConfirm(false)
    }
  }, [isOpen, initialData])

  const handleInputChange = (
    e: React.ChangeEvent<HTMLInputElement | HTMLSelectElement>
  ) => {
    const { name, value, type } = e.target

    if (type === 'checkbox') {
      const checked = (e.target as HTMLInputElement).checked
      setFormData((prev) => ({
        ...prev,
        [name]: checked,
      }))
    } else if (type === 'number') {
      setFormData((prev) => ({
        ...prev,
        [name]: value === '' ? (name === 'costPerUnit' ? null : 0) : parseFloat(value),
      }))
    } else {
      setFormData((prev) => ({
        ...prev,
        [name]: value,
      }))
    }
  }

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault()
    setError(null)

    // Validation
    if (!formData.itemCode.trim()) {
      setError('กรุณากรอกรหัสสินค้า')
      return
    }

    if (!formData.itemName.trim()) {
      setError('กรุณากรอกชื่อสินค้า')
      return
    }

    if (!formData.unit.trim()) {
      setError('กรุณาเลือกหน่วยนับ')
      return
    }

    if (formData.minStock < 0) {
      setError('จำนวนขั้นต่ำต้องไม่ติดลบ')
      return
    }

    if (formData.currentStock < 0) {
      setError('จำนวนคงเหลือต้องไม่ติดลบ')
      return
    }

    setSaving(true)
    try {
      await onSave(formData)
      onClose()
    } catch (err) {
      setError(err instanceof Error ? err.message : 'เกิดข้อผิดพลาดในการบันทึก')
    } finally {
      setSaving(false)
    }
  }

  const handleDelete = async () => {
    if (!initialData?.id || !onDelete) return

    setDeleting(true)
    setError(null)
    try {
      await onDelete(initialData.id)
      onClose()
    } catch (err) {
      setError(err instanceof Error ? err.message : 'เกิดข้อผิดพลาดในการลบ')
    } finally {
      setDeleting(false)
      setShowDeleteConfirm(false)
    }
  }

  if (!isOpen) return null

  return (
    <>
      {/* Backdrop */}
      <div
        className="fixed inset-0 bg-black/30 z-40"
        onClick={onClose}
      />

      {/* Modal */}
      <div className="fixed inset-0 z-50 flex items-center justify-center p-4">
        <div className="bg-white border border-gray-200 rounded-lg shadow-2xl w-full max-w-lg max-h-[90vh] overflow-hidden">
          {/* Header */}
          <div className="flex items-center justify-between p-4 border-b border-gray-200 bg-gray-100">
            <div className="flex items-center gap-3">
              <div className="w-10 h-10 bg-red-500/10 rounded-full flex items-center justify-center">
                <Package className="w-5 h-5 text-red-600" />
              </div>
              <h2 className="text-xl font-bold text-gray-900">
                {mode === 'create' ? 'เพิ่มสินค้าใหม่' : 'แก้ไขสินค้า'}
              </h2>
            </div>
            <button
              onClick={onClose}
              className="p-2 hover:bg-gray-100 rounded-full transition-colors"
              aria-label="ปิด"
            >
              <X className="w-5 h-5" />
            </button>
          </div>

          {/* Form */}
          <form onSubmit={handleSubmit}>
            <div className="p-4 space-y-4 overflow-y-auto max-h-[60vh]">
              {/* Error Message */}
              {error && (
                <div className="flex items-center gap-2 p-3 bg-red-50 border border-red-200 rounded-lg text-red-600">
                  <AlertCircle className="w-5 h-5 shrink-0" />
                  <span className="text-sm">{error}</span>
                </div>
              )}

              {/* Item Code (Required) */}
              <div>
                <label className="flex items-center gap-2 text-sm font-medium text-gray-700 mb-1">
                  <Hash className="w-4 h-4" />
                  รหัสสินค้า <span className="text-red-500">*</span>
                </label>
                <input
                  type="text"
                  name="itemCode"
                  value={formData.itemCode}
                  onChange={handleInputChange}
                  placeholder="เช่น AMT-001, MNB-001"
                  maxLength={20}
                  className="w-full px-3 py-2 bg-gray-100 border border-gray-300 text-gray-800 rounded-lg focus:ring-2 focus:ring-red-500 focus:border-red-500 outline-hidden transition-colors uppercase"
                  required
                />
                <p className="mt-1 text-xs text-gray-500">รหัสสินค้า (ไม่เกิน 20 ตัวอักษร)</p>
              </div>

              {/* Item Name (Required) */}
              <div>
                <label className="flex items-center gap-2 text-sm font-medium text-gray-700 mb-1">
                  <Package className="w-4 h-4" />
                  ชื่อสินค้า <span className="text-red-500">*</span>
                </label>
                <input
                  type="text"
                  name="itemName"
                  value={formData.itemName}
                  onChange={handleInputChange}
                  placeholder="กรอกชื่อสินค้า"
                  className="w-full px-3 py-2 bg-gray-100 border border-gray-300 text-gray-800 rounded-lg focus:ring-2 focus:ring-red-500 focus:border-red-500 outline-hidden transition-colors"
                  required
                />
              </div>

              {/* Category */}
              <div>
                <label className="flex items-center gap-2 text-sm font-medium text-gray-700 mb-1">
                  <Layers className="w-4 h-4" />
                  หมวดหมู่ <span className="text-red-500">*</span>
                </label>
                <select
                  name="category"
                  value={formData.category}
                  onChange={handleInputChange}
                  className="w-full px-3 py-2 bg-gray-100 border border-gray-300 text-gray-800 rounded-lg focus:ring-2 focus:ring-red-500 focus:border-red-500 outline-hidden transition-colors"
                  required
                >
                  {INVENTORY_CATEGORIES.map((cat) => (
                    <option key={cat.value} value={cat.value}>
                      {cat.label} ({cat.labelTh})
                    </option>
                  ))}
                </select>
              </div>

              {/* Unit */}
              <div>
                <label className="flex items-center gap-2 text-sm font-medium text-gray-700 mb-1">
                  <Tag className="w-4 h-4" />
                  หน่วยนับ <span className="text-red-500">*</span>
                </label>
                <select
                  name="unit"
                  value={formData.unit}
                  onChange={handleInputChange}
                  className="w-full px-3 py-2 bg-gray-100 border border-gray-300 text-gray-800 rounded-lg focus:ring-2 focus:ring-red-500 focus:border-red-500 outline-hidden transition-colors"
                  required
                >
                  <option value="">-- เลือกหน่วยนับ --</option>
                  {unitOptions.map((unit) => (
                    <option key={unit.value} value={unit.value}>
                      {unit.label}
                    </option>
                  ))}
                </select>
              </div>

              {/* Min Stock & Current Stock */}
              <div className="grid grid-cols-2 gap-4">
                <div>
                  <label className="flex items-center gap-2 text-sm font-medium text-gray-700 mb-1">
                    จำนวนขั้นต่ำ
                  </label>
                  <input
                    type="number"
                    name="minStock"
                    value={formData.minStock}
                    onChange={handleInputChange}
                    placeholder="0"
                    min="0"
                    className="w-full px-3 py-2 bg-gray-100 border border-gray-300 text-gray-800 rounded-lg focus:ring-2 focus:ring-red-500 focus:border-red-500 outline-hidden transition-colors"
                  />
                </div>
                <div>
                  <label className="flex items-center gap-2 text-sm font-medium text-gray-700 mb-1">
                    จำนวนคงเหลือ
                  </label>
                  <input
                    type="number"
                    name="currentStock"
                    value={formData.currentStock}
                    onChange={handleInputChange}
                    placeholder="0"
                    min="0"
                    className="w-full px-3 py-2 bg-gray-100 border border-gray-300 text-gray-800 rounded-lg focus:ring-2 focus:ring-red-500 focus:border-red-500 outline-hidden transition-colors"
                  />
                </div>
              </div>

              {/* Cost per Unit (Optional) */}
              <div>
                <label className="flex items-center gap-2 text-sm font-medium text-gray-700 mb-1">
                  <DollarSign className="w-4 h-4" />
                  ต้นทุนต่อหน่วย (บาท)
                </label>
                <input
                  type="number"
                  name="costPerUnit"
                  value={formData.costPerUnit ?? ''}
                  onChange={handleInputChange}
                  placeholder="0"
                  min="0"
                  step="0.01"
                  className="w-full px-3 py-2 bg-gray-100 border border-gray-300 text-gray-800 rounded-lg focus:ring-2 focus:ring-red-500 focus:border-red-500 outline-hidden transition-colors"
                />
              </div>

              {/* Active Toggle */}
              <div className="flex items-center justify-between p-3 bg-gray-100 rounded-lg">
                <label className="text-sm font-medium text-gray-700">
                  เปิดใช้งาน
                </label>
                <label className="relative inline-flex items-center cursor-pointer">
                  <input
                    type="checkbox"
                    name="active"
                    checked={formData.active}
                    onChange={handleInputChange}
                    className="sr-only peer"
                  />
                  <div className="w-11 h-6 bg-gray-100 peer-focus:outline-hidden peer-focus:ring-4 peer-focus:ring-red-500/50 rounded-full peer peer-checked:after:translate-x-full peer-checked:rtl:after:-translate-x-full peer-checked:after:border-white after:content-[''] after:absolute after:top-[2px] after:inset-s-[2px] after:bg-white after:border-gray-300 after:border after:rounded-full after:h-5 after:w-5 after:transition-all peer-checked:bg-red-600"></div>
                </label>
              </div>
            </div>

            {/* Footer */}
            <div className="p-4 border-t border-gray-200 bg-gray-100 flex items-center justify-between gap-3">
              {/* Delete Button (only in edit mode) */}
              {mode === 'edit' && onDelete && initialData?.id && (
                <div>
                  {showDeleteConfirm ? (
                    <div className="flex items-center gap-2">
                      <span className="text-sm text-red-600">ยืนยันการลบ?</span>
                      <button
                        type="button"
                        onClick={handleDelete}
                        disabled={deleting}
                        className="px-3 py-1.5 bg-red-600 text-white text-sm rounded-lg hover:bg-red-700 disabled:opacity-50 transition-colors"
                      >
                        {deleting ? (
                          <Loader2 className="w-4 h-4 animate-spin" />
                        ) : (
                          'ใช่'
                        )}
                      </button>
                      <button
                        type="button"
                        onClick={() => setShowDeleteConfirm(false)}
                        className="px-3 py-1.5 bg-gray-100 text-gray-700 text-sm rounded-lg hover:bg-gray-100 transition-colors"
                      >
                        ไม่
                      </button>
                    </div>
                  ) : (
                    <button
                      type="button"
                      onClick={() => setShowDeleteConfirm(true)}
                      className="flex items-center gap-2 px-4 py-2 text-red-600 hover:bg-red-50 rounded-lg transition-colors"
                    >
                      <Trash2 className="w-4 h-4" />
                      ลบ
                    </button>
                  )}
                </div>
              )}

              {/* Spacer when no delete button */}
              {(mode === 'create' || !onDelete) && <div />}

              {/* Save/Cancel Buttons */}
              <div className="flex items-center gap-3">
                <button
                  type="button"
                  onClick={onClose}
                  className="px-4 py-2 bg-gray-100 hover:bg-gray-100 text-gray-700 rounded-lg transition-colors"
                >
                  ยกเลิก
                </button>
                <button
                  type="submit"
                  disabled={saving}
                  className="flex items-center gap-2 px-4 py-2 bg-red-600 text-white rounded-lg hover:bg-red-700 disabled:opacity-50 transition-colors"
                >
                  {saving ? (
                    <Loader2 className="w-4 h-4 animate-spin" />
                  ) : (
                    <Save className="w-4 h-4" />
                  )}
                  {mode === 'create' ? 'เพิ่มสินค้า' : 'บันทึก'}
                </button>
              </div>
            </div>
          </form>
        </div>
      </div>
    </>
  )
}
