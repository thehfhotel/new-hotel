'use client'

import { ReactNode } from 'react'

interface StatCardProps {
  title: string
  value: string | number
  icon?: ReactNode
  iconColor?: string
  bgColor?: string
  loading?: boolean
}

export default function StatCard({ title, value, icon, iconColor = 'text-zinc-400', bgColor = '', loading = false }: StatCardProps) {
  return (
    <div className={`rounded-xl border border-zinc-800 p-4 ${bgColor || 'bg-zinc-900'}`}>
      <div className="flex items-center gap-3">
        {icon && (
          <div className="p-2 rounded-lg bg-zinc-800 border border-zinc-700">
            <div className={`w-5 h-5 ${iconColor}`}>{icon}</div>
          </div>
        )}
        <div>
          <p className="text-xs font-medium text-zinc-500">{title}</p>
          {loading ? (
            <div className="h-7 w-16 bg-zinc-800 animate-pulse rounded mt-1" />
          ) : (
            <p className="text-2xl font-bold text-zinc-100">{value}</p>
          )}
        </div>
      </div>
    </div>
  )
}
