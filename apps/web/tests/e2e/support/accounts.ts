import { expect, type Page } from '@playwright/test';

/** The password every e2e account uses. */
export const PASSWORD = 'correct horse battery';

/** A username no other test run uses (≤ 20 chars, starts with a letter). */
export function uniqueUsername(): string {
	const random = Math.floor(Math.random() * 36 ** 3).toString(36);
	return `u${Date.now().toString(36)}${random}`;
}

/**
 * Signs the page's browser context up through the API (dragon mascot); the session
 * cookie lands in the context, so the UI is signed in from then on.
 */
export async function signUp(page: Page): Promise<string> {
	const username = uniqueUsername();
	const response = await page.request.post('/api/v1/accounts', {
		data: { username, password: PASSWORD, mascot: 'dragon' }
	});
	expect(response.status()).toBe(201);
	return username;
}

/** From the home page, opens the signed-in person's room and lands on `/@username`. */
export async function openMyRoom(page: Page): Promise<void> {
	await page.goto('/');
	await page.getByRole('button', { name: 'Open my room' }).click();
	await expect(page).toHaveURL(/\/@[a-z][a-z0-9_]+$/);
}
