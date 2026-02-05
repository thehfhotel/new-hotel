'use client'

import { useState, useEffect, useRef, useCallback } from 'react'
import {
  Search,
  X,
  User,
  Phone,
  CreditCard,
  ChevronDown,
  Loader2,
  UserPlus,
} from 'lucide-react'

export interface CustomerOption {
  id: number
  firstName: string
  lastName: string
  phone: string
  email: string
  idCard: string
}

interface CustomerPickerProps {
  value: CustomerOption | null
  onChange: (customer: CustomerOption | null) => void
  placeholder?: string
  disabled?: boolean
  onAddNew?: () => void
  className?: string
}

export default function CustomerPicker({
  value,
  onChange,
  placeholder = 'ค้นหาลูกค้า...',
  disabled = false,
  onAddNew,
  className = '',
}: CustomerPickerProps) {
  const [isOpen, setIsOpen] = useState(false)
  const [searchQuery, setSearchQuery] = useState('')
  const [debouncedSearch, setDebouncedSearch] = useState('')
  const [options, setOptions] = useState<CustomerOption[]>([])
  const [loading, setLoading] = useState(false)
  const [highlightedIndex, setHighlightedIndex] = useState(-1)

  const containerRef = useRef<HTMLDivElement>(null)
  const inputRef = useRef<HTMLInputElement>(null)
  const listRef = useRef<HTMLUListElement>(null)

  // Debounce search input
  useEffect(() => {
    const timer = setTimeout(() => {
      setDebouncedSearch(searchQuery)
    }, 300)

    return () => clearTimeout(timer)
  }, [searchQuery])

  // Fetch customers when search changes
  const fetchCustomers = useCallback(async () => {
    if (!debouncedSearch.trim() && !isOpen) {
      setOptions([])
      return
    }

    setLoading(true)
    try {
      const params = new URLSearchParams({
        limit: '20',
        activeOnly: 'true',
      })

      if (debouncedSearch.trim()) {
        params.append('search', debouncedSearch.trim())
      }

      const response = await fetch(`/api/new/customers?${params}`)
      if (response.ok) {
        const data = await response.json()
        const customers: CustomerOption[] = (data.data || []).map(
          (c: {
            id: number
            firstName?: string
            lastName?: string
            phone?: string
            email?: string
            idCard?: string
          }) => ({
            id: c.id,
            firstName: c.firstName || '',
            lastName: c.lastName || '',
            phone: c.phone || '',
            email: c.email || '',
            idCard: c.idCard || '',
          })
        )
        setOptions(customers)
      }
    } catch (error) {
      console.error('Error fetching customers:', error)
      setOptions([])
    } finally {
      setLoading(false)
    }
  }, [debouncedSearch, isOpen])

  useEffect(() => {
    if (isOpen) {
      fetchCustomers()
    }
  }, [fetchCustomers, isOpen])

  // Handle click outside to close
  useEffect(() => {
    const handleClickOutside = (event: MouseEvent) => {
      if (
        containerRef.current &&
        !containerRef.current.contains(event.target as Node)
      ) {
        setIsOpen(false)
      }
    }

    document.addEventListener('mousedown', handleClickOutside)
    return () => document.removeEventListener('mousedown', handleClickOutside)
  }, [])

  // Handle keyboard navigation
  const handleKeyDown = (e: React.KeyboardEvent) => {
    if (!isOpen) {
      if (e.key === 'Enter' || e.key === 'ArrowDown') {
        setIsOpen(true)
      }
      return
    }

    switch (e.key) {
      case 'ArrowDown':
        e.preventDefault()
        setHighlightedIndex((prev) =>
          prev < options.length - 1 ? prev + 1 : prev
        )
        break
      case 'ArrowUp':
        e.preventDefault()
        setHighlightedIndex((prev) => (prev > 0 ? prev - 1 : prev))
        break
      case 'Enter':
        e.preventDefault()
        if (highlightedIndex >= 0 && highlightedIndex < options.length) {
          handleSelect(options[highlightedIndex])
        }
        break
      case 'Escape':
        setIsOpen(false)
        break
    }
  }

  // Scroll highlighted item into view
  useEffect(() => {
    if (listRef.current && highlightedIndex >= 0) {
      const items = listRef.current.querySelectorAll('li')
      if (items[highlightedIndex]) {
        items[highlightedIndex].scrollIntoView({
          block: 'nearest',
        })
      }
    }
  }, [highlightedIndex])

  const handleSelect = (customer: CustomerOption) => {
    onChange(customer)
    setIsOpen(false)
    setSearchQuery('')
  }

  const handleClear = () => {
    onChange(null)
    setSearchQuery('')
  }

  const getDisplayName = (customer: CustomerOption) => {
    const name = `${customer.firstName} ${customer.lastName}`.trim()
    return name || '-'
  }

  const formatIdCard = (idCard: string) => {
    if (!idCard) return ''
    // Only show last 4 digits
    if (idCard.length > 4) {
      return `****${idCard.slice(-4)}`
    }
    return idCard
  }

  return (
    <div ref={containerRef} className={`relative ${className}`}>
      {/* Selected Value or Search Input */}
      <div
        className={`
          flex items-center gap-2 px-3 py-2 border rounded-lg transition-colors
          ${disabled ? 'bg-gray-100 cursor-not-allowed' : 'bg-white cursor-pointer'}
          ${isOpen ? 'border-blue-500 ring-2 ring-blue-200' : 'border-gray-300 hover:border-gray-400'}
        `}
        onClick={() => !disabled && setIsOpen(true)}
      >
        {value && !isOpen ? (
          // Show selected customer
          <>
            <User className="w-4 h-4 text-gray-400 flex-shrink-0" />
            <div className="flex-1 min-w-0">
              <span className="font-medium text-gray-800 truncate">
                {getDisplayName(value)}
              </span>
              {value.phone && (
                <span className="text-gray-500 text-sm ml-2">
                  ({value.phone})
                </span>
              )}
            </div>
            {!disabled && (
              <button
                type="button"
                onClick={(e) => {
                  e.stopPropagation()
                  handleClear()
                }}
                className="p-1 hover:bg-gray-100 rounded-full transition-colors"
                aria-label="ล้าง"
              >
                <X className="w-4 h-4 text-gray-400" />
              </button>
            )}
          </>
        ) : (
          // Show search input
          <>
            <Search className="w-4 h-4 text-gray-400 flex-shrink-0" />
            <input
              ref={inputRef}
              type="text"
              value={searchQuery}
              onChange={(e) => setSearchQuery(e.target.value)}
              onKeyDown={handleKeyDown}
              onFocus={() => setIsOpen(true)}
              placeholder={placeholder}
              disabled={disabled}
              className="flex-1 min-w-0 bg-transparent outline-none text-gray-800 placeholder-gray-400"
            />
            {searchQuery && (
              <button
                type="button"
                onClick={(e) => {
                  e.stopPropagation()
                  setSearchQuery('')
                }}
                className="p-1 hover:bg-gray-100 rounded-full transition-colors"
                aria-label="ล้าง"
              >
                <X className="w-4 h-4 text-gray-400" />
              </button>
            )}
            <ChevronDown
              className={`w-4 h-4 text-gray-400 transition-transform ${
                isOpen ? 'rotate-180' : ''
              }`}
            />
          </>
        )}
      </div>

      {/* Dropdown */}
      {isOpen && !disabled && (
        <div className="absolute z-50 w-full mt-1 bg-white border border-gray-200 rounded-lg shadow-lg overflow-hidden">
          {/* Loading */}
          {loading && (
            <div className="flex items-center justify-center py-4 text-gray-500">
              <Loader2 className="w-5 h-5 animate-spin mr-2" />
              <span className="text-sm">กำลังค้นหา...</span>
            </div>
          )}

          {/* Options List */}
          {!loading && options.length > 0 && (
            <ul
              ref={listRef}
              className="max-h-60 overflow-y-auto"
              role="listbox"
            >
              {options.map((customer, index) => (
                <li
                  key={customer.id}
                  onClick={() => handleSelect(customer)}
                  onMouseEnter={() => setHighlightedIndex(index)}
                  className={`
                    px-3 py-2 cursor-pointer transition-colors
                    ${index === highlightedIndex ? 'bg-blue-50' : 'hover:bg-gray-50'}
                    ${value?.id === customer.id ? 'bg-blue-100' : ''}
                  `}
                  role="option"
                  aria-selected={value?.id === customer.id}
                >
                  <div className="flex items-start gap-3">
                    <div className="w-8 h-8 bg-gray-100 rounded-full flex items-center justify-center flex-shrink-0 mt-0.5">
                      <User className="w-4 h-4 text-gray-500" />
                    </div>
                    <div className="flex-1 min-w-0">
                      <div className="font-medium text-gray-800">
                        {getDisplayName(customer)}
                      </div>
                      <div className="flex items-center gap-3 text-sm text-gray-500 mt-0.5">
                        {customer.phone && (
                          <span className="flex items-center gap-1">
                            <Phone className="w-3 h-3" />
                            {customer.phone}
                          </span>
                        )}
                        {customer.idCard && (
                          <span className="flex items-center gap-1">
                            <CreditCard className="w-3 h-3" />
                            {formatIdCard(customer.idCard)}
                          </span>
                        )}
                      </div>
                    </div>
                  </div>
                </li>
              ))}
            </ul>
          )}

          {/* No Results */}
          {!loading && options.length === 0 && debouncedSearch && (
            <div className="py-4 text-center text-gray-500">
              <User className="w-8 h-8 text-gray-300 mx-auto mb-2" />
              <p className="text-sm">ไม่พบลูกค้าที่ตรงกับ &quot;{debouncedSearch}&quot;</p>
            </div>
          )}

          {/* Empty State (no search) */}
          {!loading && options.length === 0 && !debouncedSearch && (
            <div className="py-4 text-center text-gray-500">
              <Search className="w-8 h-8 text-gray-300 mx-auto mb-2" />
              <p className="text-sm">พิมพ์เพื่อค้นหาลูกค้า</p>
            </div>
          )}

          {/* Add New Button */}
          {onAddNew && (
            <div className="border-t">
              <button
                type="button"
                onClick={() => {
                  setIsOpen(false)
                  onAddNew()
                }}
                className="w-full flex items-center justify-center gap-2 px-3 py-2 text-blue-600 hover:bg-blue-50 transition-colors"
              >
                <UserPlus className="w-4 h-4" />
                <span className="text-sm font-medium">เพิ่มลูกค้าใหม่</span>
              </button>
            </div>
          )}
        </div>
      )}
    </div>
  )
}
