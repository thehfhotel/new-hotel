// Report HK vocabulary — the ONE place the daily room report's fixed lists are
// spelled (owner's Report HK.xlsx, digitized 2026-09-02). Imported by the maid
// /hk surface and the reception verify view; mirrored by the backend
// allowlists (hotel-backend/src/routes/hk.rs — keep in lock-step).
//
// Canned-only discipline carries over from room signals: no free text anywhere
// in the report flow; the return reasons below are the whole rejection
// vocabulary.

/** The in-room equipment checklist (อุปกรณ์ภายในห้อง), in the paper form's
 *  order. Items that ARE linen reuse the exact `LINEN_KINDS` codes so an item
 *  exception and a ขาดผ้า report name the same thing. Labels use canonical
 *  spelling (the sheet's ผ้าเช้ดเท้า / รีโมโทรทัศน์ typos corrected). */
export const REPORT_ITEMS = [
  { item: 'water_glass', label: 'แก้วน้ำ' },
  { item: 'coffee_tray', label: 'ถาดรองแก้วกาแฟ' },
  { item: 'coffee_cup', label: 'แก้วกาแฟ' },
  { item: 'coffee_sachet_jar', label: 'แก้วใส่ซองกาแฟ' },
  { item: 'kettle', label: 'กาน้ำร้อน' },
  { item: 'bathroom_bin', label: 'ถังขยะในห้องน้ำ' },
  { item: 'hairdryer', label: 'ไดร์เป่าผม' },
  { item: 'bath_amenity_tray', label: 'ถาดไม้รองอุปกรณ์อาบน้ำ' },
  { item: 'aircon_remote', label: 'รีโมทแอร์' },
  { item: 'tv_remote', label: 'รีโมทโทรทัศน์' },
  { item: 'mirror_bin', label: 'ถังขยะหน้ากระจก' },
  { item: 'hangers', label: 'ไม้แขวนเสื้อ' },
  { item: 'bath_towel', label: 'ผ้าขนหนู (รวมสีฟ้า)' },
  { item: 'face_towel', label: 'ผ้าเช็ดหน้า' },
  { item: 'foot_towel', label: 'ผ้าเช็ดเท้า' },
  { item: 'duvet', label: 'ผ้านวม' },
  { item: 'bed_sheet', label: 'ผ้าปูที่นอน' },
  { item: 'pillowcase', label: 'ปลอกหมอน' },
  { item: 'duvet_cover', label: 'ซองนวม' },
  { item: 'pillow', label: 'หมอน' },
  { item: 'ashtray', label: 'ที่เขี่ยบุหรี่' },
  { item: 'bathrobe', label: 'ผ้าคลุมอาบน้ำสีน้ำเงิน' },
] as const

export type ReportItemCode = (typeof REPORT_ITEMS)[number]['item']

/** What can be wrong with an item — the same pair the signal vocabulary uses. */
export const ITEM_PROBLEMS = [
  { problem: 'missing', label: 'หาย' },
  { problem: 'damaged', label: 'ชำรุด' },
] as const

export type ItemProblem = (typeof ITEM_PROBLEMS)[number]['problem']

/** The paper form's room status legend (VC/CO/OO/SO), verbatim. */
export const ROOM_STATUS_CODES = [
  { code: 'vc', label: 'VC ห้องทำความสะอาดแล้ว' },
  { code: 'co', label: 'CO เช็คเอาท์' },
  { code: 'oo', label: 'OO รอซ่อม' },
  { code: 'so', label: 'SO พักต่อ' },
] as const

export type RoomStatusCode = (typeof ROOM_STATUS_CODES)[number]['code']

/** Report lifecycle. Append-only: a returned report is fixed by a NEW
 *  submission that references the old one, never by editing in place. */
export type ReportStatus = 'submitted' | 'verified' | 'returned'

/** The whole rejection vocabulary — canned, like everything else. */
export const RETURN_REASONS = [
  { reason: 'not_clean', label: 'ยังไม่สะอาด' },
  { reason: 'items_mismatch', label: 'อุปกรณ์ไม่ตรงกับที่รายงาน' },
  { reason: 'photos_unclear', label: 'รูปไม่ชัดเจน' },
] as const

export type ReturnReason = (typeof RETURN_REASONS)[number]['reason']

/** Photo evidence bounds — enforced client- AND server-side. */
export const REPORT_MIN_PHOTOS = 1
export const REPORT_MAX_PHOTOS = 4

const ITEM_LABELS: Record<string, string> = Object.fromEntries(
  REPORT_ITEMS.map(({ item, label }) => [item, label])
)

/** Thai label for an item code; the raw code for one this bundle predates. PURE. */
export function reportItemLabel(item: string): string {
  return ITEM_LABELS[item] ?? item
}
