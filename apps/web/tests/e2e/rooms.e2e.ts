import { expect, test } from '@playwright/test';

test('creates a meeting and lands on a shareable room page', async ({ page }) => {
	await page.goto('/');
	await page.getByRole('button', { name: 'New meeting' }).click();

	await expect(page).toHaveURL(/\/r\/[0-9A-Za-z]{22}$/);
	await expect(page.getByRole('heading', { name: 'Your meeting is ready' })).toBeVisible();
	await expect(page.getByText(page.url())).toBeVisible();
});

test('unknown room shows a not-found error', async ({ page }) => {
	await page.goto('/r/aaaaaaaaaaaaaaaaaaaaaa');
	await expect(page.getByText('This meeting does not exist or has ended.')).toBeVisible();
});
