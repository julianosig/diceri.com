import { describe, expect, it } from 'vitest';
import { isCallProblem } from './call-problem.ts';

describe('call problems', () => {
	it('tells our problem keys from other error text', () => {
		expect(isCallProblem('filterFailed')).toBe(true);
		expect(isCallProblem('room is full')).toBe(false);
	});
});
