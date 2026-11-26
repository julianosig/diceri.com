<script lang="ts" generics="T extends string">
	import type { Component } from 'svelte';
	import { cn } from '#lib/utils/cn.ts';

	interface Option {
		value: T;
		label: string;
		icon?: Component<{ class?: string }>;
	}

	interface Props {
		options: readonly Option[];
		value: T;
		/** Names the group for assistive technology. */
		label: string;
		onchange: (value: T) => void;
		class?: string;
	}

	let { options, value, label, onchange, class: className }: Props = $props();
</script>

<!--
	One choice among a few, as a track with the selected option raised. With icons, each
	label sits under its icon so longer labels stay on one line. Neutral on purpose:
	brand blue stays for primary actions and switches that are on.
-->
<div
	class={cn('flex gap-1 rounded-xl bg-surface-raised p-1', className)}
	role="radiogroup"
	aria-label={label}
>
	{#each options as option (option.value)}
		{@const Icon = option.icon}
		<button
			type="button"
			role="radio"
			aria-checked={value === option.value}
			class={cn(
				'flex min-h-9 flex-1 items-center justify-center rounded-lg px-1.5 font-medium whitespace-nowrap',
				Icon ? 'flex-col gap-1 py-2 text-xs' : 'gap-1.5 text-sm',
				'text-ink-muted transition-colors hover:text-ink focus-visible:ring-2 focus-visible:ring-brand-500 focus-visible:outline-none aria-checked:bg-surface aria-checked:text-ink aria-checked:shadow-sm'
			)}
			onclick={() => {
				onchange(option.value);
			}}
		>
			{#if Icon}<Icon class="size-5 shrink-0" />{/if}
			{option.label}
		</button>
	{/each}
</div>
