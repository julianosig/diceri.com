<script lang="ts">
	import { Activity } from '@lucide/svelte';
	import { i18n } from '#lib/i18n/index.ts';
	import SettingsSection from './SettingsSection.svelte';
	import type { TransportMetrics } from '../transport-stats.ts';
	import type { CallMetrics } from '../stats-poller.ts';

	let { metrics }: { metrics: CallMetrics } = $props();

	function format(value: number | undefined, unit: string): string {
		return value === undefined ? '—' : `${String(value)} ${unit}`;
	}

	const rows = $derived(
		(
			[
				[i18n.t.audio.sending, metrics.send],
				[i18n.t.audio.receiving, metrics.receive]
			] as const
		).map(([label, m]: readonly [string, TransportMetrics | undefined]) => ({ label, m }))
	);
</script>

<SettingsSection title={i18n.t.audio.connection} icon={Activity}>
	<div class="rounded-xl border border-ink-muted/15 px-4 py-3">
		<table class="w-full text-start text-sm">
			<thead class="text-xs text-ink-muted">
				<tr>
					<th class="pb-2 text-start font-normal">
						<span class="sr-only">{i18n.t.audio.direction}</span>
					</th>
					<th class="pb-2 text-end font-normal">{i18n.t.audio.bitrate}</th>
					<th class="pb-2 text-end font-normal">{i18n.t.audio.loss}</th>
					<th class="pb-2 text-end font-normal">{i18n.t.audio.jitter}</th>
					<th class="pb-2 text-end font-normal">{i18n.t.audio.rtt}</th>
				</tr>
			</thead>
			<tbody class="tabular-nums">
				{#each rows as row (row.label)}
					<tr>
						<th class="py-1 text-start text-xs font-medium text-ink-muted">{row.label}</th>
						<td class="py-1 text-end">{format(row.m?.bitrateKbps, 'kbps')}</td>
						<td class="py-1 text-end">{format(row.m?.lossPercent, '%')}</td>
						<td class="py-1 text-end">{format(row.m?.jitterMs, 'ms')}</td>
						<td class="py-1 text-end">{format(row.m?.roundTripMs, 'ms')}</td>
					</tr>
				{/each}
			</tbody>
		</table>
	</div>
</SettingsSection>
