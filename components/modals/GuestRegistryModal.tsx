'use client'

import { useState, useEffect, useCallback } from 'react'
import {
  X,
  Users,
  User,
  CreditCard,
  Globe,
  Loader2,
  AlertCircle,
  Plus,
  Trash2,
  Star,
  FileText,
} from 'lucide-react'

interface Guest {
  id: number
  checkInId: number
  customerId: number | null
  firstName: string
  lastName: string | null
  idCard: string | null
  passport: string | null
  nationality: string | null
  isPrimary: boolean
  createdAt: string | null
}

interface CheckInInfo {
  id: number
  checkInNo: string
  roomNumber: string
  customerName: string
}

interface GuestRegistryModalProps {
  isOpen: boolean
  onClose: () => void
  checkIn: CheckInInfo
  onGuestsChanged?: () => void
}

interface NewGuestForm {
  firstName: string
  lastName: string
  idCard: string
  passport: string
  nationality: string
  isPrimary: boolean
}

const emptyGuestForm: NewGuestForm = {
  firstName: '',
  lastName: '',
  idCard: '',
  passport: '',
  nationality: 'Thai',
  isPrimary: false,
}

const nationalities = [
  'Thai',
  'American',
  'British',
  'Chinese',
  'Japanese',
  'Korean',
  'Australian',
  'German',
  'French',
  'Indian',
  'Russian',
  'Other',
]

export default function GuestRegistryModal({
  isOpen,
  onClose,
  checkIn,
  onGuestsChanged,
}: GuestRegistryModalProps) {
  const [guests, setGuests] = useState<Guest[]>([])
  const [loading, setLoading] = useState(true)
  const [error, setError] = useState<string | null>(null)
  const [showAddForm, setShowAddForm] = useState(false)
  const [formData, setFormData] = useState<NewGuestForm>(emptyGuestForm)
  const [saving, setSaving] = useState(false)
  const [deleting, setDeleting] = useState<number | null>(null)

  // Fetch guests for this check-in
  const fetchGuests = useCallback(async () => {
    setLoading(true)
    setError(null)
    try {
      const response = await fetch(`/api/new/checkins/${checkIn.id}/guests`)
      if (!response.ok) {
        throw new Error('Failed to fetch guests')
      }

      const data = await response.json()
      if (data.success) {
        setGuests(data.data || [])
      } else {
        throw new Error(data.message || 'Failed to fetch guests')
      }
    } catch (err) {
      setError(err instanceof Error ? err.message : 'เกิดข้อผิดพลาดในการโหลดข้อมูล')
    } finally {
      setLoading(false)
    }
  }, [checkIn.id])

  useEffect(() => {
    if (isOpen) {
      fetchGuests()
      setShowAddForm(false)
      setFormData(emptyGuestForm)
    }
  }, [isOpen, fetchGuests])

  // Handle input change
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
    } else {
      setFormData((prev) => ({
        ...prev,
        [name]: value,
      }))
    }
  }

  // Handle add guest
  const handleAddGuest = async (e: React.FormEvent) => {
    e.preventDefault()
    setError(null)

    // Validation
    if (!formData.firstName.trim()) {
      setError('กรุณากรอกชื่อผู้เข้าพัก')
      return
    }

    if (!formData.idCard.trim() && !formData.passport.trim()) {
      setError('กรุณากรอกเลขบัตรประชาชน หรือ หมายเลขพาสปอร์ต')
      return
    }

    setSaving(true)
    try {
      const response = await fetch(`/api/new/checkins/${checkIn.id}/guests`, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({
          firstName: formData.firstName,
          lastName: formData.lastName || null,
          idCard: formData.idCard || null,
          passport: formData.passport || null,
          nationality: formData.nationality || null,
          isPrimary: formData.isPrimary,
        }),
      })

      const result = await response.json()

      if (!response.ok || !result.success) {
        throw new Error(result.message || 'Failed to add guest')
      }

      // Refresh the list and reset form
      fetchGuests()
      setFormData(emptyGuestForm)
      setShowAddForm(false)
      onGuestsChanged?.()
    } catch (err) {
      setError(err instanceof Error ? err.message : 'เกิดข้อผิดพลาดในการเพิ่มผู้เข้าพัก')
    } finally {
      setSaving(false)
    }
  }

  // Handle delete guest
  const handleDeleteGuest = async (guestId: number) => {
    setDeleting(guestId)
    setError(null)
    try {
      const response = await fetch(`/api/new/checkins/${checkIn.id}/guests/${guestId}`, {
        method: 'DELETE',
      })

      const result = await response.json()

      if (!response.ok || !result.success) {
        throw new Error(result.message || 'Failed to delete guest')
      }

      // Refresh the list
      fetchGuests()
      onGuestsChanged?.()
    } catch (err) {
      setError(err instanceof Error ? err.message : 'เกิดข้อผิดพลาดในการลบผู้เข้าพัก')
    } finally {
      setDeleting(null)
    }
  }

  // Cancel add form
  const handleCancelAdd = () => {
    setShowAddForm(false)
    setFormData(emptyGuestForm)
    setError(null)
  }

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
        <div className="bg-white rounded-lg shadow-xl w-full max-w-2xl max-h-[90vh] overflow-hidden">
          {/* Header */}
          <div className="flex items-center justify-between p-4 border-b bg-gray-50">
            <div className="flex items-center gap-3">
              <div className="w-10 h-10 bg-blue-100 rounded-full flex items-center justify-center">
                <Users className="w-5 h-5 text-blue-600" />
              </div>
              <div>
                <h2 className="text-xl font-bold text-gray-800">ทะเบียนผู้เข้าพัก</h2>
                <p className="text-sm text-gray-500">
                  ห้อง {checkIn.roomNumber} - {checkIn.customerName}
                </p>
              </div>
            </div>
            <button
              onClick={onClose}
              className="p-2 hover:bg-gray-200 rounded-full transition-colors"
              aria-label="ปิด"
            >
              <X className="w-5 h-5" />
            </button>
          </div>

          {/* Content */}
          <div className="p-4 overflow-y-auto max-h-[60vh]">
            {/* TM.30 Notice */}
            <div className="mb-4 p-3 bg-yellow-50 border border-yellow-200 rounded-lg">
              <div className="flex items-start gap-2">
                <FileText className="w-5 h-5 text-yellow-600 flex-shrink-0 mt-0.5" />
                <div className="text-sm text-yellow-800">
                  <p className="font-medium">สำหรับการแจ้ง ต.ม.30</p>
                  <p className="text-yellow-700">กรุณาลงทะเบียนผู้เข้าพักทุกคนที่พักในห้อง รวมถึงข้อมูลบัตรประชาชน/พาสปอร์ต</p>
                </div>
              </div>
            </div>

            {/* Error Message */}
            {error && (
              <div className="mb-4 flex items-center gap-2 p-3 bg-red-50 border border-red-200 rounded-lg text-red-700">
                <AlertCircle className="w-5 h-5 flex-shrink-0" />
                <span className="text-sm">{error}</span>
              </div>
            )}

            {/* Guest List */}
            {loading ? (
              <div className="flex items-center justify-center py-8">
                <Loader2 className="w-6 h-6 animate-spin text-blue-600" />
                <span className="ml-2 text-gray-600">กำลังโหลดข้อมูล...</span>
              </div>
            ) : guests.length === 0 ? (
              <div className="text-center py-8 text-gray-500">
                <Users className="w-12 h-12 text-gray-300 mx-auto mb-3" />
                <p>ยังไม่มีผู้เข้าพักในทะเบียน</p>
              </div>
            ) : (
              <div className="space-y-3 mb-4">
                {guests.map((guest) => (
                  <div
                    key={guest.id}
                    className="flex items-center justify-between p-3 border border-gray-200 rounded-lg hover:bg-gray-50"
                  >
                    <div className="flex items-center gap-3">
                      <div className={`w-10 h-10 rounded-full flex items-center justify-center ${
                        guest.isPrimary ? 'bg-yellow-100' : 'bg-gray-100'
                      }`}>
                        {guest.isPrimary ? (
                          <Star className="w-5 h-5 text-yellow-600" />
                        ) : (
                          <User className="w-5 h-5 text-gray-500" />
                        )}
                      </div>
                      <div>
                        <div className="flex items-center gap-2">
                          <span className="font-medium text-gray-800">
                            {guest.firstName} {guest.lastName || ''}
                          </span>
                          {guest.isPrimary && (
                            <span className="px-2 py-0.5 bg-yellow-100 text-yellow-700 text-xs rounded-full">
                              ผู้เข้าพักหลัก
                            </span>
                          )}
                        </div>
                        <div className="flex items-center gap-4 text-sm text-gray-500">
                          {guest.idCard && (
                            <span className="flex items-center gap-1">
                              <CreditCard className="w-3 h-3" />
                              {guest.idCard}
                            </span>
                          )}
                          {guest.passport && (
                            <span className="flex items-center gap-1">
                              <FileText className="w-3 h-3" />
                              {guest.passport}
                            </span>
                          )}
                          {guest.nationality && (
                            <span className="flex items-center gap-1">
                              <Globe className="w-3 h-3" />
                              {guest.nationality}
                            </span>
                          )}
                        </div>
                      </div>
                    </div>
                    <button
                      onClick={() => handleDeleteGuest(guest.id)}
                      disabled={deleting === guest.id}
                      className="p-2 text-red-500 hover:bg-red-50 rounded-lg transition-colors disabled:opacity-50"
                      title="ลบผู้เข้าพัก"
                    >
                      {deleting === guest.id ? (
                        <Loader2 className="w-4 h-4 animate-spin" />
                      ) : (
                        <Trash2 className="w-4 h-4" />
                      )}
                    </button>
                  </div>
                ))}
              </div>
            )}

            {/* Add Guest Form */}
            {showAddForm ? (
              <form onSubmit={handleAddGuest} className="border border-blue-200 rounded-lg p-4 bg-blue-50">
                <h3 className="text-lg font-semibold text-gray-800 mb-4">เพิ่มผู้เข้าพัก</h3>

                <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
                  {/* First Name */}
                  <div>
                    <label className="flex items-center gap-2 text-sm font-medium text-gray-700 mb-1">
                      <User className="w-4 h-4" />
                      ชื่อ <span className="text-red-500">*</span>
                    </label>
                    <input
                      type="text"
                      name="firstName"
                      value={formData.firstName}
                      onChange={handleInputChange}
                      placeholder="กรอกชื่อ"
                      className="w-full px-3 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-blue-500 outline-none transition-colors bg-white"
                      required
                    />
                  </div>

                  {/* Last Name */}
                  <div>
                    <label className="flex items-center gap-2 text-sm font-medium text-gray-700 mb-1">
                      <User className="w-4 h-4" />
                      นามสกุล
                    </label>
                    <input
                      type="text"
                      name="lastName"
                      value={formData.lastName}
                      onChange={handleInputChange}
                      placeholder="กรอกนามสกุล"
                      className="w-full px-3 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-blue-500 outline-none transition-colors bg-white"
                    />
                  </div>

                  {/* ID Card */}
                  <div>
                    <label className="flex items-center gap-2 text-sm font-medium text-gray-700 mb-1">
                      <CreditCard className="w-4 h-4" />
                      เลขบัตรประชาชน
                    </label>
                    <input
                      type="text"
                      name="idCard"
                      value={formData.idCard}
                      onChange={handleInputChange}
                      placeholder="กรอกเลขบัตรประชาชน"
                      maxLength={13}
                      className="w-full px-3 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-blue-500 outline-none transition-colors font-mono bg-white"
                    />
                  </div>

                  {/* Passport */}
                  <div>
                    <label className="flex items-center gap-2 text-sm font-medium text-gray-700 mb-1">
                      <FileText className="w-4 h-4" />
                      หมายเลขพาสปอร์ต
                    </label>
                    <input
                      type="text"
                      name="passport"
                      value={formData.passport}
                      onChange={handleInputChange}
                      placeholder="กรอกหมายเลขพาสปอร์ต"
                      className="w-full px-3 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-blue-500 outline-none transition-colors font-mono bg-white"
                    />
                  </div>

                  {/* Nationality */}
                  <div>
                    <label className="flex items-center gap-2 text-sm font-medium text-gray-700 mb-1">
                      <Globe className="w-4 h-4" />
                      สัญชาติ
                    </label>
                    <select
                      name="nationality"
                      value={formData.nationality}
                      onChange={handleInputChange}
                      className="w-full px-3 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-blue-500 outline-none transition-colors bg-white"
                    >
                      {nationalities.map((nat) => (
                        <option key={nat} value={nat}>
                          {nat}
                        </option>
                      ))}
                    </select>
                  </div>

                  {/* Primary Guest Checkbox */}
                  <div className="flex items-center">
                    <label className="flex items-center gap-2 cursor-pointer">
                      <input
                        type="checkbox"
                        name="isPrimary"
                        checked={formData.isPrimary}
                        onChange={handleInputChange}
                        className="w-4 h-4 text-blue-600 bg-white border-gray-300 rounded focus:ring-blue-500"
                      />
                      <span className="flex items-center gap-1 text-sm font-medium text-gray-700">
                        <Star className="w-4 h-4 text-yellow-500" />
                        ผู้เข้าพักหลัก
                      </span>
                    </label>
                  </div>
                </div>

                {/* Form Actions */}
                <div className="flex justify-end gap-3 mt-4">
                  <button
                    type="button"
                    onClick={handleCancelAdd}
                    className="px-4 py-2 bg-gray-200 hover:bg-gray-300 text-gray-800 rounded-lg transition-colors"
                  >
                    ยกเลิก
                  </button>
                  <button
                    type="submit"
                    disabled={saving}
                    className="flex items-center gap-2 px-4 py-2 bg-blue-600 text-white rounded-lg hover:bg-blue-700 disabled:opacity-50 transition-colors"
                  >
                    {saving ? (
                      <Loader2 className="w-4 h-4 animate-spin" />
                    ) : (
                      <Plus className="w-4 h-4" />
                    )}
                    เพิ่มผู้เข้าพัก
                  </button>
                </div>
              </form>
            ) : (
              <button
                onClick={() => setShowAddForm(true)}
                className="w-full flex items-center justify-center gap-2 px-4 py-3 border-2 border-dashed border-gray-300 text-gray-600 rounded-lg hover:border-blue-500 hover:text-blue-600 transition-colors"
              >
                <Plus className="w-5 h-5" />
                เพิ่มผู้เข้าพัก
              </button>
            )}
          </div>

          {/* Footer */}
          <div className="p-4 border-t bg-gray-50">
            <div className="flex items-center justify-between">
              <div className="text-sm text-gray-600">
                จำนวนผู้เข้าพัก: <span className="font-semibold">{guests.length}</span> คน
              </div>
              <button
                onClick={onClose}
                className="px-4 py-2 bg-gray-200 hover:bg-gray-300 text-gray-800 rounded-lg transition-colors"
              >
                ปิด
              </button>
            </div>
          </div>
        </div>
      </div>
    </>
  )
}
