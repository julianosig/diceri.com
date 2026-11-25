<script lang="ts">
	import { Button } from '#lib/components/ui/index.ts';

	let { url }: { url: string } = $props();
	let copied = $state(false);

	async function copy() {
		await navigator.clipboard.writeText(url);
		copied = true;
	}
</script>

<div class="flex w-full max-w-xl items-center gap-2 rounded-tile bg-surface-raised p-2 pl-4">
	<code class="min-w-0 flex-1 truncate text-sm text-ink-muted">{url}</code>
	<Button size="sm" variant="secondary" onclick={copy}>{copied ? 'Copied' : 'Copy link'}</Button>
</div>
<span class="sr-only" aria-live="polite">{copied ? 'Link copied to clipboard' : ''}</span>
