'use client'

import { Building2, Sparkles } from 'lucide-react'
import { useMode } from '@/contexts/ModeContext'

export default function ModeToggle() {
  const { setMode, isLegacy } = useMode()

  const toggleMode = () => {
    setMode(isLegacy ? 'new' : 'legacy')
  }

  return (
    <button
      onClick={toggleMode}
      className={`flex items-center gap-2 px-3 py-1.5 rounded-lg transition-all duration-200 text-sm font-medium border ${
        isLegacy
          ? 'bg-amber-500/20 text-amber-200 border-amber-500/40 hover:bg-amber-500/30'
          : 'bg-red-500/20 text-red-200 border-red-500/40 hover:bg-red-500/30'
      }`}
      title={isLegacy ? 'Currently using Legacy mode' : 'Currently using New mode'}
    >
      {isLegacy ? (
        <>
          <Building2 size={16} />
          <span className="hidden sm:inline">Legacy</span>
        </>
      ) : (
        <>
          <Sparkles size={16} />
          <span className="hidden sm:inline">New</span>
        </>
      )}
    </button>
  )
}
