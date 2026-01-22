import { NextRequest, NextResponse } from 'next/server';
import { getPool, sql } from '@/lib/db';

export const dynamic = 'force-dynamic';

export async function GET(request: NextRequest) {
  try {
    const searchParams = request.nextUrl.searchParams;
    const search = searchParams.get('search');
    const page = parseInt(searchParams.get('page') || '1', 10);
    const limit = parseInt(searchParams.get('limit') || '20', 10);
    const offset = (page - 1) * limit;

    const pool = await getPool();
    const dbRequest = pool.request();

    let baseQuery = `
      FROM View_Customers
    `;

    if (search) {
      dbRequest.input('search', sql.NVarChar, `%${search}%`);
      baseQuery += ' WHERE Cust_name LIKE @search';
    }

    // Count query
    const countResult = await dbRequest.query(`SELECT COUNT(*) as total ${baseQuery}`);
    const total = countResult.recordset[0].total;

    // Data query with pagination
    const dataRequest = pool.request();
    if (search) dataRequest.input('search', sql.NVarChar, `%${search}%`);
    dataRequest.input('offset', sql.Int, offset);
    dataRequest.input('limit', sql.Int, limit);

    const dataQuery = `
      SELECT
        Cust_no,
        Cust_name,
        Cust_Type,
        Cust_Add_tel,
        Cust_IDcard,
        C_Address
      ${baseQuery}
      ORDER BY Cust_no
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
    console.error('Error fetching customers:', error);
    return NextResponse.json(
      {
        success: false,
        error: 'Failed to fetch customers',
      },
      { status: 500 }
    );
  }
}
