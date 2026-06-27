'use client'

import { forwardRef } from 'react'
import { generateCode39 } from '@/lib/code39'

/**
 * Printable food / breakfast coupon (คูปองอาหาร) sized for a thermal
 * receipt roll (58mm or 80mm). Replaces the old plain-text `window.print()`
 * hand-out with a proper coupon carrying a CODE 39 (3-of-9) barcode of the
 * coupon identifier so it can be scanned at the restaurant / cashier.
 *
 * The `code` prop is whatever identifier the caller has on hand:
 *   - the canonical `coupon_code` (e.g. `COUP-A1B2C3D4`) for a coupon just
 *     issued from the new app (the legacy numeric `cupon_no` is allocated
 *     asynchronously by the writeback worker and is not known at issue time);
 *   - the legacy `cupon_no` for the read-only legacy-mirror coupon panel,
 *     matching what iHOTEL prints today.
 *
 * Print isolation: the root carries `coupon-print-root`. `printCoupon()`
 * (`lib/print.ts`) toggles `coupon-print-active` on it for the duration of the
 * print; the `@media print` block below hides everything else on the page so a
 * single coupon prints cleanly even when opened over a busy folio page. This
 * mirrors the `printRegion`/`v2-print-active` pattern already used in /v2.
 */

export interface CouponTemplateProps {
  /** Barcode + human-readable value. Canonical coupon_code or legacy cupon_no. */
  code: string
  /** Hotel name printed in the header. */
  hotelName?: string
  /** Optional hotel phone shown in the footer. */
  hotelPhone?: string
  /** Coupon title. Defaults to the breakfast-coupon label. */
  title?: string
  /** Face value in baht. Shown only when > 0 (food coupons are value-less). */
  value?: number
  /** Optional expiry (ISO date or any displayable string). */
  expiresAt?: string | null
  /** Operator who issued the coupon. */
  issuedBy?: string | null
  /** Legacy `Cin_no` / folio the coupon is attached to. */
  forCinNo?: string | null
  /** Room number, when known (legacy panel rows carry this). */
  room?: string | null
  /** Roll width. Default `80mm`. */
  width?: '58mm' | '80mm'
}

function formatBaht(amount: number): string {
  return new Intl.NumberFormat('th-TH', {
    style: 'currency',
    currency: 'THB',
    minimumFractionDigits: 2,
  }).format(amount)
}

/** Inline CODE 39 barcode rendered as crisp SVG `<rect>` bars. */
function Barcode({ code, narrow }: { code: string; narrow: number }) {
  const { bars, width, height, value } = generateCode39(code, {
    narrow,
    height: 56,
  })
  return (
    <svg
      role="img"
      aria-label={`Barcode ${value}`}
      width="100%"
      viewBox={`0 0 ${width} ${height}`}
      preserveAspectRatio="xMidYMid meet"
      style={{ display: 'block' }}
    >
      <rect x={0} y={0} width={width} height={height} fill="#fff" />
      {bars.map((bar, i) => (
        <rect key={i} x={bar.x} y={0} width={bar.width} height={height} fill="#000" />
      ))}
    </svg>
  )
}

const CouponTemplate = forwardRef<HTMLDivElement, CouponTemplateProps>(function CouponTemplate(
  {
    code,
    hotelName = '',
    hotelPhone,
    title = 'คูปองอาหารเช้า / Breakfast Coupon',
    value = 0,
    expiresAt,
    issuedBy,
    forCinNo,
    room,
    width = '80mm',
  },
  ref,
) {
  // CODE 39 is low-density: 58mm rolls need thinner bars to fit a code.
  const narrow = width === '58mm' ? 1.5 : 2

  return (
    <div
      ref={ref}
      className="coupon-print-root"
      style={{
        width,
        maxWidth: '100%',
        margin: '0 auto',
        background: '#fff',
        color: '#000',
        padding: '4mm',
        boxSizing: 'border-box',
        fontFamily: 'ui-monospace, "SFMono-Regular", Menlo, monospace',
      }}
    >
      <style jsx global>{`
        @media print {
          @page {
            margin: 6mm;
          }
          body * {
            visibility: hidden !important;
          }
          .coupon-print-active,
          .coupon-print-active * {
            visibility: visible !important;
          }
          .coupon-print-active {
            position: fixed !important;
            left: 0 !important;
            top: 0 !important;
            margin: 0 !important;
          }
        }
      `}</style>

      <div style={{ textAlign: 'center' }}>
        {hotelName ? (
          <div style={{ fontSize: '14px', fontWeight: 700 }}>{hotelName}</div>
        ) : null}
        <div style={{ fontSize: '12px', fontWeight: 600, marginTop: '2px' }}>{title}</div>
      </div>

      <div
        style={{
          borderTop: '1px dashed #000',
          margin: '6px 0',
        }}
      />

      <div style={{ fontSize: '11px', lineHeight: 1.5 }}>
        {room ? (
          <div style={{ display: 'flex', justifyContent: 'space-between' }}>
            <span>ห้อง / Room</span>
            <span style={{ fontWeight: 600 }}>{room}</span>
          </div>
        ) : null}
        {forCinNo ? (
          <div style={{ display: 'flex', justifyContent: 'space-between' }}>
            <span>การเข้าพัก / Stay</span>
            <span style={{ fontWeight: 600 }}>{forCinNo}</span>
          </div>
        ) : null}
        {value > 0 ? (
          <div style={{ display: 'flex', justifyContent: 'space-between' }}>
            <span>มูลค่า / Value</span>
            <span style={{ fontWeight: 600 }}>{formatBaht(value)}</span>
          </div>
        ) : null}
        {expiresAt ? (
          <div style={{ display: 'flex', justifyContent: 'space-between' }}>
            <span>หมดอายุ / Expires</span>
            <span style={{ fontWeight: 600 }}>{expiresAt}</span>
          </div>
        ) : null}
        {issuedBy ? (
          <div style={{ display: 'flex', justifyContent: 'space-between' }}>
            <span>ผู้ออก / By</span>
            <span style={{ fontWeight: 600 }}>{issuedBy}</span>
          </div>
        ) : null}
      </div>

      <div style={{ margin: '8px 0 2px' }}>
        <Barcode code={code} narrow={narrow} />
      </div>
      <div
        style={{
          textAlign: 'center',
          fontSize: '13px',
          fontWeight: 700,
          letterSpacing: '2px',
        }}
      >
        {code}
      </div>

      <div
        style={{
          borderTop: '1px dashed #000',
          margin: '6px 0',
        }}
      />

      <div style={{ textAlign: 'center', fontSize: '10px' }}>
        <div>กรุณาแสดงคูปองนี้ที่ห้องอาหาร</div>
        <div>Please present this coupon at the restaurant</div>
        {hotelPhone ? <div style={{ marginTop: '2px' }}>โทร. {hotelPhone}</div> : null}
      </div>
    </div>
  )
})

export default CouponTemplate
