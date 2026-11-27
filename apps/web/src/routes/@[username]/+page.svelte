<script lang="ts">
	import { refreshAll } from '$app/navigation';
	import { page } from '$app/state';
	import { AccountSession, Onboarding } from '#lib/features/account/index.ts';
	import { SiteHeader } from '#lib/features/landing/index.ts';
	import { i18n } from '#lib/i18n/index.ts';
	import { MeetingFlow } from '#lib/features/meeting/index.ts';
	import { loadHostKey, RoomOffline } from '#lib/features/room/index.ts';
	import type { PageProps } from './$types';

	let { data }: PageProps = $props();
	/** Host access needs both the saved key and the owner signed in (signing out ends it). */
	const hostKey = $derived(data.room && data.isOwner ? loadHostKey(data.room.code) : undefined);
	/**
	 * A room counts as live for visitors only while someone is in its call: an opened room
	 * nobody joined (or everyone left) shows as not live instead of a door nobody answers.
	 * The host (holding the host key) always gets their pre-join screen to start it.
	 */
	const open = $derived(data.room !== null && (hostKey !== undefined || data.room.people !== 0));
	const mascot = $derived(data.room?.mascot ?? data.profile?.mascot ?? null);

	/** Only accounts join calls: a signed-out visitor signs in or up first, then pre-join. */
	const session = new AccountSession();
	$effect(() => {
		if (session.account) void refreshAll();
	});

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

{#if data.room && open && !hostKey && !data.signedIn}
	<SiteHeader />
	<main class="flex justify-center px-4 pt-6 pb-16">
		<Onboarding {session} title={i18n.t.lobby.joinTitle} />
	</main>
{:else if data.room && open}
	{#key data.room.code}
		<MeetingFlow room={data.room} {hostKey} meetingUrl={page.url.href}>
			{#snippet header()}<SiteHeader />{/snippet}
		</MeetingFlow>
	{/key}
{:else}
	<SiteHeader />
	<main class="flex justify-center px-4 pt-6 pb-16">
		<RoomOffline username={data.username} {mascot} onretry={() => refreshAll()} />
	</main>
{/if}
