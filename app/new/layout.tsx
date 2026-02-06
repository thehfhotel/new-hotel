import NewNavbar from '@/components/NewNavbar'

export default function NewLayout({
  children,
}: {
  children: React.ReactNode
}) {
  return (
    <>
      <NewNavbar />
      <main className="w-full px-4 py-6">
        {children}
      </main>
    </>
  )
}
