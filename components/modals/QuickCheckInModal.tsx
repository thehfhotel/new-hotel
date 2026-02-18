'use client'

import { useState, useEffect, useCallback } from 'react'
import { X, Search, Loader2, User, Calendar, DollarSign, FileText } from 'lucide-react'

interface Customer {
  id: number
  firstName: string | null
  lastName: string | null
  phone: string | null
  idCard: string | null
}

interface Room {
  id: number
  roomNumber: string
  type: string
  details?: string
}

interface QuickCheckInModalProps {
  isOpen: boolean
  onClose: () => void
  room: Room
  onSuccess: () => void
}

// Convert Gregorian year to Buddhist Era
function toBuddhistYear(date: Date): string {
  const year = date.getFullYear() + 543
  const month = String(date.getMonth() + 1).padStart(2, '0')
  const day = String(date.getDate()).padStart(2, '0')
  return `${day}/${month}/${year}`
}

// Format date for API (YYYY-MM-DD)
function formatDateForApi(date: Date): string {
  const year = date.getFullYear()
  const month = String(date.getMonth() + 1).padStart(2, '0')
  const day = String(date.getDate()).padStart(2, '0')
  return `${year}-${month}-${day}`
}

export default function QuickCheckInModal({
  isOpen,
  onClose,
  room,
  onSuccess,
}: QuickCheckInModalProps) {
  // Customer search state
  const [searchQuery, setSearchQuery] = useState('')
  const [customers, setCustomers] = useState<Customer[]>([])
  const [selectedCustomer, setSelectedCustomer] = useState<Customer | null>(null)
  const [isSearching, setIsSearching] = useState(false)
  const [showDropdown, setShowDropdown] = useState(false)

  // Form state
  const [expectedCheckout, setExpectedCheckout] = useState('')
  const [ratePerNight, setRatePerNight] = useState('')
  const [adults, setAdults] = useState('1')
  const [children, setChildren] = useState('0')
  const [notes, setNotes] = useState('')

  // Submission state
  const [isSubmitting, setIsSubmitting] = useState(false)
  const [error, setError] = useState<string | null>(null)

  // Set default expected checkout to tomorrow
  useEffect(() => {
    if (isOpen) {
      const tomorrow = new Date()
      tomorrow.setDate(tomorrow.getDate() + 1)
      setExpectedCheckout(formatDateForApi(tomorrow))
    }
  }, [isOpen])

  // Search customers
  const searchCustomers = useCallback(async (query: string) => {
    if (query.length < 2) {
      setCustomers([])
      return
    }

    setIsSearching(true)
    try {
      const res = await fetch(`/api/new/customers?search=${encodeURIComponent(query)}&limit=10`)
      const data = await res.json()
      if (data.success) {
        setCustomers(data.data)
      }
    } catch (err) {
      console.error('Error searching customers:', err)
    } finally {
      setIsSearching(false)
    }
  }, [])

  // Debounced search
  useEffect(() => {
    const timer = setTimeout(() => {
      if (searchQuery) {
        searchCustomers(searchQuery)
        setShowDropdown(true)
      } else {
        setCustomers([])
        setShowDropdown(false)
      }
    }, 300)

    return () => clearTimeout(timer)
  }, [searchQuery, searchCustomers])

  // Handle customer selection
  const handleSelectCustomer = (customer: Customer) => {
    setSelectedCustomer(customer)
    setSearchQuery(
      `${customer.firstName || ''} ${customer.lastName || ''}`.trim() ||
        customer.phone ||
        ''
    )
    setShowDropdown(false)
  }

  // Reset form
  const resetForm = () => {
    setSearchQuery('')
    setSelectedCustomer(null)
    setCustomers([])
    setExpectedCheckout('')
    setRatePerNight('')
    setAdults('1')
    setChildren('0')
    setNotes('')
    setError(null)
  }

  // Handle close
  const handleClose = () => {
    resetForm()
    onClose()
  }

  // Handle submit
  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault()

    if (!selectedCustomer) {
      setError('กรุณาเลือกลูกค้า')
      return
    }

    if (!expectedCheckout) {
      setError('กรุณาเลือกวันที่คาดว่าจะเช็คเอาท์')
      return
    }

    setIsSubmitting(true)
    setError(null)

    try {
      const res = await fetch('/api/new/checkins', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({
          customerId: selectedCustomer.id,
          roomId: room.id,
          expectedCheckout: expectedCheckout,
          adults: parseInt(adults) || 1,
          children: parseInt(children) || 0,
          ratePerNight: parseFloat(ratePerNight) || undefined,
          notes: notes || undefined,
        }),
      })

      const data = await res.json()

      if (!res.ok || !data.success) {
        throw new Error(data.error || data.message || 'เกิดข้อผิดพลาดในการเช็คอิน')
      }

      onSuccess()
      handleClose()
    } catch (err) {
      setError(err instanceof Error ? err.message : 'เกิดข้อผิดพลาดในการเช็คอิน')
    } finally {
      setIsSubmitting(false)
    }
  }

  if (!isOpen) return null

  return (
    <div
      className="fixed inset-0 bg-black/30 flex items-center justify-center z-50"
      onClick={handleClose}
    >
      <div
        className="bg-white border border-gray-200 rounded-xl shadow-2xl w-full max-w-lg mx-4 max-h-[90vh] overflow-y-auto"
        onClick={(e) => e.stopPropagation()}
      >
        {/* Header */}
        <div className="flex items-center justify-between p-4 border-b border-gray-200">
          <div>
            <h2 className="text-xl font-bold text-gray-900">เช็คอินด่วน</h2>
            <p className="text-sm text-gray-500">
              ห้อง {room.roomNumber} - {room.type}
            </p>
          </div>
          <button
            onClick={handleClose}
            className="p-2 hover:bg-gray-100 rounded-lg transition-colors"
          >
            <X size={20} />
          </button>
        </div>

        {/* Form */}
        <form onSubmit={handleSubmit} className="p-4 space-y-4">
          {/* Error Message */}
          {error && (
            <div className="p-3 bg-red-50 border border-red-200 rounded-lg text-red-600 text-sm">
              {error}
            </div>
          )}

          {/* Customer Search */}
          <div className="relative">
            <label className="block text-sm font-medium text-gray-700 mb-1">
              <User size={16} className="inline mr-1" />
              ลูกค้า *
            </label>
            <div className="relative">
              <input
                type="text"
                value={searchQuery}
                onChange={(e) => {
                  setSearchQuery(e.target.value)
                  setSelectedCustomer(null)
                }}
                onFocus={() => customers.length > 0 && setShowDropdown(true)}
                placeholder="ค้นหาด้วยชื่อ, เบอร์โทร, หรือเลขบัตรประชาชน..."
                className="w-full px-3 py-2 pr-10 bg-gray-100 border border-gray-300 text-gray-800 rounded-lg focus:ring-2 focus:ring-red-500 focus:border-red-500"
              />
              {isSearching && (
                <Loader2
                  className="absolute right-3 top-1/2 -translate-y-1/2 animate-spin text-gray-500"
                  size={18}
                />
              )}
              {!isSearching && (
                <Search
                  className="absolute right-3 top-1/2 -translate-y-1/2 text-gray-500"
                  size={18}
                />
              )}
            </div>

            {/* Customer Dropdown */}
            {showDropdown && customers.length > 0 && (
              <div className="absolute z-10 w-full mt-1 bg-white border border-gray-300 rounded-lg shadow-lg max-h-48 overflow-y-auto">
                {customers.map((customer) => (
                  <button
                    key={customer.id}
                    type="button"
                    onClick={() => handleSelectCustomer(customer)}
                    className="w-full px-3 py-2 text-left hover:bg-gray-100 flex items-center gap-2"
                  >
                    <div className="w-8 h-8 bg-red-500/10 rounded-full flex items-center justify-center">
                      <User size={14} className="text-red-600" />
                    </div>
                    <div>
                      <div className="font-medium text-gray-800">
                        {customer.firstName || ''} {customer.lastName || ''}
                      </div>
                      <div className="text-xs text-gray-500">
                        {customer.phone && <span className="mr-2">{customer.phone}</span>}
                        {customer.idCard && <span>{customer.idCard}</span>}
                      </div>
                    </div>
                  </button>
                ))}
              </div>
            )}

            {showDropdown && customers.length === 0 && searchQuery.length >= 2 && !isSearching && (
              <div className="absolute z-10 w-full mt-1 bg-white border border-gray-300 rounded-lg shadow-lg p-3 text-center text-gray-500 text-sm">
                ไม่พบลูกค้า
              </div>
            )}

            {/* Selected Customer Display */}
            {selectedCustomer && (
              <div className="mt-2 p-2 bg-red-500/10 border border-gray-300 rounded-lg text-sm text-red-600">
                เลือกแล้ว: {selectedCustomer.firstName} {selectedCustomer.lastName}
                {selectedCustomer.phone && ` (${selectedCustomer.phone})`}
              </div>
            )}
          </div>

          {/* Expected Checkout Date */}
          <div>
            <label className="block text-sm font-medium text-gray-700 mb-1">
              <Calendar size={16} className="inline mr-1" />
              วันที่คาดว่าจะเช็คเอาท์ *
            </label>
            <input
              type="date"
              value={expectedCheckout}
              onChange={(e) => setExpectedCheckout(e.target.value)}
              min={formatDateForApi(new Date())}
              className="w-full px-3 py-2 bg-gray-100 border border-gray-300 text-gray-800 rounded-lg focus:ring-2 focus:ring-red-500 focus:border-red-500"
              required
            />
            {expectedCheckout && (
              <p className="mt-1 text-xs text-gray-500">
                พ.ศ. {toBuddhistYear(new Date(expectedCheckout))}
              </p>
            )}
          </div>

          {/* Rate per Night */}
          <div>
            <label className="block text-sm font-medium text-gray-700 mb-1">
              <DollarSign size={16} className="inline mr-1" />
              ราคาต่อคืน (บาท)
            </label>
            <input
              type="number"
              value={ratePerNight}
              onChange={(e) => setRatePerNight(e.target.value)}
              placeholder="ระบุราคา..."
              min="0"
              step="0.01"
              className="w-full px-3 py-2 bg-gray-100 border border-gray-300 text-gray-800 rounded-lg focus:ring-2 focus:ring-red-500 focus:border-red-500"
            />
          </div>

          {/* Adults and Children */}
          <div className="grid grid-cols-2 gap-4">
            <div>
              <label className="block text-sm font-medium text-gray-700 mb-1">
                ผู้ใหญ่
              </label>
              <input
                type="number"
                value={adults}
                onChange={(e) => setAdults(e.target.value)}
                min="1"
                className="w-full px-3 py-2 bg-gray-100 border border-gray-300 text-gray-800 rounded-lg focus:ring-2 focus:ring-red-500 focus:border-red-500"
              />
            </div>
            <div>
              <label className="block text-sm font-medium text-gray-700 mb-1">
                เด็ก
              </label>
              <input
                type="number"
                value={children}
                onChange={(e) => setChildren(e.target.value)}
                min="0"
                className="w-full px-3 py-2 bg-gray-100 border border-gray-300 text-gray-800 rounded-lg focus:ring-2 focus:ring-red-500 focus:border-red-500"
              />
            </div>
          </div>

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

          {/* Submit Button */}
          <div className="flex gap-3 pt-2">
            <button
              type="button"
              onClick={handleClose}
              className="flex-1 px-4 py-2 border border-gray-300 text-gray-700 rounded-lg hover:bg-gray-100"
              disabled={isSubmitting}
            >
              ยกเลิก
            </button>
            <button
              type="submit"
              disabled={isSubmitting || !selectedCustomer}
              className="flex-1 px-4 py-2 bg-red-600 text-white rounded-lg hover:bg-red-700 disabled:opacity-50 disabled:cursor-not-allowed flex items-center justify-center gap-2"
            >
              {isSubmitting ? (
                <>
                  <Loader2 className="animate-spin" size={18} />
                  กำลังบันทึก...
                </>
              ) : (
                'เช็คอิน'
              )}
            </button>
          </div>
        </form>
      </div>
    </div>
  )
}
