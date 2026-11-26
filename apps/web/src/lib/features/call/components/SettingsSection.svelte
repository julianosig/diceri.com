<script lang="ts">
	import type { Component, Snippet } from 'svelte';

	interface Props {
		title: string;
		icon: Component<{ class?: string }>;
		children: Snippet;
	}

	let { title, icon: Icon, children }: Props = $props();
</script>

<!-- One group of the call settings panel: an icon-led title, then its controls. -->
<section
	class="flex flex-col gap-3 border-t border-ink-muted/15 pt-5 first-of-type:border-t-0 first-of-type:pt-0"
>
	<h3 class="flex items-center gap-2 text-xs font-semibold tracking-wider text-ink-muted uppercase">
		<Icon class="size-4 text-brand-500" />
		{title}
	</h3>
	{@render children()}
</section>
