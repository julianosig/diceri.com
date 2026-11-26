<script lang="ts">
	import { goto } from '$app/navigation';
	import { resolve } from '$app/paths';
	import { DoorOpen, ShieldCheck } from '@lucide/svelte';
	import { Button, Toggle } from '#lib/components/ui/index.ts';
	import { saveHostKey } from '../host-keys.ts';
	import { openRoom } from '../room-api.ts';
	import type { RoomEntry } from '../types.ts';
	import MascotAvatar from './MascotAvatar.svelte';

	interface Props {
		/** The signed-in person: their room is `@username` with their mascot. */
		username: string;
		mascot: string;
	}

	let { username, mascot }: Props = $props();

	const ENTRIES: { value: RoomEntry; label: string; icon: typeof ShieldCheck }[] = [
		{ value: 'approval', label: 'I approve each person', icon: ShieldCheck },
		{ value: 'open', label: 'Anyone can join', icon: DoorOpen }
	];

	let listed = $state(false);
	let entry = $state<RoomEntry>('approval');
	let pending = $state(false);
	let failed = $state(false);

	async function submit(event: SubmitEvent) {
		event.preventDefault();
		pending = true;
		failed = false;
		try {
			const room = await openRoom({ listed, entry });
			saveHostKey(room.code, room.hostKey);
			await goto(resolve('/@[username]', { username: room.owner }));
		} catch {
			failed = true;
		} finally {
			pending = false;
		}
	}
</script>

<form class="flex w-full max-w-md flex-col gap-5 text-center" onsubmit={submit}>
	<div class="flex flex-col items-center gap-2">
		<MascotAvatar {mascot} size={56} />
		<div class="flex flex-col items-center">
			<h2 class="text-xl font-bold">@{username}</h2>
			<p class="text-sm text-ink-muted">Your room lives at diceri.com/@{username}</p>
		</div>
	</div>
	<Toggle
		checked={listed}
		label="Show on the live board"
		description="Anyone on the home page sees your room and can open it."
		onchange={(value: boolean) => (listed = value)}
	/>
	<fieldset class="flex flex-col gap-2">
		<legend class="mx-auto mb-1 text-base font-semibold">Who gets in</legend>
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
				Anyone who knows your @ walks straight in while the room is open.
			</p>
		{/if}
	</fieldset>
	<Button size="lg" type="submit" disabled={pending}>
		{pending ? 'Opening…' : 'Open my room'}
	</Button>
	{#if failed}
		<p role="alert" class="text-sm text-red-600">Could not open your room. Try again.</p>
	{/if}
</form>
