# Runbook — Deploy CI/CD Modernization (Phase 1)

**Goal**: Replace the legacy SSH-as-root-with-password deploy with industry-standard
SSH-key + restricted user + forced-command pattern, and run CI on GitHub-hosted
runners (reaching evergreen via the existing Cloudflare Access SSH path).

**Status**: design doc, not yet executed. Read end-to-end before starting.

**Context**: The hotel app is internal-only (LAN/WG/CF Access), but the repo is
slated to go public. This phase is independent of the public flip — it improves
deploy security regardless and is the foundation Phase 5 (workflow refactor)
will plug into.

**Why this transport (not Tailscale, not direct SSH, not service tokens)**:
your `asgard` Cloudflare Tunnel routes `evergreen.thehfhotel.org → ssh://<runner-host>`,
and there is **no Access application** gating that hostname (verified via the
CF Access API — only 15 apps exist in the account, none for evergreen). So
your daily `cloudflared access ssh --hostname evergreen.thehfhotel.org` works
purely through the tunnel; Access auth never triggers because there's no app
to trigger it.

The deploy uses the SAME tunnel-only path. No service token, no Access app
changes. SSH-key auth + `restrict` + forced-command on `deploy@evergreen` is
the entire auth model — and that's sufficient because:

- The tunnel hostname is non-trivial to discover (not in DNS until you query it)
- Even if discovered, attacker hits the SSH layer needing a valid ed25519 key
- Forced-command means even with the key they get one script's stdout, nothing else
- `nut`'s daily SSH (existing key) keeps working unchanged

Defense-in-depth via an Access app for evergreen + service token is a future
option — pure additive layer on top of this design, no changes to the deploy
script or workflow.

---

## What changes

| Before | After |
|--------|-------|
| Self-hosted runner is `evergreen` itself | GitHub-hosted ubuntu-latest runners reach evergreen via `cloudflared access ssh` |
| Deploy step runs locally on evergreen as `nut` (uid 1000), full sudo | GitHub runner SSHes as `deploy` user (no sudo, just `docker` group) |
| SSH auth: password (interactive) when needed | SSH auth: ed25519 key only; password auth disabled |
| CF Access in front of evergreen SSH | None today; tunnel-only. Adding one is a future defense-in-depth option, not in scope here |
| Workflow YAML inlines all deploy logic | Deploy logic lives in `/srv/run-deploy.sh` (root-owned, 755) |
| Workflow can shell-out arbitrary commands on prod | Workflow can ONLY trigger the script (forced-command in authorized_keys) |
| Snap-docker flap blocks CI | Same flap, but only affects deploy step; build/test on GH-hosted is reliable |

---

## One-time setup

### 1. (Skipped — no CF Access setup needed)

Your `asgard` tunnel already routes evergreen SSH; no Access app exists for
that hostname so there's nothing to configure on the CF side. If you created
a `gh-actions-new-hotel-deploy` service token earlier, you can delete it
(no policy attached → it does nothing). Proceed to step 2.

### 2. SSH keypair (≈5 min)

On a developer laptop (NOT on evergreen — keep the private key off any
operator-shared host):

```bash
ssh-keygen -t ed25519 -a 100 -f /tmp/evergreen-deploy -C "gh-actions@thehfhotel-new-hotel" -N ""
# /tmp/evergreen-deploy       — private (paste into GH Secret in step 5)
# /tmp/evergreen-deploy.pub   — public (install on evergreen in step 3)
```

After both halves are placed in step 3 + step 5, **delete `/tmp/evergreen-deploy` from the laptop**.

### 3. evergreen — create deploy user, install script, install key (≈30 min)

SSH to evergreen as `nut` via your existing CF Access path. Run as root:

```bash
# 3a. Create the deploy user (system account, no shell login expected normally)
sudo useradd --create-home --shell /bin/bash --comment "GitHub Actions deploy" deploy
sudo usermod -aG docker deploy

# 3b. Move the prod deploy directory under /home/deploy; chown to the new user.
# IMPORTANT: must live under /home/, /media/, or /mnt/ — the snap-confined
# `docker` package on this host CANNOT see /srv/, /opt/, /var/, or arbitrary
# paths. `docker compose pull` from /srv/new-hotel-production silently returns
# "no configuration file provided: not found" even with the file present and
# readable. /home/deploy/ keeps the dir under the deploy user's namespace.
sudo mv /home/nut/new-hotel-production /home/deploy/new-hotel-production
sudo ln -s /home/deploy/new-hotel-production /home/nut/new-hotel-production   # muscle-memory parity
sudo chown -R deploy:deploy /home/deploy/new-hotel-production

# 3c. Install the deploy script (root-owned, mode 755 — workflow can run, can't modify)
sudo install -m 755 -o root -g root \
  /path/to/new-hotel/scripts/deploy/run-deploy.sh \
  /srv/run-deploy.sh

# 3d. Install the public SSH key with forced-command + restrict
sudo mkdir -p /home/deploy/.ssh
sudo chmod 700 /home/deploy/.ssh
sudo tee /home/deploy/.ssh/authorized_keys > /dev/null <<'EOF'
command="/srv/run-deploy.sh",restrict ssh-ed25519 PUBLIC_KEY_FROM_STEP_2 gh-actions@thehfhotel-new-hotel
EOF
sudo chown -R deploy:deploy /home/deploy/.ssh
sudo chmod 600 /home/deploy/.ssh/authorized_keys

# 3e. Disable password + keyboard-interactive SSH (after confirming key auth works for `nut` too!)
# IMPORTANT: do this LAST. Test deploy@ key auth first (step 6), then disable.
#
# Drop a hardening fragment into sshd_config.d/ rather than editing the main config —
# survives package upgrades and avoids regex-fragility around `#?` comment markers.
# Need BOTH PasswordAuthentication AND KbdInteractiveAuthentication disabled;
# leaving the latter on lets PAM-based password auth in via a different code path.
sudo tee /etc/ssh/sshd_config.d/00-hardening.conf > /dev/null <<'EOF'
# GitHub Actions deploy hardening — Phase 1 CI/CD modernization (2026-05)
PasswordAuthentication no
KbdInteractiveAuthentication no
ChallengeResponseAuthentication no
EOF
sudo chmod 644 /etc/ssh/sshd_config.d/00-hardening.conf

# Validate sshd config BEFORE reloading (a syntax error would lock you out)
sudo sshd -t || { echo "sshd config invalid — DO NOT reload"; exit 1; }
sudo systemctl reload sshd

# Verify all three settings actually applied (sshd -T resolves Include/Match)
sudo sshd -T | grep -iE 'passwordauthentication|kbdinteractiveauthentication|challengeresponseauthentication'
# Expected output:
#   passwordauthentication no
#   kbdinteractiveauthentication no
#   challengeresponseauthentication no

# 3f. Logging dir for the deploy script — owned by `deploy` so the script can write
sudo install -d -m 755 -o deploy -g deploy /var/log/deploy
```

The `restrict` keyword in authorized_keys disables port-forwarding, X11,
agent-forwarding, pty, and user-rc. `command="..."` pins the script — the
workflow gets the script's stdout/stderr but can't run anything else.

### 4. Capture evergreen's host key for known_hosts pinning (≈2 min)

From a developer laptop already authenticated to CF Access:

```bash
ssh-keyscan -t ed25519 \
  -o "ProxyCommand=cloudflared access ssh --hostname %h" \
  evergreen.thehfhotel.org 2>/dev/null
# Copy the output line — it goes into the EVERGREEN_HOST_KEY GH Secret in step 5.
```

### 5. GitHub Secrets (≈5 min)

Add these via `gh secret set NAME --body @file` or repo Settings → Secrets:

| Secret | Source | Purpose |
|--------|--------|---------|
| `EVERGREEN_DEPLOY_SSH_KEY` | step 2 (private half) | The ed25519 private key |
| `EVERGREEN_HOST_KEY` | step 4 | Server's pubkey for known_hosts pinning |

Existing secrets (`DB_SERVER`, `DB_PASSWORD`, `POSTGRES_PASSWORD`, etc.) stay
unchanged — they get passed through the JSON payload to the deploy script.

Plan to **rotate** `DB_PASSWORD` and `POSTGRES_PASSWORD` once Phase 3 lands
(those credentials have been logged on the self-hosted runner whose history is
about to become public — assume exposed).

### 6. Test the new flow before flipping the workflow (≈30 min)

From a developer laptop (already authenticated to CF Access via cloudflared),
with the same private key the GH runner will use:

```bash
# Build a payload identical to what the workflow will send
tar -czf /tmp/deploy.tar.gz docker-compose.yml init-db migrations/pg scripts/migrate.sh

jq -n \
  --arg commit_sha "$(git rev-parse HEAD)" \
  --arg deploy_payload_b64 "$(base64 < /tmp/deploy.tar.gz)" \
  --arg DB_SERVER "..." \
  --arg DB_PASSWORD "..." \
  # ... all env vars ...
  '{
    commit_sha: $commit_sha,
    deploy_payload_b64: $deploy_payload_b64,
    env: {
      DB_SERVER: $DB_SERVER,
      DB_PASSWORD: $DB_PASSWORD,
      # ...
    }
  }' > /tmp/payload.json

# Pipe payload to deploy@evergreen via the existing tunnel.
# `cloudflared access ssh --hostname %h` is the same ProxyCommand your daily
# SSH uses — no Access app gates it, the tunnel just routes the connection.
ssh -i /tmp/evergreen-deploy -o StrictHostKeyChecking=accept-new \
  -o ProxyCommand="cloudflared access ssh --hostname %h" \
  deploy@evergreen.thehfhotel.org < /tmp/payload.json
```

You should see the script's stdout streaming back, ending in
`[deploy] done <iso8601> commit=<sha> log=/var/log/deploy/deploy-<timestamp>.log`.

If anything goes wrong, the log is at `/var/log/deploy/deploy-*.log` on
evergreen. SSH to evergreen as `nut` to inspect.

### 7. Workflow refactor (after step 6 passes — Phase 5 of the master plan)

Once the manual flow works end-to-end, refactor `.github/workflows/docker-build.yml`
to use the new pattern. The deploy step becomes (roughly):

```yaml
deploy:
  runs-on: ubuntu-latest   # was: [self-hosted, linux, deploy]
  needs: [changes, test-frontend, test-backend, init-db-migrations-drift-check, build-frontend, build-backend]
  permissions:
    contents: read
    packages: read
  concurrency:
    group: deploy-prod-evergreen
    cancel-in-progress: false
  if: |
    always() &&
    (needs.test-frontend.result == 'success' || needs.test-frontend.result == 'skipped') &&
    (needs.test-backend.result == 'success' || needs.test-backend.result == 'skipped') &&
    (needs.init-db-migrations-drift-check.result == 'success' || needs.init-db-migrations-drift-check.result == 'skipped') &&
    (needs.build-frontend.result == 'success' || needs.build-backend.result == 'success' || (needs.changes.outputs.deploy == 'true' && needs.changes.outputs.frontend != 'true' && needs.changes.outputs.backend != 'true'))
  steps:
    - uses: actions/checkout@93cb6efe18208431cddfb8368fd83d5badbf9bfd  # v5.0.0

    - name: Install cloudflared
      run: |
        curl -fsSL https://github.com/cloudflare/cloudflared/releases/latest/download/cloudflared-linux-amd64 \
          -o /usr/local/bin/cloudflared
        sudo chmod +x /usr/local/bin/cloudflared
        cloudflared --version

    - name: Install SSH key + known_hosts
      env:
        SSH_KEY: ${{ secrets.EVERGREEN_DEPLOY_SSH_KEY }}
        HOST_KEY: ${{ secrets.EVERGREEN_HOST_KEY }}
      run: |
        mkdir -p ~/.ssh && chmod 700 ~/.ssh
        printf '%s\n' "$SSH_KEY"  > ~/.ssh/deploy_key  && chmod 600 ~/.ssh/deploy_key
        printf '%s\n' "$HOST_KEY" > ~/.ssh/known_hosts && chmod 600 ~/.ssh/known_hosts

    - name: Deploy via SSH
      env:
        DB_SERVER: ${{ secrets.DB_SERVER }}
        DB_NAME: ${{ secrets.DB_NAME }}
        DB_USER: ${{ secrets.DB_USER }}
        DB_PASSWORD: ${{ secrets.DB_PASSWORD }}
        MSSQL_PORT: ${{ secrets.MSSQL_PORT }}
        POSTGRES_DB: ${{ secrets.POSTGRES_DB }}
        POSTGRES_USER: ${{ secrets.POSTGRES_USER }}
        POSTGRES_PASSWORD: ${{ secrets.POSTGRES_PASSWORD }}
        SLACK_WEBHOOK_URL: ${{ secrets.SLACK_WEBHOOK }}
        LEGACY_SYNC_ENABLED: ${{ secrets.LEGACY_SYNC_ENABLED }}
        LEGACY_SYNC_SHADOW_MODE: ${{ secrets.LEGACY_SYNC_SHADOW_MODE }}
      run: |
        set -euo pipefail
        tar -czf /tmp/deploy.tar.gz docker-compose.yml init-db migrations/pg scripts/migrate.sh

        jq -n \
          --arg commit_sha "$GITHUB_SHA" \
          --arg deploy_payload_b64 "$(base64 < /tmp/deploy.tar.gz)" \
          --arg DB_SERVER "$DB_SERVER" \
          --arg DB_NAME "$DB_NAME" \
          --arg DB_USER "$DB_USER" \
          --arg DB_PASSWORD "$DB_PASSWORD" \
          --arg MSSQL_PORT "${MSSQL_PORT:-1433}" \
          --arg POSTGRES_DB "$POSTGRES_DB" \
          --arg POSTGRES_USER "$POSTGRES_USER" \
          --arg POSTGRES_PASSWORD "$POSTGRES_PASSWORD" \
          --arg SLACK_WEBHOOK_URL "$SLACK_WEBHOOK_URL" \
          --arg LEGACY_SYNC_ENABLED "$LEGACY_SYNC_ENABLED" \
          --arg LEGACY_SYNC_SHADOW_MODE "$LEGACY_SYNC_SHADOW_MODE" \
          '{
            commit_sha: $commit_sha,
            deploy_payload_b64: $deploy_payload_b64,
            env: {
              DB_SERVER:               $DB_SERVER,
              DB_NAME:                 $DB_NAME,
              DB_USER:                 $DB_USER,
              DB_PASSWORD:             $DB_PASSWORD,
              MSSQL_PORT:              $MSSQL_PORT,
              POSTGRES_DB:             $POSTGRES_DB,
              POSTGRES_USER:           $POSTGRES_USER,
              POSTGRES_PASSWORD:       $POSTGRES_PASSWORD,
              SLACK_WEBHOOK_URL:       $SLACK_WEBHOOK_URL,
              LEGACY_SYNC_ENABLED:     $LEGACY_SYNC_ENABLED,
              LEGACY_SYNC_SHADOW_MODE: $LEGACY_SYNC_SHADOW_MODE,
            }
          }' \
        | ssh -i ~/.ssh/deploy_key \
            -o StrictHostKeyChecking=yes \
            -o UserKnownHostsFile=~/.ssh/known_hosts \
            -o ProxyCommand="cloudflared access ssh --hostname %h" \
            deploy@evergreen.thehfhotel.org
```

The other jobs (`changes`, `test-frontend`, `test-backend`,
`init-db-migrations-drift-check`, `build-frontend`, `build-backend`) just flip
`runs-on: [self-hosted, ...]` → `runs-on: ubuntu-latest`. They get parallel
execution back, no more snap-docker flap on builds.

---

## Rollback plan

If the new flow fails after a workflow change, the self-hosted runner is still
registered (just disabled in step 8 below). Re-enable:

```bash
sudo systemctl enable --now actions.runner.thehfhotel-new-hotel.evergreen.service
```

…and revert the workflow file to the pre-Phase-1 commit. Keep the `deploy`
user and forced-command setup in place even on rollback — they're independently
useful and don't conflict with the self-hosted path.

After 2 weeks of green deploys via the new flow, fully deregister the
self-hosted runner (step 8).

---

## 8. Self-hosted runner cleanup (2 weeks after step 7 lands)

```bash
# Stop and disable the systemd unit
sudo systemctl stop actions.runner.thehfhotel-new-hotel.evergreen.service
sudo systemctl disable actions.runner.thehfhotel-new-hotel.evergreen.service

# Deregister from GitHub
cd /home/nut/actions-runner
sudo -u nut ./config.sh remove --token "$(gh api -X POST /repos/thehfhotel/new-hotel/actions/runners/remove-token --jq .token)"

# Free the disk
sudo rm -rf /home/nut/actions-runner
```

---

## What this DOESN'T fix (intentionally — separate phases)

- **Snap-docker reliability**: still flaky on evergreen, but only the deploy step is affected now (build/test on GH-hosted is fine). The `retry_compose` helper in the deploy script masks it. Apt CE migration is a separate operational task; not blocking.
- **Application authentication**: the backend has zero auth (Phase 4 of the master plan).
- **Sensitive content sanitization**: Phase 2 of the master plan.
- **Secret rotation**: Phase 3 of the master plan. Rotate `DB_PASSWORD` and `POSTGRES_PASSWORD` after this Phase 1 lands so the rotation is also encoded into the new deploy mechanism — minimizes coordination.

---

## Post-Phase-1 audit hooks

After everything's working, verify:

- [ ] `sudo grep PasswordAuthentication /etc/ssh/sshd_config` shows `no`
- [ ] `sudo -u deploy ssh deploy@localhost` (with key) runs the script — i.e. forced-command pins it
- [ ] `sudo -u deploy ssh deploy@localhost ls /` returns the script output, NOT `ls /` — proves the command is fixed
- [ ] `/srv/run-deploy.sh` is `-rwxr-xr-x root root`
- [ ] `/home/deploy/.ssh/authorized_keys` is `-rw------- deploy deploy`
- [ ] A real deploy via the new pipeline lands cleanly, and `/var/log/deploy/deploy-*.log` is captured
- [ ] Cloudflare tunnel `asgard` shows the deploy connection in `cloudflared` logs / CF dashboard tunnel metrics
