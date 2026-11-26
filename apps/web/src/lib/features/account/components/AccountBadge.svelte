<script lang="ts">
	import { CircleUserRound, LogOut } from '@lucide/svelte';
	import type { AccountSession } from '../account-session.svelte.ts';

	interface Props {
		session: AccountSession;
	}

	let { session }: Props = $props();
</script>

{#if session.username}
	<div class="flex items-center gap-3 text-sm">
		<span class="flex items-center gap-1.5 font-medium">
			<CircleUserRound class="size-5 text-brand-500" aria-hidden="true" />
			<span>@{session.username}</span>
		</span>
		<button
			type="button"
			class="flex items-center gap-1 text-ink-muted hover:text-ink"
			onclick={() => session.signOut()}
		>
			<LogOut class="size-4" aria-hidden="true" />
			Sign out
		</button>
	</div>
{/if}
