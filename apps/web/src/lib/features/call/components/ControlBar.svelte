<script lang="ts">
	import { i18n } from '#lib/i18n/index.ts';
	import { Mic, MicOff, PhoneOff, Settings, Users } from '@lucide/svelte';
	import { cn } from '#lib/utils/cn.ts';
	import type { CallPanel } from '../call-layout.ts';

	interface Props {
		meetingCode: string;
		muted: boolean;
		panel: CallPanel;
		badge: number;
		ontogglemute: () => void;
		onpanel: (panel: CallPanel) => void;
		onleave: () => void;
		/** Host only: ends the meeting for everyone. */
		onendforeveryone?: () => void;
	}

	let {
		meetingCode,
		muted,
		panel,
		badge,
		ontogglemute,
		onpanel,
		onleave,
		onendforeveryone
	}: Props = $props();
	let leaveMenu = $state(false);

	const round = 'flex size-12 items-center justify-center rounded-full transition-colors';
	const toggle = (target: CallPanel) => {
		onpanel(panel === target ? 'none' : target);
	};
</script>

<footer class="grid grid-cols-[1fr_auto_1fr] items-center gap-4 px-4 py-4 text-white">
	<p class="hidden truncate text-sm sm:block">{meetingCode}</p>
	<div class="flex items-center gap-3 sm:col-start-2">
		<button
			type="button"
			class={cn(round, muted ? 'bg-danger hover:bg-danger/90' : 'bg-control hover:bg-control/80')}
			aria-pressed={muted}
			aria-label={muted ? i18n.t.call.turnOnMic : i18n.t.call.turnOffMic}
			onclick={ontogglemute}
		>
			{#if muted}<MicOff class="size-5" />{:else}<Mic class="size-5" />{/if}
		</button>
		<div class="relative">
			<button
				type="button"
				class="flex h-12 items-center justify-center rounded-full bg-danger px-6 hover:bg-danger/90"
				aria-label={i18n.t.call.leaveCall}
				aria-haspopup={onendforeveryone ? 'menu' : undefined}
				aria-expanded={onendforeveryone ? leaveMenu : undefined}
				onclick={() => {
					if (onendforeveryone) leaveMenu = !leaveMenu;
					else onleave();
				}}
			>
				<PhoneOff class="size-5" />
			</button>
			{#if leaveMenu && onendforeveryone}
				<div
					class="absolute bottom-14 left-1/2 flex w-56 -translate-x-1/2 flex-col overflow-hidden rounded-xl bg-surface text-ink shadow-xl"
					role="menu"
				>
					<button
						type="button"
						role="menuitem"
						class="px-4 py-3 text-left hover:bg-surface-raised"
						onclick={onleave}
					>
						{i18n.t.call.justLeave}
					</button>
					<button
						type="button"
						role="menuitem"
						class="px-4 py-3 text-left text-danger hover:bg-surface-raised"
						onclick={onendforeveryone}
					>
						{i18n.t.call.endForEveryone}
					</button>
				</div>
			{/if}
		</div>
	</div>
	<div class="col-start-3 flex justify-end gap-2">
		<button
			type="button"
			class={cn(round, 'relative', panel === 'people' ? 'bg-brand-500' : 'hover:bg-control')}
			aria-label={i18n.t.call.people}
			aria-pressed={panel === 'people'}
			onclick={() => {
				toggle('people');
			}}
		>
			<Users class="size-5" />
			{#if badge > 0}
				<span
					class="absolute -top-0.5 -right-0.5 flex size-5 items-center justify-center rounded-full bg-brand-400 text-xs font-semibold"
				>
					{badge}
				</span>
			{/if}
		</button>
		<button
			type="button"
			class={cn(round, panel === 'settings' ? 'bg-brand-500' : 'hover:bg-control')}
			aria-label={i18n.t.call.audioSettings}
			aria-pressed={panel === 'settings'}
			onclick={() => {
				toggle('settings');
			}}
		>
			<Settings class="size-5" />
		</button>
	</div>
</footer>
