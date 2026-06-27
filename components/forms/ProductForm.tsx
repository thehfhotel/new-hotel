'use client'

import { useState, useEffect } from 'react'
import {
  X,
  Package,
  Loader2,
  AlertCircle,
  Save,
  Hash,
  DollarSign,
  Boxes,
  Tag,
} from 'lucide-react'

export interface ProductFormData {
  id?: number
  legacyNo: string
  name: string
  unit: string
  price: number
  currentStock: number
  category: string
  active: boolean
}

interface ProductFormProps {
  isOpen: boolean
  onClose: () => void
  onSave: (data: ProductFormData) => Promise<void>
  initialData?: ProductFormData | null
  mode: 'create' | 'edit'
}

const emptyFormData: ProductFormData = {
  legacyNo: '',
  name: '',
  unit: '',
  price: 0,
  currentStock: 0,
  category: '',
  active: true,
}

export default function ProductForm({
  isOpen,
  onClose,
  onSave,
  initialData,
  mode,
}: ProductFormProps) {
  const [formData, setFormData] = useState<ProductFormData>(emptyFormData)
  const [saving, setSaving] = useState(false)
  const [error, setError] = useState<string | null>(null)

  useEffect(() => {
    if (isOpen) {
      setFormData(initialData ?? emptyFormData)
      setError(null)
    }
  }, [isOpen, initialData])

  const handleInputChange = (
    e: React.ChangeEvent<HTMLInputElement | HTMLSelectElement>
  ) => {
    const { name, value, type } = e.target
    if (type === 'checkbox') {
      const checked = (e.target as HTMLInputElement).checked
      setFormData((prev) => ({ ...prev, [name]: checked }))
    } else if (type === 'number') {
      setFormData((prev) => ({ ...prev, [name]: value === '' ? 0 : parseFloat(value) }))
    } else {
      setFormData((prev) => ({ ...prev, [name]: value }))
    }
  }

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault()
    setError(null)

    if (mode === 'create' && !formData.legacyNo.trim()) {
      setError('กรุณากรอกรหัสสินค้า')
      return
    }
    if (!formData.name.trim()) {
      setError('กรุณากรอกชื่อสินค้า')
      return
    }
    if (formData.price < 0) {
      setError('ราคาต้องไม่ติดลบ')
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

  if (!isOpen) return null

  return (
    <>
      <div className="fixed inset-0 bg-black/30 z-40" onClick={onClose} />
      <div className="fixed inset-0 z-50 flex items-center justify-center p-4">
        <div className="bg-white rounded-lg shadow-xl border border-gray-200 w-full max-w-lg max-h-[90vh] overflow-hidden">
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
            <div className="p-4 space-y-4 overflow-y-auto max-h-[65vh]">
              {error && (
                <div className="flex items-center gap-2 p-3 bg-red-50 border border-red-200 rounded-lg text-red-600">
                  <AlertCircle className="w-5 h-5 shrink-0" />
                  <span className="text-sm">{error}</span>
                </div>
              )}

              {/* Legacy No (business key — create only) */}
              <div>
                <label className="flex items-center gap-2 text-sm font-medium text-gray-700 mb-1">
                  <Hash className="w-4 h-4" />
                  รหัสสินค้า {mode === 'create' && <span className="text-red-500">*</span>}
                </label>
                <input
                  type="text"
                  name="legacyNo"
                  value={formData.legacyNo}
                  onChange={handleInputChange}
                  placeholder="เช่น P001"
                  maxLength={50}
                  disabled={mode === 'edit'}
                  className="w-full px-3 py-2 bg-gray-100 border border-gray-300 text-gray-800 rounded-lg focus:ring-2 focus:ring-red-500 focus:border-red-500 outline-hidden transition-colors disabled:opacity-60 disabled:cursor-not-allowed"
                  required={mode === 'create'}
                />
                {mode === 'edit' && (
                  <p className="mt-1 text-xs text-gray-500">รหัสสินค้าแก้ไขไม่ได้ (เป็นคีย์อ้างอิงกับ iHOTEL)</p>
                )}
              </div>

              {/* Name */}
              <div>
                <label className="flex items-center gap-2 text-sm font-medium text-gray-700 mb-1">
                  <Package className="w-4 h-4" />
                  ชื่อสินค้า <span className="text-red-500">*</span>
                </label>
                <input
                  type="text"
                  name="name"
                  value={formData.name}
                  onChange={handleInputChange}
                  placeholder="กรอกชื่อสินค้า"
                  className="w-full px-3 py-2 bg-gray-100 border border-gray-300 text-gray-800 rounded-lg focus:ring-2 focus:ring-red-500 focus:border-red-500 outline-hidden transition-colors"
                  required
                />
              </div>

              {/* Price + Unit */}
              <div className="grid grid-cols-2 gap-4">
                <div>
                  <label className="flex items-center gap-2 text-sm font-medium text-gray-700 mb-1">
                    <DollarSign className="w-4 h-4" />
                    ราคา (บาท)
                  </label>
                  <input
                    type="number"
                    name="price"
                    value={formData.price}
                    onChange={handleInputChange}
                    min="0"
                    step="0.01"
                    className="w-full px-3 py-2 bg-gray-100 border border-gray-300 text-gray-800 rounded-lg focus:ring-2 focus:ring-red-500 focus:border-red-500 outline-hidden transition-colors"
                  />
                </div>
                <div>
                  <label className="text-sm font-medium text-gray-700 mb-1 block">หน่วย</label>
                  <input
                    type="text"
                    name="unit"
                    value={formData.unit}
                    onChange={handleInputChange}
                    placeholder="เช่น ขวด, กระป๋อง"
                    maxLength={50}
                    className="w-full px-3 py-2 bg-gray-100 border border-gray-300 text-gray-800 rounded-lg focus:ring-2 focus:ring-red-500 focus:border-red-500 outline-hidden transition-colors"
                  />
                </div>
              </div>

              {/* Opening stock (create only — edits go through stock-adjust) */}
              {mode === 'create' && (
                <div>
                  <label className="flex items-center gap-2 text-sm font-medium text-gray-700 mb-1">
                    <Boxes className="w-4 h-4" />
                    สต็อกเริ่มต้น
                  </label>
                  <input
                    type="number"
                    name="currentStock"
                    value={formData.currentStock}
                    onChange={handleInputChange}
                    min="0"
                    step="0.001"
                    className="w-full px-3 py-2 bg-gray-100 border border-gray-300 text-gray-800 rounded-lg focus:ring-2 focus:ring-red-500 focus:border-red-500 outline-hidden transition-colors"
                  />
                  <p className="mt-1 text-xs text-gray-500">หลังจากนี้ปรับสต็อกผ่านปุ่ม &quot;ปรับสต็อก&quot; (ซิงค์กับ iHOTEL)</p>
                </div>
              )}

              {/* Category */}
              <div>
                <label className="flex items-center gap-2 text-sm font-medium text-gray-700 mb-1">
                  <Tag className="w-4 h-4" />
                  หมวดหมู่
                </label>
                <input
                  type="text"
                  name="category"
                  value={formData.category}
                  onChange={handleInputChange}
                  placeholder="เช่น เครื่องดื่ม, ขนม"
                  maxLength={100}
                  className="w-full px-3 py-2 bg-gray-100 border border-gray-300 text-gray-800 rounded-lg focus:ring-2 focus:ring-red-500 focus:border-red-500 outline-hidden transition-colors"
                />
              </div>

              {/* Active */}
              <div className="flex items-center justify-between p-3 bg-gray-100 rounded-lg">
                <label className="text-sm font-medium text-gray-700">เปิดใช้งาน</label>
                <label className="relative inline-flex items-center cursor-pointer">
                  <input
                    type="checkbox"
                    name="active"
                    checked={formData.active}
                    onChange={handleInputChange}
                    className="sr-only peer"
                  />
                  <div className="w-11 h-6 bg-gray-200 peer-focus:outline-hidden peer-focus:ring-4 peer-focus:ring-red-500/30 rounded-full peer peer-checked:after:translate-x-full peer-checked:after:border-white after:content-[''] after:absolute after:top-[2px] after:inset-s-[2px] after:bg-white after:border-gray-400 after:border after:rounded-full after:h-5 after:w-5 after:transition-all peer-checked:bg-red-600"></div>
                </label>
              </div>
            </div>

            {/* Footer */}
            <div className="p-4 border-t border-gray-200 bg-gray-100 flex items-center justify-end gap-3">
              <button
                type="button"
                onClick={onClose}
                className="px-4 py-2 bg-gray-100 hover:bg-gray-200 text-gray-800 rounded-lg transition-colors"
              >
                ยกเลิก
              </button>
              <button
                type="submit"
                disabled={saving}
                className="flex items-center gap-2 px-4 py-2 bg-red-600 text-white rounded-lg hover:bg-red-700 disabled:opacity-50 transition-colors"
              >
                {saving ? <Loader2 className="w-4 h-4 animate-spin" /> : <Save className="w-4 h-4" />}
                {mode === 'create' ? 'เพิ่มสินค้า' : 'บันทึก'}
              </button>
            </div>
          </form>
        </div>
      </div>
    </>
  )
}
