import type { Metadata } from 'next'
import { Sarabun } from 'next/font/google'
import './globals.css'
import 'react-datepicker/dist/react-datepicker.css'
import Providers from '@/components/Providers'

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

export default function RootLayout({
  children,
}: {
  children: React.ReactNode
}) {
  return (
    <html lang="th" className={sarabun.variable}>
      <body className="font-sans bg-shell text-text">
        <Providers>
          {children}
        </Providers>
      </body>
    </html>
  )
}
