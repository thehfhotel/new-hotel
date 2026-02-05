/**
 * Test Utilities Index
 *
 * Central export point for all test utilities.
 *
 * Usage:
 * ```typescript
 * import {
 *   createMockCustomer,
 *   createMockRoom,
 *   mockLucideIcons,
 *   mockFetch,
 *   waitForLoadingToComplete,
 *   render,
 *   screen,
 * } from '@/__tests__/utils'
 * ```
 */

// Mock Factories
export {
  // Customer
  createMockCustomer,
  createMockCustomerOption,
  createMockCustomers,
  // Room
  createMockRoom,
  createMockRoomOption,
  createMockRoomWithGuest,
  createMockRooms,
  // Booking
  createMockBooking,
  createMockBookingFormData,
  createMockBookings,
  // CheckIn
  createMockCheckIn,
  createMockCheckInDetails,
  // Inventory
  createMockInventoryItem,
  createMockInventoryItems,
  createMockInventoryTransaction,
  createMockRoomInventory,
  createMockRoomInventoryItem,
  // Invoice
  createMockInvoiceRoom,
  createMockInvoiceData,
  createMockReceiptData,
  createMockHotelInfo,
} from './mockFactories'

export type {
  MockCustomer,
  CheckInDetails,
} from './mockFactories'

// Common Mocks
export {
  // Lucide Icons
  mockLucideIcons,
  // Fetch Mocks
  mockFetch,
  mockFetchError,
  mockFetchNetworkError,
  // Date Mocks
  mockToday,
  mockTodayBuddhistYear,
  createMockDateString,
  createMockApiDateString,
  createMockDateRange,
  toBuddhistYear,
  // Next.js Mocks
  mockNextNavigation,
  createMockRouter,
  // Browser Mocks
  mockWindowPrint,
  mockMatchMedia,
  mockLocalStorage,
} from './commonMocks'

export type { MockFetchResponse } from './commonMocks'

// Test Utilities (custom render, etc.)
export {
  // Render functions
  render,
  renderModal,
  renderWithForm,
  // Test ID utilities
  createTestId,
  resetTestIdCounter,
  // Event utilities
  typeInInput,
  selectOption,
  // Assertion utilities
  containsThaiText,
  findElementsWithThaiText,
  isBuddhistEraDate,
  extractBuddhistYear,
  isThaiCurrencyFormat,
  // Re-exports from testing-library
  screen,
  fireEvent,
  waitFor,
  within,
  cleanup,
  act,
} from './testUtils'

// Async Utilities
export {
  // Timeouts
  TIMEOUTS,
  // Loading utilities
  waitForLoadingToComplete,
  waitForThaiLoadingToComplete,
  waitForSpinnerToDisappear,
  // Modal utilities
  waitForModalToOpen,
  waitForModalToClose,
  waitForBackdropToAppear,
  // Debounce utilities
  waitForDebounce,
  waitForSearchResults,
  // API utilities
  waitForApiCall,
  waitForApiEndpoint,
  // DOM utilities
  waitForElementToBeVisible,
  waitForElementToBeRemoved,
  waitForTextContent,
  // Form utilities
  waitForFormToBeValid,
  waitForFormSubmission,
  // Dropdown utilities
  waitForDropdownToOpen,
  waitForDropdownOptionsToLoad,
  // Table utilities
  waitForTableToLoad,
  waitForEmptyState,
} from './asyncUtils'
