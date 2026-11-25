/** Raw cumulative counters sampled from a WebRTC sender or receiver. */
export interface StatsSample {
	timestamp: number;
	bytes: number;
	packets: number;
	packetsLost: number;
	jitterSeconds?: number;
	roundTripSeconds?: number;
}

/** Human-readable transport metrics between two samples. */
export interface TransportMetrics {
	bitrateKbps: number;
	lossPercent: number;
	jitterMs?: number;
	roundTripMs?: number;
}

export function computeMetrics(previous: StatsSample, current: StatsSample): TransportMetrics {
	const seconds = Math.max((current.timestamp - previous.timestamp) / 1000, 0.001);
	const bytes = Math.max(current.bytes - previous.bytes, 0);
	const packets = Math.max(current.packets - previous.packets, 0);
	const lost = Math.max(current.packetsLost - previous.packetsLost, 0);
	const total = packets + lost;
	return {
		bitrateKbps: Math.round((bytes * 8) / seconds / 1000),
		lossPercent: total === 0 ? 0 : Math.round((lost / total) * 1000) / 10,
		jitterMs: toMs(current.jitterSeconds),
		roundTripMs: toMs(current.roundTripSeconds)
	};
}

function toMs(seconds: number | undefined): number | undefined {
	return seconds === undefined ? undefined : Math.round(seconds * 1000);
}
