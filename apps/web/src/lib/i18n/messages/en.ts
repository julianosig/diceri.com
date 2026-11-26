/**
 * English: the source catalogue. Its shape is the `Messages` type every other language
 * must fill completely (the compiler checks it). Keep strings short; functions take the
 * values they interpolate.
 */
export const en = {
	meta: { title: 'Diceri — Google Meet alternative with two-way voice translation' },
	header: {
		signOut: 'Sign out',
		language: 'Language',
		toDark: 'Switch to dark theme',
		toLight: 'Switch to light theme'
	},
	hero: {
		headline: 'The Google Meet alternative',
		promise: 'where you talk and listen in your own language.',
		voice: 'Everyone in the meeting hears your voice in their language.',
		openSource: 'We are open-source',
		illustration:
			'Illustration: one person speaks, Diceri translates in the middle, and the other person hears it in their own language.'
	},
	onboarding: {
		label: 'Login or claim your room username',
		usernamePlaceholder: 'yourname',
		checking: (handle: string) => `Checking @${handle}…`,
		available: (handle: string) => `@${handle} is available — claim it`,
		welcomeBack: (handle: string) => `Welcome back, @${handle}`,
		invalid: '3–20 letters (a–z), numbers or _, starting with a letter.',
		checkFailed: 'Could not check the name. Try again.',
		password: 'Password',
		repeatPassword: 'Repeat password',
		newPasswordPlaceholder: (min: number) => `At least ${String(min)} characters`,
		passwordPlaceholder: 'Your password',
		repeatPlaceholder: 'Type it again',
		passwordsMatch: 'Passwords match',
		tooShort: (min: number) => `Use at least ${String(min)} characters.`,
		tooLong: (max: number) => `Use at most ${String(max)} characters.`,
		mismatch: 'The passwords do not match.',
		create: 'Create my room',
		creating: 'Creating…',
		signIn: 'Sign in',
		signingIn: 'Signing in…',
		taken: 'That username was just taken.',
		wrongPassword: 'Wrong password.',
		failed: 'Something went wrong. Try again.'
	},
	mascots: {
		legend: 'Room mascot',
		names: {
			wizard: 'Wizard',
			robot: 'Robot',
			astronaut: 'Astronaut',
			alien: 'Alien',
			ninja: 'Ninja',
			dragon: 'Dragon',
			knight: 'Knight',
			ghost: 'Ghost',
			cybercat: 'Cyber cat',
			slime: 'Slime',
			pirate: 'Space pirate',
			gamer: 'Gamer'
		}
	},
	openRoom: {
		whoGetsIn: 'Who gets in',
		approval: 'I approve each person',
		open: 'Anyone can join',
		openHint: 'Anyone who knows your @ walks straight in while the room is open.',
		submit: 'Open my room',
		submitting: 'Opening…',
		failed: 'Could not open your room. Try again.'
	},
	roomOffline: {
		liveTitle: (h: string) => `@${h} isn’t live right now`,
		liveBody: (h: string) =>
			`This room opens when @${h} starts a call. Come back soon — or open your own room in seconds.`,
		createOwn: 'Create my own room',
		tryAgain: 'Try again',
		freeTitle: (h: string) => `@${h} is still free!`,
		freeBody: 'Nobody has claimed this room yet. It could be yours.',
		claim: (h: string) => `Claim @${h}`,
		home: 'Back to home'
	},
	footer: { freeSoftware: 'Free software', sourceCode: 'Source code' }
};

export type Messages = typeof en;
