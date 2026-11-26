import { expect, type Page } from '@playwright/test';

/** A username no other test run uses (≤ 20 chars, starts with a letter). */
export function uniqueUsername(): string {
	const random = Math.floor(Math.random() * 36 ** 3).toString(36);
	return `u${Date.now().toString(36)}${random}`;
}

/**
 * Signs the page's browser context up through the API; the session cookie lands
 * in the context, so the UI is signed in from then on.
 */
export async function signUp(page: Page): Promise<string> {
	const username = uniqueUsername();
	const response = await page.request.post('/api/v1/accounts', { data: { username } });
	expect(response.status()).toBe(201);
	return username;
}
