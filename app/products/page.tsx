'use client'

import { useState, useEffect, useCallback } from 'react'
import {
  Package,
  Plus,
  Search,
  Edit3,
  Loader2,
  AlertCircle,
  X,
  Boxes,
  CheckCircle,
  XCircle,
} from 'lucide-react'
import { useBranchFetch } from '@/lib/use-branch-fetch'
import { useBranch } from '@/contexts/BranchContext'
import ProductForm, { ProductFormData } from '@/components/forms/ProductForm'

interface ProductApiItem {
  id: number
  legacyNo: string
  name: string
  unit: string | null
  price: number
  currentStock: number
  category: string | null
  active: boolean
}

export default function ProductsPage() {
  const branchFetch = useBranchFetch()
  const { canWrite } = useBranch()

  const [products, setProducts] = useState<ProductApiItem[]>([])
  const [loading, setLoading] = useState(true)
  const [error, setError] = useState<string | null>(null)
  const [searchQuery, setSearchQuery] = useState('')
  const [debouncedSearch, setDebouncedSearch] = useState('')

  // Create/edit form
  const [showForm, setShowForm] = useState(false)
  const [formMode, setFormMode] = useState<'create' | 'edit'>('create')
  const [formData, setFormData] = useState<ProductFormData | null>(null)

  // Stock-adjust modal
  const [adjustTarget, setAdjustTarget] = useState<ProductApiItem | null>(null)
  const [adjustDelta, setAdjustDelta] = useState('')
  const [adjustReason, setAdjustReason] = useState('')
  const [adjusting, setAdjusting] = useState(false)
  const [adjustError, setAdjustError] = useState<string | null>(null)

  useEffect(() => {
    const t = setTimeout(() => setDebouncedSearch(searchQuery), 300)
    return () => clearTimeout(t)
  }, [searchQuery])

  const fetchProducts = useCallback(async () => {
    setLoading(true)
    setError(null)
    try {
      const params = new URLSearchParams({ limit: '200' })
      if (debouncedSearch) params.append('search', debouncedSearch)
      const response = await branchFetch(`/api/products?${params}`)
      if (!response.ok) throw new Error('ไม่สามารถดึงข้อมูลสินค้าได้')
      const data = await response.json()
      setProducts(data.data || [])
    } catch (err) {
      setError(err instanceof Error ? err.message : 'เกิดข้อผิดพลาด')
    } finally {
      setLoading(false)
    }
  }, [branchFetch, debouncedSearch])

  useEffect(() => {
    fetchProducts()
  }, [fetchProducts])

  const handleAdd = () => {
    setFormMode('create')
    setFormData(null)
    setShowForm(true)
  }

  const handleEdit = (p: ProductApiItem) => {
    setFormMode('edit')
    setFormData({
      id: p.id,
      legacyNo: p.legacyNo,
      name: p.name,
      unit: p.unit ?? '',
      price: p.price,
      currentStock: p.currentStock,
      category: p.category ?? '',
      active: p.active,
    })
    setShowForm(true)
  }

  const handleSave = async (data: ProductFormData) => {
    const endpoint = data.id ? `/api/products/${data.id}` : '/api/products'
    const method = data.id ? 'PUT' : 'POST'
    const body = data.id
      ? {
          name: data.name,
          unit: data.unit,
          price: data.price,
          category: data.category,
          active: data.active,
        }
      : {
          legacyNo: data.legacyNo,
          name: data.name,
          unit: data.unit,
          price: data.price,
          currentStock: data.currentStock,
          category: data.category,
          active: data.active,
        }
    const response = await branchFetch(endpoint, {
      method,
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify(body),
    })
    const result = await response.json().catch(() => ({}))
    if (!response.ok || result.success === false) {
      throw new Error(result.message || 'ไม่สามารถบันทึกสินค้าได้')
    }
    fetchProducts()
  }

  const openAdjust = (p: ProductApiItem) => {
    setAdjustTarget(p)
    setAdjustDelta('')
    setAdjustReason('')
    setAdjustError(null)
  }

  const submitAdjust = async () => {
    if (!adjustTarget) return
    const delta = parseFloat(adjustDelta)
    if (!Number.isFinite(delta) || delta === 0) {
      setAdjustError('กรุณากรอกจำนวนที่ปรับ (ไม่เท่ากับ 0)')
      return
    }
    setAdjusting(true)
    setAdjustError(null)
    try {
      const response = await branchFetch(`/api/products/${adjustTarget.id}/stock-adjust`, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ delta, reason: adjustReason || null }),
      })
      const result = await response.json().catch(() => ({}))
      if (!response.ok || result.success === false) {
        throw new Error(result.message || 'ไม่สามารถปรับสต็อกได้')
      }
      setAdjustTarget(null)
      fetchProducts()
    } catch (err) {
      setAdjustError(err instanceof Error ? err.message : 'เกิดข้อผิดพลาด')
    } finally {
      setAdjusting(false)
    }
  }

  return (
    <div className="space-y-6">
      {/* Header */}
      <div className="flex items-center justify-between">
        <div className="flex items-center gap-3">
          <Package className="w-8 h-8 text-red-600" />
          <div>
            <h1 className="text-2xl font-bold text-gray-900">จัดการสินค้า / มินิบาร์</h1>
            <p className="text-gray-500">Product Management</p>
          </div>
        </div>
        <button
          onClick={handleAdd}
          disabled={!canWrite}
          className="flex items-center gap-2 px-4 py-2 bg-red-600 text-white rounded-lg hover:bg-red-700 disabled:opacity-50 disabled:cursor-not-allowed transition-colors"
        >
          <Plus className="w-5 h-5" />
          เพิ่มสินค้า
        </button>
      </div>

      {/* Search */}
      <div className="bg-white rounded-lg p-4">
        <div className="relative max-w-md">
          <Search className="absolute left-3 top-1/2 -translate-y-1/2 w-5 h-5 text-gray-500" />
          <input
            type="text"
            value={searchQuery}
            onChange={(e) => setSearchQuery(e.target.value)}
            placeholder="ค้นหาชื่อสินค้า..."
            className="w-full pl-10 pr-4 py-2 bg-white text-gray-800 border border-gray-300 rounded-lg focus:ring-2 focus:ring-red-500 focus:border-red-500 outline-hidden transition-colors"
          />
          {searchQuery && (
            <button
              onClick={() => setSearchQuery('')}
              className="absolute right-3 top-1/2 -translate-y-1/2 text-gray-500 hover:text-gray-400"
              aria-label="ล้างการค้นหา"
            >
              <X className="w-4 h-4" />
            </button>
          )}
        </div>
      </div>

      {/* Table */}
      {error && (
        <div className="flex items-center gap-2 p-4 bg-red-50 border border-red-200 rounded-lg text-red-700">
          <AlertCircle className="w-5 h-5 shrink-0" />
          <span>{error}</span>
        </div>
      )}

      <div className="bg-white rounded-lg overflow-hidden">
        {loading ? (
          <div className="flex items-center justify-center py-12">
            <Loader2 className="w-6 h-6 animate-spin text-red-600" />
            <span className="ml-2 text-gray-500">กำลังโหลด...</span>
          </div>
        ) : products.length === 0 ? (
          <div className="text-center py-12 text-gray-500">
            <Package className="w-12 h-12 text-gray-300 mx-auto mb-3" />
            <p>ไม่พบสินค้า</p>
          </div>
        ) : (
          <div className="overflow-x-auto">
            <table className="w-full">
              <thead className="bg-gray-100 border-b border-gray-200">
                <tr>
                  <th className="px-4 py-3 text-left text-sm font-semibold text-gray-700">รหัส</th>
                  <th className="px-4 py-3 text-left text-sm font-semibold text-gray-700">ชื่อสินค้า</th>
                  <th className="px-4 py-3 text-left text-sm font-semibold text-gray-700">หมวดหมู่</th>
                  <th className="px-4 py-3 text-right text-sm font-semibold text-gray-700">ราคา</th>
                  <th className="px-4 py-3 text-right text-sm font-semibold text-gray-700">สต็อก</th>
                  <th className="px-4 py-3 text-left text-sm font-semibold text-gray-700">สถานะ</th>
                  <th className="px-4 py-3 text-right text-sm font-semibold text-gray-700">จัดการ</th>
                </tr>
              </thead>
              <tbody className="divide-y divide-gray-200">
                {products.map((p) => (
                  <tr key={p.id} className={`hover:bg-gray-100 transition-colors ${!p.active ? 'opacity-60' : ''}`}>
                    <td className="px-4 py-3 text-sm text-gray-500">{p.legacyNo}</td>
                    <td className="px-4 py-3 font-medium text-gray-900">{p.name}</td>
                    <td className="px-4 py-3 text-sm text-gray-500">{p.category || '-'}</td>
                    <td className="px-4 py-3 text-right text-sm text-gray-900">
                      {p.price.toLocaleString()} บาท
                    </td>
                    <td className="px-4 py-3 text-right text-sm text-gray-900">
                      {p.currentStock.toLocaleString()} {p.unit || ''}
                    </td>
                    <td className="px-4 py-3">
                      {p.active ? (
                        <span className="flex items-center gap-1 text-xs text-green-600">
                          <CheckCircle className="w-3 h-3" />
                          เปิดใช้งาน
                        </span>
                      ) : (
                        <span className="flex items-center gap-1 text-xs text-red-600">
                          <XCircle className="w-3 h-3" />
                          ปิดใช้งาน
                        </span>
                      )}
                    </td>
                    <td className="px-4 py-3">
                      <div className="flex items-center justify-end gap-1">
                        <button
                          onClick={() => openAdjust(p)}
                          disabled={!canWrite}
                          className="p-2 text-gray-500 hover:text-red-600 hover:bg-red-500/10 rounded-lg transition-colors disabled:opacity-40 disabled:cursor-not-allowed"
                          title="ปรับสต็อก"
                        >
                          <Boxes className="w-4 h-4" />
                        </button>
                        <button
                          onClick={() => handleEdit(p)}
                          disabled={!canWrite}
                          className="p-2 text-gray-500 hover:text-red-600 hover:bg-red-500/10 rounded-lg transition-colors disabled:opacity-40 disabled:cursor-not-allowed"
                          title="แก้ไข"
                        >
                          <Edit3 className="w-4 h-4" />
                        </button>
                      </div>
                    </td>
                  </tr>
                ))}
              </tbody>
            </table>
          </div>
        )}
      </div>

      {/* Product create/edit form */}
      <ProductForm
        isOpen={showForm}
        onClose={() => setShowForm(false)}
        onSave={handleSave}
        initialData={formData}
        mode={formMode}
      />

      {/* Stock-adjust modal */}
      {adjustTarget && (
        <>
          <div className="fixed inset-0 bg-black/30 z-40" onClick={() => setAdjustTarget(null)} />
          <div className="fixed inset-0 z-50 flex items-center justify-center p-4">
            <div className="bg-white rounded-lg shadow-xl border border-gray-200 w-full max-w-md">
              <div className="flex items-center justify-between p-4 border-b border-gray-200 bg-gray-100">
                <h2 className="text-lg font-bold text-gray-900">ปรับสต็อก: {adjustTarget.name}</h2>
                <button onClick={() => setAdjustTarget(null)} className="p-2 hover:bg-gray-100 rounded-full" aria-label="ปิด">
                  <X className="w-5 h-5" />
                </button>
              </div>
              <div className="p-4 space-y-4">
                {adjustError && (
                  <div className="flex items-center gap-2 p-3 bg-red-50 border border-red-200 rounded-lg text-red-600">
                    <AlertCircle className="w-5 h-5 shrink-0" />
                    <span className="text-sm">{adjustError}</span>
                  </div>
                )}
                <p className="text-sm text-gray-500">
                  สต็อกปัจจุบัน: {adjustTarget.currentStock.toLocaleString()} {adjustTarget.unit || ''}
                </p>
                <div>
                  <label className="text-sm font-medium text-gray-700 mb-1 block">
                    จำนวนที่ปรับ (+ เพิ่ม / - ลด)
                  </label>
                  <input
                    type="number"
                    value={adjustDelta}
                    onChange={(e) => setAdjustDelta(e.target.value)}
                    placeholder="เช่น 10 หรือ -3"
                    step="0.001"
                    className="w-full px-3 py-2 bg-gray-100 border border-gray-300 text-gray-800 rounded-lg focus:ring-2 focus:ring-red-500 focus:border-red-500 outline-hidden"
                  />
                </div>
                <div>
                  <label className="text-sm font-medium text-gray-700 mb-1 block">เหตุผล</label>
                  <input
                    type="text"
                    value={adjustReason}
                    onChange={(e) => setAdjustReason(e.target.value)}
                    placeholder="เช่น เติมสต็อก, นับสต็อก"
                    className="w-full px-3 py-2 bg-gray-100 border border-gray-300 text-gray-800 rounded-lg focus:ring-2 focus:ring-red-500 focus:border-red-500 outline-hidden"
                  />
                </div>
              </div>
              <div className="p-4 border-t border-gray-200 bg-gray-100 flex items-center justify-end gap-3">
                <button
                  onClick={() => setAdjustTarget(null)}
                  className="px-4 py-2 bg-gray-100 hover:bg-gray-200 text-gray-800 rounded-lg transition-colors"
                >
                  ยกเลิก
                </button>
                <button
                  onClick={submitAdjust}
                  disabled={adjusting}
                  className="flex items-center gap-2 px-4 py-2 bg-red-600 text-white rounded-lg hover:bg-red-700 disabled:opacity-50 transition-colors"
                >
                  {adjusting ? <Loader2 className="w-4 h-4 animate-spin" /> : <Boxes className="w-4 h-4" />}
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
