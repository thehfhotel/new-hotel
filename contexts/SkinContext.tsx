'use client'

import React, { createContext, useContext, useState, useEffect, ReactNode } from 'react'

export type Skin = 'fiori' | 'modern'

interface SkinContextType {
  skin: Skin
  setSkin: (skin: Skin) => void
}

const SkinContext = createContext<SkinContextType | undefined>(undefined)

export const SKIN_STORAGE_KEY = 'hotel-skin'
const DEFAULT_SKIN: Skin = 'fiori'

function isSkin(value: unknown): value is Skin {
  return value === 'fiori' || value === 'modern'
}

export function SkinProvider({ children }: { children: ReactNode }) {
  const [skin, setSkinState] = useState<Skin>(DEFAULT_SKIN)

  // Sync state from the value the FOUC-prevention inline script already wrote
  // onto <html data-skin>. Reading from the DOM (instead of localStorage)
  // guarantees state matches what the user actually sees.
  useEffect(() => {
    const attr = document.documentElement.getAttribute('data-skin')
    if (isSkin(attr) && attr !== skin) {
      setSkinState(attr)
    }
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [])

  const setSkin = (next: Skin) => {
    setSkinState(next)
    document.documentElement.setAttribute('data-skin', next)
    try {
      localStorage.setItem(SKIN_STORAGE_KEY, next)
    } catch {
      // Storage may be disabled (private mode); skin still applies for the session.
    }
  }

  return (
    <SkinContext.Provider value={{ skin, setSkin }}>
      {children}
    </SkinContext.Provider>
  )
}

export function useSkin(): SkinContextType {
  const ctx = useContext(SkinContext)
  if (!ctx) throw new Error('useSkin must be used within a SkinProvider')
  return ctx
}
