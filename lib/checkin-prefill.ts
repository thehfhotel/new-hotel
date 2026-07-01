'use client'

/**
 * Single-use prefill hand-off for the check-in forms.
 *
 * The Thai-ID card reader (`app/card-reader/page.tsx`) runs as its own page
 * (the read itself happens over the per-PC Tauri middleware). When the
 * receptionist clicks "use this data" we stash the parsed card fields in
 * `sessionStorage` and send them to the rooms screen; the next check-in modal
 * to mount (`CheckInModal` / `QuickCheckInModal`) consumes them to prefill the
 * guest fields, then clears the slot so a stale card can't leak into an
 * unrelated check-in.
 *
 * sessionStorage (not localStorage) is deliberate: the hand-off is transient
 * and scoped to the tab, mirroring the throwaway nature of a card scan.
 */

const KEY = 'checkin-prefill'

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

/** Stash a pending prefill (overwrites any previous one). */
export function setCheckInPrefill(data: CheckInPrefill): void {
  if (typeof window === 'undefined') return
  try {
    window.sessionStorage.setItem(KEY, JSON.stringify(data))
  } catch {
    // sessionStorage can throw in private mode / quota — prefill is a
    // convenience, never block the flow.
  }
}

/**
 * Read AND clear the pending prefill. Returns `null` when none is set.
 * Single-use by design: the second caller (or a reload) sees nothing.
 */
export function consumeCheckInPrefill(): CheckInPrefill | null {
  if (typeof window === 'undefined') return null
  try {
    const raw = window.sessionStorage.getItem(KEY)
    if (!raw) return null
    window.sessionStorage.removeItem(KEY)
    const parsed = JSON.parse(raw) as CheckInPrefill
    return parsed && typeof parsed === 'object' ? parsed : null
  } catch {
    return null
  }
}
