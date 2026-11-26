<script lang="ts">
	import { AccountBadge, AccountSession, Onboarding } from '#lib/features/account/index.ts';
	import { LivePresence, LiveRooms } from '#lib/features/presence/index.ts';
	import { Logo } from '#lib/components/ui/index.ts';
	import { OpenRoomForm } from '#lib/features/room/index.ts';

	/** AGPL section 13: users of the network service can get its source. */
	const SOURCE_URL = 'https://github.com/julianosig/diceri.com';

	const presence = new LivePresence();
	const account = new AccountSession();
	$effect(() => presence.start());
	$effect(() => {
		void account.load();
	});
</script>

<svelte:head>
	<title>Diceri — the call you know, in every language</title>
</svelte:head>

<header class="mx-auto flex w-full max-w-5xl items-center justify-between gap-4 px-4 py-4">
	<Logo class="h-12 sm:h-14" />
	<AccountBadge session={account} />
</header>

<main
	class="mx-auto flex min-h-[calc(100dvh-5rem)] max-w-3xl flex-col items-center gap-10 px-4 pt-8 pb-6 text-center sm:pt-14"
>
	<div class="flex flex-col items-center gap-8">
		<h1 class="text-4xl font-semibold tracking-tight text-balance sm:text-6xl">
			The call you already know.<br />
			<span class="bg-gradient-to-r from-brand-violet to-brand-500 bg-clip-text text-transparent">
				Now in every language.
			</span>
		</h1>
		<p class="max-w-xl text-lg text-pretty text-ink-muted">
			Talk the way you always do. Everyone hears you live in their own language —
			<strong class="font-semibold text-ink">in your own voice</strong>.
		</p>
		{#if account.account}
			<OpenRoomForm username={account.account.username} mascot={account.account.mascot} />
		{:else if account.account === null}
			<Onboarding session={account} />
		{/if}
	</div>
	<LiveRooms snapshot={presence.snapshot} />
	<footer class="mt-auto text-xs text-ink-muted">
		Free software under the
		<a class="underline" href="{SOURCE_URL}/blob/main/LICENSE">AGPL-3.0</a>
		·
		<a class="underline" href={SOURCE_URL}>Source code</a>
	</footer>
</main>
