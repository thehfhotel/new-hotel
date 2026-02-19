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
  CreditCard,
  ArrowLeftRight,
} from 'lucide-react'
import { useBranch, BRANCH_LABELS, type Branch } from '@/contexts/BranchContext'

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
  const { branch, setBranch, villeAvailable } = useBranch()

  return (
    <nav className="bg-blue-800 text-white shadow-lg">
      <div className="flex items-center justify-between h-16 px-2 lg:px-6">
        {/* Logo and Hotel Name */}
        <Link href="/" className="flex items-center gap-2 shrink-0">
          <Hotel size={28} className="text-blue-200 lg:w-8 lg:h-8" />
          <div className="hidden lg:block">
            <span className="text-xl font-bold whitespace-nowrap">ระบบจัดการโรงแรม</span>
            <span className="ml-2 text-xs bg-blue-600 px-2 py-0.5 rounded-full">LEGACY</span>
          </div>
        </Link>

        {/* Navigation Links */}
        <div className="flex items-center overflow-x-auto">
          {navLinks.map((link) => {
            const isActive = pathname === link.href
            return (
              <Link
                key={link.href}
                href={link.href}
                className={`flex items-center gap-1.5 px-2 lg:px-3 py-2 rounded-lg transition-colors duration-200 shrink-0
                  ${isActive
                    ? 'bg-blue-600 text-white'
                    : 'text-blue-100 hover:bg-blue-700 hover:text-white'
                  }`}
              >
                {link.icon}
                <span className="hidden lg:block whitespace-nowrap text-sm">{link.label}</span>
              </Link>
            )
          })}
        </div>

        {/* Branch Selector */}
        {(
          <div className="flex items-center gap-1 shrink-0">
            {(['hfhotel', 'hfville', 'all'] as Branch[]).map((b) => (
              <button
                key={b}
                onClick={() => setBranch(b)}
                className={`text-xs font-medium px-2 py-1 rounded-md transition-colors ${
                  branch === b
                    ? 'bg-white text-blue-800'
                    : 'text-blue-200 hover:bg-blue-700'
                }`}
              >
                {BRANCH_LABELS[b]}
              </button>
            ))}
          </div>
        )}

        {/* Switch to New Mode */}
        <div className="flex items-center shrink-0">
          <Link
            href="/new"
            className="flex items-center gap-2 px-3 py-2 bg-red-600 hover:bg-red-500 rounded-lg transition-colors text-sm"
            title="สลับไปโหมดใหม่"
          >
            <ArrowLeftRight size={18} />
            <span className="hidden lg:block">New System</span>
          </Link>
        </div>
      </div>
    </nav>
  )
}
