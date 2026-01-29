import { NextRequest, NextResponse } from 'next/server';
import { getPool, sql } from '@/lib/db';

export const dynamic = 'force-dynamic';

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

export async function GET(
  request: NextRequest,
  { params }: { params: Promise<{ id: string }> }
) {
  try {
    const { id: bookNo } = await params;
    const pool = await getPool();

    // Fetch booking with all rooms
    const bookingRequest = pool.request();
    bookingRequest.input('bookNo', sql.NVarChar, bookNo);

    const bookingResult = await bookingRequest.query(`
      SELECT
        Book_No,
        Book_Date,
        Book_Date_in,
        Book_Date_out,
        Book_Cust_Name,
        Book_Status,
        Book_Room_Type
      FROM View_Booking_Ds
      WHERE Book_No = @bookNo
      ORDER BY Book_Room_Type
    `);

    if (bookingResult.recordset.length === 0) {
      return NextResponse.json(
        { success: false, error: 'Booking not found' },
        { status: 404 }
      );
    }

    const records = bookingResult.recordset;
    const firstRecord = records[0];

    // Customer info from booking (no separate lookup since Book_Cust_ID not available)
    const customerDetails = {
      fullName: firstRecord.Book_Cust_Name || '',
    };

    // Try to fetch notes (table may not exist yet)
    let notes: { id: number; text: string; createdAt: Date; updatedAt: Date }[] = [];
    try {
      const notesRequest = pool.request();
      notesRequest.input('bookNo', sql.NVarChar, bookNo);

      const notesResult = await notesRequest.query(`
        SELECT Note_ID, Note_Text, Created_At, Updated_At
        FROM HT_Booking_Notes
        WHERE Book_No = @bookNo
        ORDER BY Created_At DESC
      `);

      notes = notesResult.recordset.map(n => ({
        id: n.Note_ID,
        text: n.Note_Text,
        createdAt: n.Created_At,
        updatedAt: n.Updated_At,
      }));
    } catch {
      // Table doesn't exist yet - that's fine
      notes = [];
    }

    // Build rooms array
    const rooms = records.map(r => ({
      roomNo: '-',
      roomType: r.Book_Room_Type || '-',
      total: 0,
    }));

    const booking = {
      bookNo: firstRecord.Book_No,
      bookDate: firstRecord.Book_Date,
      checkIn: firstRecord.Book_Date_in,
      checkOut: firstRecord.Book_Date_out,
      status: mapStatus(firstRecord.Book_Status),
      statusCode: firstRecord.Book_Status,
      customer: customerDetails,
      rooms,
      roomCount: rooms.length,
      totalAmount: 0,
      notes,
    };

    return NextResponse.json({
      success: true,
      booking,
    });
  } catch (error) {
    console.error('Error fetching booking:', error);
    return NextResponse.json(
      { success: false, error: 'Failed to fetch booking' },
      { status: 500 }
    );
  }
}
