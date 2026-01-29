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
    const sortBy = searchParams.get('sortBy');
    const sortOrder = searchParams.get('sortOrder')?.toLowerCase() === 'desc' ? 'DESC' : 'ASC';
    const includeLastVisit = searchParams.get('includeLastVisit') === 'true';

    // Map frontend column names to SQL columns
    const columnMap: Record<string, string> = {
      id: 'CAST(SUBSTRING(c.Cust_no, 2, LEN(c.Cust_no)-1) AS INT)', // Numeric sort for C0001 format
      name: 'c.Cust_name',
      type: 'c.Cust_Type',
      phone: 'c.Cust_Add_tel',
      lastVisit: 'lv.lastVisit',
    };

    // Default sort order: by id (numeric)
    let orderByColumn = sortBy && columnMap[sortBy] ? columnMap[sortBy] : columnMap['id'];
    // Handle nulls last for lastVisit (only if lastVisit is included)
    let orderByClause: string;
    if (sortBy === 'lastVisit' && includeLastVisit) {
      orderByClause = `CASE WHEN lv.lastVisit IS NULL THEN 1 ELSE 0 END, ${orderByColumn} ${sortOrder}`;
    } else if (sortBy === 'lastVisit' && !includeLastVisit) {
      // Fall back to id sort if sorting by lastVisit but it's not included
      orderByColumn = columnMap['id'];
      orderByClause = `${orderByColumn} ${sortOrder}`;
    } else {
      orderByClause = `${orderByColumn} ${sortOrder}`;
    }

    const pool = await getPool();
    const dbRequest = pool.request();

    // Base WHERE clause
    let whereClause = '';
    if (search) {
      dbRequest.input('search', sql.NVarChar, `%${search}%`);
      whereClause = ` WHERE c.Cust_name LIKE @search
         OR c.Cust_Add_tel LIKE @search
         OR c.Cust_IDcard LIKE @search
         OR CAST(c.Cust_no AS NVARCHAR) LIKE @search`;
    }

    // Count query
    const countQuery = `SELECT COUNT(*) as total FROM View_Customers c ${whereClause}`;
    const countResult = await dbRequest.query(countQuery);
    const total = countResult.recordset[0].total;

    // Data query with pagination
    const dataRequest = pool.request();
    if (search) dataRequest.input('search', sql.NVarChar, `%${search}%`);
    dataRequest.input('offset', sql.Int, offset);
    dataRequest.input('limit', sql.Int, limit);

    // Build query with optional lastVisit JOIN
    const lastVisitSelect = includeLastVisit ? ',\n        lv.lastVisit' : '';
    const lastVisitJoin = includeLastVisit ? `
      LEFT JOIN (
        SELECT Book_Cust_ID, MAX(Book_Date_out) as lastVisit
        FROM View_Booking_Ds
        GROUP BY Book_Cust_ID
      ) lv ON c.Cust_no = lv.Book_Cust_ID` : '';

    const dataQuery = `
      SELECT
        c.Cust_no,
        c.Cust_name,
        c.Cust_Type,
        c.Cust_Add_tel,
        c.Cust_IDcard,
        c.C_Address${lastVisitSelect}
      FROM View_Customers c${lastVisitJoin}
      ${whereClause}
      ORDER BY ${orderByClause}
      OFFSET @offset ROWS FETCH NEXT @limit ROWS ONLY
    `;

    const result = await dataRequest.query(dataQuery);

    // Map the data to frontend-expected format
    const customers = result.recordset.map(row => ({
      id: row.Cust_no,
      name: row.Cust_name,
      type: row.Cust_Type,
      phone: row.Cust_Add_tel,
      idCard: row.Cust_IDcard,
      address: row.C_Address,
      lastVisit: includeLastVisit ? row.lastVisit : null,
    }));

    return NextResponse.json({
      success: true,
      customers,
      total,
      page,
      limit,
      totalPages: Math.ceil(total / limit),
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
