'use client'

import { ReactNode } from 'react'
import { ModeProvider } from '@/contexts/ModeContext'
import { AuthProvider } from '@/contexts/AuthContext'
import { SkinProvider } from '@/contexts/SkinContext'

interface ProvidersProps {
  children: ReactNode
}

/**
 * Root client-side providers. Order matters: SkinProvider sits outermost
 * (purely visual, no dependencies), ModeProvider next, AuthProvider so any
 * descendant — including the AuthGuard it injects — can run useAuth().
 * BranchProvider is mounted deeper inside AppShell because it only matters
 * for in-app pages, not the public /login route.
 */
export default function Providers({ children }: ProvidersProps) {
  return (
    <SkinProvider>
      <ModeProvider>
        <AuthProvider>
          {children}
        </AuthProvider>
      </ModeProvider>
    </SkinProvider>
  )
}
