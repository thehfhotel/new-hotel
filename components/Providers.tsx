'use client'

import { ReactNode } from 'react'
import { ModeProvider } from '@/contexts/ModeContext'
import { AuthProvider } from '@/contexts/AuthContext'

interface ProvidersProps {
  children: ReactNode
}

/**
 * Root client-side providers. Order matters: ModeProvider sits outermost
 * (it has no dependencies), AuthProvider next so any descendant — including
 * the AuthGuard it injects — can run useAuth(). BranchProvider is mounted
 * deeper inside AppShell because it only matters for in-app pages, not the
 * public /login route.
 */
export default function Providers({ children }: ProvidersProps) {
  return (
    <ModeProvider>
      <AuthProvider>
        {children}
      </AuthProvider>
    </ModeProvider>
  )
}
