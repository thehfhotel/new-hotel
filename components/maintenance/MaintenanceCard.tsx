'use client'

import { useState } from 'react'
import {
  Clock,
  AlertTriangle,
  Play,
  CheckCircle,
  User,
  Tag,
  DoorOpen,
  Edit2,
} from 'lucide-react'
import { MaintenanceRequest, MaintenanceStatus, MaintenancePriority } from '@/types/reports'

interface MaintenanceCardProps {
  request: MaintenanceRequest
  onStatusChange: (requestId: number, newStatus: MaintenanceStatus) => Promise<void>
  onEdit: (request: MaintenanceRequest) => void
  disabled?: boolean
}

// Calculate time elapsed in minutes
function getMinutesElapsed(dateString: string | null): number {
  if (!dateString) return 0
  const date = new Date(dateString)
  const now = new Date()
  return Math.floor((now.getTime() - date.getTime()) / (1000 * 60))
}

// Format time elapsed for display
function formatTimeElapsed(minutes: number): string {
  if (minutes < 60) {
    return `${minutes} นาที`
  }
  const hours = Math.floor(minutes / 60)
  const mins = minutes % 60
  if (hours < 24) {
    if (mins === 0) {
      return `${hours} ชม.`
    }
    return `${hours} ชม. ${mins} น.`
  }
  const days = Math.floor(hours / 24)
  const remainingHours = hours % 24
  if (remainingHours === 0) {
    return `${days} วัน`
  }
  return `${days} วัน ${remainingHours} ชม.`
}

// Priority configuration
const priorityConfig: Record<MaintenancePriority, { label: string; color: string; bgColor: string }> = {
  3: { label: 'ด่วน', color: 'text-red-700', bgColor: 'bg-red-100' },
  2: { label: 'ปานกลาง', color: 'text-yellow-700', bgColor: 'bg-yellow-100' },
  1: { label: 'ต่ำ', color: 'text-gray-700', bgColor: 'bg-gray-100' },
}

export default function MaintenanceCard({
  request,
  onStatusChange,
  onEdit,
  disabled = false,
}: MaintenanceCardProps) {
  const [isUpdating, setIsUpdating] = useState(false)

  // Calculate time since created
  const minutesSinceCreated = getMinutesElapsed(request.createdAt)
  const isOverdue = request.status === 'open' && minutesSinceCreated > 120 // > 2 hours

  // Calculate time in progress
  const minutesInProgress = request.status === 'in_progress'
    ? getMinutesElapsed(request.startedAt)
    : 0

  const handleStatusChange = async (newStatus: MaintenanceStatus) => {
    if (disabled || isUpdating) return
    setIsUpdating(true)
    try {
      await onStatusChange(request.id, newStatus)
    } finally {
      setIsUpdating(false)
    }
  }

  const priority = request.priority as MaintenancePriority
  const priorityInfo = priorityConfig[priority] || priorityConfig[2]

  // Status-specific colors
  const statusColors = {
    open: {
      bg: 'bg-white',
      border: 'border-red-200',
    },
    in_progress: {
      bg: 'bg-white',
      border: 'border-yellow-200',
    },
    completed: {
      bg: 'bg-white',
      border: 'border-green-200',
    },
    cancelled: {
      bg: 'bg-gray-50',
      border: 'border-gray-200',
    },
  }

  const colors = statusColors[request.status] || statusColors.open

  return (
    <div
      className={`${colors.bg} ${colors.border} border rounded-xl p-4 shadow-sm hover:shadow-md relative`}
    >
      {/* Header with Title and Priority */}
      <div className="flex items-start justify-between mb-2">
        <div className="flex-1">
          <div className="flex items-center gap-2 mb-1">
            <span className="text-xs font-mono text-gray-400">
              {request.requestNo}
            </span>
            {isOverdue && (
              <div className="flex items-center gap-1 px-2 py-0.5 bg-red-500 text-white rounded-full text-xs font-medium">
                <AlertTriangle className="w-3 h-3" />
                <span>ล่าช้า</span>
              </div>
            )}
          </div>
          <h3 className="font-semibold text-gray-900 line-clamp-2">
            {request.title}
          </h3>
        </div>

        {/* Edit Button */}
        <button
          onClick={() => onEdit(request)}
          className="p-1.5 text-gray-400 hover:text-gray-600 hover:bg-gray-100 rounded-lg"
          disabled={disabled || isUpdating}
        >
          <Edit2 className="w-4 h-4" />
        </button>
      </div>

      {/* Room and Category Info */}
      <div className="flex flex-wrap gap-2 mb-3">
        <div className="flex items-center gap-1 text-sm text-gray-600">
          <DoorOpen className="w-4 h-4 text-blue-500" />
          <span className="font-medium">{request.roomNo}</span>
        </div>
        <div className="flex items-center gap-1 text-sm text-gray-600">
          <Tag className="w-4 h-4 text-purple-500" />
          <span>{request.categoryName}</span>
        </div>
      </div>

      {/* Priority Badge */}
      <div className="flex items-center gap-2 mb-3">
        <span
          className={`px-2 py-0.5 rounded-full text-xs font-medium ${priorityInfo.bgColor} ${priorityInfo.color}`}
        >
          {priorityInfo.label}
        </span>
      </div>

      {/* Time Info */}
      <div className="space-y-1 mb-3">
        {request.status === 'open' && (
          <div className="flex items-center gap-2 text-sm text-gray-500">
            <Clock className="w-4 h-4" />
            <span>
              รอดำเนินการ {minutesSinceCreated > 0 ? formatTimeElapsed(minutesSinceCreated) : 'เพิ่งแจ้ง'}
            </span>
          </div>
        )}

        {request.status === 'in_progress' && minutesInProgress > 0 && (
          <div className="flex items-center gap-2 text-sm text-yellow-700">
            <Clock className="w-4 h-4" />
            <span>กำลังซ่อม: {formatTimeElapsed(minutesInProgress)}</span>
          </div>
        )}

        {request.assignedTo && (
          <div className="flex items-center gap-2 text-sm text-gray-600">
            <User className="w-4 h-4" />
            <span>{request.assignedTo}</span>
          </div>
        )}
      </div>

      {/* Description Preview */}
      {request.description && (
        <div className="text-sm text-gray-500 mb-3 line-clamp-2">
          {request.description}
        </div>
      )}

      {/* Action Buttons */}
      <div className="flex gap-2">
        {request.status === 'open' && (
          <button
            onClick={() => handleStatusChange('in_progress')}
            disabled={disabled || isUpdating}
            className="flex-1 flex items-center justify-center gap-2 px-3 py-2 bg-yellow-500 hover:bg-yellow-600 text-white rounded-lg text-sm font-medium disabled:opacity-50 disabled:cursor-not-allowed"
          >
            <Play className="w-4 h-4" />
            <span>เริ่มซ่อม</span>
          </button>
        )}

        {request.status === 'in_progress' && (
          <button
            onClick={() => handleStatusChange('completed')}
            disabled={disabled || isUpdating}
            className="flex-1 flex items-center justify-center gap-2 px-3 py-2 bg-green-500 hover:bg-green-600 text-white rounded-lg text-sm font-medium disabled:opacity-50 disabled:cursor-not-allowed"
          >
            <CheckCircle className="w-4 h-4" />
            <span>ซ่อมเสร็จ</span>
          </button>
        )}

        {request.status === 'completed' && (
          <div className="flex-1 flex items-center justify-center gap-2 px-3 py-2 bg-gray-100 text-gray-600 rounded-lg text-sm">
            <CheckCircle className="w-4 h-4 text-green-500" />
            <span>ซ่อมเรียบร้อยแล้ว</span>
          </div>
        )}
      </div>

      {/* Cost display for completed requests */}
      {request.status === 'completed' && request.cost !== null && request.cost > 0 && (
        <div className="mt-2 text-sm text-gray-500">
          ค่าใช้จ่าย: {request.cost.toLocaleString('th-TH')} บาท
        </div>
      )}

      {/* Resolution for completed requests */}
      {request.status === 'completed' && request.resolution && (
        <div className="mt-2 p-2 bg-green-50 rounded-lg text-sm text-green-800">
          <span className="font-medium">ผลการซ่อม:</span> {request.resolution}
        </div>
      )}

      {/* Loading Overlay */}
      {isUpdating && (
        <div className="absolute inset-0 bg-white bg-opacity-50 flex items-center justify-center rounded-xl">
          <div className="w-6 h-6 border-2 border-orange-500 border-t-transparent rounded-full animate-spin" />
        </div>
      )}
    </div>
  )
}
