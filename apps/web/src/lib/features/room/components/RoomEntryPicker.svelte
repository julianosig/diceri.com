<script lang="ts">
	import { DoorOpen, ShieldCheck } from '@lucide/svelte';
	import { i18n } from '#lib/i18n/index.ts';
	import { openRoom } from '../room-api.ts';
	import type { RoomEntry } from '../types.ts';

	interface Props {
		/** The live room's current rule. */
		entry: RoomEntry;
	}

	let { entry }: Props = $props();
	const t = $derived(i18n.t.openRoom);
	const ENTRIES = $derived([
		{ value: 'approval' as RoomEntry, label: t.approval, icon: ShieldCheck },
		{ value: 'open' as RoomEntry, label: t.open, icon: DoorOpen }
	]);

	// svelte-ignore state_referenced_locally
	let current = $state<RoomEntry>(entry);
	let failed = $state(false);

	/** The host changes who gets in; the live room is updated at once. */
	async function choose(next: RoomEntry) {
		const previous = current;
		current = next;
		failed = false;
		try {
			await openRoom({ listed: false, entry: next });
		} catch {
			current = previous;
			failed = true;
		}
	}
</script>

<fieldset class="flex w-full flex-col gap-2 text-start">
	<legend class="mb-1 text-sm text-ink-muted">{t.whoGetsIn}</legend>
	<div class="grid grid-cols-1 gap-2 sm:grid-cols-2">
		{#each ENTRIES as option (option.value)}
			{@const Icon = option.icon}
			<label
				class={[
					'flex cursor-pointer items-center justify-center gap-1.5 rounded-lg border px-3 py-2 text-sm whitespace-nowrap transition-colors has-focus-visible:ring-2 has-focus-visible:ring-brand-500',
					current === option.value
						? 'border-brand-500 bg-brand-500/10 font-medium'
						: 'border-ink-muted/30'
				]}
			>
				<input
					type="radio"
					class="sr-only"
					name="room-entry"
					value={option.value}
					checked={current === option.value}
					onchange={() => choose(option.value)}
				/>
				<Icon class="size-4 shrink-0" aria-hidden="true" />
				{option.label}
			</label>
		{/each}
	</div>
	{#if failed}
		<p role="alert" class="text-xs text-red-600">{t.failed}</p>
	{:else if current === 'open'}
		<p class="text-xs text-ink-muted">{t.openHint}</p>
	{/if}
</fieldset>
