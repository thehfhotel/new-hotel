/**
 * Custom Test Utilities for Hotel Management System
 *
 * This file provides custom render functions and testing utilities
 * that wrap components with necessary providers.
 */

import React, { ReactElement, ReactNode } from 'react'
import { render, RenderOptions, RenderResult } from '@testing-library/react'

// ============================================================================
// Provider Wrapper Types
// ============================================================================

interface WrapperProps {
  children: ReactNode
}

// ============================================================================
// Default Provider Wrapper
// ============================================================================

/**
 * Default wrapper that can be extended with providers as needed.
 * Add any global providers (theme, context, etc.) here.
 */
function DefaultWrapper({ children }: WrapperProps): ReactElement {
  return <>{children}</>
}

// ============================================================================
// Custom Render Function
// ============================================================================

/**
 * Extended render options that include custom wrapper configuration
 */
interface CustomRenderOptions extends Omit<RenderOptions, 'wrapper'> {
  /**
   * Custom wrapper component. If not provided, uses DefaultWrapper.
   */
  wrapper?: React.ComponentType<WrapperProps>
}

/**
 * Custom render function that wraps components with necessary providers.
 *
 * Usage:
 * ```typescript
 * import { customRender, screen } from '@/__tests__/utils/testUtils'
 *
 * test('renders component', () => {
 *   customRender(<MyComponent />)
 *   expect(screen.getByText('Hello')).toBeInTheDocument()
 * })
 * ```
 *
 * @param ui - The React element to render
 * @param options - Render options including custom wrapper
 * @returns Render result with additional utilities
 */
function customRender(
  ui: ReactElement,
  options: CustomRenderOptions = {}
): RenderResult {
  const { wrapper: Wrapper = DefaultWrapper, ...renderOptions } = options

  return render(ui, {
    wrapper: Wrapper,
    ...renderOptions,
  })
}

// ============================================================================
// Modal Test Utilities
// ============================================================================

/**
 * Wrapper for testing modal components that need a portal target
 */
function ModalWrapper({ children }: WrapperProps): ReactElement {
  return (
    <>
      <div id="modal-root" />
      {children}
    </>
  )
}

/**
 * Render function specifically for modal components
 *
 * Usage:
 * ```typescript
 * import { renderModal, screen } from '@/__tests__/utils/testUtils'
 *
 * test('renders modal', () => {
 *   renderModal(<CheckOutModal isOpen={true} {...props} />)
 *   expect(screen.getByText('Modal Title')).toBeInTheDocument()
 * })
 * ```
 */
function renderModal(
  ui: ReactElement,
  options: Omit<CustomRenderOptions, 'wrapper'> = {}
): RenderResult {
  return customRender(ui, { wrapper: ModalWrapper, ...options })
}

// ============================================================================
// Form Test Utilities
// ============================================================================

/**
 * Wrapper for form components that need form context
 */
interface FormWrapperProps extends WrapperProps {
  onSubmit?: (e: React.FormEvent) => void
}

function createFormWrapper(onSubmit?: (e: React.FormEvent) => void) {
  return function FormWrapper({ children }: WrapperProps): ReactElement {
    const handleSubmit = (e: React.FormEvent) => {
      e.preventDefault()
      onSubmit?.(e)
    }

    return <form onSubmit={handleSubmit}>{children}</form>
  }
}

/**
 * Render function for form components
 *
 * Usage:
 * ```typescript
 * const handleSubmit = jest.fn()
 * renderWithForm(<FormInput />, { onSubmit: handleSubmit })
 * ```
 */
function renderWithForm(
  ui: ReactElement,
  options: Omit<CustomRenderOptions, 'wrapper'> & {
    onSubmit?: (e: React.FormEvent) => void
  } = {}
): RenderResult {
  const { onSubmit, ...renderOptions } = options
  const FormWrapper = createFormWrapper(onSubmit)
  return customRender(ui, { wrapper: FormWrapper, ...renderOptions })
}

// ============================================================================
// Test Data Utilities
// ============================================================================

/**
 * Creates a unique ID for test data
 */
let testIdCounter = 0
export function createTestId(): number {
  return ++testIdCounter
}

/**
 * Resets the test ID counter (call in beforeEach)
 */
export function resetTestIdCounter(): void {
  testIdCounter = 0
}

// ============================================================================
// Event Simulation Utilities
// ============================================================================

/**
 * Simulates typing in an input field with proper event handling
 *
 * Usage:
 * ```typescript
 * await typeInInput(screen.getByLabelText('Name'), 'Test Name')
 * ```
 */
export async function typeInInput(
  element: HTMLElement,
  text: string
): Promise<void> {
  const { fireEvent } = await import('@testing-library/react')

  fireEvent.focus(element)

  // Clear existing value
  if ((element as HTMLInputElement).value) {
    fireEvent.change(element, { target: { value: '' } })
  }

  // Type new value character by character (for better simulation)
  for (const char of text) {
    fireEvent.change(element, {
      target: { value: (element as HTMLInputElement).value + char },
    })
  }

  fireEvent.blur(element)
}

/**
 * Simulates selecting an option from a select element
 *
 * Usage:
 * ```typescript
 * await selectOption(screen.getByLabelText('Status'), 'confirmed')
 * ```
 */
export async function selectOption(
  element: HTMLElement,
  value: string
): Promise<void> {
  const { fireEvent } = await import('@testing-library/react')

  fireEvent.change(element, { target: { value } })
}

// ============================================================================
// Assertion Utilities
// ============================================================================

/**
 * Checks if an element contains Thai text
 *
 * Usage:
 * ```typescript
 * expect(containsThaiText(element)).toBe(true)
 * ```
 */
export function containsThaiText(element: HTMLElement): boolean {
  const thaiRegex = /[\u0E00-\u0E7F]/
  return thaiRegex.test(element.textContent || '')
}

/**
 * Finds elements containing specific Thai text pattern
 *
 * Usage:
 * ```typescript
 * const elements = findElementsWithThaiText(container, 'ลูกค้า')
 * ```
 */
export function findElementsWithThaiText(
  container: HTMLElement,
  text: string
): HTMLElement[] {
  const allElements = container.querySelectorAll('*')
  const matches: HTMLElement[] = []

  allElements.forEach((el) => {
    if (el.textContent?.includes(text)) {
      matches.push(el as HTMLElement)
    }
  })

  return matches
}

/**
 * Checks if a date is displayed in Buddhist Era format
 *
 * Usage:
 * ```typescript
 * expect(isBuddhistEraDate('15/01/2569')).toBe(true)
 * ```
 */
export function isBuddhistEraDate(dateString: string): boolean {
  // Thai Buddhist Era years are 543 years ahead of Gregorian
  // So years should be >= 2543 (2000 CE) and < 2700 (2157 CE)
  const yearMatch = dateString.match(/\d{4}/)
  if (!yearMatch) return false

  const year = parseInt(yearMatch[0], 10)
  return year >= 2543 && year < 2700
}

/**
 * Extracts Buddhist Era year from a date string
 *
 * Usage:
 * ```typescript
 * const year = extractBuddhistYear('15/01/2569') // Returns 2569
 * ```
 */
export function extractBuddhistYear(dateString: string): number | null {
  const yearMatch = dateString.match(/\d{4}/)
  if (!yearMatch) return null
  return parseInt(yearMatch[0], 10)
}

// ============================================================================
// Currency Format Utilities
// ============================================================================

/**
 * Checks if a string is formatted as Thai Baht currency
 *
 * Usage:
 * ```typescript
 * expect(isThaiCurrencyFormat(element.textContent)).toBe(true)
 * ```
 */
export function isThaiCurrencyFormat(text: string | null): boolean {
  if (!text) return false
  // Thai Baht format: either "฿1,000" or "1,000 บาท"
  return /฿[\d,]+/.test(text) || /[\d,]+\s*บาท/.test(text)
}

// ============================================================================
// Re-export everything from testing-library
// ============================================================================

export * from '@testing-library/react'

// Override the default render with our custom render
export { customRender as render, renderModal, renderWithForm }
