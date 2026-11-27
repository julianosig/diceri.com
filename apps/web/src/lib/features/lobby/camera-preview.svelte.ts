/** Remembered between visits: whether the person joins with the camera on. */
const CAMERA_STORAGE_KEY = 'diceri-camera-on';

function loadCameraOn(): boolean {
	try {
		return localStorage.getItem(CAMERA_STORAGE_KEY) !== 'false';
	} catch {
		return true;
	}
}

/** Camera preview before joining: on/off, the device and a live picture. */
export class CameraPreview {
	/** Joins with the camera on (the default, like a meeting app). */
	on = $state(loadCameraOn());
	stream = $state.raw<MediaStream | undefined>();
	devices = $state<MediaDeviceInfo[]>([]);
	deviceId = $state<string | undefined>();
	/** Why the camera cannot be previewed; the component words it. */
	error = $state<'cameraBlocked' | 'cameraUnavailable' | undefined>();

	async start(deviceId?: string): Promise<void> {
		this.stop();
		this.deviceId = deviceId;
		if (!this.on) return;
		try {
			this.stream = await navigator.mediaDevices.getUserMedia({
				video: deviceId ? { deviceId: { exact: deviceId } } : true
			});
			const all = await navigator.mediaDevices.enumerateDevices();
			this.devices = all.filter((device) => device.kind === 'videoinput');
			this.error = undefined;
		} catch (cause) {
			this.error =
				cause instanceof DOMException && cause.name === 'NotAllowedError'
					? 'cameraBlocked'
					: 'cameraUnavailable';
		}
	}

	async toggle(): Promise<void> {
		this.on = !this.on;
		try {
			localStorage.setItem(CAMERA_STORAGE_KEY, String(this.on));
		} catch {
			// Storage blocked: the choice lasts for this visit.
		}
		if (this.on) await this.start(this.deviceId);
		else this.stop();
	}

	/** Whether the call should start with the camera (on, and it could be opened). */
	get joinsWithCamera(): boolean {
		return this.on && this.error === undefined;
	}

	/** Releases the camera so the call can capture it. */
	stop(): void {
		for (const track of this.stream?.getTracks() ?? []) track.stop();
		this.stream = undefined;
	}
}
