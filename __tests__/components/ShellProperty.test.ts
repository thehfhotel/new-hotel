/**
 * The `data-property` decision for the HF One band (lib/server/shell-property.ts).
 *
 * Runs in the repo-default `node` test environment (no `@jest-environment
 * jsdom` docblock): this is a server module that needs real `node:crypto` and
 * `Buffer`, and touches no DOM.
 *
 * Signing keys are generated per run, so nothing secret is committed and the
 * JWKS is served from a stubbed `fetch` — these tests never touch the network.
 * Mirrors the Rust resolver's own test approach
 * (hotel-backend/src/middleware/cf_access.rs).
 */

import { createSign, generateKeyPairSync, type KeyObject } from 'node:crypto'

import {
  HFVILLE_RECEPTION_KIOSK_EMAIL,
  extractAccessToken,
  shellPropertyForAccessEmail,
  shellPropertyForRequest,
  verifiedAccessEmail,
  __resetJwksCacheForTests,
} from '@/lib/server/shell-property'

const TEAM_DOMAIN = 'https://laikaexpress.cloudflareaccess.com'
const TEST_AUD = 'test-access-aud'
const KID = 'test-key-1'

let privateKey: KeyObject
let otherPrivateKey: KeyObject
let jwks: { keys: unknown[] }
const originalAud = process.env.CF_ACCESS_AUD

beforeAll(() => {
  const pair = generateKeyPairSync('rsa', { modulusLength: 2048 })
  privateKey = pair.privateKey
  // A second, UNPUBLISHED key: signatures made with it must be rejected.
  otherPrivateKey = generateKeyPairSync('rsa', { modulusLength: 2048 }).privateKey

  const jwk = pair.publicKey.export({ format: 'jwk' })
  jwks = { keys: [{ ...jwk, kid: KID, alg: 'RS256', use: 'sig' }] }

  process.env.CF_ACCESS_AUD = TEST_AUD
})

afterAll(() => {
  if (originalAud === undefined) delete process.env.CF_ACCESS_AUD
  else process.env.CF_ACCESS_AUD = originalAud
})

beforeEach(() => {
  __resetJwksCacheForTests()
  global.fetch = jest.fn(async () => ({
    ok: true,
    json: async () => jwks,
  })) as unknown as typeof fetch
})

function base64url(value: string): string {
  return Buffer.from(value, 'utf8').toString('base64url')
}

interface Claims {
  email?: string
  iss?: string
  aud?: string | string[]
  exp?: number
  nbf?: number
}

function signAssertion(claims: Claims = {}, key: KeyObject = privateKey): string {
  const nowSeconds = Math.floor(Date.now() / 1000)
  const header = base64url(JSON.stringify({ alg: 'RS256', kid: KID, typ: 'JWT' }))
  const payload = base64url(
    JSON.stringify({
      iss: TEAM_DOMAIN,
      aud: TEST_AUD,
      exp: nowSeconds + 3600,
      nbf: nowSeconds - 10,
      ...claims,
    }),
  )
  const signer = createSign('RSA-SHA256')
  signer.update(`${header}.${payload}`)
  signer.end()
  return `${header}.${payload}.${signer.sign(key).toString('base64url')}`
}

/** A request as Cloudflare Access delivers it: assertion on the header. */
function requestAs(email: string, claims: Claims = {}): Headers {
  return new Headers({
    'cf-access-jwt-assertion': signAssertion({ email, ...claims }),
  })
}

describe('shellPropertyForAccessEmail (the locked decision)', () => {
  it('scopes ONLY the HF Ville reception kiosk', () => {
    expect(shellPropertyForAccessEmail(HFVILLE_RECEPTION_KIOSK_EMAIL)).toBe('hfville')
    expect(HFVILLE_RECEPTION_KIOSK_EMAIL).toBe('hfville.hotel@gmail.com')
  })

  it('is case- and whitespace-insensitive on the verified claim', () => {
    expect(shellPropertyForAccessEmail('  HFVille.Hotel@Gmail.com ')).toBe('hfville')
  })

  it.each([
    // Also runs as Chrome Profile 1 on the HF Ville reception PC, so it names
    // no place — scoping on it would hide HF Ville's card at the HF Ville desk.
    ['theharbourfront.hotel@gmail.com', 'HF reception kiosk'],
    // The office PC works both properties.
    ['sdyoffice66@gmail.com', 'office-1 kiosk'],
    ['winut.hf@gmail.com', 'a manager'],
    ['somebody@emp.thehfhotel.org', 'an employee'],
    ['', 'an empty claim'],
  ])('omits the attribute for %s (%s)', (email) => {
    expect(shellPropertyForAccessEmail(email)).toBeUndefined()
  })

  it('omits the attribute for a missing identity', () => {
    expect(shellPropertyForAccessEmail(null)).toBeUndefined()
    expect(shellPropertyForAccessEmail(undefined)).toBeUndefined()
  })
})

describe('shellPropertyForRequest (end to end, per request)', () => {
  it('scopes the HF Ville reception kiosk from a valid assertion', async () => {
    await expect(
      shellPropertyForRequest(requestAs(HFVILLE_RECEPTION_KIOSK_EMAIL)),
    ).resolves.toBe('hfville')
  })

  it('accepts the assertion from the CF_Authorization cookie fallback', async () => {
    const headers = new Headers({
      cookie: `other=1; CF_Authorization=${signAssertion({
        email: HFVILLE_RECEPTION_KIOSK_EMAIL,
      })}; trailing=2`,
    })
    await expect(shellPropertyForRequest(headers)).resolves.toBe('hfville')
  })

  it.each([
    ['theharbourfront.hotel@gmail.com', 'the HF reception kiosk'],
    ['sdyoffice66@gmail.com', 'the office kiosk'],
    ['winut.hf@gmail.com', 'a manager'],
  ])('omits the attribute for %s (%s)', async (email) => {
    await expect(shellPropertyForRequest(requestAs(email))).resolves.toBeUndefined()
  })

  it('omits the attribute for an anonymous caller, without fetching the JWKS', async () => {
    await expect(shellPropertyForRequest(new Headers())).resolves.toBeUndefined()
    expect(global.fetch).not.toHaveBeenCalled()
  })

  it('fails OPEN when the JWKS is unreachable', async () => {
    global.fetch = jest.fn(async () => {
      throw new Error('ECONNREFUSED')
    }) as unknown as typeof fetch
    const warn = jest.spyOn(console, 'warn').mockImplementation(() => {})

    await expect(
      shellPropertyForRequest(requestAs(HFVILLE_RECEPTION_KIOSK_EMAIL)),
    ).resolves.toBeUndefined()

    warn.mockRestore()
  })

  it('fails OPEN on a malformed assertion', async () => {
    const headers = new Headers({ 'cf-access-jwt-assertion': 'not-a-jwt' })
    await expect(shellPropertyForRequest(headers)).resolves.toBeUndefined()
  })
})

describe('verifiedAccessEmail (the contract mirrored from cf_access.rs)', () => {
  const email = HFVILLE_RECEPTION_KIOSK_EMAIL

  it('returns the email claim of a valid assertion', async () => {
    await expect(verifiedAccessEmail(signAssertion({ email }))).resolves.toBe(email)
  })

  it('rejects a signature from a key that is not in the JWKS', async () => {
    await expect(
      verifiedAccessEmail(signAssertion({ email }, otherPrivateKey)),
    ).resolves.toBeNull()
  })

  it('rejects an expired assertion (beyond the 60s leeway)', async () => {
    const exp = Math.floor(Date.now() / 1000) - 120
    await expect(verifiedAccessEmail(signAssertion({ email, exp }))).resolves.toBeNull()
  })

  it('rejects a not-yet-valid assertion', async () => {
    const nbf = Math.floor(Date.now() / 1000) + 600
    await expect(verifiedAccessEmail(signAssertion({ email, nbf }))).resolves.toBeNull()
  })

  it('rejects an assertion minted for another Access application', async () => {
    await expect(
      verifiedAccessEmail(signAssertion({ email, aud: 'some-other-app' })),
    ).resolves.toBeNull()
  })

  it('rejects an assertion from another issuer', async () => {
    await expect(
      verifiedAccessEmail(signAssertion({ email, iss: 'https://evil.example' })),
    ).resolves.toBeNull()
  })

  it('rejects a service-token assertion (no email claim)', async () => {
    await expect(verifiedAccessEmail(signAssertion())).resolves.toBeNull()
  })

  it('accepts an array aud that contains our application tag', async () => {
    await expect(
      verifiedAccessEmail(signAssertion({ email, aud: ['other', TEST_AUD] })),
    ).resolves.toBe(email)
  })
})

describe('extractAccessToken', () => {
  it('prefers the header over the cookie', () => {
    const headers = new Headers({
      'cf-access-jwt-assertion': 'from-header',
      cookie: 'CF_Authorization=from-cookie',
    })
    expect(extractAccessToken(headers)).toBe('from-header')
  })

  it('returns null when neither is present', () => {
    expect(extractAccessToken(new Headers())).toBeNull()
    expect(extractAccessToken(new Headers({ cookie: 'session=abc' }))).toBeNull()
  })
})
