'use client'

import { use, useCallback, useEffect, useState } from 'react'
import Link from 'next/link'
import { ArrowLeft, Printer, AlertCircle, ScanLine, Users } from 'lucide-react'
import { useBranch } from '@/contexts/BranchContext'
import { useBranchFetch } from '@/lib/use-branch-fetch'
import { hotelInfoForBranch } from '@/lib/hotel-info'
import RegistrationSlipTemplate, {
  type RegistrationSlipData,
} from '@/components/documents/RegistrationSlipTemplate'
import { V2PageHeader, V2Spinner } from '@/components/v2/primitives'
import ScanRegistrationDocModal from '@/components/modals/ScanRegistrationDocModal'
import GuestRegistryModal from '@/components/modals/GuestRegistryModal'

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
  const [showScan, setShowScan] = useState(false)
  const [showGuests, setShowGuests] = useState(false)

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

  // Branch-aware URL to the guest's captured ID/passport image, served at
  // /api/guest-documents/{docId}. Undefined when no photo was captured, so the
  // template and preview render nothing (no layout shift).
  const photoUrl = slip?.guestPhotoDocId
    ? `/api/guest-documents/${slip.guestPhotoDocId}?branch=${encodeURIComponent(branch)}`
    : undefined

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
            {/* On-screen preview so reception can review the ID/passport before
                printing. Also warms the image cache so the first print isn't
                blank. Renders nothing when no photo was captured. */}
            {photoUrl ? (
              <div className="pt-2">
                <div className="v2-eyebrow mb-1">รูปบัตร/พาสปอร์ต ID / PASSPORT</div>
                {/* eslint-disable-next-line @next/next/no-img-element */}
                <img
                  src={photoUrl}
                  alt="ID / Passport"
                  className="h-40 w-auto rounded-md border object-contain"
                  style={{ borderColor: 'var(--v2-line)' }}
                />
              </div>
            ) : (
              <div className="pt-1 text-[12.5px]" style={{ color: 'var(--v2-ink-3)' }}>
                ยังไม่มีรูปบัตรประชาชน/พาสปอร์ตสำหรับใบลงทะเบียนนี้ — สแกนเพื่อแนบเข้าเอกสาร
              </div>
            )}
            <div className="flex flex-wrap items-center gap-2 mt-3">
              <button
                onClick={() => setShowScan(true)}
                className={`v2-btn inline-flex items-center gap-2 ${
                  photoUrl ? 'v2-btn-ghost' : 'v2-btn-primary'
                }`}
              >
                <ScanLine size={16} /> {photoUrl ? 'สแกนใหม่' : 'สแกนบัตร/พาสปอร์ต'}
              </button>
              <button
                onClick={() => setShowGuests(true)}
                className="v2-btn v2-btn-ghost inline-flex items-center gap-2"
              >
                <Users size={16} /> ผู้เข้าพักร่วม
              </button>
              <button
                onClick={() => window.print()}
                className="v2-btn v2-btn-primary inline-flex items-center gap-2"
              >
                <Printer size={16} /> พิมพ์ใบลงทะเบียน
              </button>
            </div>
          </div>
          {/* Print-only portal — renders nothing on screen, everything on paper. */}
          <RegistrationSlipTemplate data={{ ...slip, guestPhotoUrl: photoUrl }} hotelInfo={hotelInfo} />

          {showScan && (
            <ScanRegistrationDocModal
              checkInId={Number(id)}
              onClose={() => setShowScan(false)}
              onCaptured={fetchSlip}
            />
          )}

          <GuestRegistryModal
            isOpen={showGuests}
            onClose={() => setShowGuests(false)}
            checkIn={{
              id: Number(id),
              checkInNo: slip.registrationNo || '',
              roomNumber: slip.roomNumber || '',
              customerName: slip.guestName || '',
            }}
          />
        </>
      ) : null}
    </div>
  )
}
