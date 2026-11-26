<script lang="ts">
	import { page } from '$app/state';
	import { MeetingFlow } from '#lib/features/meeting/index.ts';
	import { loadHostKey } from '#lib/features/room/index.ts';
	import type { PageProps } from './$types';

	let { data }: PageProps = $props();
	const hostKey = $derived(loadHostKey(data.room.code));
</script>

<svelte:head>
	<title>{data.room.title} · Diceri</title>
</svelte:head>

{#key data.room.code}
	<MeetingFlow room={data.room} {hostKey} meetingUrl={page.url.href} />
{/key}
