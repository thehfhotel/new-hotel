'use client'

import { useState, useEffect } from 'react'
import { format, parseISO } from 'date-fns'
import { th } from 'date-fns/locale'
import {
  ScrollText,
  Loader2,
  AlertCircle,
  RefreshCw,
  ExternalLink,
  Tag,
} from 'lucide-react'

interface Release {
  tag: string
  name: string
  body: string
  publishedAt: string
  url: string
}

export default function ChangelogPage() {
  const [releases, setReleases] = useState<Release[]>([])
  const [loading, setLoading] = useState(true)
  const [error, setError] = useState<string | null>(null)

  const fetchReleases = async () => {
    setLoading(true)
    setError(null)

    try {
      const response = await fetch('/api/changelog')
      const data = await response.json()

      if (!data.success) {
        throw new Error(data.error || 'Failed to fetch releases')
      }

      setReleases(data.releases)
    } catch (err) {
      setError(err instanceof Error ? err.message : 'เกิดข้อผิดพลาด')
    } finally {
      setLoading(false)
    }
  }

  useEffect(() => {
    fetchReleases()
  }, [])

  const formatDate = (dateString: string) => {
    try {
      const date = parseISO(dateString)
      return format(date, 'd MMMM yyyy', { locale: th })
    } catch {
      return dateString
    }
  }

  // Simple markdown to HTML converter for release notes
  const renderMarkdown = (text: string) => {
    if (!text) return null

    const lines = text.split('\n')
    const elements: React.JSX.Element[] = []
    let currentList: string[] = []
    let listKey = 0

    const flushList = () => {
      if (currentList.length > 0) {
        elements.push(
          <ul key={`list-${listKey++}`} className="list-disc list-inside space-y-1 ml-4 mb-4">
            {currentList.map((item, i) => (
              <li key={i} className="text-gray-700">{item}</li>
            ))}
          </ul>
        )
        currentList = []
      }
    }

    lines.forEach((line, index) => {
      const trimmedLine = line.trim()

      if (!trimmedLine) {
        flushList()
        return
      }

      // Headings
      if (trimmedLine.startsWith('### ')) {
        flushList()
        elements.push(
          <h4 key={`h4-${index}`} className="text-md font-semibold text-gray-800 mt-4 mb-2">
            {trimmedLine.slice(4)}
          </h4>
        )
        return
      }

      if (trimmedLine.startsWith('## ')) {
        flushList()
        elements.push(
          <h3 key={`h3-${index}`} className="text-lg font-semibold text-gray-900 mt-4 mb-2">
            {trimmedLine.slice(3)}
          </h3>
        )
        return
      }

      // List items
      if (trimmedLine.startsWith('- ') || trimmedLine.startsWith('* ')) {
        currentList.push(trimmedLine.slice(2))
        return
      }

      // Regular paragraph
      flushList()
      elements.push(
        <p key={`p-${index}`} className="text-gray-700 mb-2">{trimmedLine}</p>
      )
    })

    flushList()
    return elements
  }

  return (
    <div className="space-y-6">
      {/* Header */}
      <div className="flex items-center justify-between">
        <div className="flex items-center space-x-3">
          <ScrollText className="h-8 w-8 text-blue-600" />
          <h1 className="text-2xl font-bold text-gray-900">ประวัติการเปลี่ยนแปลง</h1>
        </div>
        <button
          onClick={fetchReleases}
          disabled={loading}
          className="flex items-center space-x-2 px-4 py-2 bg-blue-600 text-white rounded-lg hover:bg-blue-700 transition-colors disabled:opacity-50"
        >
          <RefreshCw className={`h-4 w-4 ${loading ? 'animate-spin' : ''}`} />
          <span>รีเฟรช</span>
        </button>
      </div>

      {/* Content */}
      {loading ? (
        <div className="flex items-center justify-center py-12">
          <Loader2 className="h-8 w-8 animate-spin text-blue-600" />
          <span className="ml-2 text-gray-600">กำลังโหลด...</span>
        </div>
      ) : error ? (
        <div className="bg-red-50 border border-red-200 rounded-lg p-6">
          <div className="flex items-center text-red-600">
            <AlertCircle className="h-6 w-6 mr-2" />
            <span>{error}</span>
          </div>
        </div>
      ) : releases.length === 0 ? (
        <div className="bg-gray-50 rounded-lg p-8 text-center">
          <ScrollText className="h-12 w-12 text-gray-400 mx-auto mb-4" />
          <p className="text-gray-500">ยังไม่มีประวัติการเปลี่ยนแปลง</p>
        </div>
      ) : (
        <div className="space-y-6">
          {releases.map((release) => (
            <div
              key={release.tag}
              className="bg-white rounded-lg shadow-sm border border-gray-200 overflow-hidden"
            >
              {/* Release Header */}
              <div className="bg-gray-50 px-6 py-4 border-b border-gray-200">
                <div className="flex items-center justify-between">
                  <div className="flex items-center space-x-3">
                    <Tag className="h-5 w-5 text-blue-600" />
                    <h2 className="text-xl font-bold text-gray-900">
                      {release.name}
                    </h2>
                  </div>
                  <div className="flex items-center space-x-4">
                    <span className="text-sm text-gray-500">
                      {formatDate(release.publishedAt)}
                    </span>
                    <a
                      href={release.url}
                      target="_blank"
                      rel="noopener noreferrer"
                      className="flex items-center space-x-1 text-blue-600 hover:text-blue-800 text-sm"
                    >
                      <span>GitHub</span>
                      <ExternalLink className="h-4 w-4" />
                    </a>
                  </div>
                </div>
              </div>

              {/* Release Body */}
              <div className="px-6 py-4">
                {release.body ? (
                  <div className="prose prose-sm max-w-none">
                    {renderMarkdown(release.body)}
                  </div>
                ) : (
                  <p className="text-gray-500 italic">ไม่มีรายละเอียด</p>
                )}
              </div>
            </div>
          ))}
        </div>
      )}
    </div>
  )
}
