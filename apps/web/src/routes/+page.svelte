<script lang="ts">
	import { page } from '$app/state';
	import { AccountSession, Onboarding } from '#lib/features/account/index.ts';
	import { Headline, SiteFooter, SiteHeader } from '#lib/features/landing/index.ts';
	import { i18n } from '#lib/i18n/index.ts';
	import { Video } from '@lucide/svelte';
	import { Button } from '#lib/components/ui/index.ts';
	import { OpenRoomButton } from '#lib/features/room/index.ts';

	// The live board (`LiveRooms` from the presence feature) is hidden for now; the
	// API and component stay so it can come back with one line.
	const account = new AccountSession();
	/** A name to claim, from a free `/@name` (`/?claim=name`). */
	const claim = page.url.searchParams.get('claim') ?? undefined;
	/**
	 * One button for everyone: signed in, it opens your room; signed out, it first asks for
	 * your @ (login or claim), then opens your room as soon as you are in.
	 */
	let asked = $state(claim !== undefined);
	$effect(() => {
		void account.load();
	});
</script>

<svelte:head>
	<title>{i18n.t.meta.title}</title>
</svelte:head>

<SiteHeader session={account} />

<main
	class="mx-auto flex min-h-[calc(100dvh-14rem)] max-w-5xl flex-col items-center gap-10 px-4 pt-6 pb-6 text-center sm:pt-8"
>
	<div class="flex max-w-3xl flex-col items-center gap-8">
		<Headline />
		{#if account.account}
			<OpenRoomButton username={account.account.username} autostart={asked} />
		{:else if account.account === null}
			{#if asked}
				<Onboarding session={account} initialUsername={claim} />
			{:else}
				<Button size="lg" class="w-full max-w-md" onclick={() => (asked = true)}>
					<Video class="size-5" aria-hidden="true" />
					{i18n.t.openRoom.submit}
				</Button>
			{/if}
		{/if}
	</div>
	<div class="mt-auto"><SiteFooter /></div>
</main>
