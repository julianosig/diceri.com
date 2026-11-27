import { expect, test } from '@playwright/test';
import { openMyRoom, PASSWORD, signUp, uniqueUsername } from './support/accounts.ts';

test('one onboarding step creates the account, its room name and mascot', async ({ page }) => {
	const username = uniqueUsername();
	await page.goto('/');
	await expect(page.getByPlaceholder('room_username')).toBeHidden();
	await page.getByRole('button', { name: 'Open my room' }).click();
	await expect(page.getByLabel('Password', { exact: true })).toBeHidden();
	await expect(page.getByPlaceholder('room_username')).toBeFocused();
	await page.getByPlaceholder('room_username').fill(username);
	await expect(page.getByLabel(`@${username} is available — claim it`)).toBeVisible();
	await page.getByLabel('Password', { exact: true }).fill(PASSWORD);
	await page.getByLabel('Repeat password').fill('not the same');
	await expect(page.getByText('The passwords do not match.')).toBeVisible();
	await expect(page.getByRole('button', { name: 'Create my room' })).toBeDisabled();
	await page.getByLabel('Repeat password').fill(PASSWORD);
	await page.getByRole('button', { name: 'Create my room' }).click();
	// The mascot gets a step of its own, with one already picked at random.
	await expect(page.getByText('Pick your room mascot')).toBeVisible();
	await expect(page.locator('input[name=mascot]:checked')).toHaveCount(1);
	await page.getByTitle('Ninja').click();
	await page.getByRole('button', { name: 'Continue' }).click();

	// Signing up goes straight to the new room, as its host, with the chosen mascot.
	await expect(page).toHaveURL(new RegExp(`/@${username}$`));
	await expect(page.getByText('Who gets in')).toBeVisible();
	await expect(page.getByRole('img', { name: 'Ninja mascot' }).first()).toBeVisible();

	await page.getByRole('button', { name: `@${username}` }).click();
	await page.getByRole('menuitem', { name: 'Sign out' }).click();
	await page.goto('/');
	await page.getByRole('button', { name: 'Open my room' }).click();
	await page.getByPlaceholder('room_username').fill(username);
	await expect(page.getByText(`Welcome back, @${username}`)).toBeVisible();
	await page.getByLabel('Password', { exact: true }).fill(PASSWORD);
	await page.getByRole('button', { name: 'Sign in' }).click();
	await expect(page).toHaveURL(new RegExp(`/@${username}$`));
});

test('an existing username asks only for the password', async ({ page, browser }) => {
	const taken = await signUp(page);
	const visitor = await (await browser.newContext()).newPage();
	await visitor.goto('/');
	await visitor.getByRole('button', { name: 'Open my room' }).click();
	await expect(visitor.getByText('Room username')).toBeVisible();
	await visitor.getByPlaceholder('room_username').fill(taken);
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
	await expect(page.getByText('Who gets in')).toBeVisible();
	await expect(page.getByRole('button', { name: 'Join now' })).toBeVisible();
});

test('a room nobody is in shows as not live to visitors', async ({ page, browser }) => {
	const username = await signUp(page);
	await openMyRoom(page);

	// Opened, but the host has not joined the call: no door to knock on yet.
	const visitor = await (await browser.newContext()).newPage();
	await visitor.goto(`/@${username}`);
	await expect(visitor.getByText(`@${username} isn’t live right now`)).toBeVisible();
	await expect(visitor.getByRole('button', { name: 'Ask to join' })).toBeHidden();
});

test('a free @ offers to claim it, prefilled on the home page', async ({ page }) => {
	const username = uniqueUsername();
	await page.goto(`/@${username}`);
	await expect(page.getByText(`@${username} is still free!`)).toBeVisible();
	await page.getByRole('link', { name: `Claim @${username}` }).click();
	await expect(page.getByLabel(`@${username} is available`, { exact: false })).toBeVisible();
	await expect(page.getByLabel('Password', { exact: true })).toBeVisible();
});

test('a closed room shows its mascot and points to your own room', async ({ page, browser }) => {
	const username = await signUp(page);
	const visitor = await (await browser.newContext()).newPage();
	await visitor.goto(`/@${username}`);
	await expect(visitor.getByText(`@${username} isn’t live right now`)).toBeVisible();
	await visitor.getByRole('link', { name: 'Create my own room' }).click();
	await expect(visitor.getByRole('button', { name: 'Open my room' })).toBeVisible();
});
