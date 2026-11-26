import { LevelMeter } from '#lib/features/call/index.ts';
import { ApiError } from '#lib/api/http-client.ts';
import { encodeWav } from '#lib/utils/wav.ts';
import { recordVoice } from './lobby-api.ts';

/** Recording length: long enough for a close clone, short enough to not get in the way. */
export const RECORD_SECONDS = 10;
/** Stopping earlier than this gives a poor clone. */
export const MIN_SECONDS = 5;
/** Meter level (normal speech is 0.3–0.8) from which the microphone is hearing a voice. */
const VOICE_LEVEL = 0.12;
/** Voice the recording needs before it is sent anywhere (the API checks again, for words). */
const MIN_VOICED_MS = 1500;
/** What speech-to-text reads, and what the API accepts for a voice. */
const SPEECH_SAMPLE_RATE = 16_000;

export type VoiceStep = 'idle' | 'recording' | 'creating' | 'ready' | 'error';
/** Why recording failed; the component words it in the visitor's language. */
export type VoiceProblem = 'noVoice' | 'micNeeded' | 'failed';

/** Records ~10 s of the user reading a script and turns it into a voice clone. */
export class VoiceRecorder {
	step = $state<VoiceStep>('idle');
	elapsed = $state(0);
	level = $state(0);
	error = $state<VoiceProblem | undefined>();
	voiceId = $state<string | undefined>();

	#stream: MediaStream | undefined;
	#recorder: MediaRecorder | undefined;
	#meter: LevelMeter | undefined;
	#chunks: Blob[] = [];
	#timer: ReturnType<typeof setInterval> | undefined;
	#frame = 0;
	#voicedMs = 0;

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
			this.#fail('micNeeded');
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
			// Nothing leaves the browser unless the microphone clearly heard a voice.
			if (this.#voicedMs < MIN_VOICED_MS) {
				this.#fail('noVoice');
				return;
			}
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
		this.#voicedMs = 0;
		let last = performance.now();
		const tick = () => {
			const now = performance.now();
			this.level = meter.read();
			if (this.level >= VOICE_LEVEL) this.#voicedMs += now - last;
			last = now;
			this.#frame = requestAnimationFrame(tick);
		};
		tick();
	}

	async #upload(sample: Blob): Promise<void> {
		this.#cleanup();
		this.step = 'creating';
		try {
			const { voiceId } = await recordVoice(this.roomId, await toSpeechWav(sample));
			this.voiceId = voiceId;
			this.step = 'ready';
		} catch (cause) {
			// 400: the API heard no words in it (or could not read it): read the sentence again.
			this.#fail(cause instanceof ApiError && cause.status === 400 ? 'noVoice' : 'failed');
		}
	}

	#fail(problem: VoiceProblem): void {
		this.#cleanup();
		this.error = problem;
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

/** The recording decoded and resampled to 16 kHz mono WAV, the format the API checks. */
async function toSpeechWav(recording: Blob): Promise<Blob> {
	const context = new AudioContext();
	try {
		const decoded = await context.decodeAudioData(await recording.arrayBuffer());
		const frames = Math.ceil(decoded.duration * SPEECH_SAMPLE_RATE);
		const offline = new OfflineAudioContext(1, frames, SPEECH_SAMPLE_RATE);
		const source = offline.createBufferSource();
		source.buffer = decoded;
		source.connect(offline.destination);
		source.start();
		const rendered = await offline.startRendering();
		return encodeWav(rendered.getChannelData(0), SPEECH_SAMPLE_RATE);
	} finally {
		await context.close();
	}
}
