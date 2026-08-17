/** @type {import('next').NextConfig} */
const nextConfig = {
  output: 'standalone',
  poweredByHeader: false,
  async headers() {
    const contentSecurityPolicy = [
      "default-src 'self'",
      // static.cloudflareinsights.com serves the Web Analytics beacon that the
      // Cloudflare proxy auto-injects into HTML; it POSTs RUM data back to
      // cloudflareinsights.com/cdn-cgi/rum (hence the connect-src entry).
      "script-src 'self' 'unsafe-inline' 'unsafe-eval' https://static.cloudflareinsights.com https://erp.thehfhotel.org",
      "style-src 'self' 'unsafe-inline'",
      "img-src 'self' data: blob:",
      "font-src 'self' data:",
      // http://localhost:9898 / 127.0.0.1:9898 = the per-PC Thai-ID card-reader
      // middleware (a local hardware bridge). http:// is fine from an https page
      // because Chrome treats localhost/127.0.0.1 as a secure context (no
      // mixed-content block); without these the fetch is refused by connect-src.
      //
      // The narrowness of this list is LOAD-BEARING AS A DIAGNOSTIC, not just a
      // hardening measure: when a request carries the wrong Cloudflare Access
      // `aud` the edge answers with a cross-origin login redirect, connect-src
      // refuses it, and the bug announces itself as a red console error instead
      // of failing silently. Do NOT widen connect-src to quiet such an error —
      // fix the `aud` (or delete the call that should never have fired). The
      // dead /api/auth/me probe on the /hk maid surface was found exactly this
      // way and removed rather than exempted.
      "connect-src 'self' https://cloudflareinsights.com http://localhost:9898 http://127.0.0.1:9898",
      "frame-ancestors 'none'",
      "base-uri 'self'",
      "form-action 'self'",
    ].join('; ')

    return [
      {
        source: '/(.*)',
        headers: [
          { key: 'X-Content-Type-Options', value: 'nosniff' },
          { key: 'X-Frame-Options', value: 'DENY' },
          { key: 'Referrer-Policy', value: 'strict-origin-when-cross-origin' },
          { key: 'Strict-Transport-Security', value: 'max-age=31536000; includeSubDomains' },
          {
            key: 'Permissions-Policy',
            value: 'camera=(), microphone=(), geolocation=(), payment=(), usb=(), interest-cohort=()',
          },
          { key: 'Content-Security-Policy', value: contentSecurityPolicy },
        ],
      },
      {
        // Every rendered page now carries a `data-property` on the HF One band
        // derived from the viewer's Cloudflare Access identity (app/layout.tsx
        // → lib/server/shell-property.ts), so a document or RSC payload must
        // NEVER be reused for a different viewer: a shell cached across
        // identities would scope the wrong receptionist's switcher, which is
        // worse than not scoping at all. `no-store` is the whole guarantee —
        // no cache, shared or private, may store the response at all.
        //
        // There is deliberately no `Vary` entry here. MEASURED on Next 16.2.12
        // (`next start` + curl, both the HTML document and an `RSC: 1`
        // payload): the App Router overwrites `Vary` with its own router
        // tokens (`rsc, next-router-state-tree, next-router-prefetch,
        // next-router-segment-prefetch, Accept-Encoding`) AFTER both
        // next.config headers and proxy/middleware headers are applied, so any
        // `Vary` we set here is silently dropped. An entry that looks like it
        // separates identities but does not is worse than none; if a `Vary` on
        // the Access assertion is ever wanted it has to come from the layer in
        // front (shared-nginx / Cloudflare), not from this file.
        //
        // Scoped away from `/_next/static` + `/_next/image` (immutable,
        // content-hashed, identity-blind — verified still `public, max-age=
        // 31536000, immutable`) and from `/api/*`, which is rewritten to the
        // Rust backend and owns its own cache semantics.
        source: '/((?!_next/static|_next/image|api/).*)',
        headers: [{ key: 'Cache-Control', value: 'private, no-store' }],
      },
    ]
  },
  async rewrites() {
    const backendUrl = process.env.BACKEND_URL || 'http://backend:3003'
    return [
      {
        // Maid-facing housekeeping surface: its API lives UNDER /hk so ONE
        // path-scoped Cloudflare Access application (hotel.thehfhotel.org/hk)
        // covers both the pages and their API calls — the edge then attaches
        // the /hk app's Cf-Access-Jwt-Assertion (CF_ACCESS_HK_AUD) to every
        // request the backend's hk_access middleware verifies. A plain
        // /api/hk/* call from the browser would instead carry the MAIN app's
        // assertion and fail the AUD check.
        source: '/hk/api/:path*',
        destination: `${backendUrl}/api/hk/:path*`,
      },
      {
        source: '/api/:path*',
        destination: `${backendUrl}/api/:path*`,
      },
    ]
  },
}

module.exports = nextConfig
