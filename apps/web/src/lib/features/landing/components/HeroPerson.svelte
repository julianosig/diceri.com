<script lang="ts">
	import { cn } from '#lib/utils/cn.ts';

	interface Props {
		/** The traced SVG drawing of the person. */
		src: string;
		/** `talk`: a moving mouth and voice waves; `listen`: voice pulsing in the headset. */
		cue: 'talk' | 'listen';
		/** Where the cue sits on the drawing: the mouth or the ear cup, in % of its box. */
		x: number;
		y: number;
		/** Changes once per banner cycle, replaying the cue in step with the speech. */
		cycle: number;
		class?: string;
	}

	let { src, cue, x, y, cycle, class: className }: Props = $props();
</script>

<!--
	One person of the home banner. The drawing stays still; the voice is drawn on top in
	code, so the banner shows speech being translated, not text.
-->
<div class={cn('absolute bottom-0 h-[94%]', className)}>
	<img {src} alt="" class="h-full w-auto" />
	{#key cycle}
		{#if cue === 'talk'}
			<span class="cue talk" style:left="{x}%" style:top="{y}%" aria-hidden="true">
				<span class="mouth"></span>
			</span>
			<svg
				class="cue talk waves"
				style:left="{x + 9}%"
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

	/* An open mouth over the drawn smile, chattering while the person speaks. */
	.mouth {
		display: block;
		width: 2.4cqw;
		aspect-ratio: 1.4;
		border: 0.25cqw solid #1a0d10;
		border-radius: 30% 30% 50% 50% / 25% 25% 75% 75%;
		background: radial-gradient(ellipse at 50% 95%, #e86a7a 0 38%, #5a1d26 40%);
		transform-origin: top;
		animation: chatter 0.2s ease-in-out infinite alternate;
	}

	.waves {
		width: 2.6cqw;
		fill: none;
		stroke: var(--color-brand-violet);
		stroke-width: 3.5;
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
			opacity: 0.2;
		}
		40% {
			opacity: 1;
		}
		100% {
			opacity: 0.2;
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

	@media (prefers-reduced-motion: reduce) {
		.cue {
			display: none;
		}
	}
</style>
