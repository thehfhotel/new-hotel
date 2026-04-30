import { Branch } from '@/contexts/BranchContext'
import type { HotelInfo } from '@/types/invoice'

export type { HotelInfo }

/**
 * Branch-scoped hotel identity used on customer-facing documents
 * (invoices, receipts). Centralised here so that legal-entity data
 * is updated in exactly one place.
 *
 * NOTE: Do NOT inline this data into templates. The wrong hotel
 * name on a printed invoice is a customer-facing defect.
 */
export const HOTEL_INFO_BY_BRANCH: Record<Exclude<Branch, 'all'>, HotelInfo> = {
  hfhotel: {
    name: 'The HF Hotel',
    address: '123 ถนนตัวอย่าง อ.เมือง จ.กรุงเทพฯ 10000',
    phone: '02-123-4567',
    taxId: '0123456789012',
  },
  hfville: {
    // TODO(operator): replace these placeholders with the real HF Ville
    // legal-entity name + address + phone + tax ID before Ville guests
    // receive their first invoice. Until then, invoices show "[HF Ville
    // — info pending]" so the gap is visible rather than masked.
    name: '[HF Ville — info pending]',
    address: '[address pending — fill in lib/hotel-info.ts]',
    phone: '[phone pending]',
    taxId: '[tax id pending]',
  },
}

/**
 * Resolve hotel info for the active branch. The 'all' branch is a
 * dashboard view and never prints invoices — fall back to hfhotel
 * defensively if somehow reached.
 */
export function hotelInfoForBranch(branch: Branch): HotelInfo {
  if (branch === 'all') return HOTEL_INFO_BY_BRANCH.hfhotel
  return HOTEL_INFO_BY_BRANCH[branch]
}
