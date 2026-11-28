import {
	createLocalAudioTrack,
	DisconnectReason,
	type ConnectionQuality,
	LocalAudioTrack,
	RemoteAudioTrack,
	Room,
	RoomEvent,
	Track,
	VideoPresets,
	type VideoTrack,
	type RemoteParticipant,
	type RemoteTrack,
	type RemoteTrackPublication
} from 'livekit-client';
import type { CallProblem } from './call-problem.ts';
import type { MediaCredentials } from '#lib/features/room/index.ts';
import {
	captureConstraints,
	DEFAULT_AUDIO_SETTINGS,
	settingKeys,
	valuesOf,
	toCaptureOptions,
	type AudioSettings,
	type EnhancedFilter
} from './audio-settings.ts';
import { cameraOf } from './camera-track.ts';
import { LevelMeter } from './level-meter.ts';
import { CallMonitor, type MonitorMode } from './call-monitor.svelte.ts';
import { applyEnhancedFilter, outgoingMediaTrack } from './noise-filter.ts';
import {
	AGENT_IDENTITY,
	CAPTIONS_TOPIC,
	CaptionFeed,
	OUTPUT_LANGUAGE_ATTRIBUTE,
	originalVolume,
	parseCaption,
	parseTranslatedTrack,
	TranscriptLog,
	wantsTranslatedTrack,
	type Listener
} from '#lib/features/translation/index.ts';
import { StatsPoller, type CallMetrics } from './stats/stats-poller.ts';

export type CallStatus = 'idle' | 'connecting' | 'connected' | 'error';
/** Hear yourself: off, local mic, through the server, or translated by the agent. */
export type { MonitorMode } from './call-monitor.svelte.ts';

export interface Participant {
	identity: string;
	name: string;
	speaking: boolean;
	muted: boolean;
	quality: ConnectionQuality;
	isLocal: boolean;
	/** Language the participant hears everyone in (BCP 47). */
	outputLanguage: string | undefined;
	/** Their camera, while it is on (and, for others, received). */
	video: VideoTrack | undefined;
}

export interface CallSessionOptions {
	credentials: MediaCredentials;
	/** Language this participant hears everyone in; what they speak is detected. */
	outputLanguage: string;
	deviceId?: string;
	/** Start with the camera on (chosen at pre-join); `deviceId` picks which one. */
	camera?: { deviceId?: string };
	/** Fetches hidden-listener credentials for the round-trip monitor. */
	monitorCredentials: () => Promise<MediaCredentials>;
	/** Called once when the host ends the meeting for everyone. */
	onEndedByHost?: () => void;
	/** The connection was lost for good (reconnecting gave up). */
	onDisconnected?: () => void;
}

/** One participant's live media session in a room. Create per call screen. */
export class CallSession {
	status = $state<CallStatus>('idle');
	error = $state<string | undefined>();
	settings = $state<AudioSettings>({ ...DEFAULT_AUDIO_SETTINGS });
	activeFilter = $state<EnhancedFilter>('off');
	devices = $state<MediaDeviceInfo[]>([]);
	muted = $state(false);
	cameraOn = $state(false);
	level = $state(0);
	participants = $state<Participant[]>([]);
	audioBlocked = $state(false);
	metrics = $state<CallMetrics>({});
	readonly captions: CaptionFeed;
	/** Everything said while connected, originals and translations (stays in this browser). */
	readonly transcript = new TranscriptLog();

	#room = new Room({ adaptiveStream: true, dynacast: true });
	// Bookkeeping only, never rendered, so it does not need to be reactive.
	// eslint-disable-next-line svelte/prefer-svelte-reactivity
	#remoteElements = new Map<RemoteTrack, HTMLMediaElement>();
	#meter: LevelMeter | undefined;
	#meterFrame = 0;
	readonly #monitor: CallMonitor;
	/** Set as soon as the call is being left: nothing may capture audio after it. */
	#closing = false;
	/** Read through a call: it changes while a change awaits, which narrowing cannot see. */
	#isClosing(): boolean {
		return this.#closing;
	}

	/** Numbers each settings change; `#owners` says which change last edited each setting. */
	#revision = 0;
	#owners: Partial<Record<keyof AudioSettings, number>> = {};
	/** The settings the published microphone really runs with. */
	#applied: AudioSettings = DEFAULT_AUDIO_SETTINGS;
	/** The settings change in progress; the next one waits for it. */
	#settingsChange: Promise<void> = Promise.resolve();
	#stats = new StatsPoller();

	constructor(private readonly options: CallSessionOptions) {
		this.settings.deviceId = options.deviceId;
		this.#monitor = new CallMonitor({
			identity: options.credentials.identity,
			credentials: options.monitorCredentials,
			isClosing: () => this.#closing,
			onFailed: () => (this.error = 'monitorFailed' satisfies CallProblem)
		});
		this.captions = new CaptionFeed(
			() => this.#listener(),
			(speaker) => {
				this.#updateOriginalVolume(speaker);
			}
		);
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
			if (this.options.camera) await this.setCamera(true);
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

	/**
	 * Applies new capture settings to the microphone, one change at a time. Each change is
	 * applied on top of the settings the microphone really has, and a change that fails only
	 * undoes itself, never a later one.
	 */
	async updateSettings(patch: Partial<AudioSettings>): Promise<void> {
		this.settings = { ...this.settings, ...patch };
		if (this.status !== 'connected') return;
		const revision = ++this.#revision;
		for (const key of settingKeys(patch)) this.#owners[key] = revision;
		const change = this.#settingsChange.then(() => this.#restartMicrophone(patch, revision));
		this.#settingsChange = change.catch(() => undefined);
		await change;
	}

	/**
	 * Restarts the published microphone with `patch` applied: the same publication, kept muted
	 * when the caller is (re-enabling it would only unmute it and ignore the new settings).
	 * A device that fails puts this change's settings back.
	 */
	async #restartMicrophone(patch: Partial<AudioSettings>, revision: number): Promise<void> {
		const track = this.#micTrack();
		if (!track) return;
		await this.#monitor.pause();
		// Left the call while the monitor closed: never open a microphone again.
		if (this.#isClosing()) return;
		const target = { ...this.#applied, ...patch };
		let problem: CallProblem | undefined;
		try {
			await restartWith(track, target);
			this.#applied = target;
		} catch {
			// Left the call meanwhile: never open a microphone again.
			if (this.#isClosing()) return;
			// Only settings this change is still the latest edit of go back; newer choices stay.
			const owned = settingKeys(patch).filter((key) => this.#owners[key] === revision);
			this.settings = { ...this.settings, ...valuesOf(this.#applied, owned) };
			// Browsers that stop the old microphone first leave nothing if both fail.
			problem = await restartWith(track, this.#applied).then(
				() => 'micChangeFailed' as const,
				() => 'micLost' as const
			);
		}
		if (this.#isClosing()) {
			track.stop();
			return;
		}
		const result = await applyEnhancedFilter(track, this.#applied.enhancedFilter);
		// Left while the filter started: no meter or monitor comes back.
		if (this.#isClosing()) return;
		this.activeFilter = result.active;
		this.error = problem ?? result.error;
		await this.#startMeter(outgoingMediaTrack(track));
		// The caller's latest monitor choice, even one made during the change.
		await this.#monitor.resume(outgoingMediaTrack(track));
	}

	/** A microphone track with the current settings, muted when the caller is. */
	async #capture(): Promise<LocalAudioTrack> {
		const captured = await createLocalAudioTrack(toCaptureOptions(this.settings));
		// Muted before it is published, so a muted caller never sends a single frame.
		if (this.muted) await captured.mute();
		return captured;
	}

	/** How the caller hears themselves (off, locally, or round-trip through the server). */
	get monitor(): MonitorMode {
		return this.#monitor.mode;
	}

	async setMonitor(mode: MonitorMode): Promise<void> {
		const track = this.#micTrack();
		await this.#monitor.set(mode, track && outgoingMediaTrack(track));
	}

	async toggleMute(): Promise<void> {
		const track = this.#micTrack();
		if (!track) return;
		if (this.muted) await track.unmute();
		else await track.mute();
		this.muted = !this.muted;
		this.#refreshParticipants();
	}

	/** Turns the camera on or off; a camera that cannot start stays off. */
	async setCamera(on: boolean): Promise<void> {
		try {
			await this.#room.localParticipant.setCameraEnabled(on, {
				deviceId: this.options.camera?.deviceId,
				resolution: VideoPresets.h540.resolution
			});
			this.cameraOn = on;
		} catch {
			this.cameraOn = false;
		}
		this.#refreshParticipants();
	}

	async startAudio(): Promise<void> {
		await this.#room.startAudio();
		this.audioBlocked = !this.#room.canPlaybackAudio;
	}

	async disconnect(): Promise<void> {
		this.#closing = true;
		this.#stats.stop();
		try {
			await this.setMonitor('off');
			await this.#stopMeter();
		} finally {
			// Leaving the room never depends on the extras closing cleanly.
			for (const element of this.#remoteElements.values()) element.remove();
			this.#remoteElements.clear();
			this.status = 'idle';
			this.participants = [];
			await this.#room.disconnect();
		}
	}

	async #publishMicrophone(): Promise<void> {
		const captured = await this.#capture();
		try {
			await this.#room.localParticipant.publishTrack(captured, {
				source: Track.Source.Microphone
			});
		} catch (cause) {
			// Never leave the microphone capturing for a track that is not in the call.
			captured.stop();
			throw cause;
		}
		this.#applied = this.settings;
		const track = this.#micTrack();
		// The message is a problem key; CallScreen words it in the visitor's language.
		if (!track) throw new Error('micUnpublished' satisfies CallProblem);
		const result = await applyEnhancedFilter(track, this.settings.enhancedFilter);
		this.activeFilter = result.active;
		this.error = result.error;
		if (this.muted) await track.mute();
		await this.#startMeter(outgoingMediaTrack(track));
	}

	async #startMeter(track: MediaStreamTrack): Promise<void> {
		await this.#stopMeter();
		if (this.#isClosing()) return;
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
		// Detached first, so two stops never close the same meter twice.
		const meter = this.#meter;
		this.#meter = undefined;
		this.level = 0;
		await meter?.close();
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
		const humans = [...this.#room.remoteParticipants.values()].filter(
			(participant) => participant.identity !== AGENT_IDENTITY
		);
		const all = [local, ...humans];
		this.participants = all.map((participant) => ({
			identity: participant.identity,
			name: participant.name || 'Guest',
			speaking: participant.isSpeaking,
			muted: participant === local ? this.muted : !participant.isMicrophoneEnabled,
			quality: participant.connectionQuality,
			isLocal: participant === local,
			outputLanguage:
				participant === local
					? this.options.outputLanguage
					: participant.attributes[OUTPUT_LANGUAGE_ATTRIBUTE],
			video: cameraOf(participant)
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
			.on(RoomEvent.ParticipantAttributesChanged, refresh)
			.on(RoomEvent.TrackSubscribed, refresh)
			.on(RoomEvent.TrackUnsubscribed, refresh)
			.on(RoomEvent.LocalTrackPublished, refresh)
			.on(RoomEvent.LocalTrackUnpublished, refresh)
			.on(
				RoomEvent.TrackSubscribed,
				(
					track: RemoteTrack,
					publication: RemoteTrackPublication,
					participant: RemoteParticipant
				) => {
					this.#onTrackSubscribed(track, publication, participant);
				}
			)
			.on(RoomEvent.DataReceived, (payload: Uint8Array, _participant, _kind, topic?: string) => {
				if (topic !== CAPTIONS_TOPIC) return;
				const caption = parseCaption(payload);
				if (!caption) return;
				this.captions.push(caption);
				this.transcript.record(caption, this.#nameOf(caption.speaker));
			})
			.on(RoomEvent.TrackPublished, (publication: RemoteTrackPublication) => {
				if (publication.trackName.startsWith('tr:') && !this.#wants(publication)) {
					publication.setSubscribed(false);
				}
			})
			.on(RoomEvent.TrackUnsubscribed, (track: RemoteTrack) => {
				this.#remoteElements.get(track)?.remove();
				this.#remoteElements.delete(track);
			})
			.on(RoomEvent.Disconnected, (reason?: DisconnectReason) => {
				if (reason === DisconnectReason.ROOM_DELETED) this.options.onEndedByHost?.();
				// Reconnecting gave up (or the server removed us): never leave a dead call on screen.
				else if (reason !== DisconnectReason.CLIENT_INITIATED) this.options.onDisconnected?.();
			})
			.on(RoomEvent.AudioPlaybackStatusChanged, () => {
				this.audioBlocked = !this.#room.canPlaybackAudio;
			});
	}

	/**
	 * Plays the translation into this listener's language and the original voice
	 * (ducked when it is another language); other translations are unsubscribed.
	 */
	#onTrackSubscribed(
		track: RemoteTrack,
		publication: RemoteTrackPublication,
		participant: RemoteParticipant
	): void {
		if (!(track instanceof RemoteAudioTrack)) return;
		if (participant.identity === AGENT_IDENTITY) {
			if (this.#wants(publication)) this.#attachRemote(track, 1);
			else publication.setSubscribed(false);
			return;
		}
		this.#attachRemote(
			track,
			originalVolume(
				this.captions.detectedLanguage(participant.identity),
				this.options.outputLanguage
			)
		);
	}

	#wants(publication: RemoteTrackPublication): boolean {
		const translated = parseTranslatedTrack(publication.trackName);
		return translated !== undefined && wantsTranslatedTrack(translated, this.#listener());
	}

	#listener(): Listener {
		return {
			identity: this.options.credentials.identity,
			outputLanguage: this.options.outputLanguage
		};
	}

	#nameOf(identity: string): string {
		if (identity === this.options.credentials.identity) {
			return this.#room.localParticipant.name || 'You';
		}
		return this.#room.remoteParticipants.get(identity)?.name || 'Guest';
	}

	/** Re-applies ducking when a speaker is detected in a new language. */
	#updateOriginalVolume(speaker: string): void {
		const track = this.#room.remoteParticipants
			.get(speaker)
			?.getTrackPublication(Track.Source.Microphone)?.track;
		if (!(track instanceof RemoteAudioTrack)) return;
		const detected = this.captions.detectedLanguage(speaker);
		track.setVolume(originalVolume(detected, this.options.outputLanguage));
	}

	#attachRemote(track: RemoteAudioTrack, volume: number): void {
		track.setVolume(volume);
		const element = track.attach();
		element.hidden = true;
		document.body.append(element);
		this.#remoteElements.set(track, element);
	}
}

/** Reopens the microphone with `settings`; LiveKit opens the default device with no other
 * constraints, so they are applied to it after it opens. */
async function restartWith(track: LocalAudioTrack, settings: AudioSettings): Promise<void> {
	await track.restartTrack(toCaptureOptions(settings));
	if (!settings.deviceId) await track.applyConstraints(captureConstraints(settings));
}
