import type { CaptionMessage } from './captions.ts';

/** One phrase as said, with every translation produced for it. */
export interface TranscriptEntry {
	speaker: string;
	speakerName: string;
	phrase: number;
	/** Epoch milliseconds of the first caption received for the phrase. */
	at: number;
	original: { language: string; text: string } | undefined;
	/** Translated text by target language. */
	translations: Record<string, string>;
}

/**
 * Folds a caption into the transcript (partials are ignored). Returns a new
 * array; entries are ordered by when each phrase was first heard.
 */
export function applyCaption(
	entries: readonly TranscriptEntry[],
	message: CaptionMessage,
	speakerName: string,
	now: number
): TranscriptEntry[] {
	if (message.kind === 'partial') return [...entries];
	const index = entries.findIndex(
		(e) => e.speaker === message.speaker && e.phrase === message.phrase
	);
	const existing = entries[index] ?? {
		speaker: message.speaker,
		speakerName,
		phrase: message.phrase,
		at: now,
		original: undefined,
		translations: {}
	};
	const updated: TranscriptEntry =
		message.kind === 'original'
			? { ...existing, original: { language: message.language, text: message.text } }
			: {
					...existing,
					translations: { ...existing.translations, [message.language]: message.text }
				};
	return index === -1
		? [...entries, updated]
		: entries.map((entry, i) => (i === index ? updated : entry));
}

const time = (at: number) => new Date(at).toTimeString().slice(0, 8);

/** Plain-text transcript: one block per phrase, original first, then translations. */
export function formatTranscript(entries: readonly TranscriptEntry[]): string {
	return entries
		.map((entry) => {
			const said = entry.original
				? `(${entry.original.language}) ${entry.original.text}`
				: '(not captured)';
			const lines = [`[${time(entry.at)}] ${entry.speakerName} ${said}`];
			for (const [language, text] of Object.entries(entry.translations)) {
				lines.push(`    → ${language}: ${text}`);
			}
			return lines.join('\n');
		})
		.join('\n\n');
}
