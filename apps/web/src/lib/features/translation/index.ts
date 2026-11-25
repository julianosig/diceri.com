export { default as CaptionsOverlay } from './components/CaptionsOverlay.svelte';
export { CaptionFeed, type CaptionLine } from './captions.svelte.ts';
export { isCaptionForListener, parseCaption, type CaptionMessage } from './captions.ts';
export {
	AGENT_IDENTITY,
	CAPTIONS_TOPIC,
	DUCKED_ORIGINAL_VOLUME,
	originalVolume,
	parseTranslatedTrack,
	sameLanguage
} from './translation-routing.ts';
