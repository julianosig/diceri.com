import { describe, expect, it } from 'vitest';
import { computeMetrics } from './transport-stats.ts';

const base = { timestamp: 0, bytes: 0, packets: 0, packetsLost: 0 };

describe('computeMetrics', () => {
	it('computes bitrate, loss, jitter and round trip between samples', () => {
		const metrics = computeMetrics(base, {
			timestamp: 1000,
			bytes: 4000,
			packets: 49,
			packetsLost: 1,
			jitterSeconds: 0.012,
			roundTripSeconds: 0.085
		});
		expect(metrics).toEqual({ bitrateKbps: 32, lossPercent: 2, jitterMs: 12, roundTripMs: 85 });
	});

	it('never reports negative values when counters reset', () => {
		const metrics = computeMetrics(
			{ ...base, bytes: 500, packets: 10 },
			{ ...base, timestamp: 1000 }
		);
		expect(metrics.bitrateKbps).toBe(0);
		expect(metrics.lossPercent).toBe(0);
	});
});
