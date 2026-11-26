<script lang="ts">
	import { page } from '$app/state';
	import { AccountBadge, AccountSession, Onboarding } from '#lib/features/account/index.ts';
	import { HeroAnimation } from '#lib/features/landing/index.ts';
	import { LanguageSelect, Logo, ThemeToggle } from '#lib/components/ui/index.ts';
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

<header class="mx-auto flex w-full max-w-5xl items-center justify-between gap-4 px-4 py-4">
	<Logo class="h-11 sm:h-14" />
	<div class="flex items-center gap-2">
		<AccountBadge session={account} />
		<LanguageSelect />
		<ThemeToggle />
	</div>
</header>

<main
	class="mx-auto flex min-h-[calc(100dvh-5rem)] max-w-5xl flex-col items-center gap-10 px-4 pt-2 pb-6 text-center sm:pt-6"
>
	<HeroAnimation class="max-w-5xl" />
	<div class="flex max-w-3xl flex-col items-center gap-8">
		<h1 class="text-4xl font-semibold tracking-tight text-balance sm:text-6xl">
			{i18n.t.hero.headline}<br />
			<span class="bg-gradient-to-r from-brand-violet to-brand-500 bg-clip-text text-transparent">
				{i18n.t.hero.promise}
			</span>
		</h1>
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
