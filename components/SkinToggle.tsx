'use client'

import { Palette } from 'lucide-react'
import { useSkin } from '@/contexts/SkinContext'

interface SkinToggleProps {
  collapsed?: boolean
}

/**
 * Sidebar control that swaps between the dense "fiori" skin (default) and
 * the airier "modern" skin. Token values live in app/globals.css under
 * [data-skin='…'] blocks; the toggle just flips the attribute.
 */
export default function SkinToggle({ collapsed = false }: SkinToggleProps) {
  const { skin, setSkin } = useSkin()
  const next = skin === 'fiori' ? 'modern' : 'fiori'
  const label = skin === 'fiori' ? 'Fiori' : 'Modern'

  return (
    <button
      onClick={() => setSkin(next)}
      title={`Switch to ${next} skin`}
      className={`flex items-center gap-2 px-3 py-1.5 text-[13px] text-textMuted hover:bg-headerBar hover:text-text transition-colors w-full ${
        collapsed ? 'justify-center' : ''
      }`}
    >
      <Palette size={16} />
      {!collapsed && (
        <span>
          Skin: <span className="font-medium text-text">{label}</span>
        </span>
      )}
    </button>
  )
}
