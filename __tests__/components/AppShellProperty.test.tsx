/**
 * @jest-environment jsdom
 *
 * The wiring half of the `data-property` change: that AppShell actually puts
 * the resolved property on the HF One band's <script> tag, and OMITS the
 * attribute entirely (not `data-property=""` / `"undefined"`) for every
 * identity that does not name a place. The decision itself is covered by
 * __tests__/components/ShellProperty.test.ts.
 *
 * jsdom is required (not the repo default `node` environment) because this
 * renders React. Sidebar/BranchProvider are stubbed — neither is under test
 * here and both reach for browser + network state.
 */

import { render } from '@testing-library/react'
import AppShell from '@/components/AppShell'

const scriptProps: Record<string, unknown>[] = []

jest.mock('next/navigation', () => ({
  usePathname: () => '/',
}))

jest.mock('next/script', () => ({
  __esModule: true,
  default: (props: Record<string, unknown>) => {
    scriptProps.push(props)
    return null
  },
}))

jest.mock('@/components/Sidebar', () => ({
  __esModule: true,
  default: () => null,
  SIDEBAR_WIDTH: 240,
  SIDEBAR_COLLAPSED_WIDTH: 64,
}))

jest.mock('@/contexts/BranchContext', () => ({
  __esModule: true,
  BranchProvider: ({ children }: { children: React.ReactNode }) => children,
}))

function hfBarProps(): Record<string, unknown> {
  const found = scriptProps.find(
    (props) => typeof props.src === 'string' && props.src.includes('hf-bar.js'),
  )
  if (!found) throw new Error('hf-bar.js <Script> was not rendered')
  return found
}

beforeEach(() => {
  scriptProps.length = 0
})

it('passes data-property="hfville" for the HF Ville reception kiosk', () => {
  render(<AppShell shellProperty="hfville">child</AppShell>)
  expect(hfBarProps()['data-property']).toBe('hfville')
})

it('omits data-property entirely when no property was resolved', () => {
  render(<AppShell>child</AppShell>)
  expect(hfBarProps()).not.toHaveProperty('data-property')
})

it('keeps the existing band attributes untouched', () => {
  render(<AppShell>child</AppShell>)
  const props = hfBarProps()
  expect(props['data-app']).toBe('Hotel PMS')
  expect(props['data-module']).toBe('front-desk')
  expect(props.strategy).toBe('afterInteractive')
})
