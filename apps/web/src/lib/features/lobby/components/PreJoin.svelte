<script lang="ts">
	import { onMount, untrack } from 'svelte';
	import { Hand, Languages, LoaderCircle, Mic, UserRound, Video } from '@lucide/svelte';
	import { Button } from '#lib/components/ui/index.ts';
	import { i18n } from '#lib/i18n/index.ts';
	import { AudioMeter } from '#lib/features/call/index.ts';
	import { RoomEntryPicker, type RoomEntry } from '#lib/features/room/index.ts';
	import { isValidDisplayName, loadDisplayName, MAX_NAME_LENGTH } from '../display-name.ts';
	import { LANGUAGES } from '#lib/features/translation/index.ts';
	import { loadLanguage } from '../languages.ts';
	import { currentAccount } from '#lib/features/account/index.ts';
	import { getCapabilities, type JoinDevices, type Profile } from '../lobby-api.ts';
	import { CameraPreview } from '../camera-preview.svelte.ts';
	import CameraSetup from './CameraSetup.svelte';
	import { MicPreview } from '../mic-preview.svelte.ts';
	import { VoiceRecorder } from '../voice-recorder.svelte.ts';
	import { joinAction } from '../join-copy.ts';
	import VoiceSetup from './VoiceSetup.svelte';

	interface Props {
		roomId: string;
		/** Open rooms let guests straight in; others wait for the host. */
		entry: RoomEntry;
		isHost: boolean;
		busy: boolean;
		error?: string;
		onjoin: (profile: Profile, devices: JoinDevices) => void;
	}

	let { roomId, entry, isHost, busy, error, onjoin }: Props = $props();
	const action = $derived(joinAction(isHost, entry === 'open'));

	const preview = new MicPreview();
	const camera = new CameraPreview();
	// svelte-ignore state_referenced_locally
	const recorder = new VoiceRecorder(roomId);
	let name = $state(loadDisplayName());
	let outputLanguage = $state(loadLanguage());
	const recording = $derived(recorder.step === 'recording' || recorder.step === 'creating');
	/** Everyone speaks in their own trained voice (ADR 0014): joining waits for it. */
	let voiceRequired = $state(false);
	const needsVoice = $derived(voiceRequired && recorder.voiceId === undefined);
	const canJoin = $derived(isValidDisplayName(name) && !busy && !recording && !needsVoice);

	// Give the microphone back to the preview once a recording finishes.
	$effect(() => {
		if (recorder.step === 'ready' || recorder.step === 'error') {
			void preview.start(untrack(() => preview.deviceId));
		}
	});

	/** Recording your own voice: offered only when this deployment has it. */
	let voiceCloning = $state(false);

	async function reuseKeptVoice(): Promise<void> {
		const account = await currentAccount().catch(() => null);
		if (account?.hasVoice) await recorder.useKept();
	}

	onMount(() => {
		getCapabilities()
			.then((capabilities) => {
				voiceCloning = capabilities.voiceCloning;
				voiceRequired = capabilities.voiceRequired;
				// A recording kept with the account is reused: no need to read again.
				if (capabilities.voiceCloning) void reuseKeptVoice();
			})
			.catch(() => {
				voiceCloning = false;
			});
		void preview.start();
		void camera.start();
		return () => {
			void preview.stop();
			camera.stop();
		};
	});

	async function submit(event: SubmitEvent) {
		event.preventDefault();
		if (!canJoin) return;
		const devices: JoinDevices = {
			microphone: preview.deviceId,
			camera: camera.joinsWithCamera ? { deviceId: camera.deviceId } : undefined
		};
		await preview.stop();
		camera.stop();
		onjoin({ name: name.trim(), outputLanguage, voiceId: recorder.voiceId }, devices);
	}

	/** A field: its icon and control share one bordered box. */
	const fieldClass =
		'flex items-center gap-2 rounded-lg border border-ink-muted/30 bg-surface ps-3 focus-within:border-brand-500';
	const iconClass = 'size-5 shrink-0 text-ink-muted';
</script>

<!-- One column: who and where, then name, language, microphone, voice, join. -->
<form
	class="mx-auto flex w-full max-w-md flex-col items-center gap-5 px-4 text-center"
	onsubmit={submit}
>
	<h1 class="w-full text-center text-3xl font-semibold tracking-tight">
		{isHost ? i18n.t.openRoom.submit : i18n.t.lobby.joinTitle}
	</h1>
	<CameraSetup {camera} />
	<label class="w-full text-start text-sm">
		<span class="mb-1 block text-ink-muted">{i18n.t.lobby.yourName}</span>
		<span class={fieldClass}>
			<UserRound class={iconClass} aria-hidden="true" />
			<input
				class="w-full bg-transparent py-2 pe-3 text-base outline-none"
				bind:value={name}
				maxlength={MAX_NAME_LENGTH}
				autocomplete="name"
				required
			/>
		</span>
	</label>
	<label class="w-full text-start text-sm">
		<span class="mb-1 block text-ink-muted">{i18n.t.lobby.hearEveryoneIn}</span>
		<span class={fieldClass}>
			<Languages class={iconClass} aria-hidden="true" />
			<select
				class="w-full select-field bg-transparent py-2 text-base outline-none"
				bind:value={outputLanguage}
			>
				{#each LANGUAGES as option (option.code)}
					<option value={option.code}>{option.label}</option>
				{/each}
			</select>
		</span>
	</label>
	{#if isHost}
		<RoomEntryPicker {entry} />
	{/if}
	<section
		class="flex w-full flex-col gap-2 text-start"
		aria-label={i18n.t.lobby.microphonePreview}
	>
		<label class="text-sm">
			<span class="mb-1 block text-ink-muted">{i18n.t.lobby.microphone}</span>
			<span class={fieldClass}>
				<Mic class={iconClass} aria-hidden="true" />
				<select
					class="w-full select-field truncate bg-transparent py-2 text-base outline-none"
					value={preview.deviceId ?? ''}
					onchange={(event) => preview.start(event.currentTarget.value || undefined)}
				>
					<option value="">{i18n.t.lobby.systemDefault}</option>
					{#each preview.devices as device (device.deviceId)}
						<option value={device.deviceId}>{device.label || i18n.t.lobby.microphone}</option>
					{/each}
				</select>
			</span>
		</label>
		<AudioMeter
			level={preview.level}
			muted={false}
			label={i18n.t.lobby.microphone}
			showLabel={false}
		/>
		{#if preview.error}
			<p role="alert" class="text-sm text-red-600">{i18n.t.lobby[preview.error]}</p>
		{/if}
	</section>
	<VoiceSetup
		available={voiceCloning}
		required={voiceRequired}
		{recorder}
		language={outputLanguage}
		deviceId={preview.deviceId}
		onbeforerecord={() => preview.stop()}
	/>
	<!-- Joining waits for the voice: until it is ready, recording is the only button. -->
	{#if !needsVoice}
		<Button type="submit" size="lg" disabled={!canJoin}>
			{#if busy}
				<LoaderCircle class="size-5 animate-spin" aria-hidden="true" />
				{i18n.t.lobby.joining}
			{:else if action === 'join'}
				<Video class="size-5" aria-hidden="true" />
				{i18n.t.lobby.joinNow}
			{:else}
				<Hand class="size-5" aria-hidden="true" />
				{i18n.t.lobby.askToJoin}
			{/if}
		</Button>
	{/if}
	{#if error}<p role="alert" class="text-sm text-red-600">{error}</p>{/if}
</form>
