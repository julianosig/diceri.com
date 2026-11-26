import { openMyRoom, signUp } from './support/accounts.ts';
import { expect, test, type Browser, type Page } from '@playwright/test';

/**
 * Signs the page up (rooms need an account) and skips when the API has no media
 * server configured (no DICERI_LIVEKIT_*). Returns the username.
 */
async function skipWithoutMedia(page: Page): Promise<string> {
	const username = await signUp(page);
	const room = (await (await page.request.post('/api/v1/rooms', { data: {} })).json()) as {
		code: string;
		hostKey: string;
	};
	const session = await page.request.post(`/api/v1/rooms/${room.code}/host-session`, {
		headers: { 'x-diceri-key': room.hostKey },
		data: { name: 'Probe', outputLanguage: 'en' }
	});
	test.skip(session.status() === 503, 'LiveKit is not configured (set DICERI_LIVEKIT_* in .env)');
	return username;
}

async function hostJoins(page: Page): Promise<string> {
	await openMyRoom(page);
	await page.getByLabel('Your name').fill('Host');
	await page.getByRole('button', { name: 'Join now' }).click();
	await expect(page.getByRole('button', { name: 'Leave call' })).toBeVisible({ timeout: 20_000 });
	return page.url();
}

async function guestAsks(browser: Browser, url: string): Promise<Page> {
	const context = await browser.newContext({ permissions: ['microphone'] });
	const guest = await context.newPage();
	await guest.goto(url);
	await guest.getByLabel('Your name').fill('Guest');
	await guest.getByRole('button', { name: 'Ask to join' }).click();
	await expect(guest.getByRole('heading', { name: 'Asking to be let in…' })).toBeVisible();
	return guest;
}

test('host admits a guest and both are in the call', async ({ page, browser }) => {
	await skipWithoutMedia(page);
	const guest = await guestAsks(browser, await hostJoins(page));

	await expect(page.getByText('Guest wants to join this call')).toBeVisible({ timeout: 10_000 });
	await page.getByRole('button', { name: 'Admit' }).click();

	await expect(guest.getByRole('button', { name: 'Leave call' })).toBeVisible({ timeout: 20_000 });
	await expect(page.getByRole('article', { name: 'Guest' })).toBeVisible({ timeout: 20_000 });
	await expect(guest.getByRole('article', { name: 'Host' })).toBeVisible();

	await page.getByRole('button', { name: 'Audio settings' }).click();
	const receiving = page
		.getByRole('row', { name: /Receiving/ })
		.getByRole('cell')
		.first();
	await expect
		.poll(async () => Number.parseInt((await receiving.textContent()) ?? '0', 10), {
			timeout: 20_000
		})
		.toBeGreaterThan(0);
});

test('host denies a guest', async ({ page, browser }) => {
	await skipWithoutMedia(page);
	const guest = await guestAsks(browser, await hostJoins(page));

	await page.getByRole('button', { name: 'Deny' }).click();
	await expect(guest.getByRole('heading', { name: "You can't join this meeting" })).toBeVisible({
		timeout: 10_000
	});
});

test('host can hear themselves through the server', async ({ page }) => {
	await skipWithoutMedia(page);
	await hostJoins(page);
	await page.getByRole('button', { name: 'Audio settings' }).click();
	await page.getByRole('radio', { name: 'Via server' }).click();
	await expect(page.getByRole('radio', { name: 'Via server' })).toHaveAttribute(
		'aria-checked',
		'true'
	);
});

test('anyone can walk into an open room at /@username', async ({ page, browser }) => {
	const owner = await skipWithoutMedia(page);
	await openMyRoom(page);
	// The host picks who gets in on the pre-join screen; the live room updates at once.
	await Promise.all([
		page.waitForResponse((r) => r.url().endsWith('/api/v1/rooms') && r.ok()),
		page.getByText('Anyone can join').click()
	]);
	await page.getByLabel('Your name').fill('Host');
	await page.getByRole('button', { name: 'Join now' }).click();
	await expect(page.getByRole('button', { name: 'Leave call' })).toBeVisible({ timeout: 20_000 });

	const visitor = await (await browser.newContext({ permissions: ['microphone'] })).newPage();
	await visitor.goto(`/@${owner}`);
	await expect(visitor.getByRole('button', { name: 'Join now' })).toBeVisible();
	await visitor.getByLabel('Your name').fill('Walk-in');
	await visitor.getByRole('button', { name: 'Join now' }).click();
	await expect(visitor.getByRole('button', { name: 'Leave call' })).toBeVisible({
		timeout: 20_000
	});
	await expect(page.getByRole('article', { name: 'Walk-in' })).toBeVisible({ timeout: 20_000 });
});
