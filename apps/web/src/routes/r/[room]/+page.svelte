<script lang="ts">
	import { page } from '$app/state';
	import { MeetingFlow } from '#lib/features/meeting/index.ts';
	import { loadHostKey } from '#lib/features/room/index.ts';
	import type { PageProps } from './$types';

	let { data }: PageProps = $props();
	const hostKey = $derived(loadHostKey(data.room.id));
</script>

<svelte:head>
	<title>Meeting · Diceri</title>
</svelte:head>

{#key data.room.id}
	<MeetingFlow roomId={data.room.id} {hostKey} meetingUrl={page.url.href} />
{/key}
