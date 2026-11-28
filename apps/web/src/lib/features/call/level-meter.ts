/** Measures the RMS level (0..1) of a media track with an AnalyserNode. */
export class LevelMeter {
	#context = new AudioContext();
	#analyser = this.#context.createAnalyser();
	#samples = new Float32Array(1024);
	#source: MediaStreamAudioSourceNode;

	constructor(track: MediaStreamTrack) {
		this.#analyser.fftSize = this.#samples.length;
		this.#source = this.#context.createMediaStreamSource(new MediaStream([track]));
		this.#source.connect(this.#analyser);
		// Contexts created outside a user gesture start suspended and read silence.
		void this.#context.resume();
	}

	/** Current level, scaled so normal speech lands around 0.3–0.8. */
	read(): number {
		this.#analyser.getFloatTimeDomainData(this.#samples);
		let sum = 0;
		for (const sample of this.#samples) sum += sample * sample;
		return Math.min(1, Math.sqrt(sum / this.#samples.length) * 4);
	}

	async close(): Promise<void> {
		this.#source.disconnect();
		await this.#context.close();
	}
}

/** Reports a track's level every frame until stopped; a stop never closes a meter twice. */
export class LiveLevel {
	#meter: LevelMeter | undefined;
	#frame = 0;

	constructor(private readonly report: (level: number) => void) {}

	/** Starts reading `track`; while `muted()` the level is 0. */
	start(track: MediaStreamTrack, muted: () => boolean): void {
		const meter = new LevelMeter(track);
		this.#meter = meter;
		const tick = () => {
			this.report(muted() ? 0 : meter.read());
			this.#frame = requestAnimationFrame(tick);
		};
		tick();
	}

	async stop(): Promise<void> {
		cancelAnimationFrame(this.#frame);
		// Detached first, so two stops never close the same meter twice.
		const meter = this.#meter;
		this.#meter = undefined;
		this.report(0);
		await meter?.close();
	}
}
