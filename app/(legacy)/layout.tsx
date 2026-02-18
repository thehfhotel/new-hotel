'use client'

import Navbar from '@/components/Navbar'
import { BranchProvider } from '@/contexts/BranchContext'

export default function LegacyLayout({
  children,
}: {
  children: React.ReactNode
}) {
  return (
    <BranchProvider>
      <Navbar />
      <main className="w-full px-4 py-6">
        {children}
      </main>
    </BranchProvider>
  )
}
