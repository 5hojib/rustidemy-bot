
import { test, expect } from '@playwright/test';

test('homepage has a title and fetches courses', async ({ page }) => {
  await page.goto('http://localhost:3000/');

  // Wait for the page to load and have a title.
  await expect(page).toHaveTitle(/Udemy Coupon Hunter | 100% Off Deals/);

  // Wait for the courses to be fetched and displayed.
  // This is a proxy for the backend being up and running.
  await expect(page.locator('div.grid > div').first()).toBeVisible({ timeout: 10000 });

  // Take a screenshot of the page.
  await page.screenshot({ path: '/home/jules/verification/verification.png' });
});
