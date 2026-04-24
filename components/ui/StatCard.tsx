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

// Fiori-style KPI tile — flat panel, small uppercase label, prominent but not flashy value.
// `iconColor` is preserved as an opt-in accent (e.g. for status dots / icons).
export default function StatCard({
  title,
  value,
  icon,
  iconColor = 'text-textMuted',
  bgColor = '',
  loading = false,
}: StatCardProps) {
  return (
    <div className={`border border-border p-3 ${bgColor || 'bg-panel'}`}>
      <div className="flex items-center gap-3">
        {icon && (
          <div className="p-1.5 bg-headerBar border border-border">
            <div className={`w-4 h-4 ${iconColor}`}>{icon}</div>
          </div>
        )}
        <div>
          <p className="text-[11px] uppercase tracking-wide text-textMuted">{title}</p>
          {loading ? (
            <div className="h-5 w-16 bg-headerBar animate-pulse mt-1" />
          ) : (
            <p className="text-[20px] font-semibold text-text leading-tight">{value}</p>
          )}
        </div>
      </div>
    </div>
  )
}
