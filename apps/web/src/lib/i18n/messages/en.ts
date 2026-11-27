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
		label: 'Room username',
		mascotTitle: 'Pick your room mascot',
		mascotHint: 'It greets everyone who visits',
		continue: 'Continue',
		back: 'Back',
		usernamePlaceholder: 'room_username',
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
		tooEasy: 'Too easy to guess. Try a few unrelated words, like blue-otter-sings.',
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
	lobby: {
		joinTitle: 'Join the meeting',
		yourName: 'Your name',
		hearEveryoneIn: 'Hear everyone in',
		microphone: 'Microphone',
		systemDefault: 'System default',
		microphonePreview: 'Microphone preview',
		joinNow: 'Join now',
		askToJoin: 'Ask to join',
		joining: 'Joining…',
		micBlocked: 'Microphone access was blocked. Allow it in the browser address bar.',
		micUnavailable: 'Could not access the microphone.',
		waitingToJoin: 'Waiting to join',
		deny: 'Deny',
		admit: 'Admit',
		wantsToJoin: (name: string) => `${name} wants to join this call`,
		hears: (language: string) => `(hears ${language})`,
		dismiss: 'Dismiss',
		cantJoin: "You can't join this meeting",
		declined: 'The host declined your request.',
		somethingWrong: 'Something went wrong',
		back: 'Back',
		asking: 'Asking to be let in…',
		willJoin: "You'll join the meeting when someone lets you in.",
		cancel: 'Cancel'
	},
	voice: {
		pro: 'Pro',
		paidSoon: 'Coming soon as a paid feature.',
		section: 'Your voice',
		recording: 'Recording · read aloud',
		secondsLeft: (seconds: number) => `${String(seconds)}s`,
		done: 'Done',
		keepReading: (seconds: number) => `Keep reading (${String(seconds)}s min)`,
		creating: 'Creating your voice…',
		ready: 'Your voice is ready',
		redo: 'Redo',
		readyHint: 'Translations will sound like you. Deleted when you leave.',
		title: 'Make translations sound like you',
		hint: (seconds: number) =>
			`Read one short sentence for ${String(seconds)} seconds. Optional, and deleted when you leave.`,
		tryAgain: 'Try again',
		record: (seconds: number) => `Record my voice (${String(seconds)}s)`,
		noVoice: "We couldn't hear you clearly. Read the sentence out loud and try again.",
		micNeeded: 'Microphone access is needed to record your voice.',
		failed: 'Could not create your voice.'
	},
	call: {
		turnOnMic: 'Turn on microphone',
		turnOffMic: 'Turn off microphone',
		leaveCall: 'Leave call',
		justLeave: 'Just leave the call',
		endForEveryone: 'End call for everyone',
		people: 'People',
		audioSettings: 'Audio settings',
		guest: 'Guest',
		enableAudio: 'Click to enable audio from other participants',
		connecting: 'Connecting…',
		inCall: 'In call',
		closePanel: 'Close panel',
		you: (name: string) => `${name} (You)`,
		micOff: 'Microphone off',
		hearsEveryoneIn: 'Hears everyone in',
		hears: (language: string) => `hears ${language}`,
		meetingDetails: 'Meeting details',
		meetingReady: "Your meeting's ready",
		close: 'Close',
		shareLink:
			"Share this link with the person you want to talk to. You'll be asked to let them in.",
		copyLink: 'Copy meeting link',
		linkCopied: 'Link copied',
		captions: 'Captions',
		filterUnsupported: 'Enhanced noise filter is not supported in this browser.',
		filterFailed: 'Enhanced noise filter failed.',
		micUnpublished: 'Microphone could not be published.'
	},
	audio: {
		microphone: 'Microphone',
		systemDefault: 'System default',
		noiseAndEcho: 'Noise and echo',
		aiFilter: 'AI noise filter (Krisp)',
		filterOff: 'Off',
		filterNoise: 'Noise',
		filterVoice: 'My voice only',
		filterOffHint: 'Browser processing only.',
		filterNoiseHint: 'Removes background noise, keeps all voices.',
		filterVoiceHint: 'Also removes other people talking nearby (billed separately).',
		echo: 'Echo cancellation',
		echoHint: "Stops the other person's voice from coming back through your mic.",
		suppression: 'Browser noise suppression',
		suppressionOff: 'Off while the AI filter is on.',
		suppressionHint: 'Basic built-in filter.',
		autoGain: 'Auto gain',
		autoGainHint: 'Keeps your volume steady.',
		outgoingLevel: 'Outgoing level (after filtering)',
		muted: 'Muted',
		hearMyself: 'Hear myself',
		headphones: 'Use headphones, or the room hears you twice.',
		monitorOff: 'Off',
		local: 'Local',
		viaServer: 'Via server',
		translated: 'Translated',
		monitorOffHint: 'You do not hear yourself.',
		localHint: 'Hear your mic after the noise filter, instantly. Judges filter quality.',
		viaServerHint:
			'Hear yourself after a round trip through the media server. Judges transmission.',
		translatedHint: 'Hear your own voice translated by the agent. Speak, pause, and listen.',
		translateMeInto: 'Translate me into',
		sameLanguage: (language: string) =>
			`You're speaking ${language}, the language you picked, so there is nothing to translate. Pick another language to hear yourself translated.`,
		connection: 'Connection',
		direction: 'Direction',
		bitrate: 'Bitrate',
		loss: 'Loss',
		jitter: 'Jitter',
		rtt: 'RTT',
		sending: 'Sending',
		receiving: 'Receiving'
	},
	meeting: {
		left: 'You left the meeting',
		endedByMe: 'You ended the call for everyone',
		endedByHost: 'The host ended the call',
		rejoin: 'Rejoin',
		home: 'Return to home screen'
	},
	transcript: {
		title: 'Transcript',
		copy: 'Copy',
		download: 'Download',
		empty: 'Nothing was said while you were in the call.',
		copied: 'Transcript copied'
	},
	plans: {
		usedThisMonth: (used: number, total: number) =>
			`${String(used)} of ${String(total)} min used this month`,
		freeLine: (minutes: number) => `Free: ${String(minutes)} min of calls every month.`
	},
	limits: {
		network:
			'Only one room username can be claimed per network, and this one already has one. Sign in to it instead.',
		quota: "You've used this month's call minutes on your plan. They renew on the 1st (UTC).",
		daily: "You've used today's free call minutes. They renew at midnight (UTC).",
		monthly: "Diceri's free capacity for this month is used up. It renews on the 1st (UTC).",
		busy: 'Diceri is at capacity right now. Try again in a few minutes.',
		timeLeft: (time: string) => `${time} left`,
		timeUp: 'Your free call time is up',
		timeUpHint: "You've used this month's free minutes. They renew on the 1st (UTC)."
	},
	footer: { freeSoftware: 'Free software', sourceCode: 'Source code' }
};

export type Messages = typeof en;
