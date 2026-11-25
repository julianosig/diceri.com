<script lang="ts">
	import { LANGUAGES, languageLabel, sameLanguage } from '#lib/features/translation/index.ts';
	import type { MonitorMode } from '../call-session.svelte.ts';

	interface Props {
		mode: MonitorMode;
		previewLanguage: string;
		/** Language your speech was last detected in, if any. */
		spokenLanguage: string | undefined;
		onchange: (mode: MonitorMode) => void;
		onpreviewchange: (language: string) => void;
	}

	let { mode, previewLanguage, spokenLanguage, onchange, onpreviewchange }: Props = $props();
	const sameAsSpoken = $derived(
		mode === 'translated' && sameLanguage(spokenLanguage, previewLanguage)
	);

	const modes: { value: MonitorMode; label: string; hint: string }[] = [
		{ value: 'off', label: 'Off', hint: 'You do not hear yourself.' },
		{
			value: 'local',
			label: 'Local',
			hint: 'Hear your mic after the noise filter, instantly. Judges filter quality.'
		},
		{
			value: 'loopback',
			label: 'Via server',
			hint: 'Hear yourself after a round trip through the media server. Judges transmission.'
		},
		{
			value: 'translated',
			label: 'Translated',
			hint: 'Hear your own voice translated by the agent. Speak, pause, and listen.'
		}
	];
</script>

<fieldset class="flex flex-col gap-2">
	<legend class="mb-1 text-sm font-medium">Hear myself (use headphones)</legend>
	<div class="flex gap-2" role="radiogroup">
		{#each modes as option (option.value)}
			<button
				type="button"
				role="radio"
				aria-checked={mode === option.value}
				class="flex-1 rounded-lg bg-surface-raised px-3 py-2 text-sm aria-checked:bg-brand-500 aria-checked:text-white"
				onclick={() => {
					onchange(option.value);
				}}
			>
				{option.label}
			</button>
		{/each}
	</div>
	<p class="text-xs text-ink-muted">{modes.find((m) => m.value === mode)?.hint}</p>
	{#if mode === 'translated'}
		<label class="flex flex-col gap-1 text-sm font-medium">
			Translate me into
			<select
				class="rounded-lg bg-surface-raised px-3 py-2"
				value={previewLanguage}
				onchange={(event) => {
					onpreviewchange(event.currentTarget.value);
				}}
			>
				{#each LANGUAGES as option (option.code)}
					<option value={option.code}>{option.label}</option>
				{/each}
			</select>
		</label>
	{/if}
	{#if sameAsSpoken && spokenLanguage}
		<p role="status" class="rounded-lg bg-amber-100 px-3 py-2 text-xs text-amber-900">
			You're speaking {languageLabel(spokenLanguage)}, the language you picked, so there is nothing
			to translate. Pick another language to hear yourself translated.
		</p>
	{/if}
</fieldset>
