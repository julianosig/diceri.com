<script lang="ts">
	import { i18n } from '#lib/i18n/index.ts';
	import { cn } from '#lib/utils/cn.ts';

	interface Props {
		level: number;
		muted: boolean;
		label?: string;
		tone?: 'light' | 'dark';
		/** Show the label and percentage above the bar (the bar keeps its label for screen readers). */
		showLabel?: boolean;
	}

	let {
		level,
		muted,
		label = i18n.t.audio.outgoingLevel,
		tone = 'light',
		showLabel = true
	}: Props = $props();
	const percent = $derived(Math.round(level * 100));
</script>

<div class="flex flex-col gap-1">
	{#if showLabel}
		<div
			class={cn(
				'flex justify-between text-xs',
				tone === 'dark' ? 'text-white/70' : 'text-ink-muted'
			)}
		>
			<span>{label}</span>
			<span>{muted ? i18n.t.audio.muted : `${String(percent)}%`}</span>
		</div>
	{/if}
	<div
		class={cn(
			'h-2 overflow-hidden rounded-full',
			tone === 'dark' ? 'bg-white/15' : 'bg-surface-raised'
		)}
		role="meter"
		aria-label={label}
		aria-valuemin={0}
		aria-valuemax={100}
		aria-valuenow={percent}
	>
		<div
			class="h-full rounded-full bg-brand-500 transition-[width] duration-75"
			style:width="{percent}%"
		></div>
	</div>
</div>
