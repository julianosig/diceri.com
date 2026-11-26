import { describe, expect, it } from 'vitest';
import { LOCALES, direction, isLocale, matchLocale } from './locales.ts';
import { MESSAGE_LOADERS } from './message-loaders.ts';

describe('locales', () => {
	it('matches the browser preference on the primary subtag', () => {
		expect(matchLocale(['pt-BR', 'en-US'])).toBe('pt');
		expect(matchLocale(['zh-TW'])).toBe('zh');
		expect(matchLocale(['JA'])).toBe('ja');
	});

	it('takes the first supported preference and falls back to English', () => {
		expect(matchLocale(['nl-NL', 'de-AT', 'fr'])).toBe('de');
		expect(matchLocale(['nl', 'sv'])).toBe('en');
		expect(matchLocale([])).toBe('en');
	});

	it('knows which languages are written right to left', () => {
		expect(direction('ar')).toBe('rtl');
		expect(direction('pt')).toBe('ltr');
	});

	it('rejects unknown codes', () => {
		expect(isLocale('pt')).toBe(true);
		expect(isLocale('xx')).toBe(false);
		expect(isLocale(undefined)).toBe(false);
	});

	it('has messages for every language', async () => {
		for (const { code } of LOCALES) {
			const messages = await MESSAGE_LOADERS[code]();
			expect(messages.hero.promise.length, code).toBeGreaterThan(0);
			expect(Object.keys(messages.mascots.names), code).toHaveLength(12);
		}
	});
});
