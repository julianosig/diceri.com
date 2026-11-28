import type { Messages } from './en.ts';

export const de: Messages = {
	meta: { title: 'Diceri — Google-Meet-Alternative mit Sprachübersetzung in beide Richtungen' },
	header: {
		signOut: 'Abmelden',
		deleteVoice: 'Meine Stimme löschen',
		deleteVoiceConfirm:
			'Deine gespeicherte Sprachaufnahme löschen? Beim nächsten Anruf nimmst du eine neue auf.',
		voiceDeleted: 'Deine Sprachaufnahme wurde gelöscht.',
		voiceDeleteFailed:
			'Deine Stimme konnte gerade nicht gelöscht werden. Es wurde nichts gelöscht; versuch es erneut.',
		language: 'Sprache',
		toDark: 'Zum dunklen Design wechseln',
		toLight: 'Zum hellen Design wechseln'
	},
	hero: {
		headline: 'Die Google-Meet-Alternative,',
		promise: 'in der du in deiner Sprache sprichst und hörst.',
		voice: 'Alle im Meeting hören deine Stimme in ihrer Sprache.',
		openSource: 'Wir sind Open Source',
		illustration:
			'Illustration: Eine Person spricht, Diceri übersetzt in der Mitte, und die andere Person hört es in ihrer eigenen Sprache.'
	},
	onboarding: {
		label: 'Raum-Benutzername',
		mascotTitle: 'Wähle das Maskottchen deines Raums',
		mascotHint: 'Es begrüßt alle, die vorbeischauen',
		continue: 'Weiter',
		back: 'Zurück',
		usernamePlaceholder: 'raum_name',
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
		tooEasy:
			'Zu leicht zu erraten. Versuche ein paar zusammenhanglose Wörter, etwa blauer-otter-singt.',
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
	lobby: {
		joinRoomTitle: (handle: string) => `@${handle} beitreten`,
		joinRoomHint:
			'Um diesem Anruf beizutreten, melde dich mit deinem Diceri-Benutzernamen an oder erstelle kostenlos einen, wenn du neu bist. Das dauert eine Minute, und alle hören dich in ihrer Sprache – mit deiner eigenen Stimme.',
		camera: 'Kamera',
		cameraOn: 'Kamera einschalten',
		cameraOff: 'Kamera ausschalten',
		cameraBlocked: 'Erlaube die Kamera im Browser, um dein Video zu zeigen.',
		cameraUnavailable: 'Keine Kamera gefunden.',
		joinTitle: 'Dem Meeting beitreten',
		yourName: 'Dein Name',
		hearEveryoneIn: 'Alle hören auf',
		microphone: 'Mikrofon',
		systemDefault: 'Systemstandard',
		microphonePreview: 'Mikrofonvorschau',
		joinNow: 'Jetzt teilnehmen',
		askToJoin: 'Teilnahme anfragen',
		joining: 'Wird beigetreten…',
		micBlocked:
			'Der Mikrofonzugriff wurde blockiert. Erlaube ihn in der Adressleiste des Browsers.',
		micUnavailable: 'Kein Zugriff auf das Mikrofon.',
		waitingToJoin: 'Wartet auf Einlass',
		deny: 'Ablehnen',
		admit: 'Einlassen',
		wantsToJoin: (name: string) => `${name} möchte an diesem Anruf teilnehmen`,
		hears: (language: string) => `(hört auf ${language})`,
		dismiss: 'Schließen',
		cantJoin: 'Du kannst diesem Meeting nicht beitreten',
		declined: 'Der Host hat deine Anfrage abgelehnt.',
		somethingWrong: 'Etwas ist schiefgelaufen',
		back: 'Zurück',
		asking: 'Einlass wird angefragt…',
		willJoin: 'Du trittst dem Meeting bei, sobald dich jemand reinlässt.',
		cancel: 'Abbrechen'
	},
	voice: {
		requiredHint: (seconds: number) =>
			`Lies ${String(seconds)} Sekunden lang einen kurzen Satz vor: Alle hören deine Übersetzungen mit deiner eigenen Stimme. Wird verschlüsselt mit deinem Konto gespeichert.`,
		pro: 'Pro',
		paidSoon: 'Bald als kostenpflichtige Funktion verfügbar.',
		section: 'Deine Stimme',
		recording: 'Aufnahme · laut vorlesen',
		secondsLeft: (seconds: number) => `${String(seconds)} s`,
		done: 'Fertig',
		keepReading: (seconds: number) => `Weiterlesen (mind. ${String(seconds)} s)`,
		creating: 'Deine Stimme wird erstellt…',
		ready: 'Deine Stimme ist bereit',
		redo: 'Neu aufnehmen',
		readyHint:
			'Übersetzungen klingen wie du. Die Aufnahme wird verschlüsselt mit deinem Konto gespeichert.',
		keptReady: 'Deine gespeicherte Stimme ist bereit',
		keptHint:
			'Die mit deinem Konto gespeicherte Aufnahme wird verwendet. Lösche sie jederzeit über dein @-Menü.',
		title: 'Lass Übersetzungen wie dich klingen',
		hint: (seconds: number) =>
			`Lies ${String(seconds)} Sekunden lang einen kurzen Satz vor. Optional; wird verschlüsselt mit deinem Konto gespeichert.`,
		tryAgain: 'Erneut versuchen',
		record: (seconds: number) => `Meine Stimme aufnehmen (${String(seconds)} s)`,
		noVoice:
			'Wir konnten dich nicht deutlich hören. Lies den Satz laut vor und versuch es noch einmal.',
		micNeeded: 'Für die Aufnahme deiner Stimme ist Mikrofonzugriff nötig.',
		failed: 'Deine Stimme konnte nicht erstellt werden.'
	},
	call: {
		turnOnCamera: 'Kamera einschalten',
		turnOffCamera: 'Kamera ausschalten',
		turnOnMic: 'Mikrofon einschalten',
		turnOffMic: 'Mikrofon ausschalten',
		leaveCall: 'Anruf verlassen',
		justLeave: 'Nur den Anruf verlassen',
		endForEveryone: 'Anruf für alle beenden',
		people: 'Personen',
		audioSettings: 'Audioeinstellungen',
		guest: 'Gast',
		enableAudio: 'Klicke, um den Ton der anderen Teilnehmenden zu aktivieren',
		connecting: 'Verbindung wird hergestellt…',
		inCall: 'Im Anruf',
		closePanel: 'Bereich schließen',
		you: (name: string) => `${name} (Du)`,
		micOff: 'Mikrofon aus',
		hearsEveryoneIn: 'Hört alle auf',
		hears: (language: string) => `hört auf ${language}`,
		meetingDetails: 'Meeting-Details',
		meetingReady: 'Dein Meeting ist bereit',
		close: 'Schließen',
		shareLink:
			'Teile diesen Link mit der Person, mit der du sprechen möchtest. Du wirst gefragt, ob du sie reinlässt.',
		copyLink: 'Meeting-Link kopieren',
		linkCopied: 'Link kopiert',
		captions: 'Untertitel',
		filterUnsupported: 'Der erweiterte Rauschfilter wird in diesem Browser nicht unterstützt.',
		filterFailed: 'Der erweiterte Rauschfilter ist fehlgeschlagen.',
		micUnpublished: 'Das Mikrofon konnte nicht übertragen werden.',
		micChangeFailed:
			'Dieses Mikrofon konnte nicht verwendet werden; du nutzt weiter das vorherige.',
		micLost: 'Dein Mikrofon funktioniert nicht mehr. Wähle eines in den Audioeinstellungen.'
	},
	audio: {
		microphone: 'Mikrofon',
		systemDefault: 'Systemstandard',
		noiseAndEcho: 'Rauschen und Echo',
		aiFilter: 'KI-Rauschfilter (Krisp)',
		filterOff: 'Aus',
		filterNoise: 'Rauschen',
		filterVoice: 'Nur meine Stimme',
		filterOffHint: 'Nur Verarbeitung im Browser.',
		filterNoiseHint: 'Entfernt Hintergrundgeräusche, behält alle Stimmen.',
		filterVoiceHint:
			'Entfernt auch andere Personen, die in der Nähe sprechen (separat abgerechnet).',
		echo: 'Echounterdrückung',
		echoHint: 'Verhindert, dass die Stimme der anderen Person über dein Mikrofon zurückkommt.',
		suppression: 'Rauschunterdrückung des Browsers',
		suppressionOff: 'Aus, solange der KI-Filter aktiv ist.',
		suppressionHint: 'Einfacher integrierter Filter.',
		autoGain: 'Automatische Verstärkung',
		autoGainHint: 'Hält deine Lautstärke gleichmäßig.',
		outgoingLevel: 'Ausgangspegel (nach dem Filter)',
		muted: 'Stummgeschaltet',
		hearMyself: 'Mich selbst hören',
		headphones: 'Nutze Kopfhörer, sonst hört dich der Raum doppelt.',
		monitorOff: 'Aus',
		local: 'Lokal',
		viaServer: 'Über Server',
		monitorOffHint: 'Du hörst dich nicht selbst.',
		localHint: 'Hör dein Mikrofon nach dem Rauschfilter, sofort. Zum Beurteilen des Filters.',
		viaServerHint:
			'Hör dich nach einem Umweg über den Medienserver. Zum Beurteilen der Übertragung.',
		connection: 'Verbindung',
		direction: 'Richtung',
		bitrate: 'Bitrate',
		loss: 'Verlust',
		jitter: 'Jitter',
		rtt: 'RTT',
		sending: 'Senden',
		receiving: 'Empfangen'
	},
	meeting: {
		left: 'Du hast das Meeting verlassen',
		disconnected: 'Die Verbindung wurde getrennt',
		endedByMe: 'Du hast den Anruf für alle beendet',
		endedByHost: 'Der Host hat den Anruf beendet',
		rejoin: 'Erneut beitreten',
		home: 'Zurück zum Startbildschirm'
	},
	transcript: {
		title: 'Transkript',
		copy: 'Kopieren',
		download: 'Herunterladen',
		empty: 'Während du im Anruf warst, wurde nichts gesagt.',
		copied: 'Transkript kopiert'
	},
	plans: {
		betaLeft: 'freie Plätze',
		betaDetail: (total: number) => `Öffentliche Beta · ${String(total)} insgesamt`,
		betaFull: 'Vorerst sind alle Plätze vergeben',
		usedThisMonth: (used: number, total: number) =>
			`${String(used)} von ${String(total)} Min. diesen Monat genutzt`,
		freeLine: (minutes: number) => `Kostenlos: ${String(minutes)} Min. Anrufe jeden Monat.`
	},
	limits: {
		full: 'Dieser Anruf ist voll: Er hat Platz für zwei Personen. Versuch es erneut, wenn jemand geht.',
		beta: 'Diceri ist in einer öffentlichen Beta und alle Plätze sind vorerst vergeben. Versuch es später noch einmal.',
		network:
			'Dieses Netzwerk hat schon die erlaubte Höchstzahl an Raumnamen gesichert. Melde dich mit einem davon an.',
		quota:
			'Du hast die Anrufminuten deines Tarifs für diesen Monat aufgebraucht. Sie erneuern sich am 1. (UTC).',
		daily:
			'Du hast deine kostenlosen Anrufminuten für heute verbraucht. Sie werden um Mitternacht (UTC) erneuert.',
		monthly:
			'Die kostenlose Kapazität von Diceri ist für diesen Monat aufgebraucht. Sie erneuert sich am 1. (UTC).',
		busy: 'Diceri ist gerade ausgelastet. Versuche es in ein paar Minuten erneut.',
		timeLeft: (time: string) => `Noch ${time}`,
		timeUp: 'Deine kostenlose Anrufzeit ist abgelaufen',
		timeUpHint:
			'Du hast die kostenlosen Minuten dieses Monats verbraucht. Sie erneuern sich am 1. (UTC).'
	},
	terms: {
		title: 'Nutzungsbedingungen',
		updated: 'Zuletzt aktualisiert: 27. November 2026',
		notice:
			'Mit dem Erstellen deines Raums akzeptierst du die Nutzungsbedingungen, einschließlich der verschlüsselten Speicherung deiner Sprachaufnahme, um deine Stimme in Anrufen nachzubilden.',
		read: 'Nutzungsbedingungen lesen',
		sections: [
			{
				heading: 'Was Diceri ist',
				body: 'Diceri ist ein Videoanruf-Dienst mit Live-Sprachübersetzung, betrieben als freie Open-Source-Software (AGPL-3.0). Jedes Konto hat einen Raum unter diceri.com/@username. Wer Diceri nutzt, akzeptiert diese Bedingungen.'
			},
			{
				heading: 'Dein Konto',
				body: 'Du registrierst dich mit Benutzername und Passwort; eine E-Mail-Adresse wird nicht abgefragt. Behalte dein Passwort für dich. Pro Netzwerk können nur wenige Konten erstellt werden, und der kostenlose Tarif enthält ein monatliches Kontingent an Anrufminuten.'
			},
			{
				heading: 'Deine Stimme',
				body: 'Um einem Anruf beizutreten, nimmst du etwa 10 Sekunden deiner Stimme auf. Damit erstellen wir bei unserem Sprachanbieter eine Kopie deiner Stimme, sodass andere deine Übersetzungen mit deiner eigenen Stimme hören. Wir speichern die Aufnahme verschlüsselt mit deinem Konto, um deine Stimme in späteren Anrufen nachbilden zu können, auch mit einem anderen Sprachanbieter, falls wir wechseln. Sie wird nur verwendet, um deine eigenen Übersetzungen zu sprechen, niemals, um für jemand anderen zu sprechen, und wird niemals verkauft oder für andere Zwecke weitergegeben.'
			},
			{
				heading: 'Deine Stimme löschen',
				body: 'Du kannst deine Aufnahme jederzeit mit „Meine Stimme löschen“ in deinem @-Menü löschen; sie wird auch mit deinem Konto gelöscht. Die Kopie deiner Stimme beim Stimmanbieter bleibt für deine Anrufe erhalten, solange deine Aufnahme gespeichert ist, und wird mit ihr gelöscht.'
			},
			{
				heading: 'Was wir speichern und was nicht',
				body: 'Wir speichern deinen Benutzernamen, eine geschützte Form deines Passworts, das Maskottchen deines Raums, deinen Tarif, wie viele Anrufminuten du verbraucht hast, einen verschlüsselten Fingerabdruck des Netzwerks, über das du dich registriert hast (niemals deine IP-Adresse), und deine Sprachaufnahme. Wir zeichnen keine Anrufe auf und speichern nicht, was gesagt wurde: Transkripte und Untertitel bleiben in deinem Browser.'
			},
			{
				heading: 'Dienstleister',
				body: 'Anrufe laufen über LiveKit; Sprache wird von ElevenLabs, Inworld, Soniox und Google Gemini transkribiert, übersetzt und gesprochen. Sie erhalten nur, was für den Anruf nötig ist.'
			},
			{
				heading: 'Faire Nutzung',
				body: 'Nutze Diceri nicht, um gegen Gesetze zu verstoßen, Menschen zu belästigen oder dich als jemand anderes auszugeben. Wir können Konten, die den Dienst missbrauchen, einschränken oder schließen. Diceri wird so bereitgestellt, wie es ist, ohne Gewährleistung.'
			},
			{
				heading: 'Änderungen',
				body: 'Wir können diese Bedingungen aktualisieren; das Datum oben zeigt die neueste Version. Fragen: Eröffne ein Issue unter github.com/julianosig/diceri.com.'
			}
		]
	},
	footer: { freeSoftware: 'Freie Software', sourceCode: 'Quellcode', terms: 'Nutzungsbedingungen' }
};
