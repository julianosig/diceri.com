import { describe, expect, it } from 'vitest';
import { isValidUsername, normalizeUsername } from './username.ts';

describe('username', () => {
	it('normalizes like the server', () => {
		expect(normalizeUsername(' @Ana_92 ')).toBe('ana_92');
	});

	it('accepts 3–20 letters, numbers and _ starting with a letter', () => {
		expect(isValidUsername('ana')).toBe(true);
		expect(isValidUsername('ab')).toBe(false);
		expect(isValidUsername('9lives')).toBe(false);
		expect(isValidUsername('ana silva')).toBe(false);
		expect(isValidUsername('a'.repeat(21))).toBe(false);
	});
});
