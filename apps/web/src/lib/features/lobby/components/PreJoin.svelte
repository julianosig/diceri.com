<script lang="ts">
	import { onMount, untrack } from 'svelte';
	import { Hand, Languages, LoaderCircle, Mic, UserRound, Video } from '@lucide/svelte';
	import { Button } from '#lib/components/ui/index.ts';
	import { AudioMeter } from '#lib/features/call/index.ts';
	import { RoomEntryPicker, type RoomEntry } from '#lib/features/room/index.ts';
	import { isValidDisplayName, loadDisplayName, MAX_NAME_LENGTH } from '../display-name.ts';
	import { LANGUAGES } from '#lib/features/translation/index.ts';
	import { loadLanguage } from '../languages.ts';
	import type { Profile } from '../lobby-api.ts';
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
		onjoin: (profile: Profile, deviceId: string | undefined) => void;
	}

	let { roomId, entry, isHost, busy, error, onjoin }: Props = $props();
	const action = $derived(joinAction(isHost, entry === 'open'));

	const preview = new MicPreview();
	// svelte-ignore state_referenced_locally
	const recorder = new VoiceRecorder(roomId);
	let name = $state(loadDisplayName());
	let outputLanguage = $state(loadLanguage());
	const recording = $derived(recorder.step === 'recording' || recorder.step === 'creating');
	const canJoin = $derived(isValidDisplayName(name) && !busy && !recording);

	// Give the microphone back to the preview once a recording finishes.
	$effect(() => {
		if (recorder.step === 'ready' || recorder.step === 'error') {
			void preview.start(untrack(() => preview.deviceId));
		}
	});

	onMount(() => {
		void preview.start();
		return () => {
			void preview.stop();
		};
	});

	async function submit(event: SubmitEvent) {
		event.preventDefault();
		if (!canJoin) return;
		const deviceId = preview.deviceId;
		await preview.stop();
		onjoin({ name: name.trim(), outputLanguage, voiceId: recorder.voiceId }, deviceId);
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
	<label class="w-full text-start text-sm">
		<span class="mb-1 block text-ink-muted">Your name</span>
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
		<span class="mb-1 block text-ink-muted">Hear everyone in</span>
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
	<section class="flex w-full flex-col gap-3 text-start" aria-label="Microphone preview">
		<AudioMeter level={preview.level} muted={false} label="Microphone level" />
		<label class={[fieldClass, 'text-sm']}>
			<Mic class={iconClass} aria-hidden="true" />
			<span class="sr-only">Microphone</span>
			<select
				class="w-full select-field truncate bg-transparent py-2 text-base outline-none"
				value={preview.deviceId ?? ''}
				onchange={(event) => preview.start(event.currentTarget.value || undefined)}
			>
				<option value="">System default</option>
				{#each preview.devices as device (device.deviceId)}
					<option value={device.deviceId}>{device.label || 'Microphone'}</option>
				{/each}
			</select>
		</label>
		{#if preview.error}
			<p role="alert" class="text-sm text-red-600">{preview.error}</p>
		{/if}
	</section>
	<VoiceSetup
		{recorder}
		language={outputLanguage}
		deviceId={preview.deviceId}
		onbeforerecord={() => preview.stop()}
	/>
	<Button type="submit" size="lg" disabled={!canJoin}>
		{#if busy}
			<LoaderCircle class="size-5 animate-spin" aria-hidden="true" />
			Joining…
		{:else if action === 'Join now'}
			<Video class="size-5" aria-hidden="true" />
			{action}
		{:else}
			<Hand class="size-5" aria-hidden="true" />
			{action}
		{/if}
	</Button>
	{#if error}<p role="alert" class="text-sm text-red-600">{error}</p>{/if}
</form>
