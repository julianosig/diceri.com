<script lang="ts">
	import type { MonitorMode } from '../call-session.svelte.ts';

	interface Props {
		mode: MonitorMode;
		onchange: (mode: MonitorMode) => void;
	}

	let { mode, onchange }: Props = $props();

	const modes: { value: MonitorMode; label: string; hint: string }[] = [
		{ value: 'off', label: 'Off', hint: 'You do not hear yourself.' },
		{
			value: 'local',
			label: 'Local',
			hint: 'Hear your mic after the noise filter, instantly. Judges filter quality.'
		},
		{
			value: 'loopback',
			label: 'Via server',
			hint: 'Hear yourself after a round trip through the media server. Judges transmission.'
		}
	];
</script>

<fieldset class="flex flex-col gap-2">
	<legend class="mb-1 text-sm font-medium">Hear myself (use headphones)</legend>
	<div class="flex gap-2" role="radiogroup">
		{#each modes as option (option.value)}
			<button
				type="button"
				role="radio"
				aria-checked={mode === option.value}
				class="flex-1 rounded-lg bg-surface-raised px-3 py-2 text-sm aria-checked:bg-brand-500 aria-checked:text-white"
				onclick={() => {
					onchange(option.value);
				}}
			>
				{option.label}
			</button>
		{/each}
	</div>
	<p class="text-xs text-ink-muted">{modes.find((m) => m.value === mode)?.hint}</p>
</fieldset>
