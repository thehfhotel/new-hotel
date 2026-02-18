'use client'

import { ReactNode } from 'react'
import { Inbox } from 'lucide-react'

interface EmptyStateProps {
  icon?: ReactNode
  title: string
  description?: string
  action?: ReactNode
}

export default function EmptyState({ icon, title, description, action }: EmptyStateProps) {
  return (
    <div className="flex flex-col items-center justify-center py-12 px-4 text-center">
      <div className="text-zinc-600 mb-4">{icon || <Inbox className="w-12 h-12" />}</div>
      <h3 className="text-lg font-medium text-zinc-300 mb-1">{title}</h3>
      {description && <p className="text-sm text-zinc-500 mb-4 max-w-md">{description}</p>}
      {action}
    </div>
  )
}
