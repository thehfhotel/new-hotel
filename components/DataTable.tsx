'use client'

import { useState } from 'react'
import { ChevronUp, ChevronDown, ChevronLeft, ChevronRight } from 'lucide-react'

export interface Column<T> {
  key: keyof T | string
  header: string
  sortable?: boolean
  render?: (item: T) => React.ReactNode
  width?: string
}

export interface PaginationInfo {
  currentPage: number
  totalPages: number
  totalItems: number
  itemsPerPage: number
}

interface DataTableProps<T> {
  columns: Column<T>[]
  data: T[]
  onRowClick?: (item: T) => void
  pagination?: PaginationInfo
  onPageChange?: (page: number) => void
  loading?: boolean
  // Server-side sorting support
  onSort?: (column: string, direction: 'asc' | 'desc' | null) => void
  sortColumn?: string | null
  sortDirection?: 'asc' | 'desc' | null
}

type SortDirection = 'asc' | 'desc' | null

export default function DataTable<T extends object>({
  columns,
  data,
  onRowClick,
  pagination,
  onPageChange,
  loading = false,
  onSort,
  sortColumn: controlledSortColumn,
  sortDirection: controlledSortDirection,
}: DataTableProps<T>) {
  // Local state for client-side sorting (used when onSort is not provided)
  const [localSortColumn, setLocalSortColumn] = useState<string | null>(null)
  const [localSortDirection, setLocalSortDirection] = useState<SortDirection>(null)

  // Use controlled values if server-side sorting is enabled, otherwise use local state
  const isServerSide = !!onSort
  const sortColumn = isServerSide ? controlledSortColumn : localSortColumn
  const sortDirection = isServerSide ? controlledSortDirection : localSortDirection

  const handleSort = (columnKey: string, sortable?: boolean) => {
    if (!sortable) return

    let newDirection: SortDirection
    if (sortColumn === columnKey) {
      if (sortDirection === 'asc') {
        newDirection = 'desc'
      } else if (sortDirection === 'desc') {
        newDirection = null
      } else {
        newDirection = 'asc'
      }
    } else {
      newDirection = 'asc'
    }

    if (isServerSide) {
      // Server-side sorting: call the onSort callback
      onSort(columnKey, newDirection)
    } else {
      // Client-side sorting: update local state
      if (newDirection === null) {
        setLocalSortColumn(null)
        setLocalSortDirection(null)
      } else {
        setLocalSortColumn(columnKey)
        setLocalSortDirection(newDirection)
      }
    }
  }

  // Only apply client-side sorting when server-side sorting is not enabled
  const sortedData = isServerSide
    ? data
    : [...data].sort((a, b) => {
        if (!sortColumn || !sortDirection) return 0

        const aValue = (a as Record<string, unknown>)[sortColumn]
        const bValue = (b as Record<string, unknown>)[sortColumn]

        if (aValue === null || aValue === undefined) return 1
        if (bValue === null || bValue === undefined) return -1

        if (typeof aValue === 'string' && typeof bValue === 'string') {
          return sortDirection === 'asc'
            ? aValue.localeCompare(bValue, 'th')
            : bValue.localeCompare(aValue, 'th')
        }

        if (typeof aValue === 'number' && typeof bValue === 'number') {
          return sortDirection === 'asc' ? aValue - bValue : bValue - aValue
        }

        return 0
      })

  const renderPagination = () => {
    if (!pagination || !onPageChange) return null

    const { currentPage, totalPages, totalItems, itemsPerPage } = pagination
    const startItem = (currentPage - 1) * itemsPerPage + 1
    const endItem = Math.min(currentPage * itemsPerPage, totalItems)

    // Generate page numbers to show
    const pageNumbers: (number | string)[] = []
    const maxVisiblePages = 5

    if (totalPages <= maxVisiblePages) {
      for (let i = 1; i <= totalPages; i++) {
        pageNumbers.push(i)
      }
    } else {
      if (currentPage <= 3) {
        for (let i = 1; i <= 4; i++) {
          pageNumbers.push(i)
        }
        pageNumbers.push('...')
        pageNumbers.push(totalPages)
      } else if (currentPage >= totalPages - 2) {
        pageNumbers.push(1)
        pageNumbers.push('...')
        for (let i = totalPages - 3; i <= totalPages; i++) {
          pageNumbers.push(i)
        }
      } else {
        pageNumbers.push(1)
        pageNumbers.push('...')
        for (let i = currentPage - 1; i <= currentPage + 1; i++) {
          pageNumbers.push(i)
        }
        pageNumbers.push('...')
        pageNumbers.push(totalPages)
      }
    }

    return (
      <div className="flex items-center justify-between px-4 py-3 bg-white border-t border-gray-200">
        <div className="text-sm text-gray-500">
          แสดง <span className="font-medium text-gray-800">{startItem}</span> -{' '}
          <span className="font-medium text-gray-800">{endItem}</span> จาก{' '}
          <span className="font-medium text-gray-800">{totalItems.toLocaleString()}</span> รายการ
        </div>
        <div className="flex items-center gap-1">
          <button
            onClick={() => onPageChange(currentPage - 1)}
            disabled={currentPage === 1}
            className="p-2 rounded-md hover:bg-gray-100 disabled:opacity-50 disabled:cursor-not-allowed transition-colors text-gray-500"
            aria-label="หน้าก่อนหน้า"
          >
            <ChevronLeft className="w-5 h-5" />
          </button>

          {pageNumbers.map((page, index) =>
            typeof page === 'string' ? (
              <span key={`ellipsis-${index}`} className="px-3 py-1 text-gray-500">
                {page}
              </span>
            ) : (
              <button
                key={page}
                onClick={() => onPageChange(page)}
                className={`px-3 py-1 rounded-md transition-colors ${
                  currentPage === page
                    ? 'bg-red-600 text-white'
                    : 'hover:bg-gray-100 text-gray-500'
                }`}
              >
                {page}
              </button>
            )
          )}

          <button
            onClick={() => onPageChange(currentPage + 1)}
            disabled={currentPage === totalPages}
            className="p-2 rounded-md hover:bg-gray-100 disabled:opacity-50 disabled:cursor-not-allowed transition-colors text-gray-500"
            aria-label="หน้าถัดไป"
          >
            <ChevronRight className="w-5 h-5" />
          </button>
        </div>
      </div>
    )
  }

  return (
    <div className="bg-white rounded-lg border border-gray-200 overflow-hidden">
      <div className="overflow-x-auto">
        <table className="w-full">
          <thead className="bg-gray-100 border-b border-gray-300">
            <tr>
              {columns.map((column) => (
                <th
                  key={String(column.key)}
                  onClick={() => handleSort(String(column.key), column.sortable)}
                  className={`
                    px-4 py-3 text-left text-sm font-semibold text-gray-700
                    ${column.sortable ? 'cursor-pointer hover:bg-gray-200' : ''}
                    ${column.width || ''}
                  `}
                >
                  <div className="flex items-center gap-1">
                    {column.header}
                    {column.sortable && (
                      <span className="flex flex-col">
                        <ChevronUp
                          className={`w-3 h-3 -mb-1 ${
                            sortColumn === column.key && sortDirection === 'asc'
                              ? 'text-red-600'
                              : 'text-gray-400'
                          }`}
                        />
                        <ChevronDown
                          className={`w-3 h-3 ${
                            sortColumn === column.key && sortDirection === 'desc'
                              ? 'text-red-600'
                              : 'text-gray-400'
                          }`}
                        />
                      </span>
                    )}
                  </div>
                </th>
              ))}
            </tr>
          </thead>
          <tbody className="divide-y divide-gray-200">
            {loading ? (
              <tr>
                <td colSpan={columns.length} className="px-4 py-12 text-center">
                  <div className="flex items-center justify-center gap-2">
                    <div className="w-5 h-5 border-2 border-red-500 border-t-transparent rounded-full animate-spin"></div>
                    <span className="text-gray-500">กำลังโหลดข้อมูล...</span>
                  </div>
                </td>
              </tr>
            ) : sortedData.length === 0 ? (
              <tr>
                <td colSpan={columns.length} className="px-4 py-12 text-center text-gray-500">
                  ไม่พบข้อมูล
                </td>
              </tr>
            ) : (
              sortedData.map((item, index) => (
                <tr
                  key={index}
                  onClick={() => onRowClick?.(item)}
                  className={`
                    transition-colors
                    ${onRowClick ? 'cursor-pointer hover:bg-gray-100' : 'hover:bg-gray-100/50'}
                  `}
                >
                  {columns.map((column) => (
                    <td
                      key={String(column.key)}
                      className={`px-4 py-3 text-sm text-gray-700 ${column.width || ''}`}
                    >
                      {column.render
                        ? column.render(item)
                        : String(item[column.key as keyof T] ?? '-')}
                    </td>
                  ))}
                </tr>
              ))
            )}
          </tbody>
        </table>
      </div>
      {renderPagination()}
    </div>
  )
}
