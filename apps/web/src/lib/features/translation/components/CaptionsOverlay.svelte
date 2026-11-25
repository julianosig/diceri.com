<script lang="ts">
	import type { CaptionLine } from '../captions.svelte.ts';

	interface Props {
		lines: CaptionLine[];
		nameOf: (identity: string) => string;
	}

	let { lines, nameOf }: Props = $props();
</script>

{#if lines.length > 0}
	<section
		class="mx-auto flex w-full max-w-3xl flex-col gap-1 rounded-xl bg-black/70 px-5 py-3 text-white"
		aria-label="Captions"
		aria-live="polite"
	>
		{#each lines as line (`${line.speaker}#${String(line.phrase)}`)}
			<p class="text-lg leading-snug" class:opacity-70={!line.final}>
				<span class="font-semibold">{nameOf(line.speaker)}:</span>
				{line.text}
			</p>
		{/each}
	</section>
{/if}
