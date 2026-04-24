'use client'

import { ButtonHTMLAttributes, ReactNode } from 'react'
import { Loader2 } from 'lucide-react'

interface ButtonProps extends ButtonHTMLAttributes<HTMLButtonElement> {
  variant?: 'primary' | 'secondary' | 'ghost' | 'danger'
  size?: 'sm' | 'md' | 'lg'
  loading?: boolean
  icon?: ReactNode
}

// SAP Fiori-flavoured buttons in oxidized blood-red.
// `border` is intentional even on primary — Fiori buttons read as flat plates with a 1px outline.
const variantStyles: Record<string, string> = {
  primary:   'bg-brand-500 text-white border border-brand-700 hover:bg-brand-600 active:bg-brand-700',
  secondary: 'bg-panel text-text border border-borderStrong hover:bg-headerBar',
  ghost:     'bg-transparent text-text border border-transparent hover:bg-headerBar',
  danger:    'bg-error text-white border border-error hover:opacity-90',
}

const sizeStyles: Record<string, string> = {
  sm: 'h-6 px-2 text-[12px]',
  md: 'h-7 px-3 text-[13px]',
  lg: 'h-8 px-4 text-[13px]',
}

export default function Button({
  children,
  variant = 'primary',
  size = 'md',
  loading = false,
  icon,
  className = '',
  disabled,
  ...props
}: ButtonProps) {
  return (
    <button
      className={`inline-flex items-center justify-center gap-1.5 rounded-[2px] transition-colors disabled:opacity-50 disabled:cursor-not-allowed ${variantStyles[variant]} ${sizeStyles[size]} ${className}`}
      disabled={disabled || loading}
      {...props}
    >
      {loading ? <Loader2 className="w-3.5 h-3.5 animate-spin" /> : icon}
      {children}
    </button>
  )
}
