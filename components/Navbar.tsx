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
  ScrollText
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
  { href: '/changelog', label: 'ประวัติ', icon: <ScrollText size={20} /> },
]

export default function Navbar() {
  const pathname = usePathname()

  return (
    <nav className="bg-blue-800 text-white shadow-lg">
      <div className="container mx-auto px-4">
        <div className="flex items-center justify-between h-16">
          {/* Logo and Hotel Name */}
          <Link href="/" className="flex items-center space-x-3">
            <Hotel size={32} className="text-blue-200" />
            <span className="text-xl font-bold">ระบบจัดการโรงแรม</span>
          </Link>

          {/* Navigation Links */}
          <div className="flex items-center space-x-1">
            {navLinks.map((link) => {
              const isActive = pathname === link.href
              return (
                <Link
                  key={link.href}
                  href={link.href}
                  className={`flex items-center space-x-2 px-4 py-2 rounded-lg transition-colors duration-200
                    ${isActive
                      ? 'bg-blue-600 text-white'
                      : 'text-blue-100 hover:bg-blue-700 hover:text-white'
                    }`}
                >
                  {link.icon}
                  <span className="hidden md:inline">{link.label}</span>
                </Link>
              )
            })}
          </div>
        </div>
      </div>
    </nav>
  )
}
