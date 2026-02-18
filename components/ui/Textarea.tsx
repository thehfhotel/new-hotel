'use client'

import { TextareaHTMLAttributes, forwardRef } from 'react'

interface TextareaProps extends TextareaHTMLAttributes<HTMLTextAreaElement> {
  label?: string
  error?: string
}

const Textarea = forwardRef<HTMLTextAreaElement, TextareaProps>(
  ({ label, error, className = '', ...props }, ref) => {
    return (
      <div>
        {label && <label className="block text-sm font-medium text-zinc-400 mb-1.5">{label}</label>}
        <textarea
          ref={ref}
          className={`w-full bg-zinc-800 border rounded-lg px-3 py-2 text-sm text-zinc-200 placeholder:text-zinc-500 transition-colors focus:outline-none focus:ring-2 focus:ring-red-600/50 focus:border-red-600 ${
            error ? 'border-red-500' : 'border-zinc-700'
          } ${className}`}
          {...props}
        />
        {error && <p className="mt-1 text-xs text-red-400">{error}</p>}
      </div>
    )
  }
)
Textarea.displayName = 'Textarea'

export default Textarea
