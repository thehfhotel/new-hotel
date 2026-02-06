'use client'

import {
  LineChart,
  Line,
  BarChart,
  Bar,
  XAxis,
  YAxis,
  CartesianGrid,
  Tooltip,
  Legend,
  ResponsiveContainer,
  PieChart as RechartsPieChart,
  Pie,
  Cell,
} from 'recharts'

// Helper function for Thai Baht currency formatting
function formatCurrency(amount: number): string {
  return new Intl.NumberFormat('th-TH', {
    style: 'currency',
    currency: 'THB',
    minimumFractionDigits: 0,
  }).format(amount)
}

// Color palette for pie chart
const COLORS = ['#3b82f6', '#10b981', '#f59e0b', '#ef4444', '#8b5cf6', '#ec4899']

export interface OccupancyData {
  date: string
  occupiedRooms: number
}

interface OccupancyChartProps {
  data: OccupancyData[]
  chartType?: 'line' | 'bar'
  title?: string
}

function CustomTooltip({ active, payload, label }: {
  active?: boolean
  payload?: { value: number }[]
  label?: string
}) {
  if (active && payload && payload.length) {
    return (
      <div className="bg-white border border-gray-200 rounded-lg shadow-lg p-3">
        <p className="text-gray-600 text-sm">{label}</p>
        <p className="text-blue-600 font-bold">
          ห้องที่มีผู้เข้าพัก: {payload[0].value} ห้อง
        </p>
      </div>
    )
  }
  return null
}

export function OccupancyChart({
  data,
  chartType = 'bar',
  title = 'จำนวนห้องที่มีผู้เข้าพัก',
}: OccupancyChartProps) {
  const maxRooms = Math.max(...data.map(d => d.occupiedRooms), 10)

  return (
    <div className="bg-white rounded-xl shadow-sm border border-gray-100 p-6">
      {title && (
        <h3 className="text-lg font-semibold text-gray-800 mb-4">{title}</h3>
      )}
      <div className="h-[300px] w-full">
        <ResponsiveContainer width="100%" height="100%">
          {chartType === 'line' ? (
            <LineChart
              data={data}
              margin={{ top: 5, right: 30, left: 20, bottom: 5 }}
            >
              <CartesianGrid strokeDasharray="3 3" stroke="#e5e7eb" />
              <XAxis
                dataKey="date"
                tick={{ fill: '#6b7280', fontSize: 12 }}
                tickLine={{ stroke: '#e5e7eb' }}
              />
              <YAxis
                tick={{ fill: '#6b7280', fontSize: 12 }}
                tickLine={{ stroke: '#e5e7eb' }}
                domain={[0, Math.ceil(maxRooms * 1.2)]}
                tickFormatter={(value) => `${value}`}
              />
              <Tooltip content={<CustomTooltip />} />
              <Legend
                wrapperStyle={{ paddingTop: '20px' }}
                formatter={() => <span className="text-gray-600">จำนวนห้อง</span>}
              />
              <Line
                type="monotone"
                dataKey="occupiedRooms"
                stroke="#3b82f6"
                strokeWidth={3}
                dot={{ fill: '#3b82f6', strokeWidth: 2, r: 4 }}
                activeDot={{ r: 6, fill: '#1d4ed8' }}
              />
            </LineChart>
          ) : (
            <BarChart
              data={data}
              margin={{ top: 5, right: 30, left: 20, bottom: 5 }}
            >
              <CartesianGrid strokeDasharray="3 3" stroke="#e5e7eb" />
              <XAxis
                dataKey="date"
                tick={{ fill: '#6b7280', fontSize: 12 }}
                tickLine={{ stroke: '#e5e7eb' }}
              />
              <YAxis
                tick={{ fill: '#6b7280', fontSize: 12 }}
                tickLine={{ stroke: '#e5e7eb' }}
                domain={[0, Math.ceil(maxRooms * 1.2)]}
                tickFormatter={(value) => `${value}`}
              />
              <Tooltip content={<CustomTooltip />} />
              <Legend
                wrapperStyle={{ paddingTop: '20px' }}
                formatter={() => <span className="text-gray-600">จำนวนห้อง</span>}
              />
              <Bar
                dataKey="occupiedRooms"
                fill="#3b82f6"
                radius={[4, 4, 0, 0]}
              />
            </BarChart>
          )}
        </ResponsiveContainer>
      </div>
    </div>
  )
}

// Revenue Chart Types
export interface RevenueData {
  period: string
  revenue: number
  bookings: number
}

interface RevenueChartProps {
  data: RevenueData[]
  chartType?: 'line' | 'bar'
  title?: string
}

function RevenueTooltip({ active, payload, label }: {
  active?: boolean
  payload?: { value: number; dataKey: string }[]
  label?: string
}) {
  if (active && payload && payload.length) {
    const revenuePayload = payload.find(p => p.dataKey === 'revenue')
    const bookingsPayload = payload.find(p => p.dataKey === 'bookings')
    return (
      <div className="bg-white border border-gray-200 rounded-lg shadow-lg p-3">
        <p className="text-gray-600 text-sm font-medium">{label}</p>
        {revenuePayload && (
          <p className="text-green-600 font-bold">
            รายได้: {formatCurrency(revenuePayload.value)}
          </p>
        )}
        {bookingsPayload && (
          <p className="text-blue-600 font-semibold">
            จำนวนการจอง: {bookingsPayload.value} รายการ
          </p>
        )}
      </div>
    )
  }
  return null
}

export function RevenueChart({
  data,
  chartType = 'bar',
  title = 'รายได้',
}: RevenueChartProps) {
  const maxRevenue = Math.max(...data.map(d => d.revenue), 0)

  return (
    <div className="bg-white rounded-xl shadow-sm border border-gray-100 p-6">
      {title && (
        <h3 className="text-lg font-semibold text-gray-800 mb-4">{title}</h3>
      )}
      <div className="h-[300px] w-full">
        <ResponsiveContainer width="100%" height="100%">
          {chartType === 'line' ? (
            <LineChart
              data={data}
              margin={{ top: 5, right: 30, left: 20, bottom: 5 }}
            >
              <CartesianGrid strokeDasharray="3 3" stroke="#e5e7eb" />
              <XAxis
                dataKey="period"
                tick={{ fill: '#6b7280', fontSize: 12 }}
                tickLine={{ stroke: '#e5e7eb' }}
              />
              <YAxis
                tick={{ fill: '#6b7280', fontSize: 12 }}
                tickLine={{ stroke: '#e5e7eb' }}
                domain={[0, Math.ceil(maxRevenue * 1.2)]}
                tickFormatter={(value) => formatCurrency(value)}
              />
              <Tooltip content={<RevenueTooltip />} />
              <Legend
                wrapperStyle={{ paddingTop: '20px' }}
                formatter={(value) => (
                  <span className="text-gray-600">
                    {value === 'revenue' ? 'รายได้' : 'จำนวนการจอง'}
                  </span>
                )}
              />
              <Line
                type="monotone"
                dataKey="revenue"
                stroke="#10b981"
                strokeWidth={3}
                dot={{ fill: '#10b981', strokeWidth: 2, r: 4 }}
                activeDot={{ r: 6, fill: '#059669' }}
              />
              <Line
                type="monotone"
                dataKey="bookings"
                stroke="#3b82f6"
                strokeWidth={2}
                dot={{ fill: '#3b82f6', strokeWidth: 2, r: 3 }}
                activeDot={{ r: 5, fill: '#1d4ed8' }}
                yAxisId="right"
              />
            </LineChart>
          ) : (
            <BarChart
              data={data}
              margin={{ top: 5, right: 30, left: 20, bottom: 5 }}
            >
              <CartesianGrid strokeDasharray="3 3" stroke="#e5e7eb" />
              <XAxis
                dataKey="period"
                tick={{ fill: '#6b7280', fontSize: 12 }}
                tickLine={{ stroke: '#e5e7eb' }}
              />
              <YAxis
                tick={{ fill: '#6b7280', fontSize: 12 }}
                tickLine={{ stroke: '#e5e7eb' }}
                domain={[0, Math.ceil(maxRevenue * 1.2)]}
                tickFormatter={(value) => formatCurrency(value)}
              />
              <Tooltip content={<RevenueTooltip />} />
              <Legend
                wrapperStyle={{ paddingTop: '20px' }}
                formatter={(value) => (
                  <span className="text-gray-600">
                    {value === 'revenue' ? 'รายได้' : 'จำนวนการจอง'}
                  </span>
                )}
              />
              <Bar
                dataKey="revenue"
                fill="#10b981"
                radius={[4, 4, 0, 0]}
              />
            </BarChart>
          )}
        </ResponsiveContainer>
      </div>
    </div>
  )
}

// Pie Chart Types
export interface PieChartData {
  roomType: string
  revenue: number
  percentage: number
}

interface PieChartProps {
  data: PieChartData[]
  title?: string
}

function PieTooltip({ active, payload }: {
  active?: boolean
  payload?: { payload: PieChartData }[]
}) {
  if (active && payload && payload.length) {
    const data = payload[0].payload
    return (
      <div className="bg-white border border-gray-200 rounded-lg shadow-lg p-3">
        <p className="text-gray-600 text-sm font-medium">{data.roomType}</p>
        <p className="text-green-600 font-bold">
          รายได้: {formatCurrency(data.revenue)}
        </p>
        <p className="text-blue-600 font-semibold">
          สัดส่วน: {data.percentage.toFixed(1)}%
        </p>
      </div>
    )
  }
  return null
}

export function PieChart({
  data,
  title = 'รายได้ตามประเภทห้อง',
}: PieChartProps) {
  return (
    <div className="bg-white rounded-xl shadow-sm border border-gray-100 p-6">
      {title && (
        <h3 className="text-lg font-semibold text-gray-800 mb-4">{title}</h3>
      )}
      <div className="h-[300px] w-full">
        <ResponsiveContainer width="100%" height="100%">
          <RechartsPieChart>
            <Pie
              data={data}
              cx="50%"
              cy="50%"
              labelLine={false}
              label={({ roomType, percentage }) => `${roomType} (${percentage.toFixed(0)}%)`}
              outerRadius={100}
              fill="#8884d8"
              dataKey="revenue"
              nameKey="roomType"
            >
              {data.map((_, index) => (
                <Cell key={`cell-${index}`} fill={COLORS[index % COLORS.length]} />
              ))}
            </Pie>
            <Tooltip content={<PieTooltip />} />
            <Legend
              wrapperStyle={{ paddingTop: '20px' }}
              formatter={(value) => <span className="text-gray-600">{value}</span>}
            />
          </RechartsPieChart>
        </ResponsiveContainer>
      </div>
    </div>
  )
}
