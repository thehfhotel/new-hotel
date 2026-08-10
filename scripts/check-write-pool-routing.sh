#!/usr/bin/env bash
# check-write-pool-routing.sh — enforce the unified per-site write chokepoint.
#
# Ship-B / ADR 0002. Every MUTATING route handler must resolve its PostgreSQL
# pool from the request's `?branch=` through the ONE chokepoint
# (`AppState::write_pool` / the per-module helpers that delegate to it), NOT by
# referencing `state.new_pool` directly. A direct `state.new_pool` in a mutating
# handler is the risk R2 footgun: once `HFVILLE_WRITES_ENABLED` is flipped, a
# `branch=hfville` write would silently land in the HF Hotel pool instead of the
# `hotelville` pool.
#
# This was reviewer discipline only; this script makes it a CI failure so the
# next mutating handler can't reintroduce the footgun unnoticed. Modeled on
# `scripts/check-cardinality-map.sh` (same `--self-test` + `::error::` shape).
#
# ## Scope
#
# The "mutating handler" universe = every `routes::<module>::<handler>` mounted
# in `hotel-backend/src/main.rs` under `post(` / `put(` / `patch(` / `delete(`
# (this also matches the `axum::routing::post(...)` long form). For each such
# handler this script extracts its function body from
# `hotel-backend/src/routes/<module>.rs` and FAILS if the body references
# `state.new_pool` / `&state.new_pool` directly — unless the handler is on the
# explicit allowlist below (each entry carries a one-line reason).
#
# ## Known limitation (documented on purpose)
#
# This is a TEXT check for DIRECT `state.new_pool` use. Handlers that write via a
# pre-wired `state.*_service` (bound to `new_pool` at startup) are a SEPARATE R2
# surface — they don't textually mention `new_pool`, so this gate can't see them.
# The fix for those is `AppState::resolve_write_services(branch)` (rebuilds the
# service graph on the branch pool). The bookings/payments/checkins create +
# checkout + refund flows have now been converted that way (Ville bundle) and are
# no longer allowlisted; any still-service-bound handler that doesn't reference
# `new_pool` (e.g. `cancel_booking`) remains out of this gate's matching scope.
#
# ## Behavior
#
# Exits 0 when every mutating handler routes through the chokepoint (or is
# allowlisted). Exits 1 (with `::error::` diagnostics) otherwise.
#
# ## Self-tests
#
# `--self-test` runs the PASS case (handler uses the chokepoint) and the FAIL
# case (handler references `state.new_pool`) in a scratch dir. CI invokes the
# same script with no args.

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "${SCRIPT_DIR}/.." && pwd)"

ROUTES_DIR_DEFAULT="${REPO_ROOT}/hotel-backend/src/routes"
MAIN_RS_DEFAULT="${REPO_ROOT}/hotel-backend/src/main.rs"

# Allow override for self-tests.
ROUTES_DIR="${ROUTES_DIR:-${ROUTES_DIR_DEFAULT}}"
MAIN_RS="${MAIN_RS:-${MAIN_RS_DEFAULT}}"

# ----------------------------------------------------------------------
# Allowlist — `module::handler` entries exempt from the chokepoint rule.
# EVERY entry MUST carry a one-line reason. Two legitimate classes:
#   (A) HF-Hotel-only-by-design / global handlers with no per-site Ville
#       variant (Ville is guarded anyway), and
#   (B) service-bound core flows whose proper Ville support needs
#       `resolve_write_services` + a `?branch=` param. The bookings/payments/
#       checkins create + checkout + refund flows have now been converted that
#       way (Ville bundle) and removed from this list; no (B) entries remain.
# ----------------------------------------------------------------------
is_allowlisted() {
    case "$1" in
        # (A) Global auth/session store — single users+sessions table, never
        # per-site. login/logout are not branch-routed.
        auth::login) return 0 ;;
        auth::logout) return 0 ;;
        # (A) Cloudflare Access auto-login — same global users+sessions
        # store as auth::login; sessions are never per-site.
        auth::cf_login) return 0 ;;
        # (A) NFC staff-card login — mints the SAME global users+sessions store
        # as auth::login/cf_login (sessions are never per-site). HF-Hotel front-
        # desk only, no per-site Ville variant. (The badge auto-provision into
        # the global ht_users table now happens in reader::wait, a GET — out of
        # this gate's post/put/patch/delete scope; reader::claim is a POST but
        # only calls central HF-ID + the in-memory store, never new_pool, so it
        # passes on its own merits without an allowlist entry.)
        auth::card_login) return 0 ;;
        # (A) Admin user administration — writes the SAME global `ht_users`
        # store as auth::login above (CARDINALITY_MAP: `ht_users` is PG
        # canonical with no legacy twin and no per-site split), so `new_pool`
        # is the correct target and there is no `?branch=` on these routes.
        #
        # NEWLY VISIBLE 2026-08-11, not newly written: both handlers live in
        # `routes::admin_users::router()`, and until this script learned to
        # scan module-local routers they were silently outside its universe.
        # Listed explicitly so the classification is recorded rather than
        # implicit. If PMS accounts ever become per-site, these are the first
        # two handlers to revisit.
        admin_users::create_user) return 0 ;;
        admin_users::patch_user) return 0 ;;
        # (A) Legacy `/api/bookings/by-number/{book_no}/notes` path — HF-Hotel-
        # only legacy booking-notes surface; the by-number lookups read new_pool
        # and there is no Ville variant of this path. TODO(ville-bundle).
        bookings::create_note) return 0 ;;
        bookings::delete_note) return 0 ;;
        # (A) DEPRECATED `ht_rates` write path — superseded by the branch-aware
        # rate-tier endpoints (create_rate_tier/update_rate_tier route through
        # the chokepoint via `pool_and_site`). Kept only for back-compat.
        new_rates::create_rate) return 0 ;;
        new_rates::update_rate) return 0 ;;
        new_rates::delete_rate) return 0 ;;
        # (A) Inventory is an HF-Hotel-native canonical feature — no per-site
        # Ville exposure (`list_inventory_rooms` already returns empty for
        # branch=hfville by design). No Ville variant to misroute into.
        new_inventory::create_item) return 0 ;;
        new_inventory::update_item) return 0 ;;
        new_inventory::delete_item) return 0 ;;
        new_inventory::update_room_inventory) return 0 ;;
        new_inventory::check_room_inventory) return 0 ;;
        new_inventory::replenish_room_inventory) return 0 ;;
        new_inventory::create_stock_adjustment) return 0 ;;
        # (Product master + maintenance-request writers were (A) entries here.
        # They are now branch-aware — routed through `resolve_pool` →
        # `state.write_pool(branch)` (the Ville pool when `?branch=hfville`) — so
        # they pass this gate on their own merits and were removed from the
        # allowlist, pre-flip hardening for HFVILLE_WRITES_ENABLED.)
        # (C) Internal reception-verification feedback — NOT a coexistence write.
        # Stored in the primary DB regardless of branch (the branch is recorded as
        # `vr_site` data, not a routing key) so reception can submit from ANY branch
        # incl. HF Ville without `ville_write_guard` blocking it. Deliberately uses
        # state.new_pool directly.
        new_verification::create_verification) return 0 ;;
        # (B) — the former service-bound core flows (bookings create/update,
        # payments create/refund, checkins create/checkout) have been CONVERTED
        # to resolve their service graph + pool per-site via
        # `AppState::resolve_write_services(branch)` + `state.write_pool(branch)`
        # (Ville bundle). They no longer reference `state.new_pool` directly, so
        # they pass this gate on their own merits and are no longer allowlisted.
        *) return 1 ;;
    esac
}

# Extract `module::handler` for every mutating route mount in main.rs. Matches
# `post(routes::m::h` / `.put(routes::m::h` / `axum::routing::patch(routes::m::h`
# / `delete(routes::m::h`. `get(` is intentionally excluded (read-only).
extract_mutating_handlers_from_main() {
    local main="$1"
    if [ ! -f "${main}" ]; then
        echo "::error::main.rs not found: ${main}" >&2
        exit 2
    fi
    grep -oE '(post|put|patch|delete)\([[:space:]]*routes::[a-zA-Z0-9_]+::[a-zA-Z0-9_]+' "${main}" \
        | sed -E 's/^(post|put|patch|delete)\([[:space:]]*routes:://' \
        | sort -u
}

# Extract `module::handler` for mutating routes registered INSIDE a route
# module's own `router()` fn, e.g. `routes/hk.rs`:
#
#     pub fn router(state: AppState) -> Router {
#         Router::new().route("/api/hk/rooms/{id}/cleaning", post(report_cleaning))
#
# Those handlers are referenced by BARE name (they are in scope), so the
# main.rs matcher above cannot see them — main.rs only mounts
# `routes::<module>::router(...)`. Without this pass, moving a route out of
# main.rs into a module router would SILENTLY drop it from this gate's
# universe (which is exactly what happened when the `/api/hk/*` stack moved
# into `routes::hk::router`, and is a long-standing blind spot for
# `routes::admin_users::router`). Handlers are attributed to the module whose
# file they were found in.
extract_mutating_handlers_from_route_modules() {
    local routes_dir="$1"
    [ -d "${routes_dir}" ] || return 0
    local file module
    for file in "${routes_dir}"/*.rs; do
        [ -f "${file}" ] || continue
        module="$(basename "${file}" .rs)"
        [ "${module}" = "mod" ] && continue
        # Bare-name registrations only: a `routes::m::h` form here would be
        # the main.rs shape and is already covered above.
        # `|| true`: most route files register nothing this way, and a
        # no-match grep exits 1 — which would abort the whole script under
        # `set -euo pipefail`.
        grep -oE '(post|put|patch|delete)\([[:space:]]*[a-z][a-zA-Z0-9_]*[[:space:]]*\)' "${file}" 2>/dev/null \
            | sed -E 's/^(post|put|patch|delete)\([[:space:]]*//; s/[[:space:]]*\)$//' \
            | while IFS= read -r handler; do
                [ -z "${handler}" ] && continue
                echo "${module}::${handler}"
            done || true
    done | sort -u
}

# The full universe: main.rs mounts + module-local `router()` registrations.
extract_mutating_handlers() {
    local main="$1"
    {
        extract_mutating_handlers_from_main "${main}"
        extract_mutating_handlers_from_route_modules "${ROUTES_DIR}"
    } | sort -u
}

# Print the body of a top-level `fn <handler>` from a route file: from the fn
# signature line up to (but not including) the next top-level item (next fn /
# doc-comment / attribute / struct|enum|impl|const|static|mod|trait|type). Route
# handler bodies are indented, so a column-0 item reliably bounds the body.
extract_fn_body() {
    local file="$1" fn="$2"
    awk -v fn="${fn}" '
        BEGIN { incap = 0 }
        # Start: the handler fn definition (optionally `pub` / `async`).
        $0 ~ ("^(pub )?(async )?fn " fn "\\(") { incap = 1; print; next }
        incap == 1 {
            if ($0 ~ /^(pub )?(async )?fn / \
                || $0 ~ /^\/\/\// \
                || $0 ~ /^#\[/ \
                || $0 ~ /^(pub )?(struct|enum|impl|const|static|mod|trait|type) /) {
                incap = 0
                next
            }
            print
        }
    ' "${file}"
}

run_check() {
    local handlers
    handlers="$(extract_mutating_handlers "${MAIN_RS}")"

    if [ -z "${handlers}" ]; then
        echo "::error::No mutating handlers found in ${MAIN_RS} — the matcher is broken." >&2
        return 2
    fi

    local checked=0
    local skipped=0
    local violations=()

    while IFS= read -r entry; do
        [ -z "${entry}" ] && continue
        local module="${entry%%::*}"
        local handler="${entry##*::}"
        local file="${ROUTES_DIR}/${module}.rs"

        if [ ! -f "${file}" ]; then
            # A mount pointing at a non-existent module file is itself a bug.
            violations+=("${entry} (route module file not found: ${file})")
            continue
        fi

        if is_allowlisted "${entry}"; then
            skipped=$((skipped + 1))
            continue
        fi

        checked=$((checked + 1))
        local body
        body="$(extract_fn_body "${file}" "${handler}")"
        if [ -z "${body}" ]; then
            violations+=("${entry} (could not locate fn body in ${file})")
            continue
        fi
        if printf '%s\n' "${body}" | grep -qE 'state\.new_pool'; then
            violations+=("${entry} (references state.new_pool directly — route via AppState::write_pool / a delegating helper)")
        fi
    done <<< "${handlers}"

    if [ ${#violations[@]} -gt 0 ]; then
        echo "::error::Mutating handlers must resolve their pool through the per-site write chokepoint (AppState::write_pool), not state.new_pool directly:"
        local v
        for v in "${violations[@]}"; do
            echo "::error::  - ${v}"
        done
        echo "::error::Fix: \`let pool = state.write_pool(<branch>)?;\` (add a \`?branch=\` query param if the handler lacks one), then use \`pool\`. If the handler is genuinely HF-Hotel-only/global, add it to the allowlist in this script WITH a one-line reason."
        return 1
    fi

    echo "PASS: ${checked} mutating handler(s) route through the per-site write chokepoint (${skipped} allowlisted)."
    return 0
}

# ----------------------------------------------------------------------
# Self-tests (--self-test). Builds a scratch main.rs + route module and asserts
# the PASS case (chokepoint) and the FAIL case (direct state.new_pool).
# ----------------------------------------------------------------------
run_self_tests() {
    local tmp
    tmp="$(mktemp -d)"
    # shellcheck disable=SC2064
    trap "rm -rf '${tmp}'" EXIT

    local routes="${tmp}/routes"
    local main="${tmp}/main.rs"
    mkdir -p "${routes}"

    cat > "${main}" <<'RS'
fn build() -> Router {
    Router::new()
        .route("/api/widgets", get(routes::widgets::list).post(routes::widgets::create))
        .route("/api/widgets/{id}", put(routes::widgets::update))
        // long form must also be caught:
        .route("/api/widgets/{id}/toggle", axum::routing::patch(routes::widgets::toggle))
}
RS

    # PASS-case module: create + update + toggle all go through write_pool; the
    # GET `list` references new_pool but is read-only (not a mutating mount).
    cat > "${routes}/widgets.rs" <<'RS'
pub async fn list(State(state): State<AppState>) -> ApiResult<Json<X>> {
    let pool = &state.new_pool;
    do_read(pool).await
}

pub async fn create(State(state): State<AppState>, Query(q): Query<Q>) -> ApiResult<Json<X>> {
    let pool = state.write_pool(q.branch)?;
    do_write(pool).await
}

pub async fn update(State(state): State<AppState>) -> ApiResult<Json<X>> {
    let pool = state.write_pool(None)?;
    do_write(pool).await
}

pub async fn toggle(State(state): State<AppState>) -> ApiResult<Json<X>> {
    let pool = state.write_pool(None)?;
    do_write(pool).await
}
RS

    echo "[self-test] case 1 — all mutating handlers use the chokepoint → expect PASS"
    if ROUTES_DIR="${routes}" MAIN_RS="${main}" run_check >/dev/null; then
        echo "[self-test] case 1 OK"
    else
        echo "[self-test] case 1 FAILED — expected exit 0 when all handlers route correctly" >&2
        return 1
    fi

    # FAIL-case: `update` now writes state.new_pool directly.
    cat > "${routes}/widgets.rs" <<'RS'
pub async fn list(State(state): State<AppState>) -> ApiResult<Json<X>> {
    let pool = &state.new_pool;
    do_read(pool).await
}

pub async fn create(State(state): State<AppState>, Query(q): Query<Q>) -> ApiResult<Json<X>> {
    let pool = state.write_pool(q.branch)?;
    do_write(pool).await
}

pub async fn update(State(state): State<AppState>) -> ApiResult<Json<X>> {
    let mut tx = state.new_pool.begin().await?;
    do_write_tx(&mut tx).await
}

pub async fn toggle(State(state): State<AppState>) -> ApiResult<Json<X>> {
    let pool = state.write_pool(None)?;
    do_write(pool).await
}
RS

    echo "[self-test] case 2 — update references state.new_pool → expect FAIL"
    local status=0
    ROUTES_DIR="${routes}" MAIN_RS="${main}" run_check >/dev/null 2>&1 || status=$?
    if [ "${status}" -eq 1 ]; then
        echo "[self-test] case 2 OK"
    else
        echo "[self-test] case 2 FAILED — expected exit 1 when a mutating handler uses state.new_pool, got ${status}" >&2
        return 1
    fi

    echo "[self-test] all passed"
    return 0
}

if [ "${1:-}" = "--self-test" ]; then
    run_self_tests
    exit $?
fi

run_check
exit $?
