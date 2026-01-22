import { NextRequest, NextResponse } from 'next/server';
import { getPool, sql } from '@/lib/db';

export const dynamic = 'force-dynamic';

export async function GET(request: NextRequest) {
  try {
    const searchParams = request.nextUrl.searchParams;
    const startDate = searchParams.get('startDate');
    const endDate = searchParams.get('endDate');

    const pool = await getPool();
    const dbRequest = pool.request();

    let query = `
      SELECT
        room_no,
        room_date,
        room_status,
        room_Details,
        room_CheckIn_No,
        Room_Type
      FROM View_Room_status
    `;

    const conditions: string[] = [];

    if (startDate) {
      dbRequest.input('startDate', sql.Date, new Date(startDate));
      conditions.push('room_date >= @startDate');
    }

    if (endDate) {
      dbRequest.input('endDate', sql.Date, new Date(endDate));
      conditions.push('room_date <= @endDate');
    }

    if (conditions.length > 0) {
      query += ' WHERE ' + conditions.join(' AND ');
    }

    query += ' ORDER BY room_no, room_date';

    const result = await dbRequest.query(query);

    return NextResponse.json({
      success: true,
      data: result.recordset,
      total: result.recordset.length,
    });
  } catch (error) {
    console.error('Error fetching room status:', error);
    return NextResponse.json(
      {
        success: false,
        error: 'Failed to fetch room status',
      },
      { status: 500 }
    );
  }
}
