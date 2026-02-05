'use client'

import { Sparkles, Clock, CheckCircle, Timer } from 'lucide-react'

export interface HousekeepingStatsData {
  dirtyCount: number
  cleaningCount: number
  cleanedTodayCount: number
  avgCleaningTimeMinutes: number | null
}

interface HousekeepingStatsProps {
  stats: HousekeepingStatsData
  loading?: boolean
}

export default function HousekeepingStats({ stats, loading }: HousekeepingStatsProps) {
  const statCards = [
    {
      title: 'รอทำความสะอาด',
      value: stats.dirtyCount,
      icon: Clock,
      bgColor: 'bg-red-50',
      iconColor: 'text-red-500',
      valueColor: 'text-red-600',
    },
    {
      title: 'กำลังทำความสะอาด',
      value: stats.cleaningCount,
      icon: Sparkles,
      bgColor: 'bg-yellow-50',
      iconColor: 'text-yellow-500',
      valueColor: 'text-yellow-600',
    },
    {
      title: 'ทำความสะอาดแล้ววันนี้',
      value: stats.cleanedTodayCount,
      icon: CheckCircle,
      bgColor: 'bg-green-50',
      iconColor: 'text-green-500',
      valueColor: 'text-green-600',
    },
    {
      title: 'เวลาเฉลี่ยต่อห้อง',
      value: stats.avgCleaningTimeMinutes !== null
        ? `${stats.avgCleaningTimeMinutes} นาที`
        : '-',
      icon: Timer,
      bgColor: 'bg-blue-50',
      iconColor: 'text-blue-500',
      valueColor: 'text-blue-600',
    },
  ]

  return (
    <div className="grid grid-cols-2 lg:grid-cols-4 gap-4">
      {statCards.map((card) => (
        <div
          key={card.title}
          className={`${card.bgColor} rounded-xl p-4 border border-gray-100`}
        >
          <div className="flex items-center gap-3">
            <div className={`p-2 rounded-lg bg-white shadow-sm`}>
              <card.icon className={`w-5 h-5 ${card.iconColor}`} />
            </div>
            <div>
              <p className="text-xs font-medium text-gray-500">{card.title}</p>
              {loading ? (
                <div className="h-7 w-12 bg-gray-200 animate-pulse rounded mt-1" />
              ) : (
                <p className={`text-xl font-bold ${card.valueColor}`}>
                  {card.value}
                </p>
              )}
            </div>
          </div>
        </div>
      ))}
    </div>
  )
}
