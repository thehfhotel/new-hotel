'use client'

import { use, useCallback, useEffect, useState } from 'react'
import Link from 'next/link'
import { ArrowLeft, Printer, AlertCircle } from 'lucide-react'
import { useBranch } from '@/contexts/BranchContext'
import { useBranchFetch } from '@/lib/use-branch-fetch'
import { hotelInfoForBranch } from '@/lib/hotel-info'
import RegistrationSlipTemplate, {
  type RegistrationSlipData,
} from '@/components/documents/RegistrationSlipTemplate'
import { V2PageHeader, V2Spinner } from '@/components/v2/primitives'

/**
 * v2 registration slip (ใบลงทะเบียนเข้าพัก) — #29 reprint path. iHOTEL exposes a
 * standalone reprint (FrmRegMain); our check-in modal only rendered the slip
 * once in its success panel, so reception had no way to reprint for an
 * iHOTEL-created or already-completed stay. This page fetches the slip for any
 * check-in id and reuses the canonical `RegistrationSlipTemplate` (a print-only
 * portal — invisible on screen, the sole node shown by `window.print()`), so
 * the on-screen card is just a launcher. Branch-aware via `useBranchFetch`;
 * seller identity from `hotelInfoForBranch`.
 */
export default function V2RegistrationSlipPage({
  params,
}: {
  params: Promise<{ id: string }>
}) {
  const { id } = use(params)
  const branchFetch = useBranchFetch()
  const { branch } = useBranch()
  const hotelInfo = hotelInfoForBranch(branch)

  const [slip, setSlip] = useState<RegistrationSlipData | null>(null)
  const [loading, setLoading] = useState(true)
  const [error, setError] = useState<string | null>(null)

  const fetchSlip = useCallback(async () => {
    setLoading(true)
    setError(null)
    try {
      const res = await branchFetch(`/api/checkins/${id}/registration-slip`)
      const data = await res.json()
      if (res.ok && data.success) {
        setSlip(data as RegistrationSlipData)
      } else {
        setError(data.error || 'ไม่พบข้อมูลใบลงทะเบียน')
      }
    } catch (err) {
      setError(err instanceof Error ? err.message : 'เกิดข้อผิดพลาดในการโหลดข้อมูล')
    } finally {
      setLoading(false)
    }
  }, [id, branchFetch])

  useEffect(() => {
    fetchSlip()
  }, [fetchSlip])

  if (loading) return <V2Spinner label="กำลังโหลดใบลงทะเบียน…" />

  return (
    <div className="space-y-5">
      <V2PageHeader
        eyebrow="พิมพ์เอกสาร"
        title="ใบลงทะเบียนเข้าพัก"
        right={
          <Link href="/v2/rosters" className="v2-btn v2-btn-ghost inline-flex items-center gap-1.5">
            <ArrowLeft size={15} /> รายชื่อผู้เข้าพัก
          </Link>
        }
      />

      {error ? (
        <div
          className="v2-inset px-4 py-3 flex items-center gap-2 text-[13px]"
          style={{ color: 'var(--v2-wine-600)' }}
        >
          <AlertCircle size={15} /> {error}
        </div>
      ) : slip ? (
        <>
          <div className="v2-card p-5 space-y-1.5 text-[14px]">
            <div className="v2-eyebrow">เลขที่ {slip.registrationNo}</div>
            <div className="font-semibold text-[17px]">{slip.guestName || 'ไม่ระบุชื่อ'}</div>
            <div style={{ color: 'var(--v2-ink-3)' }}>
              ห้อง {slip.roomNumber}
              {slip.roomType ? ` · ${slip.roomType}` : ''} · {slip.nights} คืน
            </div>
            <button
              onClick={() => window.print()}
              className="v2-btn v2-btn-primary mt-3 inline-flex items-center gap-2"
            >
              <Printer size={16} /> พิมพ์ใบลงทะเบียน
            </button>
          </div>
          {/* Print-only portal — renders nothing on screen, everything on paper. */}
          <RegistrationSlipTemplate data={slip} hotelInfo={hotelInfo} />
        </>
      ) : null}
    </div>
  )
}
