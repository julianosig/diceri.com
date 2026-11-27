<script lang="ts">
	import type { VideoTrack } from 'livekit-client';
	import { cn } from '#lib/utils/cn.ts';

	interface Props {
		track: VideoTrack;
		/** Your own camera is shown mirrored, like a mirror (others see it the right way). */
		mirrored?: boolean;
		class?: string;
	}

	let { track, mirrored = false, class: className }: Props = $props();
	let element = $state<HTMLVideoElement>();

	$effect(() => {
		if (!element) return;
		const video = element;
		track.attach(video);
		return () => {
			track.detach(video);
		};
	});
</script>

<!-- A participant's camera; muted because sound comes from the audio tracks. -->
<video
	bind:this={element}
	class={cn('size-full object-cover', mirrored && '-scale-x-100', className)}
	autoplay
	playsinline
	muted
></video>
