import { describe, expect, it } from 'vitest';
import { joinAction } from './join-copy.ts';

describe('join action', () => {
	it('lets hosts and guests of open rooms join directly', () => {
		expect(joinAction(true, false)).toBe('Join now');
		expect(joinAction(false, true)).toBe('Join now');
	});

	it('asks guests of approval rooms to wait for the host', () => {
		expect(joinAction(false, false)).toBe('Ask to join');
	});
});
