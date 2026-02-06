'use client'

import Link from 'next/link'
import { usePathname } from 'next/navigation'
import {
  Home,
  Calendar,
  BookOpen,
  BedDouble,
  Hotel,
  BarChart3,
  Receipt,
  Wrench,
  Sparkles,
  Package,
  DollarSign,
  ArrowLeftRight,
} from 'lucide-react'

interface NavLink {
  href: string
  label: string
  icon: React.ReactNode
}

const navLinks: NavLink[] = [
  { href: '/new', label: 'หน้าหลัก', icon: <Home size={20} /> },
  { href: '/new/calendar', label: 'ปฏิทิน', icon: <Calendar size={20} /> },
  { href: '/new/bookings', label: 'การจอง', icon: <BookOpen size={20} /> },
  { href: '/new/room-types', label: 'ประเภทห้อง', icon: <BedDouble size={20} /> },
  { href: '/new/rates', label: 'ราคา', icon: <DollarSign size={20} /> },
  { href: '/new/housekeeping', label: 'แม่บ้าน', icon: <Sparkles size={20} /> },
  { href: '/new/inventory', label: 'คลังสินค้า', icon: <Package size={20} /> },
  { href: '/new/maintenance', label: 'แจ้งซ่อม', icon: <Wrench size={20} /> },
  { href: '/new/billing', label: 'ใบแจ้งหนี้', icon: <Receipt size={20} /> },
  { href: '/new/reports', label: 'รายงาน', icon: <BarChart3 size={20} /> },
]

export default function NewNavbar() {
  const pathname = usePathname()

  return (
    <nav className="bg-zinc-900 text-zinc-300 border-b border-zinc-800">
      <div className="flex items-center justify-between h-16 px-2 lg:px-6">
        {/* Logo and Hotel Name */}
        <Link href="/new" className="flex items-center gap-2 shrink-0">
          <Hotel size={28} className="text-red-500 lg:w-8 lg:h-8" />
          <div className="hidden lg:block">
            <span className="text-xl font-bold text-white whitespace-nowrap tracking-tight">ระบบจัดการโรงแรม</span>
            <span className="ml-2 text-[10px] font-semibold bg-red-600 text-white px-2 py-0.5 rounded tracking-wider uppercase">New</span>
          </div>
        </Link>

        {/* Navigation Links */}
        <div className="flex items-center overflow-x-auto">
          {navLinks.map((link) => {
            const isActive = pathname === link.href ||
              (link.href !== '/new' && pathname.startsWith(link.href))
            return (
              <Link
                key={link.href}
                href={link.href}
                className={`flex items-center gap-1.5 px-2 lg:px-3 py-2 rounded-lg transition-colors duration-200 shrink-0
                  ${isActive
                    ? 'bg-red-600/15 text-red-400 border border-red-600/30'
                    : 'text-zinc-400 hover:bg-zinc-800 hover:text-zinc-200 border border-transparent'
                  }`}
              >
                {link.icon}
                <span className="hidden lg:block whitespace-nowrap text-sm">{link.label}</span>
              </Link>
            )
          })}
        </div>

        {/* Switch to Legacy Mode */}
        <div className="flex items-center shrink-0">
          <Link
            href="/"
            className="flex items-center gap-2 px-3 py-2 bg-zinc-800 hover:bg-zinc-700 text-zinc-300 hover:text-white rounded-lg transition-colors text-sm border border-zinc-700"
            title="สลับไปโหมดเดิม"
          >
            <ArrowLeftRight size={18} />
            <span className="hidden lg:block">Legacy</span>
          </Link>
        </div>
      </div>
    </nav>
  )
}
