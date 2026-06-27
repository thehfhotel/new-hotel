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
  ShoppingCart,
  Plus,
  CheckCircle2,
} from 'lucide-react'
import CustomerPicker, { CustomerOption } from '@/components/pickers/CustomerPicker'
import CustomerForm, { CustomerFormData } from '@/components/forms/CustomerForm'
import RoomPicker, { RoomOption } from '@/components/pickers/RoomPicker'
import BookingConfirmationSlip from '@/components/documents/BookingConfirmationSlip'
import PrintButton from '@/components/ui/PrintButton'
import { hotelInfoForBranch } from '@/lib/hotel-info'
import { useBranchFetch } from '@/lib/use-branch-fetch'
import { useBranch } from '@/contexts/BranchContext'

/** A pre-ordered product line attached to a booking (task #52 — the canonical
 *  analog of iHOTEL's FrmAddBook2 / `HT_Book_Pro`). `name`/`unitPrice` are
 *  carried so the confirmation slip can render without a second lookup. */
export interface BookingProductLine {
  productId: number
  name: string
  qty: number
  /** Baht/unit. `null` ⇒ default from the product's catalog price. */
  unitPrice: number | null
}

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
  /** Pre-ordered products. Optional — absent on legacy callers / waitlist. */
  products?: BookingProductLine[]
}

/** Product as surfaced by `GET /api/products` (camelCase, see new_products.rs). */
interface ProductLite {
  id: number
  legacyNo: string
  name: string
  unit: string | null
  price: number
  currentStock: number
  active: boolean
}

interface BookingFormProps {
  isOpen: boolean
  onClose: () => void
  /** On create, returns the new booking's id + bookNo so the form can render
   *  the confirmation slip. Edit/cancel callers may resolve to void. */
  onSave: (data: BookingFormState) => Promise<{ id: number; bookNo: string } | void>
  onCancel?: (id: number) => Promise<void>
  initialData?: BookingFormState | null
  mode: 'create' | 'edit'
  /** Optional create-mode prefill (e.g. reserve-from-cell on the room board). */
  prefill?: { room?: RoomOption; checkIn?: string; checkOut?: string }
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
  products: [],
}

export default function BookingForm({
  isOpen,
  onClose,
  onSave,
  onCancel,
  initialData,
  mode,
  prefill,
}: BookingFormProps) {
  const branchFetch = useBranchFetch()
  // Spike Phase 3 (ship-dark): when BOOKING_VALIDATION_ENABLED is on the form
  // also runs the server-side date + availability check. `branch` is sent in
  // the request body so the backend validates against the right pool.
  const { branch, bookingValidationEnabled } = useBranch()
  const [formData, setFormData] = useState<BookingFormState>(emptyFormData)
  const [selectedCustomer, setSelectedCustomer] = useState<CustomerOption | null>(null)
  const [selectedRooms, setSelectedRooms] = useState<RoomOption[]>([])
  const [saving, setSaving] = useState(false)
  const [cancelling, setCancelling] = useState(false)
  const [error, setError] = useState<string | null>(null)
  const [showCancelConfirm, setShowCancelConfirm] = useState(false)
  const [showCustomerForm, setShowCustomerForm] = useState(false)

  // Pre-order product picker state (task #52).
  const [productCatalog, setProductCatalog] = useState<ProductLite[]>([])
  const [selectedProductId, setSelectedProductId] = useState<number | null>(null)
  const [productQty, setProductQty] = useState<string>('1')

  // After a successful CREATE: switch to the confirmation-slip print panel
  // instead of auto-closing (parity with QuickCheckInModal's success flow).
  const [created, setCreated] = useState<{ id: number; bookNo: string } | null>(null)

  const productLines = formData.products ?? []

  // Set default dates for new booking (prefill-aware).
  useEffect(() => {
    if (isOpen && mode === 'create' && !initialData) {
      const today = new Date()
      const tomorrow = new Date()
      tomorrow.setDate(tomorrow.getDate() + 1)
      setFormData({
        ...emptyFormData,
        checkIn: prefill?.checkIn ?? formatDateForApi(today),
        checkOut: prefill?.checkOut ?? formatDateForApi(tomorrow),
      })
      // TODO(reserve-from-cell): when launched from the room board, `prefill.room`
      // should preselect that room in the RoomPicker (and seed selectedRooms).
      // The board diverged on master; the coordinator is wiring reserve-from-cell
      // separately, so we accept the prop here but do not yet consume prefill.room.
    }
  }, [isOpen, mode, initialData, prefill])

  // Load the active product catalog for the pre-order picker (task #52).
  useEffect(() => {
    if (!isOpen) return
    let cancelled = false
    branchFetch('/api/products?active_only=true&limit=200')
      .then((res) => (res.ok ? res.json() : null))
      .then((body) => {
        if (cancelled || !body) return
        setProductCatalog((body.data ?? []) as ProductLite[])
      })
      .catch(() => {
        /* picker is optional — a load failure just yields an empty list */
      })
    return () => {
      cancelled = true
    }
  }, [isOpen, branchFetch])

  // Reset form when modal opens/closes or initialData changes
  useEffect(() => {
    if (isOpen) {
      if (initialData) {
        setFormData({ ...initialData, products: initialData.products ?? [] })
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
      setCreated(null)
      setSelectedProductId(null)
      setProductQty('1')
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

  const handleAddProduct = () => {
    if (!selectedProductId) return
    const product = productCatalog.find((p) => p.id === selectedProductId)
    if (!product) return
    const qtyNum = Number(productQty)
    if (!Number.isFinite(qtyNum) || qtyNum <= 0) return

    setFormData((prev) => {
      const lines = prev.products ?? []
      const idx = lines.findIndex((l) => l.productId === product.id)
      const next =
        idx >= 0
          ? lines.map((l, i) => (i === idx ? { ...l, qty: l.qty + qtyNum } : l))
          : [
              ...lines,
              {
                productId: product.id,
                name: product.name,
                qty: qtyNum,
                unitPrice: product.price,
              },
            ]
      return { ...prev, products: next }
    })
    setSelectedProductId(null)
    setProductQty('1')
  }

  const handleRemoveProduct = (productId: number) => {
    setFormData((prev) => ({
      ...prev,
      products: (prev.products ?? []).filter((l) => l.productId !== productId),
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

    // Rooms are OPTIONAL (task #52): a zero-room booking is a valid
    // waitlist / unassigned reservation; a room is assigned later via edit.

    // Spike Phase 3 (ship-dark): server-side date + availability validation.
    // Only runs when BOOKING_VALIDATION_ENABLED is on, we're creating a new
    // booking, AND at least one room is selected (a waitlist booking has no
    // room to validate availability against) — edits of existing/past stays are
    // left to the unchanged flow so the new "not in the past" rule never wrongly
    // blocks them. When the flag is off this entire block is skipped → behavior
    // is byte-for-byte unchanged. The check FAILS OPEN: if the call errors we
    // fall through to the normal save rather than block a booking the current
    // flow would accept.
    if (bookingValidationEnabled && mode === 'create' && formData.rooms.length > 0) {
      try {
        const results = await Promise.all(
          formData.rooms.map((r) =>
            fetch('/api/bookings/validate', {
              method: 'POST',
              headers: { 'Content-Type': 'application/json' },
              body: JSON.stringify({
                roomId: r.roomId,
                checkIn: formData.checkIn,
                checkOut: formData.checkOut,
                branch,
                excludeBookingId: initialData?.id ?? null,
              }),
            }).then((res) => (res.ok ? res.json() : null))
          )
        )
        const reasons = new Set<string>()
        for (const d of results) {
          if (d && d.success && (!d.valid || !d.available)) {
            for (const reason of (d.reasons as string[]) ?? []) reasons.add(reason)
          }
        }
        if (reasons.size > 0) {
          setError(Array.from(reasons).join(' • '))
          return
        }
      } catch {
        // Network/parse failure → ship-dark fail-open: proceed with save.
      }
    }

    setSaving(true)
    try {
      const result = await onSave(formData)
      // On create, switch to the confirmation-slip print panel (don't auto-close).
      // Edits — and create callers that don't return ids — close as before.
      if (mode === 'create' && result && 'bookNo' in result) {
        setCreated({ id: result.id, bookNo: result.bookNo })
      } else {
        onClose()
      }
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
    const response = await branchFetch('/api/customers', {
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

          {/* Success → booking-confirmation print panel */}
          {created ? (
            <div className="p-4 space-y-4">
              <div className="flex items-start gap-2 p-3 bg-emerald-50 border border-emerald-200 rounded-lg text-sm text-emerald-800">
                <CheckCircle2 className="w-5 h-5 shrink-0 mt-0.5" />
                <div>
                  <p className="font-medium">สร้างการจองสำเร็จ</p>
                  <p className="text-xs mt-0.5">เลขที่จอง: {created.bookNo}</p>
                </div>
              </div>

              <p className="text-sm text-gray-600">
                พิมพ์ใบยืนยันการจองให้ลูกค้า หรือกด &quot;เสร็จสิ้น&quot; เพื่อปิด
              </p>

              <BookingConfirmationSlip
                hotelInfo={hotelInfoForBranch(branch)}
                data={{
                  bookingNo: created.bookNo,
                  guestName: selectedCustomer
                    ? `${selectedCustomer.firstName || ''} ${selectedCustomer.lastName || ''}`.trim()
                    : formData.customerName || '',
                  guestContact: selectedCustomer?.phone || undefined,
                  checkInDate: formData.checkIn,
                  checkOutDate: formData.checkOut,
                  nights,
                  adults: formData.adults,
                  children: formData.children,
                  rooms: selectedRooms.map((r) => ({
                    roomNumber: r.roomNo,
                    roomType: r.roomTypeName || undefined,
                    ratePerNight: r.priceWeekday ?? undefined,
                  })),
                  products: productLines.map((p) => ({
                    name: p.name,
                    qty: p.qty,
                    unitPrice: p.unitPrice ?? undefined,
                  })),
                  deposit: formData.depositAmount ?? undefined,
                  notes: formData.notes ?? undefined,
                }}
              />

              <div className="flex justify-end gap-2 pt-2 border-t border-gray-200">
                <PrintButton size="sm" showPdfOption={false} />
                <button
                  type="button"
                  onClick={onClose}
                  className="px-4 py-2 text-sm font-medium text-gray-700 bg-white border border-gray-300 rounded-lg hover:bg-gray-100"
                >
                  เสร็จสิ้น
                </button>
              </div>
            </div>
          ) : (
          /* Form */
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

              {/* Room Selection — optional (task #52: waitlist booking) */}
              <div>
                <label className="flex items-center gap-2 text-sm font-medium text-gray-700 mb-2">
                  <Bed className="w-4 h-4" />
                  เลือกห้องพัก
                  <span className="text-xs font-normal text-gray-400">(ไม่บังคับ)</span>
                </label>
                <RoomPicker
                  value={selectedRooms}
                  onChange={handleRoomsChange}
                  checkInDate={formData.checkIn}
                  checkOutDate={formData.checkOut}
                  excludeBookingId={initialData?.id}
                  disabled={saving}
                />
                {selectedRooms.length === 0 && (
                  <p className="mt-2 flex items-center gap-1.5 text-xs text-amber-600">
                    <AlertCircle className="w-3.5 h-3.5 shrink-0" />
                    ยังไม่ระบุห้องพัก (จะบันทึกเป็นรายการรอจัดห้อง)
                  </p>
                )}
              </div>

              {/* Pre-order products — optional (task #52) */}
              <div>
                <label className="flex items-center gap-2 text-sm font-medium text-gray-700 mb-2">
                  <ShoppingCart className="w-4 h-4" />
                  สินค้าสั่งล่วงหน้า
                  <span className="text-xs font-normal text-gray-400">(ไม่บังคับ)</span>
                </label>

                {productLines.length > 0 && (
                  <ul className="mb-2 divide-y divide-gray-200 border border-gray-200 rounded-lg">
                    {productLines.map((line) => (
                      <li
                        key={line.productId}
                        className="flex items-center justify-between px-3 py-2 text-sm"
                      >
                        <span>
                          <span className="font-medium text-gray-800">{line.name}</span>
                          <span className="ml-2 text-xs text-gray-500">× {line.qty}</span>
                        </span>
                        <span className="flex items-center gap-3">
                          <span className="text-gray-700">
                            {line.unitPrice != null
                              ? `${(line.unitPrice * line.qty).toLocaleString('th-TH', {
                                  minimumFractionDigits: 2,
                                  maximumFractionDigits: 2,
                                })} บาท`
                              : '-'}
                          </span>
                          <button
                            type="button"
                            onClick={() => handleRemoveProduct(line.productId)}
                            className="text-gray-400 hover:text-red-600"
                            aria-label="ลบสินค้า"
                            disabled={saving}
                          >
                            <Trash2 className="w-4 h-4" />
                          </button>
                        </span>
                      </li>
                    ))}
                  </ul>
                )}

                <div className="flex gap-2">
                  <select
                    aria-label="เลือกสินค้าสั่งล่วงหน้า"
                    value={selectedProductId ?? ''}
                    onChange={(e) =>
                      setSelectedProductId(e.target.value ? Number(e.target.value) : null)
                    }
                    className="flex-1 px-3 py-2 bg-gray-100 border border-gray-300 text-gray-800 rounded-lg focus:ring-2 focus:ring-red-500 focus:border-red-500 outline-hidden transition-colors"
                    disabled={saving}
                  >
                    <option value="">-- เลือกสินค้า --</option>
                    {productCatalog.map((p) => (
                      <option key={p.id} value={p.id}>
                        {p.name} ({p.price.toLocaleString('th-TH')} บาท)
                      </option>
                    ))}
                  </select>
                  <input
                    type="number"
                    aria-label="จำนวนสินค้า"
                    min="1"
                    step="1"
                    value={productQty}
                    onChange={(e) => setProductQty(e.target.value)}
                    className="w-20 px-3 py-2 bg-gray-100 border border-gray-300 text-gray-800 rounded-lg focus:ring-2 focus:ring-red-500 focus:border-red-500 outline-hidden transition-colors"
                    disabled={saving}
                  />
                  <button
                    type="button"
                    onClick={handleAddProduct}
                    disabled={!selectedProductId || saving}
                    className="flex items-center gap-1 px-3 py-2 bg-gray-800 text-white rounded-lg hover:bg-gray-700 disabled:opacity-50 transition-colors"
                  >
                    <Plus className="w-4 h-4" />
                    เพิ่ม
                  </button>
                </div>
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
          )}
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
