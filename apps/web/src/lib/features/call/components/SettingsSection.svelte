<script lang="ts">
	import type { Component, Snippet } from 'svelte';

	interface Props {
		title: string;
		icon: Component<{ class?: string }>;
		children: Snippet;
	}

	let { title, icon: Icon, children }: Props = $props();
</script>

<!-- One group of the call settings panel: an icon-led title, then its controls; groups are
	separated by space alone, no rules. -->
<section class="flex flex-col gap-3 not-first-of-type:mt-4">
	<h3 class="flex items-center gap-2 text-xs font-semibold tracking-wider text-ink-muted uppercase">
		<Icon class="size-4 text-brand-500" />
		{title}
	</h3>
	{@render children()}
</section>
