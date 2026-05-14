'use client'

import { useState, useEffect } from 'react'
import Link from 'next/link'
import { usePathname } from 'next/navigation'
import { useRouter } from 'next/navigation'
import {
  Home,
  Calendar,
  BookOpen,
  BedDouble,
  Sparkles,
  Wrench,
  Package,
  Receipt,
  BarChart3,
  LayoutGrid,
  DollarSign,
  ChevronLeft,
  ChevronRight,
  FileSpreadsheet,
  MapPin,
  Users,
  CreditCard,
  ScrollText,
  LogOut,
  UserCircle,
} from 'lucide-react'
import { useBranch, BRANCH_LABELS, type Branch } from '@/contexts/BranchContext'
import { useAuth } from '@/contexts/AuthContext'

export const SIDEBAR_WIDTH = 240
export const SIDEBAR_COLLAPSED_WIDTH = 64

interface NavItem {
  href: string
  label: string
  icon: React.ReactNode
  /** When true, the item is hidden unless the current user is an admin. */
  adminOnly?: boolean
}

interface NavSection {
  title: string
  items: NavItem[]
}

const sections: NavSection[] = [
  {
    title: 'การดำเนินงาน',
    items: [
      { href: '/', label: 'หน้าหลัก', icon: <Home size={20} /> },
      { href: '/calendar', label: 'ปฏิทิน', icon: <Calendar size={20} /> },
      { href: '/bookings', label: 'การจอง', icon: <BookOpen size={20} /> },
      { href: '/rooms', label: 'ห้องพัก', icon: <BedDouble size={20} /> },
      { href: '/customers', label: 'ลูกค้า', icon: <Users size={20} /> },
    ],
  },
  {
    title: 'ปฏิบัติการ',
    items: [
      { href: '/housekeeping', label: 'แม่บ้าน', icon: <Sparkles size={20} /> },
      { href: '/maintenance', label: 'แจ้งซ่อม', icon: <Wrench size={20} /> },
      { href: '/inventory', label: 'คลังสินค้า', icon: <Package size={20} /> },
      { href: '/card-reader', label: 'อ่านบัตร', icon: <CreditCard size={20} /> },
    ],
  },
  {
    title: 'ธุรกิจ',
    items: [
      { href: '/billing', label: 'ใบแจ้งหนี้', icon: <Receipt size={20} /> },
      { href: '/reports', label: 'รายงาน', icon: <BarChart3 size={20} /> },
      // Track G8 — legal-critical RR.4 / ตม.30 Thai immigration foreign-guest export.
      { href: '/reports/rr4', label: 'รร.4 (ตม.30)', icon: <FileSpreadsheet size={20} /> },
    ],
  },
  {
    title: 'ผู้ดูแล',
    items: [
      { href: '/room-types', label: 'ประเภทห้อง', icon: <LayoutGrid size={20} /> },
      { href: '/rates', label: 'ราคา', icon: <DollarSign size={20} /> },
      // Phase 4 PR4: admin-only — hidden when there's no logged-in
      // user OR when role !== 'admin'. The page itself enforces the
      // same gate server-side; this is just a nav-discoverability hint.
      {
        href: '/admin/users',
        label: 'ผู้ใช้งาน',
        icon: <Users size={20} />,
        adminOnly: true,
      },
      { href: '/changelog', label: 'ประวัติ', icon: <ScrollText size={20} /> },
    ],
  },
]

export default function Sidebar() {
  const pathname = usePathname()
  const router = useRouter()
  const [collapsed, setCollapsed] = useState(false)
  const [mounted, setMounted] = useState(false)
  const { branch, setBranch, villeAvailable } = useBranch()
  const { user, logout } = useAuth()

  const handleLogout = async () => {
    await logout()
    router.replace('/login')
  }

  useEffect(() => {
    const stored = localStorage.getItem('sidebar-collapsed')
    if (stored !== null) {
      setCollapsed(stored === 'true')
    } else {
      setCollapsed(window.innerWidth < 1280)
    }
    setMounted(true)
  }, [])

  const toggleCollapse = () => {
    const next = !collapsed
    setCollapsed(next)
    localStorage.setItem('sidebar-collapsed', String(next))
    window.dispatchEvent(new Event('sidebar-toggle'))
  }

  const isActive = (href: string) => {
    if (href === '/') return pathname === '/'
    return pathname.startsWith(href)
  }

  const width = collapsed ? SIDEBAR_COLLAPSED_WIDTH : SIDEBAR_WIDTH

  return (
    <aside
      className={`fixed top-0 left-0 h-screen bg-panel border-r border-border z-40 flex flex-col transition-all duration-300 ${
        !mounted ? 'opacity-0' : 'opacity-100'
      }`}
      style={{ width }}
    >
      {/* Brand bar — minimal wordmark, no icon, no NEW pill */}
      <div className="flex items-center px-3 h-12 border-b border-border shrink-0">
        {!collapsed ? (
          <span className="text-[14px] font-semibold text-text tracking-tight whitespace-nowrap overflow-hidden">
            ระบบจัดการโรงแรม
          </span>
        ) : (
          <div className="w-full text-center text-[12px] font-semibold text-brand-500">HF</div>
        )}
      </div>

      {/* Branch Selector */}
      <div className="px-2 py-1.5 border-b border-border shrink-0">
        {collapsed ? (
          <button
            onClick={() => {
              const branches: Branch[] = ['hfhotel', 'hfville', 'all']
              const idx = branches.indexOf(branch)
              setBranch(branches[(idx + 1) % branches.length])
            }}
            className="w-full flex items-center justify-center p-1.5 text-textMuted hover:bg-headerBar"
            title={BRANCH_LABELS[branch]}
          >
            <MapPin
              size={16}
              className={
                branch === 'hfhotel'
                  ? 'text-brand-500'
                  : branch === 'hfville'
                  ? 'text-info'
                  : 'text-textMuted'
              }
            />
          </button>
        ) : (
          <div className="flex gap-0.5">
            {(['hfhotel', 'hfville', 'all'] as Branch[]).map((b) => {
              const disabled = b === 'hfville' && !villeAvailable
              return (
                <button
                  key={b}
                  onClick={() => !disabled && setBranch(b)}
                  disabled={disabled}
                  className={`flex-1 text-[11px] font-medium py-1 transition-colors ${
                    branch === b
                      ? 'bg-brand-500 text-white'
                      : 'text-textMuted hover:bg-headerBar disabled:opacity-50'
                  }`}
                >
                  {BRANCH_LABELS[b]}
                </button>
              )
            })}
          </div>
        )}
      </div>

      {/* Navigation. Admin-only items are filtered out unless the
          current user has the admin role — see PR4. When the user is
          null (not logged in OR auth disabled) admin items stay
          hidden, matching the additive-only PR contract. */}
      <nav className="flex-1 overflow-y-auto py-2 px-1">
        {sections.map((section, idx) => {
          const visibleItems = section.items.filter(
            (item) => !item.adminOnly || user?.role === 'admin',
          )
          if (visibleItems.length === 0) return null
          return (
            <div key={section.title} className={idx > 0 ? 'mt-3' : ''}>
              {!collapsed && (
                <div className="text-[10px] uppercase tracking-wide text-textMuted font-semibold px-3 py-1">
                  {section.title}
                </div>
              )}
              <div>
                {visibleItems.map((item) => (
                  <Link
                    key={item.href}
                    href={item.href}
                    title={collapsed ? item.label : undefined}
                    className={`flex items-center gap-2 px-3 py-1.5 transition-colors text-[13px] ${
                      isActive(item.href)
                        ? 'bg-brand-50 border-l-[3px] border-brand-500 text-brand-700 font-semibold'
                        : 'border-l-[3px] border-transparent text-text hover:bg-headerBar'
                    } ${collapsed ? 'justify-center' : ''}`}
                  >
                    <span className="shrink-0">{item.icon}</span>
                    {!collapsed && <span>{item.label}</span>}
                  </Link>
                ))}
              </div>
            </div>
          )
        })}
      </nav>

      {/* Bottom */}
      <div className="border-t border-border p-1 shrink-0">
        {/* Logged-in user + logout. Hidden entirely when there is no user
            so dev / pre-cutover (NEXT_PUBLIC_AUTH_REQUIRED=false) shows
            no auth chrome at all — matches the additive-only PR contract. */}
        {user && (
          <div className="border-b border-border pb-1 mb-1">
            {!collapsed && (
              <div
                className="flex items-center gap-2 px-3 py-1 text-[12px] text-text"
                title={`${user.username} (${user.role})`}
              >
                <UserCircle size={14} className="text-textMuted shrink-0" />
                <span className="truncate font-medium">{user.username}</span>
                <span className="text-[10px] text-textMuted uppercase tracking-wide ml-auto">
                  {user.role}
                </span>
              </div>
            )}
            <button
              onClick={handleLogout}
              title="ออกจากระบบ"
              className={`flex items-center gap-2 px-3 py-1.5 text-[13px] text-textMuted hover:bg-headerBar hover:text-text transition-colors w-full ${
                collapsed ? 'justify-center' : ''
              }`}
            >
              <LogOut size={16} />
              {!collapsed && <span>ออกจากระบบ</span>}
            </button>
          </div>
        )}
        <button
          onClick={toggleCollapse}
          className={`flex items-center gap-2 px-3 py-1.5 text-[13px] text-textMuted hover:bg-headerBar hover:text-text transition-colors w-full ${
            collapsed ? 'justify-center' : ''
          }`}
        >
          {collapsed ? <ChevronRight size={16} /> : <ChevronLeft size={16} />}
          {!collapsed && <span>ย่อเมนู</span>}
        </button>
      </div>
    </aside>
  )
}
