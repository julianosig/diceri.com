<script lang="ts">
	import { Sparkles } from '@lucide/svelte';
	import { MASCOTS, type MascotId } from '../mascots.ts';
	import MascotAvatar from './MascotAvatar.svelte';

	interface Props {
		value: MascotId;
		onchange: (mascot: MascotId) => void;
	}

	let { value, onchange }: Props = $props();
</script>

<fieldset class="flex flex-col gap-2">
	<legend class="mb-1 flex items-center gap-1.5 text-base font-semibold">
		<Sparkles class="size-4 text-brand-500" aria-hidden="true" />
		Room symbol
	</legend>
	<div class="grid grid-cols-6 gap-1.5">
		{#each MASCOTS as mascot (mascot.id)}
			<label
				class={[
					'cursor-pointer rounded-full p-0.5 transition has-focus-visible:ring-2 has-focus-visible:ring-brand-500',
					value === mascot.id
						? 'bg-brand-500 ring-2 ring-brand-500'
						: 'opacity-70 hover:opacity-100'
				]}
				title={mascot.label}
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
					aria-label={mascot.label}
				/>
				<MascotAvatar mascot={mascot.id} size={44} decorative class="w-full" />
			</label>
		{/each}
	</div>
</fieldset>
