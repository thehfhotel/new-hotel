import { NextResponse } from 'next/server';
import { getPool } from '@/lib/db';

export const dynamic = 'force-dynamic';

export async function GET() {
  try {
    const pool = await getPool();

    // Get room numbers that have checkout today using View_CheckIn_Ds
    // This matches the logic used in stats API for todayCheckOuts
    const result = await pool.request().query(`
      SELECT DISTINCT Cin_Room_no as room_no
      FROM View_CheckIn_Ds
      WHERE CAST(Cin_Room_Out AS DATE) = CAST(GETDATE() AS DATE)
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
