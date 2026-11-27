import { describe, expect, it } from 'vitest';
import { ApiError } from '#lib/api/http-client.ts';
import { en } from '#lib/i18n/messages/en.ts';
import { limitText } from './limit-text.ts';

describe('limitText', () => {
	it('explains each limit the API can refuse with', () => {
		const daily = new ApiError(429, 'Too Many Requests', 'detail', 'daily');
		expect(limitText(daily, en)).toBe(en.limits.daily);
		const quota = new ApiError(429, 'Too Many Requests', 'detail', 'quota');
		expect(limitText(quota, en)).toBe(en.limits.quota);
		const busy = new ApiError(429, 'Too Many Requests', 'detail', 'busy');
		expect(limitText(busy, en)).toBe(en.limits.busy);
	});

	it('leaves other errors to the caller', () => {
		expect(limitText(new ApiError(500, 'Internal Server Error'), en)).toBeUndefined();
		expect(limitText(new ApiError(429, 'Too Many Requests', 'x', 'other'), en)).toBeUndefined();
		expect(limitText(new Error('network'), en)).toBeUndefined();
	});
});
