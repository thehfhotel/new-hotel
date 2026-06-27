/**
 * Self-contained CODE 39 (a.k.a. "3 of 9") barcode generator.
 *
 * Used by the coupon printout (`components/documents/CouponTemplate.tsx`) to
 * render a scannable barcode of the coupon identifier — the canonical
 * `coupon_code` for freshly-issued coupons, or the legacy `cupon_no` for the
 * read-only legacy-mirror coupon panel (iHOTEL prints the numeric cupon_no).
 *
 * No runtime dependency: CODE 39 needs no checksum and no Reed-Solomon maths,
 * so the whole symbology fits in a ~50-line lookup table. We emit geometry
 * (an array of black-bar rectangles + total width) rather than an <svg> string
 * so the consuming React component can render real `<rect>` elements and the
 * encoding stays trivially unit-testable.
 *
 * Each CODE 39 character is 9 elements wide — 5 bars + 4 spaces, of which
 * exactly 3 are "wide" (hence "three of nine"). The element order is
 * bar, space, bar, space, bar, space, bar, space, bar. In the table below a
 * `1` marks a WIDE element and a `0` a NARROW one. Characters are separated by
 * a single narrow space. The symbol is framed by the `*` start/stop guard.
 *
 * Charset: 0-9, A-Z, space, and `- . $ / + %`. Lowercase input is
 * upper-cased; any unsupported character is dropped (CODE 39 has no lowercase).
 */

/** Width pattern per supported character (1 = wide element, 0 = narrow). */
export const CODE39_PATTERNS: Readonly<Record<string, string>> = {
  '0': '000110100',
  '1': '100100001',
  '2': '001100001',
  '3': '101100000',
  '4': '000110001',
  '5': '100110000',
  '6': '001110000',
  '7': '000100101',
  '8': '100100100',
  '9': '001100100',
  A: '100001001',
  B: '001001001',
  C: '101001000',
  D: '000011001',
  E: '100011000',
  F: '001011000',
  G: '000001101',
  H: '100001100',
  I: '001001100',
  J: '000011100',
  K: '100000011',
  L: '001000011',
  M: '101000010',
  N: '000010011',
  O: '100010010',
  P: '001010010',
  Q: '000000111',
  R: '100000110',
  S: '001000110',
  T: '000010110',
  U: '110000001',
  V: '011000001',
  W: '111000000',
  X: '010010001',
  Y: '110010000',
  Z: '011010000',
  '-': '010000101',
  '.': '110000100',
  ' ': '011000100',
  $: '010101000',
  '/': '010100010',
  '+': '010001010',
  '%': '000101010',
  '*': '010010100', // start / stop guard — not a data character
}

export interface Code39Options {
  /** Width (px) of a narrow element. Default 2. */
  narrow?: number
  /** Width (px) of a wide element. Default `narrow * 3` (3:1 ratio). */
  wide?: number
  /** Bar height (px). Default 60. */
  height?: number
  /** Left/right quiet zone (px). Default `narrow * 10`. */
  quietZone?: number
}

/** A single black bar rectangle, in user-space px. */
export interface Code39Bar {
  x: number
  width: number
}

export interface Code39Result {
  /** Black bars to render (`<rect>` per entry); spaces are the gaps between. */
  bars: Code39Bar[]
  /** Total symbol width including both quiet zones (px). */
  width: number
  /** Bar height (px). */
  height: number
  /** The sanitized value actually encoded (upper-cased, unsupported dropped). */
  value: string
}

/** Whether `char` is encodable as a CODE 39 data character (excludes `*`). */
export function isValidCode39Char(char: string): boolean {
  return char !== '*' && char in CODE39_PATTERNS
}

/**
 * Encode `value` as CODE 39 geometry. The result is deterministic given the
 * same options, so it can be snapshotted/asserted in tests.
 */
export function generateCode39(value: string, options: Code39Options = {}): Code39Result {
  const narrow = options.narrow ?? 2
  const wide = options.wide ?? narrow * 3
  const height = options.height ?? 60
  const quietZone = options.quietZone ?? narrow * 10

  const sanitized = (value ?? '')
    .toUpperCase()
    .split('')
    .filter(isValidCode39Char)
    .join('')

  // Frame the data with the start/stop guard character.
  const chars = ['*', ...sanitized.split(''), '*']

  const bars: Code39Bar[] = []
  let x = quietZone

  chars.forEach((char, charIndex) => {
    const pattern = CODE39_PATTERNS[char]
    for (let element = 0; element < pattern.length; element++) {
      const elementWidth = pattern[element] === '1' ? wide : narrow
      const isBar = element % 2 === 0
      if (isBar) {
        bars.push({ x, width: elementWidth })
      }
      x += elementWidth
    }
    // Narrow inter-character gap (a space) between every pair of characters.
    if (charIndex < chars.length - 1) {
      x += narrow
    }
  })

  return {
    bars,
    width: x + quietZone,
    height,
    value: sanitized,
  }
}
