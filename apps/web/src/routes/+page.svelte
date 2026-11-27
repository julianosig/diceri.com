<script lang="ts">
	import { resolve } from '$app/paths';
	import { Video } from '@lucide/svelte';
	import { buttonVariants } from '#lib/components/ui/index.ts';
	import { AccountSession } from '#lib/features/account/index.ts';
	import { Headline, SiteFooter, SiteHeader } from '#lib/features/landing/index.ts';
	import { PlanSummary } from '#lib/features/plans/index.ts';
	import { i18n } from '#lib/i18n/index.ts';

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

<!-- The landing page only: opening your room is `/open` (it asks you to sign in first). -->
<main
	class="mx-auto flex min-h-[calc(100dvh-14rem)] max-w-5xl flex-col items-center gap-10 px-4 pt-6 pb-6 text-center sm:pt-8"
>
	<div class="flex max-w-3xl flex-col items-center gap-8">
		<Headline />
		<a class={buttonVariants({ size: 'lg', class: 'w-full max-w-md' })} href={resolve('/open')}>
			<Video class="size-5" aria-hidden="true" />
			{i18n.t.openRoom.submit}
		</a>
		{#if account.account !== undefined}
			<div class="-mt-4"><PlanSummary usage={account.account?.usage} /></div>
		{/if}
	</div>
	<div class="mt-auto"><SiteFooter /></div>
</main>
