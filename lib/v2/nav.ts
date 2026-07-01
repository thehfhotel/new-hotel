import {
  LayoutDashboard,
  BedDouble,
  CalendarCheck,
  Users,
  ClipboardList,
  LayoutGrid,
  type LucideIcon,
} from 'lucide-react'

export interface V2NavItem {
  href: string
  label: string
  icon: LucideIcon
}

/** Primary navigation — shared by the desktop rail and the mobile tab bar.
 *  Six destinations cover the receptionist's whole day; everything else
 *  (housekeeping, reports, admin…) lives one tap deeper under "More". */
export const V2_NAV: V2NavItem[] = [
  { href: '/v2', label: 'หน้าหลัก', icon: LayoutDashboard },
  { href: '/v2/rooms', label: 'ห้องพัก', icon: BedDouble },
  { href: '/v2/reservations', label: 'การจอง', icon: CalendarCheck },
  { href: '/v2/guests', label: 'ลูกค้า', icon: Users },
  { href: '/v2/registration', label: 'ทะเบียน', icon: ClipboardList },
  { href: '/v2/more', label: 'เพิ่มเติม', icon: LayoutGrid },
]

export function isV2Active(pathname: string, href: string): boolean {
  if (href === '/v2') return pathname === '/v2'
  return pathname === href || pathname.startsWith(href + '/')
}
