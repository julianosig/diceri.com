<script lang="ts">
	import { Button } from '#lib/components/ui/index.ts';
	import type { AccountSession } from '../account-session.svelte.ts';

	interface Props {
		session: AccountSession;
	}

	let { session }: Props = $props();
	let copied = $state(false);

	async function copy(key: string) {
		await navigator.clipboard.writeText(key);
		copied = true;
	}
</script>

{#if session.newKey}
	{@const key = session.newKey}
	<section
		aria-labelledby="account-key-title"
		class="flex w-full max-w-md flex-col gap-2 rounded-tile border border-gold/60 bg-gold/10 p-4 text-left"
	>
		<h2 id="account-key-title" class="text-sm font-semibold">
			Save your sign-in code for @{session.username}
		</h2>
		<p class="text-xs text-ink-muted">
			It is the only way to use this username on another device. We show it once.
		</p>
		<code
			class="rounded-lg bg-surface px-3 py-2 font-mono text-sm break-all"
			data-testid="account-key"
		>
			{key}
		</code>
		<div class="flex gap-2">
			<Button size="sm" variant="secondary" onclick={() => copy(key)}>
				{copied ? 'Copied' : 'Copy code'}
			</Button>
			<Button
				size="sm"
				variant="ghost"
				onclick={() => {
					session.dismissKey();
				}}
			>
				I saved it
			</Button>
		</div>
	</section>
{/if}
