<script lang="ts">
	import { cn } from '#lib/utils/cn.ts';

	interface Props {
		class?: string;
	}

	let { class: className }: Props = $props();

	/** Intrinsic size of the assets in `static/hero/` (reserves space, no layout shift). */
	const WIDTH = 960;
	const HEIGHT = 640;
</script>

<!--
	Two people on opposite sides of the world talk through Diceri: "Olá!" goes in,
	"こんにちは!" comes out. Animated WebP first, GIF as fallback, a still frame for
	people who prefer reduced motion.
-->
<picture class={cn('block w-full', className)}>
	<source
		srcset="/hero/diceri-hero-still.webp"
		type="image/webp"
		media="(prefers-reduced-motion: reduce)"
	/>
	<source srcset="/hero/diceri-hero.webp" type="image/webp" />
	<img
		src="/hero/diceri-hero.gif"
		width={WIDTH}
		height={HEIGHT}
		alt="A person in Brazil says “Olá!” at their desk; Diceri translates in the middle, and a person in Japan hears “こんにちは!” in their own language."
		class="h-auto w-full rounded-tile"
		fetchpriority="high"
		decoding="async"
	/>
</picture>
