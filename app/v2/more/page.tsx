'use client'

import Link from 'next/link'
import { useRouter } from 'next/navigation'
import {
  Sparkles,
  CalendarRange,
  Wrench,
  Package,
  CreditCard,
  Receipt,
  FileText,
  BarChart3,
  ClipboardList,
  Coins,
  FileSpreadsheet,
  LayoutGrid,
  DollarSign,
  Users,
  RefreshCw,
  ScrollText,
  StickyNote,
  ClipboardCheck,
  ArrowLeft,
  LogOut,
  ExternalLink,
  type LucideIcon,
} from 'lucide-react'
import { useAuth } from '@/contexts/AuthContext'
import { V2PageHeader } from '@/components/v2/primitives'

interface HubItem {
  href: string
  label: string
  desc: string
  icon: LucideIcon
  adminOnly?: boolean
}

interface HubSection {
  title: string
  items: HubItem[]
}

const SECTIONS: HubSection[] = [
  {
    title: 'ปฏิบัติการ',
    items: [
      { href: '/v2/calendar', label: 'ปฏิทินห้องว่าง', desc: 'ผังห้องว่างตามวันที่ (ห้อง × วัน)', icon: CalendarRange },
      { href: '/housekeeping', label: 'งานแม่บ้าน', desc: 'สถานะความสะอาดของห้อง', icon: Sparkles },
      { href: '/v2/notes', label: 'โน้ตห้อง / พนักงาน', desc: 'ฝากข้อความถึงห้องและพนักงาน', icon: StickyNote },
      { href: '/maintenance', label: 'แจ้งซ่อม', desc: 'งานซ่อมบำรุงและอุปกรณ์', icon: Wrench },
      { href: '/inventory', label: 'คลังสินค้า', desc: 'สต๊อกและการเบิกจ่าย', icon: Package },
      { href: '/card-reader', label: 'อ่านบัตรประชาชน', desc: 'ดึงข้อมูลจากบัตรไทย', icon: CreditCard },
      { href: '/v2/verification/results', label: 'ตรวจสอบระบบใหม่', desc: 'ฟอร์มตรวจซ้ำแยกสาขา + ผลตรวจทั้งหมด (เทียบกับ iHOTEL)', icon: ClipboardCheck },
    ],
  },
  {
    title: 'ธุรกิจและรายงาน',
    items: [
      { href: '/billing', label: 'ใบแจ้งหนี้ / โฟลิโอ', desc: 'บิลและการชำระเงิน', icon: Receipt },
      { href: '/v2/invoice', label: 'ใบกำกับภาษี', desc: 'ออก/พิมพ์ใบกำกับภาษี A4', icon: FileText },
      { href: '/v2/cash', label: 'รายรับ-รายจ่าย', desc: 'บันทึกเงินสดเข้า-ออก', icon: Coins },
      { href: '/v2/rosters', label: 'รายชื่อผู้เข้าพัก', desc: 'มาถึง / พักอยู่ / ออก ประจำวัน — พิมพ์ A4', icon: ClipboardList },
      { href: '/reports', label: 'รายงาน', desc: 'รายได้ การเข้าพัก ADR', icon: BarChart3 },
      { href: '/reports/rr4', label: 'รร.4 (ตม.30)', desc: 'รายงานคนต่างชาติ', icon: FileSpreadsheet },
    ],
  },
  {
    title: 'ตั้งค่า',
    items: [
      { href: '/room-types', label: 'ประเภทห้อง', desc: 'จัดการประเภทและราคาฐาน', icon: LayoutGrid },
      { href: '/rates', label: 'ราคา', desc: 'ราคาตามวันและฤดูกาล', icon: DollarSign },
      { href: '/admin/users', label: 'ผู้ใช้งาน', desc: 'จัดการบัญชีและสิทธิ์', icon: Users, adminOnly: true },
      { href: '/admin/sync', label: 'สถานะการซิงค์', desc: 'การเชื่อมต่อระบบเดิม', icon: RefreshCw, adminOnly: true },
      { href: '/changelog', label: 'ประวัติเวอร์ชัน', desc: 'รายการอัปเดต', icon: ScrollText },
    ],
  },
]

export default function V2More() {
  const { user, logout } = useAuth()
  const router = useRouter()

  const handleLogout = async () => {
    await logout()
    router.replace('/login')
  }

  return (
    <div className="space-y-6">
      <V2PageHeader eyebrow="เพิ่มเติม" title="เครื่องมือทั้งหมด" />

      <p className="text-[13px] -mt-2" style={{ color: 'var(--v2-ink-3)' }}>
        เครื่องมือเหล่านี้จะเปิดในหน้าจอเวอร์ชันเดิม
      </p>

      {SECTIONS.map((section) => {
        const items = section.items.filter((i) => !i.adminOnly || user?.role === 'admin')
        if (items.length === 0) return null
        return (
          <section key={section.title}>
            <div className="v2-eyebrow mb-3">{section.title}</div>
            <div className="grid gap-3 sm:grid-cols-2 lg:grid-cols-3">
              {items.map((item) => {
                const Icon = item.icon
                return (
                  <Link
                    key={item.href}
                    href={item.href}
                    className="v2-card p-4 flex items-start gap-3.5 transition-shadow hover:shadow-[var(--v2-shadow-md)] group"
                  >
                    <span
                      className="w-11 h-11 rounded-[12px] grid place-items-center shrink-0"
                      style={{ background: 'var(--v2-wine-50)', color: 'var(--v2-wine-600)' }}
                    >
                      <Icon size={20} />
                    </span>
                    <div className="flex-1 min-w-0">
                      <div className="flex items-center gap-1.5">
                        <span className="text-[15px] font-semibold">{item.label}</span>
                        <ExternalLink size={13} className="opacity-0 group-hover:opacity-100 transition-opacity" style={{ color: 'var(--v2-ink-3)' }} />
                      </div>
                      <div className="text-[12.5px] mt-0.5" style={{ color: 'var(--v2-ink-3)' }}>{item.desc}</div>
                    </div>
                  </Link>
                )
              })}
            </div>
          </section>
        )
      })}

      {/* Account / classic */}
      <section className="pt-2">
        <div className="v2-eyebrow mb-3">บัญชี</div>
        <div className="flex flex-col sm:flex-row gap-3">
          <Link href="/" className="v2-btn v2-btn-ghost">
            <ArrowLeft size={17} /> กลับไปหน้าจอเดิม
          </Link>
          {user && (
            <button onClick={handleLogout} className="v2-btn v2-btn-ghost">
              <LogOut size={17} /> ออกจากระบบ ({user.username})
            </button>
          )}
        </div>
      </section>
    </div>
  )
}
