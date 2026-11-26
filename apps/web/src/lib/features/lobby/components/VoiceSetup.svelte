<script lang="ts">
	import { onDestroy } from 'svelte';
	import { Check, Mic, RotateCcw } from '@lucide/svelte';
	import { Button } from '#lib/components/ui/index.ts';
	import { AudioMeter } from '#lib/features/call/index.ts';
	import { MIN_SECONDS, RECORD_SECONDS, type VoiceRecorder } from '../voice-recorder.svelte.ts';
	import { voiceScript } from '../voice-scripts.ts';

	interface Props {
		recorder: VoiceRecorder;
		language: string;
		deviceId: string | undefined;
		/** Releases the preview microphone before recording. */
		onbeforerecord: () => Promise<void>;
	}

	let { recorder, language, deviceId, onbeforerecord }: Props = $props();
	const progress = $derived(Math.min(100, (recorder.elapsed / RECORD_SECONDS) * 100));
	const secondsLeft = $derived(Math.max(0, Math.ceil(RECORD_SECONDS - recorder.elapsed)));

	async function record() {
		await onbeforerecord();
		await recorder.start(deviceId);
	}

	onDestroy(() => {
		recorder.stop();
	});
</script>

<section
	class="flex w-full flex-col gap-3 rounded-tile border border-ink-muted/20 p-4 text-start"
	aria-label="Your voice"
	aria-live="polite"
>
	{#if recorder.step === 'recording'}
		<div class="flex items-center justify-between text-sm font-medium">
			<span class="flex items-center gap-2 text-red-600">
				<span class="size-2 animate-pulse rounded-full bg-red-600" aria-hidden="true"></span>
				Recording · read aloud
			</span>
			<span class="tabular-nums">{secondsLeft}s</span>
		</div>
		<p class="text-base leading-relaxed">{voiceScript(language)}</p>
		<div class="h-1.5 overflow-hidden rounded-full bg-surface-raised">
			<div class="h-full bg-red-600 transition-[width]" style:width="{progress}%"></div>
		</div>
		<AudioMeter level={recorder.level} muted={false} label="Your voice" />
		<Button
			size="sm"
			variant="secondary"
			disabled={!recorder.canStop}
			onclick={() => {
				recorder.stop();
			}}
		>
			{recorder.canStop ? 'Done' : `Keep reading (${String(MIN_SECONDS)}s min)`}
		</Button>
	{:else if recorder.step === 'creating'}
		<p class="flex items-center gap-2 text-sm">
			<span
				class="size-4 animate-spin rounded-full border-2 border-brand-500 border-t-transparent"
				aria-hidden="true"
			></span>
			Creating your voice…
		</p>
	{:else if recorder.step === 'ready'}
		<div class="flex items-center justify-between gap-2">
			<p class="flex items-center gap-2 text-sm font-medium text-green-700">
				<Check class="size-4" aria-hidden="true" /> Your voice is ready
			</p>
			<Button
				size="sm"
				variant="ghost"
				onclick={() => {
					recorder.reset();
				}}
			>
				<RotateCcw class="size-4" aria-hidden="true" /> Redo
			</Button>
		</div>
		<p class="text-xs text-ink-muted">Translations will sound like you. Deleted when you leave.</p>
	{:else}
		<p class="text-sm font-medium">Make translations sound like you</p>
		<p class="text-xs text-ink-muted">
			Read one short sentence for {RECORD_SECONDS} seconds. Optional, and deleted when you leave.
		</p>
		{#if recorder.error}<p role="alert" class="text-xs text-red-600">{recorder.error}</p>{/if}
		<Button size="sm" onclick={record}>
			<Mic class="size-4" aria-hidden="true" />
			{recorder.step === 'error' ? 'Try again' : `Record my voice (${String(RECORD_SECONDS)}s)`}
		</Button>
	{/if}
</section>
