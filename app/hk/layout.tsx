import type { Metadata } from 'next'

export const metadata: Metadata = {
  title: 'แม่บ้าน | HF Hotel',
  description: 'รายงานความสะอาดห้องพักและแจ้งของชำรุด',
}

/**
 * Minimal mobile shell for the maid-facing housekeeping surface.
 *
 * The /hk subtree is CHROMELESS (components/AppShell.tsx skips the desktop
 * sidebar, BranchProvider, and the HF One band for it) — maids open these
 * pages inside LINE's in-app browser from their Role Menu, behind the /hk
 * Cloudflare Access application (silent HF ID login, grant `housekeeping`).
 * Identity comes from the backend's hk_access middleware; there is no login
 * UI here by design — an unauthenticated state renders a fail-closed notice.
 */
export default function HkLayout({ children }: { children: React.ReactNode }) {
  return (
    <div className="min-h-screen bg-gray-50 text-gray-900">
      <div className="mx-auto w-full max-w-lg px-4 pb-16 pt-4">{children}</div>
    </div>
  )
}
