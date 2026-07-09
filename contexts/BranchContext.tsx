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
  /** Whether the backend permits HF Ville WRITES (HFVILLE_WRITES_ENABLED). */
  villeWritesEnabled: boolean
  /** Whether the backend lets our app open/close iHOTEL cashier rounds
   *  (ROUND_WRITEBACK_ENABLED). When false, round controls are read-only
   *  ("managed in iHOTEL") and POST /api/shifts/open|close return 400. */
  roundWritebackEnabled: boolean
  /** Spike Phase 2 (ship-dark): when true, the checkout UI uses the backend-
   *  computed folio total (CHECKOUT_SERVER_TOTAL_ENABLED) instead of computing
   *  nights × rate client-side. Default false → current behavior. */
  checkoutServerTotalEnabled: boolean
  /** Spike Phase 3 (ship-dark): when true, the booking form calls
   *  POST /api/bookings/validate and surfaces the backend date + availability
   *  verdict (BOOKING_VALIDATION_ENABLED). Default false → current client-only
   *  validation. */
  bookingValidationEnabled: boolean
  /** #236 จัดผัง (ship-dark): when true the rooms page shows the layout-edit
   *  toggle and PUT /api/rooms/layout accepts drops (shared board — each drop
   *  mirrors to iHOTEL HT_Rooms.Room_X/Room_y). When false the WHOLE mode is
   *  hidden (LAYOUT_WRITEBACK_ENABLED). Default false. */
  layoutWritebackEnabled: boolean
  /** True when the current branch is writable: HF Hotel always, HF Ville only
   *  when villeWritesEnabled. UIs gate mutating actions on this. */
  canWrite: boolean
}

const BranchContext = createContext<BranchContextType>({
  branch: 'hfhotel',
  setBranch: () => {},
  villeAvailable: false,
  villeWritesEnabled: false,
  roundWritebackEnabled: false,
  checkoutServerTotalEnabled: false,
  bookingValidationEnabled: false,
  layoutWritebackEnabled: false,
  canWrite: true,
})

export function BranchProvider({ children }: { children: ReactNode }) {
  const [branch, setBranchState] = useState<Branch>('hfhotel')
  const [villeAvailable, setVilleAvailable] = useState(false)
  const [villeWritesEnabled, setVilleWritesEnabled] = useState(false)
  const [roundWritebackEnabled, setRoundWritebackEnabled] = useState(false)
  const [checkoutServerTotalEnabled, setCheckoutServerTotalEnabled] = useState(false)
  const [bookingValidationEnabled, setBookingValidationEnabled] = useState(false)
  const [layoutWritebackEnabled, setLayoutWritebackEnabled] = useState(false)
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
        if (data.success && data.villeWritesEnabled) {
          setVilleWritesEnabled(true)
        }
        if (data.success && data.roundWritebackEnabled) {
          setRoundWritebackEnabled(true)
        }
        if (data.success && data.checkoutServerTotalEnabled) {
          setCheckoutServerTotalEnabled(true)
        }
        if (data.success && data.bookingValidationEnabled) {
          setBookingValidationEnabled(true)
        }
        if (data.success && data.layoutWritebackEnabled) {
          setLayoutWritebackEnabled(true)
        }
      })
      .catch((err) => console.error('Failed to check ville availability:', err))
  }, [])

  const setBranch = (b: Branch) => {
    setBranchState(b)
    localStorage.setItem('selected-branch', b)
    // Dispatch event so other components can react
    window.dispatchEvent(new CustomEvent('branch-change', { detail: b }))
  }

  // HF Hotel is always writable; HF Ville only when the backend enables it.
  const canWrite = branch === 'hfhotel' || (branch === 'hfville' && villeWritesEnabled)

  if (!mounted) {
    return (
      <BranchContext.Provider value={{ branch, setBranch, villeAvailable, villeWritesEnabled, roundWritebackEnabled, checkoutServerTotalEnabled, bookingValidationEnabled, layoutWritebackEnabled, canWrite }}>
        {children}
      </BranchContext.Provider>
    )
  }

  return (
    <BranchContext.Provider value={{ branch, setBranch, villeAvailable, villeWritesEnabled, roundWritebackEnabled, checkoutServerTotalEnabled, bookingValidationEnabled, layoutWritebackEnabled, canWrite }}>
      {children}
    </BranchContext.Provider>
  )
}

export function useBranch() {
  return useContext(BranchContext)
}
