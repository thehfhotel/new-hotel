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

    let baseQuery = `FROM View_CheckIn_Ds`;
    const conditions: string[] = [];
    const dbRequest = pool.request();
    const countRequest = pool.request();

    if (status) {
      dbRequest.input('status', sql.VarChar, status);
      countRequest.input('status', sql.VarChar, status);
      conditions.push(`Cin_status = @status`);
    }

    if (startDate) {
      dbRequest.input('startDate', sql.Date, new Date(startDate));
      countRequest.input('startDate', sql.Date, new Date(startDate));
      conditions.push(`Cin_Room_In >= @startDate`);
    }

    if (endDate) {
      dbRequest.input('endDate', sql.Date, new Date(endDate));
      countRequest.input('endDate', sql.Date, new Date(endDate));
      conditions.push(`Cin_Room_Out <= @endDate`);
    }

    if (conditions.length > 0) {
      baseQuery += ' WHERE ' + conditions.join(' AND ');
    }

    // Get total count
    const countResult = await countRequest.query(`SELECT COUNT(*) as total ${baseQuery}`);
    const total = countResult.recordset[0].total;

    // Get paginated data
    dbRequest.input('offset', sql.Int, offset);
    dbRequest.input('limit', sql.Int, limit);

    const dataQuery = `
      SELECT
        Cin_no,
        Cin_Room_No,
        Cin_Room_In,
        Cin_Room_Out,
        Cin_cust_name,
        Cin_status
      ${baseQuery}
      ORDER BY Cin_Room_In DESC
      OFFSET @offset ROWS FETCH NEXT @limit ROWS ONLY
    `;

    const result = await dbRequest.query(dataQuery);

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
    console.error('Error fetching check-ins:', error);
    return NextResponse.json(
      {
        success: false,
        error: 'Failed to fetch check-ins',
      },
      { status: 500 }
    );
  }
}
