/** `GET /api/v1/accounts/me`: the username is also the name of the person's room. */
export interface Account {
	username: string;
	/** The mascot of the person's room. */
	mascot: string;
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
