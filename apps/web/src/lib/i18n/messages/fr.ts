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
		voice: 'Chacun dans la réunion entend votre voix dans sa langue.',
		openSource: 'Nous sommes open source',
		illustration:
			'Illustration : une personne parle, Diceri traduit au milieu et l’autre personne l’entend dans sa propre langue.'
	},
	onboarding: {
		label: 'Nom d’utilisateur de la salle',
		mascotTitle: 'Choisissez la mascotte de votre salle',
		mascotHint: 'Elle accueille chaque visiteur',
		continue: 'Continuer',
		back: 'Retour',
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
	lobby: {
		joinTitle: 'Rejoindre la réunion',
		yourName: 'Votre nom',
		hearEveryoneIn: 'Entendre tout le monde en',
		microphone: 'Micro',
		systemDefault: 'Valeur par défaut du système',
		microphonePreview: 'Aperçu du micro',
		joinNow: 'Participer',
		askToJoin: 'Demander à participer',
		joining: 'Connexion…',
		micBlocked:
			'L’accès au micro a été bloqué. Autorisez-le dans la barre d’adresse du navigateur.',
		micUnavailable: 'Impossible d’accéder au micro.',
		waitingToJoin: 'En attente pour rejoindre',
		deny: 'Refuser',
		admit: 'Accepter',
		wantsToJoin: (name: string) => `${name} souhaite rejoindre cet appel`,
		hears: (language: string) => `(entend en ${language})`,
		dismiss: 'Ignorer',
		cantJoin: 'Vous ne pouvez pas rejoindre cette réunion',
		declined: 'L’organisateur a refusé votre demande.',
		somethingWrong: 'Une erreur est survenue',
		back: 'Retour',
		asking: 'Demande d’accès en cours…',
		willJoin: 'Vous rejoindrez la réunion dès que quelqu’un vous aura accepté.',
		cancel: 'Annuler'
	},
	voice: {
		pro: 'Pro',
		paidSoon: 'Bientôt disponible en option payante.',
		section: 'Votre voix',
		recording: 'Enregistrement · lisez à voix haute',
		secondsLeft: (seconds: number) => `${String(seconds)} s`,
		done: 'Terminé',
		keepReading: (seconds: number) => `Continuez à lire (${String(seconds)} s min.)`,
		creating: 'Création de votre voix…',
		ready: 'Votre voix est prête',
		redo: 'Recommencer',
		readyHint: 'Les traductions auront votre voix. Supprimée quand vous partez.',
		title: 'Donnez votre voix aux traductions',
		hint: (seconds: number) =>
			`Lisez une courte phrase pendant ${String(seconds)} secondes. Facultatif, et supprimé quand vous partez.`,
		tryAgain: 'Réessayer',
		record: (seconds: number) => `Enregistrer ma voix (${String(seconds)} s)`,
		noVoice: 'Nous ne vous avons pas bien entendu. Lisez la phrase à voix haute et réessayez.',
		micNeeded: 'L’accès au micro est nécessaire pour enregistrer votre voix.',
		failed: 'Impossible de créer votre voix.'
	},
	call: {
		turnOnMic: 'Activer le micro',
		turnOffMic: 'Désactiver le micro',
		leaveCall: 'Quitter l’appel',
		justLeave: 'Quitter simplement l’appel',
		endForEveryone: 'Mettre fin à l’appel pour tous',
		people: 'Participants',
		audioSettings: 'Paramètres audio',
		guest: 'Invité',
		enableAudio: 'Cliquez pour activer l’audio des autres participants',
		connecting: 'Connexion…',
		inCall: 'En appel',
		closePanel: 'Fermer le panneau',
		you: (name: string) => `${name} (Vous)`,
		micOff: 'Micro désactivé',
		hearsEveryoneIn: 'Entend tout le monde en',
		hears: (language: string) => `entend en ${language}`,
		meetingDetails: 'Détails de la réunion',
		meetingReady: 'Votre réunion est prête',
		close: 'Fermer',
		shareLink:
			'Partagez ce lien avec la personne à qui vous voulez parler. Vous devrez l’accepter pour qu’elle entre.',
		copyLink: 'Copier le lien de la réunion',
		linkCopied: 'Lien copié',
		captions: 'Sous-titres',
		filterUnsupported: 'Le filtre anti-bruit avancé n’est pas pris en charge par ce navigateur.',
		filterFailed: 'Le filtre anti-bruit avancé a échoué.',
		micUnpublished: 'Impossible de diffuser le micro.'
	},
	audio: {
		microphone: 'Micro',
		systemDefault: 'Valeur par défaut du système',
		noiseAndEcho: 'Bruit et écho',
		aiFilter: 'Filtre anti-bruit IA (Krisp)',
		filterOff: 'Désactivé',
		filterNoise: 'Bruit',
		filterVoice: 'Ma voix uniquement',
		filterOffHint: 'Traitement du navigateur uniquement.',
		filterNoiseHint: 'Supprime le bruit de fond, garde toutes les voix.',
		filterVoiceHint: 'Supprime aussi les autres personnes qui parlent à côté (facturé à part).',
		echo: 'Annulation de l’écho',
		echoHint: 'Empêche la voix de l’autre personne de revenir par votre micro.',
		suppression: 'Réduction du bruit du navigateur',
		suppressionOff: 'Désactivée tant que le filtre IA est actif.',
		suppressionHint: 'Filtre de base intégré.',
		autoGain: 'Gain automatique',
		autoGainHint: 'Garde votre volume stable.',
		outgoingLevel: 'Niveau sortant (après filtrage)',
		muted: 'Coupé',
		hearMyself: 'M’entendre',
		headphones: 'Utilisez un casque, sinon la salle vous entendra deux fois.',
		monitorOff: 'Désactivé',
		local: 'Local',
		viaServer: 'Via le serveur',
		translated: 'Traduit',
		monitorOffHint: 'Vous ne vous entendez pas.',
		localHint:
			'Écoutez votre micro après le filtre anti-bruit, instantanément. Pour juger le filtre.',
		viaServerHint:
			'Écoutez-vous après un aller-retour par le serveur média. Pour juger la transmission.',
		translatedHint:
			'Écoutez votre propre voix traduite par l’agent. Parlez, faites une pause, écoutez.',
		translateMeInto: 'Me traduire en',
		sameLanguage: (language: string) =>
			`Vous parlez ${language}, la langue que vous avez choisie : il n’y a rien à traduire. Choisissez une autre langue pour vous entendre traduit.`,
		connection: 'Connexion',
		direction: 'Sens',
		bitrate: 'Débit',
		loss: 'Perte',
		jitter: 'Gigue',
		rtt: 'RTT',
		sending: 'Envoi',
		receiving: 'Réception'
	},
	meeting: {
		left: 'Vous avez quitté la réunion',
		endedByMe: 'Vous avez mis fin à l’appel pour tous',
		endedByHost: 'L’organisateur a mis fin à l’appel',
		rejoin: 'Revenir',
		home: 'Revenir à l’écran d’accueil'
	},
	transcript: {
		title: 'Transcription',
		copy: 'Copier',
		download: 'Télécharger',
		empty: 'Rien n’a été dit pendant votre présence dans l’appel.',
		copied: 'Transcription copiée'
	},
	plans: {
		usedThisMonth: (used: number, total: number) =>
			`${String(used)} sur ${String(total)} min utilisées ce mois-ci`,
		freeLine: (minutes: number) => `Gratuit : ${String(minutes)} min d’appels chaque mois.`
	},
	limits: {
		quota:
			'Vous avez utilisé les minutes d’appel de votre offre ce mois-ci. Elles se renouvellent le 1er (UTC).',
		daily:
			'Vous avez utilisé vos minutes d’appel gratuites du jour. Elles se renouvellent à minuit (UTC).',
		monthly:
			'La capacité gratuite de Diceri pour ce mois est épuisée. Elle se renouvelle le 1er (UTC).',
		busy: 'Diceri est à pleine capacité pour le moment. Réessayez dans quelques minutes.',
		timeLeft: (time: string) => `${time} restantes`,
		timeUp: 'Votre temps d’appel gratuit est écoulé',
		timeUpHint:
			'Vous avez utilisé les minutes gratuites de ce mois. Elles se renouvellent le 1er (UTC).'
	},
	footer: { freeSoftware: 'Logiciel libre', sourceCode: 'Code source' }
};
