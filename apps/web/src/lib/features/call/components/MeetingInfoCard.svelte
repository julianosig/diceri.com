<script lang="ts">
	import { i18n } from '#lib/i18n/index.ts';
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
	aria-label={i18n.t.call.meetingDetails}
>
	<header class="flex items-start justify-between">
		<h2 class="text-lg">{i18n.t.call.meetingReady}</h2>
		<button
			type="button"
			class="-m-1 rounded-full p-1 hover:bg-surface-raised"
			aria-label={i18n.t.call.close}
			onclick={onclose}
		>
			<X class="size-5" />
		</button>
	</header>
	<p class="text-sm text-ink-muted">
		{i18n.t.call.shareLink}
	</p>
	<div class="flex items-center gap-2 rounded-lg bg-surface-raised p-2 pl-3">
		<code class="min-w-0 flex-1 truncate text-sm">{url}</code>
		<Button size="sm" variant="ghost" aria-label={i18n.t.call.copyLink} onclick={copy}>
			{#if copied}<Check class="size-4" />{:else}<Copy class="size-4" />{/if}
		</Button>
	</div>
	<span class="sr-only" aria-live="polite">{copied ? i18n.t.call.linkCopied : ''}</span>
</section>
