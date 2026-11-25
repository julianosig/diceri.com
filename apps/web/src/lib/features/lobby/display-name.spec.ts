import { describe, expect, it } from 'vitest';
import { isValidDisplayName } from './display-name.ts';

describe('isValidDisplayName', () => {
	it('accepts trimmed names up to 40 characters', () => {
		expect(isValidDisplayName('  Ana  ')).toBe(true);
		expect(isValidDisplayName('a'.repeat(40))).toBe(true);
	});

	it('rejects blank and overly long names', () => {
		expect(isValidDisplayName('   ')).toBe(false);
		expect(isValidDisplayName('a'.repeat(41))).toBe(false);
	});
});
