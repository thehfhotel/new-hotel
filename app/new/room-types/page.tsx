'use client'

import { useState, useEffect, useCallback } from 'react'
import {
  Home,
  Plus,
  Search,
  X,
  Edit3,
  Trash2,
  Users,
  Bed,
  DollarSign,
  Loader2,
  AlertCircle,
  CheckCircle,
  XCircle,
} from 'lucide-react'
import RoomTypeForm, { RoomTypeFormData } from '@/components/forms/RoomTypeForm'

interface RoomType {
  id: number
  typeCode: string
  typeName: string
  typeNameEn: string | null
  description: string | null
  basePrice: number
  maxGuests: number
  bedType: string | null
  sizeSqm: number | null
  active: boolean
  createdAt: string | null
  updatedAt: string | null
}

type SortField = 'typeCode' | 'typeName'
type SortOrder = 'asc' | 'desc'

export default function RoomTypesPage() {
  const [roomTypes, setRoomTypes] = useState<RoomType[]>([])
  const [loading, setLoading] = useState(true)
  const [error, setError] = useState<string | null>(null)
  const [searchQuery, setSearchQuery] = useState('')
  const [debouncedSearch, setDebouncedSearch] = useState('')
  const [sortField, setSortField] = useState<SortField>('typeCode')
  const [sortOrder, setSortOrder] = useState<SortOrder>('asc')

  // Form modal state
  const [showForm, setShowForm] = useState(false)
  const [formMode, setFormMode] = useState<'create' | 'edit'>('create')
  const [selectedRoomType, setSelectedRoomType] = useState<RoomTypeFormData | null>(null)

  // Debounce search input
  useEffect(() => {
    const timer = setTimeout(() => {
      setDebouncedSearch(searchQuery)
    }, 300)

    return () => clearTimeout(timer)
  }, [searchQuery])

  // Fetch room types
  const fetchRoomTypes = useCallback(async () => {
    setLoading(true)
    setError(null)
    try {
      const params = new URLSearchParams({
        sortBy: sortField,
        sortOrder: sortOrder,
      })

      if (debouncedSearch) {
        params.append('search', debouncedSearch)
      }

      const response = await fetch(`/api/new/room-types?${params}`)
      if (!response.ok) {
        throw new Error('Failed to fetch room types')
      }

      const data = await response.json()
      if (data.success) {
        setRoomTypes(data.data || [])
      } else {
        throw new Error(data.message || 'Failed to fetch room types')
      }
    } catch (err) {
      setError(err instanceof Error ? err.message : 'เกิดข้อผิดพลาดในการโหลดข้อมูล')
    } finally {
      setLoading(false)
    }
  }, [debouncedSearch, sortField, sortOrder])

  useEffect(() => {
    fetchRoomTypes()
  }, [fetchRoomTypes])

  // Handle sort change
  const handleSortChange = (field: SortField) => {
    if (sortField === field) {
      setSortOrder(sortOrder === 'asc' ? 'desc' : 'asc')
    } else {
      setSortField(field)
      setSortOrder('asc')
    }
  }

  // Handle add new room type
  const handleAddRoomType = () => {
    setSelectedRoomType(null)
    setFormMode('create')
    setShowForm(true)
  }

  // Handle edit room type
  const handleEditRoomType = (roomType: RoomType) => {
    setSelectedRoomType({
      id: roomType.id,
      typeCode: roomType.typeCode,
      typeName: roomType.typeName,
      typeNameEn: roomType.typeNameEn || '',
      description: roomType.description || '',
      basePrice: roomType.basePrice,
      maxGuests: roomType.maxGuests,
      bedType: roomType.bedType || '',
      sizeSqm: roomType.sizeSqm,
      active: roomType.active,
    })
    setFormMode('edit')
    setShowForm(true)
  }

  // Handle save room type
  const handleSaveRoomType = async (data: RoomTypeFormData) => {
    const endpoint = data.id
      ? `/api/new/room-types/${data.id}`
      : '/api/new/room-types'
    const method = data.id ? 'PUT' : 'POST'

    const response = await fetch(endpoint, {
      method,
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({
        typeCode: data.typeCode.toUpperCase(),
        typeName: data.typeName,
        typeNameEn: data.typeNameEn || null,
        description: data.description || null,
        basePrice: data.basePrice,
        maxGuests: data.maxGuests,
        bedType: data.bedType || null,
        sizeSqm: data.sizeSqm,
        active: data.active,
      }),
    })

    const result = await response.json()

    if (!response.ok || !result.success) {
      throw new Error(result.message || 'Failed to save room type')
    }

    // Refresh the list
    fetchRoomTypes()
  }

  // Handle delete room type
  const handleDeleteRoomType = async (id: number) => {
    const response = await fetch(`/api/new/room-types/${id}`, {
      method: 'DELETE',
    })

    const result = await response.json()

    if (!response.ok || !result.success) {
      throw new Error(result.message || 'Failed to delete room type')
    }

    // Refresh the list
    fetchRoomTypes()
  }

  // Close form
  const handleCloseForm = () => {
    setShowForm(false)
    setSelectedRoomType(null)
  }

  // Format price
  const formatPrice = (price: number) => {
    return price.toLocaleString('th-TH', {
      minimumFractionDigits: 0,
      maximumFractionDigits: 2,
    })
  }

  // Get bed type label
  const getBedTypeLabel = (bedType: string | null) => {
    const labels: Record<string, string> = {
      Single: 'เตียงเดี่ยว',
      Double: 'เตียงคู่',
      Twin: 'เตียงแฝด',
      King: 'เตียงคิงไซส์',
    }
    return bedType ? labels[bedType] || bedType : '-'
  }

  return (
    <div className="space-y-6">
      {/* Page Header */}
      <div className="flex items-center justify-between">
        <div className="flex items-center gap-3">
          <Home className="w-8 h-8 text-purple-600" />
          <div>
            <h1 className="text-2xl font-bold text-gray-800">จัดการประเภทห้อง</h1>
            <p className="text-gray-600">
              จำนวนประเภทห้องทั้งหมด {roomTypes.length} รายการ
            </p>
          </div>
        </div>

        {/* Add Button */}
        <button
          onClick={handleAddRoomType}
          className="flex items-center gap-2 px-4 py-2 bg-purple-600 text-white rounded-lg hover:bg-purple-700 transition-colors"
        >
          <Plus className="w-5 h-5" />
          เพิ่มประเภทห้อง
        </button>
      </div>

      {/* Search and Sort Bar */}
      <div className="bg-white rounded-lg shadow p-4">
        <div className="flex flex-col md:flex-row gap-4">
          {/* Search */}
          <div className="relative flex-1">
            <Search className="absolute left-3 top-1/2 -translate-y-1/2 w-5 h-5 text-gray-400" />
            <input
              type="text"
              value={searchQuery}
              onChange={(e) => setSearchQuery(e.target.value)}
              placeholder="ค้นหาด้วยรหัส หรือ ชื่อประเภทห้อง..."
              className="w-full pl-10 pr-4 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-purple-500 focus:border-purple-500 outline-none transition-colors"
            />
            {searchQuery && (
              <button
                onClick={() => setSearchQuery('')}
                className="absolute right-3 top-1/2 -translate-y-1/2 text-gray-400 hover:text-gray-600"
                aria-label="ล้างการค้นหา"
              >
                <X className="w-4 h-4" />
              </button>
            )}
          </div>

          {/* Sort */}
          <div className="flex items-center gap-2">
            <span className="text-sm text-gray-600">เรียงตาม:</span>
            <button
              onClick={() => handleSortChange('typeCode')}
              className={`px-3 py-1.5 text-sm rounded-lg border transition-colors ${
                sortField === 'typeCode'
                  ? 'border-purple-500 bg-purple-50 text-purple-700'
                  : 'border-gray-300 text-gray-600 hover:bg-gray-50'
              }`}
            >
              รหัส {sortField === 'typeCode' && (sortOrder === 'asc' ? '(A-Z)' : '(Z-A)')}
            </button>
            <button
              onClick={() => handleSortChange('typeName')}
              className={`px-3 py-1.5 text-sm rounded-lg border transition-colors ${
                sortField === 'typeName'
                  ? 'border-purple-500 bg-purple-50 text-purple-700'
                  : 'border-gray-300 text-gray-600 hover:bg-gray-50'
              }`}
            >
              ชื่อ {sortField === 'typeName' && (sortOrder === 'asc' ? '(A-Z)' : '(Z-A)')}
            </button>
          </div>
        </div>
        {debouncedSearch && (
          <p className="mt-2 text-sm text-gray-600">
            ผลการค้นหา &quot;{debouncedSearch}&quot;: พบ {roomTypes.length} รายการ
          </p>
        )}
      </div>

      {/* Error Message */}
      {error && (
        <div className="flex items-center gap-2 p-4 bg-red-50 border border-red-200 rounded-lg text-red-700">
          <AlertCircle className="w-5 h-5 flex-shrink-0" />
          <span>{error}</span>
        </div>
      )}

      {/* Room Types Grid */}
      <div className="bg-white rounded-lg shadow overflow-hidden">
        {loading ? (
          <div className="flex items-center justify-center py-12">
            <Loader2 className="w-6 h-6 animate-spin text-purple-600" />
            <span className="ml-2 text-gray-600">กำลังโหลดข้อมูล...</span>
          </div>
        ) : roomTypes.length === 0 ? (
          <div className="text-center py-12 text-gray-500">
            <Home className="w-12 h-12 text-gray-300 mx-auto mb-3" />
            <p>ไม่พบประเภทห้อง</p>
            <button
              onClick={handleAddRoomType}
              className="mt-4 text-purple-600 hover:text-purple-700 font-medium"
            >
              + เพิ่มประเภทห้องใหม่
            </button>
          </div>
        ) : (
          <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4 p-4">
            {roomTypes.map((roomType) => (
              <div
                key={roomType.id}
                className={`border rounded-lg p-4 transition-all hover:shadow-md ${
                  roomType.active
                    ? 'border-gray-200 bg-white'
                    : 'border-gray-200 bg-gray-50 opacity-60'
                }`}
              >
                {/* Header */}
                <div className="flex items-start justify-between mb-3">
                  <div>
                    <div className="flex items-center gap-2">
                      <span className="px-2 py-0.5 bg-purple-100 text-purple-700 text-xs font-bold rounded">
                        {roomType.typeCode}
                      </span>
                      {roomType.active ? (
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
                    </div>
                    <h3 className="text-lg font-semibold text-gray-800 mt-1">
                      {roomType.typeName}
                    </h3>
                    {roomType.typeNameEn && (
                      <p className="text-sm text-gray-500">{roomType.typeNameEn}</p>
                    )}
                  </div>
                  <div className="flex gap-1">
                    <button
                      onClick={() => handleEditRoomType(roomType)}
                      className="p-2 text-gray-400 hover:text-purple-600 hover:bg-purple-50 rounded-lg transition-colors"
                      title="แก้ไข"
                    >
                      <Edit3 className="w-4 h-4" />
                    </button>
                  </div>
                </div>

                {/* Description */}
                {roomType.description && (
                  <p className="text-sm text-gray-600 mb-3 line-clamp-2">
                    {roomType.description}
                  </p>
                )}

                {/* Details */}
                <div className="grid grid-cols-2 gap-2 text-sm">
                  <div className="flex items-center gap-2 text-gray-600">
                    <DollarSign className="w-4 h-4 text-gray-400" />
                    <span>{formatPrice(roomType.basePrice)} บาท</span>
                  </div>
                  <div className="flex items-center gap-2 text-gray-600">
                    <Users className="w-4 h-4 text-gray-400" />
                    <span>{roomType.maxGuests} คน</span>
                  </div>
                  <div className="flex items-center gap-2 text-gray-600">
                    <Bed className="w-4 h-4 text-gray-400" />
                    <span>{getBedTypeLabel(roomType.bedType)}</span>
                  </div>
                  {roomType.sizeSqm && (
                    <div className="flex items-center gap-2 text-gray-600">
                      <Home className="w-4 h-4 text-gray-400" />
                      <span>{roomType.sizeSqm} ตร.ม.</span>
                    </div>
                  )}
                </div>
              </div>
            ))}
          </div>
        )}
      </div>

      {/* Room Type Form Modal */}
      <RoomTypeForm
        isOpen={showForm}
        onClose={handleCloseForm}
        onSave={handleSaveRoomType}
        onDelete={handleDeleteRoomType}
        initialData={selectedRoomType}
        mode={formMode}
      />
    </div>
  )
}
