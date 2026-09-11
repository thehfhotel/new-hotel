'use client'

import { useState, useEffect } from 'react'
import { usePathname } from 'next/navigation'
import Script from 'next/script'
import Sidebar, { SIDEBAR_WIDTH, SIDEBAR_COLLAPSED_WIDTH } from '@/components/Sidebar'
import { BranchProvider } from '@/contexts/BranchContext'
import type { ShellProperty } from '@/lib/server/shell-property'

// Pages that should render WITHOUT the sidebar / branch context — the only
// public route today is /login (the auth landing). Kept as a Set so additional
// future public pages (e.g. /forgot-password) only need a single edit.
const CHROMELESS_PATHS = new Set<string>(['/login'])

/**
 * Client shell that renders the sidebar + main content area for every page.
 * Wraps children in the BranchProvider so any descendant can read the active
 * branch via `useBranch()` / `useBranchFetch()`.
 *
 * Sidebar collapse state is mirrored from localStorage and kept in sync via
 * the `sidebar-toggle` window event dispatched by `<Sidebar>`.
 *
 * For chromeless paths (e.g. /login), the sidebar + BranchProvider are
 * skipped entirely so the auth screen can occupy the full viewport.
 *
 * The HF One marquee band (hf-bar.js, see design/HF-ONE.md) is mounted here
 * rather than in app/layout.tsx: the root layout is a Server Component that
 * doesn't re-run on client-side navigation, so it can't gate by route. This
 * component already does that via `usePathname()` for the chromeless check,
 * so it's the natural (and only reactive) place to also gate the band.
 * It's skipped on /v2/* — that experience has its own `fixed inset-y-0`
 * desktop rail (components/v2/V2Shell.tsx) with the same viewport-locked
 * overlap the classic Sidebar had, and isn't wired up to
 * `--hf-band-offset` — and on /login, matching the existing chromeless set.
 */
export default function AppShell({
  children,
  shellProperty,
}: {
  children: React.ReactNode
  /**
   * The property this viewer is standing at, resolved per request from the
   * server-verified Cloudflare Access identity in `app/layout.tsx`
   * (`lib/server/shell-property.ts`). `undefined` for every identity that does
   * not name a place — HF's reception account (which also runs as a second
   * Chrome profile on the HF Ville PC), the office kiosk, managers, phones,
   * anonymous callers, and any failure — and then the attribute is omitted and
   * the band lists every tool.
   */
  shellProperty?: ShellProperty
}) {
  const pathname = usePathname()
  // The new "/v2" front-desk experience ships its own shell (rail + bottom nav
  // + its own BranchProvider) and must render WITHOUT the classic sidebar.
  // Treat the whole /v2 subtree as chromeless, like /login.
  // The "/hk" maid-facing housekeeping surface (employee-login plan Phase 4)
  // is likewise chromeless: it ships its own minimal mobile shell
  // (app/hk/layout.tsx) for LINE's in-app browser and must not load the
  // desktop sidebar, BranchProvider, or the HF One band.
  const isChromeless = pathname
    ? CHROMELESS_PATHS.has(pathname) ||
      pathname === '/v2' ||
      pathname.startsWith('/v2/') ||
      pathname === '/hk' ||
      pathname.startsWith('/hk/')
    : false

  if (isChromeless) {
    return <>{children}</>
  }

  return (
    <>
      <Script
        src="https://erp.thehfhotel.org/shell/hf-bar.js"
        data-app="Hotel PMS"
        data-module="front-desk"
        // Spread, not `data-property={shellProperty}`: React renders an
        // `undefined` value by omitting the attribute, but being explicit keeps
        // "omit entirely" — the fail-open default the band relies on — from
        // depending on that behaviour.
        {...(shellProperty ? { 'data-property': shellProperty } : {})}
        strategy="afterInteractive"
      />
      <ChromedShell>{children}</ChromedShell>
    </>
  )
}

function ChromedShell({ children }: { children: React.ReactNode }) {
  const [collapsed, setCollapsed] = useState(false)
  const [mounted, setMounted] = useState(false)

  useEffect(() => {
    const stored = localStorage.getItem('sidebar-collapsed')
    if (stored !== null) {
      setCollapsed(stored === 'true')
    } else {
      setCollapsed(window.innerWidth < 1280)
    }
    setMounted(true)

    const handler = () => {
      const val = localStorage.getItem('sidebar-collapsed')
      setCollapsed(val === 'true')
    }
    window.addEventListener('sidebar-toggle', handler)
    return () => window.removeEventListener('sidebar-toggle', handler)
  }, [])

  return (
    <BranchProvider>
      {/* min-h-screen would overshoot by --hf-band-offset once the (normal-
          flow) hf-bar has already claimed that space above this div, leaving
          a dead-scroll gap at the bottom. Sizing to the remainder keeps the
          page exactly one viewport tall whether or not the band mounted. */}
      <div className="min-h-[calc(100vh_-_var(--hf-band-offset))] bg-gray-50">
        <Sidebar />
        <main
          className={`min-h-[calc(100vh_-_var(--hf-band-offset))] px-6 py-6 ${mounted ? 'transition-all duration-300' : ''}`}
          style={{ marginLeft: mounted ? (collapsed ? SIDEBAR_COLLAPSED_WIDTH : SIDEBAR_WIDTH) : SIDEBAR_WIDTH }}
        >
          {children}
        </main>
      </div>
    </BranchProvider>
  )
}
