#!/usr/bin/env bash
# run.sh — Phase 1 of the legacy spike.
#
# Runs:
#   00 prereqs
#   01 schema dump
#   02 baseline rowcount snapshot (BEFORE)
#   03 create XE session
#   04 start XE session
#
# Then prints "READY — hand the playbook to the receptionist."
#
# Usage:
#   DB_HOST=192.168.100.222 DB_USER=sa DB_PASS=... DB_NAME=db ./run.sh
#
# Output goes to ./captured-YYYY-MM-DD-HHMM/ in the cwd.

set -euo pipefail

: "${DB_HOST:?Set DB_HOST (e.g. 192.168.100.222)}"
: "${DB_USER:?Set DB_USER (e.g. sa)}"
: "${DB_PASS:?Set DB_PASS}"
: "${DB_NAME:=master}"   # 00-prereqs queries cross-DB; later scripts work in the legacy DB context per their own filter

cli=""
if   command -v sqlcmd    >/dev/null 2>&1; then cli="sqlcmd"
elif command -v mssql-cli >/dev/null 2>&1; then cli="mssql-cli"
else
  echo "ERROR: neither sqlcmd nor mssql-cli is installed." >&2
  echo "  macOS: brew install sqlcmd     (or)  pip install mssql-cli" >&2
  exit 1
fi
echo "Using client: $cli"

ts="$(date +%Y-%m-%d-%H%M)"
out="captured-${ts}"
mkdir -p "$out"
echo "Output dir: $out"

run_sql() {
  local file="$1" target_db="${2:-$DB_NAME}" outname="$3"
  echo "  -> $file (db=$target_db)"
  if [[ "$cli" == "sqlcmd" ]]; then
    sqlcmd -S "$DB_HOST" -U "$DB_USER" -P "$DB_PASS" -d "$target_db" \
           -C -N -b -W -s '|' -i "$file" -o "$out/$outname"
  else
    mssql-cli -S "$DB_HOST" -U "$DB_USER" -P "$DB_PASS" -d "$target_db" \
              -i "$file" --output-format=tsv > "$out/$outname"
  fi
}

cd "$(dirname "$0")"

echo "[1/5] 00-prereqs.sql"
run_sql 00-prereqs.sql master 00-prereqs.txt

echo "[2/5] 01-baseline-schema.sql"
run_sql 01-baseline-schema.sql "$DB_NAME" 01-schema.txt

echo "[3/5] 02-snapshot-rowcounts.sql (BEFORE)"
run_sql 02-snapshot-rowcounts.sql "$DB_NAME" 02-rowcounts-before.txt

echo "[4/5] 03-xe-create-session.sql"
run_sql 03-xe-create-session.sql master 03-xe-create.txt

echo "[5/5] 04-xe-start.sql"
run_sql 04-xe-start.sql master 04-xe-start.txt

# Save context for finalize.sh to find
cat > "$out/.context" <<EOF
DB_HOST=$DB_HOST
DB_USER=$DB_USER
DB_NAME=$DB_NAME
TIMESTAMP=$ts
EOF

echo
echo "==================================================================="
echo "READY. Capture session is running."
echo "Hand ACTION-PLAYBOOK.md to the receptionist."
echo "When they are done, run:"
echo "  CAPTURE_DIR=$out DB_HOST=$DB_HOST DB_USER=$DB_USER DB_PASS=... ./finalize.sh"
echo "==================================================================="
