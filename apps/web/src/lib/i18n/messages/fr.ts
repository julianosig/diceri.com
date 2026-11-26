import type { Messages } from './en.ts';

export const fr: Messages = {
	meta: { title: 'Diceri — alternative à Google Meet avec traduction vocale bidirectionnelle' },
	header: {
		signOut: 'Se déconnecter',
		language: 'Langue',
		toDark: 'Passer au thème sombre',
		toLight: 'Passer au thème clair'
	},
	hero: {
		headline: 'L’alternative à Google Meet',
		promise: 'où vous parlez et écoutez dans votre propre langue.',
		voice: 'Ils vous entendent dans leur langue — avec votre propre voix.',
		openSource: 'Open source sur GitHub',
		illustration:
			'Illustration : une personne parle, Diceri traduit au milieu et l’autre personne l’entend dans sa propre langue.'
	},
	onboarding: {
		label: 'Connectez-vous ou réservez le nom de votre salle',
		usernamePlaceholder: 'votrenom',
		checking: (handle: string) => `Vérification de @${handle}…`,
		available: (handle: string) => `@${handle} est libre — réservez-le`,
		welcomeBack: (handle: string) => `Bon retour, @${handle}`,
		invalid: '3 à 20 lettres (a–z), chiffres ou _, en commençant par une lettre.',
		checkFailed: 'Impossible de vérifier le nom. Réessayez.',
		password: 'Mot de passe',
		repeatPassword: 'Répétez le mot de passe',
		newPasswordPlaceholder: (min: number) => `Au moins ${String(min)} caractères`,
		passwordPlaceholder: 'Votre mot de passe',
		repeatPlaceholder: 'Saisissez-le à nouveau',
		passwordsMatch: 'Les mots de passe correspondent',
		tooShort: (min: number) => `Utilisez au moins ${String(min)} caractères.`,
		tooLong: (max: number) => `Utilisez au plus ${String(max)} caractères.`,
		mismatch: 'Les mots de passe ne correspondent pas.',
		create: 'Créer ma salle',
		creating: 'Création…',
		signIn: 'Se connecter',
		signingIn: 'Connexion…',
		taken: 'Ce nom vient d’être pris.',
		wrongPassword: 'Mot de passe incorrect.',
		failed: 'Une erreur est survenue. Réessayez.'
	},
	mascots: {
		legend: 'Mascotte de la salle',
		names: {
			wizard: 'Sorcier',
			robot: 'Robot',
			astronaut: 'Astronaute',
			alien: 'Extraterrestre',
			ninja: 'Ninja',
			dragon: 'Dragon',
			knight: 'Chevalier',
			ghost: 'Fantôme',
			cybercat: 'Chat cyber',
			slime: 'Slime',
			pirate: 'Pirate de l’espace',
			gamer: 'Gamer'
		}
	},
	openRoom: {
		whoGetsIn: 'Qui peut entrer',
		approval: 'J’approuve chaque personne',
		open: 'Tout le monde peut entrer',
		openHint: 'Quiconque connaît votre @ entre directement tant que la salle est ouverte.',
		submit: 'Ouvrir ma salle',
		submitting: 'Ouverture…',
		failed: 'Impossible d’ouvrir votre salle. Réessayez.'
	},
	roomOffline: {
		liveTitle: (h: string) => `@${h} n’est pas en direct pour le moment`,
		liveBody: (h: string) =>
			`Cette salle s’ouvre quand @${h} lance un appel. Revenez bientôt — ou ouvrez votre propre salle en quelques secondes.`,
		createOwn: 'Créer ma propre salle',
		tryAgain: 'Réessayer',
		freeTitle: (h: string) => `@${h} est encore libre !`,
		freeBody: 'Personne n’a encore réservé cette salle. Elle peut être à vous.',
		claim: (h: string) => `Réserver @${h}`,
		home: 'Retour à l’accueil'
	},
	footer: { freeSoftware: 'Logiciel libre', sourceCode: 'Code source' }
};
