# Security Policy

## Reporting a Vulnerability

If you believe you have found a security vulnerability in this project,
please report it privately so we can fix it before public disclosure.

**Do not** open a public GitHub issue for security reports.

### How to report

Email: **winut.hf@gmail.com**

Please include:

- A description of the issue and its impact
- Steps to reproduce (proof-of-concept code is welcome but not required)
- The affected version, commit SHA, or deployment target
- Your name / handle for credit (optional)

You will receive an acknowledgement within **5 business days**. We aim
to provide a fix or mitigation within **30 days** for high-severity
issues; lower-severity issues may take longer.

## Scope

This policy covers the contents of the `thehfhotel/new-hotel`
repository — the Rust + Axum backend, the Next.js frontend, the SQL
migrations, the Tauri-based Thai ID middleware, and the deployment
configuration.

The hotel application is **internal-only** (LAN / WireGuard /
Cloudflare Access) and is not exposed to the public internet. Reports
of vulnerabilities that require network access to a production
deployment are out of scope unless you can demonstrate the attack
working from the public internet against `evergreen.thehfhotel.org`
or another production hostname.

Findings in vendored or third-party dependencies should be reported
upstream (we follow Dependabot for routine dependency updates).

## Supported versions

Only the latest tagged release on `master` is supported. There is no
LTS branch.

## Disclosure process

1. You report privately.
2. We acknowledge within 5 business days.
3. We work on a fix; we may ask follow-up questions.
4. We release a fix and credit you (with your permission) in
   `CHANGELOG.md`.
5. You may publicly disclose the issue 90 days after the initial report
   or 30 days after the fix is released, whichever comes first.

## What we will not do

- We will not pursue legal action against good-faith researchers
  acting within this policy.
- We will not request that you delay disclosure beyond the windows
  above without specific, documented justification.

## Out of scope

- Social engineering of staff or guests
- Physical attacks against the hotel premises or network equipment
- Denial-of-service attacks
- Vulnerabilities in third-party services we depend on (Cloudflare,
  GitHub, etc.) — report those upstream
- Issues that require attacker access to a logged-in receptionist's
  workstation or Cloudflare Access session
