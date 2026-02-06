'use client'

import { useState, useEffect, useCallback } from 'react'
import {
  BarChart3,
  Loader2,
  AlertCircle,
  Calendar,
  RefreshCw,
  TrendingUp,
  DollarSign,
  Percent,
  Clock,
} from 'lucide-react'
import DatePicker from 'react-datepicker'
import 'react-datepicker/dist/react-datepicker.css'
import {
  LineChart,
  Line,
  BarChart,
  Bar,
  XAxis,
  YAxis,
  CartesianGrid,
  Tooltip,
  ResponsiveContainer,
  PieChart,
  Pie,
  Cell,
  Legend,
} from 'recharts'
import { RevenueDataPoint, OccupancyResponse, RoomTypeRevenue, GroupBy } from '@/types/reports'

// Format date for API (YYYY-MM-DD)
function formatDateForApi(date: Date): string {
  const year = date.getFullYear()
  const month = String(date.getMonth() + 1).padStart(2, '0')
  const day = String(date.getDate()).padStart(2, '0')
  return `${year}-${month}-${day}`
}

// Format currency with Thai Baht
function formatCurrency(amount: number): string {
  return new Intl.NumberFormat('th-TH', {
    style: 'currency',
    currency: 'THB',
    minimumFractionDigits: 0,
    maximumFractionDigits: 0,
  }).format(amount)
}

// Format percentage
function formatPercent(value: number): string {
  return `${value.toFixed(1)}%`
}

// Colors for pie chart
const CHART_COLORS = [
  '#3B82F6', // blue-500
  '#10B981', // emerald-500
  '#F59E0B', // amber-500
  '#EF4444', // red-500
  '#8B5CF6', // violet-500
  '#EC4899', // pink-500
  '#06B6D4', // cyan-500
  '#84CC16', // lime-500
]

// Get default date range (last 30 days)
function getDefaultDateRange(): [Date, Date] {
  const end = new Date()
  const start = new Date()
  start.setDate(start.getDate() - 30)
  return [start, end]
}

export default function ReportsPage() {
  // Date range state
  const [dateRange, setDateRange] = useState<[Date | null, Date | null]>(getDefaultDateRange)
  const [startDate, endDate] = dateRange

  // Group by state
  const [groupBy, setGroupBy] = useState<GroupBy>('day')

  // Data states
  const [revenueData, setRevenueData] = useState<RevenueDataPoint[]>([])
  const [occupancyData, setOccupancyData] = useState<OccupancyResponse | null>(null)
  const [roomTypeRevenue, setRoomTypeRevenue] = useState<RoomTypeRevenue[]>([])

  // Loading states
  const [loading, setLoading] = useState(true)
  const [error, setError] = useState<string | null>(null)

  // Fetch all report data
  const fetchReportData = useCallback(async () => {
    if (!startDate || !endDate) return

    setLoading(true)
    setError(null)

    try {
      const fromDate = formatDateForApi(startDate)
      const toDate = formatDateForApi(endDate)

      // Fetch all data in parallel
      const [revenueRes, occupancyRes, roomTypeRes] = await Promise.all([
        fetch(`/api/new/reports/revenue?from=${fromDate}&to=${toDate}&groupBy=${groupBy}`),
        fetch(`/api/new/reports/occupancy?from=${fromDate}&to=${toDate}`),
        fetch(`/api/new/reports/revenue-by-room-type?from=${fromDate}&to=${toDate}`),
      ])

      const [revenueJson, occupancyJson, roomTypeJson] = await Promise.all([
        revenueRes.json(),
        occupancyRes.json(),
        roomTypeRes.json(),
      ])

      if (revenueJson.success) {
        setRevenueData(revenueJson.data || [])
      }

      if (occupancyJson.success) {
        setOccupancyData(occupancyJson)
      }

      if (roomTypeJson.success) {
        setRoomTypeRevenue(roomTypeJson.data || [])
      }
    } catch (err) {
      setError(err instanceof Error ? err.message : 'เกิดข้อผิดพลาดในการโหลดข้อมูล')
    } finally {
      setLoading(false)
    }
  }, [startDate, endDate, groupBy])

  useEffect(() => {
    fetchReportData()
  }, [fetchReportData])

  // Custom tooltip for revenue chart
  const RevenueTooltip = ({ active, payload, label }: { active?: boolean; payload?: { value: number; dataKey: string }[]; label?: string }) => {
    if (active && payload && payload.length) {
      return (
        <div className="bg-white p-3 rounded-lg shadow-lg border border-gray-200">
          <p className="text-sm font-medium text-gray-900">{label}</p>
          {payload.map((entry, index) => (
            <p key={index} className="text-sm text-gray-600">
              {entry.dataKey === 'revenue' ? 'รายได้: ' : 'จำนวนจอง: '}
              <span className="font-medium">
                {entry.dataKey === 'revenue' ? formatCurrency(entry.value) : entry.value}
              </span>
            </p>
          ))}
        </div>
      )
    }
    return null
  }

  // Custom tooltip for pie chart
  const PieTooltip = ({ active, payload }: { active?: boolean; payload?: { name: string; value: number; payload: RoomTypeRevenue }[] }) => {
    if (active && payload && payload.length) {
      const data = payload[0].payload
      return (
        <div className="bg-white p-3 rounded-lg shadow-lg border border-gray-200">
          <p className="text-sm font-medium text-gray-900">{data.roomType}</p>
          <p className="text-sm text-gray-600">
            รายได้: <span className="font-medium">{formatCurrency(data.revenue)}</span>
          </p>
          <p className="text-sm text-gray-600">
            สัดส่วน: <span className="font-medium">{formatPercent(data.percentage)}</span>
          </p>
        </div>
      )
    }
    return null
  }

  return (
    <div className="space-y-6">
      {/* Page Header */}
      <div className="flex items-center justify-between">
        <div className="flex items-center gap-3">
          <BarChart3 className="w-8 h-8 text-blue-600" />
          <div>
            <h1 className="text-2xl font-bold text-gray-800">รายงาน</h1>
            <p className="text-gray-600">Reports Dashboard</p>
          </div>
        </div>
        <button
          onClick={fetchReportData}
          disabled={loading}
          className="flex items-center space-x-2 px-4 py-2 border border-gray-300 text-gray-700 rounded-lg hover:bg-gray-50 disabled:opacity-50"
        >
          <RefreshCw className={`h-4 w-4 ${loading ? 'animate-spin' : ''}`} />
          <span className="hidden sm:inline">รีเฟรช</span>
        </button>
      </div>

      {/* Filters */}
      <div className="bg-white rounded-lg shadow p-4">
        <div className="flex flex-wrap items-center gap-4">
          {/* Date Range Picker */}
          <div className="flex items-center gap-2">
            <span className="text-sm font-medium text-gray-700">ช่วงวันที่:</span>
            <div className="relative">
              <div className="flex items-center border border-gray-300 rounded-lg overflow-hidden focus-within:ring-2 focus-within:ring-blue-500 focus-within:border-blue-500">
                <div className="flex items-center px-3 bg-gray-50 border-r border-gray-300 py-2">
                  <Calendar className="h-4 w-4 text-gray-400" />
                </div>
                <DatePicker
                  selectsRange
                  startDate={startDate}
                  endDate={endDate}
                  onChange={(update) => setDateRange(update)}
                  placeholderText="เลือกช่วงวันที่"
                  className="w-48 px-3 py-2 border-0 focus:ring-0 text-sm focus:outline-none"
                  dateFormat="dd/MM/yyyy"
                  isClearable
                />
              </div>
            </div>
          </div>

          {/* Period Grouping Buttons */}
          <div className="flex items-center gap-2">
            <span className="text-sm font-medium text-gray-700">แบ่งตาม:</span>
            <div className="flex rounded-lg border border-gray-300 overflow-hidden">
              <button
                onClick={() => setGroupBy('day')}
                className={`px-4 py-2 text-sm font-medium transition-colors ${
                  groupBy === 'day'
                    ? 'bg-blue-600 text-white'
                    : 'bg-white text-gray-700 hover:bg-gray-50'
                }`}
              >
                วัน
              </button>
              <button
                onClick={() => setGroupBy('week')}
                className={`px-4 py-2 text-sm font-medium border-l border-gray-300 transition-colors ${
                  groupBy === 'week'
                    ? 'bg-blue-600 text-white'
                    : 'bg-white text-gray-700 hover:bg-gray-50'
                }`}
              >
                สัปดาห์
              </button>
              <button
                onClick={() => setGroupBy('month')}
                className={`px-4 py-2 text-sm font-medium border-l border-gray-300 transition-colors ${
                  groupBy === 'month'
                    ? 'bg-blue-600 text-white'
                    : 'bg-white text-gray-700 hover:bg-gray-50'
                }`}
              >
                เดือน
              </button>
            </div>
          </div>
        </div>
      </div>

      {/* Error Message */}
      {error && (
        <div className="flex items-center gap-2 p-4 bg-red-50 border border-red-200 rounded-lg text-red-700">
          <AlertCircle className="w-5 h-5 flex-shrink-0" />
          <span>{error}</span>
        </div>
      )}

      {/* Stats Cards */}
      <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-4">
        {/* Occupancy Rate */}
        <div className="bg-white rounded-lg shadow p-6">
          <div className="flex items-center justify-between">
            <div>
              <p className="text-sm text-gray-600">อัตราการเข้าพัก</p>
              <p className="text-3xl font-bold text-gray-800">
                {loading ? '-' : occupancyData ? formatPercent(occupancyData.occupancyRate) : '-'}
              </p>
              <p className="text-xs text-gray-500 mt-1">Occupancy Rate</p>
            </div>
            <div className="w-12 h-12 bg-blue-100 rounded-full flex items-center justify-center">
              <Percent className="w-6 h-6 text-blue-600" />
            </div>
          </div>
        </div>

        {/* ADR */}
        <div className="bg-white rounded-lg shadow p-6">
          <div className="flex items-center justify-between">
            <div>
              <p className="text-sm text-gray-600">ราคาเฉลี่ยต่อคืน</p>
              <p className="text-3xl font-bold text-gray-800">
                {loading ? '-' : occupancyData ? formatCurrency(occupancyData.adr) : '-'}
              </p>
              <p className="text-xs text-gray-500 mt-1">ADR (Average Daily Rate)</p>
            </div>
            <div className="w-12 h-12 bg-green-100 rounded-full flex items-center justify-center">
              <DollarSign className="w-6 h-6 text-green-600" />
            </div>
          </div>
        </div>

        {/* RevPAR */}
        <div className="bg-white rounded-lg shadow p-6">
          <div className="flex items-center justify-between">
            <div>
              <p className="text-sm text-gray-600">รายได้ต่อห้องว่าง</p>
              <p className="text-3xl font-bold text-gray-800">
                {loading ? '-' : occupancyData ? formatCurrency(occupancyData.revpar) : '-'}
              </p>
              <p className="text-xs text-gray-500 mt-1">RevPAR</p>
            </div>
            <div className="w-12 h-12 bg-purple-100 rounded-full flex items-center justify-center">
              <TrendingUp className="w-6 h-6 text-purple-600" />
            </div>
          </div>
        </div>

        {/* Average Stay Length */}
        <div className="bg-white rounded-lg shadow p-6">
          <div className="flex items-center justify-between">
            <div>
              <p className="text-sm text-gray-600">ระยะเวลาเข้าพักเฉลี่ย</p>
              <p className="text-3xl font-bold text-gray-800">
                {loading ? '-' : occupancyData ? `${occupancyData.avgStayLength.toFixed(1)} คืน` : '-'}
              </p>
              <p className="text-xs text-gray-500 mt-1">Avg Stay Length</p>
            </div>
            <div className="w-12 h-12 bg-orange-100 rounded-full flex items-center justify-center">
              <Clock className="w-6 h-6 text-orange-600" />
            </div>
          </div>
        </div>
      </div>

      {/* Charts Section */}
      <div className="grid grid-cols-1 lg:grid-cols-3 gap-6">
        {/* Revenue Chart - Takes 2 columns */}
        <div className="lg:col-span-2 bg-white rounded-lg shadow p-6">
          <h2 className="text-lg font-semibold text-gray-800 mb-4">รายได้</h2>
          {loading ? (
            <div className="flex items-center justify-center h-80">
              <Loader2 className="w-8 h-8 animate-spin text-blue-600" />
            </div>
          ) : revenueData.length === 0 ? (
            <div className="flex flex-col items-center justify-center h-80 text-gray-500">
              <BarChart3 className="w-12 h-12 text-gray-300 mb-2" />
              <p>ไม่มีข้อมูลในช่วงเวลาที่เลือก</p>
            </div>
          ) : (
            <ResponsiveContainer width="100%" height={320}>
              <BarChart data={revenueData} margin={{ top: 5, right: 30, left: 20, bottom: 5 }}>
                <CartesianGrid strokeDasharray="3 3" stroke="#E5E7EB" />
                <XAxis
                  dataKey="period"
                  tick={{ fontSize: 12 }}
                  stroke="#6B7280"
                />
                <YAxis
                  tick={{ fontSize: 12 }}
                  stroke="#6B7280"
                  tickFormatter={(value) => `${(value / 1000).toFixed(0)}k`}
                />
                <Tooltip content={<RevenueTooltip />} />
                <Bar dataKey="revenue" fill="#3B82F6" radius={[4, 4, 0, 0]} name="รายได้" />
              </BarChart>
            </ResponsiveContainer>
          )}
        </div>

        {/* Room Type Revenue Pie Chart */}
        <div className="bg-white rounded-lg shadow p-6">
          <h2 className="text-lg font-semibold text-gray-800 mb-4">รายได้แยกตามประเภทห้อง</h2>
          {loading ? (
            <div className="flex items-center justify-center h-80">
              <Loader2 className="w-8 h-8 animate-spin text-blue-600" />
            </div>
          ) : roomTypeRevenue.length === 0 ? (
            <div className="flex flex-col items-center justify-center h-80 text-gray-500">
              <BarChart3 className="w-12 h-12 text-gray-300 mb-2" />
              <p>ไม่มีข้อมูล</p>
            </div>
          ) : (
            <ResponsiveContainer width="100%" height={320}>
              <PieChart>
                <Pie
                  data={roomTypeRevenue}
                  cx="50%"
                  cy="45%"
                  innerRadius={60}
                  outerRadius={100}
                  paddingAngle={2}
                  dataKey="revenue"
                  nameKey="roomType"
                  label={({ roomType, percentage }) => `${roomType} (${percentage.toFixed(0)}%)`}
                  labelLine={false}
                >
                  {roomTypeRevenue.map((entry, index) => (
                    <Cell
                      key={`cell-${index}`}
                      fill={CHART_COLORS[index % CHART_COLORS.length]}
                    />
                  ))}
                </Pie>
                <Tooltip content={<PieTooltip />} />
                <Legend
                  verticalAlign="bottom"
                  height={36}
                  formatter={(value: string) => (
                    <span className="text-sm text-gray-700">{value}</span>
                  )}
                />
              </PieChart>
            </ResponsiveContainer>
          )}
        </div>
      </div>

      {/* Booking Trend Line Chart */}
      <div className="bg-white rounded-lg shadow p-6">
        <h2 className="text-lg font-semibold text-gray-800 mb-4">แนวโน้มการจอง</h2>
        {loading ? (
          <div className="flex items-center justify-center h-64">
            <Loader2 className="w-8 h-8 animate-spin text-blue-600" />
          </div>
        ) : revenueData.length === 0 ? (
          <div className="flex flex-col items-center justify-center h-64 text-gray-500">
            <TrendingUp className="w-12 h-12 text-gray-300 mb-2" />
            <p>ไม่มีข้อมูลในช่วงเวลาที่เลือก</p>
          </div>
        ) : (
          <ResponsiveContainer width="100%" height={256}>
            <LineChart data={revenueData} margin={{ top: 5, right: 30, left: 20, bottom: 5 }}>
              <CartesianGrid strokeDasharray="3 3" stroke="#E5E7EB" />
              <XAxis
                dataKey="period"
                tick={{ fontSize: 12 }}
                stroke="#6B7280"
              />
              <YAxis
                tick={{ fontSize: 12 }}
                stroke="#6B7280"
              />
              <Tooltip
                contentStyle={{
                  backgroundColor: 'white',
                  border: '1px solid #E5E7EB',
                  borderRadius: '8px',
                }}
                formatter={(value: number) => [`${value} รายการ`, 'จำนวนการจอง']}
              />
              <Line
                type="monotone"
                dataKey="bookings"
                stroke="#10B981"
                strokeWidth={2}
                dot={{ fill: '#10B981', strokeWidth: 2, r: 4 }}
                activeDot={{ r: 6 }}
                name="จำนวนการจอง"
              />
            </LineChart>
          </ResponsiveContainer>
        )}
      </div>
    </div>
  )
}
