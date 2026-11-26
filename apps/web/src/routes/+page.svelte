<script lang="ts">
	import {
		AccountBadge,
		AccountKeyNotice,
		AccountSession,
		ClaimUsername
	} from '#lib/features/account/index.ts';
	import { LivePresence, LiveRooms } from '#lib/features/presence/index.ts';
	import { CreateRoomForm } from '#lib/features/room/index.ts';

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
	<title>Diceri — talk in any language</title>
</svelte:head>

<div class="absolute top-4 right-4">
	<AccountBadge session={account} />
</div>

<main
	class="mx-auto flex min-h-dvh max-w-3xl flex-col items-center gap-10 px-4 pt-16 pb-6 text-center sm:pt-24"
>
	<div class="flex flex-col items-center gap-8">
		<h1 class="text-4xl font-semibold tracking-tight sm:text-5xl">Talk in any language.</h1>
		<p class="max-w-xl text-lg text-ink-muted">
			Start an anonymous meeting, share the link, and each person hears the other in their own
			language — in a voice that sounds like the speaker.
		</p>
		<AccountKeyNotice session={account} />
		{#if account.username}
			<CreateRoomForm />
		{:else if account.username === null}
			<ClaimUsername session={account} />
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
