<script lang="ts">
	import { ApiError } from '#lib/api/http-client.ts';
	import { Button } from '#lib/components/ui/index.ts';
	import {
		MascotAvatar,
		MascotPicker,
		randomMascot,
		type MascotId
	} from '#lib/features/room/index.ts';
	import type { AccountSession } from '../account-session.svelte.ts';
	import { MAX_PASSWORD_LENGTH, passwordProblem } from '../password.ts';
	import { isValidUsername, normalizeUsername } from '../username.ts';

	interface Props {
		session: AccountSession;
	}

	let { session }: Props = $props();

	let mode = $state<'create' | 'signin'>('create');
	let username = $state('');
	let password = $state('');
	let repeat = $state('');
	let mascot = $state<MascotId>(randomMascot());
	let pending = $state(false);
	let error = $state<string | null>(null);

	const creating = $derived(mode === 'create');
	const problem = $derived(creating ? passwordProblem(password, repeat) : null);
	const canSubmit = $derived(
		!pending &&
			isValidUsername(username) &&
			password.length > 0 &&
			(!creating || (problem === null && repeat === password))
	);

	function message(cause: unknown): string {
		if (cause instanceof ApiError && cause.status === 409) return 'That username is taken.';
		if (cause instanceof ApiError && cause.status === 401) return 'Wrong username or password.';
		if (cause instanceof ApiError && cause.detail) return cause.detail;
		return 'Something went wrong. Try again.';
	}

	async function submit(event: SubmitEvent) {
		event.preventDefault();
		pending = true;
		error = null;
		try {
			await (creating
				? session.register({ username, password, mascot })
				: session.signIn({ username, password }));
		} catch (cause) {
			error = message(cause);
		} finally {
			pending = false;
		}
	}

	function switchMode() {
		mode = creating ? 'signin' : 'create';
		error = null;
		repeat = '';
	}
</script>

<form
	class="flex w-full max-w-md flex-col gap-4 rounded-tile bg-surface-raised p-4 text-left"
	onsubmit={submit}
>
	<div class="flex items-center gap-3">
		{#if creating}
			<MascotAvatar {mascot} size={56} />
		{/if}
		<div class="flex flex-col">
			<h2 class="text-base font-semibold">
				{creating ? 'Claim your room' : 'Welcome back'}
			</h2>
			<p class="text-xs text-ink-muted">
				{#if creating}
					Your @ is your room: <span class="font-medium">
						diceri.com/@{normalizeUsername(username) || 'you'}
					</span>
				{:else}
					Sign in to open your room.
				{/if}
			</p>
		</div>
	</div>

	<label class="flex flex-col gap-1 text-sm">
		<span class="text-ink-muted">Username</span>
		<span class="flex items-center rounded-lg border border-ink-muted/30 bg-surface px-3">
			<span class="text-ink-muted" aria-hidden="true">@</span>
			<input
				class="w-full bg-transparent py-2 pl-1 text-base outline-none"
				bind:value={username}
				maxlength={21}
				autocomplete="username"
				autocapitalize="none"
				spellcheck="false"
				placeholder="yourname"
			/>
		</span>
	</label>
	<label class="flex flex-col gap-1 text-sm">
		<span class="text-ink-muted">Password</span>
		<input
			type="password"
			class="rounded-lg border border-ink-muted/30 bg-surface px-3 py-2 text-base"
			bind:value={password}
			maxlength={MAX_PASSWORD_LENGTH}
			autocomplete={creating ? 'new-password' : 'current-password'}
		/>
	</label>
	{#if creating}
		<label class="flex flex-col gap-1 text-sm">
			<span class="text-ink-muted">Repeat password</span>
			<input
				type="password"
				class="rounded-lg border border-ink-muted/30 bg-surface px-3 py-2 text-base"
				bind:value={repeat}
				maxlength={MAX_PASSWORD_LENGTH}
				autocomplete="new-password"
			/>
		</label>
		{#if password.length > 0 && problem}
			<p class="text-xs text-red-600" aria-live="polite">{problem}</p>
		{/if}
		<MascotPicker
			value={mascot}
			onchange={(next: MascotId) => {
				mascot = next;
			}}
		/>
	{/if}

	<Button size="lg" type="submit" disabled={!canSubmit}>
		{#if pending}{creating ? 'Creating…' : 'Signing in…'}{:else}{creating
				? 'Create my room'
				: 'Sign in'}{/if}
	</Button>
	{#if error}<p role="alert" class="text-sm text-red-600">{error}</p>{/if}
	<button type="button" class="self-center text-xs text-ink-muted underline" onclick={switchMode}>
		{creating ? 'I already have an account' : 'Create a new account'}
	</button>
</form>
