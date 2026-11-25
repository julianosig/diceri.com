<script lang="ts">
	let { level, muted }: { level: number; muted: boolean } = $props();
	const percent = $derived(Math.round(level * 100));
</script>

<div class="flex flex-col gap-1">
	<div class="flex justify-between text-xs text-ink-muted">
		<span>Outgoing level (after filtering)</span>
		<span>{muted ? 'Muted' : `${String(percent)}%`}</span>
	</div>
	<div
		class="h-2 overflow-hidden rounded-full bg-surface-raised"
		role="meter"
		aria-label="Microphone level"
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
