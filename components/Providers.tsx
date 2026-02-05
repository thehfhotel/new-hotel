'use client'

import { ReactNode } from 'react'
import { ModeProvider } from '@/contexts/ModeContext'

interface ProvidersProps {
  children: ReactNode
}

export default function Providers({ children }: ProvidersProps) {
  return (
    <ModeProvider>
      {children}
    </ModeProvider>
  )
}
