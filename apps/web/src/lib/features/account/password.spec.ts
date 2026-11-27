import { describe, expect, it } from 'vitest';
import { passwordProblem } from './password.ts';

describe('password', () => {
	it('needs at least 8 characters', () => {
		expect(passwordProblem('short', '', 'ana')).toBe('too-short');
	});

	it('refuses a password that is easy to guess', () => {
		expect(passwordProblem('12345678', '12345678', 'ana')).toBe('too-easy');
	});

	it('needs both fields to match', () => {
		expect(passwordProblem('correct horse', 'correct house', 'ana')).toBe('mismatch');
	});

	it('accepts a long enough, repeated password', () => {
		expect(passwordProblem('correct horse', 'correct horse', 'ana')).toBeNull();
		expect(passwordProblem('correct horse', '', 'ana')).toBeNull();
	});
});
