import { page } from 'vitest/browser';
import { afterEach, describe, expect, it } from 'vitest';
import { render } from 'vitest-browser-svelte';
import { THEME_STORAGE_KEY, theme } from './theme.svelte.ts';
import ThemeToggle from './ThemeToggle.svelte';

describe('ThemeToggle', () => {
	afterEach(() => {
		theme.set('light');
		localStorage.removeItem(THEME_STORAGE_KEY);
	});

	it('switches the whole page between light and dark and remembers it', async () => {
		theme.set('light');
		await render(ThemeToggle);

		await page.getByRole('button', { name: 'Switch to dark theme' }).click();
		expect(document.documentElement.classList.contains('dark')).toBe(true);
		expect(localStorage.getItem(THEME_STORAGE_KEY)).toBe('dark');

		await page.getByRole('button', { name: 'Switch to light theme' }).click();
		expect(document.documentElement.classList.contains('dark')).toBe(false);
		expect(localStorage.getItem(THEME_STORAGE_KEY)).toBe('light');
	});
});
