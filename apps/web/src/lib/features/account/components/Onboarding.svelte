<script lang="ts">
	import { CircleCheck, CircleX, LoaderCircle } from '@lucide/svelte';
	import { ApiError } from '#lib/api/http-client.ts';
	import { Button } from '#lib/components/ui/index.ts';
	import { MascotPicker, randomMascot, type MascotId } from '#lib/features/room/index.ts';
	import type { AccountSession } from '../account-session.svelte.ts';
	import { MAX_PASSWORD_LENGTH, passwordProblem } from '../password.ts';
	import { UsernameCheck } from '../username-check.svelte.ts';
	import { isValidUsername, normalizeUsername } from '../username.ts';

	interface Props {
		session: AccountSession;
	}

	let { session }: Props = $props();

	const usernames = new UsernameCheck();
	$effect(() => () => {
		usernames.dispose();
	});

	let mode = $state<'create' | 'signin'>('create');
	let username = $state('');
	let password = $state('');
	let repeat = $state('');
	let mascot = $state<MascotId>(randomMascot());
	let pending = $state(false);
	let error = $state<string | null>(null);

	const creating = $derived(mode === 'create');
	/** New accounts see the rest of the form only once their @ is free. */
	const showDetails = $derived(!creating || usernames.status === 'available');
	const problem = $derived(creating ? passwordProblem(password, repeat) : null);
	const canSubmit = $derived(
		!pending &&
			isValidUsername(username) &&
			showDetails &&
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

<form class="flex w-full max-w-md flex-col gap-5 text-left" onsubmit={submit}>
	<label class="flex flex-col gap-2">
		<span class="text-xl font-bold">
			{creating ? 'Claim your room username' : 'Username'}
		</span>
		<span
			class="flex items-center rounded-xl border-2 border-ink-muted/25 bg-surface px-4 focus-within:border-brand-500"
		>
			<span class="text-xl font-bold text-ink-muted" aria-hidden="true">@</span>
			<input
				class="w-full bg-transparent py-3.5 pl-1 text-xl font-semibold outline-none"
				bind:value={username}
				oninput={() => {
					usernames.update(username);
				}}
				maxlength={21}
				autocomplete="username"
				autocapitalize="none"
				spellcheck="false"
				placeholder="yourname"
			/>
		</span>
	</label>
	{#if creating && usernames.status !== 'empty'}
		<p class="-mt-2 flex items-center gap-1.5 text-sm font-medium" aria-live="polite">
			{#if usernames.status === 'checking'}
				<LoaderCircle class="size-3.5 animate-spin text-ink-muted" aria-hidden="true" />
				<span class="text-ink-muted">Checking @{normalizeUsername(username)}…</span>
			{:else if usernames.status === 'available'}
				<CircleCheck class="size-3.5 text-online" aria-hidden="true" />
				<span class="text-ink">@{normalizeUsername(username)} is available</span>
			{:else if usernames.status === 'taken'}
				<CircleX class="size-3.5 text-red-600" aria-hidden="true" />
				<span class="text-red-600">@{normalizeUsername(username)} is taken</span>
			{:else if usernames.status === 'invalid'}
				<span class="text-ink-muted">3–20 letters, numbers or _, starting with a letter.</span>
			{:else}
				<span class="text-red-600">Could not check the name. Try again.</span>
			{/if}
		</p>
	{/if}
	{#if showDetails}
		<label class="flex flex-col gap-2">
			<span class="text-base font-semibold">Password</span>
			<input
				type="password"
				class="rounded-xl border-2 border-ink-muted/25 bg-surface px-4 py-3.5 text-xl font-semibold outline-none focus:border-brand-500"
				bind:value={password}
				maxlength={MAX_PASSWORD_LENGTH}
				autocomplete={creating ? 'new-password' : 'current-password'}
			/>
		</label>
	{/if}
	{#if creating && showDetails}
		<label class="flex flex-col gap-2">
			<span class="text-base font-semibold">Repeat password</span>
			<input
				type="password"
				class="rounded-xl border-2 border-ink-muted/25 bg-surface px-4 py-3.5 text-xl font-semibold outline-none focus:border-brand-500"
				bind:value={repeat}
				maxlength={MAX_PASSWORD_LENGTH}
				autocomplete="new-password"
			/>
		</label>
		{#if password.length > 0 && problem}
			<p class="-mt-2 text-sm font-medium text-red-600" aria-live="polite">{problem}</p>
		{/if}
		<MascotPicker
			value={mascot}
			onchange={(next: MascotId) => {
				mascot = next;
			}}
		/>
	{/if}

	{#if showDetails}
		<Button size="lg" type="submit" disabled={!canSubmit}>
			{#if pending}{creating ? 'Creating…' : 'Signing in…'}{:else}{creating
					? 'Create my room'
					: 'Sign in'}{/if}
		</Button>
	{/if}
	{#if error}<p role="alert" class="text-sm text-red-600">{error}</p>{/if}
	<button
		type="button"
		class="self-center text-sm font-medium text-ink-muted underline"
		onclick={switchMode}
	>
		{creating ? 'I already have an account' : 'Create a new account'}
	</button>
</form>
