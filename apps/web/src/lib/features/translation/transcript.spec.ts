import { describe, expect, it } from 'vitest';
import type { CaptionMessage } from './captions.ts';
import { applyCaption, formatTranscript } from './transcript.ts';

const message = (
	kind: CaptionMessage['kind'],
	language: string,
	text: string,
	from: { phrase?: number; speaker?: string } = {}
): CaptionMessage => ({
	speaker: from.speaker ?? 'p-ana',
	language,
	kind,
	phrase: from.phrase ?? 0,
	text
});

describe('applyCaption', () => {
	it('groups the original and its translations per phrase', () => {
		let entries = applyCaption([], message('original', 'pt', 'Olá'), 'Ana', 1);
		entries = applyCaption(entries, message('translation', 'en', 'Hello'), 'Ana', 2);
		entries = applyCaption(entries, message('translation', 'es', 'Hola'), 'Ana', 3);
		expect(entries).toEqual([
			{
				speaker: 'p-ana',
				speakerName: 'Ana',
				phrase: 0,
				at: 1,
				original: { language: 'pt', text: 'Olá' },
				translations: { en: 'Hello', es: 'Hola' }
			}
		]);
	});

	it('ignores partials and keeps phrases of different speakers apart', () => {
		let entries = applyCaption([], message('partial', 'pt', 'Ol'), 'Ana', 1);
		expect(entries).toEqual([]);
		entries = applyCaption(entries, message('original', 'pt', 'Oi'), 'Ana', 1);
		entries = applyCaption(
			entries,
			message('original', 'en', 'Hi', { speaker: 'p-bob' }),
			'Bob',
			2
		);
		expect(entries.map((e) => e.speakerName)).toEqual(['Ana', 'Bob']);
	});
});

describe('formatTranscript', () => {
	it('prints the original then each translation', () => {
		let entries = applyCaption([], message('original', 'pt', 'Olá'), 'Ana', 0);
		entries = applyCaption(entries, message('translation', 'en', 'Hello'), 'Ana', 0);
		const text = formatTranscript(entries);
		expect(text).toContain('Ana (pt) Olá');
		expect(text).toContain('→ en: Hello');
	});
});
