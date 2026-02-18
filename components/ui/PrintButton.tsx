'use client'

import { useState } from 'react'
import { Printer, FileDown, ChevronDown, Loader2 } from 'lucide-react'

interface PrintButtonProps {
  /** Optional class name for styling */
  className?: string;
  /** Size variant */
  size?: 'sm' | 'md' | 'lg';
  /** Whether to show the dropdown with PDF option */
  showPdfOption?: boolean;
  /** Callback before printing (can be used to prepare the document) */
  onBeforePrint?: () => Promise<void> | void;
  /** Callback after printing */
  onAfterPrint?: () => void;
}

export default function PrintButton({
  className = '',
  size = 'md',
  showPdfOption = true,
  onBeforePrint,
  onAfterPrint,
}: PrintButtonProps) {
  const [isOpen, setIsOpen] = useState(false)
  const [isPrinting, setIsPrinting] = useState(false)

  // Size classes
  const sizeClasses = {
    sm: 'px-3 py-1.5 text-sm gap-1.5',
    md: 'px-4 py-2 text-base gap-2',
    lg: 'px-5 py-2.5 text-lg gap-2',
  }

  const iconSizes = {
    sm: 16,
    md: 18,
    lg: 20,
  }

  const handlePrint = async () => {
    setIsPrinting(true)
    try {
      if (onBeforePrint) {
        await onBeforePrint()
      }
      window.print()
      if (onAfterPrint) {
        onAfterPrint()
      }
    } finally {
      setIsPrinting(false)
      setIsOpen(false)
    }
  }

  const handleSavePdf = async () => {
    setIsPrinting(true)
    try {
      if (onBeforePrint) {
        await onBeforePrint()
      }
      // Trigger print dialog - user can select "Save as PDF" in the print dialog
      // This is the most reliable cross-browser way to generate PDF
      window.print()
      if (onAfterPrint) {
        onAfterPrint()
      }
    } finally {
      setIsPrinting(false)
      setIsOpen(false)
    }
  }

  // If not showing PDF option, render a simple button
  if (!showPdfOption) {
    return (
      <button
        onClick={handlePrint}
        disabled={isPrinting}
        className={`
          inline-flex items-center justify-center
          bg-red-600 text-white rounded-lg
          hover:bg-red-700
          disabled:opacity-50 disabled:cursor-not-allowed
          no-print
          ${sizeClasses[size]}
          ${className}
        `}
      >
        {isPrinting ? (
          <Loader2 size={iconSizes[size]} className="animate-spin" />
        ) : (
          <Printer size={iconSizes[size]} />
        )}
        <span>พิมพ์</span>
      </button>
    )
  }

  return (
    <div className="relative inline-block no-print">
      {/* Main Button Group */}
      <div className="inline-flex rounded-lg shadow-sm">
        {/* Print Button */}
        <button
          onClick={handlePrint}
          disabled={isPrinting}
          className={`
            inline-flex items-center justify-center
            bg-red-600 text-white
            hover:bg-red-700
            disabled:opacity-50 disabled:cursor-not-allowed
            rounded-l-lg border-r border-red-500
            ${sizeClasses[size]}
            ${className}
          `}
        >
          {isPrinting ? (
            <Loader2 size={iconSizes[size]} className="animate-spin" />
          ) : (
            <Printer size={iconSizes[size]} />
          )}
          <span>พิมพ์</span>
        </button>

        {/* Dropdown Toggle */}
        <button
          onClick={() => setIsOpen(!isOpen)}
          disabled={isPrinting}
          className={`
            inline-flex items-center justify-center
            bg-red-600 text-white
            hover:bg-red-700
            disabled:opacity-50 disabled:cursor-not-allowed
            rounded-r-lg px-2
          `}
        >
          <ChevronDown size={iconSizes[size]} />
        </button>
      </div>

      {/* Dropdown Menu */}
      {isOpen && (
        <>
          {/* Backdrop to close dropdown */}
          <div
            className="fixed inset-0 z-10"
            onClick={() => setIsOpen(false)}
          />

          {/* Dropdown Content */}
          <div className="absolute right-0 mt-1 w-48 bg-white rounded-lg shadow-lg border border-gray-200 z-20">
            <button
              onClick={handlePrint}
              className="w-full flex items-center gap-2 px-4 py-2.5 text-sm text-gray-700 hover:bg-gray-100 rounded-t-lg"
            >
              <Printer size={16} />
              <span>พิมพ์</span>
              <span className="text-gray-500 text-xs ml-auto">Print</span>
            </button>
            <button
              onClick={handleSavePdf}
              className="w-full flex items-center gap-2 px-4 py-2.5 text-sm text-gray-700 hover:bg-gray-100 rounded-b-lg border-t border-gray-200"
            >
              <FileDown size={16} />
              <span>บันทึก PDF</span>
              <span className="text-gray-500 text-xs ml-auto">Save PDF</span>
            </button>
          </div>
        </>
      )}
    </div>
  )
}
