import {
	type ConnectionQuality,
	LocalAudioTrack,
	RemoteAudioTrack,
	Room,
	RoomEvent,
	Track,
	type RemoteTrack
} from 'livekit-client';
import type { MediaCredentials } from '#lib/features/room/index.ts';
import {
	DEFAULT_AUDIO_SETTINGS,
	toCaptureOptions,
	type AudioSettings,
	type EnhancedFilter
} from './audio-settings.ts';
import { LevelMeter } from './level-meter.ts';
import { LocalMonitor } from './local-monitor.ts';
import { LoopbackMonitor } from './loopback-monitor.ts';
import { applyEnhancedFilter, outgoingMediaTrack } from './noise-filter.ts';
import { StatsPoller, type CallMetrics } from './stats-poller.ts';

export type CallStatus = 'idle' | 'connecting' | 'connected' | 'error';
export type MonitorMode = 'off' | 'local' | 'loopback';

export interface Participant {
	identity: string;
	name: string;
	speaking: boolean;
	muted: boolean;
	quality: ConnectionQuality;
	isLocal: boolean;
}

export interface CallSessionOptions {
	credentials: MediaCredentials;
	deviceId?: string;
	/** Fetches hidden-listener credentials for the round-trip monitor. */
	monitorCredentials: () => Promise<MediaCredentials>;
}

/** One participant's live media session in a room. Create per call screen. */
export class CallSession {
	status = $state<CallStatus>('idle');
	error = $state<string | undefined>();
	settings = $state<AudioSettings>({ ...DEFAULT_AUDIO_SETTINGS });
	activeFilter = $state<EnhancedFilter>('off');
	devices = $state<MediaDeviceInfo[]>([]);
	muted = $state(false);
	level = $state(0);
	monitor = $state<MonitorMode>('off');
	participants = $state<Participant[]>([]);
	audioBlocked = $state(false);
	metrics = $state<CallMetrics>({});

	#room = new Room({ adaptiveStream: true, dynacast: true });
	// Bookkeeping only, never rendered, so it does not need to be reactive.
	// eslint-disable-next-line svelte/prefer-svelte-reactivity
	#remoteElements = new Map<RemoteTrack, HTMLMediaElement>();
	#meter: LevelMeter | undefined;
	#meterFrame = 0;
	#localMonitor: LocalMonitor | undefined;
	#loopback: LoopbackMonitor | undefined;
	#stats = new StatsPoller();

	constructor(private readonly options: CallSessionOptions) {
		this.settings.deviceId = options.deviceId;
		this.#bindRoomEvents();
	}

	async loadDevices(): Promise<void> {
		this.devices = await Room.getLocalDevices('audioinput', true);
	}

	async connect(): Promise<void> {
		this.status = 'connecting';
		this.error = undefined;
		try {
			const { serverUrl, token } = this.options.credentials;
			await this.#room.connect(serverUrl, token);
			await this.#publishMicrophone();
			void this.loadDevices();
			this.audioBlocked = !this.#room.canPlaybackAudio;
			this.#stats.start(
				() => this.#micTrack(),
				() => this.#firstRemoteAudio(),
				(metrics) => (this.metrics = metrics)
			);
			this.status = 'connected';
			this.#refreshParticipants();
		} catch (cause) {
			this.status = 'error';
			this.error = cause instanceof Error ? cause.message : String(cause);
		}
	}

	/** Applies new capture settings by republishing the microphone. */
	async updateSettings(patch: Partial<AudioSettings>): Promise<void> {
		this.settings = { ...this.settings, ...patch };
		if (this.status !== 'connected') return;
		const mode = this.monitor;
		await this.setMonitor('off');
		await this.#room.localParticipant.setMicrophoneEnabled(false);
		await this.#publishMicrophone();
		await this.setMonitor(mode);
	}

	async setMonitor(mode: MonitorMode): Promise<void> {
		await this.#localMonitor?.close();
		await this.#loopback?.stop();
		this.#localMonitor = undefined;
		this.#loopback = undefined;
		this.monitor = mode;
		const track = this.#micTrack();
		if (mode === 'local' && track) {
			this.#localMonitor = new LocalMonitor(outgoingMediaTrack(track));
		}
		if (mode === 'loopback') {
			this.#loopback = new LoopbackMonitor(this.options.credentials.identity);
			await this.#loopback.start(await this.options.monitorCredentials());
		}
	}

	async toggleMute(): Promise<void> {
		const track = this.#micTrack();
		if (!track) return;
		if (this.muted) await track.unmute();
		else await track.mute();
		this.muted = !this.muted;
		this.#refreshParticipants();
	}

	async startAudio(): Promise<void> {
		await this.#room.startAudio();
		this.audioBlocked = !this.#room.canPlaybackAudio;
	}

	async disconnect(): Promise<void> {
		this.#stats.stop();
		await this.setMonitor('off');
		await this.#stopMeter();
		for (const element of this.#remoteElements.values()) element.remove();
		this.#remoteElements.clear();
		await this.#room.disconnect();
		this.status = 'idle';
		this.participants = [];
	}

	async #publishMicrophone(): Promise<void> {
		await this.#room.localParticipant.setMicrophoneEnabled(true, toCaptureOptions(this.settings));
		const track = this.#micTrack();
		if (!track) throw new Error('Microphone could not be published.');
		const result = await applyEnhancedFilter(track, this.settings.enhancedFilter);
		this.activeFilter = result.active;
		this.error = result.error;
		if (this.muted) await track.mute();
		await this.#startMeter(outgoingMediaTrack(track));
	}

	async #startMeter(track: MediaStreamTrack): Promise<void> {
		await this.#stopMeter();
		const meter = new LevelMeter(track);
		this.#meter = meter;
		const tick = () => {
			this.level = this.muted ? 0 : meter.read();
			this.#meterFrame = requestAnimationFrame(tick);
		};
		tick();
	}

	async #stopMeter(): Promise<void> {
		cancelAnimationFrame(this.#meterFrame);
		await this.#meter?.close();
		this.#meter = undefined;
		this.level = 0;
	}

	#micTrack(): LocalAudioTrack | undefined {
		const publication = this.#room.localParticipant.getTrackPublication(Track.Source.Microphone);
		const track = publication?.track;
		return track instanceof LocalAudioTrack ? track : undefined;
	}

	#firstRemoteAudio(): RemoteAudioTrack | undefined {
		for (const participant of this.#room.remoteParticipants.values()) {
			const publication = participant.getTrackPublication(Track.Source.Microphone);
			if (publication?.track instanceof RemoteAudioTrack) return publication.track;
		}
		return undefined;
	}

	#refreshParticipants(): void {
		const local = this.#room.localParticipant;
		const all = [local, ...this.#room.remoteParticipants.values()];
		this.participants = all.map((participant) => ({
			identity: participant.identity,
			name: participant.name || 'Guest',
			speaking: participant.isSpeaking,
			muted: participant === local ? this.muted : !participant.isMicrophoneEnabled,
			quality: participant.connectionQuality,
			isLocal: participant === local
		}));
	}

	#bindRoomEvents(): void {
		const refresh = () => {
			this.#refreshParticipants();
		};
		this.#room
			.on(RoomEvent.ParticipantConnected, refresh)
			.on(RoomEvent.TrackMuted, refresh)
			.on(RoomEvent.TrackUnmuted, refresh)
			.on(RoomEvent.TrackPublished, refresh)
			.on(RoomEvent.ParticipantDisconnected, refresh)
			.on(RoomEvent.ActiveSpeakersChanged, refresh)
			.on(RoomEvent.ConnectionQualityChanged, refresh)
			.on(RoomEvent.TrackSubscribed, (track: RemoteTrack) => {
				this.#attachRemote(track);
			})
			.on(RoomEvent.TrackUnsubscribed, (track: RemoteTrack) => {
				this.#remoteElements.get(track)?.remove();
				this.#remoteElements.delete(track);
			})
			.on(RoomEvent.AudioPlaybackStatusChanged, () => {
				this.audioBlocked = !this.#room.canPlaybackAudio;
			});
	}

	#attachRemote(track: RemoteTrack): void {
		if (track.kind !== Track.Kind.Audio) return;
		const element = track.attach();
		element.hidden = true;
		document.body.append(element);
		this.#remoteElements.set(track, element);
	}
}
