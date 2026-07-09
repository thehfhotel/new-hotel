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
