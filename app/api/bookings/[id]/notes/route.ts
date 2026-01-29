import { NextRequest, NextResponse } from 'next/server';
import { getPool, sql } from '@/lib/db';

export const dynamic = 'force-dynamic';

// Ensure the notes table exists
async function ensureNotesTable(pool: Awaited<ReturnType<typeof getPool>>) {
  try {
    await pool.request().query(`
      IF NOT EXISTS (SELECT * FROM sysobjects WHERE name='HT_Booking_Notes' AND xtype='U')
      CREATE TABLE HT_Booking_Notes (
        Note_ID INT IDENTITY(1,1) PRIMARY KEY,
        Book_No NVARCHAR(50) NOT NULL,
        Note_Text NVARCHAR(MAX) NOT NULL,
        Created_At DATETIME DEFAULT GETDATE(),
        Updated_At DATETIME DEFAULT GETDATE()
      )
    `);

    // Create index if it doesn't exist
    await pool.request().query(`
      IF NOT EXISTS (SELECT * FROM sys.indexes WHERE name='IX_Booking_Notes_BookNo')
      CREATE INDEX IX_Booking_Notes_BookNo ON HT_Booking_Notes(Book_No)
    `);
  } catch (error) {
    console.error('Error ensuring notes table:', error);
    throw error;
  }
}

// GET - Fetch all notes for a booking
export async function GET(
  request: NextRequest,
  { params }: { params: Promise<{ id: string }> }
) {
  try {
    const { id: bookNo } = await params;
    const pool = await getPool();

    await ensureNotesTable(pool);

    const notesRequest = pool.request();
    notesRequest.input('bookNo', sql.NVarChar, bookNo);

    const result = await notesRequest.query(`
      SELECT Note_ID, Note_Text, Created_At, Updated_At
      FROM HT_Booking_Notes
      WHERE Book_No = @bookNo
      ORDER BY Created_At DESC
    `);

    const notes = result.recordset.map(n => ({
      id: n.Note_ID,
      text: n.Note_Text,
      createdAt: n.Created_At,
      updatedAt: n.Updated_At,
    }));

    return NextResponse.json({
      success: true,
      notes,
    });
  } catch (error) {
    console.error('Error fetching notes:', error);
    return NextResponse.json(
      { success: false, error: 'Failed to fetch notes' },
      { status: 500 }
    );
  }
}

// POST - Add a new note
export async function POST(
  request: NextRequest,
  { params }: { params: Promise<{ id: string }> }
) {
  try {
    const { id: bookNo } = await params;
    const body = await request.json();
    const { text } = body;

    if (!text || typeof text !== 'string' || text.trim().length === 0) {
      return NextResponse.json(
        { success: false, error: 'Note text is required' },
        { status: 400 }
      );
    }

    const pool = await getPool();
    await ensureNotesTable(pool);

    const insertRequest = pool.request();
    insertRequest.input('bookNo', sql.NVarChar, bookNo);
    insertRequest.input('text', sql.NVarChar, text.trim());

    const result = await insertRequest.query(`
      INSERT INTO HT_Booking_Notes (Book_No, Note_Text)
      OUTPUT INSERTED.Note_ID, INSERTED.Note_Text, INSERTED.Created_At, INSERTED.Updated_At
      VALUES (@bookNo, @text)
    `);

    const note = result.recordset[0];

    return NextResponse.json({
      success: true,
      note: {
        id: note.Note_ID,
        text: note.Note_Text,
        createdAt: note.Created_At,
        updatedAt: note.Updated_At,
      },
    });
  } catch (error) {
    console.error('Error creating note:', error);
    return NextResponse.json(
      { success: false, error: 'Failed to create note' },
      { status: 500 }
    );
  }
}

// DELETE - Delete a note
export async function DELETE(
  request: NextRequest,
  { params }: { params: Promise<{ id: string }> }
) {
  try {
    const { id: bookNo } = await params;
    const { searchParams } = new URL(request.url);
    const noteId = searchParams.get('noteId');

    if (!noteId) {
      return NextResponse.json(
        { success: false, error: 'Note ID is required' },
        { status: 400 }
      );
    }

    const pool = await getPool();

    const deleteRequest = pool.request();
    deleteRequest.input('noteId', sql.Int, parseInt(noteId, 10));
    deleteRequest.input('bookNo', sql.NVarChar, bookNo);

    const result = await deleteRequest.query(`
      DELETE FROM HT_Booking_Notes
      WHERE Note_ID = @noteId AND Book_No = @bookNo
    `);

    if (result.rowsAffected[0] === 0) {
      return NextResponse.json(
        { success: false, error: 'Note not found' },
        { status: 404 }
      );
    }

    return NextResponse.json({
      success: true,
      message: 'Note deleted',
    });
  } catch (error) {
    console.error('Error deleting note:', error);
    return NextResponse.json(
      { success: false, error: 'Failed to delete note' },
      { status: 500 }
    );
  }
}
