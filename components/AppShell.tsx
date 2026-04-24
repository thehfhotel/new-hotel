'use client'

import { useState, useEffect } from 'react'
import Sidebar, { SIDEBAR_WIDTH, SIDEBAR_COLLAPSED_WIDTH } from '@/components/Sidebar'
import { BranchProvider } from '@/contexts/BranchContext'

/**
 * Client shell that renders the sidebar + main content area for every page.
 * Wraps children in the BranchProvider so any descendant can read the active
 * branch via `useBranch()` / `useBranchFetch()`.
 *
 * Sidebar collapse state is mirrored from localStorage and kept in sync via
 * the `sidebar-toggle` window event dispatched by `<Sidebar>`.
 */
export default function AppShell({ children }: { children: React.ReactNode }) {
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
