import { LevelMeter } from '#lib/features/call/index.ts';

/** Microphone preview before joining: permission, device list and level. */
export class MicPreview {
	level = $state(0);
	devices = $state<MediaDeviceInfo[]>([]);
	deviceId = $state<string | undefined>();
	error = $state<string | undefined>();
	#stream: MediaStream | undefined;
	#meter: LevelMeter | undefined;
	#frame = 0;

	async start(deviceId?: string): Promise<void> {
		await this.stop();
		this.deviceId = deviceId;
		try {
			this.#stream = await navigator.mediaDevices.getUserMedia({
				audio: { deviceId: deviceId ? { exact: deviceId } : undefined, echoCancellation: true }
			});
			const all = await navigator.mediaDevices.enumerateDevices();
			this.devices = all.filter((device) => device.kind === 'audioinput');
			const track = this.#stream.getAudioTracks().at(0);
			if (!track) throw new Error('No microphone track available.');
			const meter = new LevelMeter(track);
			this.#meter = meter;
			const tick = () => {
				this.level = meter.read();
				this.#frame = requestAnimationFrame(tick);
			};
			tick();
			this.error = undefined;
		} catch (cause) {
			this.error =
				cause instanceof DOMException && cause.name === 'NotAllowedError'
					? 'Microphone access was blocked. Allow it in the browser address bar.'
					: 'Could not access the microphone.';
		}
	}

	/** Releases the device so the call can capture it. */
	async stop(): Promise<void> {
		cancelAnimationFrame(this.#frame);
		await this.#meter?.close();
		for (const track of this.#stream?.getTracks() ?? []) track.stop();
		this.#meter = undefined;
		this.#stream = undefined;
		this.level = 0;
	}
}
