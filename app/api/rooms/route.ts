import { NextResponse } from 'next/server';
import { getPool } from '@/lib/db';

export const dynamic = 'force-dynamic';

export async function GET() {
  try {
    const pool = await getPool();
    const result = await pool.request().query(`
      SELECT
        Room_no,
        Room_Type,
        Room_Details,
        Room_Clean,
        Room_Use,
        Room_Book,
        Room_Manternace
      FROM HT_Rooms
      ORDER BY Room_no
    `);

    return NextResponse.json({
      success: true,
      data: result.recordset,
      total: result.recordset.length,
    });
  } catch (error) {
    console.error('Error fetching rooms:', error);
    return NextResponse.json(
      {
        success: false,
        error: 'Failed to fetch rooms',
      },
      { status: 500 }
    );
  }
}
