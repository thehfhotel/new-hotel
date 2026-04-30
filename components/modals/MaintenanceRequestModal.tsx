'use client'

import { useState, useEffect } from 'react'
import {
  X,
  Loader2,
  AlertCircle,
  Wrench,
  DoorOpen,
  Tag,
  FileText,
  User,
  DollarSign,
} from 'lucide-react'
import { MaintenanceRequest, MaintenanceCategory, MaintenancePriority } from '@/types/reports'
import { useBranchFetch } from '@/lib/use-branch-fetch'

interface MaintenanceRequestModalProps {
  isOpen: boolean
  onClose: () => void
  onSuccess: () => void
  editRequest?: MaintenanceRequest | null
  categories: MaintenanceCategory[]
  rooms: Array<{ id: number; roomNo: string; roomTypeName: string | null }>
}

const priorityOptions: { value: MaintenancePriority; label: string }[] = [
  { value: 3, label: 'สูง (ด่วน)' },
  { value: 2, label: 'ปานกลาง' },
  { value: 1, label: 'ต่ำ' },
]

export default function MaintenanceRequestModal({
  isOpen,
  onClose,
  onSuccess,
  editRequest = null,
  categories,
  rooms,
}: MaintenanceRequestModalProps) {
  const branchFetch = useBranchFetch()
  const isEditing = !!editRequest

  // Form state
  const [roomId, setRoomId] = useState<number | ''>('')
  const [categoryId, setCategoryId] = useState<number | ''>('')
  const [title, setTitle] = useState('')
  const [description, setDescription] = useState('')
  const [priority, setPriority] = useState<MaintenancePriority>(2)
  const [assignedTo, setAssignedTo] = useState('')
  const [resolution, setResolution] = useState('')
  const [cost, setCost] = useState('')

  // Submission state
  const [isSubmitting, setIsSubmitting] = useState(false)
  const [error, setError] = useState<string | null>(null)

  // Initialize form when modal opens or editRequest changes
  useEffect(() => {
    if (isOpen) {
      if (editRequest) {
        setRoomId(editRequest.roomId)
        setCategoryId(editRequest.categoryId)
        setTitle(editRequest.title)
        setDescription(editRequest.description || '')
        setPriority(editRequest.priority)
        setAssignedTo(editRequest.assignedTo || '')
        setResolution(editRequest.resolution || '')
        setCost(editRequest.cost !== null ? editRequest.cost.toString() : '')
      } else {
        resetForm()
      }
    }
  }, [isOpen, editRequest])

  // Reset form
  const resetForm = () => {
    setRoomId('')
    setCategoryId('')
    setTitle('')
    setDescription('')
    setPriority(2)
    setAssignedTo('')
    setResolution('')
    setCost('')
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

    // Validation
    if (!roomId) {
      setError('กรุณาเลือกห้อง')
      return
    }
    if (!categoryId) {
      setError('กรุณาเลือกหมวดหมู่')
      return
    }
    if (!title.trim()) {
      setError('กรุณาระบุหัวข้อ')
      return
    }

    setIsSubmitting(true)
    setError(null)

    try {
      if (isEditing && editRequest) {
        // Update existing request
        const updateBody: Record<string, unknown> = {}

        if (title.trim() !== editRequest.title) updateBody.title = title.trim()
        if (description !== (editRequest.description || '')) updateBody.description = description
        if (priority !== editRequest.priority) updateBody.priority = priority
        if (assignedTo !== (editRequest.assignedTo || '')) updateBody.assignedTo = assignedTo
        if (resolution !== (editRequest.resolution || '')) updateBody.resolution = resolution
        if (cost !== (editRequest.cost !== null ? editRequest.cost.toString() : '')) {
          updateBody.cost = cost ? parseFloat(cost) : null
        }

        if (Object.keys(updateBody).length === 0) {
          // No changes
          onSuccess()
          handleClose()
          return
        }

        const res = await branchFetch(`/api/new/maintenance/requests/${editRequest.id}`, {
          method: 'PUT',
          headers: { 'Content-Type': 'application/json' },
          body: JSON.stringify(updateBody),
        })

        const data = await res.json()

        if (!res.ok || !data.success) {
          throw new Error(data.error || data.message || 'เกิดข้อผิดพลาดในการอัพเดท')
        }
      } else {
        // Create new request
        const res = await branchFetch('/api/new/maintenance/requests', {
          method: 'POST',
          headers: { 'Content-Type': 'application/json' },
          body: JSON.stringify({
            roomId: Number(roomId),
            categoryId: Number(categoryId),
            title: title.trim(),
            description: description || undefined,
            priority,
            assignedTo: assignedTo || undefined,
          }),
        })

        const data = await res.json()

        if (!res.ok || !data.success) {
          throw new Error(data.error || data.message || 'เกิดข้อผิดพลาดในการสร้างคำขอ')
        }
      }

      onSuccess()
      handleClose()
    } catch (err) {
      setError(err instanceof Error ? err.message : 'เกิดข้อผิดพลาด')
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
          <div className="flex items-center gap-2">
            <Wrench className="w-6 h-6 text-red-600" />
            <div>
              <h2 className="text-xl font-bold text-gray-900">
                {isEditing ? 'แก้ไขคำขอซ่อม' : 'แจ้งซ่อมใหม่'}
              </h2>
              {isEditing && editRequest && (
                <p className="text-sm text-gray-500">{editRequest.requestNo}</p>
              )}
            </div>
          </div>
          <button
            onClick={handleClose}
            className="p-2 hover:bg-gray-100 rounded-lg"
          >
            <X size={20} />
          </button>
        </div>

        {/* Form */}
        <form onSubmit={handleSubmit} className="p-4 space-y-4">
          {/* Error Message */}
          {error && (
            <div className="flex items-center gap-2 p-3 bg-red-50 border border-red-200 rounded-lg text-red-600 text-sm">
              <AlertCircle className="w-4 h-4 flex-shrink-0" />
              {error}
            </div>
          )}

          {/* Room Selection */}
          <div>
            <label className="block text-sm font-medium text-gray-700 mb-1">
              <DoorOpen size={16} className="inline mr-1" />
              ห้อง *
            </label>
            <select
              value={roomId}
              onChange={(e) => setRoomId(e.target.value ? parseInt(e.target.value) : '')}
              className="w-full px-3 py-2 bg-gray-100 border border-gray-300 text-gray-800 rounded-lg focus:ring-2 focus:ring-red-500 focus:border-red-500"
              disabled={isEditing}
              required
            >
              <option value="">เลือกห้อง</option>
              {rooms.map((room) => (
                <option key={room.id} value={room.id}>
                  {room.roomNo} {room.roomTypeName ? `- ${room.roomTypeName}` : ''}
                </option>
              ))}
            </select>
          </div>

          {/* Category Selection */}
          <div>
            <label className="block text-sm font-medium text-gray-700 mb-1">
              <Tag size={16} className="inline mr-1" />
              หมวดหมู่ *
            </label>
            <select
              value={categoryId}
              onChange={(e) => setCategoryId(e.target.value ? parseInt(e.target.value) : '')}
              className="w-full px-3 py-2 bg-gray-100 border border-gray-300 text-gray-800 rounded-lg focus:ring-2 focus:ring-red-500 focus:border-red-500"
              disabled={isEditing}
              required
            >
              <option value="">เลือกหมวดหมู่</option>
              {categories.map((cat) => (
                <option key={cat.id} value={cat.id}>
                  {cat.name} {cat.nameEn ? `(${cat.nameEn})` : ''}
                </option>
              ))}
            </select>
          </div>

          {/* Title */}
          <div>
            <label className="block text-sm font-medium text-gray-700 mb-1">
              <FileText size={16} className="inline mr-1" />
              หัวข้อ *
            </label>
            <input
              type="text"
              value={title}
              onChange={(e) => setTitle(e.target.value)}
              placeholder="เช่น แอร์ไม่เย็น, ก๊อกน้ำรั่ว..."
              className="w-full px-3 py-2 bg-gray-100 border border-gray-300 text-gray-800 rounded-lg focus:ring-2 focus:ring-red-500 focus:border-red-500"
              required
            />
          </div>

          {/* Description */}
          <div>
            <label className="block text-sm font-medium text-gray-700 mb-1">
              รายละเอียด
            </label>
            <textarea
              value={description}
              onChange={(e) => setDescription(e.target.value)}
              placeholder="รายละเอียดเพิ่มเติม..."
              rows={3}
              className="w-full px-3 py-2 bg-gray-100 border border-gray-300 text-gray-800 rounded-lg focus:ring-2 focus:ring-red-500 focus:border-red-500 resize-none"
            />
          </div>

          {/* Priority */}
          <div>
            <label className="block text-sm font-medium text-gray-700 mb-2">
              ความเร่งด่วน
            </label>
            <div className="grid grid-cols-3 gap-2">
              {priorityOptions.map((opt) => (
                <button
                  key={opt.value}
                  type="button"
                  onClick={() => setPriority(opt.value)}
                  className={`px-3 py-2 rounded-lg border text-sm font-medium ${
                    priority === opt.value
                      ? opt.value === 3
                        ? 'border-red-500 bg-red-500/10 text-red-600'
                        : opt.value === 2
                        ? 'border-amber-500 bg-amber-500/10 text-amber-400'
                        : 'border-gray-500 bg-gray-100 text-gray-700'
                      : 'border-gray-300 text-gray-500 hover:bg-gray-100'
                  }`}
                >
                  {opt.label}
                </button>
              ))}
            </div>
          </div>

          {/* Assigned To */}
          <div>
            <label className="block text-sm font-medium text-gray-700 mb-1">
              <User size={16} className="inline mr-1" />
              ผู้รับผิดชอบ
            </label>
            <input
              type="text"
              value={assignedTo}
              onChange={(e) => setAssignedTo(e.target.value)}
              placeholder="ชื่อช่างหรือผู้รับผิดชอบ"
              className="w-full px-3 py-2 bg-gray-100 border border-gray-300 text-gray-800 rounded-lg focus:ring-2 focus:ring-red-500 focus:border-red-500"
            />
          </div>

          {/* Edit-only fields */}
          {isEditing && (
            <>
              {/* Resolution */}
              <div>
                <label className="block text-sm font-medium text-gray-700 mb-1">
                  ผลการซ่อม
                </label>
                <textarea
                  value={resolution}
                  onChange={(e) => setResolution(e.target.value)}
                  placeholder="รายละเอียดผลการซ่อม..."
                  rows={2}
                  className="w-full px-3 py-2 bg-gray-100 border border-gray-300 text-gray-800 rounded-lg focus:ring-2 focus:ring-red-500 focus:border-red-500 resize-none"
                />
              </div>

              {/* Cost */}
              <div>
                <label className="block text-sm font-medium text-gray-700 mb-1">
                  <DollarSign size={16} className="inline mr-1" />
                  ค่าใช้จ่าย (บาท)
                </label>
                <input
                  type="number"
                  value={cost}
                  onChange={(e) => setCost(e.target.value)}
                  placeholder="0.00"
                  min="0"
                  step="0.01"
                  className="w-full px-3 py-2 bg-gray-100 border border-gray-300 text-gray-800 rounded-lg focus:ring-2 focus:ring-red-500 focus:border-red-500"
                />
              </div>
            </>
          )}

          {/* Submit Buttons */}
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
              disabled={isSubmitting}
              className="flex-1 px-4 py-2 bg-red-600 text-white rounded-lg hover:bg-red-700 disabled:opacity-50 disabled:cursor-not-allowed flex items-center justify-center gap-2"
            >
              {isSubmitting ? (
                <>
                  <Loader2 className="animate-spin" size={18} />
                  กำลังบันทึก...
                </>
              ) : isEditing ? (
                'บันทึกการแก้ไข'
              ) : (
                'แจ้งซ่อม'
              )}
            </button>
          </div>
        </form>
      </div>
    </div>
  )
}
