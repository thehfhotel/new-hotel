import { NextResponse } from 'next/server';
import { getPool } from '@/lib/db';

export const dynamic = 'force-dynamic';

export async function GET() {
  try {
    const pool = await getPool();

    // Get room numbers that have checkout today using View_CheckIn_Ds
    // Only include rooms where guest is still checked in (Room_Use = 'yes')
    // This excludes old records from guests who already checked out today
    const result = await pool.request().query(`
      SELECT DISTINCT c.Cin_Room_no as room_no
      FROM View_CheckIn_Ds c
      INNER JOIN HT_Rooms r ON c.Cin_Room_No = r.Room_no
      WHERE CAST(c.Cin_Room_Out AS DATE) = CAST(GETDATE() AS DATE)
        AND r.Room_Use = 'yes'
    `);

    return NextResponse.json({
      success: true,
      data: result.recordset.map((r: { room_no: string }) => r.room_no),
    });
  } catch (error) {
    console.error('Error fetching checkout rooms:', error);
    return NextResponse.json(
      {
        success: false,
        error: 'Failed to fetch checkout rooms',
      },
      { status: 500 }
    );
  }
}
