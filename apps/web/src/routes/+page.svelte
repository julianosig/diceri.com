<script lang="ts">
	import { LivePresence, LiveRooms } from '#lib/features/presence/index.ts';
	import { CreateRoomButton } from '#lib/features/room/index.ts';

	/** AGPL section 13: users of the network service can get its source. */
	const SOURCE_URL = 'https://github.com/julianosig/diceri.com';

	const presence = new LivePresence();
	$effect(() => presence.start());
</script>

<svelte:head>
	<title>Diceri — talk in any language</title>
</svelte:head>

<main
	class="mx-auto flex min-h-dvh max-w-3xl flex-col items-center gap-10 px-4 pt-16 pb-6 text-center sm:pt-24"
>
	<div class="flex flex-col items-center gap-8">
		<h1 class="text-4xl font-semibold tracking-tight sm:text-5xl">Talk in any language.</h1>
		<p class="max-w-xl text-lg text-ink-muted">
			Start an anonymous meeting, share the link, and each person hears the other in their own
			language — in a voice that sounds like the speaker.
		</p>
		<CreateRoomButton />
	</div>
	<LiveRooms snapshot={presence.snapshot} />
	<footer class="mt-auto text-xs text-ink-muted">
		Free software under the
		<a class="underline" href="{SOURCE_URL}/blob/main/LICENSE">AGPL-3.0</a>
		·
		<a class="underline" href={SOURCE_URL}>Source code</a>
	</footer>
</main>
