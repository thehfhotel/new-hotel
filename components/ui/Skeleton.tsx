'use client'

interface SkeletonProps {
  className?: string
  variant?: 'text' | 'card' | 'table-row'
}

const variantStyles: Record<string, string> = {
  text: 'h-4 w-full rounded',
  card: 'h-32 w-full rounded-lg',
  'table-row': 'h-12 w-full rounded',
}

export default function Skeleton({ className, variant = 'text' }: SkeletonProps) {
  return <div className={`bg-zinc-800 animate-pulse ${variantStyles[variant]} ${className || ''}`} />
}
