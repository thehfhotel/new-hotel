import sql from 'mssql';

const config: sql.config = {
  server: process.env.DB_SERVER || '192.168.100.222',
  database: process.env.DB_NAME || 'db',
  user: process.env.DB_USER || 'sa',
  password: process.env.DB_PASSWORD || '***REMOVED***',
  options: {
    encrypt: false,
    trustServerCertificate: true,
    enableArithAbort: true,
  },
  pool: {
    max: 10,
    min: 0,
    idleTimeoutMillis: 30000,
  },
};

let pool: sql.ConnectionPool | null = null;

export async function getPool(): Promise<sql.ConnectionPool> {
  try {
    if (pool) {
      return pool;
    }
    pool = await sql.connect(config);
    return pool;
  } catch (error) {
    console.error('Database connection error:', error);
    throw new Error('Failed to connect to database');
  }
}

export { sql };
