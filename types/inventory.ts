// Inventory Management Types

export interface InventoryItem {
  id: number
  itemCode: string
  itemName: string
  category: InventoryCategory
  unit: string
  minStock: number
  currentStock: number
  costPerUnit: number | null
  active: boolean
  createdAt: string
  updatedAt: string
}

export type InventoryCategory = 'Minibar' | 'Amenities' | 'Linens' | 'Equipment'

export const INVENTORY_CATEGORIES: { value: InventoryCategory; label: string; labelTh: string }[] = [
  { value: 'Minibar', label: 'Minibar', labelTh: 'เครื่องดื่ม/ของว่าง' },
  { value: 'Amenities', label: 'Amenities', labelTh: 'อุปกรณ์อำนวยความสะดวก' },
  { value: 'Linens', label: 'Linens', labelTh: 'ผ้าและเครื่องนอน' },
  { value: 'Equipment', label: 'Equipment', labelTh: 'อุปกรณ์ในห้อง' },
]

export type TransactionType = 'IN' | 'OUT' | 'ADJUST' | 'MOVE'

export const TRANSACTION_TYPES: { value: TransactionType; label: string; labelTh: string }[] = [
  { value: 'IN', label: 'Stock In', labelTh: 'รับเข้า' },
  { value: 'OUT', label: 'Stock Out', labelTh: 'เบิกออก' },
  { value: 'ADJUST', label: 'Adjustment', labelTh: 'ปรับสต็อก' },
  { value: 'MOVE', label: 'Transfer', labelTh: 'โอนย้าย' },
]

export interface InventoryTransaction {
  id: number
  itemId: number
  itemCode: string
  itemName: string
  transactionType: TransactionType
  quantity: number
  previousStock: number
  newStock: number
  roomId: number | null
  roomNumber: string | null
  notes: string | null
  createdBy: string | null
  createdAt: string
}

export interface RoomInventory {
  roomId: number
  roomNumber: string
  roomType: string
  items: RoomInventoryItem[]
}

export interface RoomInventoryItem {
  itemId: number
  itemCode: string
  itemName: string
  category: InventoryCategory
  unit: string
  assignedQuantity: number
  verifiedQuantity: number | null
  lastVerified: string | null
}

export interface InventoryItemFormData {
  id?: number
  itemCode: string
  itemName: string
  category: InventoryCategory
  unit: string
  minStock: number
  currentStock: number
  costPerUnit: number | null
  active: boolean
}

export interface StockAdjustment {
  itemId: number
  adjustmentType: 'add' | 'remove' | 'set'
  quantity: number
  notes: string
}

export interface RoomInventoryCheck {
  roomId: number
  items: {
    itemId: number
    verified: boolean
    actualQuantity: number
  }[]
  notes: string
}

// Stock level status helpers
export type StockStatus = 'good' | 'low' | 'critical' | 'out'

export function getStockStatus(currentStock: number, minStock: number): StockStatus {
  if (currentStock <= 0) return 'out'
  if (currentStock <= minStock * 0.5) return 'critical'
  if (currentStock <= minStock) return 'low'
  return 'good'
}

export function getStockStatusColor(status: StockStatus): string {
  switch (status) {
    case 'good':
      return 'text-green-600 bg-green-100'
    case 'low':
      return 'text-yellow-600 bg-yellow-100'
    case 'critical':
      return 'text-orange-600 bg-orange-100'
    case 'out':
      return 'text-red-600 bg-red-100'
  }
}

export function getStockStatusLabel(status: StockStatus): string {
  switch (status) {
    case 'good':
      return 'ปกติ'
    case 'low':
      return 'ใกล้หมด'
    case 'critical':
      return 'วิกฤต'
    case 'out':
      return 'หมด'
  }
}
