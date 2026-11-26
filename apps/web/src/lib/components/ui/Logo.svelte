<script lang="ts">
	import { cn } from '#lib/utils/cn.ts';
	import { MEANING_PATH, WORDMARK_PATH } from './logo-paths.ts';

	interface Props {
		/** Show the meaning ("from Latin dicere: to speak") under the wordmark. */
		tagline?: boolean;
		class?: string;
	}

	let { tagline = true, class: className }: Props = $props();
</script>

<!--
	The Diceri lockup as one SVG: the mark (a d20 that is also a speech bubble), the
	wordmark and its meaning, all outlines (no font needed). Text follows the theme's
	ink colour; the facet gaps follow the surface colour.
-->
<svg
	viewBox={tagline ? '0 0 390 100' : '0 0 250 100'}
	class={cn('h-10 w-auto text-ink', className)}
	role="img"
	aria-label={tagline ? 'Diceri, from Latin dicere: to speak' : 'Diceri'}
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
	<path fill="currentColor" d={WORDMARK_PATH} transform={tagline ? undefined : 'translate(0 10)'} />
	{#if tagline}
		<path fill="currentColor" fill-opacity="0.65" d={MEANING_PATH} />
	{/if}
</svg>
