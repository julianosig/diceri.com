<script lang="ts">
	import type { AccountSession } from '../account-session.svelte.ts';

	interface Props {
		session: AccountSession;
	}

	let { session }: Props = $props();
</script>

{#if session.username}
	<div class="flex items-center gap-3 text-sm">
		<span class="font-medium">@{session.username}</span>
		<button type="button" class="text-ink-muted underline" onclick={() => session.signOut()}>
			Sign out
		</button>
	</div>
{/if}
