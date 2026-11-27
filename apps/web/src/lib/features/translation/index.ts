export { default as CaptionsOverlay } from './components/CaptionsOverlay.svelte';
export { default as TranscriptView } from './components/TranscriptView.svelte';
export { TranscriptLog } from './transcript.svelte.ts';
export type { TranscriptEntry } from './transcript.ts';
export { CaptionFeed, type CaptionLine } from './captions.svelte.ts';
export { isCaptionForListener, parseCaption, type CaptionMessage } from './captions.ts';
export {
	AGENT_IDENTITY,
	CAPTIONS_TOPIC,
	DUCKED_ORIGINAL_VOLUME,
	originalVolume,
	OUTPUT_LANGUAGE_ATTRIBUTE,
	parseTranslatedTrack,
	wantsTranslatedTrack,
	type Listener,
	type TranslatedTrack
} from './translation-routing.ts';
export { LANGUAGES, languageLabel, matchBrowserLanguage } from './languages.ts';
