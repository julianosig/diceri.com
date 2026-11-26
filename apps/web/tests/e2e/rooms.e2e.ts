import { expect, test } from '@playwright/test';
import { signUp, uniqueUsername } from './support/accounts.ts';

test('claiming a username shows the sign-in code once and it signs in again', async ({ page }) => {
	const username = uniqueUsername();
	await page.goto('/');
	await page.getByPlaceholder('yourname').fill(username);
	await page.getByRole('button', { name: 'Claim username' }).click();
	await expect(page.getByText(`@${username}`, { exact: true })).toBeVisible();
	const code = (await page.getByTestId('account-key').textContent()) ?? '';
	await page.getByRole('button', { name: 'I saved it' }).click();
	await expect(page.getByTestId('account-key')).toBeHidden();

	await page.getByRole('button', { name: 'Sign out' }).click();
	await page.getByRole('button', { name: 'I already have a username' }).click();
	await page.getByLabel('Your sign-in code').fill(code);
	await page.getByRole('button', { name: 'Sign in' }).click();
	await expect(page.getByText(`@${username}`, { exact: true })).toBeVisible();
	await expect(page.getByRole('button', { name: 'New meeting' })).toBeVisible();
});

test('creator lands on the pre-join screen as host', async ({ page }) => {
	await signUp(page);
	await page.goto('/');
	await page.getByRole('button', { name: 'New meeting' }).click();

	await expect(page).toHaveURL(/\/r\/[0-9A-Za-z]{22}$/);
	await expect(page.getByRole('heading', { name: 'Ready to join?' })).toBeVisible();
	await expect(page.getByRole('button', { name: 'Join now' })).toBeVisible();
});

test('someone else opening the link must ask to join', async ({ page, browser }) => {
	await signUp(page);
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
