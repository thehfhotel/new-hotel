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
