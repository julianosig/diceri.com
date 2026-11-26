import { describe, expect, it } from 'vitest';
import { joinCopy } from './join-copy.ts';

describe('join copy', () => {
	it('lets hosts and guests of open rooms join directly', () => {
		expect(joinCopy(true, false).action).toBe('Join now');
		expect(joinCopy(false, true).action).toBe('Join now');
	});

	it('asks guests of approval rooms to wait for the host', () => {
		expect(joinCopy(false, false)).toMatchObject({ heading: 'Ask to join', action: 'Ask to join' });
	});
});
