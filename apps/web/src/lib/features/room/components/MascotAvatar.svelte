<script lang="ts">
	import { cn } from '#lib/utils/cn.ts';
	import { MASCOT_WEBP_WIDTHS, mascotLabel, mascotUrl } from '../mascots.ts';

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
	Transparent animated WebP in two sizes, so the browser takes the sharp 250 px file only
	when the avatar is large or the screen dense; GIF (1-bit transparency) as a fallback.
-->
<picture class="contents">
	<source
		srcset="{mascotUrl(mascot, 'webp')} {MASCOT_WEBP_WIDTHS.webp}w, {mascotUrl(
			mascot,
			'webp-2x'
		)} {MASCOT_WEBP_WIDTHS['webp-2x']}w"
		sizes="{size}px"
		type="image/webp"
	/>
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
