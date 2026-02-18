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

export default function StatCard({ title, value, icon, iconColor = 'text-gray-500', bgColor = '', loading = false }: StatCardProps) {
  return (
    <div className={`rounded-xl border border-gray-200 p-4 ${bgColor || 'bg-white'}`}>
      <div className="flex items-center gap-3">
        {icon && (
          <div className="p-2 rounded-lg bg-gray-100 border border-gray-300">
            <div className={`w-5 h-5 ${iconColor}`}>{icon}</div>
          </div>
        )}
        <div>
          <p className="text-xs font-medium text-gray-500">{title}</p>
          {loading ? (
            <div className="h-7 w-16 bg-gray-200 animate-pulse rounded mt-1" />
          ) : (
            <p className="text-2xl font-bold text-gray-900">{value}</p>
          )}
        </div>
      </div>
    </div>
  )
}
