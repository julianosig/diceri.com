import { expect, test, type Page } from '@playwright/test';

/** Creates a room through the API; skips the test when no media server is configured. */
async function createRoomWithMedia(page: Page): Promise<string> {
	const room = (await (await page.request.post('/api/v1/rooms')).json()) as { id: string };
	const join = await page.request.post(`/api/v1/rooms/${room.id}/join`);
	test.skip(join.status() === 503, 'LiveKit is not configured (set DICERI_LIVEKIT_* in .env)');
	return room.id;
}

/** Joins with the deterministic browser filter (Krisp needs real speech). */
async function joinCall(page: Page, roomId: string): Promise<void> {
	await page.goto(`/r/${roomId}`);
	await page.getByRole('radio', { name: 'Off' }).click();
	await page.getByRole('button', { name: 'Join with microphone' }).click();
	await expect(page.getByRole('heading', { name: 'Audio test' })).toBeVisible({ timeout: 20_000 });
}

function receivingBitrate(page: Page) {
	return page
		.getByRole('row', { name: /Receiving/ })
		.getByRole('cell')
		.first();
}

test('two participants connect and receive each other’s audio', async ({ page, browser }) => {
	const roomId = await createRoomWithMedia(page);
	const guestContext = await browser.newContext({ permissions: ['microphone'] });
	const guest = await guestContext.newPage();

	await joinCall(page, roomId);
	await joinCall(guest, roomId);

	await expect(page.getByText(/Guest · connection/)).toBeVisible({ timeout: 20_000 });
	await expect(guest.getByText(/Guest · connection/)).toBeVisible();
	await expect
		.poll(async () => Number.parseInt((await receivingBitrate(page).textContent()) ?? '0', 10), {
			timeout: 20_000
		})
		.toBeGreaterThan(0);

	await guestContext.close();
});

test('round-trip monitor connects without errors', async ({ page }) => {
	const roomId = await createRoomWithMedia(page);
	await joinCall(page, roomId);

	await page.getByRole('radio', { name: 'Via server' }).click();
	await expect(page.getByRole('radio', { name: 'Via server' })).toHaveAttribute(
		'aria-checked',
		'true'
	);
	await expect(page.getByRole('alert')).toHaveCount(0);
});
