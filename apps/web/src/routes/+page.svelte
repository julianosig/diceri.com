<script lang="ts">
	import { page } from '$app/state';
	import { AccountSession, Onboarding } from '#lib/features/account/index.ts';
	import {
		Headline,
		OpenSourceBadge,
		SiteFooter,
		SiteHeader
	} from '#lib/features/landing/index.ts';
	import { i18n } from '#lib/i18n/index.ts';
	import { OpenRoomForm } from '#lib/features/room/index.ts';

	// The live board (`LiveRooms` from the presence feature) is hidden for now; the
	// API and component stay so it can come back with one line.
	const account = new AccountSession();
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
		<div class="flex flex-col items-center gap-5">
			<OpenSourceBadge />
			<Headline />
		</div>
		{#if account.account}
			<OpenRoomForm />
		{:else if account.account === null}
			<Onboarding
				session={account}
				initialUsername={page.url.searchParams.get('claim') ?? undefined}
			/>
		{/if}
	</div>
	<div class="mt-auto"><SiteFooter /></div>
</main>
