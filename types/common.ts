export interface PaginationInfo {
  page: number
  limit: number
  total: number
  totalPages: number
}

export interface ApiResponse<T> {
  success: boolean
  data: T
  message?: string
}

export interface ApiListResponse<T> {
  success: boolean
  data: T[]
  pagination: PaginationInfo
}
