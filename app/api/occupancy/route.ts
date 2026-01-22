import { NextRequest, NextResponse } from 'next/server';
import { getPool, sql } from '@/lib/db';

export const dynamic = 'force-dynamic';

export async function GET(request: NextRequest) {
  try {
    const searchParams = request.nextUrl.searchParams;
    const days = parseInt(searchParams.get('days') || '7', 10);

    const pool = await getPool();

    // Get occupancy for the last N days
    // Count rooms where check-in date <= day AND check-out date > day
    const query = `
      WITH DateRange AS (
        SELECT CAST(DATEADD(day, -n, CAST(GETDATE() AS DATE)) AS DATE) as date_val
        FROM (
          SELECT 0 as n UNION ALL SELECT 1 UNION ALL SELECT 2 UNION ALL SELECT 3
          UNION ALL SELECT 4 UNION ALL SELECT 5 UNION ALL SELECT 6
          UNION ALL SELECT 7 UNION ALL SELECT 8 UNION ALL SELECT 9
          UNION ALL SELECT 10 UNION ALL SELECT 11 UNION ALL SELECT 12 UNION ALL SELECT 13
        ) numbers
        WHERE n < @days
      )
      SELECT
        dr.date_val as date,
        COUNT(DISTINCT c.Cin_Room_No) as occupiedRooms
      FROM DateRange dr
      LEFT JOIN View_CheckIn_Ds c ON
        CAST(c.Cin_Room_In AS DATE) <= dr.date_val
        AND CAST(c.Cin_Room_Out AS DATE) > dr.date_val
      GROUP BY dr.date_val
      ORDER BY dr.date_val ASC
    `;

    const result = await pool.request()
      .input('days', sql.Int, days)
      .query(query);

    return NextResponse.json({
      success: true,
      data: result.recordset,
    });
  } catch (error) {
    console.error('Error fetching occupancy data:', error);
    return NextResponse.json(
      {
        success: false,
        error: 'Failed to fetch occupancy data',
      },
      { status: 500 }
    );
  }
}
