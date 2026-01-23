/**
 * @jest-environment jsdom
 */

import { render, screen, fireEvent } from '@testing-library/react'
import DataTable, { Column, PaginationInfo } from '@/components/DataTable'

// Mock lucide-react icons
jest.mock('lucide-react', () => ({
  ChevronUp: () => <span data-testid="chevron-up">^</span>,
  ChevronDown: () => <span data-testid="chevron-down">v</span>,
  ChevronLeft: () => <span data-testid="chevron-left">&lt;</span>,
  ChevronRight: () => <span data-testid="chevron-right">&gt;</span>,
}))

interface TestItem {
  id: number
  name: string
  amount: number
  date: string
}

const mockData: TestItem[] = [
  { id: 1, name: 'Item A', amount: 100, date: '2026-01-20' },
  { id: 2, name: 'Item B', amount: 200, date: '2026-01-21' },
  { id: 3, name: 'Item C', amount: 50, date: '2026-01-22' },
  { id: 4, name: 'Item D', amount: 150, date: '2026-01-23' },
]

const columns: Column<TestItem>[] = [
  { key: 'id', header: 'ID', sortable: true },
  { key: 'name', header: 'Name', sortable: true },
  { key: 'amount', header: 'Amount', sortable: true },
  { key: 'date', header: 'Date', sortable: false },
]

describe('DataTable Component', () => {
  describe('Rendering', () => {
    test('renders table with headers', () => {
      render(<DataTable columns={columns} data={mockData} />)

      expect(screen.getByText('ID')).toBeInTheDocument()
      expect(screen.getByText('Name')).toBeInTheDocument()
      expect(screen.getByText('Amount')).toBeInTheDocument()
      expect(screen.getByText('Date')).toBeInTheDocument()
    })

    test('renders all data rows', () => {
      render(<DataTable columns={columns} data={mockData} />)

      expect(screen.getByText('Item A')).toBeInTheDocument()
      expect(screen.getByText('Item B')).toBeInTheDocument()
      expect(screen.getByText('Item C')).toBeInTheDocument()
      expect(screen.getByText('Item D')).toBeInTheDocument()
    })

    test('renders custom cell content via render function', () => {
      const columnsWithRender: Column<TestItem>[] = [
        ...columns.slice(0, 2),
        {
          key: 'amount',
          header: 'Amount',
          render: (item) => <span data-testid="custom-amount">฿{item.amount}</span>,
        },
      ]

      render(<DataTable columns={columnsWithRender} data={mockData} />)

      const customAmounts = screen.getAllByTestId('custom-amount')
      expect(customAmounts.length).toBe(4)
      expect(customAmounts[0]).toHaveTextContent('฿100')
    })
  })

  describe('Sorting', () => {
    test('sorts data ascending on first click', () => {
      render(<DataTable columns={columns} data={mockData} />)

      // Click on Name header to sort
      const nameHeader = screen.getByText('Name')
      fireEvent.click(nameHeader)

      // Check data is sorted A-D
      const rows = screen.getAllByRole('row')
      // First row is header, so data rows start at index 1
      expect(rows[1]).toHaveTextContent('Item A')
      expect(rows[4]).toHaveTextContent('Item D')
    })

    test('sorts data descending on second click', () => {
      render(<DataTable columns={columns} data={mockData} />)

      const nameHeader = screen.getByText('Name')

      // First click - ascending
      fireEvent.click(nameHeader)
      // Second click - descending
      fireEvent.click(nameHeader)

      const rows = screen.getAllByRole('row')
      expect(rows[1]).toHaveTextContent('Item D')
      expect(rows[4]).toHaveTextContent('Item A')
    })

    test('resets sort on third click', () => {
      render(<DataTable columns={columns} data={mockData} />)

      const nameHeader = screen.getByText('Name')

      // Three clicks should reset to original order
      fireEvent.click(nameHeader) // asc
      fireEvent.click(nameHeader) // desc
      fireEvent.click(nameHeader) // reset

      const rows = screen.getAllByRole('row')
      // Original order: Item A, Item B, Item C, Item D
      expect(rows[1]).toHaveTextContent('Item A')
    })

    test('sorts numeric columns correctly', () => {
      render(<DataTable columns={columns} data={mockData} />)

      const amountHeader = screen.getByText('Amount')
      fireEvent.click(amountHeader)

      const rows = screen.getAllByRole('row')
      // Ascending: 50, 100, 150, 200
      expect(rows[1]).toHaveTextContent('50')
      expect(rows[4]).toHaveTextContent('200')
    })

    test('non-sortable columns do not sort on click', () => {
      render(<DataTable columns={columns} data={mockData} />)

      const dateHeader = screen.getByText('Date')
      fireEvent.click(dateHeader)

      // Data should remain in original order
      const rows = screen.getAllByRole('row')
      expect(rows[1]).toHaveTextContent('Item A')
    })
  })

  describe('Pagination', () => {
    const paginationInfo: PaginationInfo = {
      currentPage: 1,
      totalPages: 5,
      totalItems: 100,
      itemsPerPage: 20,
    }

    test('renders pagination info', () => {
      const mockPageChange = jest.fn()
      render(
        <DataTable
          columns={columns}
          data={mockData}
          pagination={paginationInfo}
          onPageChange={mockPageChange}
        />
      )

      expect(screen.getByText(/แสดง/)).toBeInTheDocument()
      expect(screen.getByText(/รายการ/)).toBeInTheDocument()
    })

    test('calls onPageChange when next page is clicked', () => {
      const mockPageChange = jest.fn()
      render(
        <DataTable
          columns={columns}
          data={mockData}
          pagination={paginationInfo}
          onPageChange={mockPageChange}
        />
      )

      const nextButton = screen.getByLabelText('หน้าถัดไป')
      fireEvent.click(nextButton)

      expect(mockPageChange).toHaveBeenCalledWith(2)
    })

    test('calls onPageChange when previous page is clicked', () => {
      const mockPageChange = jest.fn()
      render(
        <DataTable
          columns={columns}
          data={mockData}
          pagination={{ ...paginationInfo, currentPage: 2 }}
          onPageChange={mockPageChange}
        />
      )

      const prevButton = screen.getByLabelText('หน้าก่อนหน้า')
      fireEvent.click(prevButton)

      expect(mockPageChange).toHaveBeenCalledWith(1)
    })

    test('disables previous button on first page', () => {
      const mockPageChange = jest.fn()
      render(
        <DataTable
          columns={columns}
          data={mockData}
          pagination={paginationInfo}
          onPageChange={mockPageChange}
        />
      )

      const prevButton = screen.getByLabelText('หน้าก่อนหน้า')
      expect(prevButton).toBeDisabled()
    })

    test('disables next button on last page', () => {
      const mockPageChange = jest.fn()
      render(
        <DataTable
          columns={columns}
          data={mockData}
          pagination={{ ...paginationInfo, currentPage: 5 }}
          onPageChange={mockPageChange}
        />
      )

      const nextButton = screen.getByLabelText('หน้าถัดไป')
      expect(nextButton).toBeDisabled()
    })

    test('calls onPageChange when specific page number is clicked', () => {
      const mockPageChange = jest.fn()
      render(
        <DataTable
          columns={columns}
          data={mockData}
          pagination={paginationInfo}
          onPageChange={mockPageChange}
        />
      )

      // Find page buttons (not data cells) - they're between the navigation buttons
      const pageButtons = screen.getAllByRole('button').filter(
        (btn) => btn.textContent === '3' && !btn.hasAttribute('aria-label')
      )
      fireEvent.click(pageButtons[0])

      expect(mockPageChange).toHaveBeenCalledWith(3)
    })
  })

  describe('Empty State', () => {
    test('displays empty message when no data', () => {
      render(<DataTable columns={columns} data={[]} />)

      expect(screen.getByText('ไม่พบข้อมูล')).toBeInTheDocument()
    })
  })

  describe('Loading State', () => {
    test('displays loading indicator when loading', () => {
      render(<DataTable columns={columns} data={mockData} loading={true} />)

      expect(screen.getByText('กำลังโหลดข้อมูล...')).toBeInTheDocument()
    })

    test('does not display data rows when loading', () => {
      render(<DataTable columns={columns} data={mockData} loading={true} />)

      expect(screen.queryByText('Item A')).not.toBeInTheDocument()
    })
  })

  describe('Row Interaction', () => {
    test('calls onRowClick when row is clicked', () => {
      const mockRowClick = jest.fn()
      render(<DataTable columns={columns} data={mockData} onRowClick={mockRowClick} />)

      const rows = screen.getAllByRole('row')
      fireEvent.click(rows[1]) // Click first data row

      expect(mockRowClick).toHaveBeenCalledWith(
        expect.objectContaining({ id: 1, name: 'Item A' })
      )
    })

    test('applies hover styles when onRowClick is provided', () => {
      const mockRowClick = jest.fn()
      render(<DataTable columns={columns} data={mockData} onRowClick={mockRowClick} />)

      const rows = screen.getAllByRole('row')
      expect(rows[1]).toHaveClass('cursor-pointer')
    })
  })
})
