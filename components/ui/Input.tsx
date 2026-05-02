'use client'

import { InputHTMLAttributes, ReactNode, forwardRef } from 'react'

interface InputProps extends InputHTMLAttributes<HTMLInputElement> {
  label?: string
  error?: string
  icon?: ReactNode
}

const Input = forwardRef<HTMLInputElement, InputProps>(
  ({ label, error, icon, className = '', ...props }, ref) => {
    return (
      <div>
        {label && (
          <label className="block text-[12px] font-medium text-text mb-1">{label}</label>
        )}
        <div className="relative">
          {icon && (
            <div className="absolute left-2 top-1/2 -translate-y-1/2 text-textMuted">{icon}</div>
          )}
          <input
            ref={ref}
            className={`w-full bg-panel border rounded-[2px] h-7 px-2 text-[13px] text-text placeholder:text-textMuted transition-colors focus:outline-hidden focus:border-brand-500 ${
              icon ? 'pl-7' : ''
            } ${error ? 'border-error' : 'border-borderStrong'} ${className}`}
            {...props}
          />
        </div>
        {error && <p className="mt-1 text-[11px] text-error">{error}</p>}
      </div>
    )
  }
)
Input.displayName = 'Input'

export default Input
