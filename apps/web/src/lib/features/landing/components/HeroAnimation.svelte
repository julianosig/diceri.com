<script lang="ts">
	import { Volume2 } from '@lucide/svelte';
	// Imported (not in static/) so the build gives them content-hashed names: a new
	// version always gets a new URL and no cache can serve a stale scene.
	import sceneDark from '#lib/assets/hero/scene-dark.webp';
	import sceneLight from '#lib/assets/hero/scene-light.webp';
	import { theme } from '#lib/components/ui/index.ts';
	import { cn } from '#lib/utils/cn.ts';
	import { CYCLE_MS, LANGUAGE_PAIRS, nextPair } from '../language-pairs.ts';

	interface Props {
		class?: string;
	}

	let { class: className }: Props = $props();

	/** Twinkles around Diceri while it translates (positions in % of the banner). */
	const SPARKLES = [
		{ left: '43%', top: '14%', delay: '0s' },
		{ left: '57%', top: '18%', delay: '0.12s' },
		{ left: '41%', top: '80%', delay: '0.24s' },
		{ left: '58%', top: '76%', delay: '0.06s' }
	] as const;

	let index = $state(0);
	const pair = $derived(LANGUAGE_PAIRS[index] ?? LANGUAGE_PAIRS[0]);
	const scene = $derived(theme.current === 'dark' ? 'dark' : 'light');

	// Cycle through language pairs, unless the visitor prefers reduced motion.
	$effect(() => {
		if (matchMedia('(prefers-reduced-motion: reduce)').matches) return;
		const timer = setInterval(() => {
			index = nextPair(index);
		}, CYCLE_MS);
		return () => {
			clearInterval(timer);
		};
	});
</script>

<!--
	The scene (two people, Diceri in the middle) is a light or dark image; the speech is
	HTML on top, so any language pair stays crisp and the banner stays tiny. Timeline:
	the left person says a line → it travels into Diceri → Diceri glows → the same line,
	translated, travels on and is heard by the right person.
-->
<div
	class={cn('banner relative w-full overflow-hidden', className)}
	role="img"
	aria-label="Illustration: one person speaks, Diceri translates in the middle, and the other person hears it in their own language."
>
	<!-- The scene has a transparent background, so this halo shines through around Diceri. -->
	{#key index}
		<span class="halo" aria-hidden="true"></span>
	{/key}
	<img
		src={scene === 'dark' ? sceneDark : sceneLight}
		width="1200"
		height="197"
		alt=""
		class="relative block h-auto w-full"
		fetchpriority="high"
	/>
	{#key `${String(index)}-${scene}`}
		{#each SPARKLES as sparkle (sparkle.left)}
			<span
				class="sparkle"
				style:left={sparkle.left}
				style:top={sparkle.top}
				style:animation-delay={sparkle.delay}
				aria-hidden="true"
			>
				✦
			</span>
		{/each}
		<div class="said bubble" lang={pair.said.lang} aria-hidden="true">
			<span class="label">{pair.said.language}</span>
			<span class="text">{pair.said.text}</span>
		</div>
		<span class="token token-in" lang={pair.said.lang} aria-hidden="true">{pair.said.text}</span>
		<span class="token token-out" lang={pair.heard.lang} aria-hidden="true">{pair.heard.text}</span>
		<div class="heard bubble" lang={pair.heard.lang} aria-hidden="true">
			<span class="label">
				<Volume2 class="inline size-[1.1em]" aria-hidden="true" />
				{pair.heard.language}
			</span>
			<span class="text">{pair.heard.text}</span>
		</div>
	{/key}
</div>

<style>
	/* A multi-step, percentage-positioned timeline Tailwind utilities cannot express. */
	.banner {
		container-type: inline-size;
	}

	.bubble {
		position: absolute;
		top: 7%;
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
		left: 22%;
		transform-origin: left bottom;
		animation: said 6s ease both;
	}

	.heard {
		right: 22%;
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

	/* The line in flight along the dotted path, shrinking into / growing out of Diceri. */
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
		background: var(--color-brand-500);
		animation: travel-out 6s ease-in-out both;
	}

	/* Behind the transparent scene: a violet halo around Diceri while it translates. */
	.halo {
		position: absolute;
		left: 50%;
		top: 50%;
		width: 17cqw;
		aspect-ratio: 1;
		translate: -50% -50%;
		border-radius: 999px;
		background: radial-gradient(
			circle,
			color-mix(in oklch, var(--color-brand-violet) 55%, transparent) 0%,
			transparent 65%
		);
		opacity: 0;
		animation: glow 6s ease both;
	}

	.sparkle {
		position: absolute;
		translate: -50% -50%;
		color: var(--color-gold);
		font-size: min(2cqw, 22px);
		opacity: 0;
		animation: glow 6s ease both;
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
			left: 27%;
			top: 72%;
			opacity: 0;
			scale: 1;
		}
		22% {
			opacity: 1;
		}
		38% {
			left: 47%;
			top: 55%;
			opacity: 1;
			scale: 0.7;
		}
		42%,
		100% {
			left: 50%;
			top: 50%;
			opacity: 0;
			scale: 0.3;
		}
	}

	@keyframes glow {
		0%,
		36% {
			opacity: 0;
		}
		42%,
		54% {
			opacity: 1;
		}
		62%,
		100% {
			opacity: 0;
		}
	}

	@keyframes travel-out {
		0%,
		50% {
			left: 50%;
			top: 50%;
			opacity: 0;
			scale: 0.3;
		}
		54% {
			left: 53%;
			top: 55%;
			opacity: 1;
			scale: 0.7;
		}
		68% {
			left: 73%;
			top: 72%;
			opacity: 1;
			scale: 1;
		}
		72%,
		100% {
			left: 75%;
			top: 72%;
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

	/* Reduced motion: no travel, both sides of the conversation shown at once. */
	@media (prefers-reduced-motion: reduce) {
		.said,
		.heard {
			animation: none;
			opacity: 1;
		}

		.token,
		.halo,
		.sparkle {
			display: none;
		}
	}
</style>
