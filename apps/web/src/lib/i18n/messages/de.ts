import type { Messages } from './en.ts';

export const de: Messages = {
	meta: { title: 'Diceri — Google-Meet-Alternative mit Sprachübersetzung in beide Richtungen' },
	header: {
		signOut: 'Abmelden',
		language: 'Sprache',
		toDark: 'Zum dunklen Design wechseln',
		toLight: 'Zum hellen Design wechseln'
	},
	hero: {
		headline: 'Die Google-Meet-Alternative,',
		promise: 'in der du in deiner Sprache sprichst und hörst.',
		voice: 'Sie hören dich in ihrer Sprache – mit deiner eigenen Stimme.',
		illustration:
			'Illustration: Eine Person spricht, Diceri übersetzt in der Mitte, und die andere Person hört es in ihrer eigenen Sprache.'
	},
	onboarding: {
		label: 'Melde dich an oder sichere dir deinen Raumnamen',
		usernamePlaceholder: 'deinname',
		checking: (handle: string) => `@${handle} wird geprüft…`,
		available: (handle: string) => `@${handle} ist frei – sichere ihn dir`,
		welcomeBack: (handle: string) => `Willkommen zurück, @${handle}`,
		invalid: '3–20 Buchstaben (a–z), Ziffern oder _, beginnend mit einem Buchstaben.',
		checkFailed: 'Der Name konnte nicht geprüft werden. Versuch es noch einmal.',
		password: 'Passwort',
		repeatPassword: 'Passwort wiederholen',
		newPasswordPlaceholder: (min: number) => `Mindestens ${String(min)} Zeichen`,
		passwordPlaceholder: 'Dein Passwort',
		repeatPlaceholder: 'Noch einmal eingeben',
		passwordsMatch: 'Passwörter stimmen überein',
		tooShort: (min: number) => `Verwende mindestens ${String(min)} Zeichen.`,
		tooLong: (max: number) => `Verwende höchstens ${String(max)} Zeichen.`,
		mismatch: 'Die Passwörter stimmen nicht überein.',
		create: 'Meinen Raum erstellen',
		creating: 'Wird erstellt…',
		signIn: 'Anmelden',
		signingIn: 'Anmeldung…',
		taken: 'Dieser Name wurde gerade vergeben.',
		wrongPassword: 'Falsches Passwort.',
		failed: 'Etwas ist schiefgelaufen. Versuch es noch einmal.'
	},
	mascots: {
		legend: 'Raum-Maskottchen',
		names: {
			wizard: 'Zauberer',
			robot: 'Roboter',
			astronaut: 'Astronaut',
			alien: 'Alien',
			ninja: 'Ninja',
			dragon: 'Drache',
			knight: 'Ritter',
			ghost: 'Geist',
			cybercat: 'Cyberkatze',
			slime: 'Schleim',
			pirate: 'Weltraumpirat',
			gamer: 'Gamer'
		}
	},
	openRoom: {
		whoGetsIn: 'Wer kommt rein',
		approval: 'Ich lasse jede Person einzeln rein',
		open: 'Alle können beitreten',
		openHint: 'Wer dein @ kennt, kommt direkt rein, solange der Raum offen ist.',
		submit: 'Meinen Raum öffnen',
		submitting: 'Wird geöffnet…',
		failed: 'Dein Raum konnte nicht geöffnet werden. Versuch es noch einmal.'
	},
	roomOffline: {
		liveTitle: (h: string) => `@${h} ist gerade nicht live`,
		liveBody: (h: string) =>
			`Dieser Raum öffnet sich, wenn @${h} einen Anruf startet. Schau bald wieder vorbei – oder öffne in Sekunden deinen eigenen Raum.`,
		createOwn: 'Eigenen Raum erstellen',
		tryAgain: 'Erneut versuchen',
		freeTitle: (h: string) => `@${h} ist noch frei!`,
		freeBody: 'Niemand hat diesen Raum bisher gesichert. Er kann deiner sein.',
		claim: (h: string) => `@${h} sichern`,
		home: 'Zur Startseite'
	},
	footer: { freeSoftware: 'Freie Software', sourceCode: 'Quellcode' }
};
