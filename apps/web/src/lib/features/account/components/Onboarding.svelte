<script lang="ts">
	import { CircleCheck, LoaderCircle, LogIn } from '@lucide/svelte';
	import { ApiError } from '#lib/api/http-client.ts';
	import { Button } from '#lib/components/ui/index.ts';
	import { MascotPicker, randomMascot, type MascotId } from '#lib/features/room/index.ts';
	import type { AccountSession } from '../account-session.svelte.ts';
	import { MAX_PASSWORD_LENGTH, passwordProblem } from '../password.ts';
	import { UsernameCheck } from '../username-check.svelte.ts';
	import { normalizeUsername } from '../username.ts';

	interface Props {
		session: AccountSession;
	}

	let { session }: Props = $props();

	const usernames = new UsernameCheck();
	$effect(() => () => {
		usernames.dispose();
	});

	let username = $state('');
	let password = $state('');
	let repeat = $state('');
	let mascot = $state<MascotId>(randomMascot());
	let pending = $state(false);
	let error = $state<string | null>(null);

	/**
	 * One field decides the path: a free @ is claimed (password twice + mascot), an
	 * existing @ signs in (password once). Nothing else shows until we know which.
	 */
	const mode = $derived(
		usernames.status === 'available' ? 'create' : usernames.status === 'taken' ? 'signin' : null
	);
	const handle = $derived(normalizeUsername(username));
	const problem = $derived(mode === 'create' ? passwordProblem(password, repeat) : null);
	const canSubmit = $derived(
		!pending &&
			mode !== null &&
			password.length > 0 &&
			(mode === 'signin' || (problem === null && repeat === password))
	);

	function message(cause: unknown): string {
		if (cause instanceof ApiError && cause.status === 409) return 'That username was just taken.';
		if (cause instanceof ApiError && cause.status === 401) return 'Wrong password.';
		if (cause instanceof ApiError && cause.detail) return cause.detail;
		return 'Something went wrong. Try again.';
	}

	async function submit(event: SubmitEvent) {
		event.preventDefault();
		if (!canSubmit) return;
		pending = true;
		error = null;
		try {
			await (mode === 'create'
				? session.register({ username, password, mascot })
				: session.signIn({ username, password }));
		} catch (cause) {
			error = message(cause);
		} finally {
			pending = false;
		}
	}

	const inputClass =
		'rounded-xl border-2 border-ink-muted/25 bg-surface px-4 py-3.5 text-center text-xl font-semibold outline-none focus:border-brand-500';
</script>

<form class="flex w-full max-w-md flex-col gap-5 text-center" onsubmit={submit}>
	<label class="flex flex-col gap-2">
		<span class="text-xl font-bold">Login or claim your room username</span>
		<span
			class="flex items-center justify-center rounded-xl border-2 border-ink-muted/25 bg-surface px-4 focus-within:border-brand-500"
		>
			<span class="text-xl font-bold text-ink-muted" aria-hidden="true">@</span>
			<input
				class="field-sizing-content min-w-[9ch] bg-transparent py-3.5 pl-1 text-xl font-semibold outline-none"
				bind:value={username}
				oninput={() => {
					usernames.update(username);
					error = null;
				}}
				maxlength={21}
				autocomplete="username"
				autocapitalize="none"
				spellcheck="false"
				placeholder="yourname"
			/>
		</span>
	</label>

	{#if usernames.status !== 'empty'}
		<p
			class="-mt-2 flex items-center justify-center gap-1.5 text-sm font-medium"
			aria-live="polite"
		>
			{#if usernames.status === 'checking'}
				<LoaderCircle class="size-3.5 animate-spin text-ink-muted" aria-hidden="true" />
				<span class="text-ink-muted">Checking @{handle}…</span>
			{:else if usernames.status === 'available'}
				<CircleCheck class="size-3.5 text-online" aria-hidden="true" />
				<span>@{handle} is available — claim it</span>
			{:else if usernames.status === 'taken'}
				<LogIn class="size-3.5 text-brand-500" aria-hidden="true" />
				<span>Welcome back, @{handle}</span>
			{:else if usernames.status === 'invalid'}
				<span class="text-ink-muted">3–20 letters, numbers or _, starting with a letter.</span>
			{:else}
				<span class="text-red-600">Could not check the name. Try again.</span>
			{/if}
		</p>
	{/if}

	{#if mode}
		<label class="flex flex-col gap-2">
			<span class="text-base font-semibold">Password</span>
			<input
				type="password"
				class={inputClass}
				bind:value={password}
				maxlength={MAX_PASSWORD_LENGTH}
				autocomplete={mode === 'create' ? 'new-password' : 'current-password'}
			/>
		</label>
	{/if}
	{#if mode === 'create'}
		<label class="flex flex-col gap-2">
			<span class="text-base font-semibold">Repeat password</span>
			<input
				type="password"
				class={inputClass}
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

	{#if mode}
		<Button size="lg" type="submit" disabled={!canSubmit}>
			{#if pending}{mode === 'create' ? 'Creating…' : 'Signing in…'}{:else}{mode === 'create'
					? 'Create my room'
					: 'Sign in'}{/if}
		</Button>
	{/if}
	{#if error}<p role="alert" class="text-sm font-medium text-red-600">{error}</p>{/if}
</form>
