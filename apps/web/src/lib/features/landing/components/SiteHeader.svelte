<script lang="ts">
	import { LanguageSelect, ThemeToggle } from '#lib/components/ui/index.ts';
	import { AccountBadge, AccountSession } from '#lib/features/account/index.ts';
	import HeroAnimation from './HeroAnimation.svelte';

	interface Props {
		/** The page's account session when it already has one (the home page shares it). */
		session?: AccountSession;
	}

	let { session }: Props = $props();
	// svelte-ignore state_referenced_locally
	const account = session ?? new AccountSession();

	$effect(() => {
		if (!session) void account.load();
	});
</script>

<!--
	The top of every page outside a call: language on the left, the signed-in @ and the
	theme on the right, then the banner with the Diceri mark as the site's masthead.
-->
<header class="mx-auto flex w-full max-w-5xl flex-col gap-6 px-4 pt-4 pb-6 sm:gap-10 sm:pb-10">
	<div class="flex items-center justify-between gap-4">
		<LanguageSelect class="-ms-2" />
		<div class="flex items-center gap-2">
			<AccountBadge session={account} />
			<ThemeToggle />
		</div>
	</div>
	<HeroAnimation />
</header>
