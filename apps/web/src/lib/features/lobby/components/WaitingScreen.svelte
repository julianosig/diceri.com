<script lang="ts">
	import { Button } from '#lib/components/ui/index.ts';
	import type { KnockStatus } from '../guest-knock.svelte.ts';

	interface Props {
		status: KnockStatus;
		error?: string;
		oncancel: () => void;
	}

	let { status, error, oncancel }: Props = $props();
</script>

<section class="flex flex-col items-center gap-4 px-4 text-center" aria-live="polite">
	{#if status === 'denied'}
		<h1 class="text-2xl">You can't join this meeting</h1>
		<p class="text-ink-muted">The host declined your request.</p>
	{:else if status === 'error'}
		<h1 class="text-2xl">Something went wrong</h1>
		<p role="alert" class="text-red-600">{error}</p>
		<Button variant="secondary" onclick={oncancel}>Back</Button>
	{:else}
		<div
			class="size-10 animate-spin rounded-full border-4 border-brand-500 border-t-transparent"
			aria-hidden="true"
		></div>
		<h1 class="text-2xl">Asking to be let in…</h1>
		<p class="text-ink-muted">You'll join the meeting when someone lets you in.</p>
		<Button variant="ghost" onclick={oncancel}>Cancel</Button>
	{/if}
</section>
