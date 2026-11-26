<script lang="ts">
	import { LogOut } from '@lucide/svelte';
	import { i18n } from '#lib/i18n/index.ts';
	import { MascotAvatar } from '#lib/features/room/index.ts';
	import type { AccountSession } from '../account-session.svelte.ts';

	interface Props {
		session: AccountSession;
	}

	let { session }: Props = $props();
</script>

{#if session.account}
	{@const account = session.account}
	<div class="flex items-center gap-3 text-sm">
		<span class="flex items-center gap-2 font-medium">
			<MascotAvatar mascot={account.mascot} size={28} decorative />
			<span>@{account.username}</span>
		</span>
		<button
			type="button"
			class="flex items-center gap-1 text-ink-muted hover:text-ink"
			onclick={() => session.signOut()}
		>
			<LogOut class="size-4" aria-hidden="true" />
			{i18n.t.header.signOut}
		</button>
	</div>
{/if}
