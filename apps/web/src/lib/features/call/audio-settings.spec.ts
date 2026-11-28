import { describe, expect, it } from 'vitest';
import {
	captureConstraints,
	DEFAULT_AUDIO_SETTINGS,
	settingKeys,
	toCaptureOptions,
	valuesOf
} from './audio-settings.ts';

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

describe('settingKeys and valuesOf', () => {
	it('take back only the settings a change touched', () => {
		const patch = { deviceId: 'usb', echoCancellation: false };
		expect(settingKeys(patch)).toEqual(['deviceId', 'echoCancellation']);
		const applied = { ...DEFAULT_AUDIO_SETTINGS, deviceId: 'built-in' };
		expect(valuesOf(applied, ['deviceId'])).toEqual({ deviceId: 'built-in' });
	});
});

describe('captureConstraints', () => {
	it('keeps every capture setting but the device', () => {
		const settings = { ...DEFAULT_AUDIO_SETTINGS, deviceId: 'usb', echoCancellation: false };
		const constraints = captureConstraints(settings);
		expect(constraints.echoCancellation).toBe(false);
		expect('deviceId' in constraints).toBe(false);
	});
});
