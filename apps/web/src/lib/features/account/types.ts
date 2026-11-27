import type { Usage } from '#lib/features/plans/index.ts';

/** `GET /api/v1/accounts/me`: the username is also the name of the person's room. */
export interface Account {
	username: string;
	/** The mascot of the person's room. */
	mascot: string;
	/** Free unless assigned another plan. */
	plan: 'free' | 'pro' | 'team';
	/** Today's call minutes and the plan's limits; only on `/accounts/me`. */
	usage?: Usage;
	/** A voice recording is kept with the account (ADR 0015); only on `/accounts/me`. */
	hasVoice?: boolean;
}

/** The single onboarding step (the password is typed twice in the form). */
export interface Registration {
	username: string;
	password: string;
	mascot: string;
}

export interface Credentials {
	username: string;
	password: string;
}
