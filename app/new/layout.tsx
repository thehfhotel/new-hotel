'use client'

import Sidebar, { SIDEBAR_WIDTH, SIDEBAR_COLLAPSED_WIDTH } from '@/components/Sidebar'
import { BranchProvider } from '@/contexts/BranchContext'
import { useState, useEffect } from 'react'

export default function NewLayout({
  children,
}: {
  children: React.ReactNode
}) {
  const [collapsed, setCollapsed] = useState(false)
  const [mounted, setMounted] = useState(false)

  useEffect(() => {
    const stored = localStorage.getItem('sidebar-collapsed')
    if (stored !== null) {
      setCollapsed(stored === 'true')
    } else {
      setCollapsed(window.innerWidth < 1280)
    }
    setMounted(true)

    const handler = () => {
      const val = localStorage.getItem('sidebar-collapsed')
      setCollapsed(val === 'true')
    }
    window.addEventListener('sidebar-toggle', handler)
    return () => window.removeEventListener('sidebar-toggle', handler)
  }, [])

  return (
    <BranchProvider>
      <div className="min-h-screen bg-gray-50">
        <Sidebar />
        <main
          className={`min-h-screen px-6 py-6 ${mounted ? 'transition-all duration-300' : ''}`}
          style={{ marginLeft: mounted ? (collapsed ? SIDEBAR_COLLAPSED_WIDTH : SIDEBAR_WIDTH) : SIDEBAR_WIDTH }}
        >
          {children}
        </main>
      </div>
    </BranchProvider>
  )
}
