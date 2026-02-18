'use client'

import { useState, useEffect, useCallback } from 'react'
import Link from 'next/link'
import {
  Sparkles,
  Package,
  Wrench,
  Receipt,
  BarChart3,
  ArrowRight,
  Loader2,
  AlertCircle,
  Calendar,
} from 'lucide-react'

interface Stats {
  totalRooms: number
  occupiedRooms: number
  availableRooms: number
  bookedRooms: number
  checkoutRooms: number
  totalCustomers: number
  activeBookings: number
  todayCheckIns: number
  todayCheckOuts: number
}

interface QuickLink {
  href: string
  label: string
  description: string
  icon: React.ReactNode
  accent: string
}

const quickLinks: QuickLink[] = [
  {
    href: '/new/bookings',
    label: 'การจอง',
    description: 'จัดการการจองห้องพัก',
    icon: <Calendar size={24} />,
    accent: 'text-red-400 bg-red-500/10 border-red-500/20',
  },
  {
    href: '/new/housekeeping',
    label: 'แม่บ้าน',
    description: 'สถานะความสะอาดห้อง',
    icon: <Sparkles size={24} />,
    accent: 'text-amber-400 bg-amber-500/10 border-amber-500/20',
  },
  {
    href: '/new/inventory',
    label: 'คลังสินค้า',
    description: 'จัดการสต็อกและอุปกรณ์',
    icon: <Package size={24} />,
    accent: 'text-sky-400 bg-sky-500/10 border-sky-500/20',
  },
  {
    href: '/new/maintenance',
    label: 'แจ้งซ่อม',
    description: 'ติดตามงานซ่อมบำรุง',
    icon: <Wrench size={24} />,
    accent: 'text-orange-400 bg-orange-500/10 border-orange-500/20',
  },
  {
    href: '/new/billing',
    label: 'ใบแจ้งหนี้',
    description: 'ดูและพิมพ์ใบแจ้งหนี้',
    icon: <Receipt size={24} />,
    accent: 'text-emerald-400 bg-emerald-500/10 border-emerald-500/20',
  },
  {
    href: '/new/reports',
    label: 'รายงาน',
    description: 'วิเคราะห์รายได้และอัตราเข้าพัก',
    icon: <BarChart3 size={24} />,
    accent: 'text-violet-400 bg-violet-500/10 border-violet-500/20',
  },
]

export default function NewDashboard() {
  const [stats, setStats] = useState<Stats>({
    totalRooms: 0,
    occupiedRooms: 0,
    availableRooms: 0,
    bookedRooms: 0,
    checkoutRooms: 0,
    totalCustomers: 0,
    activeBookings: 0,
    todayCheckIns: 0,
    todayCheckOuts: 0,
  })
  const [loading, setLoading] = useState(true)
  const [error, setError] = useState<string | null>(null)

  const fetchStats = useCallback(async () => {
    try {
      const res = await fetch('/api/stats')
      if (res.ok) {
        const data = await res.json()
        if (data.success) {
          const d = data.data
          setStats({
            ...d,
            availableRooms: d.totalRooms - d.occupiedRooms - d.checkoutRooms - d.bookedRooms,
          })
        }
      }
    } catch (err) {
      setError('ไม่สามารถโหลดข้อมูลได้')
      console.error('Error fetching stats:', err)
    } finally {
      setLoading(false)
    }
  }, [])

  useEffect(() => {
    fetchStats()
  }, [fetchStats])

  if (loading) {
    return (
      <div className="flex items-center justify-center min-h-[50vh]">
        <div className="text-center">
          <Loader2 className="animate-spin h-12 w-12 text-red-500 mx-auto mb-4" />
          <p className="text-zinc-500">กำลังโหลดข้อมูล...</p>
        </div>
      </div>
    )
  }

  return (
    <div className="space-y-6">
      {/* Page Header */}
      <div className="flex items-center justify-between">
        <div>
          <h1 className="text-2xl font-bold text-zinc-100 tracking-tight">ระบบจัดการโรงแรม</h1>
          <p className="text-zinc-500">Hotel Management System</p>
        </div>
        <p className="text-zinc-600 text-sm">
          อัปเดตล่าสุด: {new Date().toLocaleDateString('th-TH', {
            year: 'numeric',
            month: 'long',
            day: 'numeric',
            hour: '2-digit',
            minute: '2-digit',
          })}
        </p>
      </div>

      {/* Error Message */}
      {error && (
        <div className="flex items-center gap-2 p-4 bg-red-950/50 border border-red-900/50 rounded-lg text-red-400">
          <AlertCircle className="w-5 h-5 flex-shrink-0" />
          <span>{error}</span>
        </div>
      )}

      {/* Stats Cards */}
      <div className="grid grid-cols-2 md:grid-cols-3 lg:grid-cols-6 gap-4">
        <div className="bg-zinc-900 rounded-xl px-6 py-5 border border-zinc-800">
          <p className="text-sm font-medium text-zinc-500 mb-1">ห้องทั้งหมด</p>
          <p className="text-3xl font-bold text-zinc-100">{stats.totalRooms}</p>
        </div>
        <div className="bg-zinc-900 rounded-xl px-6 py-5 border border-zinc-800">
          <p className="text-sm font-medium text-zinc-500 mb-1">ห้องว่าง</p>
          <p className="text-3xl font-bold text-emerald-400">{stats.availableRooms}</p>
        </div>
        <div className="bg-zinc-900 rounded-xl px-6 py-5 border border-zinc-800">
          <p className="text-sm font-medium text-zinc-500 mb-1">มีผู้เข้าพัก</p>
          <p className="text-3xl font-bold text-amber-400">{stats.occupiedRooms}</p>
        </div>
        <div className="bg-zinc-900 rounded-xl px-6 py-5 border border-zinc-800">
          <p className="text-sm font-medium text-zinc-500 mb-1">จองแล้ว</p>
          <p className="text-3xl font-bold text-sky-400">{stats.bookedRooms}</p>
        </div>
        <div className="bg-zinc-900 rounded-xl px-6 py-5 border border-zinc-800">
          <p className="text-sm font-medium text-zinc-500 mb-1">เช็คอินวันนี้</p>
          <p className="text-3xl font-bold text-zinc-100">{stats.todayCheckIns}</p>
        </div>
        <div className="bg-zinc-900 rounded-xl px-6 py-5 border border-zinc-800">
          <p className="text-sm font-medium text-zinc-500 mb-1">เช็คเอาท์วันนี้</p>
          <p className="text-3xl font-bold text-red-400">{stats.todayCheckOuts}</p>
        </div>
      </div>

      {/* Quick Links */}
      <div>
        <h2 className="text-lg font-semibold text-zinc-200 mb-4">เมนูหลัก</h2>
        <div className="grid grid-cols-2 md:grid-cols-3 lg:grid-cols-6 gap-4">
          {quickLinks.map((link) => (
            <Link
              key={link.href}
              href={link.href}
              className="bg-zinc-900 rounded-xl border border-zinc-800 p-4 hover:border-zinc-700 transition-all group"
            >
              <div className={`w-12 h-12 rounded-lg flex items-center justify-center mb-3 border ${link.accent}`}>
                {link.icon}
              </div>
              <h3 className="font-semibold text-zinc-200 group-hover:text-red-400 transition-colors">
                {link.label}
              </h3>
              <p className="text-sm text-zinc-500 mt-1">{link.description}</p>
              <div className="flex items-center gap-1 text-red-500 text-sm mt-2 opacity-0 group-hover:opacity-100 transition-opacity">
                <span>เปิด</span>
                <ArrowRight size={14} />
              </div>
            </Link>
          ))}
        </div>
      </div>

    </div>
  )
}
