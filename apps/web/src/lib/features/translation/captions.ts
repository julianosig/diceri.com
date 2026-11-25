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
 * Whether a listener should see a caption: originals (and live partials) from
 * speakers of their language, translations into their language from others,
 * and their own voice translated into the language they are previewing.
 */
export function isCaptionForListener(
	message: CaptionMessage,
	listener: Listener,
	speakerLanguage: string | undefined
): boolean {
	if (message.speaker === listener.identity) {
		return message.kind === 'translation' && sameLanguage(message.language, listener.preview);
	}
	if (sameLanguage(speakerLanguage, listener.language)) return message.kind !== 'translation';
	return message.kind === 'translation' && sameLanguage(message.language, listener.language);
}
