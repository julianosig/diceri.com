<script lang="ts">
	import { Check, Copy, X } from '@lucide/svelte';
	import { Button } from '#lib/components/ui/index.ts';

	let { url, onclose }: { url: string; onclose: () => void } = $props();
	let copied = $state(false);

	async function copy() {
		await navigator.clipboard.writeText(url);
		copied = true;
	}
</script>

<section
	class="flex w-full max-w-sm flex-col gap-3 rounded-tile bg-surface p-5 text-ink shadow-xl"
	aria-label="Meeting details"
>
	<header class="flex items-start justify-between">
		<h2 class="text-lg">Your meeting's ready</h2>
		<button
			type="button"
			class="-m-1 rounded-full p-1 hover:bg-surface-raised"
			aria-label="Close"
			onclick={onclose}
		>
			<X class="size-5" />
		</button>
	</header>
	<p class="text-sm text-ink-muted">
		Share this link with the person you want to talk to. You'll be asked to let them in.
	</p>
	<div class="flex items-center gap-2 rounded-lg bg-surface-raised p-2 pl-3">
		<code class="min-w-0 flex-1 truncate text-sm">{url}</code>
		<Button size="sm" variant="ghost" aria-label="Copy meeting link" onclick={copy}>
			{#if copied}<Check class="size-4" />{:else}<Copy class="size-4" />{/if}
		</Button>
	</div>
	<span class="sr-only" aria-live="polite">{copied ? 'Link copied' : ''}</span>
</section>
