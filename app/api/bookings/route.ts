import { NextRequest, NextResponse } from 'next/server';
import { getPool, sql } from '@/lib/db';

export const dynamic = 'force-dynamic';

interface RoomRecord {
  Book_No: string;
  Book_Date: Date;
  Book_Date_in: Date;
  Book_Date_out: Date;
  Book_Cust_ID: string;
  Book_Cust_Name: string;
  Book_Status: string | number;
  Book_Room_No: string;
  Book_Room_Type: string;
  Book_Total: number;
}

interface Room {
  roomNo: string;
  roomType: string;
  total: number;
}

interface GroupedBooking {
  bookNo: string;
  bookDate: Date;
  checkIn: Date;
  checkOut: Date;
  customer: {
    id: string;
    name: string;
  };
  status: string;
  rooms: Room[];
  totalAmount: number;
  roomCount: number;
}

// Map status codes to Thai text
function mapStatus(status: string | number): string {
  if (typeof status === 'number') {
    switch (status) {
      case 1: return 'จอง';
      case 2: return 'เข้าพัก';
      case 3: return 'เสร็จสิ้น';
      case 4: return 'ยกเลิก';
      default: return 'ไม่ทราบ';
    }
  }
  return String(status || 'ไม่ทราบ');
}

// Group room records by Book_No
function groupByBookNo(records: RoomRecord[]): GroupedBooking[] {
  const grouped = new Map<string, GroupedBooking>();

  for (const record of records) {
    const bookNo = record.Book_No;

    if (!grouped.has(bookNo)) {
      grouped.set(bookNo, {
        bookNo,
        bookDate: record.Book_Date,
        checkIn: record.Book_Date_in,
        checkOut: record.Book_Date_out,
        customer: {
          id: record.Book_Cust_ID || '',
          name: record.Book_Cust_Name || '',
        },
        status: mapStatus(record.Book_Status),
        rooms: [],
        totalAmount: 0,
        roomCount: 0,
      });
    }

    const booking = grouped.get(bookNo)!;
    booking.rooms.push({
      roomNo: record.Book_Room_No || '-',
      roomType: record.Book_Room_Type || '-',
      total: record.Book_Total || 0,
    });
    booking.totalAmount += record.Book_Total || 0;
    booking.roomCount = booking.rooms.length;
  }

  return Array.from(grouped.values());
}

export async function GET(request: NextRequest) {
  try {
    const searchParams = request.nextUrl.searchParams;
    const status = searchParams.get('status');
    const startDate = searchParams.get('startDate');
    const endDate = searchParams.get('endDate');
    const search = searchParams.get('search');
    const roomType = searchParams.get('roomType');
    const sortBy = searchParams.get('sortBy') || 'Book_Date';
    const sortOrder = searchParams.get('sortOrder') || 'DESC';
    const page = parseInt(searchParams.get('page') || '1', 10);
    const limit = parseInt(searchParams.get('limit') || '20', 10);

    const pool = await getPool();

    // Build conditions
    const conditions: string[] = [];
    const params: { name: string; type: typeof sql.NVarChar | typeof sql.Date | typeof sql.Int; value: unknown }[] = [];

    if (search) {
      conditions.push('(Book_No LIKE @search OR Book_Cust_Name LIKE @search)');
      params.push({ name: 'search', type: sql.NVarChar, value: `%${search}%` });
    }

    if (status) {
      conditions.push('Book_Status = @status');
      params.push({ name: 'status', type: sql.NVarChar, value: status });
    }

    if (startDate) {
      conditions.push('Book_Date_in >= @startDate');
      params.push({ name: 'startDate', type: sql.Date, value: new Date(startDate) });
    }

    if (endDate) {
      conditions.push('Book_Date_out <= @endDate');
      params.push({ name: 'endDate', type: sql.Date, value: new Date(endDate) });
    }

    if (roomType) {
      conditions.push('Book_Room_Type = @roomType');
      params.push({ name: 'roomType', type: sql.NVarChar, value: roomType });
    }

    const whereClause = conditions.length > 0 ? `WHERE ${conditions.join(' AND ')}` : '';

    // Validate sort column to prevent SQL injection
    const validSortColumns = ['Book_No', 'Book_Date', 'Book_Date_in', 'Book_Date_out', 'Book_Cust_Name', 'Book_Status'];
    const safeSortBy = validSortColumns.includes(sortBy) ? sortBy : 'Book_Date';
    const safeSortOrder = sortOrder.toUpperCase() === 'ASC' ? 'ASC' : 'DESC';

    // First, get distinct Book_No count for pagination
    const countRequest = pool.request();
    params.forEach(p => countRequest.input(p.name, p.type, p.value));

    const countResult = await countRequest.query(`
      SELECT COUNT(DISTINCT Book_No) as total
      FROM View_Booking_Ds
      ${whereClause}
    `);
    const total = countResult.recordset[0].total;

    // Get paginated distinct Book_No values
    const bookNoRequest = pool.request();
    params.forEach(p => bookNoRequest.input(p.name, p.type, p.value));
    bookNoRequest.input('offset', sql.Int, (page - 1) * limit);
    bookNoRequest.input('limit', sql.Int, limit);

    const bookNoResult = await bookNoRequest.query(`
      SELECT DISTINCT Book_No, MIN(${safeSortBy}) as sort_value
      FROM View_Booking_Ds
      ${whereClause}
      GROUP BY Book_No
      ORDER BY sort_value ${safeSortOrder}
      OFFSET @offset ROWS FETCH NEXT @limit ROWS ONLY
    `);

    const bookNos = bookNoResult.recordset.map(r => r.Book_No);

    if (bookNos.length === 0) {
      return NextResponse.json({
        success: true,
        data: [],
        pagination: {
          page,
          limit,
          total,
          totalPages: Math.ceil(total / limit),
        },
      });
    }

    // Fetch all room records for these bookings
    const dataRequest = pool.request();
    const bookNoParams = bookNos.map((bn, i) => `@bn${i}`).join(',');
    bookNos.forEach((bn, i) => dataRequest.input(`bn${i}`, sql.NVarChar, bn));

    const dataResult = await dataRequest.query(`
      SELECT
        Book_No,
        Book_Date,
        Book_Date_in,
        Book_Date_out,
        Book_Cust_ID,
        Book_Cust_Name,
        Book_Status,
        Book_Room_No,
        Book_Room_Type,
        Book_Total
      FROM View_Booking_Ds
      WHERE Book_No IN (${bookNoParams})
      ORDER BY ${safeSortBy} ${safeSortOrder}, Book_Room_No
    `);

    // Group records by Book_No
    const groupedBookings = groupByBookNo(dataResult.recordset);

    // Sort grouped results to match pagination order
    const bookNoOrder = new Map(bookNos.map((bn, i) => [bn, i]));
    groupedBookings.sort((a, b) => (bookNoOrder.get(a.bookNo) || 0) - (bookNoOrder.get(b.bookNo) || 0));

    return NextResponse.json({
      success: true,
      data: groupedBookings,
      pagination: {
        page,
        limit,
        total,
        totalPages: Math.ceil(total / limit),
      },
    });
  } catch (error) {
    console.error('Error fetching bookings:', error);
    return NextResponse.json(
      {
        success: false,
        error: 'Failed to fetch bookings',
      },
      { status: 500 }
    );
  }
}
