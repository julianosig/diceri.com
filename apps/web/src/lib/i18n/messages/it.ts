import type { Messages } from './en.ts';

export const it: Messages = {
	meta: { title: 'Diceri — alternativa a Google Meet con traduzione vocale bidirezionale' },
	header: {
		signOut: 'Esci',
		language: 'Lingua',
		toDark: 'Passa al tema scuro',
		toLight: 'Passa al tema chiaro'
	},
	hero: {
		headline: 'L’alternativa a Google Meet',
		promise: 'dove parli e ascolti nella tua lingua.',
		voice: 'Ti sentono nella loro lingua, con la tua voce.',
		openSource: 'Open source su GitHub',
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
		whoGetsIn: 'Chi entra',
		approval: 'Approvo ogni persona',
		open: 'Chiunque può entrare',
		openHint: 'Chi conosce il tuo @ entra subito finché la stanza è aperta.',
		submit: 'Apri la mia stanza',
		submitting: 'Apertura…',
		failed: 'Impossibile aprire la tua stanza. Riprova.'
	},
	roomOffline: {
		liveTitle: (h: string) => `@${h} non è in diretta ora`,
		liveBody: (h: string) =>
			`Questa stanza si apre quando @${h} avvia una chiamata. Torna presto — oppure apri la tua stanza in pochi secondi.`,
		createOwn: 'Crea la mia stanza',
		tryAgain: 'Riprova',
		freeTitle: (h: string) => `@${h} è ancora libero!`,
		freeBody: 'Nessuno ha ancora preso questa stanza. Può essere tua.',
		claim: (h: string) => `Prendi @${h}`,
		home: 'Torna alla home'
	},
	footer: { freeSoftware: 'Software libero', sourceCode: 'Codice sorgente' }
};
