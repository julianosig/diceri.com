import { LevelMeter } from '#lib/features/call/index.ts';
import { ApiError } from '#lib/api/http-client.ts';
import { encodeWav } from '#lib/utils/wav.ts';
import { recordVoice, reuseKeptVoice } from './lobby-api.ts';

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
	/** The voice comes from the recording kept with the account, not a new one. */
	kept = $state(false);

	#stream: MediaStream | undefined;
	#recorder: MediaRecorder | undefined;
	#meter: LevelMeter | undefined;
	#chunks: Blob[] = [];
	#timer: ReturnType<typeof setInterval> | undefined;
	#frame = 0;
	#voicedMs = 0;
	/** Bumped when a recording starts or is discarded: late results of an older one are dropped. */
	#attempt = 0;

	constructor(private readonly roomId: string) {}

	get canStop(): boolean {
		return this.step === 'recording' && this.elapsed >= MIN_SECONDS;
	}

	async start(deviceId: string | undefined): Promise<void> {
		this.error = undefined;
		this.voiceId = undefined;
		const attempt = ++this.#attempt;
		let stream: MediaStream;
		try {
			stream = await navigator.mediaDevices.getUserMedia({
				audio: {
					deviceId: deviceId ? { exact: deviceId } : undefined,
					echoCancellation: true,
					noiseSuppression: true,
					autoGainControl: true
				}
			});
		} catch {
			if (attempt === this.#attempt) this.#fail('micNeeded');
			return;
		}
		// Left (or restarted) while the browser was asking for the microphone: never record.
		if (attempt !== this.#attempt) {
			for (const track of stream.getTracks()) track.stop();
			return;
		}
		this.#stream = stream;
		this.#record(stream);
	}

	stop(): void {
		if (this.#recorder?.state === 'recording') this.#recorder.stop();
	}

	/** Abandons a recording in progress (leaving the page): nothing is uploaded or kept. */
	discard(): void {
		this.#attempt += 1;
		if (this.#recorder) this.#recorder.onstop = null;
		this.stop();
		if (this.step === 'recording') {
			this.#cleanup();
			this.step = 'idle';
		}
	}

	reset(): void {
		this.#cleanup();
		this.step = 'idle';
		this.voiceId = undefined;
		this.kept = false;
		this.error = undefined;
	}

	/** Uses the recording kept with the account (ADR 0015); stays idle if there is none. */
	async useKept(): Promise<void> {
		if (this.step !== 'idle') return;
		this.step = 'creating';
		try {
			const { voiceId } = await reuseKeptVoice(this.roomId);
			this.voiceId = voiceId;
			this.kept = true;
			this.step = 'ready';
		} catch {
			// No kept recording (or it could not be cloned): record a new one as usual.
			this.step = 'idle';
		}
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
		const attempt = this.#attempt;
		this.step = 'creating';
		try {
			const wav = await toSpeechWav(sample);
			// Left the page while the recording was being converted: nothing is sent.
			if (attempt !== this.#attempt) return;
			const { voiceId } = await recordVoice(this.roomId, wav);
			this.voiceId = voiceId;
			this.kept = false;
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
