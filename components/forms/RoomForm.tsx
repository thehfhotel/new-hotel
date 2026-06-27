'use client'

import { useState, useEffect } from 'react'
import {
  X,
  BedDouble,
  Loader2,
  AlertCircle,
  Save,
  Hash,
  DollarSign,
  Layers,
  FileText,
} from 'lucide-react'

export interface RoomFormData {
  id?: number
  roomNo: string
  roomTypeId: number | null
  floor: number | null
  status: string
  isClean: boolean
  isMaintenance: boolean
  priceWeekday: number | null
  priceWeekend: number | null
  priceSpecial: number | null
  notes: string
}

export interface RoomTypeOption {
  id: number
  typeName: string
}

interface RoomFormProps {
  isOpen: boolean
  onClose: () => void
  onSave: (data: RoomFormData) => Promise<void>
  initialData?: RoomFormData | null
  roomTypes: RoomTypeOption[]
  mode: 'create' | 'edit'
}

const emptyFormData: RoomFormData = {
  roomNo: '',
  roomTypeId: null,
  floor: null,
  status: 'available',
  isClean: true,
  isMaintenance: false,
  priceWeekday: null,
  priceWeekend: null,
  priceSpecial: null,
  notes: '',
}

// Canonical room_status domain accepted by PATCH /api/rooms/:id/status — the
// create/update path stores the same set. Kept in lock-step with the backend's
// `valid_statuses` in routes/new_rooms.rs::update_room_status.
const statusOptions = [
  { value: 'available', label: 'ว่าง' },
  { value: 'occupied', label: 'มีผู้เข้าพัก' },
  { value: 'maintenance', label: 'ซ่อมบำรุง' },
  { value: 'cleaning', label: 'กำลังทำความสะอาด' },
]

export default function RoomForm({
  isOpen,
  onClose,
  onSave,
  initialData,
  roomTypes,
  mode,
}: RoomFormProps) {
  const [formData, setFormData] = useState<RoomFormData>(emptyFormData)
  const [saving, setSaving] = useState(false)
  const [error, setError] = useState<string | null>(null)

  useEffect(() => {
    if (isOpen) {
      setFormData(initialData ?? emptyFormData)
      setError(null)
    }
  }, [isOpen, initialData])

  const handleInputChange = (
    e: React.ChangeEvent<HTMLInputElement | HTMLTextAreaElement | HTMLSelectElement>
  ) => {
    const { name, value, type } = e.target
    if (type === 'checkbox') {
      const checked = (e.target as HTMLInputElement).checked
      setFormData((prev) => ({ ...prev, [name]: checked }))
    } else if (type === 'number') {
      // Empty → null (so the optional numeric columns stay NULL canonically).
      setFormData((prev) => ({
        ...prev,
        [name]: value === '' ? null : parseFloat(value),
      }))
    } else if (name === 'roomTypeId') {
      setFormData((prev) => ({
        ...prev,
        roomTypeId: value === '' ? null : parseInt(value, 10),
      }))
    } else {
      setFormData((prev) => ({ ...prev, [name]: value }))
    }
  }

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault()
    setError(null)

    if (!formData.roomNo.trim()) {
      setError('กรุณากรอกเลขห้อง')
      return
    }
    if (formData.priceWeekday !== null && formData.priceWeekday < 0) {
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
                <BedDouble className="w-5 h-5 text-red-600" />
              </div>
              <h2 className="text-xl font-bold text-gray-900">
                {mode === 'create' ? 'เพิ่มห้องพักใหม่' : 'แก้ไขห้องพัก'}
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

              {/* Room No */}
              <div>
                <label className="flex items-center gap-2 text-sm font-medium text-gray-700 mb-1">
                  <Hash className="w-4 h-4" />
                  เลขห้อง <span className="text-red-500">*</span>
                </label>
                <input
                  type="text"
                  name="roomNo"
                  value={formData.roomNo}
                  onChange={handleInputChange}
                  placeholder="เช่น 101, A2-1"
                  maxLength={50}
                  className="w-full px-3 py-2 bg-gray-100 border border-gray-300 text-gray-800 rounded-lg focus:ring-2 focus:ring-red-500 focus:border-red-500 outline-hidden transition-colors"
                  required
                />
              </div>

              {/* Room Type */}
              <div>
                <label className="flex items-center gap-2 text-sm font-medium text-gray-700 mb-1">
                  <Layers className="w-4 h-4" />
                  ประเภทห้อง
                </label>
                <select
                  name="roomTypeId"
                  value={formData.roomTypeId ?? ''}
                  onChange={handleInputChange}
                  className="w-full px-3 py-2 bg-gray-100 border border-gray-300 text-gray-800 rounded-lg focus:ring-2 focus:ring-red-500 focus:border-red-500 outline-hidden transition-colors"
                >
                  <option value="">-- ไม่ระบุ --</option>
                  {roomTypes.map((rt) => (
                    <option key={rt.id} value={rt.id}>
                      {rt.typeName}
                    </option>
                  ))}
                </select>
              </div>

              {/* Floor + Status */}
              <div className="grid grid-cols-2 gap-4">
                <div>
                  <label className="text-sm font-medium text-gray-700 mb-1 block">ชั้น</label>
                  <input
                    type="number"
                    name="floor"
                    value={formData.floor ?? ''}
                    onChange={handleInputChange}
                    placeholder="-"
                    min="0"
                    className="w-full px-3 py-2 bg-gray-100 border border-gray-300 text-gray-800 rounded-lg focus:ring-2 focus:ring-red-500 focus:border-red-500 outline-hidden transition-colors"
                  />
                </div>
                <div>
                  <label className="text-sm font-medium text-gray-700 mb-1 block">สถานะ</label>
                  <select
                    name="status"
                    value={formData.status}
                    onChange={handleInputChange}
                    className="w-full px-3 py-2 bg-gray-100 border border-gray-300 text-gray-800 rounded-lg focus:ring-2 focus:ring-red-500 focus:border-red-500 outline-hidden transition-colors"
                  >
                    {statusOptions.map((s) => (
                      <option key={s.value} value={s.value}>
                        {s.label}
                      </option>
                    ))}
                  </select>
                </div>
              </div>

              {/* Prices */}
              <div className="grid grid-cols-3 gap-3">
                <div>
                  <label className="flex items-center gap-1 text-sm font-medium text-gray-700 mb-1">
                    <DollarSign className="w-3.5 h-3.5" />
                    วันธรรมดา
                  </label>
                  <input
                    type="number"
                    name="priceWeekday"
                    value={formData.priceWeekday ?? ''}
                    onChange={handleInputChange}
                    placeholder="0"
                    min="0"
                    step="0.01"
                    className="w-full px-3 py-2 bg-gray-100 border border-gray-300 text-gray-800 rounded-lg focus:ring-2 focus:ring-red-500 focus:border-red-500 outline-hidden transition-colors"
                  />
                </div>
                <div>
                  <label className="text-sm font-medium text-gray-700 mb-1 block">วันหยุด</label>
                  <input
                    type="number"
                    name="priceWeekend"
                    value={formData.priceWeekend ?? ''}
                    onChange={handleInputChange}
                    placeholder="0"
                    min="0"
                    step="0.01"
                    className="w-full px-3 py-2 bg-gray-100 border border-gray-300 text-gray-800 rounded-lg focus:ring-2 focus:ring-red-500 focus:border-red-500 outline-hidden transition-colors"
                  />
                </div>
                <div>
                  <label className="text-sm font-medium text-gray-700 mb-1 block">พิเศษ</label>
                  <input
                    type="number"
                    name="priceSpecial"
                    value={formData.priceSpecial ?? ''}
                    onChange={handleInputChange}
                    placeholder="0"
                    min="0"
                    step="0.01"
                    className="w-full px-3 py-2 bg-gray-100 border border-gray-300 text-gray-800 rounded-lg focus:ring-2 focus:ring-red-500 focus:border-red-500 outline-hidden transition-colors"
                  />
                </div>
              </div>

              {/* Flags */}
              <div className="flex items-center gap-6">
                <label className="flex items-center gap-2 cursor-pointer">
                  <input
                    type="checkbox"
                    name="isClean"
                    checked={formData.isClean}
                    onChange={handleInputChange}
                    className="w-4 h-4 text-red-600 rounded focus:ring-red-500"
                  />
                  <span className="text-sm text-gray-700">สะอาด</span>
                </label>
                <label className="flex items-center gap-2 cursor-pointer">
                  <input
                    type="checkbox"
                    name="isMaintenance"
                    checked={formData.isMaintenance}
                    onChange={handleInputChange}
                    className="w-4 h-4 text-red-600 rounded focus:ring-red-500"
                  />
                  <span className="text-sm text-gray-700">ซ่อมบำรุง</span>
                </label>
              </div>

              {/* Notes */}
              <div>
                <label className="flex items-center gap-2 text-sm font-medium text-gray-700 mb-1">
                  <FileText className="w-4 h-4" />
                  หมายเหตุ
                </label>
                <textarea
                  name="notes"
                  value={formData.notes}
                  onChange={handleInputChange}
                  rows={2}
                  className="w-full px-3 py-2 bg-gray-100 border border-gray-300 text-gray-800 rounded-lg focus:ring-2 focus:ring-red-500 focus:border-red-500 outline-hidden transition-colors resize-none"
                />
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
                {mode === 'create' ? 'เพิ่มห้อง' : 'บันทึก'}
              </button>
            </div>
          </form>
        </div>
      </div>
    </>
  )
}
