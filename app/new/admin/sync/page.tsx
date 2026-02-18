'use client'

import { useState, useEffect, useCallback } from 'react'
import {
  RefreshCw,
  CheckCircle,
  AlertTriangle,
  XCircle,
  Clock,
  Database,
  Loader2,
  AlertCircle,
  Pause,
} from 'lucide-react'

interface SyncEntity {
  entityType: string
  lastSyncAt: string | null
  recordsSynced: number
  recordsAdded: number
  recordsUpdated: number
  recordsUnchanged: number
  syncDurationMs: number
  lastError: string | null
  lastErrorAt: string | null
  consecutiveFailures: number
  health: string
}

interface SyncStatusData {
  success: boolean
  syncEnabled: boolean
  overallHealth: string
  entities: SyncEntity[]
}

const entityLabels: Record<string, string> = {
  customers: 'ลูกค้า (Customers)',
  rooms: 'ห้องพัก (Rooms)',
  bookings: 'การจอง (Bookings)',
  checkins: 'เช็คอิน (Check-ins)',
}

function HealthBadge({ health }: { health: string }) {
  const config: Record<string, { icon: React.ReactNode; label: string; className: string }> = {
    healthy: {
      icon: <CheckCircle size={14} />,
      label: 'Healthy',
      className: 'bg-emerald-500/10 text-emerald-400 border-emerald-500/20',
    },
    warning: {
      icon: <AlertTriangle size={14} />,
      label: 'Warning',
      className: 'bg-amber-500/10 text-amber-400 border-amber-500/20',
    },
    error: {
      icon: <XCircle size={14} />,
      label: 'Error',
      className: 'bg-red-500/10 text-red-400 border-red-500/20',
    },
    stale: {
      icon: <Clock size={14} />,
      label: 'Stale',
      className: 'bg-amber-500/10 text-amber-400 border-amber-500/20',
    },
    pending: {
      icon: <Clock size={14} />,
      label: 'Pending',
      className: 'bg-zinc-500/10 text-zinc-400 border-zinc-500/20',
    },
    disabled: {
      icon: <Pause size={14} />,
      label: 'Disabled',
      className: 'bg-zinc-500/10 text-zinc-500 border-zinc-500/20',
    },
  }

  const c = config[health] || config.pending

  return (
    <span className={`inline-flex items-center gap-1 px-2 py-0.5 text-xs font-medium rounded-full border ${c.className}`}>
      {c.icon}
      {c.label}
    </span>
  )
}

function formatDateTime(dateStr: string | null): string {
  if (!dateStr) return '-'
  const date = new Date(dateStr + 'Z')
  return date.toLocaleString('th-TH', {
    year: 'numeric',
    month: 'short',
    day: 'numeric',
    hour: '2-digit',
    minute: '2-digit',
    second: '2-digit',
    timeZone: 'Asia/Bangkok',
  })
}

function formatDuration(ms: number): string {
  if (ms < 1000) return `${ms}ms`
  return `${(ms / 1000).toFixed(1)}s`
}

export default function SyncStatusPage() {
  const [data, setData] = useState<SyncStatusData | null>(null)
  const [loading, setLoading] = useState(true)
  const [error, setError] = useState<string | null>(null)

  const fetchStatus = useCallback(async () => {
    try {
      const res = await fetch('/api/new/sync/status')
      if (!res.ok) throw new Error(`HTTP ${res.status}`)
      const json = await res.json()
      setData(json)
      setError(null)
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Failed to fetch')
    } finally {
      setLoading(false)
    }
  }, [])

  useEffect(() => {
    fetchStatus()
    const interval = setInterval(fetchStatus, 30000)
    return () => clearInterval(interval)
  }, [fetchStatus])

  if (loading) {
    return (
      <div className="flex items-center justify-center min-h-[400px]">
        <Loader2 className="animate-spin text-zinc-400" size={32} />
      </div>
    )
  }

  if (error) {
    return (
      <div className="flex flex-col items-center justify-center min-h-[400px] gap-3">
        <AlertCircle className="text-red-400" size={32} />
        <p className="text-red-400">{error}</p>
        <button
          onClick={() => { setLoading(true); fetchStatus() }}
          className="px-4 py-2 bg-zinc-800 text-zinc-300 rounded-lg hover:bg-zinc-700 transition-colors"
        >
          ลองใหม่
        </button>
      </div>
    )
  }

  if (!data) return null

  return (
    <div className="max-w-5xl mx-auto space-y-6">
      {/* Header */}
      <div className="flex items-center justify-between">
        <div>
          <h1 className="text-2xl font-bold text-white flex items-center gap-3">
            <Database size={28} className="text-red-400" />
            Legacy Sync Status
          </h1>
          <p className="text-zinc-400 mt-1">
            สถานะการซิงค์ข้อมูลจาก SQL Server ไปยัง PostgreSQL
          </p>
        </div>
        <div className="flex items-center gap-3">
          <HealthBadge health={data.overallHealth} />
          <button
            onClick={() => { setLoading(true); fetchStatus() }}
            className="p-2 bg-zinc-800 text-zinc-400 rounded-lg hover:bg-zinc-700 hover:text-white transition-colors"
            title="Refresh"
          >
            <RefreshCw size={18} />
          </button>
        </div>
      </div>

      {/* Sync enabled banner */}
      {!data.syncEnabled && (
        <div className="bg-zinc-800/50 border border-zinc-700 rounded-xl p-4 flex items-center gap-3">
          <Pause className="text-zinc-500" size={20} />
          <span className="text-zinc-400">
            Sync ถูกปิดใช้งาน (SYNC_ENABLED=false)
          </span>
        </div>
      )}

      {/* Entity cards */}
      <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
        {data.entities.map((entity) => (
          <div
            key={entity.entityType}
            className="bg-zinc-900 border border-zinc-800 rounded-xl p-5 space-y-4"
          >
            {/* Entity header */}
            <div className="flex items-center justify-between">
              <h3 className="text-lg font-semibold text-white">
                {entityLabels[entity.entityType] || entity.entityType}
              </h3>
              <HealthBadge health={entity.health} />
            </div>

            {/* Stats grid */}
            <div className="grid grid-cols-2 gap-3">
              <div>
                <p className="text-xs text-zinc-500 uppercase tracking-wide">Total Synced</p>
                <p className="text-xl font-bold text-white">{entity.recordsSynced.toLocaleString()}</p>
              </div>
              <div>
                <p className="text-xs text-zinc-500 uppercase tracking-wide">Duration</p>
                <p className="text-xl font-bold text-white">{formatDuration(entity.syncDurationMs)}</p>
              </div>
              <div>
                <p className="text-xs text-zinc-500 uppercase tracking-wide">Added / Updated</p>
                <p className="text-sm text-zinc-300">
                  <span className="text-emerald-400">{entity.recordsAdded}</span>
                  {' / '}
                  <span className="text-amber-400">{entity.recordsUpdated}</span>
                </p>
              </div>
              <div>
                <p className="text-xs text-zinc-500 uppercase tracking-wide">Unchanged</p>
                <p className="text-sm text-zinc-300">{entity.recordsUnchanged.toLocaleString()}</p>
              </div>
            </div>

            {/* Last sync time */}
            <div className="pt-3 border-t border-zinc-800">
              <p className="text-xs text-zinc-500">
                Last sync: {formatDateTime(entity.lastSyncAt)}
              </p>

              {/* Error info */}
              {entity.lastError && (
                <div className="mt-2 p-2 bg-red-500/5 border border-red-500/10 rounded-lg">
                  <p className="text-xs text-red-400 font-mono break-all">{entity.lastError}</p>
                  <p className="text-xs text-zinc-500 mt-1">
                    at {formatDateTime(entity.lastErrorAt)} ({entity.consecutiveFailures} consecutive failures)
                  </p>
                </div>
              )}
            </div>
          </div>
        ))}
      </div>
    </div>
  )
}
