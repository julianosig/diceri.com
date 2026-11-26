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

<img
	src={mascotUrl(mascot)}
	alt={decorative ? '' : `${mascotLabel(mascot)} mascot`}
	width={size}
	height={size}
	loading="lazy"
	decoding="async"
	class={cn('shrink-0 rounded-full object-cover', className)}
/>
