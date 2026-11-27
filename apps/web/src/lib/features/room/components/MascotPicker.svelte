<script lang="ts">
	import { Sparkles } from '@lucide/svelte';
	import { i18n } from '#lib/i18n/index.ts';
	import { MASCOTS, type MascotId } from '../mascots.ts';
	import MascotAvatar from './MascotAvatar.svelte';

	interface Props {
		value: MascotId;
		onchange: (mascot: MascotId) => void;
		/** `large`: a dedicated step with big, sharp mascots and the legend left to its title. */
		variant?: 'compact' | 'large';
	}

	let { value, onchange, variant = 'compact' }: Props = $props();
	const large = $derived(variant === 'large');
</script>

<fieldset class="flex flex-col gap-2">
	<legend class={large ? 'sr-only' : 'mb-1 flex items-center gap-1.5 text-sm text-ink-muted'}>
		<Sparkles class="size-4 text-brand-500" aria-hidden="true" />
		{i18n.t.mascots.legend}
	</legend>
	<div class={large ? 'grid grid-cols-3 gap-3 sm:grid-cols-4' : 'grid grid-cols-6 gap-1.5'}>
		{#each MASCOTS as mascot (mascot.id)}
			<label
				class={[
					'cursor-pointer transition has-focus-visible:ring-2 has-focus-visible:ring-brand-500',
					large ? 'flex flex-col items-center gap-1 rounded-2xl p-2' : 'rounded-full p-0.5',
					value === mascot.id
						? 'bg-brand-500/15 ring-2 ring-brand-500'
						: 'opacity-70 hover:opacity-100'
				]}
				title={i18n.t.mascots.names[mascot.id]}
			>
				<input
					type="radio"
					class="sr-only"
					name="mascot"
					value={mascot.id}
					checked={value === mascot.id}
					onchange={() => {
						onchange(mascot.id);
					}}
					aria-label={i18n.t.mascots.names[mascot.id]}
				/>
				<MascotAvatar mascot={mascot.id} size={large ? 112 : 44} decorative class="w-full" />
				{#if large}
					<span class="text-xs font-medium text-ink-muted">{i18n.t.mascots.names[mascot.id]}</span>
				{/if}
			</label>
		{/each}
	</div>
</fieldset>
