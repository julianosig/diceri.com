import { describe, expect, it } from 'vitest';
import {
	DUCKED_ORIGINAL_VOLUME,
	originalVolume,
	parseTranslatedTrack,
	sameLanguage
} from './translation-routing.ts';

describe('parseTranslatedTrack', () => {
	it('reads speaker and language', () => {
		expect(parseTranslatedTrack('tr:p-abc:pt-BR')).toEqual({ speaker: 'p-abc', language: 'pt-BR' });
		expect(parseTranslatedTrack('tr:p-abc:en')).toEqual({ speaker: 'p-abc', language: 'en' });
	});

	it('ignores other track names', () => {
		expect(parseTranslatedTrack('microphone')).toBeUndefined();
		expect(parseTranslatedTrack('tr:p-abc:english')).toBeUndefined();
	});
});

describe('language rules', () => {
	it('matches on the base language', () => {
		expect(sameLanguage('pt-BR', 'pt')).toBe(true);
		expect(sameLanguage('en', 'es')).toBe(false);
	});

	it('ducks the original voice only for other languages', () => {
		expect(originalVolume('pt-BR', 'pt-BR')).toBe(1);
		expect(originalVolume('en', 'pt-BR')).toBe(DUCKED_ORIGINAL_VOLUME);
		expect(originalVolume(undefined, 'pt-BR')).toBe(1);
	});
});
