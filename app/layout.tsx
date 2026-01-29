import type { Metadata } from 'next'
import { Inter } from 'next/font/google'
import './globals.css'
import Navbar from '@/components/Navbar'

const inter = Inter({ subsets: ['latin'] })

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
    <html lang="th">
      <body className={inter.className}>
        <Navbar />
        <main className="w-full px-4 py-6">
          {children}
        </main>
      </body>
    </html>
  )
}
