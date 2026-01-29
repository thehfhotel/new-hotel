import { NextRequest, NextResponse } from 'next/server';
import { getPool, sql } from '@/lib/db';

export const dynamic = 'force-dynamic';

interface RoomRecord {
  Book_No: string;
  Book_Date: Date;
  Book_Date_in: Date;
  Book_Date_out: Date;
  Book_Cust_Name: string;
  Book_Status: string | number;
  Book_Room_Type: string;
}

interface Room {
  roomNo: string;
  roomType: string;
}

interface GroupedBooking {
  bookNo: string;
  bookDate: Date;
  checkIn: Date;
  checkOut: Date;
  customer: {
    name: string;
  };
  status: string;
  rooms: Room[];
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
          name: record.Book_Cust_Name || '',
        },
        status: mapStatus(record.Book_Status),
        rooms: [],
        roomCount: 0,
      });
    }

    const booking = grouped.get(bookNo)!;
    booking.rooms.push({
      roomNo: '-',
      roomType: record.Book_Room_Type || '-',
    });
    booking.roomCount = booking.rooms.length;
  }

  return Array.from(grouped.values());
}

export async function GET(request: NextRequest) {
  try {
    const searchParams = request.nextUrl.searchParams;
    const search = searchParams.get('search');
    const startDate = searchParams.get('startDate');
    const endDate = searchParams.get('endDate');
    const page = parseInt(searchParams.get('page') || '1', 10);
    const limit = parseInt(searchParams.get('limit') || '20', 10);

    const pool = await getPool();

    // Build conditions
    const conditions: string[] = [];
    const params: { name: string; type: typeof sql.NVarChar | typeof sql.Date; value: unknown }[] = [];

    if (search) {
      conditions.push('(Book_No LIKE @search OR Book_Cust_Name LIKE @search)');
      params.push({ name: 'search', type: sql.NVarChar, value: `%${search}%` });
    }

    if (startDate) {
      conditions.push('Book_Date_in >= @startDate');
      params.push({ name: 'startDate', type: sql.Date, value: new Date(startDate) });
    }

    if (endDate) {
      conditions.push('Book_Date_out <= @endDate');
      params.push({ name: 'endDate', type: sql.Date, value: new Date(endDate) });
    }

    const whereClause = conditions.length > 0 ? `WHERE ${conditions.join(' AND ')}` : '';

    // Count distinct bookings
    const countRequest = pool.request();
    params.forEach(p => countRequest.input(p.name, p.type, p.value));

    const countResult = await countRequest.query(`
      SELECT COUNT(DISTINCT Book_No) as total
      FROM View_Booking_Ds
      ${whereClause}
    `);
    const total = countResult.recordset[0].total;

    // Get all data (we'll group and paginate in JS for simplicity)
    const dataRequest = pool.request();
    params.forEach(p => dataRequest.input(p.name, p.type, p.value));

    const dataResult = await dataRequest.query(`
      SELECT
        Book_No,
        Book_Date,
        Book_Date_in,
        Book_Date_out,
        Book_Cust_Name,
        Book_Status,
        Book_Room_Type
      FROM View_Booking_Ds
      ${whereClause}
      ORDER BY Book_Date DESC
    `);

    // Group records by Book_No
    const allGrouped = groupByBookNo(dataResult.recordset);

    // Sort by bookDate descending
    allGrouped.sort((a, b) => new Date(b.bookDate).getTime() - new Date(a.bookDate).getTime());

    // Paginate
    const startIdx = (page - 1) * limit;
    const paginatedData = allGrouped.slice(startIdx, startIdx + limit);

    return NextResponse.json({
      success: true,
      data: paginatedData,
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
        details: error instanceof Error ? error.message : 'Unknown error',
      },
      { status: 500 }
    );
  }
}
