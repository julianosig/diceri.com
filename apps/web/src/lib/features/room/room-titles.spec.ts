import { describe, expect, it } from 'vitest';
import { isValidTitle, MAX_TITLE_LENGTH, suggestTitle } from './room-titles.ts';

describe('room titles', () => {
	it('suggests a valid title', () => {
		expect(isValidTitle(suggestTitle(() => 0))).toBe(true);
		expect(isValidTitle(suggestTitle(() => 0.999))).toBe(true);
	});

	it('needs text and stays within the limit', () => {
		expect(isValidTitle('   ')).toBe(false);
		expect(isValidTitle('a'.repeat(MAX_TITLE_LENGTH))).toBe(true);
		expect(isValidTitle('a'.repeat(MAX_TITLE_LENGTH + 1))).toBe(false);
	});
});
