'use client'

/**
 * Passport (MRZ) scanner page.
 *
 * MRZ acquisition path — DELIBERATE CHOICE: manual paste of the two TD3 MRZ
 * lines into a textarea, NOT client-side OCR. In-browser OCR of an MRZ
 * (tesseract.js) is heavy (~2 MB WASM) and unreliable without careful image
 * pre-processing/deskew, which makes it a poor fit for a reception desk. Every
 * passport prints the MRZ as fixed-pitch OCR-B text; a receptionist reads/types
 * (or a hardware swipe fills) the two 44-char lines far more reliably. So we
 * keep the frontend dependency-free and let the middleware do the authoritative
 * TD3 parse + checksum validation via POST /parse-mrz.
 *
 * The passport image is optional and only used for the guest-document photo:
 * it's downscaled client-side (sessionStorage/base64 hand-off has a hard size
 * budget) and echoed by the middleware for the confirmation preview.
 */

import { useState, useCallback, useEffect, useRef } from 'react'
import { useRouter } from 'next/navigation'
import {
  BookUser,
  RefreshCw,
  CheckCircle2,
  XCircle,
  User,
  AlertCircle,
  Server,
  ScanLine,
  ImageUp,
} from 'lucide-react'
import { setCheckInPrefill } from '@/lib/checkin-prefill'

type ConnectionStatus = 'disconnected' | 'connecting' | 'connected'
type ParseStatus = 'idle' | 'parsing' | 'success' | 'error'

interface ParsedMrz {
  success: boolean
  passportNumber: string
  surname: string
  givenNames: string
  nationality: string
  /** Gregorian ISO `YYYY-MM-DD`. */
  dateOfBirth: string
  /** MRZ sex code: "M" | "F" | "X" / "<". */
  sex: string
  /** Gregorian ISO `YYYY-MM-DD`. */
  expiryDate: string
  checksumValid: boolean
  /** Base64 JPEG (no data: prefix) echoed back by the middleware. */
  image?: string
}

const MIDDLEWARE_URL = process.env.NEXT_PUBLIC_CARD_READER_URL || 'http://localhost:9898'

/** Map an MRZ sex code to the legacy Thai literal stored in `Cust_sex`. */
function mrzSexToThaiSex(sex: string | undefined): string | undefined {
  if (!sex) return undefined
  const s = sex.trim().toLowerCase()
  if (s === 'm' || s === 'male') return 'ชาย'
  if (s === 'f' || s === 'female') return 'หญิง'
  return undefined
}

/**
 * Downscale an uploaded/pasted image to a reasonable max dimension and re-encode
 * as JPEG. Keeps the base64 payload well under the sessionStorage budget used by
 * the prefill hand-off and the guest-document blob. Returns both the raw base64
 * (no data: prefix, for the API bodies) and a data URL (for preview).
 */
function downscaleImage(file: File): Promise<{ raw: string; dataUrl: string }> {
  return new Promise((resolve, reject) => {
    const reader = new FileReader()
    reader.onerror = () => reject(new Error('อ่านไฟล์รูปไม่สำเร็จ'))
    reader.onload = () => {
      const img = new Image()
      img.onerror = () => reject(new Error('รูปภาพไม่ถูกต้อง'))
      img.onload = () => {
        const MAX = 1000
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
        const dataUrl = canvas.toDataURL('image/jpeg', 0.8)
        const raw = dataUrl.replace(/^data:image\/jpeg;base64,/, '')
        resolve({ raw, dataUrl })
      }
      img.src = reader.result as string
    }
    reader.readAsDataURL(file)
  })
}

export default function PassportScannerPage() {
  const router = useRouter()
  const [connectionStatus, setConnectionStatus] = useState<ConnectionStatus>('disconnected')
  const [parseStatus, setParseStatus] = useState<ParseStatus>('idle')
  const [mrzText, setMrzText] = useState('')
  const [imageRaw, setImageRaw] = useState<string | null>(null)
  const [imagePreview, setImagePreview] = useState<string | null>(null)
  const [parsed, setParsed] = useState<ParsedMrz | null>(null)
  const [errorMessage, setErrorMessage] = useState('')
  const healthCheckRef = useRef<NodeJS.Timeout | null>(null)

  // Connectivity probe to the same per-PC middleware that serves /read.
  const checkHealth = useCallback(async () => {
    try {
      const response = await fetch(`${MIDDLEWARE_URL}/health`, {
        method: 'GET',
        signal: AbortSignal.timeout(3000),
      })
      if (response.ok) {
        setConnectionStatus('connected')
        return true
      }
    } catch {
      // Connection failed
    }
    setConnectionStatus('disconnected')
    return false
  }, [])

  useEffect(() => {
    checkHealth()
    healthCheckRef.current = setInterval(checkHealth, 5000)
    return () => {
      if (healthCheckRef.current) clearInterval(healthCheckRef.current)
    }
  }, [checkHealth])

  const handleImageFile = async (file: File | undefined | null) => {
    if (!file) return
    try {
      const { raw, dataUrl } = await downscaleImage(file)
      setImageRaw(raw)
      setImagePreview(dataUrl)
    } catch (err) {
      setErrorMessage(err instanceof Error ? err.message : 'อ่านรูปไม่สำเร็จ')
    }
  }

  // Allow pasting an image straight from the clipboard (scanner apps / snips).
  useEffect(() => {
    const onPaste = (e: ClipboardEvent) => {
      const item = Array.from(e.clipboardData?.items || []).find((i) =>
        i.type.startsWith('image/')
      )
      const file = item?.getAsFile()
      if (file) handleImageFile(file)
    }
    window.addEventListener('paste', onPaste)
    return () => window.removeEventListener('paste', onPaste)
  }, [])

  const parseMrz = async () => {
    const mrz = mrzText.trim()
    if (!mrz) {
      setErrorMessage('กรุณากรอกหรือวางบรรทัด MRZ 2 บรรทัดจากหนังสือเดินทาง')
      return
    }
    setParseStatus('parsing')
    setErrorMessage('')
    setParsed(null)
    try {
      // Raw fetch — the middleware is an external per-PC service, not our API
      // (branchFetch only decorates /api/* calls).
      const response = await fetch(`${MIDDLEWARE_URL}/parse-mrz`, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ mrz, image: imageRaw || undefined }),
        signal: AbortSignal.timeout(10000),
      })
      if (!response.ok) {
        const errorData = await response.json().catch(() => ({ error: 'Unknown error' }))
        throw new Error(errorData.error || `HTTP ${response.status}`)
      }
      const result = (await response.json()) as ParsedMrz
      if (!result.success) {
        throw new Error('ไม่สามารถอ่านข้อมูล MRZ ได้ — ตรวจสอบว่าเป็น 2 บรรทัด บรรทัดละ 44 ตัวอักษร')
      }
      setParsed(result)
      setParseStatus('success')
    } catch (error) {
      if (error instanceof Error) {
        setErrorMessage(
          error.name === 'TimeoutError' ? 'หมดเวลาเชื่อมต่อ Middleware' : error.message
        )
      } else {
        setErrorMessage('ไม่สามารถอ่านข้อมูล MRZ ได้')
      }
      setParseStatus('error')
    }
  }

  const handleUseData = () => {
    if (!parsed) return
    // Hand off the passport payload to the check-in modal (single-use slot),
    // then route to the rooms screen. Photo comes from the (optional) uploaded
    // image, downscaled above. Nationality is the MRZ 3-letter code — a hint the
    // receptionist can refine.
    setCheckInPrefill({
      docType: 'passport',
      firstName: parsed.givenNames || undefined,
      lastName: parsed.surname || undefined,
      passport: parsed.passportNumber || undefined,
      nationality: parsed.nationality || undefined,
      sex: mrzSexToThaiSex(parsed.sex),
      dob: parsed.dateOfBirth || undefined,
      photoBase64: parsed.image || imageRaw || undefined,
    })
    router.push('/rooms')
  }

  return (
    <div className="container mx-auto px-4 py-8">
      <div className="max-w-2xl mx-auto">
        {/* Header */}
        <div className="flex items-center justify-between mb-6">
          <h1 className="text-2xl font-bold flex items-center gap-3">
            <BookUser size={28} />
            สแกนพาสปอร์ต
          </h1>
        </div>

        {/* Connection Status */}
        <div className="bg-white rounded-lg shadow-sm p-4 mb-6">
          <div className="flex items-center gap-2">
            <Server size={20} className="text-gray-500" />
            <span className="text-gray-600">Middleware:</span>
            {connectionStatus === 'connected' ? (
              <span className="flex items-center gap-1 text-green-600">
                <CheckCircle2 size={16} />
                เชื่อมต่อแล้ว ({MIDDLEWARE_URL})
              </span>
            ) : (
              <span className="flex items-center gap-1 text-red-500">
                <XCircle size={16} />
                ไม่พบ Middleware
              </span>
            )}
          </div>
          {connectionStatus !== 'connected' && (
            <p className="text-xs text-gray-500 mt-2">
              ต้องรัน Middleware (port 9898) บนเครื่องนี้เพื่ออ่านข้อมูล MRZ — เช่นเดียวกับหน้าอ่านบัตรประชาชน
            </p>
          )}
        </div>

        {/* Input card */}
        <div className="bg-white rounded-lg shadow-sm p-6 mb-6 space-y-5">
          {/* Image upload / paste */}
          <div>
            <label className="block text-sm font-medium text-gray-700 mb-2">
              รูปหน้าพาสปอร์ต (ไม่บังคับ)
            </label>
            <div className="flex items-start gap-4">
              <div className="shrink-0">
                {imagePreview ? (
                  // eslint-disable-next-line @next/next/no-img-element
                  <img
                    src={imagePreview}
                    alt="passport"
                    className="w-32 h-40 object-cover rounded-lg border"
                  />
                ) : (
                  <div className="w-32 h-40 bg-gray-100 rounded-lg border flex items-center justify-center">
                    <User size={40} className="text-gray-400" />
                  </div>
                )}
              </div>
              <div className="flex-1 space-y-2">
                <label className="inline-flex items-center gap-2 bg-gray-100 hover:bg-gray-200 text-gray-700 px-4 py-2 rounded-lg cursor-pointer transition-colors">
                  <ImageUp size={18} />
                  เลือกรูป / ถ่ายภาพ
                  <input
                    type="file"
                    accept="image/*"
                    capture="environment"
                    className="hidden"
                    onChange={(e) => handleImageFile(e.target.files?.[0])}
                  />
                </label>
                <p className="text-xs text-gray-500">
                  หรือกด Ctrl/Cmd+V เพื่อวางรูปจากคลิปบอร์ด รูปจะถูกย่อขนาดอัตโนมัติ
                </p>
              </div>
            </div>
          </div>

          {/* MRZ textarea */}
          <div>
            <label className="block text-sm font-medium text-gray-700 mb-2">
              ข้อมูล MRZ (2 บรรทัดล่างสุดของพาสปอร์ต บรรทัดละ 44 ตัวอักษร) *
            </label>
            <textarea
              value={mrzText}
              onChange={(e) => setMrzText(e.target.value)}
              rows={2}
              spellCheck={false}
              placeholder={
                'P<THAHONGSA<<SOMCHAI<<<<<<<<<<<<<<<<<<<<<<<<<\nAB1234567<7THA8501012M3001019<<<<<<<<<<<<<<08'
              }
              className="w-full px-3 py-2 border border-gray-300 rounded-md font-mono text-sm tracking-wider focus:outline-hidden focus:ring-2 focus:ring-blue-500 whitespace-pre"
            />
          </div>

          <div className="flex justify-end">
            <button
              onClick={parseMrz}
              disabled={parseStatus === 'parsing' || !mrzText.trim() || connectionStatus !== 'connected'}
              className="flex items-center gap-2 bg-blue-600 hover:bg-blue-700 disabled:bg-blue-400 disabled:cursor-not-allowed text-white px-4 py-2 rounded-lg transition-colors"
            >
              {parseStatus === 'parsing' ? (
                <RefreshCw size={20} className="animate-spin" />
              ) : (
                <ScanLine size={20} />
              )}
              {parseStatus === 'parsing' ? 'กำลังอ่าน...' : 'อ่านข้อมูล'}
            </button>
          </div>
        </div>

        {/* Error State */}
        {parseStatus === 'error' && (
          <div className="bg-red-50 rounded-lg p-6 mb-6">
            <div className="flex items-start gap-4">
              <AlertCircle size={40} className="text-red-500 shrink-0" />
              <div>
                <h3 className="text-red-700 font-semibold mb-1">เกิดข้อผิดพลาด</h3>
                <p className="text-red-600">{errorMessage}</p>
              </div>
            </div>
          </div>
        )}

        {/* Success State */}
        {parseStatus === 'success' && parsed && (
          <div className="bg-white rounded-lg shadow-sm">
            <div className="p-6">
              <div className="flex gap-6">
                <div className="shrink-0">
                  {imagePreview ? (
                    // eslint-disable-next-line @next/next/no-img-element
                    <img
                      src={imagePreview}
                      alt="passport"
                      className="w-32 h-40 object-cover rounded-lg border"
                    />
                  ) : (
                    <div className="w-32 h-40 bg-gray-100 rounded-lg border flex items-center justify-center">
                      <User size={48} className="text-gray-400" />
                    </div>
                  )}
                </div>

                <div className="grow">
                  <div className="flex items-center justify-between mb-4 border-b pb-2">
                    <h2 className="text-lg font-semibold">ข้อมูลหนังสือเดินทาง</h2>
                    {parsed.checksumValid ? (
                      <span className="flex items-center gap-1 text-green-600 text-xs">
                        <CheckCircle2 size={14} /> checksum ถูกต้อง
                      </span>
                    ) : (
                      <span className="flex items-center gap-1 text-amber-600 text-xs">
                        <AlertCircle size={14} /> checksum ไม่ผ่าน
                      </span>
                    )}
                  </div>

                  <div className="space-y-3">
                    <Row label="เลขหนังสือเดินทาง" value={parsed.passportNumber} mono />
                    <Row label="นามสกุล" value={parsed.surname} />
                    <Row label="ชื่อ" value={parsed.givenNames} />
                    <Row label="สัญชาติ" value={parsed.nationality} />
                    <Row label="วันเกิด" value={parsed.dateOfBirth} />
                    <Row label="เพศ" value={parsed.sex} />
                    <Row label="วันหมดอายุ" value={parsed.expiryDate} />
                  </div>
                </div>
              </div>
            </div>

            <div className="border-t px-6 py-4 bg-gray-50 rounded-b-lg flex justify-end">
              <button
                onClick={handleUseData}
                className="bg-green-600 hover:bg-green-700 text-white px-6 py-2 rounded-lg transition-colors flex items-center gap-2"
              >
                <CheckCircle2 size={20} />
                ใช้ข้อมูลนี้
              </button>
            </div>
          </div>
        )}
      </div>
    </div>
  )
}

function Row({ label, value, mono }: { label: string; value: string; mono?: boolean }) {
  return (
    <div className="grid grid-cols-3 gap-2">
      <span className="text-gray-500">{label}:</span>
      <span className={`col-span-2 ${mono ? 'font-mono' : ''}`}>{value || '-'}</span>
    </div>
  )
}
