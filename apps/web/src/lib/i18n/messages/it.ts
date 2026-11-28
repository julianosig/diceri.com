import type { Messages } from './en.ts';

export const it: Messages = {
	meta: { title: 'Diceri — alternativa a Google Meet con traduzione vocale bidirezionale' },
	header: {
		signOut: 'Esci',
		deleteVoice: 'Elimina la mia voce',
		deleteVoiceConfirm:
			'Eliminare la registrazione salvata della tua voce? Ne registrerai una nuova alla prossima chiamata.',
		voiceDeleted: 'La registrazione della tua voce è stata eliminata.',
		voiceDeleteFailed:
			'Non è stato possibile eliminare la tua voce ora. Non è stato eliminato nulla; riprova.',
		language: 'Lingua',
		toDark: 'Passa al tema scuro',
		toLight: 'Passa al tema chiaro'
	},
	hero: {
		headline: 'L’alternativa a Google Meet',
		promise: 'dove parli e ascolti nella tua lingua.',
		voice: 'Tutti nella riunione sentono la tua voce nella loro lingua.',
		openSource: 'Siamo open source',
		illustration:
			'Illustrazione: una persona parla, Diceri traduce nel mezzo e l’altra persona la sente nella propria lingua.'
	},
	onboarding: {
		label: 'Nome utente della stanza',
		mascotTitle: 'Scegli la mascotte della tua stanza',
		mascotHint: 'Accoglie chiunque passi a trovarti',
		continue: 'Continua',
		back: 'Indietro',
		usernamePlaceholder: 'nome_stanza',
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
		tooEasy:
			'Troppo facile da indovinare. Prova con alcune parole scollegate, come lontra-blu-canta.',
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
	lobby: {
		joinRoomTitle: (handle: string) => `Entra in @${handle}`,
		joinRoomHint:
			'Per entrare in questa chiamata, accedi con il tuo nome utente Diceri o creane uno gratis se sei nuovo. Ci vuole un minuto e permette a tutti di sentirti nella loro lingua, con la tua voce.',
		camera: 'Fotocamera',
		cameraOn: 'Attiva fotocamera',
		cameraOff: 'Disattiva fotocamera',
		cameraBlocked: 'Consenti la fotocamera nel browser per mostrare il tuo video.',
		cameraUnavailable: 'Nessuna fotocamera trovata.',
		joinTitle: 'Partecipa alla riunione',
		yourName: 'Il tuo nome',
		hearEveryoneIn: 'Ascolta tutti in',
		microphone: 'Microfono',
		systemDefault: 'Predefinito di sistema',
		microphonePreview: 'Anteprima del microfono',
		joinNow: 'Partecipa ora',
		askToJoin: 'Chiedi di partecipare',
		joining: 'Accesso in corso…',
		micBlocked:
			'L’accesso al microfono è stato bloccato. Consentilo dalla barra degli indirizzi del browser.',
		micUnavailable: 'Impossibile accedere al microfono.',
		waitingToJoin: 'In attesa di entrare',
		deny: 'Rifiuta',
		admit: 'Ammetti',
		wantsToJoin: (name: string) => `${name} vuole partecipare a questa chiamata`,
		hears: (language: string) => `(ascolta in ${language})`,
		dismiss: 'Ignora',
		cantJoin: 'Non puoi partecipare a questa riunione',
		declined: 'L’organizzatore ha rifiutato la tua richiesta.',
		somethingWrong: 'Qualcosa è andato storto',
		back: 'Indietro',
		asking: 'Richiesta di accesso in corso…',
		willJoin: 'Entrerai nella riunione quando qualcuno ti farà entrare.',
		cancel: 'Annulla'
	},
	voice: {
		requiredHint: (seconds: number) =>
			`Leggi una frase breve per ${String(seconds)} secondi: tutti sentiranno le tue traduzioni con la tua voce. Viene conservata, cifrata, con il tuo account.`,
		pro: 'Pro',
		paidSoon: 'Presto disponibile come funzione a pagamento.',
		section: 'La tua voce',
		recording: 'Registrazione · leggi ad alta voce',
		secondsLeft: (seconds: number) => `${String(seconds)} s`,
		done: 'Fatto',
		keepReading: (seconds: number) => `Continua a leggere (min. ${String(seconds)} s)`,
		creating: 'Creazione della tua voce…',
		ready: 'La tua voce è pronta',
		redo: 'Rifai',
		readyHint:
			'Le traduzioni avranno la tua voce. La registrazione è conservata, cifrata, con il tuo account.',
		keptReady: 'La tua voce salvata è pronta',
		keptHint:
			'Stiamo usando la registrazione conservata con il tuo account. Eliminala quando vuoi dal tuo menu @.',
		title: 'Fai parlare le traduzioni con la tua voce',
		hint: (seconds: number) =>
			`Leggi una frase breve per ${String(seconds)} secondi. Facoltativo; conservato, cifrato, con il tuo account.`,
		tryAgain: 'Riprova',
		record: (seconds: number) => `Registra la mia voce (${String(seconds)} s)`,
		noVoice: 'Non ti abbiamo sentito bene. Leggi la frase ad alta voce e riprova.',
		micNeeded: 'Serve l’accesso al microfono per registrare la tua voce.',
		failed: 'Impossibile creare la tua voce.'
	},
	call: {
		turnOnCamera: 'Attiva fotocamera',
		turnOffCamera: 'Disattiva fotocamera',
		turnOnMic: 'Attiva microfono',
		turnOffMic: 'Disattiva microfono',
		leaveCall: 'Abbandona la chiamata',
		justLeave: 'Abbandona e basta',
		endForEveryone: 'Termina la chiamata per tutti',
		people: 'Persone',
		audioSettings: 'Impostazioni audio',
		guest: 'Ospite',
		enableAudio: 'Fai clic per attivare l’audio degli altri partecipanti',
		connecting: 'Connessione…',
		inCall: 'In chiamata',
		closePanel: 'Chiudi pannello',
		you: (name: string) => `${name} (Tu)`,
		micOff: 'Microfono disattivato',
		hearsEveryoneIn: 'Ascolta tutti in',
		hears: (language: string) => `ascolta in ${language}`,
		meetingDetails: 'Dettagli della riunione',
		meetingReady: 'La tua riunione è pronta',
		close: 'Chiudi',
		shareLink:
			'Condividi questo link con la persona con cui vuoi parlare. Ti verrà chiesto di farla entrare.',
		copyLink: 'Copia link della riunione',
		linkCopied: 'Link copiato',
		captions: 'Sottotitoli',
		filterUnsupported: 'Il filtro antirumore avanzato non è supportato da questo browser.',
		filterFailed: 'Il filtro antirumore avanzato non è riuscito.',
		micUnpublished: 'Impossibile trasmettere il microfono.',
		micChangeFailed:
			'Non è stato possibile usare quel microfono; stai ancora usando quello precedente.',
		micLost: 'Il microfono ha smesso di funzionare. Scegline uno nelle impostazioni audio.'
	},
	audio: {
		microphone: 'Microfono',
		systemDefault: 'Predefinito di sistema',
		noiseAndEcho: 'Rumore ed eco',
		aiFilter: 'Filtro antirumore IA (Krisp)',
		filterOff: 'Disattivato',
		filterNoise: 'Rumore',
		filterVoice: 'Solo la mia voce',
		filterOffHint: 'Solo l’elaborazione del browser.',
		filterNoiseHint: 'Elimina il rumore di fondo, mantiene tutte le voci.',
		filterVoiceHint: 'Elimina anche altre persone che parlano vicino (fatturato a parte).',
		echo: 'Cancellazione dell’eco',
		echoHint: 'Impedisce che la voce dell’altra persona ritorni dal tuo microfono.',
		suppression: 'Soppressione del rumore del browser',
		suppressionOff: 'Disattivata mentre il filtro IA è attivo.',
		suppressionHint: 'Filtro di base integrato.',
		autoGain: 'Guadagno automatico',
		autoGainHint: 'Mantiene stabile il tuo volume.',
		outgoingLevel: 'Livello in uscita (dopo il filtro)',
		muted: 'Disattivato',
		hearMyself: 'Ascoltami',
		headphones: 'Usa le cuffie, o la stanza ti sentirà due volte.',
		monitorOff: 'Disattivato',
		local: 'Locale',
		viaServer: 'Tramite server',
		monitorOffHint: 'Non senti la tua voce.',
		localHint:
			'Ascolta il tuo microfono dopo il filtro antirumore, all’istante. Per valutare il filtro.',
		viaServerHint:
			'Ascoltati dopo un giro di andata e ritorno sul server multimediale. Per valutare la trasmissione.',
		connection: 'Connessione',
		direction: 'Direzione',
		bitrate: 'Bitrate',
		loss: 'Perdita',
		jitter: 'Jitter',
		rtt: 'RTT',
		sending: 'Invio',
		receiving: 'Ricezione'
	},
	meeting: {
		left: 'Hai abbandonato la riunione',
		disconnected: 'Sei stato disconnesso',
		endedByMe: 'Hai terminato la chiamata per tutti',
		endedByHost: 'L’organizzatore ha terminato la chiamata',
		rejoin: 'Partecipa di nuovo',
		home: 'Torna alla schermata iniziale'
	},
	transcript: {
		title: 'Trascrizione',
		copy: 'Copia',
		download: 'Scarica',
		empty: 'Non è stato detto nulla mentre eri nella chiamata.',
		copied: 'Trascrizione copiata'
	},
	plans: {
		betaLeft: 'posti liberi',
		betaDetail: (total: number) => `Beta pubblica · ${String(total)} in totale`,
		betaFull: 'Per ora i posti sono esauriti',
		usedThisMonth: (used: number, total: number) =>
			`${String(used)} su ${String(total)} min usati questo mese`,
		freeLine: (minutes: number) => `Gratis: ${String(minutes)} min di chiamate ogni mese.`
	},
	limits: {
		full: 'Questa chiamata è piena: c’è posto per due persone. Riprova quando qualcuno esce.',
		beta: 'Diceri è in beta pubblica e per ora tutti i posti sono occupati. Riprova più tardi.',
		network:
			'Questa rete ha già prenotato il numero massimo di nomi di stanza. Accedi con uno di quelli.',
		quota:
			'Hai usato i minuti di chiamata del tuo piano per questo mese. Si rinnovano il giorno 1 (UTC).',
		daily: 'Hai usato i minuti di chiamata gratuiti di oggi. Si rinnovano a mezzanotte (UTC).',
		monthly:
			'La capacità gratuita di Diceri per questo mese è esaurita. Si rinnova il giorno 1 (UTC).',
		busy: 'Diceri ha raggiunto la capacità massima in questo momento. Riprova tra qualche minuto.',
		timeLeft: (time: string) => `Restano ${time}`,
		timeUp: 'Il tuo tempo di chiamata gratuito è finito',
		timeUpHint: 'Hai usato i minuti gratuiti di questo mese. Si rinnovano il giorno 1 (UTC).'
	},
	terms: {
		title: 'Termini di utilizzo',
		updated: 'Ultimo aggiornamento: 27 novembre 2026',
		notice:
			'Creando la tua stanza accetti i termini di utilizzo, inclusa la conservazione della registrazione della tua voce, cifrata, per ricreare la tua voce nelle chiamate.',
		read: 'Leggi i termini di utilizzo',
		sections: [
			{
				heading: 'Cos’è Diceri',
				body: 'Diceri è un servizio di videochiamate con traduzione vocale in tempo reale, gestito come software libero e open source (AGPL-3.0). Ogni account ha una stanza su diceri.com/@username. Usare Diceri significa accettare questi termini.'
			},
			{
				heading: 'Il tuo account',
				body: 'Ti registri con un nome utente e una password; non viene chiesta nessuna email. Tieni la password per te. Si possono creare solo pochi account per rete, e il piano gratuito include una quantità mensile di minuti di chiamata.'
			},
			{
				heading: 'La tua voce',
				body: 'Per entrare in una chiamata registri circa 10 secondi della tua voce. La usiamo per creare una copia della tua voce presso il nostro fornitore vocale, così le persone sentono le tue traduzioni con la tua voce. Conserviamo la registrazione con il tuo account, cifrata, per poter ricreare la tua voce nelle chiamate successive, anche con un altro fornitore vocale se dovessimo cambiarlo. Viene usata solo per pronunciare le tue traduzioni, mai per parlare al posto di qualcun altro, e non viene mai venduta né condivisa per altri scopi.'
			},
			{
				heading: 'Eliminare la tua voce',
				body: 'Puoi eliminare la tua registrazione in qualsiasi momento con "Elimina la mia voce" nel tuo menu @; viene eliminata anche con il tuo account. La copia della tua voce creata presso il fornitore di voce resta disponibile per le tue chiamate finché la registrazione è salvata, e viene eliminata insieme a essa.'
			},
			{
				heading: 'Cosa conserviamo e cosa no',
				body: 'Conserviamo il tuo nome utente, una forma protetta della tua password, la mascotte della tua stanza, il tuo piano, quanti minuti di chiamata hai usato, un’impronta cifrata della rete da cui ti sei registrato (mai il tuo indirizzo IP) e la registrazione della tua voce. Non registriamo le chiamate e non conserviamo ciò che è stato detto: trascrizioni e sottotitoli restano nel tuo browser.'
			},
			{
				heading: 'Fornitori di servizi',
				body: 'Le chiamate passano per LiveKit; il parlato viene trascritto, tradotto e pronunciato da ElevenLabs, Inworld, Soniox e Google Gemini. Ricevono solo ciò che serve per fornire la chiamata.'
			},
			{
				heading: 'Uso corretto',
				body: 'Non usare Diceri per violare la legge, molestare le persone o spacciarti per qualcun altro. Possiamo limitare o chiudere gli account che abusano del servizio. Diceri è fornito così com’è, senza garanzie.'
			},
			{
				heading: 'Modifiche',
				body: 'Possiamo aggiornare questi termini; la data qui sopra indica la versione più recente. Domande: apri una issue su github.com/julianosig/diceri.com.'
			}
		]
	},
	footer: {
		freeSoftware: 'Software libero',
		sourceCode: 'Codice sorgente',
		terms: 'Termini di utilizzo'
	}
};
