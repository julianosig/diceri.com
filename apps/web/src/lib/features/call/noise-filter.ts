import type { LocalAudioTrack } from 'livekit-client';
import type { EnhancedFilter } from './audio-settings.ts';
import type { CallProblem } from './call-problem.ts';

export interface FilterResult {
	active: EnhancedFilter;
	error?: CallProblem;
}

/**
 * Applies the Krisp filter to a published microphone track. Must run after
 * publishing: the processor authenticates against the LiveKit Cloud room.
 */
export async function applyEnhancedFilter(
	track: LocalAudioTrack,
	filter: EnhancedFilter
): Promise<FilterResult> {
	await track.stopProcessor();
	if (filter === 'off') return { active: 'off' };

	const { KrispNoiseFilter, isKrispNoiseFilterSupported } =
		await import('@livekit/krisp-noise-filter');
	if (!isKrispNoiseFilterSupported()) {
		return { active: 'off', error: 'filterUnsupported' };
	}
	try {
		await track.setProcessor(KrispNoiseFilter({ useBVC: filter === 'voice', quality: 'high' }));
		return { active: filter };
	} catch {
		return { active: 'off', error: 'filterFailed' };
	}
}

/** The audio actually sent to the room: the processed track when a filter runs. */
export function outgoingMediaTrack(track: LocalAudioTrack): MediaStreamTrack {
	return track.getProcessor()?.processedTrack ?? track.mediaStreamTrack;
}
