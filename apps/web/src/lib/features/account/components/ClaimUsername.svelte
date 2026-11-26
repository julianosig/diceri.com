<script lang="ts">
	import { ApiError } from '#lib/api/http-client.ts';
	import { Button } from '#lib/components/ui/index.ts';
	import type { AccountSession } from '../account-session.svelte.ts';
	import { isValidUsername } from '../username.ts';

	interface Props {
		session: AccountSession;
	}

	let { session }: Props = $props();

	let mode = $state<'claim' | 'code'>('claim');
	let username = $state('');
	let code = $state('');
	let pending = $state(false);
	let error = $state<string | null>(null);

	function message(cause: unknown): string {
		if (cause instanceof ApiError && cause.status === 409) return 'That username is taken.';
		if (cause instanceof ApiError && cause.status === 401) return 'That code does not match.';
		if (cause instanceof ApiError && cause.detail) return cause.detail;
		return 'Something went wrong. Try again.';
	}

	async function submit(event: SubmitEvent) {
		event.preventDefault();
		pending = true;
		error = null;
		try {
			await (mode === 'claim' ? session.register(username) : session.signIn(code));
		} catch (cause) {
			error = message(cause);
		} finally {
			pending = false;
		}
	}
</script>

<form
	class="flex w-full max-w-md flex-col gap-3 rounded-tile bg-surface-raised p-4 text-left"
	onsubmit={submit}
>
	{#if mode === 'claim'}
		<label class="flex flex-col gap-1 text-sm">
			<span class="font-medium">Pick a username to start rooms</span>
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
			<span class="text-xs text-ink-muted">No email, no password. Letters, numbers and _.</span>
		</label>
		<Button size="lg" type="submit" disabled={pending || !isValidUsername(username)}>
			{pending ? 'Claiming…' : 'Claim username'}
		</Button>
	{:else}
		<label class="flex flex-col gap-1 text-sm">
			<span class="font-medium">Your sign-in code</span>
			<input
				class="rounded-lg border border-ink-muted/30 bg-surface px-3 py-2 font-mono text-base"
				bind:value={code}
				autocomplete="off"
				spellcheck="false"
			/>
		</label>
		<Button size="lg" type="submit" disabled={pending || code.trim().length === 0}>
			{pending ? 'Signing in…' : 'Sign in'}
		</Button>
	{/if}
	{#if error}<p role="alert" class="text-sm text-red-600">{error}</p>{/if}
	<button
		type="button"
		class="self-center text-xs text-ink-muted underline"
		onclick={() => {
			mode = mode === 'claim' ? 'code' : 'claim';
			error = null;
		}}
	>
		{mode === 'claim' ? 'I already have a username' : 'Pick a new username instead'}
	</button>
</form>
