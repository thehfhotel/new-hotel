'use client'

import { useState, useEffect, useCallback } from 'react'
import {
  Percent,
  Plus,
  Search,
  X,
  Edit3,
  Trash2,
  Loader2,
  AlertCircle,
  CheckCircle,
  XCircle,
  Filter,
  Calendar,
  Tag,
  Home,
} from 'lucide-react'
import RateForm, { RateFormData } from '@/components/forms/RateForm'
import RateCalendar, { Rate } from '@/components/ui/RateCalendar'
import { useBranchFetch } from '@/lib/use-branch-fetch'

interface RoomType {
  id: number
  typeCode: string
  typeName: string
  basePrice: number
  active: boolean
}

interface RateRecord {
  id: number
  rateName: string
  roomTypeId: number
  roomTypeName?: string
  roomTypeCode?: string
  rateType: 'multiplier' | 'fixed'
  rateValue: number
  validFrom: string
  validTo: string
  daysOfWeek: number[]
  active: boolean
  createdAt?: string
  updatedAt?: string
}

const DAYS_OF_WEEK_SHORT: Record<number, string> = {
  0: 'อา',
  1: 'จ',
  2: 'อ',
  3: 'พ',
  4: 'พฤ',
  5: 'ศ',
  6: 'ส',
}

// Convert to Buddhist Era for display
function formatDateBE(dateStr: string | null): string {
  if (!dateStr) return '-'
  const date = new Date(dateStr)
  const year = date.getFullYear() + 543
  const month = String(date.getMonth() + 1).padStart(2, '0')
  const day = String(date.getDate()).padStart(2, '0')
  return `${day}/${month}/${year.toString().slice(-2)}`
}

export default function RatesPage() {
  const branchFetch = useBranchFetch()

  // State
  const [rates, setRates] = useState<RateRecord[]>([])
  const [roomTypes, setRoomTypes] = useState<RoomType[]>([])
  const [loading, setLoading] = useState(true)
  const [error, setError] = useState<string | null>(null)

  // Filters
  const [searchQuery, setSearchQuery] = useState('')
  const [debouncedSearch, setDebouncedSearch] = useState('')
  const [roomTypeFilter, setRoomTypeFilter] = useState<number | ''>('')
  const [activeOnlyFilter, setActiveOnlyFilter] = useState(false)

  // Form modal state
  const [showForm, setShowForm] = useState(false)
  const [editingRate, setEditingRate] = useState<RateFormData | null>(null)
  const [showDeleteConfirm, setShowDeleteConfirm] = useState<number | null>(null)
  const [deleting, setDeleting] = useState(false)

  // Calendar preview state
  const [calendarRoomType, setCalendarRoomType] = useState<number | null>(null)
  const [calendarMonth, setCalendarMonth] = useState(new Date().getMonth())
  const [calendarYear, setCalendarYear] = useState(new Date().getFullYear())

  // Debounce search input
  useEffect(() => {
    const timer = setTimeout(() => {
      setDebouncedSearch(searchQuery)
    }, 300)
    return () => clearTimeout(timer)
  }, [searchQuery])

  // Fetch room types
  const fetchRoomTypes = useCallback(async () => {
    try {
      const response = await branchFetch('/api/new/room-types')
      if (!response.ok) throw new Error('Failed to fetch room types')
      const data = await response.json()
      if (data.success) {
        setRoomTypes(data.data || [])
      }
    } catch (err) {
      console.error('Error fetching room types:', err)
    }
  }, [branchFetch])

  // Fetch rates
  const fetchRates = useCallback(async () => {
    setLoading(true)
    setError(null)
    try {
      const params = new URLSearchParams()
      if (debouncedSearch) params.append('search', debouncedSearch)
      if (roomTypeFilter) params.append('roomTypeId', roomTypeFilter.toString())
      if (activeOnlyFilter) params.append('activeOnly', 'true')

      const response = await branchFetch(`/api/new/rates?${params}`)
      if (!response.ok) throw new Error('Failed to fetch rates')
      const data = await response.json()
      if (data.success) {
        setRates(data.data || [])
      } else {
        throw new Error(data.message || 'Failed to fetch rates')
      }
    } catch (err) {
      setError(err instanceof Error ? err.message : 'เกิดข้อผิดพลาดในการโหลดข้อมูล')
    } finally {
      setLoading(false)
    }
  }, [branchFetch, debouncedSearch, roomTypeFilter, activeOnlyFilter])

  useEffect(() => {
    fetchRoomTypes()
  }, [fetchRoomTypes])

  useEffect(() => {
    fetchRates()
  }, [fetchRates])

  // Handle add new rate
  const handleAddRate = () => {
    setEditingRate(null)
    setShowForm(true)
  }

  // Handle edit rate
  const handleEditRate = (rate: RateRecord) => {
    setEditingRate({
      id: rate.id,
      rateName: rate.rateName,
      roomTypeId: rate.roomTypeId,
      rateType: rate.rateType,
      rateValue: rate.rateValue,
      validFrom: rate.validFrom.split('T')[0],
      validTo: rate.validTo.split('T')[0],
      daysOfWeek: rate.daysOfWeek,
      active: rate.active,
    })
    setShowForm(true)
  }

  // Handle save rate
  const handleSaveRate = async (data: RateFormData) => {
    const endpoint = data.id ? `/api/new/rates/${data.id}` : '/api/new/rates'
    const method = data.id ? 'PUT' : 'POST'

    const response = await branchFetch(endpoint, {
      method,
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({
        rateName: data.rateName,
        roomTypeId: data.roomTypeId,
        rateType: data.rateType,
        rateValue: data.rateValue,
        validFrom: data.validFrom,
        validTo: data.validTo,
        daysOfWeek: data.daysOfWeek,
        active: data.active,
      }),
    })

    const result = await response.json()

    if (!response.ok || !result.success) {
      throw new Error(result.message || 'Failed to save rate')
    }

    fetchRates()
  }

  // Handle delete rate
  const handleDeleteRate = async (id: number) => {
    setDeleting(true)
    try {
      const response = await branchFetch(`/api/new/rates/${id}`, {
        method: 'DELETE',
      })

      const result = await response.json()

      if (!response.ok || !result.success) {
        throw new Error(result.message || 'Failed to delete rate')
      }

      fetchRates()
    } catch (err) {
      setError(err instanceof Error ? err.message : 'เกิดข้อผิดพลาดในการลบ')
    } finally {
      setDeleting(false)
      setShowDeleteConfirm(null)
    }
  }

  // Close form
  const handleCloseForm = () => {
    setShowForm(false)
    setEditingRate(null)
  }

  // Reset filters
  const handleResetFilters = () => {
    setSearchQuery('')
    setRoomTypeFilter('')
    setActiveOnlyFilter(false)
  }

  // Get room type info
  const getRoomTypeInfo = (roomTypeId: number) => {
    return roomTypes.find((rt) => rt.id === roomTypeId)
  }

  // Format days of week display
  const formatDaysOfWeek = (days: number[]) => {
    if (days.length === 7) return 'ทุกวัน'
    if (days.length === 5 && !days.includes(0) && !days.includes(6)) return 'จ-ศ'
    if (days.length === 2 && days.includes(0) && days.includes(6)) return 'ส-อา'
    return days.map((d) => DAYS_OF_WEEK_SHORT[d]).join(', ')
  }

  // Format rate value display
  const formatRateValue = (rate: RateRecord) => {
    if (rate.rateType === 'multiplier') {
      const percent = ((rate.rateValue - 1) * 100).toFixed(0)
      if (rate.rateValue < 1) {
        return `ลด ${Math.abs(parseFloat(percent))}%`
      } else if (rate.rateValue > 1) {
        return `เพิ่ม ${percent}%`
      }
      return 'ราคาปกติ'
    }
    return `${rate.rateValue.toLocaleString()} บาท`
  }

  // Calculate effective price example
  const getEffectivePriceExample = (rate: RateRecord) => {
    const roomType = getRoomTypeInfo(rate.roomTypeId)
    if (!roomType) return null

    const basePrice = roomType.basePrice
    let effectivePrice: number

    if (rate.rateType === 'multiplier') {
      effectivePrice = basePrice * rate.rateValue
    } else {
      effectivePrice = rate.rateValue
    }

    return {
      basePrice,
      effectivePrice,
      discount: basePrice - effectivePrice,
    }
  }

  // Convert rates for calendar component
  const calendarRates: Rate[] = rates.map((r) => ({
    id: r.id,
    rateName: r.rateName,
    roomTypeId: r.roomTypeId,
    rateType: r.rateType,
    rateValue: r.rateValue,
    validFrom: r.validFrom,
    validTo: r.validTo,
    daysOfWeek: r.daysOfWeek,
    active: r.active,
  }))

  return (
    <div className="space-y-6">
      {/* Page Header */}
      <div className="flex items-center justify-between">
        <div className="flex items-center gap-3">
          <Percent className="w-8 h-8 text-red-600" />
          <div>
            <h1 className="text-2xl font-bold text-gray-900">จัดการอัตราพิเศษ</h1>
            <p className="text-gray-500">Special Rate Management</p>
          </div>
        </div>

        {/* Add Button */}
        <button
          onClick={handleAddRate}
          className="flex items-center gap-2 px-4 py-2 bg-red-600 text-white rounded-lg hover:bg-red-700 transition-colors"
        >
          <Plus className="w-5 h-5" />
          เพิ่มอัตรา
        </button>
      </div>

      {/* Filters */}
      <div className="bg-white rounded-lg p-4">
        <div className="flex items-center gap-2 mb-4">
          <Filter className="w-5 h-5 text-gray-500" />
          <span className="font-medium text-gray-700">ตัวกรอง</span>
        </div>

        <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-4">
          {/* Search */}
          <div className="relative">
            <Search className="absolute left-3 top-1/2 -translate-y-1/2 w-5 h-5 text-gray-500" />
            <input
              type="text"
              value={searchQuery}
              onChange={(e) => setSearchQuery(e.target.value)}
              placeholder="ค้นหาชื่ออัตรา..."
              className="w-full pl-10 pr-4 py-2 bg-white text-gray-800 border border-gray-300 rounded-lg focus:ring-2 focus:ring-red-500 focus:border-red-500 outline-hidden transition-colors"
            />
            {searchQuery && (
              <button
                onClick={() => setSearchQuery('')}
                className="absolute right-3 top-1/2 -translate-y-1/2 text-gray-500 hover:text-gray-400"
                aria-label="ล้างการค้นหา"
              >
                <X className="w-4 h-4" />
              </button>
            )}
          </div>

          {/* Room Type Filter */}
          <div>
            <select
              value={roomTypeFilter}
              onChange={(e) => setRoomTypeFilter(e.target.value ? parseInt(e.target.value) : '')}
              className="w-full px-3 py-2 bg-white text-gray-800 border border-gray-300 rounded-lg focus:ring-2 focus:ring-red-500 focus:border-red-500 outline-hidden transition-colors"
            >
              <option value="">ประเภทห้องทั้งหมด</option>
              {roomTypes.map((rt) => (
                <option key={rt.id} value={rt.id}>
                  {rt.typeName} ({rt.typeCode})
                </option>
              ))}
            </select>
          </div>

          {/* Active Only Toggle */}
          <div className="flex items-center">
            <label className="flex items-center gap-2 cursor-pointer">
              <input
                type="checkbox"
                checked={activeOnlyFilter}
                onChange={(e) => setActiveOnlyFilter(e.target.checked)}
                className="w-4 h-4 text-red-600 rounded focus:ring-red-500"
              />
              <span className="text-sm text-gray-700">แสดงเฉพาะที่เปิดใช้งาน</span>
            </label>
          </div>

          {/* Reset Button */}
          <div>
            <button
              onClick={handleResetFilters}
              className="w-full px-4 py-2 border border-gray-300 rounded-lg text-gray-700 hover:bg-gray-100 transition-colors"
            >
              ล้างตัวกรอง
            </button>
          </div>
        </div>

        {debouncedSearch && (
          <p className="mt-2 text-sm text-gray-500">
            ผลการค้นหา &quot;{debouncedSearch}&quot;: พบ {rates.length} รายการ
          </p>
        )}
      </div>

      {/* Main Content Grid */}
      <div className="grid grid-cols-1 lg:grid-cols-3 gap-6">
        {/* Rates Table */}
        <div className="lg:col-span-2">
          {/* Error Message */}
          {error && (
            <div className="flex items-center gap-2 p-4 bg-red-50 border border-red-200 rounded-lg text-red-700 mb-4">
              <AlertCircle className="w-5 h-5 shrink-0" />
              <span>{error}</span>
            </div>
          )}

          <div className="bg-white rounded-lg overflow-hidden">
            {loading ? (
              <div className="flex items-center justify-center py-12">
                <Loader2 className="w-6 h-6 animate-spin text-red-600" />
                <span className="ml-2 text-gray-500">กำลังโหลดข้อมูล...</span>
              </div>
            ) : rates.length === 0 ? (
              <div className="text-center py-12 text-gray-500">
                <Percent className="w-12 h-12 text-gray-300 mx-auto mb-3" />
                <p>ไม่พบอัตราพิเศษ</p>
                <button
                  onClick={handleAddRate}
                  className="mt-4 text-red-600 hover:text-red-600 font-medium"
                >
                  + เพิ่มอัตราใหม่
                </button>
              </div>
            ) : (
              <div className="overflow-x-auto">
                <table className="w-full">
                  <thead className="bg-gray-100 border-b border-gray-200">
                    <tr>
                      <th className="px-4 py-3 text-left text-sm font-semibold text-gray-700">
                        ชื่ออัตรา
                      </th>
                      <th className="px-4 py-3 text-left text-sm font-semibold text-gray-700">
                        ประเภทห้อง
                      </th>
                      <th className="px-4 py-3 text-left text-sm font-semibold text-gray-700">
                        ค่า
                      </th>
                      <th className="px-4 py-3 text-left text-sm font-semibold text-gray-700">
                        ช่วงเวลา
                      </th>
                      <th className="px-4 py-3 text-left text-sm font-semibold text-gray-700">
                        วัน
                      </th>
                      <th className="px-4 py-3 text-left text-sm font-semibold text-gray-700">
                        สถานะ
                      </th>
                      <th className="px-4 py-3 text-right text-sm font-semibold text-gray-700">
                        จัดการ
                      </th>
                    </tr>
                  </thead>
                  <tbody className="divide-y divide-gray-200">
                    {rates.map((rate) => {
                      const priceExample = getEffectivePriceExample(rate)
                      const roomType = getRoomTypeInfo(rate.roomTypeId)

                      return (
                        <tr
                          key={rate.id}
                          className={`hover:bg-gray-100 transition-colors ${
                            !rate.active ? 'opacity-60' : ''
                          }`}
                        >
                          <td className="px-4 py-3">
                            <div className="flex items-center gap-2">
                              <Tag className="w-4 h-4 text-red-600" />
                              <span className="font-medium text-gray-900">{rate.rateName}</span>
                            </div>
                          </td>
                          <td className="px-4 py-3">
                            <div className="flex items-center gap-2">
                              <Home className="w-4 h-4 text-gray-500" />
                              <span className="text-sm text-gray-500">
                                {roomType?.typeName || '-'}
                              </span>
                            </div>
                          </td>
                          <td className="px-4 py-3">
                            <div>
                              <span
                                className={`inline-block px-2 py-1 text-xs rounded-full ${
                                  rate.rateType === 'multiplier'
                                    ? rate.rateValue < 1
                                      ? 'bg-emerald-500/10 text-emerald-400'
                                      : rate.rateValue > 1
                                      ? 'bg-amber-500/10 text-amber-400'
                                      : 'bg-gray-200 text-gray-700'
                                    : 'bg-sky-500/10 text-sky-400'
                                }`}
                              >
                                {formatRateValue(rate)}
                              </span>
                              {priceExample && (
                                <p className="text-xs text-gray-500 mt-1">
                                  {priceExample.basePrice.toLocaleString()} {'->'}{' '}
                                  {priceExample.effectivePrice.toLocaleString()} บาท
                                </p>
                              )}
                            </div>
                          </td>
                          <td className="px-4 py-3">
                            <div className="flex items-center gap-1 text-sm text-gray-500">
                              <Calendar className="w-4 h-4 text-gray-500" />
                              <span>
                                {formatDateBE(rate.validFrom)} - {formatDateBE(rate.validTo)}
                              </span>
                            </div>
                          </td>
                          <td className="px-4 py-3">
                            <span className="text-sm text-gray-500">
                              {formatDaysOfWeek(rate.daysOfWeek)}
                            </span>
                          </td>
                          <td className="px-4 py-3">
                            {rate.active ? (
                              <span className="flex items-center gap-1 text-xs text-green-600">
                                <CheckCircle className="w-3 h-3" />
                                เปิดใช้งาน
                              </span>
                            ) : (
                              <span className="flex items-center gap-1 text-xs text-red-600">
                                <XCircle className="w-3 h-3" />
                                ปิดใช้งาน
                              </span>
                            )}
                          </td>
                          <td className="px-4 py-3">
                            <div className="flex items-center justify-end gap-1">
                              <button
                                onClick={() => handleEditRate(rate)}
                                className="p-2 text-gray-500 hover:text-red-600 hover:bg-red-500/10 rounded-lg transition-colors"
                                title="แก้ไข"
                              >
                                <Edit3 className="w-4 h-4" />
                              </button>
                              {showDeleteConfirm === rate.id ? (
                                <div className="flex items-center gap-1">
                                  <button
                                    onClick={() => handleDeleteRate(rate.id)}
                                    disabled={deleting}
                                    className="px-2 py-1 bg-red-600 text-white text-xs rounded hover:bg-red-700 disabled:opacity-50 transition-colors"
                                  >
                                    {deleting ? (
                                      <Loader2 className="w-3 h-3 animate-spin" />
                                    ) : (
                                      'ลบ'
                                    )}
                                  </button>
                                  <button
                                    onClick={() => setShowDeleteConfirm(null)}
                                    className="px-2 py-1 bg-gray-100 text-gray-800 text-xs rounded hover:bg-gray-200 transition-colors"
                                  >
                                    ยกเลิก
                                  </button>
                                </div>
                              ) : (
                                <button
                                  onClick={() => setShowDeleteConfirm(rate.id)}
                                  className="p-2 text-gray-500 hover:text-red-600 hover:bg-red-500/10 rounded-lg transition-colors"
                                  title="ลบ"
                                >
                                  <Trash2 className="w-4 h-4" />
                                </button>
                              )}
                            </div>
                          </td>
                        </tr>
                      )
                    })}
                  </tbody>
                </table>
              </div>
            )}
          </div>
        </div>

        {/* Rate Calendar Preview */}
        <div className="lg:col-span-1">
          <div className="bg-white rounded-lg p-4 mb-4">
            <h3 className="font-semibold text-gray-700 mb-3">ดูตัวอย่างอัตราในปฏิทิน</h3>
            <select
              value={calendarRoomType ?? ''}
              onChange={(e) =>
                setCalendarRoomType(e.target.value ? parseInt(e.target.value) : null)
              }
              className="w-full px-3 py-2 bg-white text-gray-800 border border-gray-300 rounded-lg focus:ring-2 focus:ring-red-500 focus:border-red-500 outline-hidden transition-colors"
            >
              <option value="">-- เลือกประเภทห้อง --</option>
              {roomTypes.map((rt) => (
                <option key={rt.id} value={rt.id}>
                  {rt.typeName} - {rt.basePrice.toLocaleString()} บาท
                </option>
              ))}
            </select>
          </div>

          <RateCalendar
            roomTypeId={calendarRoomType}
            month={calendarMonth}
            year={calendarYear}
            rates={calendarRates}
            roomTypes={roomTypes}
            onMonthChange={(month, year) => {
              setCalendarMonth(month)
              setCalendarYear(year)
            }}
          />
        </div>
      </div>

      {/* Rate Form Modal */}
      <RateForm
        isOpen={showForm}
        onClose={handleCloseForm}
        onSave={handleSaveRate}
        onDelete={editingRate?.id ? handleDeleteRate : undefined}
        rate={editingRate}
        roomTypes={roomTypes}
      />
    </div>
  )
}
