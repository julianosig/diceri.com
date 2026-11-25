import { describe, expect, it } from 'vitest';
import { matchBrowserLanguage } from './languages.ts';

describe('matchBrowserLanguage', () => {
	it('prefers an exact match, then the base language', () => {
		expect(matchBrowserLanguage('pt-BR')).toBe('pt-BR');
		expect(matchBrowserLanguage('pt-PT')).toBe('pt-BR');
		expect(matchBrowserLanguage('es-MX')).toBe('es');
	});

	it('falls back to English', () => {
		expect(matchBrowserLanguage('xx')).toBe('en');
	});
});
