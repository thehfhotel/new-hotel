# Runbook — Git History Rewrite (Phase 6.5)

**Goal**: Scrub historical leaked credentials and large legacy blobs from
git history before the public-flip in Phase 7. After Phase 3 rotates the
live credentials, this rewrite is no longer a security gate (rotated
secrets authenticate against nothing) — but it removes scanner false-
positive noise, reduces clone size, and prevents future "look at this
hardcoded password" embarrassment.

**Status**: design doc, not yet executed. Do AFTER Phase 3 (rotation
verified working), BEFORE Phase 7 (public flip).

**Tool of choice**: `git filter-repo` — modern Python replacement for
the deprecated `git filter-branch` and the now-dated BFG. Officially
recommended in GitHub's own docs.

**Destructive op**: this rewrites every commit hash. Force-pushing
afterwards invalidates every existing clone, PR, and direct commit-SHA
reference. Acceptable here because the repo is private at the time of
rewrite, no external forks exist, and all maintainers can re-clone.

---

## Pre-flight checklist

Hard blockers — if any are false, **DO NOT** start the rewrite.

- [ ] **Phase 3 secret rotation is fully complete**. Specifically:
  - Prod PG `postgres` password rotated on `newdb` cluster
  - MSSQL `sa` password rotated on both HF Hotel + HF Ville (coordinated with the legacy iHOTEL2025 client config)
  - Slack webhook regenerated
  - Any other secret found in audits #101 / #103 rotated
  - At least one full deploy via the new credentials passed (proves rotation took effect)
- [ ] **No PR is currently open against master**. Force-push will invalidate the PR's branch ref.
- [ ] **All maintainers notified**. They will need to re-clone post-rewrite.
- [ ] **Backup**. `git clone --mirror git@github.com:thehfhotel/new-hotel.git ~/backup/new-hotel-pre-rewrite-YYYY-MM-DD.git` — keeps the pre-rewrite state in case anything is lost.
- [ ] **`git filter-repo` installed**. `brew install git-filter-repo` on macOS.

---

## Step 1 — Identify what to scrub (≈30 min)

Build a `replacements.txt` listing every secret + format variation. Use
the audit findings (#101 secret-history scan) as the source. Format:

```
secret==>replacement-marker
```

Each line is one substitution. URL-encoded variants need their own line
(e.g. `REDACTED-pg-2026` for `REDACTED-pg-2026` percent-encoded in
`DATABASE_URL` strings).

```
# /tmp/replacements.txt
REDACTED-pg-2026==><REDACTED-rotated-pg-pw>
REDACTED-pg-2026==><REDACTED-rotated-pg-pw>
REDACTED-sa-pw==><REDACTED-rotated-sa-pw>
REDACTED-ville-2026==><REDACTED-rotated-ville-pw>
REDACTED-jumpbox-pw==><REDACTED-rotated-jumpbox-pw>
```

Verify the list is complete by re-running the secret audit afterwards
(`gitleaks detect --log-opts="--all"`) — it should come back clean.

If the audit found other formats (PEM blocks, API tokens, etc.), use
`literal:` or `regex:` prefixes per `git filter-repo` docs.

---

## Step 2 — Tag the pre-rewrite state (≈2 min)

Belt-and-suspenders insurance:

```bash
cd /path/to/new-hotel
git tag pre-history-rewrite-$(date +%Y-%m-%d) master
git push origin pre-history-rewrite-$(date +%Y-%m-%d)
```

Also keep a local mirror clone (see pre-flight backup step above).

---

## Step 3 — Run the rewrite (≈5–15 min depending on history size)

`git filter-repo` insists on a fresh clone for safety. Do the rewrite
in a working copy, then push back.

```bash
# Fresh clone (filter-repo refuses to operate on a non-fresh clone by default)
cd ~/scratch
git clone --no-local /path/to/new-hotel new-hotel-rewrite
cd new-hotel-rewrite

# Apply the replacements across all branches and tags
git filter-repo --replace-text /tmp/replacements.txt

# Optional: also drop legacy-reference/ from history entirely (35 MB savings)
# Skip this if you want to preserve historical access to the vendor decompile
# via `git checkout <pre-removal-commit> -- legacy-reference/`. Recommended:
# DO drop it — copyright concerns persist in history, not just current tree.
git filter-repo --path legacy-reference --invert-paths

# Verify history was rewritten
git log --all --oneline | head -10
git rev-list --count HEAD   # commit count should be similar (rewrite doesn't drop commits, only edits them)
```

`filter-repo` re-creates `.git/refs/` from scratch. It also disconnects
the remote (`origin`) by design — you re-add it manually before the
push. This is to prevent accidentally force-pushing to the wrong place.

---

## Step 4 — Verify the rewrite (≈10 min)

```bash
# Re-run the secret scanner on the rewritten history
gitleaks detect --log-opts="--all" --redact -v

# Should report 0 findings (or only known-acceptable false positives).

# Confirm replacements were applied
git log --all -p | grep -F "REDACTED-pg-2026" | head -3
# Expected: empty (string is gone)

git log --all -p | grep -F "<REDACTED-rotated-pg-pw>" | head -3
# Expected: shows the replacement marker in the commits where the secret was

# Confirm legacy-reference/ is gone if you ran the optional step
git log --all --diff-filter=D --name-only | grep -c legacy-reference
# Number doesn't really matter — what matters is the next check:
git log --all -- legacy-reference/ | head -1
# Expected: empty (no commits touch that path anymore)

# Run the full test suite to confirm nothing broke
pnpm test:components
cd hotel-backend && cargo check --workspace
```

---

## Step 5 — Force-push (destructive; ≈2 min) — **POINT OF NO RETURN**

```bash
# Re-add origin (filter-repo removed it as a safety measure)
git remote add origin git@github.com:thehfhotel/new-hotel.git

# Push every branch + tag, force-with-lease
git push --force-with-lease origin --all
git push --force-with-lease origin --tags

# Push the pre-rewrite tag too (so it's recoverable from origin)
git push origin pre-history-rewrite-YYYY-MM-DD
```

`--force-with-lease` is safer than `--force`: it refuses to push if
someone else has pushed to master in the meantime. If it complains, that
means someone (you, on another machine?) pushed during the rewrite —
investigate before retrying.

---

## Step 6 — Have all maintainers re-clone (≈5 min each)

Send each maintainer this snippet:

```bash
cd ~/path/to/working-copy
git fetch origin
git rev-parse origin/master   # remember this hash
cd ..
mv new-hotel new-hotel.pre-rewrite-backup
git clone git@github.com:thehfhotel/new-hotel.git new-hotel
cd new-hotel
# Verify you have the rewritten history
git log --oneline | head -3
```

`git pull` on an existing clone WILL NOT WORK after history rewrite —
the local commits are now unrelated to remote commits, and merging
them produces a Frankenstein history. Re-clone is the only safe path.

---

## Step 7 — Cleanup (≈10 min)

After 1–2 days of confirming the rewrite hasn't broken anything:

```bash
# Delete the pre-rewrite tag locally + remotely (keep the local mirror backup,
# remove the public tag once you're sure)
git tag -d pre-history-rewrite-YYYY-MM-DD
git push origin --delete pre-history-rewrite-YYYY-MM-DD
```

Keep the local mirror backup (`~/backup/new-hotel-pre-rewrite-*.git`)
indefinitely — disk is cheap, recovery from total disaster is priceless.

GitHub's web UI may show stale references for a few hours (PRs, commit
links) — these clear themselves as the search index re-builds.

---

## Rollback

If something terrible happens between step 5 and step 6:

```bash
# Restore from the local mirror backup
cd /path/to/restore-location
git clone --mirror ~/backup/new-hotel-pre-rewrite-YYYY-MM-DD.git
cd new-hotel-pre-rewrite-YYYY-MM-DD.git
git push --mirror git@github.com:thehfhotel/new-hotel.git
```

This pushes the entire pre-rewrite state back to origin (force overwriting
the broken rewrite). Combine with `git push --force` to override branch
protection rules if needed (you should be the only one who has push access
at that moment anyway).

---

## What this DOESN'T fix

- **Already-harvested secrets**: if scanner bots crawled the public repo
  before the rewrite (n/a here — repo will be rewritten BEFORE flipping
  public), or if the rotation in Phase 3 was incomplete, those credentials
  are still compromised regardless of what `filter-repo` does. Phase 3
  rotation is the actual security action; the rewrite is hygiene.
- **External clones / mirrors**: anyone who cloned the repo before the
  rewrite still has the old history locally. Same caveat as
  "already-harvested secrets" — in our case, only maintainer laptops have
  clones; we control all of them.
- **GitHub's own indexing**: GitHub may keep some stale internal references
  (e.g., for old PRs or issue links). Their support can purge these on
  request, but they're not security-impacting (they don't expose the old
  blobs).

---

## What about BFG?

BFG (the older Java-based alternative) does the same job. Reasons we prefer
`git filter-repo`:

- Modern, actively maintained
- More expressive replacement syntax (regex, callbacks)
- Officially recommended by GitHub
- No JVM dependency

If you already have BFG muscle memory and don't want to install
`git-filter-repo`, BFG works fine for the password-scrubbing case:

```bash
java -jar bfg.jar --replace-text /tmp/replacements.txt path/to/repo.git
```

Same outcome. Pick whichever you're comfortable with.

---

## Integration with the master plan

This runbook fits as **Phase 6.5** in the master plan:

- Phase 0–4: secret rotation (Phase 3) is the prerequisite
- Phase 5: CI workflow refactor (already done — Phase 1)
- Phase 6: CI migration (already done — Phase 1)
- **Phase 6.5: this runbook** (~30 min execution + 1–2 day soak)
- Phase 7: pre-flip checks (re-run secret audit, expect clean)
- Phase 8: flip to public

Do not run this runbook unless you're committed to going public soon
after — the cost of force-pushing the rewrite, having every maintainer
re-clone, and breaking PR/commit links is only worth paying when it
unblocks the public flip.
