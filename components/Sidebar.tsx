'use client'

import { useState, useEffect } from 'react'
import Link from 'next/link'
import { usePathname } from 'next/navigation'
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
  Hotel,
  ArrowLeftRight,
  ChevronLeft,
  ChevronRight,
} from 'lucide-react'

export const SIDEBAR_WIDTH = 240
export const SIDEBAR_COLLAPSED_WIDTH = 64

interface NavItem {
  href: string
  label: string
  icon: React.ReactNode
}

interface NavSection {
  title: string
  items: NavItem[]
}

const sections: NavSection[] = [
  {
    title: 'การดำเนินงาน',
    items: [
      { href: '/new', label: 'หน้าหลัก', icon: <Home size={20} /> },
      { href: '/new/calendar', label: 'ปฏิทิน', icon: <Calendar size={20} /> },
      { href: '/new/bookings', label: 'การจอง', icon: <BookOpen size={20} /> },
      { href: '/new/rooms', label: 'ห้องพัก', icon: <BedDouble size={20} /> },
    ],
  },
  {
    title: 'ปฏิบัติการ',
    items: [
      { href: '/new/housekeeping', label: 'แม่บ้าน', icon: <Sparkles size={20} /> },
      { href: '/new/maintenance', label: 'แจ้งซ่อม', icon: <Wrench size={20} /> },
      { href: '/new/inventory', label: 'คลังสินค้า', icon: <Package size={20} /> },
    ],
  },
  {
    title: 'ธุรกิจ',
    items: [
      { href: '/new/billing', label: 'ใบแจ้งหนี้', icon: <Receipt size={20} /> },
      { href: '/new/reports', label: 'รายงาน', icon: <BarChart3 size={20} /> },
    ],
  },
  {
    title: 'ผู้ดูแล',
    items: [
      { href: '/new/room-types', label: 'ประเภทห้อง', icon: <LayoutGrid size={20} /> },
      { href: '/new/rates', label: 'ราคา', icon: <DollarSign size={20} /> },
    ],
  },
]

export default function Sidebar() {
  const pathname = usePathname()
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
  }, [])

  const toggleCollapse = () => {
    const next = !collapsed
    setCollapsed(next)
    localStorage.setItem('sidebar-collapsed', String(next))
    window.dispatchEvent(new Event('sidebar-toggle'))
  }

  const isActive = (href: string) => {
    if (href === '/new') return pathname === '/new'
    return pathname.startsWith(href)
  }

  const width = collapsed ? SIDEBAR_COLLAPSED_WIDTH : SIDEBAR_WIDTH

  return (
    <aside
      className={`fixed top-0 left-0 h-screen bg-white border-r border-gray-200 z-40 flex flex-col transition-all duration-300 ${
        !mounted ? 'opacity-0' : 'opacity-100'
      }`}
      style={{ width }}
    >
      {/* Logo */}
      <div className="flex items-center gap-2 px-3 h-16 border-b border-gray-200 shrink-0">
        <Hotel size={28} className="text-red-500 shrink-0" />
        {!collapsed && (
          <div className="overflow-hidden whitespace-nowrap">
            <span className="text-lg font-bold text-gray-900 tracking-tight">ระบบจัดการโรงแรม</span>
            <span className="ml-2 text-[10px] font-semibold bg-red-600 text-white px-1.5 py-0.5 rounded tracking-wider uppercase">
              New
            </span>
          </div>
        )}
      </div>

      {/* Navigation */}
      <nav className="flex-1 overflow-y-auto py-4 px-2">
        {sections.map((section, idx) => (
          <div key={section.title} className={idx > 0 ? 'mt-6' : ''}>
            {!collapsed && (
              <div className="text-[10px] uppercase tracking-wider text-gray-500 font-semibold px-3 mb-1">
                {section.title}
              </div>
            )}
            <div className="space-y-1">
              {section.items.map((item) => (
                <Link
                  key={item.href}
                  href={item.href}
                  title={collapsed ? item.label : undefined}
                  className={`flex items-center gap-3 px-3 py-2 rounded-lg transition-colors text-sm ${
                    isActive(item.href)
                      ? 'bg-red-600/15 text-red-600 border-l-2 border-red-500'
                      : 'text-gray-500 hover:bg-gray-100 hover:text-gray-900'
                  } ${collapsed ? 'justify-center' : ''}`}
                >
                  <span className="shrink-0">{item.icon}</span>
                  {!collapsed && <span>{item.label}</span>}
                </Link>
              ))}
            </div>
          </div>
        ))}
      </nav>

      {/* Bottom */}
      <div className="border-t border-gray-200 p-2 shrink-0 space-y-1">
        <Link
          href="/"
          title={collapsed ? 'Legacy' : undefined}
          className={`flex items-center gap-3 px-3 py-2 rounded-lg text-sm text-gray-500 hover:bg-gray-100 hover:text-gray-900 transition-colors ${
            collapsed ? 'justify-center' : ''
          }`}
        >
          <ArrowLeftRight size={20} className="shrink-0" />
          {!collapsed && <span>Legacy</span>}
        </Link>
        <button
          onClick={toggleCollapse}
          className={`flex items-center gap-3 px-3 py-2 rounded-lg text-sm text-gray-500 hover:bg-gray-100 hover:text-gray-900 transition-colors w-full ${
            collapsed ? 'justify-center' : ''
          }`}
        >
          {collapsed ? <ChevronRight size={20} /> : <ChevronLeft size={20} />}
          {!collapsed && <span>ย่อเมนู</span>}
        </button>
      </div>
    </aside>
  )
}
