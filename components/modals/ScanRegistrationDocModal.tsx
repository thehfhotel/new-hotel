'use client'

import { useEffect, useState } from 'react'
import { CreditCard, BookUser, X, RefreshCw, Check, AlertCircle, Upload } from 'lucide-react'
import { useBranchFetch } from '@/lib/use-branch-fetch'

// The Thai-ID smart-card reader is a per-PC middleware service (localhost),
// NOT our API — so it's a raw fetch, not branchFetch.
const MIDDLEWARE_URL = process.env.NEXT_PUBLIC_CARD_READER_URL || 'http://localhost:9898'

interface ThaiIdCardData {
  cid: string
  thaiTitle: string
  thaiFirstName: string
  thaiLastName: string
  englishTitle: string
  englishFirstName: string
  englishLastName: string
  dateOfBirth: string
  gender: string
  address: string
  issueDate: string
  expireDate: string
  photo?: string
}

/** Downscale a picked/pasted passport image to keep the stored blob reasonable.
 *  Mirrors app/passport-scanner/page.tsx. */
function downscaleImage(file: File): Promise<{ raw: string; dataUrl: string }> {
  return new Promise((resolve, reject) => {
    const reader = new FileReader()
    reader.onerror = () => reject(new Error('อ่านไฟล์รูปไม่สำเร็จ'))
    reader.onload = () => {
      const img = new Image()
      img.onerror = () => reject(new Error('รูปภาพไม่ถูกต้อง'))
      img.onload = () => {
        const MAX = 1200
        let { width, height } = img
        if (width > MAX || height > MAX) {
          const scale = Math.min(MAX / width, MAX / height)
          width = Math.round(width * scale)
          height = Math.round(height * scale)
        }
        const canvas = document.createElement('canvas')
        canvas.width = width
        canvas.height = height
        const ctx = canvas.getContext('2d')
        if (!ctx) {
          reject(new Error('ไม่สามารถประมวลผลรูปได้'))
          return
        }
        ctx.drawImage(img, 0, 0, width, height)
        const dataUrl = canvas.toDataURL('image/jpeg', 0.85)
        resolve({ raw: dataUrl.replace(/^data:image\/jpeg;base64,/, ''), dataUrl })
      }
      img.src = reader.result as string
    }
    reader.readAsDataURL(file)
  })
}

/**
 * "Scan ID now" for the registration reprint page. For a stay iHOTEL never
 * captured a card for, reception can read the guest's Thai national-ID smart
 * card (chip → backend renders the card) or attach a passport image — both
 * stored against THIS check-in (`cinId`) so they print on the registration form.
 * Reuses the render-thai-id + guest-documents endpoints (both accept `cinId`).
 */
export default function ScanRegistrationDocModal({
  checkInId,
  onClose,
  onCaptured,
}: {
  checkInId: number
  onClose: () => void
  onCaptured: () => void
}) {
  const branchFetch = useBranchFetch()
  const [busy, setBusy] = useState(false)
  const [error, setError] = useState<string | null>(null)
  const [passportImg, setPassportImg] = useState<{ raw: string; dataUrl: string } | null>(null)

  // Thai ID: read the chip via the local middleware, then have the backend
  // render + store the card image linked to this check-in.
  const readThaiId = async () => {
    setBusy(true)
    setError(null)
    try {
      const res = await fetch(`${MIDDLEWARE_URL}/read?photo=true`, {
        method: 'GET',
        signal: AbortSignal.timeout(30000),
      })
      if (!res.ok) throw new Error('อ่านบัตรไม่สำเร็จ — ตรวจสอบเครื่องอ่านบัตรและบัตรในเครื่อง')
      const body = await res.json()
      const card: ThaiIdCardData = body.data ?? body
      const r = await branchFetch('/api/guest-documents/render-thai-id', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({
          cid: card.cid,
          thaiTitle: card.thaiTitle,
          thaiFirstName: card.thaiFirstName,
          thaiLastName: card.thaiLastName,
          englishTitle: card.englishTitle,
          englishFirstName: card.englishFirstName,
          englishLastName: card.englishLastName,
          dateOfBirth: card.dateOfBirth,
          issueDate: card.issueDate,
          address: card.address,
          photoBase64: card.photo,
          cinId: checkInId,
        }),
      })
      const data = await r.json().catch(() => ({}))
      if (!r.ok || !data.success) {
        throw new Error(
          data.reason === 'render_unavailable'
            ? 'ระบบสร้างรูปบัตรยังไม่พร้อม (เทมเพลต/ฟอนต์บนเซิร์ฟเวอร์)'
            : 'บันทึกรูปบัตรไม่สำเร็จ'
        )
      }
      onCaptured()
      onClose()
    } catch (e) {
      setError(
        e instanceof Error
          ? e.name === 'TimeoutError'
            ? 'หมดเวลาอ่านบัตร'
            : e.message
          : 'อ่านบัตรไม่สำเร็จ'
      )
    } finally {
      setBusy(false)
    }
  }

  const pickImage = async (file?: File | null) => {
    if (!file) return
    setError(null)
    try {
      setPassportImg(await downscaleImage(file))
    } catch (e) {
      setError(e instanceof Error ? e.message : 'อ่านรูปไม่สำเร็จ')
    }
  }

  const attachPassport = async () => {
    if (!passportImg) return
    setBusy(true)
    setError(null)
    try {
      const r = await branchFetch('/api/guest-documents', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({
          docType: 'passport',
          mime: 'image/jpeg',
          imageBase64: passportImg.raw,
          source: 'scanner',
          cinId: checkInId,
        }),
      })
      const data = await r.json().catch(() => ({}))
      if (!r.ok || !data.success) throw new Error('บันทึกรูปพาสปอร์ตไม่สำเร็จ')
      onCaptured()
      onClose()
    } catch (e) {
      setError(e instanceof Error ? e.message : 'บันทึกไม่สำเร็จ')
    } finally {
      setBusy(false)
    }
  }

  // Let reception paste a scanned passport image straight from the clipboard.
  useEffect(() => {
    const onPaste = (e: ClipboardEvent) => {
      const item = Array.from(e.clipboardData?.items || []).find((i) => i.type.startsWith('image/'))
      const file = item?.getAsFile()
      if (file) pickImage(file)
    }
    window.addEventListener('paste', onPaste)
    return () => window.removeEventListener('paste', onPaste)
  }, [])

  return (
    <div
      className="fixed inset-0 z-50 flex items-center justify-center bg-black/40 p-4"
      onClick={onClose}
    >
      <div className="v2-card w-full max-w-md p-5 space-y-4" onClick={(e) => e.stopPropagation()}>
        <div className="flex items-center justify-between">
          <div className="font-semibold text-[16px]">สแกนบัตร/พาสปอร์ต · Scan ID / Passport</div>
          <button onClick={onClose} className="v2-btn v2-btn-ghost p-1" aria-label="ปิด">
            <X size={16} />
          </button>
        </div>

        {error && (
          <div
            className="v2-inset px-3 py-2 flex items-center gap-2 text-[13px]"
            style={{ color: 'var(--v2-wine-600)' }}
          >
            <AlertCircle size={15} /> {error}
          </div>
        )}

        {/* Thai national ID (smart-card reader) */}
        <div className="space-y-1.5">
          <button
            onClick={readThaiId}
            disabled={busy}
            className="v2-btn v2-btn-primary w-full inline-flex items-center justify-center gap-2"
          >
            {busy ? <RefreshCw size={16} className="animate-spin" /> : <CreditCard size={16} />}{' '}
            อ่านบัตรประชาชน (เครื่องอ่านบัตร)
          </button>
          <div className="text-[11px]" style={{ color: 'var(--v2-ink-3)' }}>
            เสียบบัตรในเครื่องอ่าน แล้วกดปุ่มด้านบน — ระบบจะสร้างรูปบัตรและแนบเข้ากับใบลงทะเบียนนี้
          </div>
        </div>

        <div className="h-px" style={{ background: 'var(--v2-line)' }} />

        {/* Passport (scan / photo of the passport page) */}
        <div className="space-y-2">
          <div className="flex items-center gap-1.5 text-[13px] font-medium">
            <BookUser size={15} /> พาสปอร์ต (สแกน/ถ่ายรูปหน้าพาสปอร์ต)
          </div>
          <label className="v2-btn v2-btn-ghost w-full inline-flex items-center justify-center gap-2 cursor-pointer">
            <Upload size={15} /> เลือกรูปพาสปอร์ต หรือวาง (paste)
            <input
              type="file"
              accept="image/*"
              className="hidden"
              onChange={(e) => pickImage(e.target.files?.[0])}
            />
          </label>
          {passportImg && (
            <>
              {/* eslint-disable-next-line @next/next/no-img-element */}
              <img
                src={passportImg.dataUrl}
                alt="passport"
                className="w-full rounded-md border object-contain max-h-48"
                style={{ borderColor: 'var(--v2-line)' }}
              />
              <button
                onClick={attachPassport}
                disabled={busy}
                className="v2-btn v2-btn-primary w-full inline-flex items-center justify-center gap-2"
              >
                {busy ? <RefreshCw size={16} className="animate-spin" /> : <Check size={16} />}{' '}
                แนบพาสปอร์ตเข้าใบลงทะเบียน
              </button>
            </>
          )}
        </div>
      </div>
    </div>
  )
}
