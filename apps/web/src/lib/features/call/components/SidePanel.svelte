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
		<h2 class="text-lg font-semibold">{title}</h2>
		<button
			type="button"
			class="-me-2 rounded-full p-2 text-ink-muted hover:bg-surface-raised hover:text-ink"
			aria-label="Close panel"
			onclick={onclose}
		>
			<X class="size-5" />
		</button>
	</header>
	{@render children()}
</aside>
