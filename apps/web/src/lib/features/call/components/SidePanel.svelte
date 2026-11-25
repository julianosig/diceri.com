<script lang="ts">
	import type { Snippet } from 'svelte';
	import { X } from '@lucide/svelte';

	interface Props {
		title: string;
		onclose: () => void;
		children: Snippet;
	}

	let { title, onclose, children }: Props = $props();
</script>

<aside
	class="flex w-full flex-col gap-5 overflow-y-auto rounded-tile bg-surface p-5 text-ink sm:w-96"
	aria-label={title}
>
	<header class="flex items-center justify-between">
		<h2 class="text-lg">{title}</h2>
		<button
			type="button"
			class="rounded-full p-2 hover:bg-surface-raised"
			aria-label="Close panel"
			onclick={onclose}
		>
			<X class="size-5" />
		</button>
	</header>
	{@render children()}
</aside>
