//! Direct-booking program D3 — channel rollup.
//!
//! "How much of our business is direct?" is unanswerable from the PMS today:
//! `ht_bookings.book_channel` (migration 076) and `ht_bookings.book_source`
//! are free text wired to no reporting route. B6 (PR #300) put `bookChannel`
//! on the booking DTOs so reception can *see* it per row; this module makes it
//! *countable*.
//!
//! Pure read-path, per `docs/architecture.md` §1: no outbox, no writeback, no
//! legacy touch. Nothing here writes anything anywhere.
//!
//! ## The two halves
//!
//! 1. [`load_channel_rollup`] — one grouped PG query over `ht_bookings`
//!    (+ `ht_booking_rooms` for the room multiplier), returning one row per
//!    distinct `(book_channel, book_source)` pair in the window.
//! 2. [`rollup`] — a pure fold of those rows into the four reporting buckets
//!    via [`classify`], plus totals and the direct-share percentages.
//!
//! The split exists so the classification rules — the part that is actually
//! contentious — are unit-testable without a database.
//!
//! ## Why this does NOT reuse the occupancy/revenue report's SQL
//!
//! `/api/reports/revenue` and `/api/reports/occupancy` are **check-in
//! centric**: they read `ht_checkins` and share `CHECKIN_REVENUE_EXPR`
//! (`routes::new_reports`). That is the right basis for realised occupancy,
//! and the wrong basis here, for two reasons that are not fixable by picking
//! different joins:
//!
//! * **Channel lives on the booking.** `book_channel` / `book_source` are
//!   `ht_bookings` columns. `ht_checkins` has no channel column and no
//!   guaranteed booking link (`cin_book_id` is nullable — a walk-in has none),
//!   so a check-in-based rollup would silently drop every walk-in into a
//!   "no channel" hole.
//! * **Cancellations have no check-in row at all.** A cancelled booking never
//!   produces an `ht_checkins` row, so a check-in-based query cannot report a
//!   cancelled count per channel — which is the whole of KPI K7.
//!
//! Consequence to state plainly wherever these numbers are shown: `roomNights`
//! here will NOT equal `occupied_nights` from `/api/reports/occupancy`. This
//! report measures *booked* demand attributed to its arrival date; that one
//! measures *realised* room-nights clipped to the window. They answer
//! different questions and are expected to differ.

use chrono::NaiveDate;
use serde::Serialize;
use sqlx::{PgPool, Row};

use crate::service::channel::LOYALTY_CHANNEL;

/// Longest window the route will aggregate, in days (inclusive of both ends).
/// A rollup is a dashboard read, not an export; an unbounded range invites a
/// seq-scan over the whole booking history on every dashboard refresh.
pub const MAX_RANGE_DAYS: i64 = 366;

/// Default window when the caller supplies neither `from` nor `to`.
pub const DEFAULT_RANGE_DAYS: i64 = 30;

/// `book_source` the CT sync mapper stamps on every booking it discovers in
/// iHOTEL (`sync/mappers/booking.rs` — hardcoded on insert).
///
/// It records only *that* the row came from the legacy app, never how the
/// guest reached us: iHOTEL has no channel column (its `HT_Book_H.Book_Sale`
/// is written blank by our own byte-parity recipe and is not populated by
/// reception either). These rows are therefore [`ChannelBucket::Unknown`],
/// NOT direct — see the module-level note on the `unknown` bucket in
/// `docs/channel-rollup.md`. Today they are the overwhelming majority of
/// production bookings, because daily ops still run in iHOTEL.
const SOURCE_LEGACY_APP: &str = "legacy_app";

/// `book_source` values that positively assert a direct (non-OTA) origin.
///
/// Closed allowlist on purpose: an unrecognised source is reported as
/// `unknown` rather than silently inflating the direct share. A big `unknown`
/// bucket is a visible bug report; a quietly-flattering `direct` number is
/// not. `walk-in`, `phone`, `online` and `ota` are the four values the desk
/// booking form offers (`components/forms/BookingForm.tsx`); the rest are
/// defensive spellings.
const DIRECT_SOURCES: &[&str] = &[
    "walk-in",
    "walkin",
    "walk_in",
    "phone",
    "telephone",
    "line",
    "direct",
    "desk",
    "online",
    "website",
    "email",
];

/// `book_channel` values that are a direct origin rather than an OTA name.
///
/// Mirrors `NO_CHIP_CHANNELS` in `components/v2/BookingChannelChip.tsx` — the
/// B6 chip renders nothing for these, and this report must agree with what
/// reception sees on the row.
const DIRECT_CHANNELS: &[&str] = &["walkin", "walk-in", "walk_in", "phone", "direct", "desk"];

/// Display names for the OTA slugs `book_channel` actually carries.
///
/// **Twin of `OTA_LABELS` in `components/v2/BookingChannelChip.tsx`** — keep
/// the two in step, or the weekly pack and the reservations list will name the
/// same OTA differently. Anything unmapped falls through to the raw slug,
/// which still buckets correctly (it is only the label that is less pretty).
const OTA_LABELS: &[(&str, &str)] = &[
    ("bookingcom", "Booking.com"),
    ("booking.com", "Booking.com"),
    ("agoda", "Agoda"),
    ("expedia", "Expedia"),
    ("traveloka", "Traveloka"),
    ("trip", "Trip.com"),
    ("trip.com", "Trip.com"),
    ("ctrip", "Trip.com"),
    ("airbnb", "Airbnb"),
    ("ota", "OTA"),
];

/// Bucket key used when an OTA booking is known to be an OTA but its channel
/// was never recorded (the pre-076 `book_source='ota'` class).
const OTA_UNNAMED_SLUG: &str = "ota";

/// Which reporting bucket a booking falls into.
///
/// The derived `Ord` is the report's display order and is load-bearing:
/// variant order gives app → OTAs → direct → unknown, and `Ota(String)`
/// compares by slug so the OTAs come out alphabetically. Reordering the
/// variants reorders the response.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum ChannelBucket {
    /// Booked in the guest app — `book_channel = 'loyalty'`.
    App,
    /// Came from an OTA. Carries the normalised slug (`"agoda"`,
    /// `"bookingcom"`, or `"ota"` when the specific OTA is unrecorded).
    Ota(String),
    /// Walk-in / phone / LINE / our own website — a booking we did not pay
    /// commission on and did not take through the app.
    Direct,
    /// Provenance not recorded. Dominated today by legacy-sync rows
    /// (`book_source='legacy_app'`), which iHOTEL cannot attribute.
    Unknown,
}

impl ChannelBucket {
    /// Stable machine key for the bucket (`"app"`, an OTA slug, `"direct"`,
    /// `"unknown"`).
    pub fn key(&self) -> &str {
        match self {
            ChannelBucket::App => "app",
            ChannelBucket::Ota(slug) => slug,
            ChannelBucket::Direct => "direct",
            ChannelBucket::Unknown => "unknown",
        }
    }

    /// Coarse family: `"app"` | `"ota"` | `"direct"` | `"unknown"`. Lets a
    /// consumer sum every OTA without knowing the slug list.
    pub fn kind(&self) -> &'static str {
        match self {
            ChannelBucket::App => "app",
            ChannelBucket::Ota(_) => "ota",
            ChannelBucket::Direct => "direct",
            ChannelBucket::Unknown => "unknown",
        }
    }

    /// Human label for a report or a chart legend.
    pub fn label(&self) -> String {
        match self {
            ChannelBucket::App => "App".to_string(),
            ChannelBucket::Ota(slug) => ota_label(slug),
            ChannelBucket::Direct => "Direct".to_string(),
            ChannelBucket::Unknown => "Unknown".to_string(),
        }
    }

    /// Counts toward the direct share: the app and every direct origin.
    /// An OTA does not; neither does an unattributable booking.
    pub fn is_direct_share(&self) -> bool {
        matches!(self, ChannelBucket::App | ChannelBucket::Direct)
    }
}

fn ota_label(slug: &str) -> String {
    OTA_LABELS
        .iter()
        .find(|(k, _)| *k == slug)
        .map(|(_, v)| (*v).to_string())
        .unwrap_or_else(|| slug.to_string())
}

/// Normalise a free-text provenance field: trim, lowercase, and treat an
/// empty string as absent. `book_channel` / `book_source` are `VARCHAR(50)`
/// with no constraint, so `"  Agoda "` and `"agoda"` are the same channel.
fn normalise(raw: Option<&str>) -> Option<String> {
    raw.map(|s| s.trim().to_lowercase())
        .filter(|s| !s.is_empty())
}

/// Bucket one booking from its `(book_channel, book_source)` pair.
///
/// The rules, in order — this is the contentious part, so it is spelled out:
///
/// | # | condition | bucket |
/// |---|---|---|
/// | 1 | `channel = 'loyalty'` | `App` |
/// | 2 | `channel` set, and a direct spelling (`walkin`, `phone`, …) | `Direct` |
/// | 3 | `channel` set, anything else | `Ota(channel)` |
/// | 4 | no channel, `source = 'loyalty'` | `App` |
/// | 5 | no channel, `source = 'ota'` | `Ota("ota")` — pre-076 rows |
/// | 6 | no channel, `source` names a known OTA | `Ota(source)` |
/// | 7 | no channel, `source` a known direct spelling | `Direct` |
/// | 8 | no channel, `source` absent / `legacy_app` / anything else | `Unknown` |
///
/// Rules 1-3 and 5-6 are deliberately the same decision the reservations chip
/// makes (`bookingChannelView` in `components/v2/BookingChannelChip.tsx`), so
/// a row that reads "Agoda" at the desk counts as Agoda here.
///
/// **Deviation worth knowing about:** the D3 brief defined direct as
/// "`book_channel` null and `book_source` not ota", which would sweep both
/// `legacy_app` and a null source into `Direct`. Rule 8 sends them to
/// `Unknown` instead. Reason: essentially every production booking today is a
/// legacy-sync row, and iHOTEL records no channel — calling those "direct"
/// would report a direct share near 100% that means nothing. Reverting to the
/// literal brief is a one-line change to rule 8.
pub fn classify(channel: Option<&str>, source: Option<&str>) -> ChannelBucket {
    let channel = normalise(channel);
    let source = normalise(source);

    if let Some(channel) = channel.as_deref() {
        if channel == LOYALTY_CHANNEL {
            return ChannelBucket::App;
        }
        if DIRECT_CHANNELS.contains(&channel) {
            return ChannelBucket::Direct;
        }
        return ChannelBucket::Ota(channel.to_string());
    }

    let Some(source) = source.as_deref() else {
        return ChannelBucket::Unknown;
    };

    if source == LOYALTY_CHANNEL {
        return ChannelBucket::App;
    }
    if source == OTA_UNNAMED_SLUG {
        return ChannelBucket::Ota(OTA_UNNAMED_SLUG.to_string());
    }
    if OTA_LABELS.iter().any(|(k, _)| *k == source) {
        return ChannelBucket::Ota(source.to_string());
    }
    if DIRECT_SOURCES.contains(&source) {
        return ChannelBucket::Direct;
    }
    if source == SOURCE_LEGACY_APP {
        // Recorded, but records nothing about origin.
        return ChannelBucket::Unknown;
    }
    // Every unrecognised spelling. Deliberately not `Direct`: see the
    // `DIRECT_SOURCES` note.
    ChannelBucket::Unknown
}

/// One `(book_channel, book_source)` group as it comes back from PG.
#[derive(Debug, Clone)]
pub struct ChannelSourceRow {
    pub channel: Option<String>,
    pub source: Option<String>,
    /// Bookings whose `book_checkin` falls in the window — every status,
    /// cancellations included.
    pub bookings: i64,
    /// Subset of `bookings` with `book_status = 'cancelled'`.
    pub cancelled: i64,
    /// Subset of `cancelled` whose `book_hold_auto_released_at` is set —
    /// loyalty holds the sweep auto-released on the clock (migration 096, B13).
    pub holds_auto_released: i64,
    /// `book_nights × rooms`, summed over NON-cancelled bookings only.
    pub room_nights: i64,
    /// `book_total_amount`, summed over NON-cancelled bookings only.
    pub gross_revenue: f64,
}

/// Aggregated figures for one bucket (or for the whole window, as `totals`).
#[derive(Debug, Clone, Default, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ChannelTotals {
    /// All bookings arriving in the window, cancellations included.
    pub bookings: i64,
    /// How many of `bookings` were cancelled.
    pub cancelled: i64,
    /// How many of `cancelled` were loyalty holds the expiry sweep
    /// AUTO-RELEASED because their payment window lapsed —
    /// `book_hold_auto_released_at IS NOT NULL` (migration 096, B13). A strict
    /// SUBSET of `cancelled`: counted over the same rows, on the same
    /// `book_checkin` basis, so `holdsExpired / bookings` in the `app` bucket
    /// is the expired-hold rate B13 must read before taking a `HOLD_TTL`
    /// decision. Structurally `0` in every non-`app` bucket, because only a
    /// loyalty hold has a payment window to lapse.
    ///
    /// **The wire name is deliberately NOT the field name.** The column and
    /// this field are named for the mechanism (an auto-release, so they cannot
    /// be typo-confused with `book_hold_expires_at` and stay accurate if
    /// `HOLD_TTL`'s rules change); `holdsExpired` is the business question the
    /// number answers, and it is the name loyalty-app's friction card reads.
    /// The explicit `rename` pins the two together so neither can drift.
    #[serde(rename = "holdsExpired")]
    pub holds_auto_released: i64,
    /// Room-nights on non-cancelled bookings: `book_nights × rooms booked`.
    pub room_nights: i64,
    /// `book_total_amount` on non-cancelled bookings, in baht.
    pub gross_revenue: f64,
}

impl ChannelTotals {
    fn add(&mut self, row: &ChannelSourceRow) {
        self.bookings += row.bookings;
        self.cancelled += row.cancelled;
        self.holds_auto_released += row.holds_auto_released;
        self.room_nights += row.room_nights;
        self.gross_revenue += row.gross_revenue;
    }
}

/// One bucket's line in the report.
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ChannelBucketRow {
    /// `"app"` | an OTA slug | `"direct"` | `"unknown"`.
    pub key: String,
    /// `"app"` | `"ota"` | `"direct"` | `"unknown"`.
    pub kind: String,
    /// Display name (`"Agoda"`, `"Direct"`, …).
    pub label: String,
    #[serde(flatten)]
    pub totals: ChannelTotals,
}

/// The whole report body.
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ChannelRollup {
    pub buckets: Vec<ChannelBucketRow>,
    pub totals: ChannelTotals,
    /// (app + direct) room-nights over all room-nights, as a percentage
    /// rounded to one decimal. Room-nights is the basis KPI K5 is written
    /// against; `0.0` when the window has no room-nights.
    pub direct_share: f64,
    /// Same ratio on booking counts.
    pub direct_share_by_bookings: f64,
    /// Same ratio on gross revenue.
    pub direct_share_by_revenue: f64,
}

fn pct(part: f64, whole: f64) -> f64 {
    if whole <= 0.0 {
        return 0.0;
    }
    ((part / whole) * 1000.0).round() / 10.0
}

/// Fold raw `(channel, source)` groups into buckets, totals and the direct
/// shares. Pure — this is what the unit tests exercise.
pub fn rollup(rows: &[ChannelSourceRow]) -> ChannelRollup {
    let mut buckets: Vec<(ChannelBucket, ChannelTotals)> = Vec::new();
    let mut totals = ChannelTotals::default();
    let mut direct = ChannelTotals::default();

    for row in rows {
        let bucket = classify(row.channel.as_deref(), row.source.as_deref());
        totals.add(row);
        if bucket.is_direct_share() {
            direct.add(row);
        }
        match buckets.iter_mut().find(|(b, _)| *b == bucket) {
            Some((_, acc)) => acc.add(row),
            None => {
                let mut acc = ChannelTotals::default();
                acc.add(row);
                buckets.push((bucket, acc));
            }
        }
    }

    // Derived `Ord` on `ChannelBucket` is the display order.
    buckets.sort_by(|(a, _), (b, _)| a.cmp(b));

    ChannelRollup {
        buckets: buckets
            .into_iter()
            .map(|(bucket, totals)| ChannelBucketRow {
                key: bucket.key().to_string(),
                kind: bucket.kind().to_string(),
                label: bucket.label(),
                totals,
            })
            .collect(),
        direct_share: pct(direct.room_nights as f64, totals.room_nights as f64),
        direct_share_by_bookings: pct(direct.bookings as f64, totals.bookings as f64),
        direct_share_by_revenue: pct(direct.gross_revenue, totals.gross_revenue),
        totals,
    }
}

/// One grouped read over `ht_bookings`, attributed by **check-in (arrival)
/// date** — `book_checkin BETWEEN from AND to`.
///
/// Sources, stated explicitly because the brief asked which columns these
/// numbers come from:
///
/// * **room-nights** = `ht_bookings.book_nights × rooms`, where `book_nights`
///   is the stored generated column (`book_checkout - book_checkin`) and
///   `rooms` is `COUNT(*)` over `ht_booking_rooms` for the booking, floored at
///   1. The floor matters: a booking whose rooms have not been assigned yet
///   has zero `ht_booking_rooms` rows, and counting it as zero room-nights
///   would quietly under-report every channel's forward book. Multi-room
///   stays are iHOTEL-created (our app rejects multi-room walk-ins), so the
///   multiplier is what makes their nights count correctly.
/// * **gross revenue** = `ht_bookings.book_total_amount`. Chosen over the
///   reports' `CHECKIN_REVENUE_EXPR` because that expression reads
///   `ht_checkins` — see the module header. `book_total_amount` is also the
///   only revenue figure that exists for a booking that was cancelled or has
///   not arrived yet, and it is the column the OTA bridge writes the OTA
///   gross into.
/// * **cancelled** = `book_status = 'cancelled'`.
///
/// Room-nights and gross revenue deliberately EXCLUDE cancelled bookings
/// (a cancelled stay sells no nights and earns no baht); `bookings` includes
/// them, so `cancelled / bookings` is the cancellation rate KPI K7 wants.
pub async fn load_channel_rollup(
    pool: &PgPool,
    from: NaiveDate,
    to: NaiveDate,
) -> Result<Vec<ChannelSourceRow>, sqlx::Error> {
    // Static SQL with bound dates — no interpolation, so no `.sqlx/` entry is
    // needed (dynamic `sqlx::query()`, same idiom as the VAT summary).
    //
    // The `'cancelled'` literal is the canonical `book_status` value: our own
    // cancel path writes it, and the CT sync mapper maps legacy `ยกเลิก` onto
    // it (`sync::mappers::booking::legacy_status_to_pg`). `IS DISTINCT FROM`
    // rather than `<>` because `book_status` is nullable — a NULL status must
    // count as sellable, not vanish from both sides of the split.
    //
    // `holds_auto_released` (B13, reported as `holdsExpired`) counts
    // `book_hold_auto_released_at IS NOT NULL` — the typed marker migration 096
    // added, stamped only by the expiry sweep. Two deliberate choices:
    //
    //   * **Typed, not textual.** The obvious alternative was to match
    //     `book_cancel_reason`. That is wrong today: the sweep writes "loyalty
    //     hold expired (auto-release)" and the channel's own release endpoint
    //     writes "loyalty payment window lapsed (channel release)". Both read
    //     as "the payment window ran out", but only the first is an expiry
    //     whose TTL we control — the endpoint fires when the loyalty app hands
    //     the room back, a guest abandonment. A prose match would silently
    //     count those as TTL expiries, inflating the exact rate B13 exists to
    //     read, and would be one rename away from silently returning 0 with no
    //     compile error.
    //   * **Same window basis as `cancelled`, on purpose.** Every figure here
    //     is attributed by `book_checkin` (arrival), so a hold that expired in
    //     January for a March stay lands in March. An expiry-time basis
    //     (`book_cancelled_at`) would read more naturally on its own, but it
    //     would break the invariant that makes this number useful: counted
    //     this way it is a strict SUBSET of `cancelled`, over the same rows,
    //     so `holdsExpired / bookings` within the `app` bucket is
    //     a real rate with a matching numerator and denominator. Split the
    //     bases and the ratio silently compares two different populations.
    //     See `docs/channel-rollup.md`.
    //
    // No status guard is needed (unlike `book_hold_expires_at`, which is left
    // behind on confirmation): the marker is written only by the release that
    // cancels, so it is NULL on every confirmed booking.
    let rows = sqlx::query(
        r#"
        WITH scoped AS (
            SELECT
                NULLIF(BTRIM(LOWER(b.book_channel)), '') AS channel,
                NULLIF(BTRIM(LOWER(b.book_source)), '')  AS source,
                NULLIF(BTRIM(LOWER(b.book_status)), '')  AS status,
                b.book_hold_auto_released_at             AS auto_released_at,
                b.book_nights                            AS nights,
                COALESCE(b.book_total_amount, 0)         AS gross,
                GREATEST(
                    (SELECT COUNT(*) FROM ht_booking_rooms br WHERE br.br_book_id = b.book_id),
                    1
                )                                        AS rooms
            FROM ht_bookings b
            WHERE b.book_checkin >= $1::date
              AND b.book_checkin <= $2::date
        )
        SELECT
            channel,
            source,
            COUNT(*)::bigint AS bookings,
            COUNT(*) FILTER (WHERE status = 'cancelled')::bigint AS cancelled,
            COUNT(*) FILTER (WHERE auto_released_at IS NOT NULL)::bigint AS holds_auto_released,
            COALESCE(
                SUM(nights * rooms) FILTER (WHERE status IS DISTINCT FROM 'cancelled'), 0
            )::bigint AS room_nights,
            COALESCE(
                SUM(gross) FILTER (WHERE status IS DISTINCT FROM 'cancelled'), 0
            )::float8 AS gross_revenue
        FROM scoped
        GROUP BY channel, source
        "#,
    )
    .bind(from)
    .bind(to)
    .fetch_all(pool)
    .await?;

    Ok(rows
        .iter()
        .map(|row| ChannelSourceRow {
            channel: row.try_get::<Option<String>, _>("channel").unwrap_or(None),
            source: row.try_get::<Option<String>, _>("source").unwrap_or(None),
            bookings: row.try_get::<i64, _>("bookings").unwrap_or(0),
            cancelled: row.try_get::<i64, _>("cancelled").unwrap_or(0),
            holds_auto_released: row.try_get::<i64, _>("holds_auto_released").unwrap_or(0),
            room_nights: row.try_get::<i64, _>("room_nights").unwrap_or(0),
            gross_revenue: row.try_get::<f64, _>("gross_revenue").unwrap_or(0.0),
        })
        .collect())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn row(channel: Option<&str>, source: Option<&str>, bookings: i64) -> ChannelSourceRow {
        ChannelSourceRow {
            channel: channel.map(str::to_string),
            source: source.map(str::to_string),
            bookings,
            cancelled: 0,
            holds_auto_released: 0,
            room_nights: bookings * 2,
            gross_revenue: bookings as f64 * 1000.0,
        }
    }

    // ---- classify: the four buckets ------------------------------------

    #[test]
    fn loyalty_channel_is_the_app_bucket() {
        assert_eq!(classify(Some("loyalty"), None), ChannelBucket::App);
        // The canonical constant and the literal must not drift apart.
        assert_eq!(classify(Some(LOYALTY_CHANNEL), None), ChannelBucket::App);
    }

    #[test]
    fn named_ota_channel_buckets_by_name() {
        assert_eq!(
            classify(Some("agoda"), None),
            ChannelBucket::Ota("agoda".to_string())
        );
        assert_eq!(classify(Some("agoda"), None).label(), "Agoda");
        assert_eq!(
            classify(Some("bookingcom"), None).label(),
            "Booking.com".to_string()
        );
    }

    #[test]
    fn walk_in_phone_and_line_are_direct() {
        // No channel recorded + an explicitly direct source.
        assert_eq!(classify(None, Some("walk-in")), ChannelBucket::Direct);
        assert_eq!(classify(None, Some("phone")), ChannelBucket::Direct);
        assert_eq!(classify(None, Some("line")), ChannelBucket::Direct);
        assert_eq!(classify(None, Some("online")), ChannelBucket::Direct);
        // A direct spelling that landed in `book_channel` instead — the B6
        // chip renders nothing for these, and neither do we bucket them OTA.
        assert_eq!(classify(Some("walkin"), None), ChannelBucket::Direct);
        assert_eq!(classify(Some("desk"), Some("ota")), ChannelBucket::Direct);
    }

    #[test]
    fn nothing_recorded_is_unknown() {
        assert_eq!(classify(None, None), ChannelBucket::Unknown);
        assert_eq!(classify(Some("  "), Some("")), ChannelBucket::Unknown);
    }

    #[test]
    fn legacy_sync_rows_are_unknown_not_direct() {
        // Every booking the CT sync discovers in iHOTEL carries this source
        // and no channel. iHOTEL cannot say where the guest came from, so
        // neither can we — see `classify`'s deviation note.
        assert_eq!(
            classify(None, Some(SOURCE_LEGACY_APP)),
            ChannelBucket::Unknown
        );
        assert!(!classify(None, Some(SOURCE_LEGACY_APP)).is_direct_share());
    }

    // ---- classify: the two edge cases the brief called out --------------

    #[test]
    fn unmapped_ota_slug_still_buckets_as_that_ota() {
        // A slug we have no pretty label for must NOT fall into `unknown` —
        // it is a perfectly good channel, just a new one.
        let bucket = classify(Some("hostelworld"), None);
        assert_eq!(bucket, ChannelBucket::Ota("hostelworld".to_string()));
        assert_eq!(bucket.kind(), "ota");
        assert_eq!(bucket.label(), "hostelworld");
        assert!(!bucket.is_direct_share());
    }

    #[test]
    fn pre_076_ota_rows_bucket_as_unnamed_ota() {
        // Written before `book_channel` was wired: we know it is an OTA, we
        // do not know which. Must not count toward the direct share.
        let bucket = classify(None, Some("ota"));
        assert_eq!(bucket, ChannelBucket::Ota("ota".to_string()));
        assert_eq!(bucket.label(), "OTA");
        assert!(!bucket.is_direct_share());
    }

    #[test]
    fn pre_076_named_ota_source_keeps_its_name() {
        assert_eq!(
            classify(None, Some("Agoda")),
            ChannelBucket::Ota("agoda".to_string())
        );
    }

    // ---- classify: normalisation + precedence --------------------------

    #[test]
    fn channel_is_trimmed_and_case_folded() {
        assert_eq!(classify(Some("  Agoda "), None).key(), "agoda");
        assert_eq!(classify(Some(" LOYALTY "), None), ChannelBucket::App);
    }

    #[test]
    fn channel_wins_over_a_stale_source() {
        // A loyalty booking whose `book_source` still says `ota` is an app
        // booking — same precedence the reservations chip uses.
        assert_eq!(classify(Some("loyalty"), Some("ota")), ChannelBucket::App);
        assert_eq!(
            classify(Some("agoda"), Some("walk-in")),
            ChannelBucket::Ota("agoda".to_string())
        );
    }

    #[test]
    fn loyalty_source_without_a_channel_is_still_the_app() {
        // `ChannelService::create_hold` writes both columns; this is the
        // defensive case where only the source survived.
        assert_eq!(classify(None, Some("loyalty")), ChannelBucket::App);
    }

    // ---- rollup --------------------------------------------------------

    #[test]
    fn rollup_groups_merges_and_orders_buckets() {
        let rows = vec![
            row(Some("loyalty"), Some("loyalty"), 2),
            row(Some("agoda"), None, 5),
            row(Some("AGODA"), Some("ota"), 1), // same bucket, different casing
            row(None, Some("walk-in"), 3),
            row(None, Some("phone"), 1), // merges into direct
            row(None, Some("legacy_app"), 10),
            row(Some("bookingcom"), None, 4),
        ];

        let out = rollup(&rows);

        let keys: Vec<&str> = out.buckets.iter().map(|b| b.key.as_str()).collect();
        // app first, OTAs alphabetically, then direct, then unknown.
        assert_eq!(
            keys,
            vec!["app", "agoda", "bookingcom", "direct", "unknown"]
        );

        let agoda = &out.buckets[1];
        assert_eq!(agoda.totals.bookings, 6);
        assert_eq!(agoda.totals.room_nights, 12);

        let direct = out.buckets.iter().find(|b| b.key == "direct").unwrap();
        assert_eq!(direct.totals.bookings, 4);

        assert_eq!(out.totals.bookings, 26);
        assert_eq!(out.totals.room_nights, 52);
    }

    #[test]
    fn direct_share_counts_app_plus_direct_over_everything() {
        let rows = vec![
            row(Some("loyalty"), None, 1),    // 2 room-nights
            row(None, Some("walk-in"), 2),    // 4 room-nights
            row(Some("agoda"), None, 5),      // 10 room-nights
            row(None, Some("legacy_app"), 2), // 4 room-nights
        ];

        let out = rollup(&rows);

        // (2 + 4) / 20 = 30.0%
        assert_eq!(out.direct_share, 30.0);
        // (1 + 2) / 10 = 30.0%
        assert_eq!(out.direct_share_by_bookings, 30.0);
        assert_eq!(out.direct_share_by_revenue, 30.0);
    }

    #[test]
    fn direct_share_is_zero_not_nan_on_an_empty_window() {
        let out = rollup(&[]);
        assert_eq!(out.buckets, vec![]);
        assert_eq!(out.totals, ChannelTotals::default());
        assert_eq!(out.direct_share, 0.0);
        assert_eq!(out.direct_share_by_bookings, 0.0);
        assert_eq!(out.direct_share_by_revenue, 0.0);
    }

    #[test]
    fn cancelled_bookings_count_but_sell_no_nights() {
        let rows = vec![ChannelSourceRow {
            channel: Some("agoda".to_string()),
            source: None,
            bookings: 10,
            cancelled: 3,
            holds_auto_released: 0,
            room_nights: 14, // already excludes the 3 cancelled
            gross_revenue: 7000.0,
        }];

        let out = rollup(&rows);
        assert_eq!(out.totals.bookings, 10);
        assert_eq!(out.totals.cancelled, 3);
        assert_eq!(out.totals.room_nights, 14);
    }

    #[test]
    fn direct_share_rounds_to_one_decimal() {
        // 1 of 3 room-nights = 33.333…%
        let rows = vec![
            ChannelSourceRow {
                channel: None,
                source: Some("walk-in".to_string()),
                bookings: 1,
                cancelled: 0,
                holds_auto_released: 0,
                room_nights: 1,
                gross_revenue: 0.0,
            },
            ChannelSourceRow {
                channel: Some("agoda".to_string()),
                source: None,
                bookings: 1,
                cancelled: 0,
                holds_auto_released: 0,
                room_nights: 2,
                gross_revenue: 0.0,
            },
        ];

        assert_eq!(rollup(&rows).direct_share, 33.3);
    }

    #[test]
    fn bucket_kinds_are_stable_strings() {
        assert_eq!(ChannelBucket::App.kind(), "app");
        assert_eq!(ChannelBucket::Ota("agoda".into()).kind(), "ota");
        assert_eq!(ChannelBucket::Direct.kind(), "direct");
        assert_eq!(ChannelBucket::Unknown.kind(), "unknown");
    }
}
