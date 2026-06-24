'use client'

import { useEffect, useState } from 'react'
import { Loader2, AlertCircle, Ticket, ShoppingBag, ArrowLeftRight, Info } from 'lucide-react'
import { useBranchFetch } from '@/lib/use-branch-fetch'
import { formatStoredDateTime } from '@/lib/format'

/**
 * Phase 5.5e — read-only panels surfacing the `legacy_mirror.*`
 * schema in our UI. Three sections per check-in:
 *
 *  1. Coupons attached (food / breakfast vouchers)
 *  2. In-stay POS / minibar charges
 *  3. Mid-stay room moves
 *
 * Each panel handles its own loading / empty / error state so the
 * receptionist sees something meaningful even when one feature is
 * unused at this site (e.g. HT_CheckIn_Product is empty at HF Hotel).
 *
 * The data is opaque pass-through from the legacy .NET app — we
 * never write to these tables. Visual treatment makes the
 * "view-only, from legacy app" provenance clear.
 */

interface CouponRow {
  cuponNo: number
  cuponCinNo: string | null
  cuponCinRoom: string | null
  cuponDate: string | null
  cuponGenDate: string | null
  cuponBy: string | null
  cuponPrint: number
}

interface ProductRow {
  id: number
  cinNo: string | null
  cinRoomNo: string | null
  cinDsDate: string | null
  cinProId: string | null
  cinProName: string | null
  cinProUnit: string | null
  cinProNum: number | null
  cinProPrice: number | null
  cinProPricetotal: number | null
  cinProPay: number | null
  cinProNote: string | null
}

interface RoomChangeRow {
  id: number
  cinNo: string
  roomBefore: string | null
  roomAfter: string | null
  changeDate: string | null
  roomBeforePrice: number
  note: string | null
  toprice: string | null
}

type ApiState<T> =
  | { kind: 'loading' }
  | { kind: 'error'; message: string }
  | { kind: 'ready'; rows: T[] }

function useLegacyMirror<T>(cinNo: string, endpoint: string): ApiState<T> {
  const branchFetch = useBranchFetch()
  const [state, setState] = useState<ApiState<T>>({ kind: 'loading' })

  useEffect(() => {
    let cancelled = false
    setState({ kind: 'loading' })
    const url = `/api/legacy-mirror/${endpoint}?cin_no=${encodeURIComponent(cinNo)}`
    branchFetch(url)
      .then((res) =>
        res.ok ? res.json() : Promise.reject(new Error(`HTTP ${res.status}`))
      )
      .then((rows: T[]) => {
        if (!cancelled) setState({ kind: 'ready', rows })
      })
      .catch((err) => {
        if (!cancelled)
          setState({
            kind: 'error',
            message: err instanceof Error ? err.message : String(err),
          })
      })
    return () => {
      cancelled = true
    }
  }, [cinNo, endpoint, branchFetch])

  return state
}

function LegacyBadge() {
  return (
    <span className="inline-flex items-center gap-1 px-2 py-0.5 rounded text-xs bg-amber-50 text-amber-700 border border-amber-200">
      <Info className="w-3 h-3" />
      จากระบบเดิม / view-only
    </span>
  )
}

function PanelShell({
  title,
  icon,
  children,
}: {
  title: string
  icon: React.ReactNode
  children: React.ReactNode
}) {
  return (
    <section className="bg-white rounded-xl shadow-xs border border-gray-200 overflow-hidden">
      <header className="flex items-center justify-between px-4 py-3 border-b border-gray-100 bg-gray-50">
        <div className="flex items-center gap-2 text-gray-700 font-medium">
          {icon}
          {title}
        </div>
        <LegacyBadge />
      </header>
      <div className="p-4">{children}</div>
    </section>
  )
}

function StateMessage({ kind, message, count }: { kind: string; message?: string; count?: number }) {
  if (kind === 'loading')
    return (
      <div className="flex items-center gap-2 text-gray-500 text-sm">
        <Loader2 className="w-4 h-4 animate-spin" />
        กำลังโหลด...
      </div>
    )
  if (kind === 'error')
    return (
      <div className="flex items-center gap-2 text-red-600 text-sm">
        <AlertCircle className="w-4 h-4" />
        เกิดข้อผิดพลาด: {message}
      </div>
    )
  if (kind === 'empty')
    return <p className="text-sm text-gray-500">ไม่มีข้อมูล (count: {count ?? 0})</p>
  return null
}

function formatThaiDateTime(s: string | null): string {
  if (!s) return '—'
  // Backend returns NaiveDateTime (no TZ) — values are already Thai local
  // per CLAUDE.md. Render as-is without any TZ conversion.
  return formatStoredDateTime(s)
}

function CouponsPanel({ cinNo }: { cinNo: string }) {
  const state = useLegacyMirror<CouponRow>(cinNo, 'coupons')
  return (
    <PanelShell title="คูปองอาหารเช้า / Coupons" icon={<Ticket className="w-4 h-4" />}>
      {state.kind !== 'ready' && <StateMessage kind={state.kind} message={(state as any).message} />}
      {state.kind === 'ready' && state.rows.length === 0 && <StateMessage kind="empty" count={0} />}
      {state.kind === 'ready' && state.rows.length > 0 && (
        <table className="w-full text-sm">
          <thead className="text-xs text-gray-500 uppercase">
            <tr>
              <th className="text-left py-1 pr-2">เลขคูปอง</th>
              <th className="text-left py-1 pr-2">ห้อง</th>
              <th className="text-left py-1 pr-2">วันที่ออก</th>
              <th className="text-left py-1 pr-2">โดย</th>
              <th className="text-left py-1 pr-2">พิมพ์แล้ว</th>
            </tr>
          </thead>
          <tbody>
            {state.rows.map((c) => (
              <tr key={c.cuponNo} className="border-t border-gray-100">
                <td className="py-1 pr-2 font-mono">{c.cuponNo}</td>
                <td className="py-1 pr-2">{c.cuponCinRoom ?? '—'}</td>
                <td className="py-1 pr-2">{formatThaiDateTime(c.cuponGenDate ?? c.cuponDate)}</td>
                <td className="py-1 pr-2">{c.cuponBy ?? '—'}</td>
                <td className="py-1 pr-2">
                  {c.cuponPrint > 0 ? (
                    <span className="text-green-600">✓ {c.cuponPrint}</span>
                  ) : (
                    <span className="text-gray-400">—</span>
                  )}
                </td>
              </tr>
            ))}
          </tbody>
        </table>
      )}
    </PanelShell>
  )
}

function ProductsPanel({ cinNo }: { cinNo: string }) {
  const state = useLegacyMirror<ProductRow>(cinNo, 'products')
  return (
    <PanelShell title="มินิบาร์ / สินค้าระหว่างเข้าพัก" icon={<ShoppingBag className="w-4 h-4" />}>
      {state.kind !== 'ready' && <StateMessage kind={state.kind} message={(state as any).message} />}
      {state.kind === 'ready' && state.rows.length === 0 && <StateMessage kind="empty" count={0} />}
      {state.kind === 'ready' && state.rows.length > 0 && (
        <table className="w-full text-sm">
          <thead className="text-xs text-gray-500 uppercase">
            <tr>
              <th className="text-left py-1 pr-2">วันที่</th>
              <th className="text-left py-1 pr-2">รายการ</th>
              <th className="text-right py-1 pr-2">จำนวน</th>
              <th className="text-right py-1 pr-2">ราคา/หน่วย</th>
              <th className="text-right py-1 pr-2">รวม</th>
            </tr>
          </thead>
          <tbody>
            {state.rows.map((p) => (
              <tr key={p.id} className="border-t border-gray-100">
                <td className="py-1 pr-2">{formatThaiDateTime(p.cinDsDate)}</td>
                <td className="py-1 pr-2">
                  {p.cinProName ?? '—'}{' '}
                  {p.cinProUnit && <span className="text-gray-400">({p.cinProUnit})</span>}
                </td>
                <td className="py-1 pr-2 text-right">{p.cinProNum?.toLocaleString() ?? '—'}</td>
                <td className="py-1 pr-2 text-right">
                  {p.cinProPrice != null ? `฿${p.cinProPrice.toLocaleString()}` : '—'}
                </td>
                <td className="py-1 pr-2 text-right font-medium">
                  {p.cinProPricetotal != null ? `฿${p.cinProPricetotal.toLocaleString()}` : '—'}
                </td>
              </tr>
            ))}
          </tbody>
        </table>
      )}
    </PanelShell>
  )
}

function RoomChangesPanel({ cinNo }: { cinNo: string }) {
  const state = useLegacyMirror<RoomChangeRow>(cinNo, 'room-changes')
  return (
    <PanelShell title="ประวัติย้ายห้อง / Room moves" icon={<ArrowLeftRight className="w-4 h-4" />}>
      {state.kind !== 'ready' && <StateMessage kind={state.kind} message={(state as any).message} />}
      {state.kind === 'ready' && state.rows.length === 0 && <StateMessage kind="empty" count={0} />}
      {state.kind === 'ready' && state.rows.length > 0 && (
        <ul className="space-y-2 text-sm">
          {state.rows.map((rc) => (
            <li key={rc.id} className="flex items-center gap-2 border-l-2 border-amber-300 pl-3">
              <span className="font-mono">
                {rc.roomBefore ?? '?'} → {rc.roomAfter ?? '?'}
              </span>
              <span className="text-gray-400">·</span>
              <span className="text-gray-600">{formatThaiDateTime(rc.changeDate)}</span>
              {rc.note && (
                <>
                  <span className="text-gray-400">·</span>
                  <span className="text-gray-500 italic">{rc.note}</span>
                </>
              )}
            </li>
          ))}
        </ul>
      )}
    </PanelShell>
  )
}

/**
 * Container — renders all three legacy-mirror panels stacked. Drop
 * into any page that has a legacy `cinNo` in scope (billing folio,
 * check-in detail drawer, etc.).
 */
export default function LegacyMirrorPanels({ cinNo }: { cinNo: string }) {
  if (!cinNo) return null
  return (
    <div className="space-y-4">
      <CouponsPanel cinNo={cinNo} />
      <ProductsPanel cinNo={cinNo} />
      <RoomChangesPanel cinNo={cinNo} />
    </div>
  )
}
