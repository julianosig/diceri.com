/**
 * English: the source catalogue. Its shape is the `Messages` type every other language
 * must fill completely (the compiler checks it). Keep strings short; functions take the
 * values they interpolate.
 */
export const en = {
	meta: { title: 'Diceri — Google Meet alternative with two-way voice translation' },
	header: {
		signOut: 'Sign out',
		deleteVoice: 'Delete my voice',
		deleteVoiceConfirm:
			'Delete your saved voice recording? You will record a new one at your next call.',
		voiceDeleted: 'Your voice recording was deleted.',
		voiceDeleteFailed:
			'Your voice could not be deleted right now. Nothing was deleted; please try again.',
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
		joinRoomTitle: (handle: string) => `Join @${handle}`,
		joinRoomHint:
			'To join this call, sign in with your Diceri username, or create one for free if you are new. It takes a minute and lets everyone hear you in their own language, in your own voice.',
		camera: 'Camera',
		cameraOn: 'Turn camera on',
		cameraOff: 'Turn camera off',
		cameraBlocked: 'Allow the camera in your browser to show your video.',
		cameraUnavailable: 'No camera found.',
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
		requiredHint: (seconds: number) =>
			`Read one short sentence for ${String(seconds)} seconds: everyone hears your translations in your own voice. Kept, encrypted, with your account.`,
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
		readyHint:
			'Translations will sound like you. The recording is kept, encrypted, with your account.',
		keptReady: 'Your saved voice is ready',
		keptHint: 'Using the recording kept with your account. Delete it any time from your @ menu.',
		title: 'Make translations sound like you',
		hint: (seconds: number) =>
			`Read one short sentence for ${String(seconds)} seconds. Optional; kept, encrypted, with your account.`,
		tryAgain: 'Try again',
		record: (seconds: number) => `Record my voice (${String(seconds)}s)`,
		noVoice: "We couldn't hear you clearly. Read the sentence out loud and try again.",
		micNeeded: 'Microphone access is needed to record your voice.',
		failed: 'Could not create your voice.'
	},
	call: {
		turnOnCamera: 'Turn camera on',
		turnOffCamera: 'Turn camera off',
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
		monitorOffHint: 'You do not hear yourself.',
		localHint: 'Hear your mic after the noise filter, instantly. Judges filter quality.',
		viaServerHint:
			'Hear yourself after a round trip through the media server. Judges transmission.',
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
		disconnected: 'You were disconnected',
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
		betaLeft: 'spots left',
		betaDetail: (total: number) => `Public beta · ${String(total)} in total`,
		betaFull: 'All spots are taken for now',
		usedThisMonth: (used: number, total: number) =>
			`${String(used)} of ${String(total)} min used this month`,
		freeLine: (minutes: number) => `Free: ${String(minutes)} min of calls every month.`
	},
	limits: {
		full: 'This call is full: it has room for two people. Try again when someone leaves.',
		beta: 'Diceri is in a public beta and all spots are taken for now. Please try again later.',
		network:
			'This network has already claimed the most room usernames allowed. Sign in to one of them instead.',
		quota: "You've used this month's call minutes on your plan. They renew on the 1st (UTC).",
		daily: "You've used today's free call minutes. They renew at midnight (UTC).",
		monthly: "Diceri's free capacity for this month is used up. It renews on the 1st (UTC).",
		busy: 'Diceri is at capacity right now. Try again in a few minutes.',
		timeLeft: (time: string) => `${time} left`,
		timeUp: 'Your free call time is up',
		timeUpHint: "You've used this month's free minutes. They renew on the 1st (UTC)."
	},
	terms: {
		title: 'Terms of use',
		updated: 'Last updated: 27 November 2026',
		notice:
			'By creating your room you agree to the terms of use, including keeping your voice recording, encrypted, to recreate your voice in calls.',
		read: 'Read the terms of use',
		sections: [
			{
				heading: 'What Diceri is',
				body: 'Diceri is a video-call service with live voice translation, run as free and open-source software (AGPL-3.0). Each account owns one room at diceri.com/@username. Using Diceri means accepting these terms.'
			},
			{
				heading: 'Your account',
				body: 'You sign up with a username and a password; no email is asked. Keep your password to yourself. A few accounts can be created per network, and the free plan includes a monthly amount of call minutes.'
			},
			{
				heading: 'Your voice',
				body: 'To join a call you record about 10 seconds of your voice. We use it to create a copy of your voice with our voice provider, so that people hear your translations in your own voice. We keep the recording with your account, encrypted, so we can recreate your voice in later calls, including with another voice provider if we change. It is used only to speak your own translations, never to speak for anyone else, and never sold or shared for other purposes.'
			},
			{
				heading: 'Deleting your voice',
				body: 'You can delete your recording at any time with "Delete my voice" in your @ menu; it is also deleted with your account. The copy of your voice made at the voice provider is kept for your calls while your recording is saved, and is deleted with it.'
			},
			{
				heading: 'What we keep, and what we do not',
				body: "We keep your username, a protected form of your password, your room's mascot, your plan, how many call minutes you used, an encrypted fingerprint of the network you signed up from (never your IP address) and your voice recording. We do not record calls and do not keep what was said: transcripts and captions stay in your browser."
			},
			{
				heading: 'Service providers',
				body: 'Calls run through LiveKit; speech is transcribed, translated and spoken by ElevenLabs, Inworld, Soniox and Google Gemini. They receive only what is needed to provide the call.'
			},
			{
				heading: 'Fair use',
				body: 'Do not use Diceri to break the law, harass people or impersonate anyone. We may limit or close accounts that abuse the service. Diceri is provided as is, without warranties.'
			},
			{
				heading: 'Changes',
				body: 'We may update these terms; the date above shows the latest version. Questions: open an issue at github.com/julianosig/diceri.com.'
			}
		]
	},
	footer: { freeSoftware: 'Free software', sourceCode: 'Source code', terms: 'Terms of use' }
};

export type Messages = typeof en;
