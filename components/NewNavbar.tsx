'use client'

import Link from 'next/link'
import { usePathname } from 'next/navigation'
import { ArrowLeftRight } from 'lucide-react'

/**
 * Thin top bar — used on pages that don't render the Sidebar (legacy fallback).
 * Most /new/* pages render the Sidebar instead and don't show this navbar.
 * Kept minimal: brand wordmark left, breadcrumb middle, Legacy link right.
 */
export default function NewNavbar() {
  const pathname = usePathname()

  // Best-effort breadcrumb from URL: /new/foo/bar -> ['foo', 'bar']
  const crumbs = pathname.replace(/^\/new\/?/, '').split('/').filter(Boolean)

  return (
    <nav className="bg-panel border-b border-border h-10 flex items-center px-3">
      <Link href="/new" className="text-[13px] font-semibold text-text whitespace-nowrap">
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
      <Link
        href="/"
        className="inline-flex items-center gap-1 px-2 h-6 text-[12px] text-textMuted hover:bg-headerBar hover:text-text transition-colors"
        title="สลับไปโหมดเดิม"
      >
        <ArrowLeftRight size={14} />
        <span className="hidden lg:inline">Legacy</span>
      </Link>
    </nav>
  )
}
