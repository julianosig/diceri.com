import { expect, test } from '@playwright/test';

test('creator lands on the pre-join screen as host', async ({ page }) => {
	await page.goto('/');
	await page.getByRole('button', { name: 'New meeting' }).click();

	await expect(page).toHaveURL(/\/r\/[0-9A-Za-z]{22}$/);
	await expect(page.getByRole('heading', { name: 'Ready to join?' })).toBeVisible();
	await expect(page.getByRole('button', { name: 'Join now' })).toBeVisible();
});

test('someone else opening the link must ask to join', async ({ page, browser }) => {
	await page.goto('/');
	await page.getByRole('button', { name: 'New meeting' }).click();
	await expect(page).toHaveURL(/\/r\//);

	const guest = await (await browser.newContext()).newPage();
	await guest.goto(page.url());
	await expect(guest.getByRole('button', { name: 'Ask to join' })).toBeVisible();
});

test('unknown room shows a not-found error', async ({ page }) => {
	await page.goto('/r/aaaaaaaaaaaaaaaaaaaaaa');
	await expect(page.getByText('This meeting does not exist or has ended.')).toBeVisible();
});
