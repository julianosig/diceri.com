import type {
	AudioReceiverStats,
	AudioSenderStats,
	LocalAudioTrack,
	RemoteAudioTrack
} from 'livekit-client';
import { computeMetrics, type StatsSample, type TransportMetrics } from './transport-stats.ts';

const INTERVAL_MS = 1000;

export interface CallMetrics {
	send?: TransportMetrics;
	receive?: TransportMetrics;
}

/** Polls WebRTC stats for the outgoing and (first) incoming audio track. */
export class StatsPoller {
	#timer: ReturnType<typeof setInterval> | undefined;
	#lastSend: StatsSample | undefined;
	#lastReceive: StatsSample | undefined;

	start(
		local: () => LocalAudioTrack | undefined,
		remote: () => RemoteAudioTrack | undefined,
		onMetrics: (metrics: CallMetrics) => void
	): void {
		this.stop();
		this.#timer = setInterval(() => {
			void this.#poll(local(), remote()).then(onMetrics);
		}, INTERVAL_MS);
	}

	stop(): void {
		clearInterval(this.#timer);
		this.#lastSend = undefined;
		this.#lastReceive = undefined;
	}

	async #poll(local?: LocalAudioTrack, remote?: RemoteAudioTrack): Promise<CallMetrics> {
		const [sent, received] = await Promise.all([
			local?.getSenderStats(),
			remote?.getReceiverStats()
		]);
		const send = sent && toSendSample(sent);
		const receive = received && toReceiveSample(received);
		const metrics: CallMetrics = {
			send: send && this.#lastSend ? computeMetrics(this.#lastSend, send) : undefined,
			receive: receive && this.#lastReceive ? computeMetrics(this.#lastReceive, receive) : undefined
		};
		this.#lastSend = send;
		this.#lastReceive = receive;
		return metrics;
	}
}

function toSendSample(stats: AudioSenderStats): StatsSample {
	return {
		timestamp: stats.timestamp,
		bytes: stats.bytesSent ?? 0,
		packets: stats.packetsSent ?? 0,
		packetsLost: stats.packetsLost ?? 0,
		jitterSeconds: stats.jitter,
		roundTripSeconds: stats.roundTripTime
	};
}

function toReceiveSample(stats: AudioReceiverStats): StatsSample {
	return {
		timestamp: stats.timestamp,
		bytes: stats.bytesReceived ?? 0,
		packets: stats.packetsReceived ?? 0,
		packetsLost: stats.packetsLost ?? 0,
		jitterSeconds: stats.jitter
	};
}
