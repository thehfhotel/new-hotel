/**
 * Async Testing Utilities for Hotel Management System
 *
 * This file provides utilities for handling async operations in tests,
 * including waiting for loading states, modals, and debounced operations.
 */

import { waitFor, screen, within } from '@testing-library/react'

// ============================================================================
// Default Timeouts
// ============================================================================

/**
 * Default timeout values used throughout the application
 */
export const TIMEOUTS = {
  /** Default debounce time for search inputs (ms) */
  DEBOUNCE: 300,
  /** Default timeout for API calls (ms) */
  API_CALL: 1000,
  /** Default timeout for modal animations (ms) */
  MODAL_ANIMATION: 200,
  /** Default timeout for loading states (ms) */
  LOADING: 3000,
  /** Extended timeout for long operations (ms) */
  EXTENDED: 5000,
} as const

// ============================================================================
// Loading State Utilities
// ============================================================================

/**
 * Waits for a loading indicator to disappear
 *
 * Usage:
 * ```typescript
 * await waitForLoadingToComplete()
 * // or with custom text
 * await waitForLoadingToComplete('Loading data...')
 * ```
 *
 * @param loadingText - Text or regex to identify loading state
 * @param options - waitFor options
 */
export async function waitForLoadingToComplete(
  loadingText: string | RegExp = /loading|loading/i,
  options: { timeout?: number; container?: HTMLElement } = {}
): Promise<void> {
  const { timeout = TIMEOUTS.LOADING, container } = options

  await waitFor(
    () => {
      const queryFn = container ? within(container) : screen
      const loadingElement = queryFn.queryByText(loadingText)
      expect(loadingElement).not.toBeInTheDocument()
    },
    { timeout }
  )
}

/**
 * Waits for Thai loading text to disappear
 *
 * Common Thai loading texts:
 * - กำลังโหลด... (Loading...)
 * - กำลังค้นหา... (Searching...)
 * - กำลังบันทึก... (Saving...)
 */
export async function waitForThaiLoadingToComplete(
  options: { timeout?: number; container?: HTMLElement } = {}
): Promise<void> {
  const thaiLoadingPattern = /กำลัง(โหลด|ค้นหา|บันทึก|ประมวลผล)/

  await waitForLoadingToComplete(thaiLoadingPattern, options)
}

/**
 * Waits for the Loader2 icon (spinning loader) to disappear
 */
export async function waitForSpinnerToDisappear(
  options: { timeout?: number } = {}
): Promise<void> {
  const { timeout = TIMEOUTS.LOADING } = options

  await waitFor(
    () => {
      const spinner = screen.queryByTestId('icon-loader')
      expect(spinner).not.toBeInTheDocument()
    },
    { timeout }
  )
}

// ============================================================================
// Modal Utilities
// ============================================================================

/**
 * Waits for a modal to appear in the DOM
 *
 * Usage:
 * ```typescript
 * await waitForModalToOpen('Modal Title')
 * // or with container query
 * await waitForModalToOpen('Modal Title', { findByRole: 'dialog' })
 * ```
 *
 * @param titleText - Text in the modal title/header
 * @param options - Additional options
 */
export async function waitForModalToOpen(
  titleText: string | RegExp,
  options: { timeout?: number; findByRole?: string } = {}
): Promise<HTMLElement> {
  const { timeout = TIMEOUTS.MODAL_ANIMATION } = options

  let modalElement: HTMLElement

  await waitFor(
    () => {
      modalElement = screen.getByText(titleText)
      expect(modalElement).toBeInTheDocument()
      expect(modalElement).toBeVisible()
    },
    { timeout }
  )

  return modalElement!
}

/**
 * Waits for a modal to close/disappear
 *
 * Usage:
 * ```typescript
 * fireEvent.click(closeButton)
 * await waitForModalToClose('Modal Title')
 * ```
 *
 * @param titleText - Text in the modal title that should disappear
 * @param options - waitFor options
 */
export async function waitForModalToClose(
  titleText: string | RegExp,
  options: { timeout?: number } = {}
): Promise<void> {
  const { timeout = TIMEOUTS.MODAL_ANIMATION } = options

  await waitFor(
    () => {
      const modalTitle = screen.queryByText(titleText)
      expect(modalTitle).not.toBeInTheDocument()
    },
    { timeout }
  )
}

/**
 * Waits for a modal backdrop to appear
 */
export async function waitForBackdropToAppear(
  options: { timeout?: number } = {}
): Promise<HTMLElement> {
  const { timeout = TIMEOUTS.MODAL_ANIMATION } = options

  let backdrop: HTMLElement

  await waitFor(
    () => {
      // Modal backdrops typically have bg-opacity or bg-black classes
      backdrop = document.querySelector(
        '[class*="bg-opacity"], [class*="bg-black/"]'
      ) as HTMLElement
      expect(backdrop).toBeInTheDocument()
    },
    { timeout }
  )

  return backdrop!
}

// ============================================================================
// Debounce Utilities
// ============================================================================

/**
 * Waits for debounce to complete (default 300ms)
 *
 * Usage:
 * ```typescript
 * fireEvent.change(searchInput, { target: { value: 'test' } })
 * await waitForDebounce()
 * // Now the debounced search should have executed
 * ```
 *
 * @param debounceTime - Time to wait in milliseconds
 */
export async function waitForDebounce(
  debounceTime: number = TIMEOUTS.DEBOUNCE
): Promise<void> {
  await new Promise((resolve) => setTimeout(resolve, debounceTime + 50))
}

/**
 * Waits for search results to appear after debounced search
 *
 * Usage:
 * ```typescript
 * await typeAndWaitForSearchResults(searchInput, 'query', 'Expected Result')
 * ```
 *
 * @param input - The search input element
 * @param query - The search query to type
 * @param expectedText - Text expected to appear in results
 */
export async function waitForSearchResults(
  expectedText: string | RegExp,
  options: { timeout?: number; container?: HTMLElement } = {}
): Promise<HTMLElement> {
  const { timeout = TIMEOUTS.API_CALL + TIMEOUTS.DEBOUNCE, container } = options

  let resultElement: HTMLElement

  await waitFor(
    () => {
      const queryFn = container ? within(container) : screen
      resultElement = queryFn.getByText(expectedText)
      expect(resultElement).toBeInTheDocument()
    },
    { timeout }
  )

  return resultElement!
}

// ============================================================================
// API Call Utilities
// ============================================================================

/**
 * Waits for an API call to complete (fetch mock to be called)
 *
 * Usage:
 * ```typescript
 * const fetchMock = mockFetch({ '/api/data': { data: mockData } })
 * global.fetch = fetchMock
 *
 * render(<Component />)
 * await waitForApiCall(fetchMock)
 * ```
 *
 * @param fetchMock - The jest mock function for fetch
 * @param expectedCalls - Minimum number of expected calls
 * @param options - waitFor options
 */
export async function waitForApiCall(
  fetchMock: jest.Mock,
  expectedCalls: number = 1,
  options: { timeout?: number } = {}
): Promise<void> {
  const { timeout = TIMEOUTS.API_CALL } = options

  await waitFor(
    () => {
      expect(fetchMock).toHaveBeenCalledTimes(expectedCalls)
    },
    { timeout }
  )
}

/**
 * Waits for a specific API endpoint to be called
 *
 * Usage:
 * ```typescript
 * await waitForApiEndpoint(fetchMock, '/api/customers')
 * ```
 *
 * @param fetchMock - The jest mock function for fetch
 * @param endpoint - The endpoint URL pattern to wait for
 * @param options - waitFor options
 */
export async function waitForApiEndpoint(
  fetchMock: jest.Mock,
  endpoint: string,
  options: { timeout?: number } = {}
): Promise<void> {
  const { timeout = TIMEOUTS.API_CALL } = options

  await waitFor(
    () => {
      const calls = fetchMock.mock.calls
      const matchingCall = calls.find(
        ([url]: [string]) => url.includes(endpoint)
      )
      expect(matchingCall).toBeDefined()
    },
    { timeout }
  )
}

// ============================================================================
// DOM State Utilities
// ============================================================================

/**
 * Waits for an element to become visible
 *
 * @param element - The element to wait for
 * @param options - waitFor options
 */
export async function waitForElementToBeVisible(
  getElement: () => HTMLElement,
  options: { timeout?: number } = {}
): Promise<HTMLElement> {
  const { timeout = TIMEOUTS.LOADING } = options

  let element: HTMLElement

  await waitFor(
    () => {
      element = getElement()
      expect(element).toBeVisible()
    },
    { timeout }
  )

  return element!
}

/**
 * Waits for an element to be removed from the DOM
 *
 * @param getElement - Function that queries for the element
 * @param options - waitFor options
 */
export async function waitForElementToBeRemoved(
  getElement: () => HTMLElement | null,
  options: { timeout?: number } = {}
): Promise<void> {
  const { timeout = TIMEOUTS.LOADING } = options

  await waitFor(
    () => {
      const element = getElement()
      expect(element).not.toBeInTheDocument()
    },
    { timeout }
  )
}

/**
 * Waits for an element to have specific text content
 *
 * @param getElement - Function that queries for the element
 * @param expectedText - Expected text content
 * @param options - waitFor options
 */
export async function waitForTextContent(
  getElement: () => HTMLElement,
  expectedText: string | RegExp,
  options: { timeout?: number } = {}
): Promise<void> {
  const { timeout = TIMEOUTS.LOADING } = options

  await waitFor(
    () => {
      const element = getElement()
      if (typeof expectedText === 'string') {
        expect(element.textContent).toContain(expectedText)
      } else {
        expect(element.textContent).toMatch(expectedText)
      }
    },
    { timeout }
  )
}

// ============================================================================
// Form State Utilities
// ============================================================================

/**
 * Waits for a form to be in a valid/submittable state
 *
 * @param submitButton - The submit button element
 * @param options - waitFor options
 */
export async function waitForFormToBeValid(
  submitButton: HTMLElement,
  options: { timeout?: number } = {}
): Promise<void> {
  const { timeout = TIMEOUTS.LOADING } = options

  await waitFor(
    () => {
      expect(submitButton).not.toBeDisabled()
    },
    { timeout }
  )
}

/**
 * Waits for form submission to complete (button to be re-enabled)
 *
 * @param submitButton - The submit button element
 * @param options - waitFor options
 */
export async function waitForFormSubmission(
  submitButton: HTMLElement,
  options: { timeout?: number } = {}
): Promise<void> {
  const { timeout = TIMEOUTS.API_CALL } = options

  // First wait for button to be disabled (submission started)
  await waitFor(
    () => {
      expect(submitButton).toBeDisabled()
    },
    { timeout: 100 }
  ).catch(() => {
    // Button might already be disabled or enabled again
  })

  // Then wait for button to be enabled again (submission complete)
  await waitFor(
    () => {
      expect(submitButton).not.toBeDisabled()
    },
    { timeout }
  )
}

// ============================================================================
// Dropdown/Select Utilities
// ============================================================================

/**
 * Waits for a dropdown to open and display options
 *
 * @param optionText - Text of an option expected to appear
 * @param options - waitFor options
 */
export async function waitForDropdownToOpen(
  optionText: string | RegExp,
  options: { timeout?: number; container?: HTMLElement } = {}
): Promise<HTMLElement> {
  const { timeout = TIMEOUTS.MODAL_ANIMATION, container } = options

  let optionElement: HTMLElement

  await waitFor(
    () => {
      const queryFn = container ? within(container) : screen
      optionElement = queryFn.getByText(optionText)
      expect(optionElement).toBeVisible()
    },
    { timeout }
  )

  return optionElement!
}

/**
 * Waits for dropdown options to load (e.g., from API)
 *
 * @param minOptions - Minimum number of options expected
 * @param listboxSelector - Selector for the listbox/options container
 * @param options - waitFor options
 */
export async function waitForDropdownOptionsToLoad(
  minOptions: number = 1,
  listboxSelector: string = '[role="listbox"] li, [role="option"]',
  options: { timeout?: number } = {}
): Promise<HTMLElement[]> {
  const { timeout = TIMEOUTS.API_CALL + TIMEOUTS.DEBOUNCE } = options

  let optionElements: NodeListOf<HTMLElement>

  await waitFor(
    () => {
      optionElements = document.querySelectorAll(listboxSelector)
      expect(optionElements.length).toBeGreaterThanOrEqual(minOptions)
    },
    { timeout }
  )

  return Array.from(optionElements!)
}

// ============================================================================
// Table/List Utilities
// ============================================================================

/**
 * Waits for table data to load
 *
 * @param expectedRows - Minimum number of data rows expected
 * @param options - waitFor options
 */
export async function waitForTableToLoad(
  expectedRows: number = 1,
  options: { timeout?: number; container?: HTMLElement } = {}
): Promise<HTMLElement[]> {
  const { timeout = TIMEOUTS.API_CALL, container } = options

  let rows: HTMLElement[]

  await waitFor(
    () => {
      const queryFn = container ? within(container) : screen
      rows = queryFn.getAllByRole('row')
      // Subtract 1 for header row
      expect(rows.length - 1).toBeGreaterThanOrEqual(expectedRows)
    },
    { timeout }
  )

  return rows!
}

/**
 * Waits for an empty state message to appear
 *
 * @param emptyText - Text indicating empty state
 * @param options - waitFor options
 */
export async function waitForEmptyState(
  emptyText: string | RegExp = /ไม่พบ|ไม่มี|no data|not found/i,
  options: { timeout?: number } = {}
): Promise<HTMLElement> {
  const { timeout = TIMEOUTS.API_CALL } = options

  let emptyElement: HTMLElement

  await waitFor(
    () => {
      emptyElement = screen.getByText(emptyText)
      expect(emptyElement).toBeVisible()
    },
    { timeout }
  )

  return emptyElement!
}
