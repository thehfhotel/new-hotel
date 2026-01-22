import { NextRequest, NextResponse } from 'next/server';
import { getPool, sql } from '@/lib/db';

export const dynamic = 'force-dynamic';

export async function GET(request: NextRequest) {
  try {
    const searchParams = request.nextUrl.searchParams;
    const status = searchParams.get('status');
    const startDate = searchParams.get('startDate');
    const endDate = searchParams.get('endDate');
    const page = parseInt(searchParams.get('page') || '1', 10);
    const limit = parseInt(searchParams.get('limit') || '20', 10);
    const offset = (page - 1) * limit;

    const pool = await getPool();
    const dbRequest = pool.request();

    let baseQuery = `
      FROM View_Booking_Ds
    `;

    const conditions: string[] = [];

    // Status filtering removed - Book_Status may be an integer code

    if (startDate) {
      dbRequest.input('startDate', sql.Date, new Date(startDate));
      conditions.push('Book_Date_in >= @startDate');
    }

    if (endDate) {
      dbRequest.input('endDate', sql.Date, new Date(endDate));
      conditions.push('Book_Date_out <= @endDate');
    }

    if (conditions.length > 0) {
      baseQuery += ' WHERE ' + conditions.join(' AND ');
    }

    // Count query
    const countResult = await dbRequest.query(`SELECT COUNT(*) as total ${baseQuery}`);
    const total = countResult.recordset[0].total;

    // Data query with pagination
    const dataRequest = pool.request();
    if (startDate) dataRequest.input('startDate', sql.Date, new Date(startDate));
    if (endDate) dataRequest.input('endDate', sql.Date, new Date(endDate));
    dataRequest.input('offset', sql.Int, offset);
    dataRequest.input('limit', sql.Int, limit);

    const dataQuery = `
      SELECT
        Book_No,
        Book_Date,
        Book_Date_in,
        Book_Date_out,
        Book_Cust_Name,
        Book_Status,
        Book_Room_Type
      ${baseQuery}
      ORDER BY Book_Date DESC
      OFFSET @offset ROWS FETCH NEXT @limit ROWS ONLY
    `;

    const result = await dataRequest.query(dataQuery);

    return NextResponse.json({
      success: true,
      data: result.recordset,
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
