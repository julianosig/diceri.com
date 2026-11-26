<script lang="ts">
	import { cn } from '#lib/utils/cn.ts';
	import { MEANING_PATH, WORDMARK_PATH } from './logo-paths.ts';

	interface Props {
		/** Show the meaning ("from Latin dicere: to speak") under the wordmark. */
		tagline?: boolean;
		/** `row`: wordmark beside the mark; `stacked`: wordmark centred under it. */
		layout?: 'row' | 'stacked';
		class?: string;
	}

	let { tagline = true, layout = 'row', class: className }: Props = $props();

	/**
	 * The wordmark outline sits at x 113–243, y 18–57 in mark units; stacked, it moves
	 * under the mark (x 5–97) and is centred on it.
	 */
	const STACKED_WORDMARK = 'translate(-127.9 90.4)';
	const viewBox = $derived(
		layout === 'stacked' ? '-16 0 132 150' : tagline ? '0 0 390 100' : '0 0 250 100'
	);
</script>

<!--
	The Diceri lockup as one SVG: the mark (a d20 that is also a speech bubble), the
	wordmark and its meaning, all outlines (no font needed). Text follows the theme's
	ink colour; the facet gaps follow the surface colour.
-->
<svg
	{viewBox}
	class={cn('h-10 w-auto text-ink', className)}
	role="img"
	aria-label={tagline && layout === 'row' ? 'Diceri, from Latin dicere: to speak' : 'Diceri'}
>
	<g stroke="var(--color-surface)" stroke-width="3" stroke-linejoin="round">
		<polygon points="50,4 10,27 28,34" fill="#7c3aed" />
		<polygon points="50,4 28,34 72,34" fill="#2563eb" />
		<polygon points="50,4 72,34 90,27" fill="#7c3aed" />
		<polygon points="10,27 10,70 28,34" fill="#2563eb" />
		<polygon points="28,34 72,34 50,64" fill="#7c3aed" />
		<polygon points="90,27 72,34 90,70" fill="#2563eb" />
		<polygon points="10,70 28,34 50,64" fill="#2563eb" />
		<polygon points="90,70 72,34 50,64" fill="#2563eb" />
		<polygon points="10,70 50,64 50,94" fill="#7c3aed" />
		<polygon points="90,70 50,64 50,94" fill="#7c3aed" />
		<polygon points="10,70 32,84 5,97" fill="#7c3aed" />
	</g>
	<path
		fill="currentColor"
		d={WORDMARK_PATH}
		transform={layout === 'stacked' ? STACKED_WORDMARK : tagline ? undefined : 'translate(0 10)'}
	/>
	{#if tagline && layout === 'row'}
		<path fill="currentColor" fill-opacity="0.65" d={MEANING_PATH} />
	{/if}
</svg>
