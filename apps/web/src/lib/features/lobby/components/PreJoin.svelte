<script lang="ts">
	import { onMount, untrack } from 'svelte';
	import { Mic } from '@lucide/svelte';
	import { Button } from '#lib/components/ui/index.ts';
	import { AudioMeter } from '#lib/features/call/index.ts';
	import { MascotAvatar } from '#lib/features/room/index.ts';
	import { isValidDisplayName, loadDisplayName, MAX_NAME_LENGTH } from '../display-name.ts';
	import { LANGUAGES } from '#lib/features/translation/index.ts';
	import { loadLanguage } from '../languages.ts';
	import type { Profile } from '../lobby-api.ts';
	import { MicPreview } from '../mic-preview.svelte.ts';
	import { VoiceRecorder } from '../voice-recorder.svelte.ts';
	import { joinCopy } from '../join-copy.ts';
	import VoiceSetup from './VoiceSetup.svelte';

	interface Props {
		roomId: string;
		/** Username of the room's owner, which is also the room's name. */
		owner: string;
		/** The room's animated symbol. */
		mascot: string;
		/** Open rooms let guests straight in; others wait for the host. */
		open: boolean;
		isHost: boolean;
		busy: boolean;
		error?: string;
		onjoin: (profile: Profile, deviceId: string | undefined) => void;
	}

	let { roomId, owner, mascot, open, isHost, busy, error, onjoin }: Props = $props();
	const copy = $derived(joinCopy(isHost, open));

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
</script>

<div class="mx-auto grid w-full max-w-5xl items-center gap-10 px-4 md:grid-cols-[3fr_2fr]">
	<section
		class="relative flex aspect-video flex-col items-center justify-center gap-6 rounded-tile bg-stage p-6 text-white"
		aria-label="Microphone preview"
	>
		<div
			class="flex size-24 items-center justify-center rounded-full bg-brand-500 text-4xl font-medium"
		>
			{name.trim().charAt(0).toUpperCase() || '?'}
		</div>
		<div class="w-full max-w-xs">
			<AudioMeter level={preview.level} muted={false} label="Microphone level" tone="dark" />
		</div>
		<label class="flex w-full max-w-xs items-center gap-2 text-sm">
			<Mic class="size-4 shrink-0" aria-hidden="true" />
			<span class="sr-only">Microphone</span>
			<select
				class="w-full truncate rounded-lg bg-white/10 px-3 py-2"
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
			<p role="alert" class="text-sm text-red-300">{preview.error}</p>
		{/if}
	</section>

	<form class="flex flex-col items-center gap-5 text-center" onsubmit={submit}>
		<div class="flex flex-col items-center gap-1">
			<MascotAvatar {mascot} size={88} class="mb-1 shadow-md" />
			<p class="text-lg font-semibold">@{owner}</p>
			<p class="text-sm text-ink-muted">{isHost ? 'Your room' : `@${owner}'s room`}</p>
		</div>
		<h1 class="text-3xl font-normal">{copy.heading}</h1>
		<p class="text-sm text-ink-muted">{copy.hint}</p>
		<label class="w-full max-w-xs text-left text-sm">
			<span class="mb-1 block text-ink-muted">Your name</span>
			<input
				class="w-full rounded-lg border border-ink-muted/30 bg-surface px-3 py-2 text-base"
				bind:value={name}
				maxlength={MAX_NAME_LENGTH}
				autocomplete="name"
				required
			/>
		</label>
		<label class="w-full max-w-xs text-left text-sm">
			<span class="mb-1 block text-ink-muted">Hear everyone in</span>
			<select
				class="w-full rounded-lg border border-ink-muted/30 bg-surface px-3 py-2 text-base"
				bind:value={outputLanguage}
			>
				{#each LANGUAGES as option (option.code)}
					<option value={option.code}>{option.label}</option>
				{/each}
			</select>
		</label>
		<VoiceSetup
			{recorder}
			language={outputLanguage}
			deviceId={preview.deviceId}
			onbeforerecord={() => preview.stop()}
		/>
		<Button type="submit" size="lg" disabled={!canJoin}>
			{#if busy}Joining…{:else}{copy.action}{/if}
		</Button>
		{#if error}<p role="alert" class="text-sm text-red-600">{error}</p>{/if}
	</form>
</div>
