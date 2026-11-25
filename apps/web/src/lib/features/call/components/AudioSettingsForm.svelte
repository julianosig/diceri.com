<script lang="ts">
	import { Toggle } from '#lib/components/ui/index.ts';
	import type { AudioSettings, EnhancedFilter } from '../audio-settings.ts';

	interface Props {
		settings: AudioSettings;
		devices: MediaDeviceInfo[];
		onchange: (patch: Partial<AudioSettings>) => void;
	}

	let { settings, devices, onchange }: Props = $props();

	const filters: { value: EnhancedFilter; label: string; hint: string }[] = [
		{ value: 'off', label: 'Off', hint: 'Browser processing only.' },
		{ value: 'noise', label: 'Noise', hint: 'Removes background noise, keeps all voices.' },
		{
			value: 'voice',
			label: 'My voice only',
			hint: 'Also removes other people talking nearby (billed separately).'
		}
	];
	const enhanced = $derived(settings.enhancedFilter !== 'off');
</script>

<div class="flex flex-col gap-4">
	<label class="flex flex-col gap-1 text-sm font-medium">
		Microphone
		<select
			class="rounded-lg bg-surface-raised px-3 py-2"
			value={settings.deviceId ?? ''}
			onchange={(event) => {
				onchange({ deviceId: event.currentTarget.value || undefined });
			}}
		>
			<option value="">System default</option>
			{#each devices as device (device.deviceId)}
				<option value={device.deviceId}>{device.label || 'Microphone'}</option>
			{/each}
		</select>
	</label>

	<fieldset class="flex flex-col gap-2">
		<legend class="mb-1 text-sm font-medium">AI noise filter (Krisp)</legend>
		<div class="flex gap-2" role="radiogroup">
			{#each filters as filter (filter.value)}
				<button
					type="button"
					role="radio"
					aria-checked={settings.enhancedFilter === filter.value}
					title={filter.hint}
					class="flex-1 rounded-lg bg-surface-raised px-3 py-2 text-sm aria-checked:bg-brand-500 aria-checked:text-white"
					onclick={() => {
						onchange({ enhancedFilter: filter.value });
					}}
				>
					{filter.label}
				</button>
			{/each}
		</div>
		<p class="text-xs text-ink-muted">
			{filters.find((f) => f.value === settings.enhancedFilter)?.hint}
		</p>
	</fieldset>

	<Toggle
		label="Echo cancellation"
		description="Prevents the other person's voice from coming back through your mic."
		checked={settings.echoCancellation}
		onchange={(echoCancellation: boolean) => {
			onchange({ echoCancellation });
		}}
	/>
	<Toggle
		label="Browser noise suppression"
		description={enhanced ? 'Disabled while the AI filter is on.' : 'Basic built-in filter.'}
		checked={settings.noiseSuppression && !enhanced}
		disabled={enhanced}
		onchange={(noiseSuppression: boolean) => {
			onchange({ noiseSuppression });
		}}
	/>
	<Toggle
		label="Auto gain"
		description="Keeps your volume steady."
		checked={settings.autoGainControl}
		onchange={(autoGainControl: boolean) => {
			onchange({ autoGainControl });
		}}
	/>
</div>
