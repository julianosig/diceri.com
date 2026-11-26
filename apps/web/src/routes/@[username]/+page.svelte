<script lang="ts">
	import { refreshAll } from '$app/navigation';
	import { page } from '$app/state';
	import { SiteHeader } from '#lib/features/landing/index.ts';
	import { MeetingFlow } from '#lib/features/meeting/index.ts';
	import { loadHostKey, RoomOffline } from '#lib/features/room/index.ts';
	import type { PageProps } from './$types';

	let { data }: PageProps = $props();
	const hostKey = $derived(data.room ? loadHostKey(data.room.code) : undefined);
</script>

<svelte:head>
	<title>@{data.username} · Diceri</title>
</svelte:head>

{#if data.room}
	{#key data.room.code}
		<MeetingFlow room={data.room} {hostKey} meetingUrl={page.url.href}>
			{#snippet header()}<SiteHeader />{/snippet}
		</MeetingFlow>
	{/key}
{:else}
	<SiteHeader />
	<main class="flex justify-center px-4 pt-6 pb-16">
		<RoomOffline
			username={data.username}
			mascot={data.profile?.mascot ?? null}
			onretry={() => refreshAll()}
		/>
	</main>
{/if}
