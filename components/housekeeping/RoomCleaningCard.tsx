'use client'

import { useState } from 'react'
import {
  Clock,
  AlertTriangle,
  Play,
  CheckCircle,
  User,
  MessageSquare,
  ChevronDown,
  ChevronUp,
} from 'lucide-react'

export type HousekeepingStatus = 'dirty' | 'cleaning' | 'available'

export interface HousekeepingRoom {
  id: number
  roomNo: string
  roomTypeName: string | null
  floor: number | null
  status: HousekeepingStatus
  lastCheckoutTime: string | null
  statusChangedAt: string | null
  housekeeperName: string | null
  notes: string | null
}

interface RoomCleaningCardProps {
  room: HousekeepingRoom
  onStatusChange: (roomId: number, newStatus: HousekeepingStatus, notes?: string) => Promise<void>
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
  if (mins === 0) {
    return `${hours} ชม.`
  }
  return `${hours} ชม. ${mins} น.`
}

// Format datetime for display (Thai local time)
function formatDateTime(dateString: string | null): string {
  if (!dateString) return '-'
  const date = new Date(dateString)
  return date.toLocaleTimeString('th-TH', {
    hour: '2-digit',
    minute: '2-digit',
    timeZone: 'UTC', // Database stores Thai time as UTC
  })
}

export default function RoomCleaningCard({
  room,
  onStatusChange,
  disabled = false,
}: RoomCleaningCardProps) {
  const [showNotes, setShowNotes] = useState(false)
  const [notes, setNotes] = useState(room.notes || '')
  const [isUpdating, setIsUpdating] = useState(false)

  // Calculate priority based on time since checkout
  const minutesSinceCheckout = room.lastCheckoutTime
    ? getMinutesElapsed(room.lastCheckoutTime)
    : 0
  const isHighPriority = room.status === 'dirty' && minutesSinceCheckout > 120 // > 2 hours

  // Calculate time in current status
  const minutesInStatus = getMinutesElapsed(room.statusChangedAt)

  const handleStatusChange = async (newStatus: HousekeepingStatus) => {
    if (disabled || isUpdating) return
    setIsUpdating(true)
    try {
      await onStatusChange(room.id, newStatus, notes || undefined)
    } finally {
      setIsUpdating(false)
    }
  }

  // Status-specific colors for dark theme
  const statusColors = {
    dirty: {
      bg: 'bg-white',
      border: 'border-red-900/30',
      badge: 'bg-red-500/10 text-red-600',
    },
    cleaning: {
      bg: 'bg-white',
      border: 'border-amber-900/30',
      badge: 'bg-amber-500/10 text-amber-400',
    },
    available: {
      bg: 'bg-white',
      border: 'border-emerald-900/30',
      badge: 'bg-emerald-500/10 text-emerald-400',
    },
  }

  const colors = statusColors[room.status]

  return (
    <div
      className={`${colors.bg} ${colors.border} border rounded-xl p-4 transition-all hover:border-gray-300`}
    >
      {/* Header */}
      <div className="flex items-start justify-between mb-3">
        <div>
          <h3 className="text-2xl font-bold text-gray-900">{room.roomNo}</h3>
          <p className="text-sm text-gray-500">
            {room.roomTypeName || 'ไม่ระบุประเภท'}
          </p>
          {room.floor && (
            <p className="text-xs text-gray-500">ชั้น {room.floor}</p>
          )}
        </div>

        {/* Priority Badge */}
        {isHighPriority && (
          <div className="flex items-center gap-1 px-2 py-1 bg-red-600 text-white rounded-full text-xs font-medium">
            <AlertTriangle className="w-3 h-3" />
            <span>ด่วน</span>
          </div>
        )}
      </div>

      {/* Time Info */}
      <div className="space-y-1 mb-3">
        {room.lastCheckoutTime && room.status === 'dirty' && (
          <div className="flex items-center gap-2 text-sm text-gray-500">
            <Clock className="w-4 h-4" />
            <span>
              เช็คเอาท์: {formatDateTime(room.lastCheckoutTime)}
              {minutesSinceCheckout > 0 && (
                <span className="text-gray-500 ml-1">
                  ({formatTimeElapsed(minutesSinceCheckout)}ที่แล้ว)
                </span>
              )}
            </span>
          </div>
        )}

        {room.status === 'cleaning' && minutesInStatus > 0 && (
          <div className="flex items-center gap-2 text-sm text-amber-400">
            <Clock className="w-4 h-4" />
            <span>กำลังทำ: {formatTimeElapsed(minutesInStatus)}</span>
          </div>
        )}

        {room.housekeeperName && (
          <div className="flex items-center gap-2 text-sm text-gray-500">
            <User className="w-4 h-4" />
            <span>{room.housekeeperName}</span>
          </div>
        )}
      </div>

      {/* Notes Section */}
      <div className="mb-3">
        <button
          onClick={() => setShowNotes(!showNotes)}
          className="flex items-center gap-1 text-sm text-gray-500 hover:text-gray-700"
        >
          <MessageSquare className="w-4 h-4" />
          <span>บันทึก</span>
          {showNotes ? (
            <ChevronUp className="w-4 h-4" />
          ) : (
            <ChevronDown className="w-4 h-4" />
          )}
        </button>

        {showNotes && (
          <textarea
            value={notes}
            onChange={(e) => setNotes(e.target.value)}
            placeholder="เพิ่มบันทึก..."
            className="mt-2 w-full px-3 py-2 text-sm bg-gray-100 border border-gray-300 text-gray-800 rounded-lg focus:ring-2 focus:ring-red-500 focus:border-red-500 resize-none placeholder-gray-400"
            rows={2}
            disabled={disabled || isUpdating}
          />
        )}
      </div>

      {/* Action Buttons */}
      <div className="flex gap-2">
        {room.status === 'dirty' && (
          <button
            onClick={() => handleStatusChange('cleaning')}
            disabled={disabled || isUpdating}
            className="flex-1 flex items-center justify-center gap-2 px-3 py-2 bg-amber-600 hover:bg-amber-700 text-white rounded-lg text-sm font-medium disabled:opacity-50 disabled:cursor-not-allowed transition-colors"
          >
            <Play className="w-4 h-4" />
            <span>เริ่มทำความสะอาด</span>
          </button>
        )}

        {room.status === 'cleaning' && (
          <button
            onClick={() => handleStatusChange('available')}
            disabled={disabled || isUpdating}
            className="flex-1 flex items-center justify-center gap-2 px-3 py-2 bg-emerald-600 hover:bg-emerald-700 text-white rounded-lg text-sm font-medium disabled:opacity-50 disabled:cursor-not-allowed transition-colors"
          >
            <CheckCircle className="w-4 h-4" />
            <span>เสร็จแล้ว</span>
          </button>
        )}

        {room.status === 'available' && (
          <button
            onClick={() => handleStatusChange('dirty')}
            disabled={disabled || isUpdating}
            className="flex-1 flex items-center justify-center gap-2 px-3 py-2 bg-gray-100 hover:bg-gray-200 text-gray-700 rounded-lg text-sm font-medium disabled:opacity-50 disabled:cursor-not-allowed transition-colors"
          >
            <span>ทำเครื่องหมายสกปรก</span>
          </button>
        )}
      </div>

      {/* Loading Overlay */}
      {isUpdating && (
        <div className="absolute inset-0 bg-gray-50 flex items-center justify-center rounded-xl">
          <div className="w-6 h-6 border-2 border-red-500 border-t-transparent rounded-full animate-spin" />
        </div>
      )}
    </div>
  )
}
