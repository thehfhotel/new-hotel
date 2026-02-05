'use client'

import React, { createContext, useContext, useState, useEffect, ReactNode } from 'react'

export type SystemMode = 'legacy' | 'new'

interface ModeContextType {
  mode: SystemMode
  setMode: (mode: SystemMode) => void
  isLegacy: boolean
  isNew: boolean
}

const ModeContext = createContext<ModeContextType | undefined>(undefined)

const STORAGE_KEY = 'hotel-system-mode'

interface ModeProviderProps {
  children: ReactNode
}

export function ModeProvider({ children }: ModeProviderProps) {
  const [mode, setModeState] = useState<SystemMode>('legacy')
  const [isInitialized, setIsInitialized] = useState(false)

  // Read mode from localStorage on mount
  useEffect(() => {
    const stored = localStorage.getItem(STORAGE_KEY)
    if (stored === 'legacy' || stored === 'new') {
      setModeState(stored)
    }
    setIsInitialized(true)
  }, [])

  // Write mode to localStorage when it changes
  const setMode = (newMode: SystemMode) => {
    setModeState(newMode)
    localStorage.setItem(STORAGE_KEY, newMode)
  }

  const value: ModeContextType = {
    mode,
    setMode,
    isLegacy: mode === 'legacy',
    isNew: mode === 'new',
  }

  // Prevent hydration mismatch by not rendering until initialized
  if (!isInitialized) {
    return null
  }

  return (
    <ModeContext.Provider value={value}>
      {children}
    </ModeContext.Provider>
  )
}

export function useMode(): ModeContextType {
  const context = useContext(ModeContext)
  if (context === undefined) {
    throw new Error('useMode must be used within a ModeProvider')
  }
  return context
}
