<script lang="ts">
	import { page } from '$app/state';
	import { AccountSession, Onboarding } from '#lib/features/account/index.ts';
	import { Headline, SiteHeader } from '#lib/features/landing/index.ts';
	import { i18n } from '#lib/i18n/index.ts';
	import { OpenRoomForm } from '#lib/features/room/index.ts';

	/** AGPL section 13: users of the network service can get its source. */
	const SOURCE_URL = 'https://github.com/julianosig/diceri.com';

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
		<Headline />
		{#if account.account}
			<OpenRoomForm username={account.account.username} mascot={account.account.mascot} />
		{:else if account.account === null}
			<Onboarding
				session={account}
				initialUsername={page.url.searchParams.get('claim') ?? undefined}
			/>
		{/if}
	</div>
	<footer class="mt-auto text-xs text-ink-muted">
		{i18n.t.footer.freeSoftware} ·
		<a class="underline" href="{SOURCE_URL}/blob/main/LICENSE">AGPL-3.0</a>
		·
		<a class="underline" href={SOURCE_URL}>{i18n.t.footer.sourceCode}</a>
	</footer>
</main>
