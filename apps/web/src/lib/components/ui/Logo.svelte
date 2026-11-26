<script lang="ts">
	import logo from '#lib/assets/favicon.svg';
	import { cn } from '#lib/utils/cn.ts';

	interface Props {
		/** Show the "Diceri" wordmark next to the mark. */
		wordmark?: boolean;
		class?: string;
	}

	let { wordmark = true, class: className }: Props = $props();
</script>

<!-- The mark: a d20 that is also a speech bubble (Latin "dicere", to say). -->
<span class={cn('inline-flex items-center gap-2', className)}>
	<img src={logo} alt={wordmark ? '' : 'Diceri'} class="size-8" />
	{#if wordmark}
		<span class="text-xl font-semibold tracking-tight">Diceri</span>
	{/if}
</span>
