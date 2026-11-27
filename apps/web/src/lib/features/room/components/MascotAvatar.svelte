<script lang="ts">
	import { cn } from '#lib/utils/cn.ts';
	import { mascotLabel, mascotUrl } from '../mascots.ts';

	interface Props {
		mascot: string;
		/** Pixel size of the square avatar. */
		size?: number;
		/** Decorative when the room is already named next to it. */
		decorative?: boolean;
		class?: string;
	}

	let { mascot, size = 40, decorative = false, class: className }: Props = $props();
</script>

<!--
	Transparent animated WebP at 128 px, whose edges are smoothly antialiased (a 250 px
	version had near 1-bit edges that looked jagged); GIF (1-bit transparency) as a fallback.
-->
<picture class="contents">
	<source srcset={mascotUrl(mascot, 'webp')} type="image/webp" />
	<img
		src={mascotUrl(mascot, 'gif')}
		alt={decorative ? '' : `${mascotLabel(mascot)} mascot`}
		width={size}
		height={size}
		loading="lazy"
		decoding="async"
		class={cn('shrink-0 object-contain', className)}
	/>
</picture>
