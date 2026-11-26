import { describe, expect, it } from 'vitest';
import { fitFontSize, lineWidthEm } from './headline-fit.ts';

describe('headline fit', () => {
	it('counts CJK characters as full-width and others as narrow', () => {
		expect(lineWidthEm('ab')).toBeCloseTo(0.94);
		expect(lineWidthEm('元気')).toBe(2);
		expect(lineWidthEm('한국')).toBe(2);
	});

	it('sizes the text by its widest line, capped at the maximum', () => {
		const size = fitFontSize(['The Google Meet alternative', 'Talk and listen.'], 60, 768);
		const scale = (1 / lineWidthEm('The Google Meet alternative')).toFixed(4);
		expect(size).toBe(`min(60px, calc((min(100vw, 768px) - 2rem) * ${scale}))`);
	});
});
