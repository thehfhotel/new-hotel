import Navbar from '@/components/Navbar'

export default function LegacyLayout({
  children,
}: {
  children: React.ReactNode
}) {
  return (
    <>
      <Navbar />
      <main className="w-full px-4 py-6">
        {children}
      </main>
    </>
  )
}
