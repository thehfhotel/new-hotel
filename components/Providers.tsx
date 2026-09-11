'use client'

import { ReactNode } from 'react'
import { ModeProvider } from '@/contexts/ModeContext'
import { AuthProvider } from '@/contexts/AuthContext'
import { SkinProvider } from '@/contexts/SkinContext'
import LegacyStaleBridge from '@/components/LegacyStaleBridge'

interface ProvidersProps {
  children: ReactNode
}

/**
 * Root client-side providers. Order matters: SkinProvider sits outermost
 * (purely visual, no dependencies), ModeProvider next, AuthProvider so any
 * descendant — including the AuthGuard it injects — can run useAuth().
 * BranchProvider is mounted deeper inside AppShell because it only matters
 * for in-app pages, not the public /login route.
 *
 * `LegacyStaleBridge` mounts here (rather than inside AppShell) so it stays
 * live on every route, including the chromeless ones (`/v2`, `/hk`, `/login`)
 * that skip AppShell's own BranchProvider — see its doc comment for why.
 */
export default function Providers({ children }: ProvidersProps) {
  return (
    <SkinProvider>
      <ModeProvider>
        <AuthProvider>
          <LegacyStaleBridge />
          {children}
        </AuthProvider>
      </ModeProvider>
    </SkinProvider>
  )
}
