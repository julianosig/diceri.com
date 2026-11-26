<script lang="ts">
	import { Activity } from '@lucide/svelte';
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
				['Sending', metrics.send],
				['Receiving', metrics.receive]
			] as const
		).map(([label, m]: readonly [string, TransportMetrics | undefined]) => ({ label, m }))
	);
</script>

<SettingsSection title="Connection" icon={Activity}>
	<div class="rounded-xl border border-ink-muted/15 px-4 py-3">
		<table class="w-full text-start text-sm">
			<thead class="text-xs text-ink-muted">
				<tr>
					<th class="pb-2 text-start font-normal"><span class="sr-only">Direction</span></th>
					<th class="pb-2 text-end font-normal">Bitrate</th>
					<th class="pb-2 text-end font-normal">Loss</th>
					<th class="pb-2 text-end font-normal">Jitter</th>
					<th class="pb-2 text-end font-normal">RTT</th>
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
