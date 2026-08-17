/**
 * Which HF property is the human in front of this browser standing at?
 *
 * Answers ONE question, for ONE consumer: the `data-property` attribute of the
 * HF One marquee band (`public/shell/hf-bar.js` in hf-erp; contract in that
 * repo's `design/HF-ONE.md`). When the band is told the property, it drops the
 * OTHER property's branch-specific tools from its switcher — today the two
 * identical-looking "Room Daily Report" entries, which the HF Ville reception
 * desk kept confusing for HF's, filing a day of rooms against the wrong hotel.
 *
 * ## This is NOT the branch axis
 *
 * Deliberately unrelated to `BranchContext` / `HK_BRANCHES` / `VILLE_*`. Those
 * say which DATA the backend serves; this says where the HUMAN is standing.
 * Coupling the two is how a read ends up pointed at the wrong server, so this
 * module imports none of them and nothing else may derive a branch from it.
 * It is a cosmetic navigation hint, it never reaches an access decision, and
 * every URL stays reachable whatever it returns.
 *
 * ## Fail OPEN, always
 *
 * Every failure path — no assertion on the request, unverifiable assertion,
 * unreachable JWKS, malformed claims, anything thrown — returns `undefined`,
 * which omits the attribute and yields the FULL switcher. A missing tool is a
 * worse failure than an extra one (the band applies the same rule to an
 * unrecognised value).
 *
 * ## Verification contract
 *
 * Mirrors `hotel-backend/src/middleware/cf_access.rs`, which is the repo's
 * authoritative Cloudflare Access resolver and stays the source of truth for
 * these rules: RS256 over the team JWKS at `<team domain>/cdn-cgi/access/certs`,
 * `iss` MUST equal the team domain, `aud` MUST contain the Access application
 * tag (`CF_ACCESS_AUD` env override, else the same baked default — the AUD tag
 * is a public application identifier, not a secret), `exp`/`nbf` enforced with
 * the same 60s leeway `jsonwebtoken` applies, and the `email` claim is
 * REQUIRED (service-token assertions, which carry none, resolve to nothing).
 *
 * Re-stated here rather than called across the wire because this runs in the
 * Next.js HTML render path, where a round trip to the backend would put a
 * second failure mode in front of every page. If the two copies ever drift the
 * frontend simply stops recognising the kiosk and shows the full switcher —
 * the safe direction.
 */

import { createPublicKey, createVerify } from 'node:crypto'

/** The property values `hf-bar.js` understands. Anything else is ignored by it. */
export type ShellProperty = 'hf' | 'hfville'

/**
 * The HF Ville reception kiosk's Google identity — the shared account behind
 * the `Kiosk: hfville-reception-1` Cloudflare Access policy. This single
 * address is the ONLY input that scopes the switcher.
 *
 * There is deliberately no entry for HF's reception identity
 * (`theharbourfront.hotel@gmail.com`): it also runs as a second Chrome profile
 * on the HF Ville reception PC, so it names no place. Scoping on it would hide
 * HF Ville's own card at the HF Ville desk — the same bug from the other side.
 * Nor for `sdyoffice66@gmail.com` (`office-1`), the office PC that works both
 * properties. Managers, employees and phones are never scoped either.
 */
export const HFVILLE_RECEPTION_KIOSK_EMAIL = 'hfville.hotel@gmail.com'

/** Request header Cloudflare Access attaches the signed assertion under. */
const CF_ACCESS_JWT_HEADER = 'cf-access-jwt-assertion'

/** Cookie fallback, for requests that reach the origin without the header. */
const CF_ACCESS_COOKIE_NAME = 'CF_Authorization'

/** REQUIRED `iss`, and the base URL the JWKS is fetched from. */
const CF_ACCESS_TEAM_DOMAIN = 'https://laikaexpress.cloudflareaccess.com'

/** Public Access-application identifier, not a secret. */
const DEFAULT_CF_ACCESS_AUD =
  '832861a2b62e6ce2b0e100d1bd40c84789ddee561aa5973c896f4aced4d821cf'

/** Clock leeway on `exp`/`nbf`, matching the Rust side's `jsonwebtoken` default. */
const CLOCK_LEEWAY_SECONDS = 60

/** How long a fetched JWKS is trusted. Same 10 minutes as the Rust resolver. */
const JWKS_CACHE_TTL_MS = 10 * 60 * 1000

/**
 * How long a FAILED JWKS fetch is remembered. Without it, an unreachable team
 * domain would mean one outbound request per page render.
 */
const JWKS_FAILURE_TTL_MS = 30 * 1000

/** Hard ceiling on the JWKS fetch — this sits in the HTML render path. */
const JWKS_FETCH_TIMEOUT_MS = 2000

interface Jwk {
  kid?: string
  kty?: string
  alg?: string
  n?: string
  e?: string
}

/**
 * The decision, isolated from all I/O: a VERIFIED Cloudflare Access email in,
 * a property (or nothing) out. Comparison is case-insensitive because the
 * `email` claim's casing is not guaranteed stable.
 */
export function shellPropertyForAccessEmail(
  email: string | null | undefined,
): ShellProperty | undefined {
  if (!email) return undefined
  return email.trim().toLowerCase() === HFVILLE_RECEPTION_KIOSK_EMAIL
    ? 'hfville'
    : undefined
}

/**
 * Pull the Access assertion off a request: header first, `CF_Authorization`
 * cookie second. Returns `null` when neither carries one (an unauthenticated
 * or non-Access caller).
 */
export function extractAccessToken(headers: Headers): string | null {
  const fromHeader = headers.get(CF_ACCESS_JWT_HEADER)
  if (fromHeader && fromHeader.trim()) return fromHeader.trim()

  const cookieHeader = headers.get('cookie')
  if (!cookieHeader) return null
  for (const part of cookieHeader.split(';')) {
    const separator = part.indexOf('=')
    if (separator === -1) continue
    if (part.slice(0, separator).trim() !== CF_ACCESS_COOKIE_NAME) continue
    const value = part.slice(separator + 1).trim()
    if (value) return value
  }
  return null
}

/**
 * Resolve `data-property` for one request. Never throws, never returns `'hf'`
 * today — HF's reception identity names no place (see the constant above), so
 * `'hfville'` is the only value any current identity produces.
 */
export async function shellPropertyForRequest(
  headers: Headers,
): Promise<ShellProperty | undefined> {
  try {
    const token = extractAccessToken(headers)
    if (!token) return undefined
    return shellPropertyForAccessEmail(await verifiedAccessEmail(token))
  } catch {
    // Fail open: the full switcher is always the safe answer.
    return undefined
  }
}

/**
 * Verify a Cloudflare Access assertion and return its `email` claim, or `null`
 * if it fails any part of the contract above.
 */
export async function verifiedAccessEmail(token: string): Promise<string | null> {
  const segments = token.split('.')
  if (segments.length !== 3) return null
  const [encodedHeader, encodedPayload, encodedSignature] = segments

  const header = decodeSegment(encodedHeader)
  if (!header || header.alg !== 'RS256') return null
  const kid = typeof header.kid === 'string' ? header.kid : null
  if (!kid) return null

  const jwk = (await teamJwks()).find((key) => key.kid === kid)
  if (!jwk || jwk.kty !== 'RSA' || !jwk.n || !jwk.e) return null

  const verifier = createVerify('RSA-SHA256')
  verifier.update(`${encodedHeader}.${encodedPayload}`)
  verifier.end()
  const publicKey = createPublicKey({
    key: { kty: 'RSA', n: jwk.n, e: jwk.e },
    format: 'jwk',
  })
  if (!verifier.verify(publicKey, Buffer.from(encodedSignature, 'base64url'))) {
    return null
  }

  const payload = decodeSegment(encodedPayload)
  if (!payload) return null
  if (payload.iss !== CF_ACCESS_TEAM_DOMAIN) return null

  const audience = Array.isArray(payload.aud)
    ? payload.aud
    : typeof payload.aud === 'string'
      ? [payload.aud]
      : []
  if (!audience.includes(expectedAud())) return null

  const now = Date.now() / 1000
  if (typeof payload.exp !== 'number' || payload.exp + CLOCK_LEEWAY_SECONDS < now) {
    return null
  }
  if (typeof payload.nbf === 'number' && payload.nbf - CLOCK_LEEWAY_SECONDS > now) {
    return null
  }

  const email = payload.email
  return typeof email === 'string' && email.trim() ? email : null
}

/**
 * `CF_ACCESS_AUD` override, else the baked default. Trimmed so a stray newline
 * in an env file doesn't silently break every verification (same guard as the
 * Rust `expected_aud`).
 */
function expectedAud(): string {
  const override = process.env.CF_ACCESS_AUD?.trim()
  return override || DEFAULT_CF_ACCESS_AUD
}

function decodeSegment(segment: string): Record<string, unknown> | null {
  try {
    const parsed = JSON.parse(Buffer.from(segment, 'base64url').toString('utf8'))
    return parsed && typeof parsed === 'object' ? parsed : null
  } catch {
    return null
  }
}

let jwksCache: { keys: Jwk[]; expiresAt: number } | null = null
let jwksInFlight: Promise<Jwk[]> | null = null

/**
 * The team JWKS, cached in-process. A failed fetch caches an EMPTY key set for
 * a shorter window — every verification then fails, which fails open, without
 * one outbound request per render.
 */
async function teamJwks(): Promise<Jwk[]> {
  if (jwksCache && jwksCache.expiresAt > Date.now()) return jwksCache.keys
  if (jwksInFlight) return jwksInFlight

  jwksInFlight = (async () => {
    try {
      const response = await fetch(`${CF_ACCESS_TEAM_DOMAIN}/cdn-cgi/access/certs`, {
        signal: AbortSignal.timeout(JWKS_FETCH_TIMEOUT_MS),
        cache: 'no-store',
      })
      if (!response.ok) throw new Error(`JWKS fetch returned ${response.status}`)
      const body = await response.json()
      const keys: Jwk[] = Array.isArray(body?.keys) ? body.keys : []
      jwksCache = { keys, expiresAt: Date.now() + JWKS_CACHE_TTL_MS }
      return keys
    } catch (error) {
      // Rate-limited by the shorter negative TTL, so this cannot spam the log.
      console.warn('[shell-property] Cloudflare Access JWKS fetch failed', error)
      jwksCache = { keys: [], expiresAt: Date.now() + JWKS_FAILURE_TTL_MS }
      return []
    } finally {
      jwksInFlight = null
    }
  })()

  return jwksInFlight
}

/** Test-only: drop the in-process JWKS cache between cases. */
export function __resetJwksCacheForTests(): void {
  jwksCache = null
  jwksInFlight = null
}
