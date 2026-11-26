import { describe, expect, it } from 'vitest';
import { passwordProblem } from './password.ts';

describe('password', () => {
	it('needs at least 8 characters', () => {
		expect(passwordProblem('short', '')).toBe('too-short');
	});

	it('needs both fields to match', () => {
		expect(passwordProblem('correct horse', 'correct house')).toBe('mismatch');
	});

	it('accepts a long enough, repeated password', () => {
		expect(passwordProblem('correct horse', 'correct horse')).toBeNull();
		expect(passwordProblem('correct horse', '')).toBeNull();
	});
});
