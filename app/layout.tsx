import type { Metadata } from 'next'
import { Sarabun } from 'next/font/google'
import { headers } from 'next/headers'
import './globals.css'
import 'react-datepicker/dist/react-datepicker.css'
import Providers from '@/components/Providers'
import AppShell from '@/components/AppShell'
import { shellPropertyForRequest } from '@/lib/server/shell-property'

// Sarabun is the Thai government standard typeface (supports both Thai + Latin).
// Exposed as a CSS variable so tailwind.config.ts can pick it up via `var(--font-sarabun)`.
const sarabun = Sarabun({
  subsets: ['latin', 'thai'],
  weight: ['300', '400', '500', '600', '700'],
  variable: '--font-sarabun',
  display: 'swap',
})

export const metadata: Metadata = {
  title: 'ระบบจัดการโรงแรม',
  description: 'Hotel Management Visualization System',
}

// Read the persisted skin from localStorage and apply it to <html> BEFORE
// React hydrates. Without this, the page paints with the default skin first
// and then snaps to the user's choice — a visible flash. Falls back silently
// to 'fiori' if storage is denied (private mode, etc).
const SKIN_BOOTSTRAP = `(function(){try{var s=localStorage.getItem('hotel-skin');if(s!=='fiori'&&s!=='modern')s='fiori';document.documentElement.setAttribute('data-skin',s);}catch(e){document.documentElement.setAttribute('data-skin','fiori');}})();`

// Reading `headers()` makes every route render PER REQUEST instead of being
// prerendered at build time. That is required, not incidental: the HF One band
// below carries a `data-property` derived from the viewer's Cloudflare Access
// identity, and a prerendered (or cross-identity cached) shell would bake one
// receptionist's value into everybody else's page — the exact wrong-hotel bug
// this exists to prevent. `next.config.js` pairs it with
// `Cache-Control: private, no-store` on the same responses (see the note there
// about `Vary`, which the App Router owns and overwrites).
export default async function RootLayout({
  children,
}: {
  children: React.ReactNode
}) {
  const shellProperty = await shellPropertyForRequest(await headers())

  return (
    <html lang="th" className={sarabun.variable} data-skin="fiori">
      <head>
        <script dangerouslySetInnerHTML={{ __html: SKIN_BOOTSTRAP }} />
      </head>
      <body className="font-sans bg-shell text-text">
        <Providers>
          <AppShell shellProperty={shellProperty}>
            {children}
          </AppShell>
        </Providers>
      </body>
    </html>
  )
}
