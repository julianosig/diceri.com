<script lang="ts">
	import { Moon, Sun } from '@lucide/svelte';
	import { cn } from '#lib/utils/cn.ts';
	import { theme } from './theme.svelte.ts';

	interface Props {
		class?: string;
	}

	let { class: className }: Props = $props();
	const next = $derived(theme.current === 'dark' ? 'light' : 'dark');
</script>

<button
	type="button"
	class={cn(
		'inline-flex size-9 items-center justify-center rounded-full text-ink-muted transition-colors hover:bg-surface-raised hover:text-ink focus-visible:ring-2 focus-visible:ring-brand-500 focus-visible:outline-none',
		className
	)}
	aria-label={`Switch to ${next} theme`}
	title={`Switch to ${next} theme`}
	onclick={() => {
		theme.toggle();
	}}
>
	{#if theme.current === 'dark'}
		<Sun class="size-5" aria-hidden="true" />
	{:else}
		<Moon class="size-5" aria-hidden="true" />
	{/if}
</button>
