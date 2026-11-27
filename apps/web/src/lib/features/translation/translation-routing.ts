/**
 * Contract with the media agent (apps/media-agent/src/track_names.rs) and the
 * rules deciding what each listener hears.
 */

export const AGENT_IDENTITY = 'diceri-translator';
export const CAPTIONS_TOPIC = 'diceri.captions';
/** Attribute with the language a participant hears everyone in (set by the API token). */
export const OUTPUT_LANGUAGE_ATTRIBUTE = 'output_language';
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

/**
 * Full volume while the speaker is detected speaking the listener's language
 * (or nothing was detected yet); ducked under the translation otherwise.
 */
export function originalVolume(detectedLanguage: string | undefined, listener: string): number {
	return detectedLanguage === undefined || sameLanguage(detectedLanguage, listener)
		? 1
		: DUCKED_ORIGINAL_VOLUME;
}

export interface Listener {
	identity: string;
	/** Language this listener hears everyone in. */
	outputLanguage: string;
}

/** Whether a listener plays a translated track: others translated into their language. */
export function wantsTranslatedTrack(track: TranslatedTrack, listener: Listener): boolean {
	return (
		track.speaker !== listener.identity && sameLanguage(track.language, listener.outputLanguage)
	);
}
