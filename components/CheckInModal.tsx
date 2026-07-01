'use client'

import { useState, useEffect, useCallback } from 'react'
import { X, User, Calendar, AlertCircle, Loader2, DollarSign, CheckCircle2 } from 'lucide-react'
import { useBranchFetch } from '@/lib/use-branch-fetch'
import { useBranch } from '@/contexts/BranchContext'
import { consumeCheckInPrefill, type CheckInPrefill } from '@/lib/checkin-prefill'
import { hotelInfoForBranch } from '@/lib/hotel-info'
import PrintButton from '@/components/ui/PrintButton'
import RegistrationSlipTemplate, {
  RegistrationSlipData,
} from '@/components/documents/RegistrationSlipTemplate'

/**
 * Minimal walk-in check-in modal.
 *
 * Flow:
 *   1. Receptionist enters customer name + phone (+ optional ID).
 *   2. Picks expected check-out date (defaults to tomorrow).
 *   3. Submit: POSTs a new customer (skipping if a matching existing
 *      customer is selected from the type-ahead), then POSTs the check-in
 *      against this room.
 *
 * The check-in fires the `walkin` writeback which mirrors the row to legacy
 * MSSQL (`HT_CheckIn_H` + `HT_CheckIn_Ds` + `HT_POWER_LOG` + `HT_Cupon` etc).
 */

interface RoomLite {
  id: number
  roomNo: string
  roomTypeName?: string | null
  rate?: number | null
}

interface ExistingCustomer {
  id: number
  firstName: string
  lastName?: string | null
  phone?: string | null
}

interface CheckInModalProps {
  room: RoomLite
  onClose: () => void
  onSuccess: () => void
}

function tomorrowYmd(): string {
  const d = new Date()
  d.setDate(d.getDate() + 1)
  return d.toISOString().slice(0, 10)
}

/** Nights from today until an ISO `YYYY-MM-DD` checkout date (min 1). */
function nightsUntil(checkoutYmd: string): number {
  const out = new Date(checkoutYmd)
  if (Number.isNaN(out.getTime())) return 1
  const today = new Date()
  const ms = out.getTime() - today.getTime()
  return Math.max(1, Math.round(ms / 86_400_000))
}

export default function CheckInModal({ room, onClose, onSuccess }: CheckInModalProps) {
  const branchFetch = useBranchFetch()
  const { branch } = useBranch()
  const [firstName, setFirstName] = useState('')
  const [lastName, setLastName] = useState('')
  const [phone, setPhone] = useState('')
  const [idCard, setIdCard] = useState('')
  const [deposit, setDeposit] = useState('')
  const [adults, setAdults] = useState(1)
  const [children, setChildren] = useState(0)
  const [expectedCheckout, setExpectedCheckout] = useState(tomorrowYmd())
  const [submitting, setSubmitting] = useState(false)
  const [error, setError] = useState<string | null>(null)
  // After a successful check-in we switch the modal to a "print the
  // registration slip" panel instead of closing immediately.
  const [created, setCreated] = useState<{ cinNo: string; id: number } | null>(null)
  // Branch-aware URL of the guest's captured ID/passport photo, resolved from
  // the registration-slip endpoint once the check-in exists. Stays undefined
  // when there is no photo — the slip then renders exactly as before.
  const [photoUrl, setPhotoUrl] = useState<string | undefined>(undefined)
  // The full document prefill (card reader / passport scanner). Held so the
  // richer customer fields + photo flow through submit; null for a plain
  // walk-in (behaviour then identical to before this feature).
  const [prefill, setPrefill] = useState<CheckInPrefill | null>(null)

  // ID-card / passport prefill hand-off: the reader/scanner stashes the parsed
  // document fields and routes here; consume them once on open.
  useEffect(() => {
    const p = consumeCheckInPrefill()
    if (!p) return
    setPrefill(p)
    if (p.firstName) setFirstName(p.firstName)
    if (p.lastName) setLastName(p.lastName)
    // The visible field is labelled "เลขบัตรประชาชน / Passport": show the Thai
    // national id, or the passport number for a foreign guest.
    const docNo = p.idCard || p.passport
    if (docNo) setIdCard(docNo)
  }, [])

  // Type-ahead lookup for existing customers by phone — avoids creating a
  // duplicate customer for repeat guests. If a match is picked, we skip the
  // POST /api/customers step.
  const [phoneMatches, setPhoneMatches] = useState<ExistingCustomer[]>([])
  const [pickedCustomerId, setPickedCustomerId] = useState<number | null>(null)

  const lookupByPhone = useCallback(async (q: string) => {
    if (q.length < 4) {
      setPhoneMatches([])
      return
    }
    try {
      const res = await branchFetch(`/api/customers/search?search=${encodeURIComponent(q)}&limit=5`)
      if (!res.ok) return
      const data = await res.json()
      setPhoneMatches(data.data || [])
    } catch {
      // Silent — non-blocking lookup
    }
  }, [branchFetch])

  // Debounce phone lookups so we don't spam the API on every keystroke.
  useEffect(() => {
    if (pickedCustomerId !== null) return
    const t = setTimeout(() => lookupByPhone(phone), 300)
    return () => clearTimeout(t)
  }, [phone, pickedCustomerId, lookupByPhone])

  const pickExisting = (c: ExistingCustomer) => {
    setPickedCustomerId(c.id)
    setFirstName(c.firstName)
    setLastName(c.lastName || '')
    setPhone(c.phone || phone)
    setPhoneMatches([])
  }

  const clearPicked = () => {
    setPickedCustomerId(null)
  }

  // Build the POST /api/customers body. With no document prefill this is
  // byte-identical to the original walk-in body (firstName/lastName/phone/
  // idCard); a scanned Thai-ID card or passport adds the extended, only-when-
  // present fields (undefined values are dropped by JSON.stringify, so a blank
  // field never overwrites an existing value — the backend enriches with
  // COALESCE).
  const buildCustomerBody = (): Record<string, unknown> => {
    const isPassport = prefill?.docType === 'passport'
    const body: Record<string, unknown> = {
      firstName: firstName.trim(),
      lastName: lastName.trim() || undefined,
      phone: phone.trim() || undefined,
    }
    if (isPassport) {
      // Foreign guest: the visible number field carries the passport number.
      body.passport = idCard.trim() || prefill?.passport || undefined
    } else {
      body.idCard = idCard.trim() || undefined
      if (prefill?.passport) body.passport = prefill.passport
    }
    if (prefill) {
      const englishName = [prefill.englishFirstName, prefill.englishLastName]
        .filter(Boolean)
        .join(' ')
        .trim()
      Object.assign(body, {
        title: prefill.title,
        englishName: englishName || undefined,
        nationality: prefill.nationality,
        sex: prefill.sex,
        dob: prefill.dob,
        address: prefill.address,
        addNo: prefill.addNo,
        addMoo: prefill.addMoo,
        addSoi: prefill.addSoi,
        addRoad: prefill.addRoad,
        addTambon: prefill.addTambon,
        addAmpore: prefill.addAmpore,
        addProvince: prefill.addProvince,
        addCode: prefill.addCode,
      })
    }
    return body
  }

  const submit = async (e: React.FormEvent) => {
    e.preventDefault()
    setError(null)

    if (!firstName.trim()) {
      setError('กรุณากรอกชื่อลูกค้า')
      return
    }

    setSubmitting(true)
    try {
      // Step 1: ensure we have a customer_id. Reuse picked existing, or
      // POST a new walk-in customer (enriched with any scanned document fields).
      let customerId = pickedCustomerId
      if (customerId === null) {
        const custRes = await branchFetch('/api/customers', {
          method: 'POST',
          headers: { 'Content-Type': 'application/json' },
          body: JSON.stringify(buildCustomerBody()),
        })
        const custData = await custRes.json()
        if (!custRes.ok || !custData.success || !custData.id) {
          throw new Error(custData.message || 'สร้างข้อมูลลูกค้าไม่สำเร็จ')
        }
        customerId = custData.id
      }

      // Step 1b (optional): persist the scanned document image, then link it to
      // the check-in via its provisional tmpNo. Best-effort — a photo is a
      // convenience and must never block the check-in.
      let photoTmpNo: string | undefined
      if (prefill?.docTmpNo) {
        // The card reader already rendered AND stored the full Thai-ID card
        // server-side; link that provisional doc straight through by its
        // tmp_no. No image re-upload — the check-in linkage backfills
        // ht_guest_documents.doc_cin_id by doc_legacy_tmp_no.
        photoTmpNo = prefill.docTmpNo
      } else if (prefill?.photoBase64) {
        try {
          const docType = prefill.docType ?? 'thai_id_card'
          const docRes = await branchFetch('/api/guest-documents', {
            method: 'POST',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify({
              docType,
              mime: 'image/jpeg',
              imageBase64: prefill.photoBase64,
              source: docType === 'thai_id_card' ? 'chip' : 'scanner',
            }),
          })
          const docData = await docRes.json().catch(() => ({}))
          if (docRes.ok && docData.tmpNo) photoTmpNo = docData.tmpNo
        } catch {
          // Swallow — proceed with the check-in without the photo link.
        }
      }

      // Step 2: create the check-in.
      const depositAmount = parseFloat(deposit)
      const checkinRes = await branchFetch('/api/checkins', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({
          customerId,
          roomId: room.id,
          expectedCheckout,
          adults,
          children,
          depositAmount: Number.isFinite(depositAmount) && depositAmount > 0 ? depositAmount : undefined,
          photoTmpNo,
        }),
      })
      const checkinData = await checkinRes.json()
      if (!checkinRes.ok || !checkinData.success) {
        throw new Error(checkinData.message || 'เช็คอินไม่สำเร็จ')
      }

      // Refresh the room grid behind the modal, then offer the registration
      // slip print. The modal stays open (showing the success panel) until
      // the receptionist clicks "เสร็จสิ้น".
      onSuccess()
      setCreated({ cinNo: checkinData.cinNo || '', id: checkinData.id })

      // Best-effort: resolve the captured ID/passport photo so it prints on the
      // immediate slip (mirrors the /v2/registration/[id] reprint page). The
      // registration-slip response is camelCase. A failure here must never block
      // the slip — proceed without a photo on any error.
      try {
        const slipRes = await branchFetch(`/api/checkins/${checkinData.id}/registration-slip`)
        const slip = await slipRes.json()
        if (slip?.guestPhotoDocId) {
          setPhotoUrl(`/api/guest-documents/${slip.guestPhotoDocId}?branch=${encodeURIComponent(branch)}`)
        }
      } catch {
        // Swallow — the slip renders without a photo.
      }
    } catch (err) {
      setError(err instanceof Error ? err.message : 'เกิดข้อผิดพลาด')
    } finally {
      setSubmitting(false)
    }
  }

  return (
    <div
      className="fixed inset-0 bg-black/50 flex items-center justify-center z-50 p-4"
      onClick={onClose}
    >
      <div
        className="bg-white rounded-lg shadow-xl w-full max-w-md max-h-[90vh] overflow-y-auto"
        onClick={(e) => e.stopPropagation()}
      >
        <div className="flex items-center justify-between p-4 border-b border-gray-200">
          <h2 className="text-lg font-bold text-gray-900">
            เช็คอิน ห้อง {room.roomNo}
          </h2>
          <button
            onClick={onClose}
            className="p-1 hover:bg-gray-100 rounded"
            aria-label="Close"
          >
            <X size={20} />
          </button>
        </div>

        {created ? (
          <div className="p-4 space-y-4">
            <div className="flex items-start p-3 bg-emerald-50 border border-emerald-200 rounded text-sm text-emerald-800">
              <CheckCircle2 size={18} className="mr-2 shrink-0 mt-0.5" />
              <div>
                <p className="font-medium">เช็คอินสำเร็จ</p>
                {created.cinNo && <p className="text-xs mt-0.5">เลขที่: {created.cinNo}</p>}
              </div>
            </div>

            <p className="text-sm text-gray-600">
              พิมพ์ใบลงทะเบียนเข้าพักให้ผู้เข้าพัก หรือกด &quot;เสร็จสิ้น&quot; เพื่อปิด
            </p>

            <RegistrationSlipTemplate
              hotelInfo={hotelInfoForBranch(branch)}
              data={
                {
                  registrationNo: created.cinNo,
                  checkInId: created.id,
                  guestName: `${firstName} ${lastName}`.trim(),
                  guestIdCard: idCard.trim() || undefined,
                  guestContact: phone.trim() || undefined,
                  roomNumber: room.roomNo,
                  roomType: room.roomTypeName || undefined,
                  checkInDate: new Date().toISOString(),
                  checkOutDate: expectedCheckout,
                  nights: nightsUntil(expectedCheckout),
                  ratePerNight: room.rate ?? undefined,
                  deposit: parseFloat(deposit) > 0 ? parseFloat(deposit) : undefined,
                  adults,
                  children,
                  guestPhotoUrl: photoUrl,
                } satisfies RegistrationSlipData
              }
            />

            <div className="flex justify-end gap-2 pt-2 border-t border-gray-200">
              <PrintButton size="sm" showPdfOption={false} />
              <button
                type="button"
                onClick={onClose}
                className="px-4 py-2 text-sm font-medium text-gray-700 bg-white border border-gray-300 rounded hover:bg-gray-50"
              >
                เสร็จสิ้น
              </button>
            </div>
          </div>
        ) : (
        <form onSubmit={submit} className="p-4 space-y-4">
          {/* Picked-existing pill */}
          {pickedCustomerId !== null && (
            <div className="flex items-center justify-between p-2 bg-emerald-50 border border-emerald-200 rounded">
              <div className="flex items-center text-sm text-emerald-800">
                <User size={14} className="mr-1.5" />
                ลูกค้าเดิม #{pickedCustomerId}: {firstName} {lastName}
              </div>
              <button
                type="button"
                onClick={clearPicked}
                className="text-xs text-emerald-700 underline"
              >
                ใช้ลูกค้าใหม่แทน
              </button>
            </div>
          )}

          <div>
            <label className="block text-sm font-medium text-gray-700 mb-1">
              ชื่อ *
            </label>
            <input
              type="text"
              value={firstName}
              onChange={(e) => { setFirstName(e.target.value); clearPicked() }}
              required
              className="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-hidden focus:ring-2 focus:ring-red-500"
            />
          </div>

          <div>
            <label className="block text-sm font-medium text-gray-700 mb-1">
              นามสกุล
            </label>
            <input
              type="text"
              value={lastName}
              onChange={(e) => { setLastName(e.target.value); clearPicked() }}
              className="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-hidden focus:ring-2 focus:ring-red-500"
            />
          </div>

          <div className="relative">
            <label className="block text-sm font-medium text-gray-700 mb-1">
              เบอร์โทร
            </label>
            <input
              type="tel"
              value={phone}
              onChange={(e) => { setPhone(e.target.value); clearPicked() }}
              placeholder="เช่น 08REDACTED-sa-pw"
              className="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-hidden focus:ring-2 focus:ring-red-500"
            />
            {phoneMatches.length > 0 && pickedCustomerId === null && (
              <ul className="absolute z-10 left-0 right-0 mt-1 bg-white border border-gray-300 rounded shadow-lg max-h-40 overflow-y-auto">
                {phoneMatches.map((c) => (
                  <li
                    key={c.id}
                    onClick={() => pickExisting(c)}
                    className="px-3 py-2 text-sm hover:bg-gray-100 cursor-pointer"
                  >
                    <div className="font-medium">{c.firstName} {c.lastName || ''}</div>
                    <div className="text-xs text-gray-500">{c.phone || '(no phone)'}</div>
                  </li>
                ))}
              </ul>
            )}
          </div>

          <div>
            <label className="block text-sm font-medium text-gray-700 mb-1">
              เลขบัตรประชาชน / Passport
            </label>
            <input
              type="text"
              value={idCard}
              onChange={(e) => setIdCard(e.target.value)}
              className="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-hidden focus:ring-2 focus:ring-red-500"
            />
          </div>

          <div className="grid grid-cols-2 gap-3">
            <div>
              <label className="block text-sm font-medium text-gray-700 mb-1">
                ผู้ใหญ่
              </label>
              <input
                type="number"
                min={1}
                value={adults}
                onChange={(e) => setAdults(Math.max(1, parseInt(e.target.value) || 1))}
                className="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-hidden focus:ring-2 focus:ring-red-500"
              />
            </div>
            <div>
              <label className="block text-sm font-medium text-gray-700 mb-1">
                เด็ก
              </label>
              <input
                type="number"
                min={0}
                value={children}
                onChange={(e) => setChildren(Math.max(0, parseInt(e.target.value) || 0))}
                className="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-hidden focus:ring-2 focus:ring-red-500"
              />
            </div>
          </div>

          <div>
            <label className="block text-sm font-medium text-gray-700 mb-1">
              <Calendar size={14} className="inline mr-1" />
              วันออก
            </label>
            <input
              type="date"
              value={expectedCheckout}
              onChange={(e) => setExpectedCheckout(e.target.value)}
              required
              className="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-hidden focus:ring-2 focus:ring-red-500"
            />
          </div>

          <div>
            <label className="block text-sm font-medium text-gray-700 mb-1">
              <DollarSign size={14} className="inline mr-1" />
              เงินมัดจำ (บาท)
            </label>
            <input
              type="number"
              min={0}
              step="0.01"
              value={deposit}
              onChange={(e) => setDeposit(e.target.value)}
              placeholder="0"
              className="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-hidden focus:ring-2 focus:ring-red-500"
            />
          </div>

          {error && (
            <div className="flex items-start p-3 bg-red-50 border border-red-200 rounded text-sm text-red-700">
              <AlertCircle size={16} className="mr-2 shrink-0 mt-0.5" />
              {error}
            </div>
          )}

          <div className="flex justify-end gap-2 pt-2 border-t border-gray-200">
            <button
              type="button"
              onClick={onClose}
              disabled={submitting}
              className="px-4 py-2 text-sm font-medium text-gray-700 bg-white border border-gray-300 rounded hover:bg-gray-50 disabled:opacity-50"
            >
              ยกเลิก
            </button>
            <button
              type="submit"
              disabled={submitting || !firstName.trim()}
              className="px-4 py-2 text-sm font-medium text-white bg-red-600 rounded hover:bg-red-700 disabled:opacity-50 flex items-center"
            >
              {submitting && <Loader2 size={14} className="mr-2 animate-spin" />}
              เช็คอิน
            </button>
          </div>
        </form>
        )}
      </div>
    </div>
  )
}
