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
        Book_Cust_ID,
        Book_Cust_Name,
        Book_Status,
        Book_Room_No,
        Book_Room_Type,
        Book_Total
      FROM View_Booking_Ds
      WHERE Book_No = @bookNo
      ORDER BY Book_Room_No
    `);

    if (bookingResult.recordset.length === 0) {
      return NextResponse.json(
        { success: false, error: 'Booking not found' },
        { status: 404 }
      );
    }

    const records = bookingResult.recordset;
    const firstRecord = records[0];

    // Fetch customer details if we have a customer ID
    let customerDetails = null;
    if (firstRecord.Book_Cust_ID) {
      const customerRequest = pool.request();
      customerRequest.input('custId', sql.NVarChar, firstRecord.Book_Cust_ID);

      const customerResult = await customerRequest.query(`
        SELECT
          Cust_no,
          Cust_Fname,
          Cust_Lname,
          Cust_Phone,
          Cust_IDcard,
          Cust_Addr,
          Cust_Type
        FROM View_Customers
        WHERE Cust_no = @custId
      `);

      if (customerResult.recordset.length > 0) {
        const c = customerResult.recordset[0];
        customerDetails = {
          id: c.Cust_no,
          firstName: c.Cust_Fname || '',
          lastName: c.Cust_Lname || '',
          fullName: `${c.Cust_Fname || ''} ${c.Cust_Lname || ''}`.trim(),
          phone: c.Cust_Phone || '',
          idCard: c.Cust_IDcard || '',
          address: c.Cust_Addr || '',
          type: c.Cust_Type || 'Regular',
        };
      }
    }

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
      roomNo: r.Book_Room_No || '-',
      roomType: r.Book_Room_Type || '-',
      total: r.Book_Total || 0,
    }));

    const totalAmount = rooms.reduce((sum, r) => sum + r.total, 0);

    const booking = {
      bookNo: firstRecord.Book_No,
      bookDate: firstRecord.Book_Date,
      checkIn: firstRecord.Book_Date_in,
      checkOut: firstRecord.Book_Date_out,
      status: mapStatus(firstRecord.Book_Status),
      statusCode: firstRecord.Book_Status,
      customer: customerDetails || {
        id: firstRecord.Book_Cust_ID || '',
        fullName: firstRecord.Book_Cust_Name || '',
      },
      rooms,
      roomCount: rooms.length,
      totalAmount,
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
