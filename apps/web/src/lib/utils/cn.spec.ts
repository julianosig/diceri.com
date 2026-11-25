import { describe, expect, it } from 'vitest';
import { cn } from './cn.ts';

describe('cn', () => {
	it('joins truthy classes', () => {
		expect(cn('a', false, 'b')).toBe('a b');
	});

	it('lets later tailwind utilities win', () => {
		expect(cn('px-2', 'px-4')).toBe('px-4');
	});
});
