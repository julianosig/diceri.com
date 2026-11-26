import { describe, expect, it } from 'vitest';
import { passwordProblem } from './password.ts';

describe('password', () => {
	it('needs at least 8 characters', () => {
		expect(passwordProblem('short', '')).toBe('Use at least 8 characters.');
	});

	it('needs both fields to match', () => {
		expect(passwordProblem('correct horse', 'correct house')).toBe('The passwords do not match.');
	});

	it('accepts a long enough, repeated password', () => {
		expect(passwordProblem('correct horse', 'correct horse')).toBeNull();
		expect(passwordProblem('correct horse', '')).toBeNull();
	});
});
