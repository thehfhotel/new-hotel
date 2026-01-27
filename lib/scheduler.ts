import cron from 'node-cron';
import { getPool } from './db';
import {
  sendSlackMessage,
  buildHourlyReportMessage,
  buildCheckInAlertMessage,
  buildCheckOutAlertMessage,
  buildNewBookingAlertMessage,
} from './slack';

// Track last checked timestamp for check-in polling
let lastCheckedTimestamp: Date | null = null;

// Track last checked timestamp for checkout polling
let lastCheckoutTimestamp: Date | null = null;

// Track last checked timestamp for booking polling
let lastBookingTimestamp: Date | null = null;

// Track if scheduler is initialized
let isInitialized = false;

/**
 * Fetch hourly report data and send to Slack
 */
async function sendHourlyReport(): Promise<void> {
  console.log('[Scheduler] Running hourly report...');

  try {
    const pool = await getPool();

    // Get occupied rooms count
    const occupiedResult = await pool.request().query(`
      SELECT COUNT(*) as count FROM HT_Rooms
      WHERE Room_Use = 'yes' OR Room_Book = 'yes'
    `);
    const occupiedRooms = occupiedResult.recordset[0]?.count ?? 0;

    // Get total rooms count
    const totalResult = await pool.request().query(`
      SELECT COUNT(*) as count FROM HT_Rooms
    `);
    const totalRooms = totalResult.recordset[0]?.count ?? 0;

    // Get today's new bookings count
    const bookingsResult = await pool.request().query(`
      SELECT COUNT(*) as count FROM View_Booking_Ds
      WHERE CAST(Book_Date AS DATE) = CAST(GETDATE() AS DATE)
    `);
    const todayBookings = bookingsResult.recordset[0]?.count ?? 0;

    const message = buildHourlyReportMessage(occupiedRooms, totalRooms, todayBookings);
    await sendSlackMessage(message);

    console.log('[Scheduler] Hourly report sent successfully');
  } catch (error) {
    console.error('[Scheduler] Error sending hourly report:', error);
  }
}

/**
 * Poll for new check-ins and send alerts to Slack
 */
async function pollCheckIns(): Promise<void> {
  console.log('[Scheduler] Polling for new check-ins...');

  try {
    const pool = await getPool();

    // Initialize lastCheckedTimestamp to current time on first run
    if (!lastCheckedTimestamp) {
      lastCheckedTimestamp = new Date();
      console.log('[Scheduler] Initialized check-in polling from:', lastCheckedTimestamp.toISOString());
      return;
    }

    // Query for new check-ins since last poll
    const result = await pool.request()
      .input('lastChecked', lastCheckedTimestamp)
      .query(`
        SELECT Cin_no, Cin_Room_No, Cin_Room_In, Cin_cust_name
        FROM View_CheckIn_Ds
        WHERE Cin_Room_In > @lastChecked
        ORDER BY Cin_Room_In ASC
      `);

    const newCheckIns = result.recordset;

    if (newCheckIns.length > 0) {
      console.log(`[Scheduler] Found ${newCheckIns.length} new check-in(s)`);

      for (const checkIn of newCheckIns) {
        // Parse check-in time - database stores local Thai time
        // mssql returns it as ISO string with Z, but it's actually local time
        const checkInTimeStr = checkIn.Cin_Room_In;
        const checkInTime = new Date(checkInTimeStr);

        const message = buildCheckInAlertMessage(
          checkIn.Cin_cust_name || 'Unknown',
          checkIn.Cin_Room_No || 'Unknown',
          checkInTime
        );

        await sendSlackMessage(message);

        // Update lastCheckedTimestamp to the latest check-in time
        if (!lastCheckedTimestamp || checkInTime > lastCheckedTimestamp) {
          lastCheckedTimestamp = checkInTime;
        }
      }
    } else {
      console.log('[Scheduler] No new check-ins found');
    }
  } catch (error) {
    console.error('[Scheduler] Error polling check-ins:', error);
  }
}

/**
 * Poll for new checkouts and send alerts to Slack
 */
async function pollCheckouts(): Promise<void> {
  console.log('[Scheduler] Polling for new checkouts...');

  try {
    const pool = await getPool();

    // Initialize lastCheckoutTimestamp to current time on first run
    if (!lastCheckoutTimestamp) {
      lastCheckoutTimestamp = new Date();
      console.log('[Scheduler] Initialized checkout polling from:', lastCheckoutTimestamp.toISOString());
      return;
    }

    // Query for new checkouts since last poll
    const result = await pool.request()
      .input('lastChecked', lastCheckoutTimestamp)
      .query(`
        SELECT Cin_no, Cin_Room_No, Cin_Room_Out, Cin_cust_name
        FROM View_CheckIn_Ds
        WHERE Cin_Room_Out > @lastChecked
        ORDER BY Cin_Room_Out ASC
      `);

    const newCheckouts = result.recordset;

    if (newCheckouts.length > 0) {
      console.log(`[Scheduler] Found ${newCheckouts.length} new checkout(s)`);

      for (const checkout of newCheckouts) {
        // Parse checkout time - database stores local Thai time
        const checkoutTimeStr = checkout.Cin_Room_Out;
        const checkoutTime = new Date(checkoutTimeStr);

        const message = buildCheckOutAlertMessage(
          checkout.Cin_cust_name || 'Unknown',
          checkout.Cin_Room_No || 'Unknown',
          checkoutTime
        );

        await sendSlackMessage(message);

        // Update lastCheckoutTimestamp to the latest checkout time
        if (!lastCheckoutTimestamp || checkoutTime > lastCheckoutTimestamp) {
          lastCheckoutTimestamp = checkoutTime;
        }
      }
    } else {
      console.log('[Scheduler] No new checkouts found');
    }
  } catch (error) {
    console.error('[Scheduler] Error polling checkouts:', error);
  }
}

/**
 * Poll for new bookings and send alerts to Slack
 */
async function pollNewBookings(): Promise<void> {
  console.log('[Scheduler] Polling for new bookings...');

  try {
    const pool = await getPool();

    // Initialize lastBookingTimestamp to current time on first run
    if (!lastBookingTimestamp) {
      lastBookingTimestamp = new Date();
      console.log('[Scheduler] Initialized booking polling from:', lastBookingTimestamp.toISOString());
      return;
    }

    // Query for new bookings since last poll
    const result = await pool.request()
      .input('lastChecked', lastBookingTimestamp)
      .query(`
        SELECT Book_no, Book_Cust_Name, Book_Room_Type, Book_Date_in, Book_Date_out, Book_Date
        FROM View_Booking_Ds
        WHERE Book_Date > @lastChecked
        ORDER BY Book_Date ASC
      `);

    const newBookings = result.recordset;

    if (newBookings.length > 0) {
      console.log(`[Scheduler] Found ${newBookings.length} new booking(s)`);

      for (const booking of newBookings) {
        // Parse dates - database stores local Thai time
        const bookingTime = new Date(booking.Book_Date);
        const checkInDate = new Date(booking.Book_Date_in);
        const checkOutDate = new Date(booking.Book_Date_out);

        const message = buildNewBookingAlertMessage(
          booking.Book_Cust_Name || 'Unknown',
          booking.Book_Room_Type || 'Unknown',
          checkInDate,
          checkOutDate
        );

        await sendSlackMessage(message);

        // Update lastBookingTimestamp to the latest booking time
        if (!lastBookingTimestamp || bookingTime > lastBookingTimestamp) {
          lastBookingTimestamp = bookingTime;
        }
      }
    } else {
      console.log('[Scheduler] No new bookings found');
    }
  } catch (error) {
    console.error('[Scheduler] Error polling bookings:', error);
  }
}

/**
 * Initialize the cron job scheduler
 * Should be called once on app startup
 */
export function initScheduler(): void {
  if (isInitialized) {
    console.log('[Scheduler] Already initialized, skipping...');
    return;
  }

  const webhookUrl = process.env.SLACK_WEBHOOK_URL;
  const enabled = process.env.SLACK_NOTIFICATIONS_ENABLED !== 'false';

  if (!enabled) {
    console.log('[Scheduler] Slack notifications disabled, scheduler not started');
    return;
  }

  if (!webhookUrl) {
    console.warn('[Scheduler] SLACK_WEBHOOK_URL not configured, scheduler not started');
    return;
  }

  console.log('[Scheduler] Starting cron jobs...');

  // Hourly report at minute 0 of every hour
  cron.schedule('0 * * * *', () => {
    sendHourlyReport().catch((error) => {
      console.error('[Scheduler] Unhandled error in hourly report:', error);
    });
  });

  // Poll for check-ins every 2 minutes
  cron.schedule('*/2 * * * *', () => {
    pollCheckIns().catch((error) => {
      console.error('[Scheduler] Unhandled error in check-in polling:', error);
    });
  });

  // Poll for checkouts every 2 minutes
  cron.schedule('*/2 * * * *', () => {
    pollCheckouts().catch((error) => {
      console.error('[Scheduler] Unhandled error in checkout polling:', error);
    });
  });

  // Poll for new bookings every 2 minutes
  cron.schedule('*/2 * * * *', () => {
    pollNewBookings().catch((error) => {
      console.error('[Scheduler] Unhandled error in booking polling:', error);
    });
  });

  isInitialized = true;
  console.log('[Scheduler] Cron jobs started');
  console.log('[Scheduler] - Hourly report: minute 0 of every hour');
  console.log('[Scheduler] - Check-in polling: every 2 minutes');
  console.log('[Scheduler] - Checkout polling: every 2 minutes');
  console.log('[Scheduler] - Booking polling: every 2 minutes');
}
