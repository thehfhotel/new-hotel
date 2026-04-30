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
    // Filled 2026-04-30 from legacy TB_SETTINGS (HF Hotel MSSQL).
    // Both sites operate under the same legal entity
    // (บริษัท สายชล เฮอริเทจ จำกัด), hence the shared taxId.
    name: 'HF Hotel',
    address: '33 ถนนชนเกษม ต.ตลาด อ.เมืองสุราษฎร์ธานี จ.สุราษฎร์ธานี 84000',
    phone: '077313808',
    taxId: '0845557000341',
  },
  hfville: {
    // Filled 2026-04-30 from operator. Verify before any printed invoice
    // goes to a real Ville guest.
    name: 'HF Ville',
    address: '196/6 หมู่ 5 ตำบลมะขามเตี้ย อำเภอเมืองสุราษฎร์ธานี จังหวัดสุราษฎร์ธานี 84000',
    phone: '077275838',
    taxId: '0845557000341',
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
