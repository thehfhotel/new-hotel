'use client'

import { useEffect, useState, type ReactNode } from 'react'
import Link from 'next/link'
import { usePathname, useRouter } from 'next/navigation'
import { Search, LogOut, Command as CommandIcon } from 'lucide-react'
import { useBranch, BRANCH_LABELS, type Branch } from '@/contexts/BranchContext'
import { useAuth } from '@/contexts/AuthContext'
import { V2_NAV, isV2Active } from '@/lib/v2/nav'
import CommandPalette from '@/components/v2/CommandPalette'

function BranchSwitch({ compact = false }: { compact?: boolean }) {
  const { branch, setBranch, villeAvailable } = useBranch()
  const branches: Branch[] = ['hfhotel', 'hfville', 'all']
  return (
    <div
      className="flex p-0.5 rounded-full"
      style={{ background: 'var(--v2-surface-2)', border: '1px solid var(--v2-line)' }}
    >
      {branches.map((b) => {
        const disabled = b === 'hfville' && !villeAvailable
        const activeB = branch === b
        return (
          <button
            key={b}
            onClick={() => !disabled && setBranch(b)}
            disabled={disabled}
            className={`rounded-full font-semibold transition-colors disabled:opacity-40 ${
              compact ? 'px-2.5 py-1 text-[12px]' : 'px-3 py-1.5 text-[12.5px]'
            }`}
            style={
              activeB
                ? { background: 'var(--v2-wine-600)', color: '#fff' }
                : { color: 'var(--v2-ink-2)' }
            }
          >
            {b === 'all' ? 'ทั้งหมด' : BRANCH_LABELS[b]}
          </button>
        )
      })}
    </div>
  )
}

export default function V2Shell({ children }: { children: ReactNode }) {
  const pathname = usePathname() || '/v2'
  const router = useRouter()
  const { user, logout } = useAuth()
  const [cmdkOpen, setCmdkOpen] = useState(false)

  // ⌘K / Ctrl-K opens the command palette anywhere in the new UI.
  useEffect(() => {
    const onKey = (e: KeyboardEvent) => {
      if ((e.metaKey || e.ctrlKey) && e.key.toLowerCase() === 'k') {
        e.preventDefault()
        setCmdkOpen((v) => !v)
      }
    }
    window.addEventListener('keydown', onKey)
    return () => window.removeEventListener('keydown', onKey)
  }, [])

  const handleLogout = async () => {
    await logout()
    router.replace('/login')
  }

  return (
    <div className="min-h-screen">
      {/* ── Desktop rail ─────────────────────────────────────────────── */}
      <aside
        className="hidden lg:flex flex-col fixed inset-y-0 left-0 w-[248px] z-30"
        style={{ background: 'var(--v2-surface)', borderRight: '1px solid var(--v2-line)' }}
      >
        <div className="px-5 pt-6 pb-5">
          <Link href="/v2" className="block">
            <div className="v2-eyebrow" style={{ color: 'var(--v2-brass)' }}>Saichon Heritage</div>
            <div className="v2-display text-[19px] leading-tight mt-1" style={{ color: 'var(--v2-ink)' }}>
              Front&nbsp;Desk
            </div>
          </Link>
        </div>

        <div className="px-4 pb-4">
          <button
            onClick={() => setCmdkOpen(true)}
            className="w-full flex items-center gap-2.5 px-3 h-10 rounded-[10px] text-[13.5px]"
            style={{ background: 'var(--v2-surface-2)', border: '1px solid var(--v2-line)', color: 'var(--v2-ink-3)' }}
          >
            <Search size={16} />
            <span className="flex-1 text-left">ค้นหา…</span>
            <span className="flex items-center gap-0.5 text-[11px]">
              <CommandIcon size={12} /> K
            </span>
          </button>
        </div>

        <nav className="flex-1 overflow-y-auto px-4 space-y-1">
          {V2_NAV.map((item) => {
            const Icon = item.icon
            return (
              <Link
                key={item.href}
                href={item.href}
                className="v2-rail-item"
                data-active={isV2Active(pathname, item.href)}
              >
                <Icon size={20} />
                <span>{item.label}</span>
              </Link>
            )
          })}
        </nav>

        <div className="p-4 space-y-3" style={{ borderTop: '1px solid var(--v2-line)' }}>
          <BranchSwitch />
          {user ? (
            <div className="flex items-center gap-2">
              <div
                className="w-9 h-9 rounded-full grid place-items-center text-[13px] font-semibold v2-num"
                style={{ background: 'var(--v2-wine-50)', color: 'var(--v2-wine-700)' }}
              >
                {user.username.slice(0, 2).toUpperCase()}
              </div>
              <div className="flex-1 min-w-0">
                <div className="text-[13px] font-semibold truncate">{user.username}</div>
                <div className="text-[11px]" style={{ color: 'var(--v2-ink-3)' }}>{user.role}</div>
              </div>
              <button
                onClick={handleLogout}
                title="ออกจากระบบ"
                className="p-2 rounded-[8px] transition-colors"
                style={{ color: 'var(--v2-ink-3)' }}
              >
                <LogOut size={17} />
              </button>
            </div>
          ) : (
            <Link href="/" className="block text-center text-[12px] py-1" style={{ color: 'var(--v2-ink-3)' }}>
              กลับไปหน้าจอเดิม
            </Link>
          )}
        </div>
      </aside>

      {/* ── Mobile top bar ───────────────────────────────────────────── */}
      <header
        className="lg:hidden sticky top-0 z-30 flex items-center gap-3 px-4 h-14"
        style={{ background: 'var(--v2-surface)', borderBottom: '1px solid var(--v2-line)' }}
      >
        <Link href="/v2" className="v2-display text-[16px]" style={{ color: 'var(--v2-ink)' }}>
          Front&nbsp;Desk
        </Link>
        <div className="flex-1" />
        <BranchSwitch compact />
        <button
          onClick={() => setCmdkOpen(true)}
          className="p-2 rounded-full"
          style={{ background: 'var(--v2-surface-2)', border: '1px solid var(--v2-line)', color: 'var(--v2-ink-2)' }}
          aria-label="ค้นหา"
        >
          <Search size={18} />
        </button>
      </header>

      {/* ── Main content ─────────────────────────────────────────────── */}
      <main className="lg:pl-[248px] pb-[76px] lg:pb-0">
        <div className="mx-auto max-w-[1240px] px-4 lg:px-9 py-5 lg:py-8">{children}</div>
      </main>

      {/* ── Mobile bottom tab bar ────────────────────────────────────── */}
      <nav
        className="lg:hidden fixed bottom-0 inset-x-0 z-30 flex px-2"
        style={{
          background: 'var(--v2-surface)',
          borderTop: '1px solid var(--v2-line)',
          paddingBottom: 'env(safe-area-inset-bottom)',
        }}
      >
        {V2_NAV.map((item) => {
          const Icon = item.icon
          return (
            <Link key={item.href} href={item.href} className="v2-tab" data-active={isV2Active(pathname, item.href)}>
              <span className="v2-tab-ico">
                <Icon size={20} />
              </span>
              <span>{item.label}</span>
            </Link>
          )
        })}
      </nav>

      {cmdkOpen && <CommandPalette onClose={() => setCmdkOpen(false)} />}
    </div>
  )
}
