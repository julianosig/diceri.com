<script lang="ts">
	import { i18n } from '#lib/i18n/index.ts';
	import { MicOff } from '@lucide/svelte';
	import { cn } from '#lib/utils/cn.ts';
	import type { Participant } from '../call-session.svelte.ts';

	let { participant }: { participant: Participant } = $props();
	const label = $derived(
		participant.isLocal ? i18n.t.call.you(participant.name) : participant.name
	);
</script>

<article
	class={cn(
		'relative flex aspect-video items-center justify-center rounded-tile bg-tile transition-shadow',
		participant.speaking && !participant.muted && 'ring-4 ring-brand-400'
	)}
	aria-label={label}
>
	<div
		class="flex size-20 items-center justify-center rounded-full bg-brand-600 text-3xl font-medium text-white sm:size-28 sm:text-5xl"
		aria-hidden="true"
	>
		{participant.name.charAt(0).toUpperCase()}
	</div>
	{#if participant.muted}
		<span
			class="absolute top-3 right-3 flex size-8 items-center justify-center rounded-full bg-black/40"
			title={i18n.t.call.micOff}
		>
			<MicOff class="size-4 text-white" aria-label={i18n.t.call.micOff} />
		</span>
	{/if}
	<span
		class="absolute bottom-3 left-4 flex items-center gap-2 truncate text-sm font-medium text-white"
	>
		{label}
		{#if participant.outputLanguage}
			<span class="rounded bg-white/20 px-1.5 py-0.5 text-xs" title={i18n.t.call.hearsEveryoneIn}>
				{i18n.t.call.hears(participant.outputLanguage)}
			</span>
		{/if}
	</span>
	{#if participant.speaking && !participant.muted}<span class="sr-only">speaking</span>{/if}
</article>
