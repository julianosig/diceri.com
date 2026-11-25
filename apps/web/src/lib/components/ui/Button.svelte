<script lang="ts">
	import type { Snippet } from 'svelte';
	import type { HTMLButtonAttributes } from 'svelte/elements';
	import { cn } from '#lib/utils/cn.ts';
	import { buttonVariants, type ButtonVariants } from './button-variants.ts';

	type Props = HTMLButtonAttributes &
		ButtonVariants & {
			children: Snippet;
		};

	let { variant, size, type = 'button', class: className, children, ...rest }: Props = $props();
</script>

<button {type} class={cn(buttonVariants({ variant, size }), className)} {...rest}>
	{@render children()}
</button>
