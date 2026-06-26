import type { Metadata } from 'next'
import { Space_Grotesk } from 'next/font/google'
import './v2.css'
import { BranchProvider } from '@/contexts/BranchContext'
import V2Shell from '@/components/v2/V2Shell'

// Space Grotesk — the precise numeral/label face for the new UI. Scoped to the
// `.v2` wrapper via the CSS variable so it never touches the classic app.
const spaceGrotesk = Space_Grotesk({
  subsets: ['latin'],
  weight: ['400', '500', '600', '700'],
  variable: '--font-v2-display',
  display: 'swap',
})

export const metadata: Metadata = {
  title: 'Front Desk — ระบบจัดการโรงแรม',
  description: 'แผงควบคุมหน้าบ้านสำหรับพนักงานต้อนรับ',
}

export default function V2Layout({ children }: { children: React.ReactNode }) {
  return (
    <BranchProvider>
      <div className={`v2 ${spaceGrotesk.variable}`}>
        <V2Shell>{children}</V2Shell>
      </div>
    </BranchProvider>
  )
}
