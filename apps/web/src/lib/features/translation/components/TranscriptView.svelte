<script lang="ts">
	import { Check, Copy, Download } from '@lucide/svelte';
	import { Button } from '#lib/components/ui/index.ts';
	import { languageLabel } from '../languages.ts';
	import type { TranscriptLog } from '../transcript.svelte.ts';

	let { transcript }: { transcript: TranscriptLog } = $props();
	let copied = $state(false);

	const time = (at: number) =>
		new Date(at).toLocaleTimeString([], { hour: '2-digit', minute: '2-digit' });

	function download() {
		const blob = new Blob([transcript.toText()], { type: 'text/plain;charset=utf-8' });
		const link = document.createElement('a');
		link.href = URL.createObjectURL(blob);
		link.download = `diceri-transcript-${new Date().toISOString().slice(0, 16)}.txt`;
		link.click();
		URL.revokeObjectURL(link.href);
	}

	async function copy() {
		await navigator.clipboard.writeText(transcript.toText());
		copied = true;
	}
</script>

<section class="flex w-full max-w-2xl flex-col gap-4 text-left" aria-label="Transcript">
	<header class="flex flex-wrap items-center justify-between gap-3">
		<h2 class="text-xl">Transcript</h2>
		{#if transcript.entries.length > 0}
			<div class="flex gap-2">
				<Button size="sm" variant="secondary" onclick={copy}>
					{#if copied}<Check class="size-4" />{:else}<Copy class="size-4" />{/if} Copy
				</Button>
				<Button size="sm" onclick={download}><Download class="size-4" /> Download</Button>
			</div>
		{/if}
	</header>
	{#if transcript.entries.length === 0}
		<p class="text-sm text-ink-muted">Nothing was said while you were in the call.</p>
	{:else}
		<ol
			class="flex max-h-[60dvh] flex-col gap-3 overflow-y-auto rounded-tile bg-surface-raised p-4"
		>
			{#each transcript.entries as entry (`${entry.speaker}#${String(entry.phrase)}`)}
				<li class="flex flex-col gap-1">
					<p class="text-xs text-ink-muted">{time(entry.at)} · {entry.speakerName}</p>
					{#if entry.original}
						<p>
							<span class="mr-1 rounded bg-surface px-1 text-xs uppercase">
								{entry.original.language}
							</span>{entry.original.text}
						</p>
					{/if}
					{#each Object.entries(entry.translations) as [language, text] (language)}
						<p class="text-sm text-ink-muted" title={languageLabel(language)}>
							<span class="mr-1 rounded bg-surface px-1 text-xs uppercase">{language}</span>{text}
						</p>
					{/each}
				</li>
			{/each}
		</ol>
	{/if}
	<span class="sr-only" aria-live="polite">{copied ? 'Transcript copied' : ''}</span>
</section>
