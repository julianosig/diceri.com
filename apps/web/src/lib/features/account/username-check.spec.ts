import { afterEach, beforeEach, describe, expect, it, vi } from 'vitest';
import { UsernameCheck } from './username-check.svelte.ts';

describe('username check', () => {
	beforeEach(() => {
		vi.useFakeTimers();
	});
	afterEach(() => {
		vi.useRealTimers();
	});

	it('rejects malformed names without asking the server', () => {
		const check = vi.fn();
		const usernames = new UsernameCheck(check, 10);
		usernames.update('ab');
		expect(usernames.status).toBe('invalid');
		usernames.update('');
		expect(usernames.status).toBe('empty');
		expect(check).not.toHaveBeenCalled();
	});

	it('asks once after typing stops and reports availability', async () => {
		const check = vi.fn((username: string) =>
			Promise.resolve({ username, available: username !== 'taken' })
		);
		const usernames = new UsernameCheck(check, 10);
		usernames.update('an');
		usernames.update('@Ana');
		expect(usernames.status).toBe('checking');
		await vi.advanceTimersByTimeAsync(10);
		expect(check).toHaveBeenCalledTimes(1);
		expect(check).toHaveBeenCalledWith('ana');
		expect(usernames.status).toBe('available');

		usernames.update('taken');
		await vi.advanceTimersByTimeAsync(10);
		expect(usernames.status).toBe('taken');
	});

	it('ignores answers for names that are no longer typed', async () => {
		let answer: (value: { username: string; available: boolean }) => void = () => undefined;
		const check = vi.fn(
			() =>
				new Promise<{ username: string; available: boolean }>((resolve) => {
					answer = resolve;
				})
		);
		const usernames = new UsernameCheck(check, 10);
		usernames.update('first');
		await vi.advanceTimersByTimeAsync(10);
		usernames.update('ab');
		answer({ username: 'first', available: true });
		await Promise.resolve();
		expect(usernames.status).toBe('invalid');
	});
});
