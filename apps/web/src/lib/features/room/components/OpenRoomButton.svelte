<script lang="ts">
	import { goto } from '$app/navigation';
	import { resolve } from '$app/paths';
	import { LoaderCircle, Video } from '@lucide/svelte';
	import { onMount } from 'svelte';
	import { ApiError } from '#lib/api/http-client.ts';
	import { Button } from '#lib/components/ui/index.ts';
	import { i18n } from '#lib/i18n/index.ts';
	import { limitText } from '#lib/features/plans/index.ts';
	import { saveHostKey } from '../host-keys.ts';
	import { getUserRoom, openRoom } from '../room-api.ts';
	import type { RoomEntry } from '../types.ts';

	interface Props {
		/** The signed-in person; their room is `/@username`. */
		username: string;
		/** Opens right away (the person just signed up or in after asking to open it). */
		autostart?: boolean;
	}

	let { username, autostart = false }: Props = $props();
	const t = $derived(i18n.t.openRoom);
	let pending = $state(false);
	/** Why opening failed, in the visitor's language (a plan limit, or a plain failure). */
	let problem = $state<string | undefined>();

	/** The live room's entry rule, so reopening keeps it; new rooms ask for approval. */
	async function currentEntry(): Promise<RoomEntry> {
		try {
			return (await getUserRoom(username)).entry;
		} catch (cause) {
			if (cause instanceof ApiError && cause.status === 404) return 'approval';
			throw cause;
		}
	}

	async function open() {
		pending = true;
		problem = undefined;
		try {
			// Opening (or reopening) returns the host key, so this browser can host it.
			// Rooms stay off the live board while it is hidden; the API keeps the option.
			const room = await openRoom({ listed: false, entry: await currentEntry() });
			saveHostKey(room.code, room.hostKey);
			await goto(resolve('/@[username]', { username: room.owner }));
		} catch (cause) {
			problem = limitText(cause, i18n.t) ?? t.failed;
		} finally {
			pending = false;
		}
	}

	onMount(() => {
		if (autostart) void open();
	});
</script>

<div class="flex w-full max-w-md flex-col items-center gap-3">
	<Button size="lg" class="w-full" disabled={pending} onclick={open}>
		{#if pending}
			<LoaderCircle class="size-5 animate-spin" aria-hidden="true" />
			{t.submitting}
		{:else}
			<Video class="size-5" aria-hidden="true" />
			{t.submit}
		{/if}
	</Button>
	{#if problem}
		<p role="alert" class="text-sm text-red-600">{problem}</p>
	{/if}
</div>
