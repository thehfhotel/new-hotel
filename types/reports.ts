/**
 * Report types for hotel analytics and reporting
 */

// Revenue report types
export interface RevenueDataPoint {
  period: string;
  revenue: number;
  bookings: number;
}

export interface RevenueResponse {
  success: boolean;
  data: RevenueDataPoint[];
}

// Occupancy report types
export interface OccupancyResponse {
  success: boolean;
  occupancyRate: number;
  totalRooms: number;
  occupiedNights: number;
  availableNights: number;
  adr: number; // Average Daily Rate
  revpar: number; // Revenue Per Available Room
  avgStayLength: number;
}

// Revenue by room type
export interface RoomTypeRevenue {
  roomType: string;
  revenue: number;
  percentage: number;
}

export interface RevenueByRoomTypeResponse {
  success: boolean;
  data: RoomTypeRevenue[];
}

// Group by options for revenue report
export type GroupBy = 'day' | 'week' | 'month';

// Maintenance types
export type MaintenanceStatus = 'open' | 'in_progress' | 'completed' | 'cancelled';
export type MaintenancePriority = 1 | 2 | 3; // 1=low, 2=medium, 3=high

export interface MaintenanceCategory {
  id: number;
  name: string;
  nameEn: string | null;
  priority: number;
  active: boolean;
}

export interface MaintenanceRequest {
  id: number;
  requestNo: string;
  roomId: number;
  roomNo: string;
  categoryId: number;
  categoryName: string;
  title: string;
  description: string | null;
  priority: MaintenancePriority;
  status: MaintenanceStatus;
  assignedTo: string | null;
  startedAt: string | null;
  completedAt: string | null;
  resolution: string | null;
  cost: number | null;
  createdAt: string;
  updatedAt: string;
}

// Payment types
export type PaymentMethod = 'cash' | 'credit' | 'transfer' | 'qr';

export interface Payment {
  id: number;
  checkInId: number;
  amount: number;
  method: PaymentMethod;
  reference: string | null;
  notes: string | null;
  paymentDate: string;
  createdBy: string | null;
}

export interface PaymentsResponse {
  success: boolean;
  data: Payment[];
  totalPaid: number;
  balance: number;
}
