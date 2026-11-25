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

const listener = (language: string, preview?: string) => ({
	identity: 'me',
	language,
	preview
});

describe('isCaptionForListener', () => {
	it('shows translations into the listener language from other-language speakers', () => {
		expect(isCaptionForListener(caption('translation', 'en'), listener('en'), 'pt-BR')).toBe(true);
		expect(isCaptionForListener(caption('translation', 'es'), listener('en'), 'pt-BR')).toBe(false);
		expect(isCaptionForListener(caption('original', 'pt-BR'), listener('en'), 'pt-BR')).toBe(false);
	});

	it('shows originals from same-language speakers', () => {
		expect(isCaptionForListener(caption('original', 'pt-BR'), listener('pt-BR'), 'pt-BR')).toBe(
			true
		);
		expect(isCaptionForListener(caption('partial', 'pt-BR'), listener('pt-BR'), 'pt-BR')).toBe(
			true
		);
	});

	it('shows my own voice only when previewing that language', () => {
		const own = { ...caption('translation', 'ja'), speaker: 'me' };
		expect(isCaptionForListener(own, listener('pt-BR'), undefined)).toBe(false);
		expect(isCaptionForListener(own, listener('pt-BR', 'ja'), undefined)).toBe(true);
	});
});
