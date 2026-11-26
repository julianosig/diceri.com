import { describe, expect, it } from 'vitest';
import { LANGUAGE_PAIRS, nextPair } from './language-pairs.ts';

describe('language pairs', () => {
	it('always translates into a different language', () => {
		for (const pair of LANGUAGE_PAIRS) {
			expect(pair.said.lang).not.toBe(pair.heard.lang);
		}
	});

	it('keeps lines short enough for the banner bubbles', () => {
		for (const { said, heard } of LANGUAGE_PAIRS) {
			expect(Array.from(said.text).length).toBeLessThanOrEqual(20);
			expect(Array.from(heard.text).length).toBeLessThanOrEqual(20);
		}
	});

	it('cycles through every pair and wraps around', () => {
		expect(nextPair(0)).toBe(1);
		expect(nextPair(LANGUAGE_PAIRS.length - 1)).toBe(0);
	});
});
