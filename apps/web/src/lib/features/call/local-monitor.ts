/**
 * Plays a local track back to the user (sidetone) so they can hear exactly
 * what is sent after noise processing. Use headphones to avoid feedback.
 */
export class LocalMonitor {
	#context = new AudioContext({ latencyHint: 'interactive' });
	#source: MediaStreamAudioSourceNode;

	constructor(track: MediaStreamTrack) {
		this.#source = this.#context.createMediaStreamSource(new MediaStream([track]));
		this.#source.connect(this.#context.destination);
	}

	async close(): Promise<void> {
		this.#source.disconnect();
		await this.#context.close();
	}
}
