#!/usr/bin/env bash
# finalize.sh — Phase 2 of the legacy spike. Run after the receptionist is done.
#
# Runs:
#   05 stop XE session
#   06 baseline rowcount snapshot (AFTER)
#   07 dump XE events out as a tsv
#   (optional 08 drop session — disabled by default; pass DROP=1 to enable)
#
# Usage:
#   CAPTURE_DIR=captured-2026-04-24-1530 \
#   DB_HOST=192.168.100.222 DB_USER=sa DB_PASS=... DB_NAME=db \
#   ./finalize.sh

set -euo pipefail

: "${CAPTURE_DIR:?Set CAPTURE_DIR (the captured-* folder created by run.sh)}"
: "${DB_HOST:?Set DB_HOST}"
: "${DB_USER:?Set DB_USER}"
: "${DB_PASS:?Set DB_PASS}"
: "${DB_NAME:=master}"
DROP="${DROP:-0}"

[[ -d "$CAPTURE_DIR" ]] || { echo "ERROR: $CAPTURE_DIR not found" >&2; exit 1; }

cli=""
if   command -v sqlcmd    >/dev/null 2>&1; then cli="sqlcmd"
elif command -v mssql-cli >/dev/null 2>&1; then cli="mssql-cli"
else
  echo "ERROR: install sqlcmd or mssql-cli first" >&2; exit 1
fi

run_sql() {
  local file="$1" target_db="${2:-$DB_NAME}" outname="$3"
  echo "  -> $file (db=$target_db)"
  if [[ "$cli" == "sqlcmd" ]]; then
    sqlcmd -S "$DB_HOST" -U "$DB_USER" -P "$DB_PASS" -d "$target_db" \
           -C -N -b -W -s '|' -i "$file" -o "$CAPTURE_DIR/$outname"
  else
    mssql-cli -S "$DB_HOST" -U "$DB_USER" -P "$DB_PASS" -d "$target_db" \
              -i "$file" --output-format=tsv > "$CAPTURE_DIR/$outname"
  fi
}

cd "$(dirname "$0")"

echo "[1/3] 05-xe-stop.sql"
run_sql 05-xe-stop.sql master 05-xe-stop.txt

echo "[2/3] 06-snapshot-rowcounts.sql (AFTER)"
run_sql 06-snapshot-rowcounts.sql "$DB_NAME" 06-rowcounts-after.txt

echo "[3/3] 07-xe-read.sql (parsed events)"
run_sql 07-xe-read.sql master 07-events.txt

if [[ "$DROP" == "1" ]]; then
  echo "[bonus] 08-xe-drop-session.sql (DROP=1)"
  run_sql 08-xe-drop-session.sql master 08-xe-drop.txt
else
  echo "(Skipping 08-drop-session — pass DROP=1 to also remove the session.)"
fi

# Compute row count diffs as a quick win — full content diff still recommended
echo
echo "==== TABLES THAT CHANGED (row_count delta or checksum mismatch) ===="
python3 - "$CAPTURE_DIR/02-rowcounts-before.txt" "$CAPTURE_DIR/06-rowcounts-after.txt" <<'PY' || true
import sys, csv, re
from pathlib import Path

def parse(path):
    rows = {}
    for line in Path(path).read_text(errors='replace').splitlines():
        parts = [p.strip() for p in re.split(r'\s*\|\s*|\t', line) if p.strip()]
        if len(parts) >= 3 and not parts[0].startswith('-') and parts[0].lower() != 'table_name':
            try:
                rows[parts[0]] = (int(parts[1]), parts[2])
            except ValueError:
                pass
    return rows

before, after = parse(sys.argv[1]), parse(sys.argv[2])
print(f"{'table':40} {'before':>10} {'after':>10} {'delta':>10}  checksum")
for name in sorted(set(before) | set(after)):
    b = before.get(name, (0, '-'))
    a = after.get(name, (0, '-'))
    if b[0] != a[0] or b[1] != a[1]:
        delta = a[0] - b[0]
        cks = "DIFF" if b[1] != a[1] else "same"
        print(f"{name:40} {b[0]:>10} {a[0]:>10} {delta:>+10}  {cks}")
PY

echo
echo "Done. Files in $CAPTURE_DIR:"
ls -la "$CAPTURE_DIR"
echo
echo "NEXT: copy $CAPTURE_DIR off the SQL server, share for analysis."
echo "      Also grab the .xel file from SQL Server's LOG dir if you want raw data."
