<script lang="ts">
	import { onDestroy } from 'svelte';
	import { Check, Mic, RotateCcw } from '@lucide/svelte';
	import { Button } from '#lib/components/ui/index.ts';
	import { i18n } from '#lib/i18n/index.ts';
	import { AudioMeter } from '#lib/features/call/index.ts';
	import { MIN_SECONDS, RECORD_SECONDS, type VoiceRecorder } from '../voice-recorder.svelte.ts';
	import { voiceScript } from '../voice-scripts.ts';

	interface Props {
		recorder: VoiceRecorder;
		language: string;
		deviceId: string | undefined;
		/** Releases the preview microphone before recording. */
		onbeforerecord: () => Promise<void>;
		/** Whether this deployment offers it. */
		available: boolean;
		/** Joining needs it: everyone speaks in their own voice. */
		required?: boolean;
	}

	let {
		recorder,
		language,
		deviceId,
		onbeforerecord,
		available,
		required = false
	}: Props = $props();
	const progress = $derived(Math.min(100, (recorder.elapsed / RECORD_SECONDS) * 100));
	const secondsLeft = $derived(Math.max(0, Math.ceil(RECORD_SECONDS - recorder.elapsed)));

	const t = $derived(i18n.t.voice);

	async function record() {
		await onbeforerecord();
		await recorder.start(deviceId);
	}

	onDestroy(() => {
		recorder.stop();
	});
</script>

<section class="flex w-full flex-col gap-3 text-start" aria-label={t.section} aria-live="polite">
	{#if !available}
		<!-- Shown but switched off: it will be a paid feature. -->
		<div class="flex items-center justify-between gap-2">
			<p class="text-sm font-medium text-ink-muted">{t.title}</p>
			<span class="rounded-full bg-brand-500/15 px-2 py-0.5 text-xs font-semibold text-brand-500">
				{t.pro}
			</span>
		</div>
		<p class="text-xs text-ink-muted">{t.paidSoon}</p>
	{:else if recorder.step === 'recording'}
		<div class="flex items-center justify-between text-sm font-medium">
			<span class="flex items-center gap-2 text-red-600">
				<span class="size-2 animate-pulse rounded-full bg-red-600" aria-hidden="true"></span>
				{t.recording}
			</span>
			<span class="tabular-nums">{t.secondsLeft(secondsLeft)}</span>
		</div>
		<p class="text-base leading-relaxed">{voiceScript(language)}</p>
		<div class="h-1.5 overflow-hidden rounded-full bg-surface-raised">
			<div class="h-full bg-red-600 transition-[width]" style:width="{progress}%"></div>
		</div>
		<AudioMeter level={recorder.level} muted={false} label={t.section} />
		<Button
			size="sm"
			variant="secondary"
			disabled={!recorder.canStop}
			onclick={() => {
				recorder.stop();
			}}
		>
			{recorder.canStop ? t.done : t.keepReading(MIN_SECONDS)}
		</Button>
	{:else if recorder.step === 'creating'}
		<p class="flex items-center gap-2 text-sm">
			<span
				class="size-4 animate-spin rounded-full border-2 border-brand-500 border-t-transparent"
				aria-hidden="true"
			></span>
			{t.creating}
		</p>
	{:else if recorder.step === 'ready'}
		<div class="flex items-center justify-between gap-2">
			<p class="flex items-center gap-2 text-sm font-medium text-green-700">
				<Check class="size-4" aria-hidden="true" />
				{recorder.kept ? t.keptReady : t.ready}
			</p>
			<Button
				size="sm"
				variant="ghost"
				onclick={() => {
					recorder.reset();
				}}
			>
				<RotateCcw class="size-4" aria-hidden="true" />
				{t.redo}
			</Button>
		</div>
		<p class="text-xs text-ink-muted">{recorder.kept ? t.keptHint : t.readyHint}</p>
	{:else}
		<p class="text-center text-sm font-medium">{t.title}</p>
		<p class="text-center text-xs text-ink-muted">
			{required ? t.requiredHint(RECORD_SECONDS) : t.hint(RECORD_SECONDS)}
		</p>
		{#if recorder.error}<p role="alert" class="text-center text-xs text-red-600">
				{t[recorder.error]}
			</p>{/if}
		<!-- When joining needs it, recording is the main step, so it gets the main button size. -->
		<Button
			size={required ? 'lg' : 'sm'}
			class={required ? 'self-center' : undefined}
			onclick={record}
		>
			<Mic class="size-4" aria-hidden="true" />
			{recorder.step === 'error' ? t.tryAgain : t.record(RECORD_SECONDS)}
		</Button>
	{/if}
</section>
