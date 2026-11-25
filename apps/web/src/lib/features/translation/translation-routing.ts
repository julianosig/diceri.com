/**
 * Contract with the media agent (apps/media-agent/src/track_names.rs) and the
 * rules deciding what each listener hears.
 */

export const AGENT_IDENTITY = 'diceri-translator';
export const CAPTIONS_TOPIC = 'diceri.captions';
/** Attribute asking the agent to translate this speaker for themselves (apps/media-agent/src/targets.rs). */
export const PREVIEW_ATTRIBUTE = 'preview_language';
/** Volume of the original voice under a translation, like an interpreter booth. */
export const DUCKED_ORIGINAL_VOLUME = 0.15;

export interface TranslatedTrack {
	speaker: string;
	language: string;
}

/** Parses `tr:<speaker identity>:<language>`; anything else is not a translation. */
export function parseTranslatedTrack(name: string): TranslatedTrack | undefined {
	const match = /^tr:(.+):([A-Za-z]{2,3}(?:-[A-Z]{2})?)$/.exec(name);
	if (!match?.[1] || !match[2]) return undefined;
	return { speaker: match[1], language: match[2] };
}

/** Same base language (`pt-BR` and `pt` match): no translation needed. */
export function sameLanguage(a: string | undefined, b: string | undefined): boolean {
	if (!a || !b) return false;
	return a.split('-')[0]?.toLowerCase() === b.split('-')[0]?.toLowerCase();
}

/** Full volume when the listener understands the speaker; ducked otherwise. */
export function originalVolume(speakerLanguage: string | undefined, listener: string): number {
	return speakerLanguage === undefined || sameLanguage(speakerLanguage, listener)
		? 1
		: DUCKED_ORIGINAL_VOLUME;
}

export interface Listener {
	identity: string;
	language: string;
	/** Language this listener previews their own voice in, if any. */
	preview: string | undefined;
}

/**
 * Whether a listener plays a translated track: translations of others into their
 * language, or their own voice in the language they are previewing.
 */
export function wantsTranslatedTrack(track: TranslatedTrack, listener: Listener): boolean {
	if (track.speaker === listener.identity) {
		return listener.preview !== undefined && sameLanguage(track.language, listener.preview);
	}
	return sameLanguage(track.language, listener.language);
}
