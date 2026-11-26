import type { Messages } from './en.ts';

export const it: Messages = {
	meta: { title: 'Diceri — la chiamata che conosci, in ogni lingua' },
	header: {
		signOut: 'Esci',
		language: 'Lingua',
		toDark: 'Passa al tema scuro',
		toLight: 'Passa al tema chiaro'
	},
	hero: {
		headline: 'La videochiamata che conosci già.',
		promise: 'Ora in ogni lingua.',
		illustration:
			'Illustrazione: una persona parla, Diceri traduce nel mezzo e l’altra persona la sente nella propria lingua.'
	},
	onboarding: {
		label: 'Accedi o prenota il nome della tua stanza',
		usernamePlaceholder: 'tuonome',
		checking: (handle: string) => `Verifica di @${handle}…`,
		available: (handle: string) => `@${handle} è libero — prendilo`,
		welcomeBack: (handle: string) => `Ciao di nuovo, @${handle}`,
		invalid: 'Da 3 a 20 lettere (a–z), numeri o _, iniziando con una lettera.',
		checkFailed: 'Impossibile verificare il nome. Riprova.',
		password: 'Password',
		repeatPassword: 'Ripeti la password',
		newPasswordPlaceholder: (min: number) => `Almeno ${String(min)} caratteri`,
		passwordPlaceholder: 'La tua password',
		repeatPlaceholder: 'Scrivila di nuovo',
		passwordsMatch: 'Le password coincidono',
		tooShort: (min: number) => `Usa almeno ${String(min)} caratteri.`,
		tooLong: (max: number) => `Usa al massimo ${String(max)} caratteri.`,
		mismatch: 'Le password non coincidono.',
		create: 'Crea la mia stanza',
		creating: 'Creazione…',
		signIn: 'Accedi',
		signingIn: 'Accesso…',
		taken: 'Questo nome è appena stato preso.',
		wrongPassword: 'Password errata.',
		failed: 'Qualcosa è andato storto. Riprova.'
	},
	mascots: {
		legend: 'Mascotte della stanza',
		names: {
			wizard: 'Mago',
			robot: 'Robot',
			astronaut: 'Astronauta',
			alien: 'Alieno',
			ninja: 'Ninja',
			dragon: 'Drago',
			knight: 'Cavaliere',
			ghost: 'Fantasma',
			cybercat: 'Gatto cyber',
			slime: 'Slime',
			pirate: 'Pirata spaziale',
			gamer: 'Gamer'
		}
	},
	openRoom: {
		livesAt: (username: string) => `La tua stanza è su diceri.com/@${username}`,
		listed: 'Mostra nella bacheca live',
		listedHint: 'Chiunque sulla home vede la tua stanza e può aprirla.',
		whoGetsIn: 'Chi entra',
		approval: 'Approvo ogni persona',
		open: 'Chiunque può entrare',
		openHint: 'Chi conosce il tuo @ entra subito finché la stanza è aperta.',
		submit: 'Apri la mia stanza',
		submitting: 'Apertura…',
		failed: 'Impossibile aprire la tua stanza. Riprova.'
	},
	footer: { freeSoftware: 'Software libero', sourceCode: 'Codice sorgente' }
};
