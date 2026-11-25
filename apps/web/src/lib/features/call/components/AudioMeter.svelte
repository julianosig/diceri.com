<script lang="ts">
	import { cn } from '#lib/utils/cn.ts';

	interface Props {
		level: number;
		muted: boolean;
		label?: string;
		tone?: 'light' | 'dark';
	}

	let {
		level,
		muted,
		label = 'Outgoing level (after filtering)',
		tone = 'light'
	}: Props = $props();
	const percent = $derived(Math.round(level * 100));
</script>

<div class="flex flex-col gap-1">
	<div
		class={cn('flex justify-between text-xs', tone === 'dark' ? 'text-white/70' : 'text-ink-muted')}
	>
		<span>{label}</span>
		<span>{muted ? 'Muted' : `${String(percent)}%`}</span>
	</div>
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
