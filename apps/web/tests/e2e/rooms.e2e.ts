import { expect, test } from '@playwright/test';
import { openMyRoom, PASSWORD, signUp, uniqueUsername } from './support/accounts.ts';

test('one onboarding step creates the account, its room name and mascot', async ({ page }) => {
	const username = uniqueUsername();
	await page.goto('/');
	await expect(page.getByLabel('Password', { exact: true })).toBeHidden();
	await page.getByPlaceholder('yourname').fill(username);
	await expect(page.getByText(`@${username} is available — claim it`)).toBeVisible();
	await page.getByLabel('Password', { exact: true }).fill(PASSWORD);
	await page.getByLabel('Repeat password').fill('not the same');
	await expect(page.getByText('The passwords do not match.')).toBeVisible();
	await expect(page.getByRole('button', { name: 'Create my room' })).toBeDisabled();
	await page.getByLabel('Repeat password').fill(PASSWORD);
	await page.getByTitle('Ninja').click();
	await page.getByRole('button', { name: 'Create my room' }).click();

	await expect(page.getByText(`@${username}`, { exact: true }).first()).toBeVisible();
	await expect(page.getByRole('button', { name: 'Open my room' })).toBeVisible();
	await expect(page.getByRole('img', { name: 'Ninja mascot' }).first()).toBeVisible();

	await page.getByRole('button', { name: 'Sign out' }).click();
	await page.getByPlaceholder('yourname').fill(username);
	await expect(page.getByText(`Welcome back, @${username}`)).toBeVisible();
	await page.getByLabel('Password', { exact: true }).fill(PASSWORD);
	await page.getByRole('button', { name: 'Sign in' }).click();
	await expect(page.getByRole('button', { name: 'Open my room' })).toBeVisible();
});

test('an existing username asks only for the password', async ({ page, browser }) => {
	const taken = await signUp(page);
	const visitor = await (await browser.newContext()).newPage();
	await visitor.goto('/');
	await expect(visitor.getByText('Login or claim your room username')).toBeVisible();
	await visitor.getByPlaceholder('yourname').fill(taken);
	await expect(visitor.getByText(`Welcome back, @${taken}`)).toBeVisible();
	await expect(visitor.getByLabel('Repeat password')).toBeHidden();
	await expect(visitor.getByRole('button', { name: 'Create my room' })).toBeHidden();
	await visitor.getByLabel('Password', { exact: true }).fill('not the password');
	await visitor.getByRole('button', { name: 'Sign in' }).click();
	await expect(visitor.getByText('Wrong password.')).toBeVisible();
});

test('the owner opens their room at /@username as host', async ({ page }) => {
	const username = await signUp(page);
	await openMyRoom(page);

	await expect(page).toHaveURL(new RegExp(`/@${username}$`));
	await expect(page.getByRole('heading', { name: 'Ready to join?' })).toBeVisible();
	await expect(page.getByText('Your room')).toBeVisible();
});

test('someone else opening /@username must ask to join', async ({ page, browser }) => {
	const username = await signUp(page);
	await openMyRoom(page);

	const guest = await (await browser.newContext()).newPage();
	await guest.goto(`/@${username}`);
	await expect(guest.getByRole('button', { name: 'Ask to join' })).toBeVisible();
	await expect(guest.getByText(`@${username}'s room`)).toBeVisible();
});

test('a free @ offers to claim it, prefilled on the home page', async ({ page }) => {
	const username = uniqueUsername();
	await page.goto(`/@${username}`);
	await expect(page.getByText(`@${username} is still free!`)).toBeVisible();
	await page.getByRole('link', { name: `Claim @${username}` }).click();
	await expect(page.getByText(`@${username} is available`, { exact: false })).toBeVisible();
	await expect(page.getByLabel('Password', { exact: true })).toBeVisible();
});

test('a closed room shows its mascot and points to your own room', async ({ page, browser }) => {
	const username = await signUp(page);
	const visitor = await (await browser.newContext()).newPage();
	await visitor.goto(`/@${username}`);
	await expect(visitor.getByText(`@${username} isn’t live right now`)).toBeVisible();
	await visitor.getByRole('link', { name: 'Create my own room' }).click();
	await expect(visitor.getByPlaceholder('yourname')).toBeVisible();
});
