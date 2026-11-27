<script lang="ts">
	// Vector art (traced from Codex drawings), imported so the build hashes their names:
	// crisp at any size, one asset for both themes, and never served stale from a cache.
	import leftPerson from '#lib/assets/hero/left-person.svg';
	import rightPerson from '#lib/assets/hero/right-person.svg';
	import { Logo } from '#lib/components/ui/index.ts';
	import { i18n } from '#lib/i18n/index.ts';
	import { cn } from '#lib/utils/cn.ts';
	import { CYCLE_MS, LANGUAGE_PAIRS, nextPair } from '../language-pairs.ts';
	import HeroPerson from './HeroPerson.svelte';
	import HeroSpeech from './HeroSpeech.svelte';

	interface Props {
		class?: string;
	}

	let { class: className }: Props = $props();

	/** Where the voice shows on each drawing (% of its box): the speaker's mouth, the listener's ear cup. */
	const LEFT_MOUTH = { x: 63.9, y: 48.1 } as const;
	const RIGHT_EAR = { x: 58.6, y: 43 } as const;

	/** Twinkles around the mark while Diceri translates (positions in % of the banner). */
	const SPARKLES = [
		{ left: '44%', top: '10%', delay: '0s' },
		{ left: '56%', top: '13%', delay: '0.12s' },
		{ left: '43%', top: '56%', delay: '0.24s' },
		{ left: '57%', top: '53%', delay: '0.06s' }
	] as const;

	let index = $state(0);
	const pair = $derived(LANGUAGE_PAIRS[index] ?? LANGUAGE_PAIRS[0]);

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
	Everything is vector or code: two traced SVG people, the real Diceri mark (wordmark
	under it) in the middle, dotted links that flow, and HTML speech on top. Timeline:
	the left person says a line → it travels into Diceri → Diceri glows → the same line,
	translated, travels on and is heard by the right person.
-->
<div
	class={cn('banner relative aspect-[9/2] w-full sm:aspect-[6/1]', className)}
	role="img"
	aria-label={i18n.t.hero.illustration}
	dir="ltr"
>
	<svg
		class="absolute inset-0 h-full w-full"
		viewBox="0 0 1200 200"
		preserveAspectRatio="none"
		aria-hidden="true"
	>
		<path class="link link-in" d="M 285 122 Q 400 60 525 78" />
		<path class="link link-out" d="M 675 78 Q 800 60 915 122" />
	</svg>
	<HeroPerson
		src={leftPerson}
		width={1403}
		height={960}
		cue="talk"
		{...LEFT_MOUTH}
		cycle={index}
		class="left-0"
	/>
	<HeroPerson
		src={rightPerson}
		width={1547}
		height={960}
		cue="listen"
		{...RIGHT_EAR}
		cycle={index}
		class="right-0"
	/>

	{#key index}
		<span class="halo" aria-hidden="true"></span>
	{/key}
	<div class="mark absolute top-1/2 left-1/2 h-[96%] -translate-x-1/2 -translate-y-1/2 sm:h-[82%]">
		<Logo layout="stacked" class="h-full" />
	</div>

	{#key index}
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
		<HeroSpeech {pair} />
	{/key}
</div>

<style>
	/* A multi-step, percentage-positioned timeline Tailwind utilities cannot express. */
	.banner {
		container-type: inline-size;
	}

	/* Dotted links that keep flowing from speaker to Diceri to listener. */
	.link {
		fill: none;
		stroke: var(--color-ink-muted);
		stroke-opacity: 0.55;
		/* The SVG stretches with the banner; keep dots round and evenly spaced in pixels. */
		vector-effect: non-scaling-stroke;
		stroke-width: 3.5px;
		stroke-linecap: round;
		stroke-dasharray: 0.1px 11px;
		animation: flow 1.2s linear infinite;
	}

	@keyframes flow {
		to {
			stroke-dashoffset: -11.1px;
		}
	}

	/* Behind the mark: a violet halo while Diceri translates. */
	.halo {
		position: absolute;
		left: 50%;
		top: 36%;
		width: 15cqw;
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

	.mark {
		animation: pulse 6s ease both;
	}

	.sparkle {
		position: absolute;
		translate: -50% -50%;
		color: var(--color-gold);
		font-size: min(2cqw, 22px);
		opacity: 0;
		animation: glow 6s ease both;
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

	@keyframes pulse {
		0%,
		38% {
			scale: 1;
		}
		46% {
			scale: 1.08;
		}
		54%,
		100% {
			scale: 1;
		}
	}

	/* Reduced motion: nothing moves (HeroSpeech shows both lines at once). */
	@media (prefers-reduced-motion: reduce) {
		.link,
		.mark {
			animation: none;
		}

		.halo,
		.sparkle {
			display: none;
		}
	}
</style>
