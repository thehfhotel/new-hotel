'use client'

import { useState, useEffect } from 'react'
import {
  X,
  User,
  Phone,
  Mail,
  CreditCard,
  MapPin,
  FileText,
  Loader2,
  AlertCircle,
  Save,
  Trash2,
} from 'lucide-react'

export interface CustomerFormData {
  id?: number
  firstName: string
  lastName: string
  phone: string
  email: string
  idCard: string
  address: string
  notes: string
}

interface CustomerFormProps {
  isOpen: boolean
  onClose: () => void
  onSave: (data: CustomerFormData) => Promise<void>
  onDelete?: (id: number) => Promise<void>
  initialData?: CustomerFormData | null
  mode: 'create' | 'edit'
}

const emptyFormData: CustomerFormData = {
  firstName: '',
  lastName: '',
  phone: '',
  email: '',
  idCard: '',
  address: '',
  notes: '',
}

export default function CustomerForm({
  isOpen,
  onClose,
  onSave,
  onDelete,
  initialData,
  mode,
}: CustomerFormProps) {
  const [formData, setFormData] = useState<CustomerFormData>(emptyFormData)
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
    e: React.ChangeEvent<HTMLInputElement | HTMLTextAreaElement>
  ) => {
    const { name, value } = e.target
    setFormData((prev) => ({
      ...prev,
      [name]: value,
    }))
  }

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault()
    setError(null)

    // Validation
    if (!formData.firstName.trim()) {
      setError('กรุณากรอกชื่อ')
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
                <User className="w-5 h-5 text-red-400" />
              </div>
              <h2 className="text-xl font-bold text-zinc-100">
                {mode === 'create' ? 'เพิ่มลูกค้าใหม่' : 'แก้ไขข้อมูลลูกค้า'}
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

              {/* First Name (Required) */}
              <div>
                <label className="flex items-center gap-2 text-sm font-medium text-zinc-300 mb-1">
                  <User className="w-4 h-4" />
                  ชื่อ <span className="text-red-500">*</span>
                </label>
                <input
                  type="text"
                  name="firstName"
                  value={formData.firstName}
                  onChange={handleInputChange}
                  placeholder="กรอกชื่อ"
                  className="w-full px-3 py-2 bg-zinc-800 border border-zinc-700 text-zinc-200 rounded-lg focus:ring-2 focus:ring-red-500 focus:border-red-500 outline-none transition-colors"
                  required
                />
              </div>

              {/* Last Name */}
              <div>
                <label className="flex items-center gap-2 text-sm font-medium text-zinc-300 mb-1">
                  <User className="w-4 h-4" />
                  นามสกุล
                </label>
                <input
                  type="text"
                  name="lastName"
                  value={formData.lastName}
                  onChange={handleInputChange}
                  placeholder="กรอกนามสกุล"
                  className="w-full px-3 py-2 bg-zinc-800 border border-zinc-700 text-zinc-200 rounded-lg focus:ring-2 focus:ring-red-500 focus:border-red-500 outline-none transition-colors"
                />
              </div>

              {/* Phone */}
              <div>
                <label className="flex items-center gap-2 text-sm font-medium text-zinc-300 mb-1">
                  <Phone className="w-4 h-4" />
                  เบอร์โทร
                </label>
                <input
                  type="tel"
                  name="phone"
                  value={formData.phone}
                  onChange={handleInputChange}
                  placeholder="กรอกเบอร์โทรศัพท์"
                  className="w-full px-3 py-2 bg-zinc-800 border border-zinc-700 text-zinc-200 rounded-lg focus:ring-2 focus:ring-red-500 focus:border-red-500 outline-none transition-colors"
                />
              </div>

              {/* Email */}
              <div>
                <label className="flex items-center gap-2 text-sm font-medium text-zinc-300 mb-1">
                  <Mail className="w-4 h-4" />
                  อีเมล
                </label>
                <input
                  type="email"
                  name="email"
                  value={formData.email}
                  onChange={handleInputChange}
                  placeholder="กรอกอีเมล"
                  className="w-full px-3 py-2 bg-zinc-800 border border-zinc-700 text-zinc-200 rounded-lg focus:ring-2 focus:ring-red-500 focus:border-red-500 outline-none transition-colors"
                />
              </div>

              {/* ID Card */}
              <div>
                <label className="flex items-center gap-2 text-sm font-medium text-zinc-300 mb-1">
                  <CreditCard className="w-4 h-4" />
                  เลขบัตรประชาชน
                </label>
                <input
                  type="text"
                  name="idCard"
                  value={formData.idCard}
                  onChange={handleInputChange}
                  placeholder="กรอกเลขบัตรประชาชน"
                  className="w-full px-3 py-2 bg-zinc-800 border border-zinc-700 text-zinc-200 rounded-lg focus:ring-2 focus:ring-red-500 focus:border-red-500 outline-none transition-colors font-mono"
                />
              </div>

              {/* Address */}
              <div>
                <label className="flex items-center gap-2 text-sm font-medium text-zinc-300 mb-1">
                  <MapPin className="w-4 h-4" />
                  ที่อยู่
                </label>
                <textarea
                  name="address"
                  value={formData.address}
                  onChange={handleInputChange}
                  placeholder="กรอกที่อยู่"
                  rows={2}
                  className="w-full px-3 py-2 bg-zinc-800 border border-zinc-700 text-zinc-200 rounded-lg focus:ring-2 focus:ring-red-500 focus:border-red-500 outline-none transition-colors resize-none"
                />
              </div>

              {/* Notes */}
              <div>
                <label className="flex items-center gap-2 text-sm font-medium text-zinc-300 mb-1">
                  <FileText className="w-4 h-4" />
                  หมายเหตุ
                </label>
                <textarea
                  name="notes"
                  value={formData.notes}
                  onChange={handleInputChange}
                  placeholder="กรอกหมายเหตุ"
                  rows={3}
                  className="w-full px-3 py-2 bg-zinc-800 border border-zinc-700 text-zinc-200 rounded-lg focus:ring-2 focus:ring-red-500 focus:border-red-500 outline-none transition-colors resize-none"
                />
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
                  {mode === 'create' ? 'เพิ่มลูกค้า' : 'บันทึก'}
                </button>
              </div>
            </div>
          </form>
        </div>
      </div>
    </>
  )
}
