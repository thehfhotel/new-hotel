'use client'

import { useState, useCallback, useEffect, useRef } from 'react'
import { CreditCard, RefreshCw, CheckCircle2, XCircle, User, AlertCircle, Server, Terminal, Download } from 'lucide-react'

type ConnectionStatus = 'disconnected' | 'connecting' | 'connected'
type ReadStatus = 'idle' | 'reading' | 'success' | 'error'

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

const MIDDLEWARE_URL = process.env.NEXT_PUBLIC_CARD_READER_URL || 'http://localhost:9898'

function formatCitizenId(id: string | undefined): string {
  if (!id || id.length !== 13) return id || ''
  return `${id[0]}-${id.slice(1, 5)}-${id.slice(5, 10)}-${id.slice(10, 12)}-${id[12]}`
}

function formatThaiDate(dateStr: string): string {
  try {
    const date = new Date(dateStr)
    const buddhistYear = date.getFullYear() + 543
    return date.toLocaleDateString('th-TH', {
      day: 'numeric',
      month: 'short',
      year: 'numeric',
      timeZone: 'UTC',
    }).replace(/\d{4}/, buddhistYear.toString())
  } catch {
    return dateStr
  }
}

export default function CardReaderPage() {
  const [connectionStatus, setConnectionStatus] = useState<ConnectionStatus>('disconnected')
  const [readStatus, setReadStatus] = useState<ReadStatus>('idle')
  const [cardData, setCardData] = useState<ThaiIdCardData | null>(null)
  const [errorMessage, setErrorMessage] = useState<string>('')
  const healthCheckRef = useRef<NodeJS.Timeout | null>(null)

  // Health check function
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

  // Start health check polling
  useEffect(() => {
    // Initial check
    checkHealth()

    // Poll every 5 seconds
    healthCheckRef.current = setInterval(checkHealth, 5000)

    return () => {
      if (healthCheckRef.current) {
        clearInterval(healthCheckRef.current)
      }
    }
  }, []) // Run once on mount, clean up on unmount

  const readCard = async () => {
    if (connectionStatus !== 'connected') {
      setErrorMessage('กรุณาเชื่อมต่อ Middleware ก่อน')
      return
    }

    setReadStatus('reading')
    setErrorMessage('')
    setCardData(null)

    try {
      const response = await fetch(`${MIDDLEWARE_URL}/read?photo=true`, {
        method: 'GET',
        signal: AbortSignal.timeout(30000), // 30 second timeout for card reading
      })

      if (!response.ok) {
        const errorData = await response.json().catch(() => ({ error: 'Unknown error' }))
        throw new Error(errorData.error || `HTTP ${response.status}`)
      }

      const result = await response.json()
      // Middleware returns { success: true, data: CardData }
      setCardData(result.data || result)
      setReadStatus('success')
    } catch (error) {
      if (error instanceof Error) {
        if (error.name === 'TimeoutError' || error.message.includes('timeout')) {
          setErrorMessage('หมดเวลาในการอ่านบัตร - กรุณาตรวจสอบว่าใส่บัตรแล้ว')
        } else {
          setErrorMessage(error.message)
        }
      } else {
        setErrorMessage('ไม่สามารถอ่านบัตรได้')
      }
      setReadStatus('error')
    }
  }

  const handleUseData = () => {
    if (cardData) {
      alert(`ข้อมูลบัตรประชาชน:\n\nเลขบัตร: ${cardData.cid}\nชื่อ: ${cardData.thaiTitle}${cardData.thaiFirstName} ${cardData.thaiLastName}\n\n(ฟังก์ชันนี้จะเชื่อมต่อกับระบบ check-in ในอนาคต)`)
    }
  }

  return (
    <div className="container mx-auto px-4 py-8">
      <div className="max-w-2xl mx-auto">
        {/* Header */}
        <div className="flex items-center justify-between mb-6">
          <h1 className="text-2xl font-bold flex items-center gap-3">
            <CreditCard size={28} />
            อ่านบัตรประชาชน
          </h1>
          <button
            onClick={readCard}
            disabled={readStatus === 'reading' || connectionStatus !== 'connected'}
            className="flex items-center gap-2 bg-blue-600 hover:bg-blue-700 disabled:bg-blue-400 disabled:cursor-not-allowed text-white px-4 py-2 rounded-lg transition-colors"
          >
            <RefreshCw size={20} className={readStatus === 'reading' ? 'animate-spin' : ''} />
            {readStatus === 'reading' ? 'กำลังอ่าน...' : 'อ่านบัตร'}
          </button>
        </div>

        {/* Connection Status */}
        <div className="bg-white rounded-lg shadow-sm p-4 mb-6">
          <div className="flex items-center justify-between">
            <div className="flex items-center gap-2">
              <Server size={20} className="text-gray-500" />
              <span className="text-gray-600">Middleware:</span>
              {connectionStatus === 'disconnected' && (
                <span className="flex items-center gap-1 text-red-500">
                  <XCircle size={16} />
                  ไม่พบ Middleware
                </span>
              )}
              {connectionStatus === 'connecting' && (
                <span className="flex items-center gap-1 text-yellow-600">
                  <RefreshCw size={16} className="animate-spin" />
                  กำลังเชื่อมต่อ...
                </span>
              )}
              {connectionStatus === 'connected' && (
                <span className="flex items-center gap-1 text-green-600">
                  <CheckCircle2 size={16} />
                  เชื่อมต่อแล้ว ({MIDDLEWARE_URL})
                </span>
              )}
            </div>
          </div>
        </div>

        {/* Middleware Not Found - Setup Instructions */}
        {connectionStatus === 'disconnected' && (
          <div className="bg-yellow-50 border border-yellow-200 rounded-lg p-6 mb-6">
            <div className="flex items-start gap-4">
              <Terminal size={32} className="text-yellow-600 shrink-0 mt-1" />
              <div className="flex-1">
                <h3 className="font-semibold text-yellow-800 mb-2">ต้องติดตั้ง Middleware</h3>
                <p className="text-yellow-700 text-sm mb-4">
                  การอ่านบัตรประชาชนต้องใช้โปรแกรม Middleware ที่ทำงานบนเครื่องที่ต่อเครื่องอ่านบัตร
                </p>

                {/* Download Buttons */}
                <div className="flex flex-wrap gap-3 mb-4">
                  <a
                    href="https://github.com/nutphi/new-hotel/releases/download/middleware-v1.0.0/thai-id-middleware-1.0.0-win.exe"
                    className="inline-flex items-center gap-2 bg-blue-600 hover:bg-blue-700 text-white px-4 py-2 rounded-lg transition-colors"
                  >
                    <Download size={20} />
                    Windows (.exe)
                  </a>
                  <a
                    href="https://github.com/nutphi/new-hotel/releases/download/middleware-v1.0.0/thai-id-middleware-1.0.0-mac.dmg"
                    className="inline-flex items-center gap-2 bg-gray-600 hover:bg-gray-700 text-white px-4 py-2 rounded-lg transition-colors"
                  >
                    <Download size={20} />
                    macOS (.dmg)
                  </a>
                </div>

                <div className="bg-yellow-100 rounded p-3 text-sm text-yellow-800">
                  <p className="mb-2 font-mono"># วิธีติดตั้ง</p>
                  <div className="space-y-2">
                    <p><strong>Windows:</strong> ดาวน์โหลดและดับเบิ้ลคลิกไฟล์ .exe</p>
                    <div>
                      <p><strong>macOS:</strong> เปิดไฟล์ .dmg แล้วลากไปที่ Applications</p>
                      <div className="ml-4 mt-1 text-xs text-yellow-700 bg-yellow-50 rounded p-2">
                        <p className="font-semibold mb-1">หมายเหตุ: เปิดแอปครั้งแรก</p>
                        <p>เนื่องจากแอปไม่ได้ลงนามจาก Apple จะต้อง:</p>
                        <ul className="list-disc list-inside mt-1 space-y-0.5">
                          <li>คลิกขวาที่แอป → เลือก &quot;Open&quot;</li>
                          <li>หรือไปที่ System Settings → Privacy &amp; Security → กด &quot;Open Anyway&quot;</li>
                        </ul>
                      </div>
                    </div>
                  </div>
                  <p className="mt-3 font-mono">Middleware จะทำงานที่ port 9898</p>
                </div>
                <p className="text-yellow-600 text-xs mt-3">
                  URL ปัจจุบัน: {MIDDLEWARE_URL}
                </p>
              </div>
            </div>
          </div>
        )}

        {/* Connected - Ready to Read */}
        {connectionStatus === 'connected' && readStatus === 'idle' && !cardData && (
          <div className="bg-green-50 rounded-lg p-8 text-center">
            <CreditCard size={64} className="mx-auto text-green-500 mb-4" />
            <p className="text-green-700 text-lg mb-2">พร้อมอ่านบัตร</p>
            <p className="text-green-600 text-sm">กรุณาใส่บัตรประชาชนและกดปุ่ม &quot;อ่านบัตร&quot;</p>
          </div>
        )}

        {/* Reading State */}
        {readStatus === 'reading' && (
          <div className="bg-blue-50 rounded-lg p-8 text-center">
            <RefreshCw size={64} className="mx-auto text-blue-500 mb-4 animate-spin" />
            <p className="text-blue-600 text-lg">กำลังอ่านบัตร...</p>
            <p className="text-blue-500 text-sm mt-2">กรุณารอสักครู่</p>
          </div>
        )}

        {/* Error State */}
        {readStatus === 'error' && (
          <div className="bg-red-50 rounded-lg p-8">
            <div className="flex items-start gap-4">
              <AlertCircle size={48} className="text-red-500 shrink-0" />
              <div>
                <h3 className="text-red-700 font-semibold text-lg mb-2">เกิดข้อผิดพลาด</h3>
                <p className="text-red-600 mb-4">{errorMessage}</p>
                <button
                  onClick={readCard}
                  className="bg-red-600 hover:bg-red-700 text-white px-4 py-2 rounded-lg transition-colors"
                >
                  ลองอีกครั้ง
                </button>
              </div>
            </div>
          </div>
        )}

        {/* Success State - Card Data Display */}
        {readStatus === 'success' && cardData && (
          <div className="bg-white rounded-lg shadow-sm">
            <div className="p-6">
              <div className="flex gap-6">
                {/* Photo */}
                <div className="shrink-0">
                  {cardData.photo ? (
                    <img
                      src={`data:image/jpeg;base64,${cardData.photo}`}
                      alt="รูปถ่าย"
                      className="w-32 h-40 object-cover rounded-lg border"
                    />
                  ) : (
                    <div className="w-32 h-40 bg-gray-200 rounded-lg border flex items-center justify-center">
                      <User size={48} className="text-gray-400" />
                    </div>
                  )}
                </div>

                {/* Card Information */}
                <div className="grow">
                  <h2 className="text-lg font-semibold mb-4 border-b pb-2">ข้อมูลบัตร</h2>

                  <div className="space-y-3">
                    <div className="grid grid-cols-3 gap-2">
                      <span className="text-gray-500">เลขบัตร:</span>
                      <span className="col-span-2 font-mono">{formatCitizenId(cardData.cid)}</span>
                    </div>

                    <div className="grid grid-cols-3 gap-2">
                      <span className="text-gray-500">ชื่อ-สกุล:</span>
                      <span className="col-span-2">{cardData.thaiTitle}{cardData.thaiFirstName} {cardData.thaiLastName}</span>
                    </div>

                    <div className="grid grid-cols-3 gap-2">
                      <span className="text-gray-500">Name:</span>
                      <span className="col-span-2">{cardData.englishTitle}{cardData.englishFirstName} {cardData.englishLastName}</span>
                    </div>

                    <div className="grid grid-cols-3 gap-2">
                      <span className="text-gray-500">วันเกิด:</span>
                      <span className="col-span-2">{formatThaiDate(cardData.dateOfBirth)}</span>
                    </div>

                    <div className="grid grid-cols-3 gap-2">
                      <span className="text-gray-500">เพศ:</span>
                      <span className="col-span-2">{cardData.gender === 'male' ? 'ชาย' : 'หญิง'}</span>
                    </div>

                    <div className="grid grid-cols-3 gap-2">
                      <span className="text-gray-500">ที่อยู่:</span>
                      <span className="col-span-2 text-sm">{cardData.address}</span>
                    </div>

                    <div className="grid grid-cols-3 gap-2">
                      <span className="text-gray-500">วันออกบัตร:</span>
                      <span className="col-span-2">{formatThaiDate(cardData.issueDate)}</span>
                    </div>

                    <div className="grid grid-cols-3 gap-2">
                      <span className="text-gray-500">วันหมดอายุ:</span>
                      <span className="col-span-2">{formatThaiDate(cardData.expireDate)}</span>
                    </div>
                  </div>
                </div>
              </div>
            </div>

            {/* Action Footer */}
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

        {/* Middleware Info */}
        <div className="mt-6 bg-blue-50 border border-blue-200 rounded-lg p-6">
          <h3 className="font-semibold text-blue-800 mb-3 flex items-center gap-2">
            <Server size={20} />
            เกี่ยวกับ Middleware
          </h3>
          <div className="text-blue-700 text-sm space-y-2">
            <p>ระบบนี้ใช้ Middleware อ่านบัตรประชาชนผ่าน PC/SC interface</p>
            <p className="font-medium mt-3">ข้อดี:</p>
            <ul className="list-disc list-inside ml-2 space-y-1">
              <li>รองรับเครื่องอ่านบัตรทุกรุ่นที่ใช้ PC/SC</li>
              <li>ใช้งานได้กับทุกเบราว์เซอร์</li>
              <li>เสถียรและเชื่อถือได้</li>
            </ul>
            <p className="mt-3 text-blue-600">
              <strong>หมายเหตุ:</strong> ต้องรัน Middleware บนเครื่องที่ต่อเครื่องอ่านบัตร
            </p>
          </div>
        </div>
      </div>
    </div>
  )
}
