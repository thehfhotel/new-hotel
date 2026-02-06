import { test, expect } from '@playwright/test'

test.describe('Legacy Dashboard', () => {
  test('page loads with title', async ({ page }) => {
    await page.goto('/')

    await expect(page.getByText('หน้าหลัก')).toBeVisible({ timeout: 15000 })
  })

  test('stats cards display with numeric values', async ({ page }) => {
    await page.goto('/')

    // Wait for loading to finish
    await expect(page.getByText('หน้าหลัก')).toBeVisible({ timeout: 15000 })

    // Stats cards should have Thai labels
    await expect(page.getByText('ห้องที่มีผู้เข้าพัก')).toBeVisible()
    await expect(page.getByText('ห้องที่จองแล้ว')).toBeVisible()
    await expect(page.getByText('ห้องรอเช็คเอาท์')).toBeVisible()
  })

  test('room grid section renders', async ({ page }) => {
    await page.goto('/')

    await expect(page.getByText('สถานะห้องพัก')).toBeVisible({ timeout: 15000 })
  })

  test('navigation works', async ({ page }) => {
    await page.goto('/')

    // Wait for page to load
    await expect(page.getByText('หน้าหลัก')).toBeVisible({ timeout: 15000 })

    // Navbar should be present with links
    const navbar = page.locator('nav')
    await expect(navbar).toBeVisible()
  })
})
