export { default as CaptionsOverlay } from './components/CaptionsOverlay.svelte';
export { CaptionFeed, type CaptionLine } from './captions.svelte.ts';
export { isCaptionForListener, parseCaption, type CaptionMessage } from './captions.ts';
export {
	AGENT_IDENTITY,
	CAPTIONS_TOPIC,
	DUCKED_ORIGINAL_VOLUME,
	originalVolume,
	OUTPUT_LANGUAGE_ATTRIBUTE,
	parseTranslatedTrack,
	PREVIEW_ATTRIBUTE,
	sameLanguage,
	wantsTranslatedTrack,
	type Listener,
	type TranslatedTrack
} from './translation-routing.ts';
export { LANGUAGES, languageLabel, matchBrowserLanguage } from './languages.ts';
