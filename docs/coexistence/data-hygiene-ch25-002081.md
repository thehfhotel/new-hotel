## CH25-002081 / Room 515 — duplicate detail row

Discovered 2026-05-18 during the cardinality-drift investigation (PR #128 follow-up).

### What

`HT_CheckIn_Ds` for `Cin_No = 'CH25-002081'` has **two rows for the same room with the same arrival timestamp**:

| ds_id | Cin_Room_No | Cin_Room_In | Cin_Room_Out | Cin_Room_Status |
|---|---|---|---|---|
| 20980 | 515 | 2025-06-30 12:06:27 | (one value) | Check-Out |
| 20982 | 515 | 2025-06-30 12:06:27 | (different value) | Check-Out |

This is the **only true-duplicate `HT_CheckIn_Ds` row** across all 2,261 detail rows of the 766 multi-room folios audited on HF Hotel as of 2026-05-18. Estimated rate: 0.04% of detail rows.

### Why it matters

Not a code defect — iHOTEL allows the receptionist to add a folio line manually and does not enforce uniqueness on `(Cin_No, Cin_Room_No, Cin_Room_In)`. The duplicate produces:

- One extra row in `ht_checkin_rooms` once Track B reconciliation propagates it
- An asymmetric financial line (the two rows have different `Cin_Room_Out` and may have different `Cin_Room_Pay_Total`)
- Slightly inflated revenue if both rows are summed by `Cin_Room_Pay_Total` reports

### Verification query

Receptionist or auditor can re-run this to find similar duplicates if the pattern recurs:

```sql
SELECT Cin_No, Cin_Room_No, Cin_Room_In, COUNT(*) AS dupes,
       STRING_AGG(CAST(id AS varchar(20)), ',') AS ds_ids
  FROM HT_CheckIn_Ds
 GROUP BY Cin_No, Cin_Room_No, Cin_Room_In
HAVING COUNT(*) > 1
 ORDER BY Cin_No;
```

### Operator action (recommended)

Pick one of:
- **Keep both, reconcile in iHOTEL**: open the folio in iHOTEL, identify which `Cin_Room_Out` is correct, manually delete the obsolete row via the iHOTEL UI (or NULL out its `Cin_Room_Pay_Total` to neutralise).
- **Or do nothing**: 0.04% rate is well within acceptable revenue-report tolerance; flag only if it recurs.

### Slack message draft (for the operator channel)

```
:eyes: Data hygiene flag — 2026-05-18 audit found exactly 1 duplicate detail row in HT_CheckIn_Ds
across 2,261 audited rows on HF Hotel:

  Cin_No CH25-002081 / room 515 / arrival 2025-06-30 12:06:27
  → HT_CheckIn_Ds.id IN (20980, 20982), same room+arrival, different Cin_Room_Out values.

Probably a receptionist double-tap during folio edit. Not blocking anything — just FYI in case
financial reports for that folio look off. SQL probe to find similar duplicates:
docs/coexistence/data-hygiene-ch25-002081.md
```

### No code-side fix recommended

Adding a UNIQUE constraint on `HT_CheckIn_Ds(Cin_No, Cin_Room_No, Cin_Room_In)` is tempting but **wrong**:
- iHOTEL would error on insert and break the receptionist UI
- Some legitimate flows (rebill, split-stay) may genuinely need the same triple

The canonical PG side already has stronger constraints on `ht_checkin_rooms` (UNIQUE on `(cr_cin_id, cr_room_id)`) which would surface this naturally as a constraint-conflict on apply rather than silently propagate. The cleanest long-term fix is to let the canonical UPSERT-deduplicate behave as designed and accept that legacy retains the duplicate.
