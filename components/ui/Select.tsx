'use client'

import { SelectHTMLAttributes, forwardRef } from 'react'

interface SelectProps extends SelectHTMLAttributes<HTMLSelectElement> {
  label?: string
  error?: string
  options: Array<{ value: string; label: string }>
}

const Select = forwardRef<HTMLSelectElement, SelectProps>(
  ({ label, error, options, className = '', ...props }, ref) => {
    return (
      <div>
        {label && <label className="block text-sm font-medium text-zinc-400 mb-1.5">{label}</label>}
        <select
          ref={ref}
          className={`w-full bg-zinc-800 border rounded-lg px-3 py-2 text-sm text-zinc-200 transition-colors focus:outline-none focus:ring-2 focus:ring-red-600/50 focus:border-red-600 ${
            error ? 'border-red-500' : 'border-zinc-700'
          } ${className}`}
          {...props}
        >
          {options.map((opt) => (
            <option key={opt.value} value={opt.value}>
              {opt.label}
            </option>
          ))}
        </select>
        {error && <p className="mt-1 text-xs text-red-400">{error}</p>}
      </div>
    )
  }
)
Select.displayName = 'Select'

export default Select
