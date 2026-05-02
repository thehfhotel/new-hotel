'use client'

import { useState, useEffect, useCallback } from 'react'
import { X, User, Calendar, AlertCircle, Loader2 } from 'lucide-react'
import { useBranchFetch } from '@/lib/use-branch-fetch'

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

export default function CheckInModal({ room, onClose, onSuccess }: CheckInModalProps) {
  const branchFetch = useBranchFetch()
  const [firstName, setFirstName] = useState('')
  const [lastName, setLastName] = useState('')
  const [phone, setPhone] = useState('')
  const [idCard, setIdCard] = useState('')
  const [adults, setAdults] = useState(1)
  const [children, setChildren] = useState(0)
  const [expectedCheckout, setExpectedCheckout] = useState(tomorrowYmd())
  const [submitting, setSubmitting] = useState(false)
  const [error, setError] = useState<string | null>(null)

  // Type-ahead lookup for existing customers by phone — avoids creating a
  // duplicate customer for repeat guests. If a match is picked, we skip the
  // POST /api/new/customers step.
  const [phoneMatches, setPhoneMatches] = useState<ExistingCustomer[]>([])
  const [pickedCustomerId, setPickedCustomerId] = useState<number | null>(null)

  const lookupByPhone = useCallback(async (q: string) => {
    if (q.length < 4) {
      setPhoneMatches([])
      return
    }
    try {
      const res = await branchFetch(`/api/new/customers?search=${encodeURIComponent(q)}&limit=5`)
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
      // POST a new walk-in customer.
      let customerId = pickedCustomerId
      if (customerId === null) {
        const custRes = await branchFetch('/api/new/customers', {
          method: 'POST',
          headers: { 'Content-Type': 'application/json' },
          body: JSON.stringify({
            firstName: firstName.trim(),
            lastName: lastName.trim() || undefined,
            phone: phone.trim() || undefined,
            idCard: idCard.trim() || undefined,
          }),
        })
        const custData = await custRes.json()
        if (!custRes.ok || !custData.success || !custData.id) {
          throw new Error(custData.message || 'สร้างข้อมูลลูกค้าไม่สำเร็จ')
        }
        customerId = custData.id
      }

      // Step 2: create the check-in.
      const checkinRes = await branchFetch('/api/new/checkins', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({
          customerId,
          roomId: room.id,
          expectedCheckout,
          adults,
          children,
        }),
      })
      const checkinData = await checkinRes.json()
      if (!checkinRes.ok || !checkinData.success) {
        throw new Error(checkinData.message || 'เช็คอินไม่สำเร็จ')
      }

      onSuccess()
      onClose()
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
              placeholder="เช่น 0812345678"
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
      </div>
    </div>
  )
}
