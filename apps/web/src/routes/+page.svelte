<script lang="ts">
	import { CreateRoomButton } from '#lib/features/room/index.ts';

	/** AGPL section 13: users of the network service can get its source. */
	const SOURCE_URL = 'https://github.com/julianosig/diceri.com';
</script>

<svelte:head>
	<title>Diceri — talk in any language</title>
</svelte:head>

<main
	class="mx-auto flex min-h-dvh max-w-3xl flex-col items-center justify-center gap-8 px-4 text-center"
>
	<h1 class="text-4xl font-semibold tracking-tight sm:text-5xl">Talk in any language.</h1>
	<p class="max-w-xl text-lg text-ink-muted">
		Start an anonymous meeting, share the link, and each person hears the other in their own
		language — in a voice that sounds like the speaker.
	</p>
	<CreateRoomButton />
	<footer class="absolute bottom-4 text-xs text-ink-muted">
		Free software under the
		<a class="underline" href="{SOURCE_URL}/blob/main/LICENSE">AGPL-3.0</a>
		·
		<a class="underline" href={SOURCE_URL}>Source code</a>
	</footer>
</main>
