'use client'

import { useState, useEffect, useMemo } from 'react'
import {
  X,
  Calendar,
  Users,
  Bed,
  CreditCard,
  FileText,
  Loader2,
  AlertCircle,
  Save,
  Trash2,
  Moon,
} from 'lucide-react'
import CustomerPicker, { CustomerOption } from '@/components/pickers/CustomerPicker'
import CustomerForm, { CustomerFormData } from '@/components/forms/CustomerForm'
import RoomPicker, { RoomOption } from '@/components/pickers/RoomPicker'
import { useBranchFetch } from '@/lib/use-branch-fetch'

export interface BookingFormState {
  id?: number
  bookNo?: string
  customerId: number
  customerName?: string
  checkIn: string
  checkOut: string
  adults: number
  children: number
  status: string
  source: string | null
  depositAmount: number | null
  notes: string | null
  rooms: { roomId: number; pricePerNight: number | null }[]
}

interface BookingFormProps {
  isOpen: boolean
  onClose: () => void
  onSave: (data: BookingFormState) => Promise<void>
  onCancel?: (id: number) => Promise<void>
  initialData?: BookingFormState | null
  mode: 'create' | 'edit'
}

const sourceOptions = [
  { value: 'walk-in', label: 'Walk-in' },
  { value: 'phone', label: 'โทรศัพท์' },
  { value: 'online', label: 'Online' },
  { value: 'ota', label: 'OTA' },
]

// User-settable booking statuses. `checkedin` and `completed` are
// intentionally NOT in this list — those transitions must go through the
// proper Check-In and Check-Out flows so the legacy MSSQL DB also gets the
// matching `HT_CheckIn_H` / `HT_Power_Log` / `HT_Cupon` rows. Setting
// `book_status='checkedin'` directly via this form (which fires
// `modify_booking` writeback) leaves the booking status updated but no
// check-in record exists in legacy — receptionist sees the room flip back
// to "available" because no active check-in claims it.
// Cancellation also goes through its own button (PUT /cancel) for the same
// reason, so we omit `cancelled` here too.
const statusOptions = [
  { value: 'pending', label: 'รอยืนยัน' },
  { value: 'confirmed', label: 'ยืนยันแล้ว' },
  { value: 'noshow', label: 'ไม่มาตามนัด' },
]

// Convert Gregorian year to Buddhist Era
function toBuddhistYear(date: Date): string {
  const year = date.getUTCFullYear() + 543
  const month = String(date.getUTCMonth() + 1).padStart(2, '0')
  const day = String(date.getUTCDate()).padStart(2, '0')
  return `${day}/${month}/${year}`
}

// Format date for API (YYYY-MM-DD)
function formatDateForApi(date: Date): string {
  const year = date.getFullYear()
  const month = String(date.getMonth() + 1).padStart(2, '0')
  const day = String(date.getDate()).padStart(2, '0')
  return `${year}-${month}-${day}`
}

// Calculate nights between two dates
function calculateNights(checkIn: string, checkOut: string): number {
  if (!checkIn || !checkOut) return 0
  const start = new Date(checkIn + 'T00:00')
  const end = new Date(checkOut + 'T00:00')
  const diff = end.getTime() - start.getTime()
  return Math.max(0, Math.ceil(diff / (1000 * 60 * 60 * 24)))
}

const emptyFormData: BookingFormState = {
  customerId: 0,
  checkIn: '',
  checkOut: '',
  adults: 1,
  children: 0,
  status: 'pending',
  source: 'walk-in',
  depositAmount: null,
  notes: null,
  rooms: [],
}

export default function BookingForm({
  isOpen,
  onClose,
  onSave,
  onCancel,
  initialData,
  mode,
}: BookingFormProps) {
  const branchFetch = useBranchFetch()
  const [formData, setFormData] = useState<BookingFormState>(emptyFormData)
  const [selectedCustomer, setSelectedCustomer] = useState<CustomerOption | null>(null)
  const [selectedRooms, setSelectedRooms] = useState<RoomOption[]>([])
  const [saving, setSaving] = useState(false)
  const [cancelling, setCancelling] = useState(false)
  const [error, setError] = useState<string | null>(null)
  const [showCancelConfirm, setShowCancelConfirm] = useState(false)
  const [showCustomerForm, setShowCustomerForm] = useState(false)

  // Set default dates for new booking
  useEffect(() => {
    if (isOpen && mode === 'create' && !initialData) {
      const today = new Date()
      const tomorrow = new Date()
      tomorrow.setDate(tomorrow.getDate() + 1)
      setFormData({
        ...emptyFormData,
        checkIn: formatDateForApi(today),
        checkOut: formatDateForApi(tomorrow),
      })
    }
  }, [isOpen, mode, initialData])

  // Reset form when modal opens/closes or initialData changes
  useEffect(() => {
    if (isOpen) {
      if (initialData) {
        setFormData(initialData)
        // Reconstruct customer from initialData
        if (initialData.customerId) {
          // The customer will need to be fetched or passed in
          // For now, set a minimal customer object
          setSelectedCustomer({
            id: initialData.customerId,
            firstName: initialData.customerName?.split(' ')[0] || '',
            lastName: initialData.customerName?.split(' ').slice(1).join(' ') || '',
            phone: '',
            email: '',
            idCard: '',
          })
        }
      } else {
        setFormData(emptyFormData)
        setSelectedCustomer(null)
        setSelectedRooms([])
      }
      setError(null)
      setShowCancelConfirm(false)
    }
  }, [isOpen, initialData])

  // Calculate nights
  const nights = useMemo(
    () => calculateNights(formData.checkIn, formData.checkOut),
    [formData.checkIn, formData.checkOut]
  )

  const handleInputChange = (
    e: React.ChangeEvent<HTMLInputElement | HTMLTextAreaElement | HTMLSelectElement>
  ) => {
    const { name, value, type } = e.target
    setFormData((prev) => ({
      ...prev,
      [name]: type === 'number' ? (value === '' ? null : Number(value)) : value,
    }))
  }

  const handleCustomerChange = (customer: CustomerOption | null) => {
    setSelectedCustomer(customer)
    setFormData((prev) => ({
      ...prev,
      customerId: customer?.id || 0,
    }))
  }

  const handleRoomsChange = (rooms: RoomOption[]) => {
    setSelectedRooms(rooms)
    setFormData((prev) => ({
      ...prev,
      rooms: rooms.map((r) => ({
        roomId: r.id,
        pricePerNight: r.priceWeekday,
      })),
    }))
  }

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault()
    setError(null)

    // Validation
    if (!formData.customerId) {
      setError('กรุณาเลือกลูกค้า')
      return
    }

    if (!formData.checkIn) {
      setError('กรุณาเลือกวันเช็คอิน')
      return
    }

    if (!formData.checkOut) {
      setError('กรุณาเลือกวันเช็คเอาท์')
      return
    }

    if (nights <= 0) {
      setError('วันเช็คเอาท์ต้องหลังวันเช็คอิน')
      return
    }

    if (formData.rooms.length === 0) {
      setError('กรุณาเลือกห้องพักอย่างน้อย 1 ห้อง')
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

  const handleCancel = async () => {
    if (!initialData?.id || !onCancel) return

    setCancelling(true)
    setError(null)
    try {
      await onCancel(initialData.id)
      onClose()
    } catch (err) {
      setError(err instanceof Error ? err.message : 'เกิดข้อผิดพลาดในการยกเลิก')
    } finally {
      setCancelling(false)
      setShowCancelConfirm(false)
    }
  }

  const handleAddNewCustomer = () => {
    setShowCustomerForm(true)
  }

  const handleSaveNewCustomer = async (data: CustomerFormData) => {
    const response = await branchFetch('/api/new/customers', {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({
        firstName: data.firstName,
        lastName: data.lastName || null,
        phone: data.phone || null,
        email: data.email || null,
        idCard: data.idCard || null,
        address: data.address || null,
        notes: data.notes || null,
      }),
    })

    const result = await response.json()

    if (!response.ok || !result.success) {
      throw new Error(result.message || 'Failed to save customer')
    }

    // Set the new customer as selected
    if (result.id) {
      const newCustomer: CustomerOption = {
        id: result.id,
        firstName: data.firstName,
        lastName: data.lastName || '',
        phone: data.phone || '',
        email: data.email || '',
        idCard: data.idCard || '',
      }
      handleCustomerChange(newCustomer)
    }
  }

  if (!isOpen) return null

  const canCancel =
    mode === 'edit' &&
    onCancel &&
    initialData?.id &&
    initialData.status !== 'cancelled' &&
    initialData.status !== 'completed'

  return (
    <>
      {/* Backdrop */}
      <div
        className="fixed inset-0 bg-black/30 z-40"
        onClick={onClose}
      />

      {/* Modal */}
      <div className="fixed inset-0 z-50 flex items-center justify-center p-4">
        <div className="bg-white rounded-lg shadow-xl border border-gray-200 w-full max-w-2xl max-h-[90vh] overflow-hidden">
          {/* Header */}
          <div className="flex items-center justify-between p-4 border-b border-gray-200 bg-gray-100">
            <div className="flex items-center gap-3">
              <div className="w-10 h-10 bg-red-500/10 rounded-full flex items-center justify-center">
                <Calendar className="w-5 h-5 text-red-600" />
              </div>
              <div>
                <h2 className="text-xl font-bold text-gray-900">
                  {mode === 'create' ? 'สร้างการจองใหม่' : 'แก้ไขการจอง'}
                </h2>
                {initialData?.bookNo && (
                  <p className="text-sm text-gray-500">เลขที่จอง: {initialData.bookNo}</p>
                )}
              </div>
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

              {/* Customer Picker */}
              <div>
                <label className="flex items-center gap-2 text-sm font-medium text-gray-700 mb-1">
                  <Users className="w-4 h-4" />
                  ลูกค้า <span className="text-red-500">*</span>
                </label>
                <CustomerPicker
                  value={selectedCustomer}
                  onChange={handleCustomerChange}
                  onAddNew={handleAddNewCustomer}
                  placeholder="ค้นหาลูกค้า..."
                  disabled={saving}
                />
              </div>

              {/* Check-in / Check-out Dates */}
              <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
                <div>
                  <label className="flex items-center gap-2 text-sm font-medium text-gray-700 mb-1">
                    <Calendar className="w-4 h-4" />
                    วันเช็คอิน <span className="text-red-500">*</span>
                  </label>
                  <input
                    type="date"
                    name="checkIn"
                    value={formData.checkIn}
                    onChange={handleInputChange}
                    className="w-full px-3 py-2 bg-gray-100 border border-gray-300 text-gray-800 rounded-lg focus:ring-2 focus:ring-red-500 focus:border-red-500 outline-hidden transition-colors"
                    required
                    disabled={saving}
                  />
                  {formData.checkIn && (
                    <p className="mt-1 text-xs text-gray-500">
                      พ.ศ. {toBuddhistYear(new Date(formData.checkIn))}
                    </p>
                  )}
                </div>

                <div>
                  <label className="flex items-center gap-2 text-sm font-medium text-gray-700 mb-1">
                    <Calendar className="w-4 h-4" />
                    วันเช็คเอาท์ <span className="text-red-500">*</span>
                  </label>
                  <input
                    type="date"
                    name="checkOut"
                    value={formData.checkOut}
                    onChange={handleInputChange}
                    min={formData.checkIn || undefined}
                    className="w-full px-3 py-2 bg-gray-100 border border-gray-300 text-gray-800 rounded-lg focus:ring-2 focus:ring-red-500 focus:border-red-500 outline-hidden transition-colors"
                    required
                    disabled={saving}
                  />
                  {formData.checkOut && (
                    <p className="mt-1 text-xs text-gray-500">
                      พ.ศ. {toBuddhistYear(new Date(formData.checkOut))}
                    </p>
                  )}
                </div>
              </div>

              {/* Nights Display */}
              {nights > 0 && (
                <div className="flex items-center gap-2 p-3 bg-red-500/10 border border-red-200 rounded-lg">
                  <Moon className="w-5 h-5 text-red-600" />
                  <span className="text-red-600 font-medium">
                    จำนวน {nights} คืน
                  </span>
                </div>
              )}

              {/* Adults / Children */}
              <div className="grid grid-cols-2 gap-4">
                <div>
                  <label className="flex items-center gap-2 text-sm font-medium text-gray-700 mb-1">
                    ผู้ใหญ่
                  </label>
                  <input
                    type="number"
                    name="adults"
                    value={formData.adults}
                    onChange={handleInputChange}
                    min="1"
                    className="w-full px-3 py-2 bg-gray-100 border border-gray-300 text-gray-800 rounded-lg focus:ring-2 focus:ring-red-500 focus:border-red-500 outline-hidden transition-colors"
                    disabled={saving}
                  />
                </div>
                <div>
                  <label className="flex items-center gap-2 text-sm font-medium text-gray-700 mb-1">
                    เด็ก
                  </label>
                  <input
                    type="number"
                    name="children"
                    value={formData.children}
                    onChange={handleInputChange}
                    min="0"
                    className="w-full px-3 py-2 bg-gray-100 border border-gray-300 text-gray-800 rounded-lg focus:ring-2 focus:ring-red-500 focus:border-red-500 outline-hidden transition-colors"
                    disabled={saving}
                  />
                </div>
              </div>

              {/* Room Selection */}
              <div>
                <label className="flex items-center gap-2 text-sm font-medium text-gray-700 mb-2">
                  <Bed className="w-4 h-4" />
                  เลือกห้องพัก <span className="text-red-500">*</span>
                </label>
                <RoomPicker
                  value={selectedRooms}
                  onChange={handleRoomsChange}
                  checkInDate={formData.checkIn}
                  checkOutDate={formData.checkOut}
                  excludeBookingId={initialData?.id}
                  disabled={saving}
                />
              </div>

              {/* Source / Status */}
              <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
                <div>
                  <label className="flex items-center gap-2 text-sm font-medium text-gray-700 mb-1">
                    ช่องทางการจอง
                  </label>
                  <select
                    name="source"
                    value={formData.source || ''}
                    onChange={handleInputChange}
                    className="w-full px-3 py-2 bg-gray-100 border border-gray-300 text-gray-800 rounded-lg focus:ring-2 focus:ring-red-500 focus:border-red-500 outline-hidden transition-colors"
                    disabled={saving}
                  >
                    <option value="">-- เลือก --</option>
                    {sourceOptions.map((opt) => (
                      <option key={opt.value} value={opt.value}>
                        {opt.label}
                      </option>
                    ))}
                  </select>
                </div>

                {mode === 'edit' && (
                  <div>
                    <label className="flex items-center gap-2 text-sm font-medium text-gray-700 mb-1">
                      สถานะ
                    </label>
                    <select
                      name="status"
                      value={formData.status}
                      onChange={handleInputChange}
                      className="w-full px-3 py-2 bg-gray-100 border border-gray-300 text-gray-800 rounded-lg focus:ring-2 focus:ring-red-500 focus:border-red-500 outline-hidden transition-colors"
                      disabled={saving}
                    >
                      {statusOptions.map((opt) => (
                        <option key={opt.value} value={opt.value}>
                          {opt.label}
                        </option>
                      ))}
                    </select>
                  </div>
                )}
              </div>

              {/* Deposit Amount */}
              <div>
                <label className="flex items-center gap-2 text-sm font-medium text-gray-700 mb-1">
                  <CreditCard className="w-4 h-4" />
                  เงินมัดจำ (บาท)
                </label>
                <input
                  type="number"
                  name="depositAmount"
                  value={formData.depositAmount ?? ''}
                  onChange={handleInputChange}
                  min="0"
                  step="0.01"
                  placeholder="ระบุจำนวนเงินมัดจำ"
                  className="w-full px-3 py-2 bg-gray-100 border border-gray-300 text-gray-800 rounded-lg focus:ring-2 focus:ring-red-500 focus:border-red-500 outline-hidden transition-colors"
                  disabled={saving}
                />
              </div>

              {/* Notes (special requests + internal notes combined) */}
              <div>
                <label className="flex items-center gap-2 text-sm font-medium text-gray-700 mb-1">
                  <FileText className="w-4 h-4" />
                  ความต้องการพิเศษ / หมายเหตุภายใน
                </label>
                <textarea
                  name="notes"
                  value={formData.notes ?? ''}
                  onChange={handleInputChange}
                  placeholder="ระบุความต้องการพิเศษหรือหมายเหตุ..."
                  rows={3}
                  className="w-full px-3 py-2 bg-gray-100 border border-gray-300 text-gray-800 rounded-lg focus:ring-2 focus:ring-red-500 focus:border-red-500 outline-hidden transition-colors resize-none"
                  disabled={saving}
                />
              </div>
            </div>

            {/* Footer */}
            <div className="p-4 border-t border-gray-200 bg-gray-100 flex items-center justify-between gap-3">
              {/* Cancel Booking Button (only in edit mode for active bookings) */}
              {canCancel && (
                <div>
                  {showCancelConfirm ? (
                    <div className="flex items-center gap-2">
                      <span className="text-sm text-red-600">ยืนยันการยกเลิก?</span>
                      <button
                        type="button"
                        onClick={handleCancel}
                        disabled={cancelling}
                        className="px-3 py-1.5 bg-red-600 text-white text-sm rounded-lg hover:bg-red-700 disabled:opacity-50 transition-colors"
                      >
                        {cancelling ? (
                          <Loader2 className="w-4 h-4 animate-spin" />
                        ) : (
                          'ใช่'
                        )}
                      </button>
                      <button
                        type="button"
                        onClick={() => setShowCancelConfirm(false)}
                        className="px-3 py-1.5 bg-gray-100 text-gray-800 text-sm rounded-lg hover:bg-gray-100 transition-colors"
                      >
                        ไม่
                      </button>
                    </div>
                  ) : (
                    <button
                      type="button"
                      onClick={() => setShowCancelConfirm(true)}
                      className="flex items-center gap-2 px-4 py-2 text-red-600 hover:bg-red-500/10 rounded-lg transition-colors"
                    >
                      <Trash2 className="w-4 h-4" />
                      ยกเลิกการจอง
                    </button>
                  )}
                </div>
              )}

              {/* Spacer when no cancel button */}
              {!canCancel && <div />}

              {/* Save/Cancel Buttons */}
              <div className="flex items-center gap-3">
                <button
                  type="button"
                  onClick={onClose}
                  className="px-4 py-2 bg-gray-100 hover:bg-gray-100 text-gray-800 rounded-lg transition-colors"
                  disabled={saving}
                >
                  ปิด
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
                  {mode === 'create' ? 'สร้างการจอง' : 'บันทึก'}
                </button>
              </div>
            </div>
          </form>
        </div>
      </div>

      {/* Customer Form Modal */}
      <CustomerForm
        isOpen={showCustomerForm}
        onClose={() => setShowCustomerForm(false)}
        onSave={handleSaveNewCustomer}
        mode="create"
      />
    </>
  )
}
