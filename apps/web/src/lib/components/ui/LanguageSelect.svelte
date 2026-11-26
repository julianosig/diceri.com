<script lang="ts">
	import { Languages } from '@lucide/svelte';
	import { i18n, LOCALES, type Locale } from '#lib/i18n/index.ts';
	import { cn } from '#lib/utils/cn.ts';

	interface Props {
		class?: string;
	}

	let { class: className }: Props = $props();
</script>

<!-- The interface language; each option is written in its own language. -->
<label
	class={cn(
		'relative inline-flex h-9 items-center gap-1.5 rounded-full px-2 text-sm text-ink-muted transition-colors focus-within:ring-2 focus-within:ring-brand-500 hover:bg-surface-raised hover:text-ink',
		className
	)}
	title={i18n.t.header.language}
>
	<Languages class="size-5 shrink-0" aria-hidden="true" />
	<select
		class="cursor-pointer appearance-none bg-transparent font-medium outline-none"
		aria-label={i18n.t.header.language}
		value={i18n.locale}
		onchange={(event) => {
			void i18n.set(event.currentTarget.value as Locale);
		}}
	>
		{#each LOCALES as locale (locale.code)}
			<option value={locale.code} lang={locale.code}>{locale.name}</option>
		{/each}
	</select>
</label>
