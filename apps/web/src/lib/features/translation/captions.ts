import { sameLanguage, type Listener } from './translation-routing.ts';

/** Wire format published by the media agent (apps/media-agent/src/output.rs). */
export interface CaptionMessage {
	speaker: string;
	language: string;
	kind: 'partial' | 'original' | 'translation';
	phrase: number;
	text: string;
}

const KINDS = new Set(['partial', 'original', 'translation']);

export function parseCaption(payload: Uint8Array): CaptionMessage | undefined {
	try {
		const value = JSON.parse(new TextDecoder().decode(payload)) as Partial<CaptionMessage>;
		const valid =
			typeof value.speaker === 'string' &&
			typeof value.language === 'string' &&
			typeof value.text === 'string' &&
			typeof value.phrase === 'number' &&
			KINDS.has(value.kind ?? '');
		return valid ? (value as CaptionMessage) : undefined;
	} catch {
		return undefined;
	}
}

/**
 * Whether a listener should see a caption: speech detected in their output
 * language as said, everything else as a translation into it, and, while
 * previewing, their own recognized text and its translation.
 * `speakerLanguage` is the language last detected for the speaker (for partials).
 */
export function isCaptionForListener(
	message: CaptionMessage,
	listener: Listener,
	speakerLanguage: string | undefined
): boolean {
	if (message.speaker === listener.identity) {
		if (listener.preview === undefined) return false;
		if (message.kind === 'original') return true;
		return message.kind === 'translation' && sameLanguage(message.language, listener.preview);
	}
	if (message.kind === 'partial') return sameLanguage(speakerLanguage, listener.outputLanguage);
	return sameLanguage(message.language, listener.outputLanguage);
}
