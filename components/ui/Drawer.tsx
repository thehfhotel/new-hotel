'use client'

import { ReactNode, useEffect, useCallback } from 'react'
import { createPortal } from 'react-dom'
import { X } from 'lucide-react'

interface DrawerProps {
  isOpen: boolean
  onClose: () => void
  title: string
  children: ReactNode
  width?: string
}

export default function Drawer({ isOpen, onClose, title, children, width = '480px' }: DrawerProps) {
  const handleEscape = useCallback(
    (e: KeyboardEvent) => {
      if (e.key === 'Escape') onClose()
    },
    [onClose]
  )

  useEffect(() => {
    if (isOpen) {
      document.addEventListener('keydown', handleEscape)
      document.body.style.overflow = 'hidden'
    }
    return () => {
      document.removeEventListener('keydown', handleEscape)
      document.body.style.overflow = ''
    }
  }, [isOpen, handleEscape])

  if (!isOpen) return null

  return createPortal(
    <div className="fixed inset-0 z-50">
      <div className="fixed inset-0 bg-black/40" onClick={onClose} />
      <div
        className="fixed top-0 right-0 h-full bg-zinc-900 border-l border-zinc-800 shadow-xl overflow-y-auto"
        style={{ width }}
      >
        <div className="flex items-center justify-between p-4 border-b border-zinc-800 sticky top-0 bg-zinc-900 z-10">
          <h2 className="text-lg font-semibold text-zinc-100">{title}</h2>
          <button onClick={onClose} className="p-1 rounded-lg hover:bg-zinc-800 text-zinc-400 hover:text-zinc-200 transition-colors">
            <X className="w-5 h-5" />
          </button>
        </div>
        <div className="p-4">{children}</div>
      </div>
    </div>,
    document.body
  )
}
