<script lang="ts">
	import { SiteFooter, SiteHeader } from '#lib/features/landing/index.ts';
	import { i18n } from '#lib/i18n/index.ts';

	const t = $derived(i18n.t.terms);
</script>

<svelte:head>
	<title>{t.title} · Diceri</title>
</svelte:head>

<SiteHeader />
<main class="mx-auto flex w-full max-w-2xl flex-col gap-6 px-4 pt-6 pb-16">
	<header class="flex flex-col gap-1">
		<h1 class="text-3xl font-semibold tracking-tight">{t.title}</h1>
		<p class="text-sm text-ink-muted">{t.updated}</p>
	</header>
	{#each t.sections as section (section.heading)}
		<section class="flex flex-col gap-2">
			<h2 class="text-lg font-semibold">{section.heading}</h2>
			<p class="leading-relaxed text-ink-muted">{section.body}</p>
		</section>
	{/each}
	<div class="mt-6"><SiteFooter /></div>
</main>
