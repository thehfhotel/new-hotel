'use client'

import Link from 'next/link'
import { usePathname } from 'next/navigation'

/**
 * Thin top bar — brand wordmark left, breadcrumb middle.
 * Used on pages that don't render the Sidebar; most pages render the Sidebar
 * via the root AppShell and don't need this component.
 */
export default function Navbar() {
  const pathname = usePathname()

  // Best-effort breadcrumb from URL: /foo/bar -> ['foo', 'bar']
  const crumbs = pathname.replace(/^\/+/, '').split('/').filter(Boolean)

  return (
    <nav className="bg-panel border-b border-border h-10 flex items-center px-3">
      <Link href="/" className="text-[13px] font-semibold text-text whitespace-nowrap">
        ระบบจัดการโรงแรม
      </Link>
      <div className="flex items-center gap-1 ml-4 text-[12px] text-textMuted overflow-x-auto">
        {crumbs.map((c, i) => (
          <span key={i} className="flex items-center gap-1">
            <span className="text-textMuted/60">/</span>
            <span>{c}</span>
          </span>
        ))}
      </div>
      <div className="flex-1" />
    </nav>
  )
}
