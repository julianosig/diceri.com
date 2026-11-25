<script lang="ts">
	import { onMount } from 'svelte';
	import { Mic } from '@lucide/svelte';
	import { Button } from '#lib/components/ui/index.ts';
	import { AudioMeter } from '#lib/features/call/index.ts';
	import { isValidDisplayName, loadDisplayName, MAX_NAME_LENGTH } from '../display-name.ts';
	import { LANGUAGES } from '#lib/features/translation/index.ts';
	import { loadLanguage } from '../languages.ts';
	import type { Profile } from '../lobby-api.ts';
	import { MicPreview } from '../mic-preview.svelte.ts';

	interface Props {
		isHost: boolean;
		busy: boolean;
		error?: string;
		onjoin: (profile: Profile, deviceId: string | undefined) => void;
	}

	let { isHost, busy, error, onjoin }: Props = $props();

	const preview = new MicPreview();
	let name = $state(loadDisplayName());
	let language = $state(loadLanguage());
	const canJoin = $derived(isValidDisplayName(name) && !busy);

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
		onjoin({ name: name.trim(), language }, deviceId);
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
		<h1 class="text-3xl font-normal">{isHost ? 'Ready to join?' : 'Ask to join'}</h1>
		<p class="text-sm text-ink-muted">
			{isHost ? 'You are the host of this meeting.' : 'The host will see your name and let you in.'}
		</p>
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
			<span class="mb-1 block text-ink-muted">I speak and want to hear</span>
			<select
				class="w-full rounded-lg border border-ink-muted/30 bg-surface px-3 py-2 text-base"
				bind:value={language}
			>
				{#each LANGUAGES as option (option.code)}
					<option value={option.code}>{option.label}</option>
				{/each}
			</select>
		</label>
		<Button type="submit" size="lg" disabled={!canJoin}>
			{#if busy}Joining…{:else}{isHost ? 'Join now' : 'Ask to join'}{/if}
		</Button>
		{#if error}<p role="alert" class="text-sm text-red-600">{error}</p>{/if}
	</form>
</div>
