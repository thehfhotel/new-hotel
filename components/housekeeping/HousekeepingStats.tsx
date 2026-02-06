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
      bgColor: 'bg-red-500/5',
      iconColor: 'text-red-400',
      valueColor: 'text-red-400',
    },
    {
      title: 'กำลังทำความสะอาด',
      value: stats.cleaningCount,
      icon: Sparkles,
      bgColor: 'bg-amber-500/5',
      iconColor: 'text-amber-400',
      valueColor: 'text-amber-400',
    },
    {
      title: 'ทำความสะอาดแล้ววันนี้',
      value: stats.cleanedTodayCount,
      icon: CheckCircle,
      bgColor: 'bg-emerald-500/5',
      iconColor: 'text-emerald-400',
      valueColor: 'text-emerald-400',
    },
    {
      title: 'เวลาเฉลี่ยต่อห้อง',
      value: stats.avgCleaningTimeMinutes !== null
        ? `${stats.avgCleaningTimeMinutes} นาที`
        : '-',
      icon: Timer,
      bgColor: 'bg-sky-500/5',
      iconColor: 'text-sky-400',
      valueColor: 'text-sky-400',
    },
  ]

  return (
    <div className="grid grid-cols-2 lg:grid-cols-4 gap-4">
      {statCards.map((card) => (
        <div
          key={card.title}
          className={`${card.bgColor} rounded-xl p-4 border border-zinc-800`}
        >
          <div className="flex items-center gap-3">
            <div className="p-2 rounded-lg bg-zinc-900 border border-zinc-800">
              <card.icon className={`w-5 h-5 ${card.iconColor}`} />
            </div>
            <div>
              <p className="text-xs font-medium text-zinc-500">{card.title}</p>
              {loading ? (
                <div className="h-7 w-12 bg-zinc-800 animate-pulse rounded mt-1" />
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
