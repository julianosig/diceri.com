import { LevelMeter } from '#lib/features/call/index.ts';
import { recordVoice } from './lobby-api.ts';

/** Recording length: long enough for a close clone, short enough to not get in the way. */
export const RECORD_SECONDS = 10;
/** Stopping earlier than this gives a poor clone. */
export const MIN_SECONDS = 5;

export type VoiceStep = 'idle' | 'recording' | 'creating' | 'ready' | 'error';

/** Records ~10 s of the user reading a script and turns it into a voice clone. */
export class VoiceRecorder {
	step = $state<VoiceStep>('idle');
	elapsed = $state(0);
	level = $state(0);
	error = $state<string | undefined>();
	voiceId = $state<string | undefined>();

	#stream: MediaStream | undefined;
	#recorder: MediaRecorder | undefined;
	#meter: LevelMeter | undefined;
	#chunks: Blob[] = [];
	#timer: ReturnType<typeof setInterval> | undefined;
	#frame = 0;

	constructor(private readonly roomId: string) {}

	get canStop(): boolean {
		return this.step === 'recording' && this.elapsed >= MIN_SECONDS;
	}

	async start(deviceId: string | undefined): Promise<void> {
		this.error = undefined;
		this.voiceId = undefined;
		try {
			this.#stream = await navigator.mediaDevices.getUserMedia({
				audio: {
					deviceId: deviceId ? { exact: deviceId } : undefined,
					echoCancellation: true,
					noiseSuppression: true,
					autoGainControl: true
				}
			});
		} catch {
			this.#fail('Microphone access is needed to record your voice.');
			return;
		}
		this.#record(this.#stream);
	}

	stop(): void {
		if (this.#recorder?.state === 'recording') this.#recorder.stop();
	}

	reset(): void {
		this.#cleanup();
		this.step = 'idle';
		this.voiceId = undefined;
		this.error = undefined;
	}

	#record(stream: MediaStream): void {
		this.#chunks = [];
		const recorder = new MediaRecorder(stream, { mimeType: pickMimeType() });
		this.#recorder = recorder;
		recorder.ondataavailable = (event) => {
			if (event.data.size > 0) this.#chunks.push(event.data);
		};
		recorder.onstop = () => {
			void this.#upload(new Blob(this.#chunks, { type: recorder.mimeType }));
		};
		recorder.start(250);
		this.step = 'recording';
		this.elapsed = 0;
		const started = performance.now();
		this.#timer = setInterval(() => {
			this.elapsed = (performance.now() - started) / 1000;
			if (this.elapsed >= RECORD_SECONDS) this.stop();
		}, 100);
		const track = stream.getAudioTracks().at(0);
		if (track) this.#startMeter(track);
	}

	#startMeter(track: MediaStreamTrack): void {
		const meter = new LevelMeter(track);
		this.#meter = meter;
		const tick = () => {
			this.level = meter.read();
			this.#frame = requestAnimationFrame(tick);
		};
		tick();
	}

	async #upload(sample: Blob): Promise<void> {
		this.#cleanup();
		this.step = 'creating';
		try {
			const { voiceId } = await recordVoice(this.roomId, sample);
			this.voiceId = voiceId;
			this.step = 'ready';
		} catch (cause) {
			this.#fail(cause instanceof Error ? cause.message : 'Could not create your voice.');
		}
	}

	#fail(message: string): void {
		this.#cleanup();
		this.error = message;
		this.step = 'error';
	}

	#cleanup(): void {
		clearInterval(this.#timer);
		cancelAnimationFrame(this.#frame);
		void this.#meter?.close();
		this.#meter = undefined;
		for (const track of this.#stream?.getTracks() ?? []) track.stop();
		this.#stream = undefined;
		this.level = 0;
	}
}

function pickMimeType(): string {
	const preferred = ['audio/webm;codecs=opus', 'audio/webm', 'audio/mp4'];
	return preferred.find((type) => MediaRecorder.isTypeSupported(type)) ?? '';
}
