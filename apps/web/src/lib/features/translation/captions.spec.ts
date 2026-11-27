import { describe, expect, it } from 'vitest';
import { isCaptionForListener, parseCaption, type CaptionMessage } from './captions.ts';

const encode = (value: unknown) => new TextEncoder().encode(JSON.stringify(value));

const caption = (kind: CaptionMessage['kind'], language: string): CaptionMessage => ({
	speaker: 'p-ana',
	language,
	kind,
	phrase: 0,
	text: 'x'
});

describe('parseCaption', () => {
	it('accepts the agent wire format', () => {
		const message = caption('translation', 'en');
		expect(parseCaption(encode(message))).toEqual(message);
	});

	it('rejects malformed payloads', () => {
		expect(parseCaption(new TextEncoder().encode('not json'))).toBeUndefined();
		expect(parseCaption(encode({ speaker: 'p', kind: 'other' }))).toBeUndefined();
	});
});

const listener = (outputLanguage: string) => ({ identity: 'me', outputLanguage });

describe('isCaptionForListener', () => {
	it('shows translations into the output language and hides other languages', () => {
		expect(isCaptionForListener(caption('translation', 'en'), listener('en'), 'pt')).toBe(true);
		expect(isCaptionForListener(caption('translation', 'es'), listener('en'), 'pt')).toBe(false);
		expect(isCaptionForListener(caption('original', 'pt'), listener('en'), 'pt')).toBe(false);
	});

	it('shows speech detected in the output language as said', () => {
		expect(isCaptionForListener(caption('original', 'pt'), listener('pt-BR'), 'pt')).toBe(true);
		expect(isCaptionForListener(caption('partial', 'pt-BR'), listener('pt-BR'), 'pt')).toBe(true);
		expect(isCaptionForListener(caption('partial', 'pt-BR'), listener('pt-BR'), 'en')).toBe(false);
	});

	it('never shows my own speech', () => {
		const own = { ...caption('translation', 'pt-BR'), speaker: 'me' };
		expect(isCaptionForListener(own, listener('pt-BR'), undefined)).toBe(false);
	});
});
