<script lang="ts">
	import { cn } from '#lib/utils/cn.ts';

	interface Props {
		/** The traced SVG drawing of the person. */
		src: string;
		/** The drawing's intrinsic size, so its box is reserved before it loads. */
		width: number;
		height: number;
		/** `talk`: a moving mouth and voice waves; `listen`: voice pulsing in the headset. */
		cue: 'talk' | 'listen';
		/** Where the cue sits on the drawing: the mouth or the ear cup, in % of its box. */
		x: number;
		y: number;
		/** Changes once per banner cycle, replaying the cue in step with the speech. */
		cycle: number;
		class?: string;
	}

	let { src, width, height, cue, x, y, cycle, class: className }: Props = $props();
</script>

<!--
	One person of the home banner. The drawing stays still; the voice is drawn on top in
	code, so the banner shows speech being translated, not text.
-->
<!-- Smaller on phones, so the Diceri mark between them can be larger. -->
<div class={cn('absolute bottom-0 h-[70%] sm:h-[94%]', className)}>
	<img {src} {width} {height} alt="" decoding="async" fetchpriority="low" class="h-full w-auto" />
	{#key cycle}
		{#if cue === 'talk'}
			<span class="cue mouth-spot" style:left="{x}%" style:top="{y}%" aria-hidden="true">
				<svg class="smile" viewBox="0 0 24 10"><path d="M 2 2 Q 12 11 22 2" /></svg>
				<span class="mouth"></span>
			</span>
			<svg
				class="cue talk waves"
				style:left="{x + 16}%"
				style:top="{y - 1}%"
				viewBox="0 0 30 40"
				aria-hidden="true"
			>
				<path d="M 4 14 Q 9 20 4 26" />
				<path d="M 12 8 Q 21 20 12 32" />
				<path d="M 20 2 Q 33 20 20 38" />
			</svg>
		{:else}
			<span class="cue listen" style:left="{x}%" style:top="{y}%" aria-hidden="true">
				<span class="ring"></span>
				<span class="ring"></span>
			</span>
		{/if}
	{/key}
</div>

<style>
	/* Cue windows follow the 6 s speech timeline in HeroSpeech (said 2–34 %, heard 66–94 %). */
	.cue {
		position: absolute;
		translate: -50% -50%;
		opacity: 0;
	}

	.talk {
		animation: talking 6s ease both;
	}

	.listen {
		width: 11%;
		aspect-ratio: 1;
		animation: listening 6s ease both;
	}

	/* The drawing has no mouth: a resting smile, swapped for an open, chattering mouth while
	   the person speaks. Both are centred on the same spot. */
	.mouth-spot {
		opacity: 1;
	}

	.smile,
	.mouth {
		position: absolute;
		left: 50%;
		top: 50%;
		translate: -50% -50%;
	}

	.smile {
		width: 1.7cqw;
		fill: none;
		stroke: #181410;
		stroke-width: 2.4;
		stroke-linecap: round;
		animation: resting 6s ease both;
	}

	.mouth {
		display: block;
		width: 2cqw;
		aspect-ratio: 1.4;
		border: 0.25cqw solid #1a0d10;
		border-radius: 30% 30% 50% 50% / 25% 25% 75% 75%;
		background: radial-gradient(ellipse at 50% 95%, #e86a7a 0 38%, #5a1d26 40%);
		transform-origin: top;
		animation:
			talking 6s ease both,
			chatter 0.2s ease-in-out infinite alternate;
	}

	@keyframes resting {
		0%,
		2% {
			opacity: 1;
		}
		5%,
		32% {
			opacity: 0;
		}
		36%,
		100% {
			opacity: 1;
		}
	}

	.waves {
		width: 2.6cqw;
		fill: none;
		/* Violet pulled towards the ink: deep on light, bright lilac on dark, always contrasted. */
		stroke: color-mix(in oklch, var(--color-brand-violet) 45%, var(--color-ink));
		stroke-width: 4;
		stroke-linecap: round;
	}

	.waves path {
		animation: wave 0.9s ease-out infinite;
	}

	.waves path:nth-child(2) {
		animation-delay: 0.15s;
	}

	.waves path:nth-child(3) {
		animation-delay: 0.3s;
	}

	/* Voice arriving in the headset: rings that keep spreading from the ear cup. */
	.ring {
		position: absolute;
		inset: 0;
		border: 0.45cqw solid var(--color-brand-500);
		border-radius: 999px;
		animation: ring 1s ease-out infinite;
	}

	.ring + .ring {
		animation-delay: 0.5s;
	}

	@keyframes talking {
		0%,
		2% {
			opacity: 0;
		}
		5%,
		32% {
			opacity: 1;
		}
		36%,
		100% {
			opacity: 0;
		}
	}

	@keyframes listening {
		0%,
		66% {
			opacity: 0;
		}
		70%,
		92% {
			opacity: 1;
		}
		96%,
		100% {
			opacity: 0;
		}
	}

	@keyframes chatter {
		from {
			scale: 1 0.35;
		}
		to {
			scale: 1 1;
		}
	}

	@keyframes wave {
		0% {
			opacity: 0.5;
		}
		40% {
			opacity: 1;
		}
		100% {
			opacity: 0.5;
		}
	}

	@keyframes ring {
		from {
			opacity: 1;
			scale: 0.9;
		}
		to {
			opacity: 0;
			scale: 2.2;
		}
	}

	/* Reduced motion: only the resting smile, so the face is never left without a mouth. */
	@media (prefers-reduced-motion: reduce) {
		.cue:not(.mouth-spot),
		.mouth {
			display: none;
		}

		.smile {
			animation: none;
		}
	}
</style>
