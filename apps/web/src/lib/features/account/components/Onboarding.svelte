<script lang="ts">
	import { untrack } from 'svelte';
	import {
		AtSign,
		CircleCheck,
		LoaderCircle,
		Lock,
		LockKeyhole,
		LogIn,
		UserPlus
	} from '@lucide/svelte';
	import { ApiError } from '#lib/api/http-client.ts';
	import { Button } from '#lib/components/ui/index.ts';
	import { randomMascot, type MascotId } from '#lib/features/room/index.ts';
	import { i18n } from '#lib/i18n/index.ts';
	import type { AccountSession } from '../account-session.svelte.ts';
	import {
		MAX_PASSWORD_LENGTH,
		MIN_PASSWORD_LENGTH,
		passwordProblem,
		type PasswordProblem
	} from '../password.ts';
	import { UsernameCheck } from '../username-check.svelte.ts';
	import { normalizeUsername } from '../username.ts';
	import MascotStep from './MascotStep.svelte';

	interface Props {
		session: AccountSession;
		/** Pre-fills and checks a name (`/?claim=ana` from a free `/@ana`). */
		initialUsername?: string;
	}

	let { session, initialUsername }: Props = $props();

	const usernames = new UsernameCheck();
	$effect(() => () => {
		usernames.dispose();
	});

	const initial = untrack(() => initialUsername) ?? '';
	let username = $state(initial);
	if (initial) usernames.update(initial);
	let password = $state('');
	let repeat = $state('');
	let mascot = $state<MascotId>(randomMascot());
	/** A new account picks its mascot on a step of its own, after name and password. */
	let step = $state<'details' | 'mascot'>('details');
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

	const t = $derived(i18n.t.onboarding);

	function message(cause: unknown): string {
		if (cause instanceof ApiError && cause.status === 409) return t.taken;
		if (cause instanceof ApiError && cause.status === 401) return t.wrongPassword;
		return t.failed;
	}

	function describe(found: PasswordProblem): string {
		if (found === 'too-short') return t.tooShort(MIN_PASSWORD_LENGTH);
		if (found === 'too-long') return t.tooLong(MAX_PASSWORD_LENGTH);
		return t.mismatch;
	}

	function submit(event: SubmitEvent) {
		event.preventDefault();
		if (!canSubmit) return;
		if (mode === 'create') {
			mascot = randomMascot();
			step = 'mascot';
			return;
		}
		void send();
	}

	async function send() {
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

	/** Styled like the pre-join screen; only the @ field shows until the name resolves. */
	const fieldClass =
		'flex items-center gap-2 rounded-lg border border-ink-muted/30 bg-surface px-3 focus-within:border-brand-500';
	const inputClass = 'w-full bg-transparent py-2 text-base outline-none';
	const labelClass = 'text-sm text-ink-muted';
</script>

{#if step === 'mascot' && mode === 'create'}
	<MascotStep
		{handle}
		value={mascot}
		{pending}
		{error}
		onchange={(next: MascotId) => {
			mascot = next;
		}}
		onback={() => {
			step = 'details';
			error = null;
		}}
		oncreate={() => void send()}
	/>
{:else}
	<form class="flex w-full max-w-md flex-col gap-5 text-start" onsubmit={submit}>
		<h1 class="text-center text-3xl font-semibold tracking-tight">{i18n.t.openRoom.submit}</h1>
		<label class="flex flex-col gap-1">
			<span class={labelClass}>{t.label}</span>
			<span class={fieldClass} dir="ltr">
				<AtSign class="size-5 shrink-0 text-ink-muted" aria-hidden="true" />
				<input
					class={inputClass}
					bind:value={username}
					oninput={() => {
						usernames.update(username);
						error = null;
					}}
					maxlength={21}
					autocomplete="username"
					autocapitalize="none"
					spellcheck="false"
					placeholder={t.usernamePlaceholder}
				/>
				{#if usernames.status === 'available'}
					<CircleCheck class="size-5 shrink-0 text-online" aria-label={t.available(handle)} />
				{/if}
			</span>
		</label>

		{#if usernames.status !== 'empty' && usernames.status !== 'available'}
			<p class="-mt-2 flex items-center gap-1.5 text-sm font-medium" aria-live="polite">
				{#if usernames.status === 'checking'}
					<LoaderCircle class="size-3.5 animate-spin text-ink-muted" aria-hidden="true" />
					<span class="text-ink-muted">{t.checking(handle)}</span>
				{:else if usernames.status === 'taken'}
					<LogIn class="size-3.5 text-brand-500" aria-hidden="true" />
					<span>{t.welcomeBack(handle)}</span>
				{:else if usernames.status === 'invalid'}
					<span class="text-ink-muted">{t.invalid}</span>
				{:else}
					<span class="text-red-600">{t.checkFailed}</span>
				{/if}
			</p>
		{/if}

		{#if mode}
			<label class="flex flex-col gap-1">
				<span class={labelClass}>{t.password}</span>
				<span class={fieldClass}>
					<Lock class="size-5 shrink-0 text-ink-muted" aria-hidden="true" />
					<input
						type="password"
						class={inputClass}
						bind:value={password}
						maxlength={MAX_PASSWORD_LENGTH}
						autocomplete={mode === 'create' ? 'new-password' : 'current-password'}
						placeholder={mode === 'create'
							? t.newPasswordPlaceholder(MIN_PASSWORD_LENGTH)
							: t.passwordPlaceholder}
					/>
				</span>
			</label>
		{/if}
		{#if mode === 'create'}
			<label class="flex flex-col gap-1">
				<span class={labelClass}>{t.repeatPassword}</span>
				<span class={fieldClass}>
					<LockKeyhole class="size-5 shrink-0 text-ink-muted" aria-hidden="true" />
					<input
						type="password"
						class={inputClass}
						bind:value={repeat}
						maxlength={MAX_PASSWORD_LENGTH}
						autocomplete="new-password"
						placeholder={t.repeatPlaceholder}
					/>
					{#if repeat.length > 0 && repeat === password}
						<CircleCheck class="size-5 shrink-0 text-online" aria-label={t.passwordsMatch} />
					{/if}
				</span>
			</label>
			{#if password.length > 0 && problem}
				<p class="-mt-2 text-sm font-medium text-red-600" aria-live="polite">
					{describe(problem)}
				</p>
			{/if}
		{/if}

		{#if mode}
			<Button size="lg" type="submit" class="self-center" disabled={!canSubmit}>
				{#if pending}
					<LoaderCircle class="size-5 animate-spin" aria-hidden="true" />
					{mode === 'create' ? t.creating : t.signingIn}
				{:else if mode === 'create'}
					<UserPlus class="size-5" aria-hidden="true" />
					{t.create}
				{:else}
					<LogIn class="size-5" aria-hidden="true" />
					{t.signIn}
				{/if}
			</Button>
		{/if}
		{#if error}<p role="alert" class="text-sm font-medium text-red-600">{error}</p>{/if}
	</form>
{/if}
