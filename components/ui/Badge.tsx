'use client'

import { ReactNode } from 'react'

interface BadgeProps {
  children: ReactNode
  variant?: 'success' | 'warning' | 'error' | 'info' | 'neutral'
  className?: string
  size?: 'sm' | 'md'
}

const variantStyles: Record<string, string> = {
  success: 'bg-emerald-500/10 text-emerald-400',
  warning: 'bg-amber-500/10 text-amber-400',
  error: 'bg-red-500/10 text-red-400',
  info: 'bg-sky-500/10 text-sky-400',
  neutral: 'bg-zinc-700 text-zinc-300',
}

const sizeStyles: Record<string, string> = {
  sm: 'text-xs px-2 py-0.5',
  md: 'text-sm px-2.5 py-1',
}

export default function Badge({ children, variant = 'neutral', className, size = 'sm' }: BadgeProps) {
  return (
    <span className={`inline-flex items-center font-medium rounded-full ${sizeStyles[size]} ${className || variantStyles[variant]}`}>
      {children}
    </span>
  )
}
