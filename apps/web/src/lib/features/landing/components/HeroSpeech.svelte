<script lang="ts">
	import { AudioLines, Volume2 } from '@lucide/svelte';
	import type { LanguagePair } from '../language-pairs.ts';

	interface Props {
		pair: LanguagePair;
	}

	let { pair }: Props = $props();
</script>

<!--
	The speech layer of the home banner, positioned in its container: the line said on
	the left, the pill that carries it into Diceri, the translated pill that leaves, and
	the line heard on the right. Remount (key) it to replay the 6 s timeline.
-->
<div class="said bubble" lang={pair.said.lang} aria-hidden="true">
	<span class="label">{pair.said.language}</span>
	<span class="text">{pair.said.text}</span>
</div>
<span class="token token-in" lang={pair.said.lang} aria-hidden="true">
	<AudioLines class="inline size-[1.1em]" aria-hidden="true" />
	{pair.said.text}
</span>
<span class="token token-out" lang={pair.heard.lang} aria-hidden="true">
	<AudioLines class="inline size-[1.1em]" aria-hidden="true" />
	{pair.heard.text}
</span>
<div class="heard bubble" lang={pair.heard.lang} aria-hidden="true">
	<span class="label">
		<Volume2 class="inline size-[1.1em]" aria-hidden="true" />
		{pair.heard.language}
	</span>
	<span class="text">{pair.heard.text}</span>
</div>

<style>
	/* A multi-step, percentage-positioned timeline Tailwind utilities cannot express. */
	.bubble {
		position: absolute;
		top: 4%;
		display: flex;
		flex-direction: column;
		gap: 0.1em;
		max-width: 26%;
		padding: 0.45em 0.8em;
		border: 2px solid var(--color-ink);
		border-radius: 1em;
		background: var(--color-surface);
		color: var(--color-ink);
		/* Scales with the banner only, so bubbles keep their place at every width. */
		font-size: min(2.2cqw, 24px);
		line-height: 1.15;
		opacity: 0;
		box-shadow: 0 2px 0 var(--color-ink);
	}

	.said {
		left: 19%;
		transform-origin: left bottom;
		animation: said 6s ease both;
	}

	.heard {
		right: 19%;
		text-align: right;
		align-items: flex-end;
		transform-origin: right bottom;
		animation: heard 6s ease both;
	}

	.label {
		font-size: 0.55em;
		font-weight: 600;
		letter-spacing: 0.02em;
		color: var(--color-brand-violet);
		text-transform: uppercase;
	}

	.text {
		font-weight: 800;
		white-space: nowrap;
	}

	/* The line in flight along the dotted link, shrinking into / growing out of Diceri. */
	.token {
		position: absolute;
		padding: 0.15em 0.6em;
		border-radius: 999px;
		background: var(--color-brand-violet);
		color: white;
		font-size: min(1.4cqw, 15px);
		font-weight: 700;
		white-space: nowrap;
		opacity: 0;
		translate: -50% -50%;
	}

	.token-in {
		animation: travel-in 6s ease-in-out both;
	}

	.token-out {
		background: var(--color-brand-600);
		animation: travel-out 6s ease-in-out both;
	}

	@keyframes said {
		0% {
			opacity: 0;
			scale: 0.6;
		}
		6%,
		34% {
			opacity: 1;
			scale: 1;
		}
		42%,
		100% {
			opacity: 0;
			scale: 1;
		}
	}

	@keyframes travel-in {
		0%,
		18% {
			left: 24%;
			top: 61%;
			opacity: 0;
			scale: 1;
		}
		22% {
			opacity: 1;
		}
		38% {
			left: 42%;
			top: 38%;
			opacity: 1;
			scale: 0.7;
		}
		42%,
		100% {
			left: 46%;
			top: 38%;
			opacity: 0;
			scale: 0.3;
		}
	}

	@keyframes travel-out {
		0%,
		50% {
			left: 54%;
			top: 38%;
			opacity: 0;
			scale: 0.3;
		}
		54% {
			left: 57%;
			top: 38%;
			opacity: 1;
			scale: 0.7;
		}
		68% {
			left: 75%;
			top: 61%;
			opacity: 1;
			scale: 1;
		}
		72%,
		100% {
			left: 76%;
			top: 61%;
			opacity: 0;
		}
	}

	@keyframes heard {
		0%,
		66% {
			opacity: 0;
			scale: 0.6;
		}
		72%,
		94% {
			opacity: 1;
			scale: 1;
		}
		100% {
			opacity: 0;
			scale: 1;
		}
	}

	/* Reduced motion: both sides of the conversation shown at once, nothing in flight. */
	@media (prefers-reduced-motion: reduce) {
		.said,
		.heard {
			animation: none;
			opacity: 1;
		}

		.token {
			display: none;
		}
	}
</style>
