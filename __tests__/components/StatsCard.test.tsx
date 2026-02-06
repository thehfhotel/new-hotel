/**
 * @jest-environment jsdom
 */

import { render, screen } from '@testing-library/react'
import StatsCard from '@/components/StatsCard'

describe('StatsCard Component', () => {
  test('renders title and value correctly', () => {
    render(<StatsCard title="ห้องที่มีผู้เข้าพัก" value={12} />)

    expect(screen.getByText('ห้องที่มีผู้เข้าพัก')).toBeInTheDocument()
    expect(screen.getByText('12')).toBeInTheDocument()
  })

  test('renders string value', () => {
    render(<StatsCard title="สถานะ" value="ปกติ" />)

    expect(screen.getByText('สถานะ')).toBeInTheDocument()
    expect(screen.getByText('ปกติ')).toBeInTheDocument()
  })

  test('uses light theme colors (not dark theme)', () => {
    const { container } = render(<StatsCard title="Test" value={0} />)

    const card = container.firstChild as HTMLElement
    expect(card.className).toContain('bg-white')
    expect(card.className).toContain('border-gray-100')
    expect(card.className).toContain('shadow-sm')
    expect(card.className).not.toContain('bg-zinc-900')
    expect(card.className).not.toContain('border-zinc-800')
  })

  test('title uses light theme text color', () => {
    render(<StatsCard title="ทดสอบ" value={5} />)

    const title = screen.getByText('ทดสอบ')
    expect(title.className).toContain('text-gray-500')
    expect(title.className).not.toContain('text-zinc-500')
  })

  test('value uses light theme text color', () => {
    render(<StatsCard title="Test" value={42} />)

    const value = screen.getByText('42')
    expect(value.className).toContain('text-gray-900')
    expect(value.className).not.toContain('text-zinc-100')
  })

  test('renders subtitle when provided', () => {
    render(<StatsCard title="Test" value={10} subtitle="จากทั้งหมด 50 ห้อง" />)

    expect(screen.getByText('จากทั้งหมด 50 ห้อง')).toBeInTheDocument()
  })

  test('hides subtitle when not provided', () => {
    const { container } = render(<StatsCard title="Test" value={10} />)

    const subtitleElements = container.querySelectorAll('.text-xs')
    expect(subtitleElements.length).toBe(0)
  })

  test('subtitle uses light theme text color', () => {
    render(<StatsCard title="Test" value={10} subtitle="subtitle text" />)

    const subtitle = screen.getByText('subtitle text')
    expect(subtitle.className).toContain('text-gray-400')
    expect(subtitle.className).not.toContain('text-zinc-600')
  })
})
