# Mark-clean audit insertion evidence

`MarkRoomClean` clears the room's cleaning flag and attempts an `HT_Housewife`
audit INSERT. Its existing five-minute guard can suppress the audit INSERT even
when the room update succeeds. This observation records that distinction without
changing the guard, maid attribution, SQL write literals, or `MarkRoomDirty`.

## Recorded result

New successful `mark_room_clean` jobs include
`legacy_ids.extra.housewife_inserted`:

- `true`: this execution attempt inserted one audit row.
- `false`: this execution attempt inserted no audit row because its guard matched.
- Missing: the job predates this instrumentation; do not count it as `false`.

The INSERT is followed immediately by `SELECT @@ROWCOUNT` in the same SQL batch
and transaction. No intervening statement can replace the INSERT's row count.
An absent, NULL, or unexpected count is an error and the surrounding transaction
rolls back instead of recording invented suppression evidence.

The warning event `writeback.housewife_insert_suppressed` carries room number and
room ID only. It does not include maid identity, guest details, or SQL text.
It describes a transaction attempt, not proof that COMMIT succeeded; the durable
result is the completed job's `legacy_ids`.

## Interpretation

A suppressed row does not prove that an audit row was lost. For example, a retry
after a lost COMMIT reply can encounter the row written by its previous attempt.
Conversely, a real dirty→clean cycle within five minutes can be suppressed by the
same guard. Inspect the cleaning event sequence before classifying an incident.

Run the following read-only query separately against each property's canonical
database. It counts only completed jobs that actually carry the new boolean:

```sql
SELECT legacy_ids #>> '{extra,housewife_inserted}' AS inserted,
       count(*) AS completed_attempt_results
FROM writeback_jobs
WHERE intent = 'mark_room_clean'
  AND status = 'done'
  AND legacy_ids #>> '{extra,housewife_inserted}' IN ('true', 'false')
GROUP BY 1;
```

Rollback is a code revert through the normal deployment pipeline. Existing JSON
observations remain readable; no schema or configuration change is involved.
