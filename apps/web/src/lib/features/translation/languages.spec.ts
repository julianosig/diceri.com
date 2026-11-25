import { describe, expect, it } from 'vitest';
import { languageLabel, matchBrowserLanguage } from './languages.ts';

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

describe('languageLabel', () => {
	it('labels detected base codes like the agent reports them', () => {
		expect(languageLabel('pt')).toBe('Português (Brasil)');
		expect(languageLabel('en')).toBe('English');
		expect(languageLabel('xx')).toBe('xx');
	});
});
