import type { AudioCaptureOptions } from 'livekit-client';

/**
 * Enhanced (ML) noise filtering, applied on top of browser processing:
 * - `off`: browser processing only.
 * - `noise`: Krisp noise cancellation; removes non-speech background noise.
 * - `voice`: Krisp background voice cancellation; keeps only the main speaker.
 *   Billed separately by LiveKit Cloud.
 */
export type EnhancedFilter = 'off' | 'noise' | 'voice';

export interface AudioSettings {
	deviceId: string | undefined;
	echoCancellation: boolean;
	noiseSuppression: boolean;
	autoGainControl: boolean;
	voiceIsolation: boolean;
	enhancedFilter: EnhancedFilter;
}

export const DEFAULT_AUDIO_SETTINGS: AudioSettings = {
	deviceId: undefined,
	echoCancellation: true,
	noiseSuppression: true,
	autoGainControl: true,
	voiceIsolation: false,
	enhancedFilter: 'noise'
};

/**
 * Browser capture constraints. Krisp models are trained on raw audio, so the
 * browser's own noise suppression is disabled while an enhanced filter is on.
 */
export function toCaptureOptions(settings: AudioSettings): AudioCaptureOptions {
	const enhanced = settings.enhancedFilter !== 'off';
	return {
		deviceId: settings.deviceId,
		echoCancellation: settings.echoCancellation,
		noiseSuppression: enhanced ? false : settings.noiseSuppression,
		autoGainControl: settings.autoGainControl,
		voiceIsolation: enhanced ? false : settings.voiceIsolation
	};
}
