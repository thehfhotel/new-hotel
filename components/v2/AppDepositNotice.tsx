/**
 * "The app already took this deposit" signpost — Task B7.
 *
 * A guest who pays a deposit in the loyalty app is CONFIRMED and paid on the
 * canonical PG side, but iHOTEL shows that booking's deposit as **0** until
 * checkout. That is deliberate and documented: the validated `booking_modify`
 * writeback recipe has no `Book_Price_Pay` leg, and inventing one would break
 * the byte-parity rule, so payment-verified is a PG-only flip with no legacy
 * write.
 * See docs/loyalty-channel.md §"Dual-write policy for holds" "deposit is not mirrored".
 *
 * Daily ops still run in iHOTEL, so the failure mode is a receptionist reading
 * "ยังไม่ชำระ / 0" on a guest who has already paid, asking for the money twice,
 * and quietly killing adoption of the direct channel. This banner is the tell.
 *
 * WHAT COUNTS AS "PAID" HERE — and the gap it papers over:
 * the canonical `ht_bookings` row carries only `book_deposit_amount` (baht)
 * and `book_deposit_date`. There is NO `deposit_paid` / `deposit_verified`
 * marker column. For a loyalty booking that is nevertheless a sound signal:
 * `repository::channel::confirm_booking_payment` is the only writer of
 * `book_deposit_amount` on this path and it runs exactly once, after the
 * loyalty app has verified the guest's payment (it flips `pending` →
 * `confirmed` in the same statement). Hold creation explicitly leaves the
 * deposit at 0. So for `book_channel = 'loyalty'`, a non-zero
 * `book_deposit_amount` means "the app collected this". The wording below says
 * "ชำระผ่านแอปแล้ว / collected by the app" rather than claiming a verified
 * flag we do not have.
 *
 * Deliberately NOT shown for an OTA booking (that money sits with the agency,
 * not with us) nor for a walk-in/phone booking with a desk deposit — those are
 * already recorded in iHOTEL the normal way and carry no divergence.
 *
 * Read-only. Renders nothing when the rule does not apply.
 */

'use client'

import { Smartphone } from 'lucide-react'
import { formatCurrency } from '@/lib/format'

/**
 * The canonical `ht_bookings.book_channel` literal for a guest-app booking —
 * `service::channel::LOYALTY_CHANNEL`. Matched case-insensitively and with
 * padding trimmed, exactly like `bookingChannelView` in `BookingChannelChip`.
 */
const LOYALTY_CHANNEL = 'loyalty'

export type AppDepositNoticeView = {
  /** Booking-level deposit in BAHT (`ht_bookings.book_deposit_amount`). */
  amount: number
}

/**
 * Resolve whether the signpost applies, or `null` for "say nothing".
 *
 * Exported so the reservation detail, the registration card, the folio, the
 * payment dialog, the printed note and the tests all agree on ONE rule — the
 * whole point of this task is that reception never sees a half-signposted
 * booking.
 *
 * `depositAmount` is booking-level baht. A room deposit taken at the desk
 * (`ht_checkin_rooms.cr_dep_amount`) is a different thing and must never be
 * passed here.
 */
export function appDepositNoticeView(
  bookChannel: string | null | undefined,
  depositAmount: number | null | undefined,
): AppDepositNoticeView | null {
  if ((bookChannel ?? '').trim().toLowerCase() !== LOYALTY_CHANNEL) return null
  if (typeof depositAmount !== 'number' || !Number.isFinite(depositAmount)) return null
  if (depositAmount <= 0) return null
  return { amount: depositAmount }
}

/** Thai line, verbatim across every surface. */
export function appDepositThaiLine(amount: number): string {
  return `มัดจำ ${formatCurrency(amount)} ชำระผ่านแอปแล้ว — ยอดในระบบเดิม (iHOTEL) จะแสดง 0 จนกว่าจะเช็คเอาต์`
}

/** English line, shown beneath the Thai one. */
export function appDepositEnglishLine(amount: number): string {
  return `Deposit ${formatCurrency(amount)} was already collected by the app. iHOTEL shows 0 for this booking until checkout — do not ask the guest to pay it again.`
}

export default function AppDepositNotice({
  bookChannel,
  depositAmount,
  className = '',
}: {
  bookChannel: string | null | undefined
  /** Booking-level deposit in baht — `ht_bookings.book_deposit_amount`. */
  depositAmount: number | null | undefined
  className?: string
}) {
  const view = appDepositNoticeView(bookChannel, depositAmount)
  if (!view) return null

  return (
    <div
      data-testid="app-deposit-notice"
      role="note"
      // v2 `dep` teal tokens with literal fallbacks — same pairing as the "แอป"
      // chip, and the tokens are scoped under `.v2` while this also renders on
      // the classic /billing folio and inside BookingForm.
      className={`no-print v2-no-print flex items-start gap-2.5 rounded-lg px-3.5 py-3 ${className}`}
      style={{
        background: 'var(--v2-dep-bg, #e2eff1)',
        color: 'var(--v2-dep, #2a7585)',
        border: '1px solid var(--v2-dep, #2a7585)',
      }}
    >
      <Smartphone size={18} className="shrink-0 mt-0.5" aria-hidden />
      <div className="min-w-0">
        <p data-testid="app-deposit-notice-th" className="text-[13.5px] font-semibold leading-snug">
          {appDepositThaiLine(view.amount)}
        </p>
        <p
          data-testid="app-deposit-notice-en"
          className="text-[12px] leading-snug mt-1 opacity-80"
        >
          {appDepositEnglishLine(view.amount)}
        </p>
      </div>
    </div>
  )
}
