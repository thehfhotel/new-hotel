import { NextRequest, NextResponse } from 'next/server';
import { getPool, sql } from '@/lib/db';

export const dynamic = 'force-dynamic';

export async function GET(
  request: NextRequest,
  { params }: { params: Promise<{ id: string }> }
) {
  try {
    // Customer ID comes as "C0001" format - use as-is for Book_Cust_ID match
    const { id: custId } = await params;

    const pool = await getPool();
    const dbRequest = pool.request();
    dbRequest.input('custId', sql.NVarChar, custId);

    const result = await dbRequest.query(`
      SELECT
        Book_No,
        Book_Room_No,
        Book_Room_Type,
        Book_Date_in,
        Book_Date_out,
        Book_Status,
        Book_Total
      FROM View_Booking_Ds
      WHERE Book_Cust_ID = @custId
      ORDER BY Book_Date_in DESC
    `);

    const bookings = result.recordset.map(row => ({
      id: row.Book_No,
      roomNumber: row.Book_Room_No || '-',
      roomType: row.Book_Room_Type || '-',
      checkInDate: row.Book_Date_in,
      checkOutDate: row.Book_Date_out,
      status: row.Book_Status === 1 ? 'confirmed' : row.Book_Status === 2 ? 'completed' : 'pending',
      totalAmount: row.Book_Total || 0,
    }));

    return NextResponse.json({
      success: true,
      bookings,
    });
  } catch (error) {
    console.error('Error fetching customer bookings:', error);
    return NextResponse.json(
      { success: false, error: 'Failed to fetch customer bookings' },
      { status: 500 }
    );
  }
}
