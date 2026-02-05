'use client'

import { useState, useEffect } from 'react'
import {
  X,
  Percent,
  Loader2,
  AlertCircle,
  Save,
  Trash2,
  Tag,
  Calendar,
  DollarSign,
  ToggleLeft,
  Home,
} from 'lucide-react'

export interface RateFormData {
  id?: number
  rateName: string
  roomTypeId: number | null
  rateType: 'multiplier' | 'fixed'
  rateValue: number
  validFrom: string
  validTo: string
  daysOfWeek: number[] // 0=Sun, 1=Mon, ..., 6=Sat
  active: boolean
}

interface RoomType {
  id: number
  typeCode: string
  typeName: string
  basePrice: number
}

interface RateFormProps {
  isOpen: boolean
  onClose: () => void
  onSave: (data: RateFormData) => Promise<void>
  onDelete?: (id: number) => Promise<void>
  rate?: RateFormData | null
  roomTypes: RoomType[]
}

const DAYS_OF_WEEK = [
  { value: 0, label: 'อา', fullLabel: 'อาทิตย์' },
  { value: 1, label: 'จ', fullLabel: 'จันทร์' },
  { value: 2, label: 'อ', fullLabel: 'อังคาร' },
  { value: 3, label: 'พ', fullLabel: 'พุธ' },
  { value: 4, label: 'พฤ', fullLabel: 'พฤหัสบดี' },
  { value: 5, label: 'ศ', fullLabel: 'ศุกร์' },
  { value: 6, label: 'ส', fullLabel: 'เสาร์' },
]

const emptyFormData: RateFormData = {
  rateName: '',
  roomTypeId: null,
  rateType: 'multiplier',
  rateValue: 1.0,
  validFrom: '',
  validTo: '',
  daysOfWeek: [0, 1, 2, 3, 4, 5, 6],
  active: true,
}

// Format date for API (YYYY-MM-DD)
function formatDateForInput(date: Date): string {
  const year = date.getFullYear()
  const month = String(date.getMonth() + 1).padStart(2, '0')
  const day = String(date.getDate()).padStart(2, '0')
  return `${year}-${month}-${day}`
}

// Convert to Buddhist Era for display
function toBuddhistYear(dateStr: string): string {
  if (!dateStr) return ''
  const date = new Date(dateStr)
  const year = date.getFullYear() + 543
  const month = String(date.getMonth() + 1).padStart(2, '0')
  const day = String(date.getDate()).padStart(2, '0')
  return `${day}/${month}/${year}`
}

export default function RateForm({
  isOpen,
  onClose,
  onSave,
  onDelete,
  rate,
  roomTypes,
}: RateFormProps) {
  const [formData, setFormData] = useState<RateFormData>(emptyFormData)
  const [saving, setSaving] = useState(false)
  const [deleting, setDeleting] = useState(false)
  const [error, setError] = useState<string | null>(null)
  const [showDeleteConfirm, setShowDeleteConfirm] = useState(false)

  const mode = rate?.id ? 'edit' : 'create'

  // Reset form when modal opens/closes or rate changes
  useEffect(() => {
    if (isOpen) {
      if (rate) {
        setFormData(rate)
      } else {
        // Set default dates for new rate
        const today = new Date()
        const nextMonth = new Date()
        nextMonth.setMonth(nextMonth.getMonth() + 1)
        setFormData({
          ...emptyFormData,
          validFrom: formatDateForInput(today),
          validTo: formatDateForInput(nextMonth),
        })
      }
      setError(null)
      setShowDeleteConfirm(false)
    }
  }, [isOpen, rate])

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
    } else if (name === 'roomTypeId') {
      setFormData((prev) => ({
        ...prev,
        [name]: value === '' ? null : parseInt(value, 10),
      }))
    } else if (name === 'rateValue') {
      setFormData((prev) => ({
        ...prev,
        [name]: value === '' ? 0 : parseFloat(value),
      }))
    } else {
      setFormData((prev) => ({
        ...prev,
        [name]: value,
      }))
    }
  }

  const handleDayToggle = (day: number) => {
    setFormData((prev) => {
      const newDays = prev.daysOfWeek.includes(day)
        ? prev.daysOfWeek.filter((d) => d !== day)
        : [...prev.daysOfWeek, day].sort((a, b) => a - b)
      return { ...prev, daysOfWeek: newDays }
    })
  }

  const handleSelectAllDays = () => {
    setFormData((prev) => ({
      ...prev,
      daysOfWeek: [0, 1, 2, 3, 4, 5, 6],
    }))
  }

  const handleSelectWeekdays = () => {
    setFormData((prev) => ({
      ...prev,
      daysOfWeek: [1, 2, 3, 4, 5],
    }))
  }

  const handleSelectWeekends = () => {
    setFormData((prev) => ({
      ...prev,
      daysOfWeek: [0, 6],
    }))
  }

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault()
    setError(null)

    // Validation
    if (!formData.rateName.trim()) {
      setError('กรุณากรอกชื่ออัตรา')
      return
    }

    if (!formData.roomTypeId) {
      setError('กรุณาเลือกประเภทห้อง')
      return
    }

    if (!formData.validFrom) {
      setError('กรุณาเลือกวันที่เริ่มต้น')
      return
    }

    if (!formData.validTo) {
      setError('กรุณาเลือกวันที่สิ้นสุด')
      return
    }

    if (new Date(formData.validFrom) > new Date(formData.validTo)) {
      setError('วันที่เริ่มต้นต้องก่อนวันที่สิ้นสุด')
      return
    }

    if (formData.rateType === 'multiplier') {
      if (formData.rateValue < 0.1 || formData.rateValue > 2.0) {
        setError('ตัวคูณต้องอยู่ระหว่าง 0.1 ถึง 2.0')
        return
      }
    } else if (formData.rateType === 'fixed') {
      if (formData.rateValue <= 0) {
        setError('ราคาคงที่ต้องมากกว่า 0')
        return
      }
    }

    if (formData.daysOfWeek.length === 0) {
      setError('กรุณาเลือกวันที่มีผลอย่างน้อย 1 วัน')
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
    if (!rate?.id || !onDelete) return

    setDeleting(true)
    setError(null)
    try {
      await onDelete(rate.id)
      onClose()
    } catch (err) {
      setError(err instanceof Error ? err.message : 'เกิดข้อผิดพลาดในการลบ')
    } finally {
      setDeleting(false)
      setShowDeleteConfirm(false)
    }
  }

  // Calculate effective price preview
  const getEffectivePricePreview = () => {
    if (!formData.roomTypeId) return null
    const roomType = roomTypes.find((rt) => rt.id === formData.roomTypeId)
    if (!roomType) return null

    const basePrice = roomType.basePrice
    let effectivePrice: number

    if (formData.rateType === 'multiplier') {
      effectivePrice = basePrice * formData.rateValue
    } else {
      effectivePrice = formData.rateValue
    }

    const discount = basePrice - effectivePrice
    const discountPercent = basePrice > 0 ? ((discount / basePrice) * 100).toFixed(0) : '0'

    return {
      basePrice,
      effectivePrice,
      discount,
      discountPercent,
      isDiscount: discount > 0,
    }
  }

  const pricePreview = getEffectivePricePreview()

  if (!isOpen) return null

  return (
    <>
      {/* Backdrop */}
      <div
        className="fixed inset-0 bg-black bg-opacity-50 z-40"
        onClick={onClose}
      />

      {/* Modal */}
      <div className="fixed inset-0 z-50 flex items-center justify-center p-4">
        <div className="bg-white rounded-lg shadow-xl w-full max-w-lg max-h-[90vh] overflow-hidden">
          {/* Header */}
          <div className="flex items-center justify-between p-4 border-b bg-gray-50">
            <div className="flex items-center gap-3">
              <div className="w-10 h-10 bg-amber-100 rounded-full flex items-center justify-center">
                <Percent className="w-5 h-5 text-amber-600" />
              </div>
              <h2 className="text-xl font-bold text-gray-800">
                {mode === 'create' ? 'เพิ่มอัตราพิเศษ' : 'แก้ไขอัตราพิเศษ'}
              </h2>
            </div>
            <button
              onClick={onClose}
              className="p-2 hover:bg-gray-200 rounded-full transition-colors"
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
                <div className="flex items-center gap-2 p-3 bg-red-50 border border-red-200 rounded-lg text-red-700">
                  <AlertCircle className="w-5 h-5 flex-shrink-0" />
                  <span className="text-sm">{error}</span>
                </div>
              )}

              {/* Rate Name */}
              <div>
                <label className="flex items-center gap-2 text-sm font-medium text-gray-700 mb-1">
                  <Tag className="w-4 h-4" />
                  ชื่ออัตรา <span className="text-red-500">*</span>
                </label>
                <input
                  type="text"
                  name="rateName"
                  value={formData.rateName}
                  onChange={handleInputChange}
                  placeholder="เช่น Weekend Special, Low Season"
                  className="w-full px-3 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-amber-500 focus:border-amber-500 outline-none transition-colors"
                  required
                />
              </div>

              {/* Room Type */}
              <div>
                <label className="flex items-center gap-2 text-sm font-medium text-gray-700 mb-1">
                  <Home className="w-4 h-4" />
                  ประเภทห้อง <span className="text-red-500">*</span>
                </label>
                <select
                  name="roomTypeId"
                  value={formData.roomTypeId ?? ''}
                  onChange={handleInputChange}
                  className="w-full px-3 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-amber-500 focus:border-amber-500 outline-none transition-colors bg-white"
                  required
                >
                  <option value="">-- เลือกประเภทห้อง --</option>
                  {roomTypes.map((rt) => (
                    <option key={rt.id} value={rt.id}>
                      {rt.typeName} ({rt.typeCode}) - {rt.basePrice.toLocaleString()} บาท
                    </option>
                  ))}
                </select>
              </div>

              {/* Rate Type */}
              <div>
                <label className="flex items-center gap-2 text-sm font-medium text-gray-700 mb-1">
                  <Percent className="w-4 h-4" />
                  ประเภทอัตรา <span className="text-red-500">*</span>
                </label>
                <div className="flex gap-4">
                  <label className="flex items-center gap-2 cursor-pointer">
                    <input
                      type="radio"
                      name="rateType"
                      value="multiplier"
                      checked={formData.rateType === 'multiplier'}
                      onChange={handleInputChange}
                      className="w-4 h-4 text-amber-600 focus:ring-amber-500"
                    />
                    <span className="text-sm">ตัวคูณ (Multiplier)</span>
                  </label>
                  <label className="flex items-center gap-2 cursor-pointer">
                    <input
                      type="radio"
                      name="rateType"
                      value="fixed"
                      checked={formData.rateType === 'fixed'}
                      onChange={handleInputChange}
                      className="w-4 h-4 text-amber-600 focus:ring-amber-500"
                    />
                    <span className="text-sm">ราคาคงที่ (Fixed Price)</span>
                  </label>
                </div>
              </div>

              {/* Rate Value */}
              <div>
                <label className="flex items-center gap-2 text-sm font-medium text-gray-700 mb-1">
                  <DollarSign className="w-4 h-4" />
                  {formData.rateType === 'multiplier' ? 'ตัวคูณ' : 'ราคา (บาท)'}{' '}
                  <span className="text-red-500">*</span>
                </label>
                <input
                  type="number"
                  name="rateValue"
                  value={formData.rateValue}
                  onChange={handleInputChange}
                  placeholder={formData.rateType === 'multiplier' ? '0.8' : '1000'}
                  min={formData.rateType === 'multiplier' ? '0.1' : '1'}
                  max={formData.rateType === 'multiplier' ? '2.0' : undefined}
                  step={formData.rateType === 'multiplier' ? '0.01' : '1'}
                  className="w-full px-3 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-amber-500 focus:border-amber-500 outline-none transition-colors"
                  required
                />
                {formData.rateType === 'multiplier' && (
                  <p className="mt-1 text-xs text-gray-500">
                    เช่น 0.8 = ลด 20%, 1.2 = เพิ่ม 20% (ค่าอยู่ระหว่าง 0.1 - 2.0)
                  </p>
                )}
              </div>

              {/* Price Preview */}
              {pricePreview && (
                <div
                  className={`p-3 rounded-lg border ${
                    pricePreview.isDiscount
                      ? 'bg-green-50 border-green-200'
                      : 'bg-amber-50 border-amber-200'
                  }`}
                >
                  <p className="text-sm font-medium text-gray-700 mb-1">ตัวอย่างราคา:</p>
                  <div className="flex items-center gap-2">
                    <span className="text-gray-500 line-through">
                      {pricePreview.basePrice.toLocaleString()} บาท
                    </span>
                    <span className="text-lg font-bold text-gray-800">
                      {pricePreview.effectivePrice.toLocaleString()} บาท
                    </span>
                    {pricePreview.isDiscount && (
                      <span className="px-2 py-0.5 bg-green-500 text-white text-xs rounded-full">
                        -{pricePreview.discountPercent}%
                      </span>
                    )}
                    {!pricePreview.isDiscount && pricePreview.discount !== 0 && (
                      <span className="px-2 py-0.5 bg-amber-500 text-white text-xs rounded-full">
                        +{Math.abs(parseInt(pricePreview.discountPercent))}%
                      </span>
                    )}
                  </div>
                </div>
              )}

              {/* Valid Period */}
              <div className="grid grid-cols-2 gap-4">
                <div>
                  <label className="flex items-center gap-2 text-sm font-medium text-gray-700 mb-1">
                    <Calendar className="w-4 h-4" />
                    วันที่เริ่มต้น <span className="text-red-500">*</span>
                  </label>
                  <input
                    type="date"
                    name="validFrom"
                    value={formData.validFrom}
                    onChange={handleInputChange}
                    className="w-full px-3 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-amber-500 focus:border-amber-500 outline-none transition-colors"
                    required
                  />
                  {formData.validFrom && (
                    <p className="mt-1 text-xs text-gray-500">
                      พ.ศ. {toBuddhistYear(formData.validFrom)}
                    </p>
                  )}
                </div>
                <div>
                  <label className="flex items-center gap-2 text-sm font-medium text-gray-700 mb-1">
                    <Calendar className="w-4 h-4" />
                    วันที่สิ้นสุด <span className="text-red-500">*</span>
                  </label>
                  <input
                    type="date"
                    name="validTo"
                    value={formData.validTo}
                    onChange={handleInputChange}
                    min={formData.validFrom || undefined}
                    className="w-full px-3 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-amber-500 focus:border-amber-500 outline-none transition-colors"
                    required
                  />
                  {formData.validTo && (
                    <p className="mt-1 text-xs text-gray-500">
                      พ.ศ. {toBuddhistYear(formData.validTo)}
                    </p>
                  )}
                </div>
              </div>

              {/* Days of Week */}
              <div>
                <label className="flex items-center gap-2 text-sm font-medium text-gray-700 mb-2">
                  <ToggleLeft className="w-4 h-4" />
                  วันที่มีผล <span className="text-red-500">*</span>
                </label>
                <div className="flex gap-2 mb-2">
                  <button
                    type="button"
                    onClick={handleSelectAllDays}
                    className="px-2 py-1 text-xs bg-gray-100 hover:bg-gray-200 rounded transition-colors"
                  >
                    ทุกวัน
                  </button>
                  <button
                    type="button"
                    onClick={handleSelectWeekdays}
                    className="px-2 py-1 text-xs bg-gray-100 hover:bg-gray-200 rounded transition-colors"
                  >
                    วันธรรมดา
                  </button>
                  <button
                    type="button"
                    onClick={handleSelectWeekends}
                    className="px-2 py-1 text-xs bg-gray-100 hover:bg-gray-200 rounded transition-colors"
                  >
                    วันหยุด
                  </button>
                </div>
                <div className="flex gap-2">
                  {DAYS_OF_WEEK.map((day) => (
                    <button
                      key={day.value}
                      type="button"
                      onClick={() => handleDayToggle(day.value)}
                      className={`w-10 h-10 rounded-full text-sm font-medium transition-colors ${
                        formData.daysOfWeek.includes(day.value)
                          ? 'bg-amber-500 text-white'
                          : 'bg-gray-100 text-gray-600 hover:bg-gray-200'
                      } ${
                        day.value === 0
                          ? 'ring-1 ring-red-300'
                          : day.value === 6
                          ? 'ring-1 ring-blue-300'
                          : ''
                      }`}
                      title={day.fullLabel}
                    >
                      {day.label}
                    </button>
                  ))}
                </div>
              </div>

              {/* Active Toggle */}
              <div className="flex items-center justify-between p-3 bg-gray-50 rounded-lg">
                <label className="text-sm font-medium text-gray-700">เปิดใช้งาน</label>
                <label className="relative inline-flex items-center cursor-pointer">
                  <input
                    type="checkbox"
                    name="active"
                    checked={formData.active}
                    onChange={handleInputChange}
                    className="sr-only peer"
                  />
                  <div className="w-11 h-6 bg-gray-200 peer-focus:outline-none peer-focus:ring-4 peer-focus:ring-amber-300 rounded-full peer peer-checked:after:translate-x-full rtl:peer-checked:after:-translate-x-full peer-checked:after:border-white after:content-[''] after:absolute after:top-[2px] after:start-[2px] after:bg-white after:border-gray-300 after:border after:rounded-full after:h-5 after:w-5 after:transition-all peer-checked:bg-amber-600"></div>
                </label>
              </div>
            </div>

            {/* Footer */}
            <div className="p-4 border-t bg-gray-50 flex items-center justify-between gap-3">
              {/* Delete Button (only in edit mode) */}
              {mode === 'edit' && onDelete && rate?.id && (
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
                        {deleting ? <Loader2 className="w-4 h-4 animate-spin" /> : 'ใช่'}
                      </button>
                      <button
                        type="button"
                        onClick={() => setShowDeleteConfirm(false)}
                        className="px-3 py-1.5 bg-gray-200 text-gray-700 text-sm rounded-lg hover:bg-gray-300 transition-colors"
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
                  className="px-4 py-2 bg-gray-200 hover:bg-gray-300 text-gray-800 rounded-lg transition-colors"
                >
                  ยกเลิก
                </button>
                <button
                  type="submit"
                  disabled={saving}
                  className="flex items-center gap-2 px-4 py-2 bg-amber-600 text-white rounded-lg hover:bg-amber-700 disabled:opacity-50 transition-colors"
                >
                  {saving ? <Loader2 className="w-4 h-4 animate-spin" /> : <Save className="w-4 h-4" />}
                  {mode === 'create' ? 'เพิ่มอัตรา' : 'บันทึก'}
                </button>
              </div>
            </div>
          </form>
        </div>
      </div>
    </>
  )
}
