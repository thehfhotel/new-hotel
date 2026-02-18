'use client'

import { createContext, useContext, useState, useEffect, type ReactNode } from 'react'

export type Branch = 'hfhotel' | 'hfville' | 'all'

export const BRANCH_LABELS: Record<Branch, string> = {
  hfhotel: 'HF Hotel',
  hfville: 'HF Ville',
  all: 'ทั้งหมด',
}

interface BranchContextType {
  branch: Branch
  setBranch: (branch: Branch) => void
  villeAvailable: boolean
}

const BranchContext = createContext<BranchContextType>({
  branch: 'hfhotel',
  setBranch: () => {},
  villeAvailable: false,
})

export function BranchProvider({ children }: { children: ReactNode }) {
  const [branch, setBranchState] = useState<Branch>('hfhotel')
  const [villeAvailable, setVilleAvailable] = useState(false)
  const [mounted, setMounted] = useState(false)

  useEffect(() => {
    const stored = localStorage.getItem('selected-branch') as Branch | null
    if (stored && ['hfhotel', 'hfville', 'all'].includes(stored)) {
      setBranchState(stored)
    }
    setMounted(true)

    // Check if HF Ville is available
    fetch('/api/mode')
      .then(res => res.json())
      .then(data => {
        if (data.success && data.villeAvailable) {
          setVilleAvailable(true)
        }
      })
      .catch(() => {})
  }, [])

  const setBranch = (b: Branch) => {
    setBranchState(b)
    localStorage.setItem('selected-branch', b)
    // Dispatch event so other components can react
    window.dispatchEvent(new CustomEvent('branch-change', { detail: b }))
  }

  if (!mounted) {
    return <>{children}</>
  }

  return (
    <BranchContext.Provider value={{ branch, setBranch, villeAvailable }}>
      {children}
    </BranchContext.Provider>
  )
}

export function useBranch() {
  return useContext(BranchContext)
}
