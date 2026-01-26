/**
 * Slack Integration Tests
 *
 * These tests send ACTUAL messages to Slack on every test run.
 * Requires: SLACK_WEBHOOK_URL in .env.local
 *
 * To run: pnpm test -- --testPathPattern=slack
 */

import { sendSlackMessage } from '@/lib/slack';

describe('Slack Integration', () => {
  test('should send test message to Slack', async () => {
    const result = await sendSlackMessage({
      text: '🧪 Test message from hotel system',
      blocks: [
        {
          type: 'header',
          text: {
            type: 'plain_text',
            text: '🧪 ทดสอบการแจ้งเตือน',
            emoji: true,
          },
        },
        {
          type: 'section',
          text: {
            type: 'mrkdwn',
            text: `*ข้อความทดสอบ*\nส่งจากระบบทดสอบอัตโนมัติ\nเวลา: ${new Date().toLocaleString('th-TH')}`,
          },
        },
      ],
    });

    expect(result).toBe(true);
  });
});
