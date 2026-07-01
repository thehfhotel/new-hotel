# Legacy write-back test runbook (Task #8)

Coordinated live test of the two **dark legacy-write flags** before enabling them
permanently. These make the NEW app write **into the shared iHOTEL production
database**, so this is a careful, one-action-at-a-time test with reception
watching iHOTEL — not a blind flag flip.

| Flag (backend env) | When on, the new app… | Legacy write |
|---|---|---|
| `GUEST_DOCUMENT_STORAGE_ENABLED` | mirrors a captured ID/passport image into iHOTEL | `INSERT Tb_Save_Image` (`MirrorGuestImage` → `writeback/recipes/save_image.rs`) |
| `TM30_COMPANION_WRITEBACK_ENABLED` | mirrors a companion added in the app into iHOTEL | `INSERT HT_CheckIn_Other_People` (`MirrorCompanion` → `writeback/recipes/companion_people.rs`) |

Both are read at the **enqueue** site in the backend API (`config.rs`), so flipping
one only needs the **backend** container restarted (the writeback worker already
processes any intent it's handed).

The reception-facing half is the in-app form **`/v2/verification` → "ทดสอบเขียนข้อมูลกลับ iHOTEL"** (`writeback_test`). This doc is the IT half.

---

## Pre-flight
- Do it in a **quiet window** (no guest at the counter). Test one site at a time.
- Reception on standby with **iHOTEL open** on the same guest/stay.
- Have this runbook + the reception form open. Test with a **real stay that already exists** (has a legacy `Cin_no`), or one you create in the app.
- **Abort rule:** if iHOTEL errors, hangs, or a save fails at any point → flip the flag back off immediately (below) and record it on the form. Nothing else to undo (no schema change; the flag only gates new enqueues).

## Where the flags live
Temporary test flip (does not survive a redeploy — fine for the test):
```bash
# on evergreen, as deploy/root
cd /home/deploy/new-hotel-production
# add/flip the line in the materialised env, then restart ONLY the backend:
sudo sed -i '/^GUEST_DOCUMENT_STORAGE_ENABLED=/d' .env
echo 'GUEST_DOCUMENT_STORAGE_ENABLED=true' | sudo tee -a .env
docker compose up -d backend        # restart backend with the new env
docker compose logs -f backend | grep -i guest_document   # optional: watch
```
To make it **permanent** after the test passes, add it to the deploy env in the
repo (`docker-compose.yml` backend `environment:` / the deploy env payload) and
ship — do NOT rely on the hand-edited `.env` (the next deploy overwrites it).

---

## Step 1 — Guest image write-back (`GUEST_DOCUMENT_STORAGE_ENABLED`)
1. IT: set `GUEST_DOCUMENT_STORAGE_ENABLED=true`, restart `backend` (above). Tell reception "go".
2. Reception (in the NEW app): do one test check-in and **scan the ID / attach a passport** (or use "สแกนบัตร" on the stay's registration page). Note the check-in's `Cin_no`.
3. Verify the image reached iHOTEL — reception opens iHOTEL's registration report for that stay (photo shows), OR IT runs the SQL below.
4. Confirm iHOTEL still saves/opens normally.
5. Reception records Q2/Q3 on the form.

**Verify SQL (legacy MSSQL — read-only):**
```sql
-- newest images for the test Cin_no; a mirrored ID card = ttype 'บัตรประชาชน'
SELECT id, ttype, cust_no, cin_no, DATALENGTH(pic) AS bytes, pic_date
  FROM Tb_Save_Image
 WHERE cin_no = '<TEST_CIN_NO>'
 ORDER BY id DESC;
```
Also confirm the intent was applied (idempotency ledger, on the legacy server):
```sql
SELECT TOP 5 intent_name, applied_at FROM dbo.ht_writeback_ledger
 WHERE intent_name = 'mirror_guest_image' ORDER BY applied_at DESC;
```

## Step 2 — Companion write-back (`TM30_COMPANION_WRITEBACK_ENABLED`)
1. IT: set `TM30_COMPANION_WRITEBACK_ENABLED=true`, restart `backend`. Tell reception "go".
2. Reception (NEW app): open the stay → **add one companion guest**.
3. Verify it reached iHOTEL — reception opens the stay's ผู้เข้าพักร่วม / TM.30 list in iHOTEL, OR IT runs the SQL.
4. Confirm iHOTEL is fine.
5. Reception records Q4/Q5.

**Verify SQL:**
```sql
SELECT Cin_no, Cin_name, Cin_contry FROM HT_CheckIn_Other_People
 WHERE Cin_no = '<TEST_CIN_NO>' ORDER BY id DESC;

SELECT TOP 5 intent_name, applied_at FROM dbo.ht_writeback_ledger
 WHERE intent_name = 'mirror_companion' ORDER BY applied_at DESC;
```

---

## Verdict
- **Both pass + iHOTEL healthy** → decide whether to make the flags permanent (repo env + ship). Reception marks the form "ผ่าน".
- **Anything off** → roll back now, mark "มีปัญหา", capture the error text on the form + here, and hand back to dev.

## Rollback (instant, no state to clean up)
```bash
cd /home/deploy/new-hotel-production
sudo sed -i 's/^GUEST_DOCUMENT_STORAGE_ENABLED=true/GUEST_DOCUMENT_STORAGE_ENABLED=false/' .env
sudo sed -i 's/^TM30_COMPANION_WRITEBACK_ENABLED=true/TM30_COMPANION_WRITEBACK_ENABLED=false/' .env
docker compose up -d backend
```
Rows already written to `Tb_Save_Image` / `HT_CheckIn_Other_People` during the test
are valid legacy rows (they're what a real iHOTEL capture would create) — leave
them, or delete the specific test `Cin_no` rows if you want a clean slate.

## Notes / hazards
- The image mirror is **idempotent** (writeback ledger keyed on the idempotency
  key), so a retry never double-inserts.
- Companion legacy row has **name + country only** (no ID field — legacy has none).
- iHOTEL allocates ids app-side (MAX+1); if a receptionist reports a save error in
  iHOTEL during the test, check for a concurrent same-table save first (known
  coexistence hazard).
