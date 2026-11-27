<script lang="ts">
	import { goto } from '$app/navigation';
	import { resolve } from '$app/paths';
	import { page } from '$app/state';
	import { AccountSession, Onboarding } from '#lib/features/account/index.ts';
	import { SiteFooter, SiteHeader } from '#lib/features/landing/index.ts';
	import { getUserProfile, MascotAvatar } from '#lib/features/room/index.ts';
	import { i18n } from '#lib/i18n/index.ts';

	/**
	 * `/signin?next=…`: sign in or claim an @, then go on: to `/@username` when joining
	 * someone's room, otherwise to your own room (`/open`). Nothing else is followed, so the
	 * link cannot send anyone elsewhere.
	 */
	const requested = page.url.searchParams.get('next') ?? '';
	/** A name to claim, from a free `/@name`. */
	const claim = page.url.searchParams.get('claim') ?? undefined;
	/** Joining someone's room: the page shows whose room it is. */
	const roomOwner = /^\/@([a-z0-9_]+)$/.exec(requested)?.[1];
	const next = roomOwner ? resolve('/@[username]', { username: roomOwner }) : resolve('/open');
	let mascot = $state<string>();

	const session = new AccountSession();
	$effect(() => {
		void session.load();
		if (roomOwner) {
			void getUserProfile(roomOwner)
				.then((profile) => (mascot = profile?.mascot))
				.catch(() => undefined);
		}
	});
	// Signed in (already, or just now): go where the visitor was heading.
	$effect(() => {
		if (session.account) void goto(next, { replace: true });
	});
</script>

<svelte:head>
	<title>{i18n.t.openRoom.submit} · Diceri</title>
</svelte:head>

<SiteHeader {session} />
<main class="flex flex-col items-center gap-10 px-4 pt-6 pb-10">
	{#if session.account === null}
		{#if roomOwner}
			<Onboarding {session} initialUsername={claim}>
				{#snippet intro()}
					<!-- Whose room this is, and in plain words why an account comes first. -->
					<div class="flex flex-col items-center gap-3 text-center">
						{#if mascot}<MascotAvatar {mascot} size={112} />{/if}
						<h1 class="text-3xl font-semibold tracking-tight">
							{i18n.t.lobby.joinRoomTitle(roomOwner)}
						</h1>
						<p class="text-ink-muted">{i18n.t.lobby.joinRoomHint}</p>
					</div>
				{/snippet}
			</Onboarding>
		{:else}
			<Onboarding {session} initialUsername={claim} />
		{/if}
	{/if}
	<SiteFooter />
</main>
