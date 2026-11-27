import { describe, expect, it } from 'vitest';
import { voiceScript } from './voice-scripts.ts';

describe('voiceScript', () => {
	it('uses the base language', () => {
		expect(voiceScript('pt-BR')).toMatch(/^Olá! É um prazer/);
		expect(voiceScript('es')).toMatch(/^¡Hola! Es un placer/);
	});

	it('falls back to English', () => {
		expect(voiceScript('ja')).toMatch(/^Hi! It's great to meet you/);
	});

	it('holds no spoken consent: the terms of use cover keeping the recording', () => {
		for (const language of ['pt', 'en', 'es', 'fr', 'de', 'it']) {
			expect(voiceScript(language)).not.toMatch(/autoriz|agree|erlaube|consent/i);
		}
	});
});
