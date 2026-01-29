import { NextRequest, NextResponse } from 'next/server';
import { getPool, sql } from '@/lib/db';

export const dynamic = 'force-dynamic';

export async function GET(
  request: NextRequest,
  { params }: { params: { id: string } }
) {
  try {
    const roomNo = params.id;
    const pool = await getPool();

    // Get room details
    const roomRequest = pool.request();
    roomRequest.input('roomNo', sql.NVarChar, roomNo);
    const roomResult = await roomRequest.query(`
      SELECT
        Room_no,
        Room_Type,
        Room_Details,
        Room_Clean,
        Room_Use,
        Room_Book,
        Room_Manternace,
        Room_PriceA,
        Room_PriceB,
        Room_PriceC,
        Room_Group,
        Room_Book_Name,
        Room_Book_Time
      FROM HT_Rooms
      WHERE Room_no = @roomNo
    `);

    if (roomResult.recordset.length === 0) {
      return NextResponse.json(
        { success: false, error: 'Room not found' },
        { status: 404 }
      );
    }

    const room = roomResult.recordset[0];

    // Get current/recent check-in for this room
    const checkinRequest = pool.request();
    checkinRequest.input('roomNo', sql.NVarChar, roomNo);
    const checkinResult = await checkinRequest.query(`
      SELECT TOP 1
        Cin_Cust_Name,
        Cin_Room_In,
        Cin_Room_Out
      FROM View_CheckIn_Ds
      WHERE Cin_Room_No = @roomNo
      ORDER BY Cin_Room_In DESC
    `);

    const currentGuest = checkinResult.recordset[0] || null;

    return NextResponse.json({
      success: true,
      room: {
        ...room,
        currentGuest: currentGuest ? {
          name: currentGuest.Cin_Cust_Name,
          checkIn: currentGuest.Cin_Room_In,
          checkOut: currentGuest.Cin_Room_Out,
        } : null,
      },
    });
  } catch (error) {
    console.error('Error fetching room details:', error);
    return NextResponse.json(
      { success: false, error: 'Failed to fetch room details' },
      { status: 500 }
    );
  }
}
