'use client'

/**
 * Single-use prefill hand-off for the check-in forms.
 *
 * The Thai-ID card reader (`app/card-reader/page.tsx`) runs as its own page
 * (the read itself happens over the per-PC Tauri middleware). When the
 * receptionist clicks "use this data" we stash the parsed card fields in an
 * in-memory module slot and send them to the rooms screen; the next check-in
 * modal to mount (`CheckInModal` / `QuickCheckInModal`) consumes them to
 * prefill the guest fields, then clears the slot so a stale card can't leak
 * into an unrelated check-in.
 *
 * In-memory (NOT sessionStorage/localStorage) is deliberate: the payload is
 * guest PII (national-ID / passport number, DOB, address, document photo) and
 * must never be persisted to browser storage, which can be written to disk by
 * session restore and outlives the hand-off. The scanner pages navigate with
 * a client-side `router.push`, so the module state survives the hop; a full
 * reload drops any unconsumed prefill, which just means re-scanning the card
 * — strictly safer than a stale scan resurfacing.
 */

export interface CheckInPrefill {
  firstName?: string
  lastName?: string
  idCard?: string
  /** ISO nationality hint (e.g. "ไทย"); not all forms use it. */
  nationality?: string

  // --- Extended document fields (Thai-ID card reader / passport scanner) ---
  // All optional and additive: consumers that ignore them keep their old
  // behaviour, and a minimal prefill (name + id only) is unchanged.

  /** Latin given name(s) — Thai-ID English first name or passport givenNames. */
  englishFirstName?: string
  /** Latin surname — Thai-ID English last name or passport surname. */
  englishLastName?: string
  /** Thai title / prefix (e.g. "นาย", "นาง", "น.ส."). */
  title?: string
  /** Passport / travel-document number (foreign guests). */
  passport?: string
  /** Gender as the legacy Thai literal ("ชาย" / "หญิง"). */
  sex?: string
  /** Date of birth, Gregorian ISO `YYYY-MM-DD`. */
  dob?: string
  /** Full single-line address (the Thai-ID chip returns it unsplit). */
  address?: string
  /** Structured Thai address parts (rarely populated; forwarded when present). */
  addNo?: string
  addMoo?: string
  addSoi?: string
  addRoad?: string
  addTambon?: string
  addAmpore?: string
  addProvince?: string
  addCode?: string
  /** Raw base64 JPEG (NO `data:` prefix) of the guest photo / document scan. */
  photoBase64?: string
  /**
   * Provisional guest-document `tmp_no` for a card the reader already rendered
   * AND stored server-side (via `POST /api/guest-documents/render-thai-id`).
   * When present the check-in links the stored doc by `photoTmpNo` instead of
   * re-uploading an image — so the full rendered card is preserved rather than
   * just the raw face crop. Mutually exclusive with `photoBase64` in practice.
   */
  docTmpNo?: string
  /** Source document type — drives guest-document storage + legacy `ttype`. */
  docType?: 'thai_id_card' | 'passport'
}

/**
 * The pending hand-off. Module-level so it survives the client-side
 * navigation from the scanner page to the check-in screen, and nothing more.
 * Never touched during SSR (the `window` guards below), so the server-side
 * module instance can't retain one request's PII into another.
 */
let pending: CheckInPrefill | null = null

// One-time hygiene: older builds parked the hand-off in sessionStorage under
// this key. A long-lived reception tab that picked up this build mid-session
// could still hold pre-upgrade scan PII there — purge any residue on load.
if (typeof window !== 'undefined') {
  try {
    window.sessionStorage.removeItem('checkin-prefill')
  } catch {
    // Private-mode storage access can throw; nothing to clean then.
  }
}

/** Stash a pending prefill (overwrites any previous one). */
export function setCheckInPrefill(data: CheckInPrefill): void {
  if (typeof window === 'undefined') return
  pending = data
}

/**
 * Read AND clear the pending prefill. Returns `null` when none is set.
 * Single-use by design: the second caller (or a reload) sees nothing.
 */
export function consumeCheckInPrefill(): CheckInPrefill | null {
  if (typeof window === 'undefined') return null
  const data = pending
  pending = null
  return data
}
