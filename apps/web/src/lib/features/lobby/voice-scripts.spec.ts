import { describe, expect, it } from 'vitest';
import { voiceScript } from './voice-scripts.ts';

describe('voiceScript', () => {
	it('uses the base language and starts with consent', () => {
		expect(voiceScript('pt-BR')).toMatch(/^Eu autorizo o Diceri/);
		expect(voiceScript('es')).toMatch(/^Autorizo a Diceri/);
	});

	it('falls back to English', () => {
		expect(voiceScript('ja')).toMatch(/^I agree to Diceri/);
	});
});
