<script lang="ts">
	import { goto } from '$app/navigation';
	import { resolve } from '$app/paths';
	import { Button } from '#lib/components/ui/index.ts';
	import { saveHostKey } from '../host-keys.ts';
	import { createRoom } from '../room-api.ts';

	let pending = $state(false);
	let failed = $state(false);

	async function handleClick() {
		pending = true;
		failed = false;
		try {
			const room = await createRoom();
			saveHostKey(room.id, room.hostKey);
			await goto(resolve('/r/[room]', { room: room.id }));
		} catch {
			failed = true;
		} finally {
			pending = false;
		}
	}
</script>

<div class="flex flex-col items-center gap-3">
	<Button size="lg" disabled={pending} onclick={handleClick}>
		{pending ? 'Creating…' : 'New meeting'}
	</Button>
	{#if failed}
		<p role="alert" class="text-sm text-red-600">Could not create the meeting. Try again.</p>
	{/if}
</div>
