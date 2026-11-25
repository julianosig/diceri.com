<script lang="ts">
	import { cn } from '#lib/utils/cn.ts';

	interface Props {
		checked: boolean;
		label: string;
		description?: string;
		disabled?: boolean;
		onchange: (checked: boolean) => void;
		class?: string;
	}

	let {
		checked,
		label,
		description,
		disabled = false,
		onchange,
		class: className
	}: Props = $props();
</script>

<label class={cn('flex cursor-pointer items-start justify-between gap-4', className)}>
	<span class="flex flex-col">
		<span class="text-sm font-medium">{label}</span>
		{#if description}<span class="text-xs text-ink-muted">{description}</span>{/if}
	</span>
	<button
		type="button"
		role="switch"
		aria-checked={checked}
		aria-label={label}
		{disabled}
		onclick={() => {
			onchange(!checked);
		}}
		class={cn(
			'relative h-6 w-11 shrink-0 rounded-full transition-colors disabled:opacity-50',
			checked ? 'bg-brand-500' : 'bg-ink-muted/30'
		)}
	>
		<span
			class={cn(
				'absolute top-0.5 left-0.5 size-5 rounded-full bg-white shadow transition-transform',
				checked && 'translate-x-5'
			)}
		></span>
	</button>
</label>
