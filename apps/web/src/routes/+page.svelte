<script lang="ts">
	import { CreateRoomButton } from '#lib/features/room/index.ts';
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
</main>
