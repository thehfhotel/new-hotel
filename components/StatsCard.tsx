import { LucideIcon } from 'lucide-react'

interface StatsCardProps {
  title: string
  value: string | number
  icon: LucideIcon
  color: 'blue' | 'green' | 'yellow' | 'red' | 'purple' | 'indigo' | 'orange'
}

const colorClasses = {
  blue: {
    bg: 'bg-blue-50',
    icon: 'bg-blue-500',
    text: 'text-blue-600',
  },
  green: {
    bg: 'bg-green-50',
    icon: 'bg-green-500',
    text: 'text-green-600',
  },
  yellow: {
    bg: 'bg-yellow-50',
    icon: 'bg-yellow-500',
    text: 'text-yellow-600',
  },
  red: {
    bg: 'bg-red-50',
    icon: 'bg-red-500',
    text: 'text-red-600',
  },
  purple: {
    bg: 'bg-purple-50',
    icon: 'bg-purple-500',
    text: 'text-purple-600',
  },
  indigo: {
    bg: 'bg-indigo-50',
    icon: 'bg-indigo-500',
    text: 'text-indigo-600',
  },
  orange: {
    bg: 'bg-orange-50',
    icon: 'bg-orange-500',
    text: 'text-orange-600',
  },
}

export default function StatsCard({ title, value, icon: Icon, color }: StatsCardProps) {
  const colors = colorClasses[color]

  return (
    <div className={`${colors.bg} rounded-xl p-6 shadow-sm border border-gray-100 hover:shadow-md transition-shadow duration-200`}>
      <div className="flex items-center justify-between">
        <div>
          <p className="text-gray-500 text-sm font-medium mb-1">{title}</p>
          <p className={`text-3xl font-bold ${colors.text}`}>{value}</p>
        </div>
        <div className={`${colors.icon} p-3 rounded-lg`}>
          <Icon size={28} className="text-white" />
        </div>
      </div>
    </div>
  )
}
