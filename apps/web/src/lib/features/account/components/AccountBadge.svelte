<script lang="ts">
	import { ChevronDown, LogOut } from '@lucide/svelte';
	import { i18n } from '#lib/i18n/index.ts';
	import { MascotAvatar } from '#lib/features/room/index.ts';
	import type { AccountSession } from '../account-session.svelte.ts';

	interface Props {
		session: AccountSession;
	}

	let { session }: Props = $props();
	let open = $state(false);
	let root = $state<HTMLElement>();

	function closeOnOutside(event: PointerEvent) {
		if (open && root && !root.contains(event.target as Node)) open = false;
	}
</script>

<svelte:window
	onpointerdown={closeOnOutside}
	onkeydown={(event: KeyboardEvent) => {
		if (event.key === 'Escape') open = false;
	}}
/>

<!-- The signed-in @ with its mascot; account actions live in its menu. -->
{#if session.account}
	{@const account = session.account}
	<div class="relative" bind:this={root}>
		<button
			type="button"
			class="flex items-center gap-2 rounded-full py-1 ps-1 pe-2.5 text-sm font-medium transition-colors hover:bg-surface-raised focus-visible:ring-2 focus-visible:ring-brand-500 focus-visible:outline-none"
			aria-haspopup="menu"
			aria-expanded={open}
			onclick={() => (open = !open)}
		>
			<MascotAvatar mascot={account.mascot} size={28} decorative />
			<span>@{account.username}</span>
			<ChevronDown
				class={['size-4 text-ink-muted transition-transform', open && 'rotate-180']}
				aria-hidden="true"
			/>
		</button>
		{#if open}
			<div
				class="absolute end-0 top-full z-20 mt-2 min-w-48 rounded-xl border border-ink-muted/15 bg-surface p-1 shadow-lg"
				role="menu"
			>
				<button
					type="button"
					role="menuitem"
					class="flex w-full items-center gap-2 rounded-lg px-3 py-2 text-start text-sm text-ink hover:bg-surface-raised focus-visible:bg-surface-raised focus-visible:outline-none"
					onclick={() => {
						open = false;
						void session.signOut();
					}}
				>
					<LogOut class="size-4 text-ink-muted" aria-hidden="true" />
					{i18n.t.header.signOut}
				</button>
			</div>
		{/if}
	</div>
{/if}
