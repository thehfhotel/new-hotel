// ============================================================
// Booking Status
// ============================================================

export const BOOKING_STATUS_COLORS: Record<string, string> = {
  pending: 'bg-amber-500/10 text-amber-400',
  confirmed: 'bg-sky-500/10 text-sky-400',
  checkedin: 'bg-emerald-500/10 text-emerald-400',
  completed: 'bg-zinc-700 text-zinc-300',
  cancelled: 'bg-red-500/10 text-red-400',
  noshow: 'bg-orange-500/10 text-orange-400',
}

export const BOOKING_STATUS_LABELS: Record<string, string> = {
  pending: 'รอยืนยัน',
  confirmed: 'ยืนยันแล้ว',
  checkedin: 'เช็คอินแล้ว',
  completed: 'เสร็จสิ้น',
  cancelled: 'ยกเลิก',
  noshow: 'ไม่มาตามนัด',
}

export const BOOKING_STATUS_OPTIONS = [
  { value: '', label: 'สถานะทั้งหมด' },
  { value: 'pending', label: 'รอยืนยัน' },
  { value: 'confirmed', label: 'ยืนยันแล้ว' },
  { value: 'checkedin', label: 'เช็คอินแล้ว' },
  { value: 'completed', label: 'เสร็จสิ้น' },
  { value: 'cancelled', label: 'ยกเลิก' },
  { value: 'noshow', label: 'ไม่มาตามนัด' },
] as const

export const BOOKING_SOURCE_OPTIONS = [
  { value: 'walk-in', label: 'Walk-in' },
  { value: 'phone', label: 'โทรศัพท์' },
  { value: 'online', label: 'Online' },
  { value: 'ota', label: 'OTA' },
] as const

// ============================================================
// Room Status
// ============================================================

export const ROOM_STATUS_COLORS: Record<string, string> = {
  available: 'bg-emerald-500/10 text-emerald-400',
  occupied: 'bg-sky-500/10 text-sky-400',
  booked: 'bg-amber-500/10 text-amber-400',
  maintenance: 'bg-red-500/10 text-red-400',
  checkout_pending: 'bg-orange-500/10 text-orange-400',
}

export const ROOM_STATUS_LABELS: Record<string, string> = {
  available: 'ว่าง',
  occupied: 'เข้าพัก',
  booked: 'จอง',
  maintenance: 'ซ่อมบำรุง',
  checkout_pending: 'รอเช็คเอาท์',
}

// ============================================================
// Housekeeping Status
// ============================================================

export const HOUSEKEEPING_STATUS_COLORS: Record<string, string> = {
  dirty: 'bg-red-500/10 text-red-400',
  cleaning: 'bg-amber-500/10 text-amber-400',
  available: 'bg-emerald-500/10 text-emerald-400',
}

export const HOUSEKEEPING_STATUS_LABELS: Record<string, string> = {
  dirty: 'สกปรก',
  cleaning: 'กำลังทำความสะอาด',
  available: 'พร้อมใช้งาน',
}

// ============================================================
// Maintenance Status & Priority
// ============================================================

export const MAINTENANCE_STATUS_COLORS: Record<string, string> = {
  open: 'bg-red-500/10 text-red-400',
  in_progress: 'bg-amber-500/10 text-amber-400',
  completed: 'bg-emerald-500/10 text-emerald-400',
  cancelled: 'bg-zinc-700 text-zinc-400',
}

export const MAINTENANCE_STATUS_LABELS: Record<string, string> = {
  open: 'รอดำเนินการ',
  in_progress: 'กำลังดำเนินการ',
  completed: 'เสร็จสิ้น',
  cancelled: 'ยกเลิก',
}

export const MAINTENANCE_PRIORITY_COLORS: Record<number, string> = {
  1: 'bg-emerald-500/10 text-emerald-400',
  2: 'bg-amber-500/10 text-amber-400',
  3: 'bg-red-500/10 text-red-400',
}

export const MAINTENANCE_PRIORITY_LABELS: Record<number, string> = {
  1: 'ต่ำ',
  2: 'ปานกลาง',
  3: 'สูง',
}

// ============================================================
// Payment
// ============================================================

export const PAYMENT_METHOD_LABELS: Record<string, string> = {
  cash: 'เงินสด / Cash',
  credit: 'บัตรเครดิต / Credit Card',
  transfer: 'โอนเงิน / Bank Transfer',
  qr: 'QR Code / PromptPay',
}

export const PAYMENT_METHOD_OPTIONS = [
  { value: 'cash', label: 'เงินสด' },
  { value: 'credit', label: 'บัตรเครดิต' },
  { value: 'transfer', label: 'โอนเงิน' },
  { value: 'qr', label: 'QR Code' },
] as const

// ============================================================
// Helpers
// ============================================================

/** Get status color class from a color map, with fallback */
export function getStatusColor(statusMap: Record<string, string>, status: string, fallback = 'bg-zinc-700 text-zinc-400'): string {
  return statusMap[status] ?? fallback
}

/** Get status label from a label map, with fallback */
export function getStatusLabel(labelMap: Record<string, string>, status: string, fallback = status): string {
  return labelMap[status] ?? fallback
}
