'use client'

import { useEffect, useMemo, useRef, useState } from 'react'
import { useRouter } from 'next/navigation'
import {
  Search,
  CornerDownLeft,
  LayoutDashboard,
  BedDouble,
  CalendarCheck,
  Users,
  CalendarPlus,
  LogIn,
  Sparkles,
  Package,
  BarChart3,
  Receipt,
  ArrowLeft,
  type LucideIcon,
} from 'lucide-react'

interface Command {
  id: string
  label: string
  hint?: string
  icon: LucideIcon
  href: string
  keywords?: string
}

const COMMANDS: Command[] = [
  { id: 'new-booking', label: 'สร้างการจองใหม่', hint: 'การจอง', icon: CalendarPlus, href: '/v2/reservations?new=1', keywords: 'reservation booking new จอง' },
  { id: 'walk-in', label: 'เช็คอิน (Walk-in)', hint: 'ห้องพัก', icon: LogIn, href: '/v2/rooms', keywords: 'checkin walk in เข้าพัก' },
  { id: 'find-guest', label: 'ค้นหาลูกค้า', hint: 'ลูกค้า', icon: Users, href: '/v2/guests', keywords: 'guest customer search ลูกค้า' },
  { id: 'today', label: 'หน้าหลัก', hint: 'ไปที่', icon: LayoutDashboard, href: '/v2', keywords: 'today dashboard home หน้าหลัก' },
  { id: 'rooms', label: 'ผังห้องพัก', hint: 'ไปที่', icon: BedDouble, href: '/v2/rooms', keywords: 'rooms map ห้อง' },
  { id: 'reservations', label: 'รายการจอง', hint: 'ไปที่', icon: CalendarCheck, href: '/v2/reservations', keywords: 'reservations bookings จอง' },
  { id: 'housekeeping', label: 'งานแม่บ้าน', hint: 'ไปที่', icon: Sparkles, href: '/v2/housekeeping', keywords: 'housekeeping clean แม่บ้าน' },
  { id: 'inventory', label: 'คลังสินค้า', hint: 'ไปที่', icon: Package, href: '/inventory', keywords: 'inventory stock คลัง' },
  { id: 'billing', label: 'ใบแจ้งหนี้ / โฟลิโอ', hint: 'ไปที่', icon: Receipt, href: '/billing', keywords: 'billing folio invoice บิล' },
  { id: 'reports', label: 'รายงาน', hint: 'ไปที่', icon: BarChart3, href: '/reports', keywords: 'reports revenue รายงาน' },
  { id: 'classic', label: 'กลับไปหน้าจอเดิม', hint: 'ระบบเดิม', icon: ArrowLeft, href: '/', keywords: 'classic old legacy เดิม' },
]

// Mounted only while open (the parent conditionally renders it), so initial
// state is always a clean slate — no reset-on-open effect needed.
export default function CommandPalette({ onClose }: { onClose: () => void }) {
  const router = useRouter()
  const [query, setQuery] = useState('')
  const [active, setActive] = useState(0)
  const inputRef = useRef<HTMLInputElement>(null)

  const results = useMemo(() => {
    const q = query.trim().toLowerCase()
    if (!q) return COMMANDS
    return COMMANDS.filter(
      (c) => c.label.toLowerCase().includes(q) || c.keywords?.toLowerCase().includes(q),
    )
  }, [query])

  useEffect(() => {
    // Focus the input once on mount (DOM side-effect only — no state writes).
    const t = setTimeout(() => inputRef.current?.focus(), 20)
    return () => clearTimeout(t)
  }, [])

  const onQueryChange = (v: string) => {
    setQuery(v)
    setActive(0)
  }

  const go = (cmd: Command) => {
    onClose()
    router.push(cmd.href)
  }

  const onKeyDown = (e: React.KeyboardEvent) => {
    if (e.key === 'ArrowDown') {
      e.preventDefault()
      setActive((a) => Math.min(a + 1, results.length - 1))
    } else if (e.key === 'ArrowUp') {
      e.preventDefault()
      setActive((a) => Math.max(a - 1, 0))
    } else if (e.key === 'Enter') {
      e.preventDefault()
      const cmd = results[active]
      if (cmd) go(cmd)
    } else if (e.key === 'Escape') {
      e.preventDefault()
      onClose()
    }
  }

  return (
    <div className="v2-cmdk-backdrop" onClick={onClose} role="dialog" aria-modal="true" aria-label="ค้นหาและคำสั่ง">
      <div className="v2-cmdk v2-panel-in" onClick={(e) => e.stopPropagation()}>
        <div className="flex items-center gap-3 px-4 h-14 border-b" style={{ borderColor: 'var(--v2-line)' }}>
          <Search size={18} style={{ color: 'var(--v2-ink-3)' }} />
          <input
            ref={inputRef}
            value={query}
            onChange={(e) => onQueryChange(e.target.value)}
            onKeyDown={onKeyDown}
            placeholder="ค้นหาเมนู หรือพิมพ์คำสั่ง…"
            className="flex-1 bg-transparent outline-none text-[15px]"
            style={{ color: 'var(--v2-ink)' }}
          />
          <kbd
            className="hidden sm:inline-flex items-center px-1.5 h-6 text-[11px] font-semibold rounded"
            style={{ background: 'var(--v2-surface-2)', color: 'var(--v2-ink-3)', border: '1px solid var(--v2-line)' }}
          >
            ESC
          </kbd>
        </div>

        <div className="max-h-[52vh] overflow-y-auto py-2">
          {results.length === 0 ? (
            <div className="px-4 py-8 text-center text-[14px]" style={{ color: 'var(--v2-ink-3)' }}>
              ไม่พบคำสั่งที่ตรงกับ “{query}”
            </div>
          ) : (
            results.map((cmd, i) => {
              const Icon = cmd.icon
              return (
                <div
                  key={cmd.id}
                  className="v2-cmdk-row"
                  data-active={i === active}
                  onMouseEnter={() => setActive(i)}
                  onClick={() => go(cmd)}
                >
                  <span className="v2-cmdk-ico">
                    <Icon size={18} />
                  </span>
                  <span className="flex-1">{cmd.label}</span>
                  {cmd.hint && (
                    <span className="text-[12px]" style={{ color: 'var(--v2-ink-3)' }}>
                      {cmd.hint}
                    </span>
                  )}
                  {i === active && <CornerDownLeft size={15} style={{ color: 'var(--v2-ink-3)' }} />}
                </div>
              )
            })
          )}
        </div>
      </div>
    </div>
  )
}
