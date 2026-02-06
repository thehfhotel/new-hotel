interface StatsCardProps {
  title: string
  value: string | number
  subtitle?: string
}

export default function StatsCard({ title, value, subtitle }: StatsCardProps) {
  return (
    <div className="bg-zinc-900 rounded-xl px-6 py-5 border border-zinc-800">
      <p className="text-sm font-medium text-zinc-500 mb-1">{title}</p>
      <p className="text-3xl font-bold text-zinc-100">{value}</p>
      {subtitle && (
        <p className="text-xs text-zinc-600 mt-1">{subtitle}</p>
      )}
    </div>
  )
}
