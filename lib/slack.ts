// Slack notification utilities

interface SlackBlock {
  type: string;
  text?: {
    type: string;
    text: string;
    emoji?: boolean;
  };
}

interface SlackMessage {
  text: string;
  blocks?: SlackBlock[];
}

/**
 * Send a message to Slack via webhook
 * Retries up to 3 times with exponential backoff (1s, 2s, 4s)
 */
export async function sendSlackMessage(message: SlackMessage): Promise<boolean> {
  const webhookUrl = process.env.SLACK_WEBHOOK_URL;
  const enabled = process.env.SLACK_NOTIFICATIONS_ENABLED !== 'false';

  if (!enabled) {
    console.log('[Slack] Notifications disabled');
    return false;
  }

  if (!webhookUrl) {
    console.warn('[Slack] SLACK_WEBHOOK_URL not configured, skipping notification');
    return false;
  }

  const maxRetries = 3;
  let lastError: Error | null = null;

  for (let attempt = 1; attempt <= maxRetries; attempt++) {
    try {
      const response = await fetch(webhookUrl, {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
        },
        body: JSON.stringify(message),
      });

      if (response.ok) {
        console.log('[Slack] Message sent successfully');
        return true;
      }

      const errorText = await response.text();
      throw new Error(`Slack API error (${response.status}): ${errorText}`);
    } catch (error) {
      lastError = error instanceof Error ? error : new Error(String(error));
      console.error(`[Slack] Attempt ${attempt}/${maxRetries} failed:`, lastError.message);

      if (attempt < maxRetries) {
        const delay = Math.pow(2, attempt - 1) * 1000; // 1s, 2s, 4s
        await new Promise((resolve) => setTimeout(resolve, delay));
      }
    }
  }

  console.error('[Slack] All retry attempts failed:', lastError?.message);
  return false;
}

/**
 * Format Thai date string for database dates
 * Uses UTC because database stores local Thai time but mssql marks it as UTC
 */
function formatThaiDateFromDb(date: Date): string {
  const thaiMonths = [
    'มกราคม', 'กุมภาพันธ์', 'มีนาคม', 'เมษายน', 'พฤษภาคม', 'มิถุนายน',
    'กรกฎาคม', 'สิงหาคม', 'กันยายน', 'ตุลาคม', 'พฤศจิกายน', 'ธันวาคม'
  ];

  const day = date.getUTCDate();
  const month = thaiMonths[date.getUTCMonth()];
  const year = date.getUTCFullYear() + 543; // Buddhist Era

  return `${day} ${month} ${year}`;
}

/**
 * Format Thai date string for current time (uses Asia/Bangkok timezone)
 */
function formatThaiDateNow(date: Date): string {
  const thaiMonths = [
    'มกราคม', 'กุมภาพันธ์', 'มีนาคม', 'เมษายน', 'พฤษภาคม', 'มิถุนายน',
    'กรกฎาคม', 'สิงหาคม', 'กันยายน', 'ตุลาคม', 'พฤศจิกายน', 'ธันวาคม'
  ];

  // Get Thai time components
  const formatter = new Intl.DateTimeFormat('en-US', {
    timeZone: 'Asia/Bangkok',
    day: 'numeric',
    month: 'numeric',
    year: 'numeric',
  });
  const parts = formatter.formatToParts(date);
  const day = parseInt(parts.find(p => p.type === 'day')?.value || '1');
  const monthIndex = parseInt(parts.find(p => p.type === 'month')?.value || '1') - 1;
  const year = parseInt(parts.find(p => p.type === 'year')?.value || '2024') + 543;

  return `${day} ${thaiMonths[monthIndex]} ${year}`;
}

/**
 * Format time string (HH:MM) for database dates
 * Uses UTC because database stores local Thai time but mssql marks it as UTC
 */
function formatTimeFromDb(date: Date): string {
  return date.toLocaleTimeString('th-TH', {
    hour: '2-digit',
    minute: '2-digit',
    hour12: false,
    timeZone: 'UTC',
  });
}

/**
 * Format time string (HH:MM) for current time (uses Asia/Bangkok timezone)
 */
function formatTimeNow(date: Date): string {
  return date.toLocaleTimeString('th-TH', {
    hour: '2-digit',
    minute: '2-digit',
    hour12: false,
    timeZone: 'Asia/Bangkok',
  });
}

/**
 * Build hourly report message for Slack
 */
export function buildHourlyReportMessage(
  occupiedRooms: number,
  totalRooms: number,
  todayBookings: number
): SlackMessage {
  const now = new Date();
  const occupancyPercent = totalRooms > 0 ? Math.round((occupiedRooms / totalRooms) * 100) : 0;

  const text = `รายงานประจำชั่วโมง - ห้องที่มีผู้เข้าพัก: ${occupiedRooms}/${totalRooms} (${occupancyPercent}%), การจองวันนี้: ${todayBookings} รายการ`;

  return {
    text,
    blocks: [
      {
        type: 'header',
        text: {
          type: 'plain_text',
          text: ':hotel: รายงานประจำชั่วโมง',
          emoji: true,
        },
      },
      {
        type: 'section',
        text: {
          type: 'mrkdwn',
          text: [
            '─────────────────────────',
            `*วันที่:* ${formatThaiDateNow(now)}`,
            `*เวลา:* ${formatTimeNow(now)}`,
            '',
            `:bed: *ห้องที่มีผู้เข้าพัก:* ${occupiedRooms}/${totalRooms} (${occupancyPercent}%)`,
            `:calendar: *การจองวันนี้:* ${todayBookings} รายการ`,
          ].join('\n'),
        },
      },
    ],
  };
}

/**
 * Build check-in alert message for Slack
 */
export function buildCheckInAlertMessage(
  guestName: string,
  roomNumber: string,
  checkInTime: Date
): SlackMessage {
  const text = `เช็คอินใหม่ - ${guestName} ห้อง ${roomNumber}`;

  return {
    text,
    blocks: [
      {
        type: 'header',
        text: {
          type: 'plain_text',
          text: ':key: เช็คอินใหม่!',
          emoji: true,
        },
      },
      {
        type: 'section',
        text: {
          type: 'mrkdwn',
          text: [
            '',
            `:bust_in_silhouette: *ชื่อผู้เข้าพัก:* ${guestName}`,
            `:door: *ห้อง:* ${roomNumber}`,
            `:clock3: *เวลา:* ${formatTimeFromDb(checkInTime)}`,
          ].join('\n'),
        },
      },
    ],
  };
}

/**
 * Build check-out alert message for Slack
 */
export function buildCheckOutAlertMessage(
  guestName: string,
  roomNumber: string,
  checkOutTime: Date
): SlackMessage {
  const text = `เช็คเอาท์ - ${guestName} ห้อง ${roomNumber}`;

  return {
    text,
    blocks: [
      {
        type: 'header',
        text: {
          type: 'plain_text',
          text: ':wave: เช็คเอาท์!',
          emoji: true,
        },
      },
      {
        type: 'section',
        text: {
          type: 'mrkdwn',
          text: [
            '─────────────────────────',
            `:bust_in_silhouette: *ชื่อผู้เข้าพัก:* ${guestName}`,
            `:door: *ห้อง:* ${roomNumber}`,
            `:clock3: *เวลา:* ${formatTimeFromDb(checkOutTime)}`,
          ].join('\n'),
        },
      },
    ],
  };
}

/**
 * Build new booking alert message for Slack
 */
export function buildNewBookingAlertMessage(
  guestName: string,
  roomType: string,
  checkInDate: Date,
  checkOutDate: Date
): SlackMessage {
  const text = `การจองใหม่ - ${guestName} ประเภทห้อง ${roomType}`;

  return {
    text,
    blocks: [
      {
        type: 'header',
        text: {
          type: 'plain_text',
          text: ':calendar: การจองใหม่!',
          emoji: true,
        },
      },
      {
        type: 'section',
        text: {
          type: 'mrkdwn',
          text: [
            '─────────────────────────',
            `:bust_in_silhouette: *ชื่อผู้จอง:* ${guestName}`,
            `:bed: *ประเภทห้อง:* ${roomType}`,
            `:airplane_arriving: *วันเข้าพัก:* ${formatThaiDateFromDb(checkInDate)}`,
            `:airplane_departure: *วันออก:* ${formatThaiDateFromDb(checkOutDate)}`,
          ].join('\n'),
        },
      },
    ],
  };
}
