<script lang="ts">
	import { goto } from '$app/navigation';
	import { resolve } from '$app/paths';
	import { DoorOpen, LoaderCircle, ShieldCheck, Video } from '@lucide/svelte';
	import { Button } from '#lib/components/ui/index.ts';
	import { i18n } from '#lib/i18n/index.ts';
	import { saveHostKey } from '../host-keys.ts';
	import { openRoom } from '../room-api.ts';
	import type { RoomEntry } from '../types.ts';

	const t = $derived(i18n.t.openRoom);
	const ENTRIES = $derived([
		{ value: 'approval' as RoomEntry, label: t.approval, icon: ShieldCheck },
		{ value: 'open' as RoomEntry, label: t.open, icon: DoorOpen }
	]);

	let entry = $state<RoomEntry>('approval');
	let pending = $state(false);
	let failed = $state(false);

	async function submit(event: SubmitEvent) {
		event.preventDefault();
		pending = true;
		failed = false;
		try {
			// Rooms stay off the live board while it is hidden; the API keeps the option.
			const room = await openRoom({ listed: false, entry });
			saveHostKey(room.code, room.hostKey);
			await goto(resolve('/@[username]', { username: room.owner }));
		} catch {
			failed = true;
		} finally {
			pending = false;
		}
	}
</script>

<form class="flex w-full max-w-md flex-col gap-5 text-start" onsubmit={submit}>
	<fieldset class="flex flex-col gap-2">
		<legend class="mb-1 text-base font-semibold">{t.whoGetsIn}</legend>
		<div class="grid grid-cols-2 gap-2">
			{#each ENTRIES as option (option.value)}
				{@const Icon = option.icon}
				<label
					class={[
						'flex cursor-pointer items-center justify-center gap-1.5 rounded-lg border px-3 py-2 text-sm transition-colors has-focus-visible:ring-2 has-focus-visible:ring-brand-500',
						entry === option.value
							? 'border-brand-500 bg-brand-500/10 font-medium'
							: 'border-ink-muted/30'
					]}
				>
					<input type="radio" class="sr-only" bind:group={entry} value={option.value} />
					<Icon class="size-4 shrink-0" aria-hidden="true" />
					{option.label}
				</label>
			{/each}
		</div>
		{#if entry === 'open'}
			<p class="text-xs text-ink-muted">
				{t.openHint}
			</p>
		{/if}
	</fieldset>
	<Button size="lg" type="submit" disabled={pending}>
		{#if pending}
			<LoaderCircle class="size-5 animate-spin" aria-hidden="true" />
			{t.submitting}
		{:else}
			<Video class="size-5" aria-hidden="true" />
			{t.submit}
		{/if}
	</Button>
	{#if failed}
		<p role="alert" class="text-sm text-red-600">{t.failed}</p>
	{/if}
</form>
