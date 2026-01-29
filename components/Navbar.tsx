'use client'

import Link from 'next/link'
import { usePathname } from 'next/navigation'
import {
  Home,
  Calendar,
  Users,
  BookOpen,
  BedDouble,
  Hotel,
  ScrollText,
  CreditCard
} from 'lucide-react'

interface NavLink {
  href: string
  label: string
  icon: React.ReactNode
}

const navLinks: NavLink[] = [
  { href: '/', label: 'หน้าหลัก', icon: <Home size={20} /> },
  { href: '/calendar', label: 'ปฏิทิน', icon: <Calendar size={20} /> },
  { href: '/customers', label: 'ลูกค้า', icon: <Users size={20} /> },
  { href: '/bookings', label: 'การจอง', icon: <BookOpen size={20} /> },
  { href: '/rooms', label: 'ห้องพัก', icon: <BedDouble size={20} /> },
  { href: '/card-reader', label: 'อ่านบัตร', icon: <CreditCard size={20} /> },
  { href: '/changelog', label: 'ประวัติ', icon: <ScrollText size={20} /> },
]

export default function Navbar() {
  const pathname = usePathname()

  return (
    <nav className="bg-blue-800 text-white shadow-lg">
      <div className="flex items-center justify-between h-16 px-2 lg:px-6">
        {/* Logo and Hotel Name */}
        <Link href="/" className="flex items-center gap-2 shrink-0">
          <Hotel size={28} className="text-blue-200 lg:w-8 lg:h-8" />
          <span className="hidden lg:block text-xl font-bold whitespace-nowrap">ระบบจัดการโรงแรม</span>
        </Link>

        {/* Navigation Links */}
        <div className="flex items-center">
          {navLinks.map((link) => {
            const isActive = pathname === link.href
            return (
              <Link
                key={link.href}
                href={link.href}
                className={`flex items-center gap-2 px-2 lg:px-4 py-2 rounded-lg transition-colors duration-200
                  ${isActive
                    ? 'bg-blue-600 text-white'
                    : 'text-blue-100 hover:bg-blue-700 hover:text-white'
                  }`}
              >
                {link.icon}
                <span className="hidden lg:block whitespace-nowrap">{link.label}</span>
              </Link>
            )
          })}
        </div>
      </div>
    </nav>
  )
}
