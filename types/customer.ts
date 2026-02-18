import { PaginationInfo } from './common'

export interface Customer {
  id: number
  firstName: string
  lastName: string
  phone: string | null
  email: string | null
  idCard: string | null
  address: string | null
  createdAt: string | null
}

export interface CustomerOption {
  id: number
  firstName: string
  lastName: string
  phone: string
  email: string
  idCard: string
}

export interface CustomersResponse {
  success: boolean
  data: Customer[]
  pagination: PaginationInfo
}
