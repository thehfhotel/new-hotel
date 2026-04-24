'use client'

import { ReactNode } from 'react'

interface CardProps {
  children: ReactNode
  className?: string
  padding?: 'none' | 'sm' | 'md' | 'lg'
}

const paddingStyles: Record<string, string> = {
  none: '',
  sm: 'p-2',
  md: 'p-3',
  lg: 'p-4',
}

// Flat panel — no rounded corners, single 1px border. Subtle visual weight only.
export default function Card({ children, className = '', padding = 'md' }: CardProps) {
  return (
    <div className={`bg-panel border border-border ${paddingStyles[padding]} ${className}`}>
      {children}
    </div>
  )
}

export function CardHeader({ children, className = '' }: { children: ReactNode; className?: string }) {
  return (
    <div
      className={`flex items-center justify-between bg-headerBar border-b border-border px-3 py-2 text-[12px] font-semibold uppercase tracking-wide text-text ${className}`}
    >
      {children}
    </div>
  )
}

export function CardContent({ children, className = '' }: { children: ReactNode; className?: string }) {
  return <div className={`p-3 ${className}`}>{children}</div>
}
