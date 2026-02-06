'use client'

import { useState, useEffect } from 'react'
import {
  X,
  Home,
  Loader2,
  AlertCircle,
  Save,
  Trash2,
  Hash,
  DollarSign,
  Users,
  Bed,
  Maximize,
  FileText,
  Globe,
} from 'lucide-react'

export interface RoomTypeFormData {
  id?: number
  typeCode: string
  typeName: string
  typeNameEn: string
  description: string
  basePrice: number
  maxGuests: number
  bedType: string
  sizeSqm: number | null
  active: boolean
}

interface RoomTypeFormProps {
  isOpen: boolean
  onClose: () => void
  onSave: (data: RoomTypeFormData) => Promise<void>
  onDelete?: (id: number) => Promise<void>
  initialData?: RoomTypeFormData | null
  mode: 'create' | 'edit'
}

const emptyFormData: RoomTypeFormData = {
  typeCode: '',
  typeName: '',
  typeNameEn: '',
  description: '',
  basePrice: 0,
  maxGuests: 2,
  bedType: '',
  sizeSqm: null,
  active: true,
}

const bedTypes = [
  { value: 'Single', label: 'Single (เตียงเดี่ยว)' },
  { value: 'Double', label: 'Double (เตียงคู่)' },
  { value: 'Twin', label: 'Twin (เตียงแฝด)' },
  { value: 'King', label: 'King (เตียงคิงไซส์)' },
]

export default function RoomTypeForm({
  isOpen,
  onClose,
  onSave,
  onDelete,
  initialData,
  mode,
}: RoomTypeFormProps) {
  const [formData, setFormData] = useState<RoomTypeFormData>(emptyFormData)
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
    e: React.ChangeEvent<HTMLInputElement | HTMLTextAreaElement | HTMLSelectElement>
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
        [name]: value === '' ? (name === 'sizeSqm' ? null : 0) : parseFloat(value),
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
    if (!formData.typeCode.trim()) {
      setError('กรุณากรอกรหัสประเภทห้อง')
      return
    }

    if (!formData.typeName.trim()) {
      setError('กรุณากรอกชื่อประเภทห้อง')
      return
    }

    if (formData.basePrice < 0) {
      setError('ราคาพื้นฐานต้องไม่ติดลบ')
      return
    }

    if (formData.maxGuests < 1) {
      setError('จำนวนผู้เข้าพักสูงสุดต้องมากกว่า 0')
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
        className="fixed inset-0 bg-black/60 z-40"
        onClick={onClose}
      />

      {/* Modal */}
      <div className="fixed inset-0 z-50 flex items-center justify-center p-4">
        <div className="bg-zinc-900 rounded-lg shadow-xl border border-zinc-800 w-full max-w-lg max-h-[90vh] overflow-hidden">
          {/* Header */}
          <div className="flex items-center justify-between p-4 border-b border-zinc-800 bg-zinc-800">
            <div className="flex items-center gap-3">
              <div className="w-10 h-10 bg-red-500/10 rounded-full flex items-center justify-center">
                <Home className="w-5 h-5 text-red-400" />
              </div>
              <h2 className="text-xl font-bold text-zinc-100">
                {mode === 'create' ? 'เพิ่มประเภทห้องใหม่' : 'แก้ไขประเภทห้อง'}
              </h2>
            </div>
            <button
              onClick={onClose}
              className="p-2 hover:bg-zinc-800 rounded-full transition-colors"
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
                <div className="flex items-center gap-2 p-3 bg-red-950/50 border border-red-900/50 rounded-lg text-red-400">
                  <AlertCircle className="w-5 h-5 flex-shrink-0" />
                  <span className="text-sm">{error}</span>
                </div>
              )}

              {/* Type Code (Required) */}
              <div>
                <label className="flex items-center gap-2 text-sm font-medium text-zinc-300 mb-1">
                  <Hash className="w-4 h-4" />
                  รหัส <span className="text-red-500">*</span>
                </label>
                <input
                  type="text"
                  name="typeCode"
                  value={formData.typeCode}
                  onChange={handleInputChange}
                  placeholder="เช่น STD, DLX, SUI"
                  maxLength={20}
                  className="w-full px-3 py-2 bg-zinc-800 border border-zinc-700 text-zinc-200 rounded-lg focus:ring-2 focus:ring-red-500 focus:border-red-500 outline-none transition-colors uppercase"
                  required
                />
                <p className="mt-1 text-xs text-zinc-500">รหัสประเภทห้อง (ไม่เกิน 20 ตัวอักษร)</p>
              </div>

              {/* Type Name Thai (Required) */}
              <div>
                <label className="flex items-center gap-2 text-sm font-medium text-zinc-300 mb-1">
                  <Home className="w-4 h-4" />
                  ชื่อ <span className="text-red-500">*</span>
                </label>
                <input
                  type="text"
                  name="typeName"
                  value={formData.typeName}
                  onChange={handleInputChange}
                  placeholder="กรอกชื่อประเภทห้อง"
                  className="w-full px-3 py-2 bg-zinc-800 border border-zinc-700 text-zinc-200 rounded-lg focus:ring-2 focus:ring-red-500 focus:border-red-500 outline-none transition-colors"
                  required
                />
              </div>

              {/* Type Name English */}
              <div>
                <label className="flex items-center gap-2 text-sm font-medium text-zinc-300 mb-1">
                  <Globe className="w-4 h-4" />
                  ชื่อภาษาอังกฤษ
                </label>
                <input
                  type="text"
                  name="typeNameEn"
                  value={formData.typeNameEn}
                  onChange={handleInputChange}
                  placeholder="Enter English name"
                  className="w-full px-3 py-2 bg-zinc-800 border border-zinc-700 text-zinc-200 rounded-lg focus:ring-2 focus:ring-red-500 focus:border-red-500 outline-none transition-colors"
                />
              </div>

              {/* Description */}
              <div>
                <label className="flex items-center gap-2 text-sm font-medium text-zinc-300 mb-1">
                  <FileText className="w-4 h-4" />
                  รายละเอียด
                </label>
                <textarea
                  name="description"
                  value={formData.description}
                  onChange={handleInputChange}
                  placeholder="กรอกรายละเอียดประเภทห้อง"
                  rows={3}
                  className="w-full px-3 py-2 bg-zinc-800 border border-zinc-700 text-zinc-200 rounded-lg focus:ring-2 focus:ring-red-500 focus:border-red-500 outline-none transition-colors resize-none"
                />
              </div>

              {/* Base Price */}
              <div>
                <label className="flex items-center gap-2 text-sm font-medium text-zinc-300 mb-1">
                  <DollarSign className="w-4 h-4" />
                  ราคาพื้นฐาน (บาท)
                </label>
                <input
                  type="number"
                  name="basePrice"
                  value={formData.basePrice}
                  onChange={handleInputChange}
                  placeholder="0"
                  min="0"
                  step="0.01"
                  className="w-full px-3 py-2 bg-zinc-800 border border-zinc-700 text-zinc-200 rounded-lg focus:ring-2 focus:ring-red-500 focus:border-red-500 outline-none transition-colors"
                />
              </div>

              {/* Max Guests */}
              <div>
                <label className="flex items-center gap-2 text-sm font-medium text-zinc-300 mb-1">
                  <Users className="w-4 h-4" />
                  จำนวนผู้เข้าพักสูงสุด
                </label>
                <input
                  type="number"
                  name="maxGuests"
                  value={formData.maxGuests}
                  onChange={handleInputChange}
                  placeholder="2"
                  min="1"
                  max="20"
                  className="w-full px-3 py-2 bg-zinc-800 border border-zinc-700 text-zinc-200 rounded-lg focus:ring-2 focus:ring-red-500 focus:border-red-500 outline-none transition-colors"
                />
              </div>

              {/* Bed Type */}
              <div>
                <label className="flex items-center gap-2 text-sm font-medium text-zinc-300 mb-1">
                  <Bed className="w-4 h-4" />
                  ประเภทเตียง
                </label>
                <select
                  name="bedType"
                  value={formData.bedType}
                  onChange={handleInputChange}
                  className="w-full px-3 py-2 bg-zinc-800 border border-zinc-700 text-zinc-200 rounded-lg focus:ring-2 focus:ring-red-500 focus:border-red-500 outline-none transition-colors"
                >
                  <option value="">-- เลือกประเภทเตียง --</option>
                  {bedTypes.map((bed) => (
                    <option key={bed.value} value={bed.value}>
                      {bed.label}
                    </option>
                  ))}
                </select>
              </div>

              {/* Size SQM */}
              <div>
                <label className="flex items-center gap-2 text-sm font-medium text-zinc-300 mb-1">
                  <Maximize className="w-4 h-4" />
                  ขนาดห้อง (ตร.ม.)
                </label>
                <input
                  type="number"
                  name="sizeSqm"
                  value={formData.sizeSqm ?? ''}
                  onChange={handleInputChange}
                  placeholder="0"
                  min="0"
                  step="0.01"
                  className="w-full px-3 py-2 bg-zinc-800 border border-zinc-700 text-zinc-200 rounded-lg focus:ring-2 focus:ring-red-500 focus:border-red-500 outline-none transition-colors"
                />
              </div>

              {/* Active Toggle */}
              <div className="flex items-center justify-between p-3 bg-zinc-800 rounded-lg">
                <label className="text-sm font-medium text-zinc-300">
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
                  <div className="w-11 h-6 bg-zinc-700 peer-focus:outline-none peer-focus:ring-4 peer-focus:ring-red-500/30 rounded-full peer peer-checked:after:translate-x-full rtl:peer-checked:after:-translate-x-full peer-checked:after:border-white after:content-[''] after:absolute after:top-[2px] after:start-[2px] after:bg-white after:border-zinc-600 after:border after:rounded-full after:h-5 after:w-5 after:transition-all peer-checked:bg-red-600"></div>
                </label>
              </div>
            </div>

            {/* Footer */}
            <div className="p-4 border-t border-zinc-800 bg-zinc-800 flex items-center justify-between gap-3">
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
                        className="px-3 py-1.5 bg-zinc-800 text-zinc-200 text-sm rounded-lg hover:bg-zinc-800 transition-colors"
                      >
                        ไม่
                      </button>
                    </div>
                  ) : (
                    <button
                      type="button"
                      onClick={() => setShowDeleteConfirm(true)}
                      className="flex items-center gap-2 px-4 py-2 text-red-400 hover:bg-red-500/10 rounded-lg transition-colors"
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
                  className="px-4 py-2 bg-zinc-800 hover:bg-zinc-800 text-zinc-200 rounded-lg transition-colors"
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
                  {mode === 'create' ? 'เพิ่มประเภทห้อง' : 'บันทึก'}
                </button>
              </div>
            </div>
          </form>
        </div>
      </div>
    </>
  )
}
