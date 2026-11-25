import { describe, expect, it } from 'vitest';
import { DEFAULT_AUDIO_SETTINGS, toCaptureOptions } from './audio-settings.ts';

describe('toCaptureOptions', () => {
	it('disables browser noise processing while an enhanced filter runs', () => {
		const options = toCaptureOptions({ ...DEFAULT_AUDIO_SETTINGS, enhancedFilter: 'voice' });
		expect(options.noiseSuppression).toBe(false);
		expect(options.voiceIsolation).toBe(false);
		expect(options.echoCancellation).toBe(true);
	});

	it('keeps browser noise suppression when enhanced filtering is off', () => {
		const options = toCaptureOptions({ ...DEFAULT_AUDIO_SETTINGS, enhancedFilter: 'off' });
		expect(options.noiseSuppression).toBe(true);
	});
});
