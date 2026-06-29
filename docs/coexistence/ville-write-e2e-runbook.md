# HF Ville write e2e — coordinated live test runbook (task #42)

**Purpose.** The final gate before flipping `HFVILLE_WRITES_ENABLED` on production:
prove that a real HF Ville write performed through the new app lands in the
`hotelville` canonical pool **and** mirrors into HF Ville's shared legacy `HOTEL`
DB (the leg the automated test can't cover), and that **nothing** leaks into the
HF Hotel side. This is a coordinated, reception-aware exercise — it writes real
rows into the legacy DB iHOTEL shares, so run it in a quiet window with reception
informed, and clean up after.

The pure-routing half (rows land in `hotelville`, not `hotelnew`; round binds
`shift_site_id='hfville'`) is already covered automatically by
`hotel-backend/tests/test_ville_write_routing.rs`. This runbook covers the part
that needs a live Ville legacy DB + the writeback worker.

> **Who runs this:** an operator with prod access + reception coordination. Claude
> does **not** flip the prod flag or write to the legacy DB unattended.

---

## 0. Preconditions

- [ ] Branch-safety code deployed (commit `55f4ec8` or later) — confirm the
      running backend image is at/after it.
- [ ] The `writeback-hfville` worker is up (compose service; `NEW_DB_NAME=hotelville`,
      `SITE_ID=hfville`, Ville MSSQL `DB_SERVER=192.168.11.51,1436` / `HOTEL`,
      `WRITEBACK_ENABLED=true`). Check: `ssh evergreen "docker ps | grep writeback-hfville"`.
- [ ] A login that can act on HF Ville (auth is live in prod). Use the authenticated
      `/v2` UI with the **branch switched to HF Ville** — every `/api/*` call then
      carries `?branch=hfville`.
- [ ] Decide the payment path:
      - **Lighter test (HFVILLE_WRITES only):** if iHOTEL already has an **open Ville
        round** (it syncs into `ht_shifts` as `shift_site_id='hfville'`), the new
        app's payment gate will find it, so check-in→charge→pay→checkout works
        **without** `ROUND_WRITEBACK_ENABLED`. Confirm an open Ville round exists:
        `SELECT shift_no FROM ht_shifts WHERE shift_site_id='hfville' AND shift_closed_at IS NULL;`
        (on `hotelville`).
      - **Full test (also exercises round open/close):** additionally requires
        `ROUND_WRITEBACK_ENABLED=true` — that is task **#61** and its own
        shift-boundary coordination. Keep the two flips independent unless you
        intend to validate both at once.

## 1. Flip the flag (deploy-time env, via the pipeline)

`HFVILLE_WRITES_ENABLED` defaults off (unset) — `ville_write_guard` 403s every
`?branch=hfville` mutation until it is `true`. Flip it the same way `AUTH_ENABLED`
was turned on: set it in the backend service environment on evergreen and let the
deploy apply it (do **not** hand-edit the running container — deployment policy).
After deploy, verify:

```
ssh evergreen "docker exec new-hotel-production-backend-1 printenv HFVILLE_WRITES_ENABLED"
# expect: true
```

> Rollback at any point: set it back to `false` (or unset) and redeploy. The guard
> immediately re-blocks Ville mutations; the branch-aware handlers go dormant.

## 2. Snapshot "before" (so leakage is detectable)

On `hotelnew` (HF Hotel) — these MUST NOT change during the test:
```
ssh evergreen "docker exec new-hotel-db psql -U postgres -d hotelnew -tA -c \
 \"SELECT (SELECT count(*) FROM ht_checkins), (SELECT count(*) FROM ht_payments), \
          (SELECT count(*) FROM ht_pos_sales), (SELECT count(*) FROM ht_coupons);\""
```
Note the four counts.

## 3. Perform the Ville write flow (authenticated /v2 UI, branch = HF Ville)

Use an obviously-fake guest name with a unique marker, e.g. `ZZ_E2E_<today>`.
1. **Check-in** a walk-in to a free Ville room (record a small deposit if testing that).
2. **Charge** one POS item to the folio (เพิ่มรายการในบิล).
3. **Take payment** for the balance (ชำระเงิน).
4. **Check-out & settle**.
5. *(full test only)* **open + close a Ville round** around the above.

Capture the new `cin_no` / receipt no the UI shows.

## 4. Verify canonical (`hotelville`) — the write landed here

```
ssh evergreen "docker exec new-hotel-db psql -U postgres -d hotelville -tA -c \
 \"SELECT cin_id, cin_no, cin_guest_name FROM ht_checkins WHERE cin_guest_name LIKE 'ZZ_E2E_%';\""
# repeat for ht_payments (pay_cin_id), ht_pos_sales (sale_cin_id), ht_coupons, and
# ht_shifts (shift_site_id='hfville') for the full test.
```
All expected rows present in `hotelville`.

## 5. Verify NO leak into HF Hotel (`hotelnew`)

Re-run the step-2 count query on `hotelnew`. **The four counts must be unchanged**,
and:
```
ssh evergreen "docker exec new-hotel-db psql -U postgres -d hotelnew -tA -c \
 \"SELECT count(*) FROM ht_checkins WHERE cin_guest_name LIKE 'ZZ_E2E_%';\""
# expect: 0
```

## 6. Verify the legacy-MSSQL mirror — the writeback leg

Give the worker a few seconds, then confirm the outbox drained on `hotelville`:
```
ssh evergreen "docker exec new-hotel-db psql -U postgres -d hotelville -tA -c \
 \"SELECT intent, status, count(*) FROM writeback_jobs \
    WHERE created_at > now() - interval '15 min' GROUP BY 1,2 ORDER BY 1;\""
# expect the new jobs (RecordPosSale, RecordPayment, checkout, OpenRound/CloseRound,
# IssueCoupon as applicable) at status='done', none stuck 'pending'/'failed'.
```
Then confirm the rows mirrored into HF Ville's legacy `HOTEL` DB (password read from
the secret file so it is never echoed):
```
ssh evergreen 'docker run --rm --network host \
  -e SQLCMDPASSWORD="$(cat /home/deploy/secrets/db_password)" \
  --entrypoint /opt/mssql-tools18/bin/sqlcmd mcr.microsoft.com/mssql/server:2022-latest \
  -C -S "192.168.11.51,1436" -U sa -d HOTEL -b -W -Q \
  "SELECT Cin_no, Cus_Name FROM View_CheckIn_Ds WHERE Cus_Name LIKE '\''ZZ_E2E_%'\'';"'
# repeat against the legacy check-in/receipt/round/coupon views as applicable.
```
Expected: the test rows are present in Ville's legacy `HOTEL` (and, by inspection
of HF Hotel's legacy DB if you want belt-and-suspenders, absent there).

## 7. Cleanup / decision

- **Clean up the test data** in BOTH canonical (`hotelville`) and legacy (Ville
  `HOTEL`): void/cancel the receipt + check-out + delete the fake check-in/coupon,
  using the app's void/refund flows where possible (so the writeback un-mirrors
  cleanly) and a targeted DELETE otherwise. Re-run steps 4–6 to confirm removal.
- **If everything was green:** the flip is validated. Decide whether to leave
  `HFVILLE_WRITES_ENABLED=true` (begin Ville dual-use) or flip it back until the
  broader go-live — coordinate with reception. (#42 closes when the flip is made
  with reception's sign-off.)
- **If anything leaked or the mirror failed:** flip `HFVILLE_WRITES_ENABLED` back
  to false, redeploy, capture the failing `writeback_jobs` row(s), and file the
  specifics — do NOT leave the flag on with a known leak.

---

### What this gates vs. what's already proven

| Leg | Covered by |
|---|---|
| `?branch=hfville` write → `hotelville` pool, not `hotelnew` | automated `test_ville_write_routing.rs` (commit-gated) |
| Round binds `shift_site_id='hfville'` | automated test |
| Outbox enqueues into the hotelville outbox | automated test (job-count delta) |
| **Writeback worker mirrors hotelville outbox → Ville legacy `HOTEL`** | **this runbook (live)** |
| **No leak into HF Hotel legacy under real load** | **this runbook (live)** |
