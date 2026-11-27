<script lang="ts">
	import { refreshAll } from '$app/navigation';
	import { page } from '$app/state';
	import { SiteHeader } from '#lib/features/landing/index.ts';
	import { MeetingFlow } from '#lib/features/meeting/index.ts';
	import { RoomOffline } from '#lib/features/room/index.ts';
	import type { PageProps } from './$types';

	let { data }: PageProps = $props();
	/**
	 * A room counts as live for visitors only while someone is in its call: an opened room
	 * nobody joined (or everyone left) shows as not live instead of a door nobody answers.
	 */
	const open = $derived(data.room !== null && data.room.people !== 0);
	const mascot = $derived(data.room?.mascot ?? data.profile?.mascot ?? null);

	/** How often a visitor at a claimed, not-live room looks again. */
	const RECHECK_MS = 4000;

	// A claimed room that is not live yet keeps checking, so visitors move on to join (or
	// ask to) as soon as the host starts the call, without pressing Try again. The media
	// server also needs a second or two to report someone who just joined.
	$effect(() => {
		if (open || mascot === null) return;
		const timer = setInterval(() => {
			void refreshAll();
		}, RECHECK_MS);
		return () => {
			clearInterval(timer);
		};
	});
</script>

<svelte:head>
	<title>@{data.username} · Diceri</title>
</svelte:head>

{#if data.room && open}
	{#key data.room.code}
		<MeetingFlow room={data.room} hostKey={undefined} meetingUrl={page.url.href}>
			{#snippet header()}<SiteHeader />{/snippet}
		</MeetingFlow>
	{/key}
{:else}
	<SiteHeader />
	<main class="flex justify-center px-4 pt-6 pb-16">
		<RoomOffline username={data.username} {mascot} onretry={() => refreshAll()} />
	</main>
{/if}
