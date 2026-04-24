'use client'

import { ReactNode } from 'react'

interface BadgeProps {
  children: ReactNode
  variant?: 'success' | 'warning' | 'error' | 'info' | 'neutral'
  className?: string
  size?: 'sm' | 'md'
}

// Flat rectangles, never pills. Each variant is a faint tinted background +
// matching 1px border + saturated text for legibility.
const variantStyles: Record<string, string> = {
  success: 'bg-success/10 text-success border border-success/40',
  warning: 'bg-warning/10 text-warning border border-warning/40',
  error:   'bg-error/10 text-error border border-error/40',
  info:    'bg-info/10 text-info border border-info/40',
  neutral: 'bg-headerBar text-text border border-border',
}

const sizeStyles: Record<string, string> = {
  sm: 'text-[11px] px-1.5 py-0',
  md: 'text-[12px] px-2 py-0.5',
}

export default function Badge({ children, variant = 'neutral', className, size = 'sm' }: BadgeProps) {
  return (
    <span
      className={`inline-flex items-center font-medium rounded-[2px] leading-tight ${sizeStyles[size]} ${className || variantStyles[variant]}`}
    >
      {children}
    </span>
  )
}
