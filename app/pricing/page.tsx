'use client'

import { useState, useEffect, useCallback } from 'react'
import {
  Coins,
  Plus,
  Edit3,
  Loader2,
  AlertCircle,
  X,
  Save,
  CheckCircle,
  XCircle,
  Clock,
} from 'lucide-react'
import { useBranchFetch } from '@/lib/use-branch-fetch'
import { useBranch } from '@/contexts/BranchContext'

interface RateTier {
  id: number
  roomType: string
  custType: string
  price: number
  priceHourly: number | null
  priceMonthly: number | null
  legacyId: number | null
  active: boolean
}

interface ExtensionPrice {
  id: number
  conName: string | null
  conMinute: number | null
  conPrice: number | null
  conType: string | null
}

interface PricingReference {
  extensionPrices: ExtensionPrice[]
}

interface TierFormState {
  id?: number
  roomType: string
  custType: string
  price: string
  priceHourly: string
  priceMonthly: string
  active: boolean
}

const emptyForm: TierFormState = {
  roomType: '',
  custType: '',
  price: '',
  priceHourly: '',
  priceMonthly: '',
  active: true,
}

export default function PricingPage() {
  const branchFetch = useBranchFetch()
  const { canWrite } = useBranch()

  const [tiers, setTiers] = useState<RateTier[]>([])
  const [reference, setReference] = useState<PricingReference | null>(null)
  const [loading, setLoading] = useState(true)
  const [error, setError] = useState<string | null>(null)

  // Edit/create modal
  const [showForm, setShowForm] = useState(false)
  const [formMode, setFormMode] = useState<'create' | 'edit'>('create')
  const [form, setForm] = useState<TierFormState>(emptyForm)
  const [saving, setSaving] = useState(false)
  const [formError, setFormError] = useState<string | null>(null)

  const fetchAll = useCallback(async () => {
    setLoading(true)
    setError(null)
    try {
      const [tiersRes, refRes] = await Promise.all([
        branchFetch('/api/rate-tiers'),
        branchFetch('/api/legacy-mirror/pricing'),
      ])
      if (!tiersRes.ok) throw new Error('ไม่สามารถดึงข้อมูลราคาได้')
      const tiersData = await tiersRes.json()
      setTiers(tiersData.data || [])
      if (refRes.ok) {
        setReference(await refRes.json())
      }
    } catch (err) {
      setError(err instanceof Error ? err.message : 'เกิดข้อผิดพลาด')
    } finally {
      setLoading(false)
    }
  }, [branchFetch])

  useEffect(() => {
    fetchAll()
  }, [fetchAll])

  const handleAdd = () => {
    setFormMode('create')
    setForm(emptyForm)
    setFormError(null)
    setShowForm(true)
  }

  const handleEdit = (t: RateTier) => {
    setFormMode('edit')
    setForm({
      id: t.id,
      roomType: t.roomType,
      custType: t.custType,
      price: String(t.price),
      priceHourly: t.priceHourly !== null ? String(t.priceHourly) : '',
      priceMonthly: t.priceMonthly !== null ? String(t.priceMonthly) : '',
      active: t.active,
    })
    setFormError(null)
    setShowForm(true)
  }

  const handleSave = async () => {
    setFormError(null)
    const price = parseFloat(form.price)
    if (formMode === 'create' && (!form.roomType.trim() || !form.custType.trim())) {
      setFormError('กรุณากรอกประเภทห้องและประเภทลูกค้า')
      return
    }
    if (!Number.isFinite(price) || price < 0) {
      setFormError('กรุณากรอกราคาที่ถูกต้อง')
      return
    }
    const priceHourly = form.priceHourly === '' ? null : parseFloat(form.priceHourly)
    const priceMonthly = form.priceMonthly === '' ? null : parseFloat(form.priceMonthly)

    setSaving(true)
    try {
      const endpoint = form.id ? `/api/rate-tiers/${form.id}` : '/api/rate-tiers'
      const method = form.id ? 'PUT' : 'POST'
      const body = form.id
        ? { price, priceHourly, priceMonthly, active: form.active }
        : {
            roomType: form.roomType,
            custType: form.custType,
            price,
            priceHourly,
            priceMonthly,
            active: form.active,
          }
      const response = await branchFetch(endpoint, {
        method,
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify(body),
      })
      const result = await response.json().catch(() => ({}))
      if (!response.ok || result.success === false) {
        throw new Error(result.message || 'ไม่สามารถบันทึกราคาได้')
      }
      setShowForm(false)
      fetchAll()
    } catch (err) {
      setFormError(err instanceof Error ? err.message : 'เกิดข้อผิดพลาด')
    } finally {
      setSaving(false)
    }
  }

  // Group tiers by room type for a matrix-style display.
  const grouped = tiers.reduce<Record<string, RateTier[]>>((acc, t) => {
    ;(acc[t.roomType] ||= []).push(t)
    return acc
  }, {})

  return (
    <div className="space-y-6">
      {/* Header */}
      <div className="flex items-center justify-between">
        <div className="flex items-center gap-3">
          <Coins className="w-8 h-8 text-red-600" />
          <div>
            <h1 className="text-2xl font-bold text-gray-900">เมทริกซ์ราคา (ประเภทห้อง × ประเภทลูกค้า)</h1>
            <p className="text-gray-500">Room price matrix — mirrored to iHOTEL HT_Rooms_Price</p>
          </div>
        </div>
        <button
          onClick={handleAdd}
          disabled={!canWrite}
          className="flex items-center gap-2 px-4 py-2 bg-red-600 text-white rounded-lg hover:bg-red-700 disabled:opacity-50 disabled:cursor-not-allowed transition-colors"
        >
          <Plus className="w-5 h-5" />
          เพิ่มราคา
        </button>
      </div>

      {error && (
        <div className="flex items-center gap-2 p-4 bg-red-50 border border-red-200 rounded-lg text-red-700">
          <AlertCircle className="w-5 h-5 shrink-0" />
          <span>{error}</span>
        </div>
      )}

      {loading ? (
        <div className="flex items-center justify-center py-12">
          <Loader2 className="w-6 h-6 animate-spin text-red-600" />
          <span className="ml-2 text-gray-500">กำลังโหลด...</span>
        </div>
      ) : tiers.length === 0 ? (
        <div className="bg-white rounded-lg text-center py-12 text-gray-500">
          <Coins className="w-12 h-12 text-gray-300 mx-auto mb-3" />
          <p>ยังไม่มีข้อมูลราคา</p>
        </div>
      ) : (
        <div className="space-y-6">
          {Object.entries(grouped).map(([roomType, rows]) => (
            <div key={roomType} className="bg-white rounded-lg overflow-hidden">
              <div className="px-4 py-3 bg-gray-100 border-b border-gray-200 font-semibold text-gray-700">
                {roomType}
              </div>
              <div className="overflow-x-auto">
                <table className="w-full">
                  <thead className="bg-gray-50 border-b border-gray-200">
                    <tr>
                      <th className="px-4 py-2 text-left text-sm font-semibold text-gray-700">ประเภทลูกค้า</th>
                      <th className="px-4 py-2 text-right text-sm font-semibold text-gray-700">ราคา/คืน</th>
                      <th className="px-4 py-2 text-right text-sm font-semibold text-gray-700">ราคา/ชม.</th>
                      <th className="px-4 py-2 text-right text-sm font-semibold text-gray-700">ราคา/เดือน</th>
                      <th className="px-4 py-2 text-left text-sm font-semibold text-gray-700">สถานะ</th>
                      <th className="px-4 py-2 text-right text-sm font-semibold text-gray-700">จัดการ</th>
                    </tr>
                  </thead>
                  <tbody className="divide-y divide-gray-200">
                    {rows.map((t) => (
                      <tr key={t.id} className={`hover:bg-gray-100 ${!t.active ? 'opacity-60' : ''}`}>
                        <td className="px-4 py-2 text-sm text-gray-900">{t.custType}</td>
                        <td className="px-4 py-2 text-right text-sm text-gray-900">
                          {t.price.toLocaleString()}
                        </td>
                        <td className="px-4 py-2 text-right text-sm text-gray-500">
                          {t.priceHourly !== null ? t.priceHourly.toLocaleString() : '-'}
                        </td>
                        <td className="px-4 py-2 text-right text-sm text-gray-500">
                          {t.priceMonthly !== null ? t.priceMonthly.toLocaleString() : '-'}
                        </td>
                        <td className="px-4 py-2">
                          {t.active ? (
                            <span className="flex items-center gap-1 text-xs text-green-600">
                              <CheckCircle className="w-3 h-3" />
                              เปิด
                            </span>
                          ) : (
                            <span className="flex items-center gap-1 text-xs text-red-600">
                              <XCircle className="w-3 h-3" />
                              ปิด
                            </span>
                          )}
                        </td>
                        <td className="px-4 py-2 text-right">
                          <button
                            onClick={() => handleEdit(t)}
                            disabled={!canWrite}
                            className="p-2 text-gray-500 hover:text-red-600 hover:bg-red-500/10 rounded-lg transition-colors disabled:opacity-40 disabled:cursor-not-allowed"
                            title="แก้ไข"
                          >
                            <Edit3 className="w-4 h-4" />
                          </button>
                        </td>
                      </tr>
                    ))}
                  </tbody>
                </table>
              </div>
            </div>
          ))}
        </div>
      )}

      {/* Read-only legacy reference: extension / continue-time pricing. */}
      {reference && reference.extensionPrices.length > 0 && (
        <div className="bg-white rounded-lg overflow-hidden">
          <div className="px-4 py-3 bg-gray-100 border-b border-gray-200 font-semibold text-gray-700 flex items-center gap-2">
            <Clock className="w-4 h-4" />
            ราคาต่อเวลา (HT_ContinueTime) — อ้างอิงจาก iHOTEL (อ่านอย่างเดียว)
          </div>
          <div className="overflow-x-auto">
            <table className="w-full">
              <thead className="bg-gray-50 border-b border-gray-200">
                <tr>
                  <th className="px-4 py-2 text-left text-sm font-semibold text-gray-700">ชื่อ</th>
                  <th className="px-4 py-2 text-right text-sm font-semibold text-gray-700">นาที</th>
                  <th className="px-4 py-2 text-right text-sm font-semibold text-gray-700">ราคา</th>
                  <th className="px-4 py-2 text-left text-sm font-semibold text-gray-700">ประเภท</th>
                </tr>
              </thead>
              <tbody className="divide-y divide-gray-200">
                {reference.extensionPrices.map((e) => (
                  <tr key={e.id}>
                    <td className="px-4 py-2 text-sm text-gray-900">{e.conName || '-'}</td>
                    <td className="px-4 py-2 text-right text-sm text-gray-500">{e.conMinute ?? '-'}</td>
                    <td className="px-4 py-2 text-right text-sm text-gray-900">
                      {e.conPrice !== null ? e.conPrice.toLocaleString() : '-'}
                    </td>
                    <td className="px-4 py-2 text-sm text-gray-500">{e.conType || '-'}</td>
                  </tr>
                ))}
              </tbody>
            </table>
          </div>
        </div>
      )}

      {/* Create/edit modal */}
      {showForm && (
        <>
          <div className="fixed inset-0 bg-black/30 z-40" onClick={() => setShowForm(false)} />
          <div className="fixed inset-0 z-50 flex items-center justify-center p-4">
            <div className="bg-white rounded-lg shadow-xl border border-gray-200 w-full max-w-md">
              <div className="flex items-center justify-between p-4 border-b border-gray-200 bg-gray-100">
                <h2 className="text-lg font-bold text-gray-900">
                  {formMode === 'create' ? 'เพิ่มราคา' : 'แก้ไขราคา'}
                </h2>
                <button onClick={() => setShowForm(false)} className="p-2 hover:bg-gray-100 rounded-full" aria-label="ปิด">
                  <X className="w-5 h-5" />
                </button>
              </div>
              <div className="p-4 space-y-4">
                {formError && (
                  <div className="flex items-center gap-2 p-3 bg-red-50 border border-red-200 rounded-lg text-red-600">
                    <AlertCircle className="w-5 h-5 shrink-0" />
                    <span className="text-sm">{formError}</span>
                  </div>
                )}
                <div className="grid grid-cols-2 gap-3">
                  <div>
                    <label className="text-sm font-medium text-gray-700 mb-1 block">ประเภทห้อง</label>
                    <input
                      type="text"
                      value={form.roomType}
                      onChange={(e) => setForm((p) => ({ ...p, roomType: e.target.value }))}
                      disabled={formMode === 'edit'}
                      className="w-full px-3 py-2 bg-gray-100 border border-gray-300 text-gray-800 rounded-lg focus:ring-2 focus:ring-red-500 outline-hidden disabled:opacity-60"
                    />
                  </div>
                  <div>
                    <label className="text-sm font-medium text-gray-700 mb-1 block">ประเภทลูกค้า</label>
                    <input
                      type="text"
                      value={form.custType}
                      onChange={(e) => setForm((p) => ({ ...p, custType: e.target.value }))}
                      disabled={formMode === 'edit'}
                      className="w-full px-3 py-2 bg-gray-100 border border-gray-300 text-gray-800 rounded-lg focus:ring-2 focus:ring-red-500 outline-hidden disabled:opacity-60"
                    />
                  </div>
                </div>
                <div className="grid grid-cols-3 gap-3">
                  <div>
                    <label className="text-sm font-medium text-gray-700 mb-1 block">ราคา/คืน</label>
                    <input
                      type="number"
                      value={form.price}
                      onChange={(e) => setForm((p) => ({ ...p, price: e.target.value }))}
                      min="0"
                      step="0.01"
                      className="w-full px-3 py-2 bg-gray-100 border border-gray-300 text-gray-800 rounded-lg focus:ring-2 focus:ring-red-500 outline-hidden"
                    />
                  </div>
                  <div>
                    <label className="text-sm font-medium text-gray-700 mb-1 block">ราคา/ชม.</label>
                    <input
                      type="number"
                      value={form.priceHourly}
                      onChange={(e) => setForm((p) => ({ ...p, priceHourly: e.target.value }))}
                      min="0"
                      step="0.01"
                      className="w-full px-3 py-2 bg-gray-100 border border-gray-300 text-gray-800 rounded-lg focus:ring-2 focus:ring-red-500 outline-hidden"
                    />
                  </div>
                  <div>
                    <label className="text-sm font-medium text-gray-700 mb-1 block">ราคา/เดือน</label>
                    <input
                      type="number"
                      value={form.priceMonthly}
                      onChange={(e) => setForm((p) => ({ ...p, priceMonthly: e.target.value }))}
                      min="0"
                      step="0.01"
                      className="w-full px-3 py-2 bg-gray-100 border border-gray-300 text-gray-800 rounded-lg focus:ring-2 focus:ring-red-500 outline-hidden"
                    />
                  </div>
                </div>
                <label className="flex items-center gap-2 cursor-pointer">
                  <input
                    type="checkbox"
                    checked={form.active}
                    onChange={(e) => setForm((p) => ({ ...p, active: e.target.checked }))}
                    className="w-4 h-4 text-red-600 rounded focus:ring-red-500"
                  />
                  <span className="text-sm text-gray-700">เปิดใช้งาน (ปิด = ไม่ส่งราคาไป iHOTEL)</span>
                </label>
              </div>
              <div className="p-4 border-t border-gray-200 bg-gray-100 flex items-center justify-end gap-3">
                <button
                  onClick={() => setShowForm(false)}
                  className="px-4 py-2 bg-gray-100 hover:bg-gray-200 text-gray-800 rounded-lg transition-colors"
                >
                  ยกเลิก
                </button>
                <button
                  onClick={handleSave}
                  disabled={saving}
                  className="flex items-center gap-2 px-4 py-2 bg-red-600 text-white rounded-lg hover:bg-red-700 disabled:opacity-50 transition-colors"
                >
                  {saving ? <Loader2 className="w-4 h-4 animate-spin" /> : <Save className="w-4 h-4" />}
                  บันทึก
                </button>
              </div>
            </div>
          </div>
        </>
      )}
    </div>
  )
}
