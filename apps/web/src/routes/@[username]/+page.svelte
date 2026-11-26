<script lang="ts">
	import { refreshAll } from '$app/navigation';
	import { page } from '$app/state';
	import { LanguageSelect, ThemeToggle } from '#lib/components/ui/index.ts';
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
		<MeetingFlow room={data.room} {hostKey} meetingUrl={page.url.href} />
	{/key}
{:else}
	<header class="mx-auto flex w-full max-w-5xl items-center justify-between gap-4 px-4 py-4">
		<LanguageSelect class="-ms-2" />
		<ThemeToggle />
	</header>
	<main class="flex min-h-[calc(100dvh-6rem)] items-center justify-center px-4 pb-16">
		<RoomOffline
			username={data.username}
			mascot={data.profile?.mascot ?? null}
			onretry={() => refreshAll()}
		/>
	</main>
{/if}
