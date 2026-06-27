'use client'

/**
 * Print exactly one report region as a clean A4 sheet.
 *
 * The page can hold several `.v2-print` regions at once — e.g. a round-report
 * sheet open over the `/v2/rounds` summary. Tagging only the clicked region
 * `.v2-print-active` (the `@media print` rules in v2.css render exactly that
 * one) stops them overlapping in the printout. The tag is removed afterwards.
 */
export function printRegion(root: HTMLElement | null): void {
  if (!root) {
    window.print()
    return
  }
  const ACTIVE = 'v2-print-active'
  root.classList.add(ACTIVE)
  const cleanup = () => {
    root.classList.remove(ACTIVE)
    window.removeEventListener('afterprint', cleanup)
  }
  window.addEventListener('afterprint', cleanup)
  window.print()
  // `afterprint` is unreliable on some browsers; ensure the tag is dropped.
  window.setTimeout(cleanup, 1500)
}

/**
 * Print exactly one coupon (`components/documents/CouponTemplate.tsx`).
 *
 * Same isolation strategy as {@link printRegion} but keyed on the
 * coupon-specific `coupon-print-active` class so a single coupon prints
 * cleanly on a 58/80mm roll even when the template is opened over a busy
 * folio page (the `@media print` block in CouponTemplate hides everything
 * else while this class is set). The tag is removed after printing.
 */
export function printCoupon(root: HTMLElement | null): void {
  if (!root) {
    window.print()
    return
  }
  const ACTIVE = 'coupon-print-active'
  root.classList.add(ACTIVE)
  const cleanup = () => {
    root.classList.remove(ACTIVE)
    window.removeEventListener('afterprint', cleanup)
  }
  window.addEventListener('afterprint', cleanup)
  window.print()
  // `afterprint` is unreliable on some browsers; ensure the tag is dropped.
  window.setTimeout(cleanup, 1500)
}

/**
 * Print exactly one room-change slip
 * (`components/documents/RoomChangeSlip.tsx`) — Task #54 item 5.
 *
 * Same isolation strategy as {@link printCoupon} but keyed on the
 * room-change-specific `room-change-print-active` class so a single slip prints
 * cleanly on a 58/80mm roll even when opened over the room-grid page.
 */
export function printRoomChange(root: HTMLElement | null): void {
  if (!root) {
    window.print()
    return
  }
  const ACTIVE = 'room-change-print-active'
  root.classList.add(ACTIVE)
  const cleanup = () => {
    root.classList.remove(ACTIVE)
    window.removeEventListener('afterprint', cleanup)
  }
  window.addEventListener('afterprint', cleanup)
  window.print()
  // `afterprint` is unreliable on some browsers; ensure the tag is dropped.
  window.setTimeout(cleanup, 1500)
}
