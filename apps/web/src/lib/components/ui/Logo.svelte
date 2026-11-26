<script lang="ts">
	import { cn } from '#lib/utils/cn.ts';

	interface Props {
		/** Show the meaning ("from Latin dicere: to speak") under the wordmark. */
		tagline?: boolean;
		class?: string;
	}

	let { tagline = true, class: className }: Props = $props();
</script>

<!--
	The Diceri lockup as one SVG: the mark (a d20 that is also a speech bubble), the
	wordmark and its meaning. Inline so the text uses the page font and colour.
-->
<svg
	viewBox={tagline ? '0 0 364 100' : '0 0 236 100'}
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
	<text
		x="110"
		y={tagline ? 56 : 66}
		fill="currentColor"
		font-size="50"
		font-weight="700"
		letter-spacing="-1.5"
	>
		Diceri
	</text>
	{#if tagline}
		<text x="112" y="88" fill="currentColor" opacity="0.65" font-size="21" font-style="italic">
			from Latin <tspan font-weight="700">dicere</tspan>: to speak
		</text>
	{/if}
</svg>
