import { expect, test } from '@playwright/test';

test('landing page offers to start a meeting', async ({ page }) => {
	await page.goto('/');
	await expect(page.getByRole('heading', { level: 1 })).toBeVisible();
	await expect(page.getByRole('button', { name: 'New meeting' })).toBeVisible();
});
