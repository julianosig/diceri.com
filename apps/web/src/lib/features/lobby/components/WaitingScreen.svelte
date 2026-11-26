<script lang="ts">
	import { Button } from '#lib/components/ui/index.ts';
	import { i18n } from '#lib/i18n/index.ts';
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
		<h1 class="text-2xl">{i18n.t.lobby.cantJoin}</h1>
		<p class="text-ink-muted">{i18n.t.lobby.declined}</p>
	{:else if status === 'error'}
		<h1 class="text-2xl">{i18n.t.lobby.somethingWrong}</h1>
		<p role="alert" class="text-red-600">{error}</p>
		<Button variant="secondary" onclick={oncancel}>{i18n.t.lobby.back}</Button>
	{:else}
		<div
			class="size-10 animate-spin rounded-full border-4 border-brand-500 border-t-transparent"
			aria-hidden="true"
		></div>
		<h1 class="text-2xl">{i18n.t.lobby.asking}</h1>
		<p class="text-ink-muted">{i18n.t.lobby.willJoin}</p>
		<Button variant="ghost" onclick={oncancel}>{i18n.t.lobby.cancel}</Button>
	{/if}
</section>
