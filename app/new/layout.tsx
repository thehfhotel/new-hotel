import NewNavbar from '@/components/NewNavbar'

export default function NewLayout({
  children,
}: {
  children: React.ReactNode
}) {
  return (
    <div className="new-system-layout min-h-screen bg-zinc-950">
      <NewNavbar />
      <main className="w-full px-4 py-6">
        {children}
      </main>
    </div>
  )
}
