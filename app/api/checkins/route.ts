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

    if (status) {
      conditions.push(`Cin_status = '${status.replace(/'/g, "''")}'`);
    }

    if (startDate) {
      conditions.push(`Cin_Room_In >= '${startDate}'`);
    }

    if (endDate) {
      conditions.push(`Cin_Room_Out <= '${endDate}'`);
    }

    if (conditions.length > 0) {
      baseQuery += ' WHERE ' + conditions.join(' AND ');
    }

    // Get total count
    const countResult = await pool.request().query(`SELECT COUNT(*) as total ${baseQuery}`);
    const total = countResult.recordset[0].total;

    // Get paginated data
    const dataRequest = pool.request();
    dataRequest.input('offset', sql.Int, offset);
    dataRequest.input('limit', sql.Int, limit);

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

    const result = await dataRequest.query(dataQuery);

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
