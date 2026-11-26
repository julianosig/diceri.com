<script lang="ts">
	import { goto } from '$app/navigation';
	import { resolve } from '$app/paths';
	import { DoorOpen, ShieldCheck } from '@lucide/svelte';
	import { Button, Toggle } from '#lib/components/ui/index.ts';
	import { saveHostKey } from '../host-keys.ts';
	import { randomMascot, type MascotId } from '../mascots.ts';
	import { createRoom } from '../room-api.ts';
	import { isValidTitle, MAX_TITLE_LENGTH, suggestTitle } from '../room-titles.ts';
	import type { RoomEntry } from '../types.ts';
	import MascotAvatar from './MascotAvatar.svelte';
	import MascotPicker from './MascotPicker.svelte';

	const ENTRIES: { value: RoomEntry; label: string; icon: typeof ShieldCheck }[] = [
		{ value: 'approval', label: 'I approve each person', icon: ShieldCheck },
		{ value: 'open', label: 'Anyone can join', icon: DoorOpen }
	];

	let title = $state(suggestTitle());
	let mascot = $state<MascotId>(randomMascot());
	let listed = $state(false);
	let entry = $state<RoomEntry>('approval');
	let pending = $state(false);
	let failed = $state(false);

	async function submit(event: SubmitEvent) {
		event.preventDefault();
		pending = true;
		failed = false;
		try {
			const room = await createRoom({ title: title.trim(), listed, entry, mascot });
			saveHostKey(room.code, room.hostKey);
			await goto(resolve('/r/[room]', { room: room.code }));
		} catch {
			failed = true;
		} finally {
			pending = false;
		}
	}
</script>

<form
	class="flex w-full max-w-md flex-col gap-4 rounded-tile bg-surface-raised p-4 text-left"
	onsubmit={submit}
>
	<div class="flex items-end gap-3">
		<MascotAvatar {mascot} size={56} />
		<label class="flex flex-1 flex-col gap-1 text-sm">
			<span class="text-ink-muted">Room title</span>
			<input
				class="rounded-lg border border-ink-muted/30 bg-surface px-3 py-2 text-base"
				bind:value={title}
				maxlength={MAX_TITLE_LENGTH}
				required
			/>
		</label>
	</div>
	<MascotPicker
		value={mascot}
		onchange={(next: MascotId) => {
			mascot = next;
		}}
	/>
	<Toggle
		checked={listed}
		label="Show on the live board"
		description="Anyone on the home page sees the title and your username."
		onchange={(value: boolean) => (listed = value)}
	/>
	<fieldset class="flex flex-col gap-2">
		<legend class="mb-1 text-sm font-medium">Who gets in</legend>
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
	</fieldset>
	<Button size="lg" type="submit" disabled={pending || !isValidTitle(title)}>
		{pending ? 'Creating…' : 'New meeting'}
	</Button>
	{#if failed}
		<p role="alert" class="text-sm text-red-600">Could not create the meeting. Try again.</p>
	{/if}
</form>
