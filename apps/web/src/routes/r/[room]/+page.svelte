<script lang="ts">
	import { page } from '$app/state';
	import { CallPanel } from '#lib/features/call/index.ts';
	import { ShareLink } from '#lib/features/room/index.ts';
	import type { PageProps } from './$types';

	let { data }: PageProps = $props();
</script>

<svelte:head>
	<title>Meeting · Diceri</title>
</svelte:head>

<main
	class="mx-auto flex min-h-dvh max-w-3xl flex-col items-center justify-center gap-6 px-4 py-10"
>
	<h1 class="text-2xl font-semibold">Your meeting is ready</h1>
	<p class="text-ink-muted">
		Share this link with the person you want to talk to (up to {data.room.maxParticipants}).
	</p>
	<ShareLink url={page.url.href} />
	<CallPanel roomId={data.room.id} />
</main>
