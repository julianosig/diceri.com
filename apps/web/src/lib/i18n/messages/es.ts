import type { Messages } from './en.ts';

export const es: Messages = {
	meta: { title: 'Diceri — alternativa a Google Meet con traducción de voz bidireccional' },
	header: {
		signOut: 'Cerrar sesión',
		deleteVoice: 'Borrar mi voz',
		deleteVoiceConfirm:
			'¿Borrar la grabación guardada de tu voz? Grabarás una nueva en tu próxima llamada.',
		voiceDeleted: 'Se borró la grabación de tu voz.',
		language: 'Idioma',
		toDark: 'Cambiar al tema oscuro',
		toLight: 'Cambiar al tema claro'
	},
	hero: {
		headline: 'La alternativa a Google Meet',
		promise: 'donde hablas y escuchas en tu propio idioma.',
		voice: 'Todos en la reunión oyen tu voz en su idioma.',
		openSource: 'Somos código abierto',
		illustration:
			'Ilustración: una persona habla, Diceri traduce en el medio y la otra persona lo oye en su propio idioma.'
	},
	onboarding: {
		label: 'Usuario de la sala',
		mascotTitle: 'Elige la mascota de tu sala',
		mascotHint: 'Recibe a todos los que visitan',
		continue: 'Continuar',
		back: 'Atrás',
		usernamePlaceholder: 'usuario_de_sala',
		checking: (handle: string) => `Comprobando @${handle}…`,
		available: (handle: string) => `@${handle} está libre — resérvalo`,
		welcomeBack: (handle: string) => `Hola de nuevo, @${handle}`,
		invalid: 'De 3 a 20 letras (a–z), números o _, empezando por una letra.',
		checkFailed: 'No se pudo comprobar el nombre. Inténtalo de nuevo.',
		password: 'Contraseña',
		repeatPassword: 'Repite la contraseña',
		newPasswordPlaceholder: (min: number) => `Al menos ${String(min)} caracteres`,
		passwordPlaceholder: 'Tu contraseña',
		repeatPlaceholder: 'Escríbela otra vez',
		passwordsMatch: 'Las contraseñas coinciden',
		tooShort: (min: number) => `Usa al menos ${String(min)} caracteres.`,
		tooLong: (max: number) => `Usa como máximo ${String(max)} caracteres.`,
		tooEasy:
			'Demasiado fácil de adivinar. Prueba con varias palabras sin relación, como nutria-azul-canta.',
		mismatch: 'Las contraseñas no coinciden.',
		create: 'Crear mi sala',
		creating: 'Creando…',
		signIn: 'Entrar',
		signingIn: 'Entrando…',
		taken: 'Alguien acaba de reservar ese nombre.',
		wrongPassword: 'Contraseña incorrecta.',
		failed: 'Algo salió mal. Inténtalo de nuevo.'
	},
	mascots: {
		legend: 'Mascota de la sala',
		names: {
			wizard: 'Mago',
			robot: 'Robot',
			astronaut: 'Astronauta',
			alien: 'Alienígena',
			ninja: 'Ninja',
			dragon: 'Dragón',
			knight: 'Caballero',
			ghost: 'Fantasma',
			cybercat: 'Gato cibernético',
			slime: 'Slime',
			pirate: 'Pirata espacial',
			gamer: 'Gamer'
		}
	},
	openRoom: {
		whoGetsIn: '¿Quién entra?',
		approval: 'Apruebo a cada persona',
		open: 'Cualquiera puede entrar',
		openHint: 'Quien conozca tu @ entra directamente mientras la sala esté abierta.',
		submit: 'Abrir mi sala',
		submitting: 'Abriendo…',
		failed: 'No se pudo abrir tu sala. Inténtalo de nuevo.'
	},
	roomOffline: {
		liveTitle: (h: string) => `@${h} no está en vivo ahora`,
		liveBody: (h: string) =>
			`Esta sala se abre cuando @${h} inicia una llamada. Vuelve pronto — o abre tu propia sala en segundos.`,
		createOwn: 'Crear mi propia sala',
		tryAgain: 'Reintentar',
		freeTitle: (h: string) => `¡@${h} todavía está libre!`,
		freeBody: 'Nadie ha reservado esta sala todavía. Puede ser tuya.',
		claim: (h: string) => `Reservar @${h}`,
		home: 'Volver al inicio'
	},
	lobby: {
		joinRoomTitle: (handle: string) => `Entrar a @${handle}`,
		joinRoomHint:
			'Para unirte a esta llamada, inicia sesión con tu usuario de Diceri o crea uno gratis si eres nuevo. Lleva un minuto y hace que todos te escuchen en su idioma, con tu propia voz.',
		camera: 'Cámara',
		cameraOn: 'Activar cámara',
		cameraOff: 'Desactivar cámara',
		cameraBlocked: 'Permite la cámara en tu navegador para mostrar tu video.',
		cameraUnavailable: 'No se encontró ninguna cámara.',
		joinTitle: 'Unirse a la reunión',
		yourName: 'Tu nombre',
		hearEveryoneIn: 'Escuchar a todos en',
		microphone: 'Micrófono',
		systemDefault: 'Predeterminado del sistema',
		microphonePreview: 'Vista previa del micrófono',
		joinNow: 'Unirse ahora',
		askToJoin: 'Pedir unirse',
		joining: 'Uniéndote…',
		micBlocked:
			'Se bloqueó el acceso al micrófono. Permítelo en la barra de direcciones del navegador.',
		micUnavailable: 'No se pudo acceder al micrófono.',
		waitingToJoin: 'Esperando para unirse',
		deny: 'Rechazar',
		admit: 'Admitir',
		wantsToJoin: (name: string) => `${name} quiere unirse a esta llamada`,
		hears: (language: string) => `(escucha en ${language})`,
		dismiss: 'Descartar',
		cantJoin: 'No puedes unirte a esta reunión',
		declined: 'El anfitrión rechazó tu solicitud.',
		somethingWrong: 'Algo salió mal',
		back: 'Volver',
		asking: 'Pidiendo permiso para entrar…',
		willJoin: 'Te unirás a la reunión cuando alguien te deje entrar.',
		cancel: 'Cancelar'
	},
	voice: {
		requiredHint: (seconds: number) =>
			`Lee una frase corta durante ${String(seconds)} segundos: todos oirán tus traducciones con tu propia voz. Se guarda, cifrada, con tu cuenta.`,
		pro: 'Pro',
		paidSoon: 'Muy pronto como función de pago.',
		section: 'Tu voz',
		recording: 'Grabando · lee en voz alta',
		secondsLeft: (seconds: number) => `${String(seconds)} s`,
		done: 'Listo',
		keepReading: (seconds: number) => `Sigue leyendo (mín. ${String(seconds)} s)`,
		creating: 'Creando tu voz…',
		ready: 'Tu voz está lista',
		redo: 'Repetir',
		readyHint: 'Las traducciones sonarán como tú. La grabación se guarda, cifrada, con tu cuenta.',
		keptReady: 'Tu voz guardada está lista',
		keptHint: 'Usando la grabación guardada con tu cuenta. Bórrala cuando quieras desde tu menú @.',
		title: 'Haz que las traducciones suenen como tú',
		hint: (seconds: number) =>
			`Lee una frase corta durante ${String(seconds)} segundos. Es opcional; se guarda, cifrada, con tu cuenta.`,
		tryAgain: 'Reintentar',
		record: (seconds: number) => `Grabar mi voz (${String(seconds)} s)`,
		noVoice: 'No te oímos con claridad. Lee la frase en voz alta e inténtalo de nuevo.',
		micNeeded: 'Se necesita acceso al micrófono para grabar tu voz.',
		failed: 'No se pudo crear tu voz.'
	},
	call: {
		turnOnCamera: 'Activar cámara',
		turnOffCamera: 'Desactivar cámara',
		turnOnMic: 'Activar micrófono',
		turnOffMic: 'Desactivar micrófono',
		leaveCall: 'Salir de la llamada',
		justLeave: 'Solo salir de la llamada',
		endForEveryone: 'Finalizar la llamada para todos',
		people: 'Personas',
		audioSettings: 'Configuración de audio',
		guest: 'Invitado',
		enableAudio: 'Haz clic para activar el audio de los demás participantes',
		connecting: 'Conectando…',
		inCall: 'En la llamada',
		closePanel: 'Cerrar panel',
		you: (name: string) => `${name} (Tú)`,
		micOff: 'Micrófono desactivado',
		hearsEveryoneIn: 'Escucha a todos en',
		hears: (language: string) => `escucha en ${language}`,
		meetingDetails: 'Detalles de la reunión',
		meetingReady: 'Tu reunión está lista',
		close: 'Cerrar',
		shareLink:
			'Comparte este enlace con la persona con la que quieres hablar. Te pediremos que la dejes entrar.',
		copyLink: 'Copiar enlace de la reunión',
		linkCopied: 'Enlace copiado',
		captions: 'Subtítulos',
		filterUnsupported: 'El filtro de ruido avanzado no es compatible con este navegador.',
		filterFailed: 'Falló el filtro de ruido avanzado.',
		micUnpublished: 'No se pudo publicar el micrófono.'
	},
	audio: {
		microphone: 'Micrófono',
		systemDefault: 'Predeterminado del sistema',
		noiseAndEcho: 'Ruido y eco',
		aiFilter: 'Filtro de ruido con IA (Krisp)',
		filterOff: 'Desactivado',
		filterNoise: 'Ruido',
		filterVoice: 'Solo mi voz',
		filterOffHint: 'Solo el procesamiento del navegador.',
		filterNoiseHint: 'Elimina el ruido de fondo y mantiene todas las voces.',
		filterVoiceHint: 'También elimina a otras personas que hablan cerca (se factura aparte).',
		echo: 'Cancelación de eco',
		echoHint: 'Evita que la voz de la otra persona vuelva por tu micrófono.',
		suppression: 'Supresión de ruido del navegador',
		suppressionOff: 'Desactivada mientras el filtro con IA está activo.',
		suppressionHint: 'Filtro básico integrado.',
		autoGain: 'Ganancia automática',
		autoGainHint: 'Mantiene tu volumen estable.',
		outgoingLevel: 'Nivel de salida (tras el filtrado)',
		muted: 'Silenciado',
		hearMyself: 'Escucharme',
		headphones: 'Usa auriculares o la sala te oirá dos veces.',
		monitorOff: 'Desactivado',
		local: 'Local',
		viaServer: 'Vía servidor',
		monitorOffHint: 'No te escuchas a ti mismo.',
		localHint:
			'Escucha tu micrófono tras el filtro de ruido, al instante. Sirve para juzgar el filtro.',
		viaServerHint:
			'Escúchate tras un viaje de ida y vuelta por el servidor de medios. Sirve para juzgar la transmisión.',
		connection: 'Conexión',
		direction: 'Dirección',
		bitrate: 'Tasa de bits',
		loss: 'Pérdida',
		jitter: 'Jitter',
		rtt: 'RTT',
		sending: 'Enviando',
		receiving: 'Recibiendo'
	},
	meeting: {
		left: 'Saliste de la reunión',
		endedByMe: 'Finalizaste la llamada para todos',
		endedByHost: 'El anfitrión finalizó la llamada',
		rejoin: 'Volver a unirse',
		home: 'Volver a la pantalla de inicio'
	},
	transcript: {
		title: 'Transcripción',
		copy: 'Copiar',
		download: 'Descargar',
		empty: 'No se dijo nada mientras estabas en la llamada.',
		copied: 'Transcripción copiada'
	},
	plans: {
		betaLabel: 'Beta pública',
		betaSpots: (left: number, total: number) =>
			`Quedan ${String(left)} de ${String(total)} lugares`,
		betaFull: 'Por ahora no quedan lugares',
		usedThisMonth: (used: number, total: number) =>
			`${String(used)} de ${String(total)} min usados este mes`,
		freeLine: (minutes: number) => `Gratis: ${String(minutes)} min de llamadas cada mes.`
	},
	limits: {
		beta: 'Diceri está en beta pública y por ahora todos los lugares están ocupados. Vuelve a intentarlo más tarde.',
		network:
			'Esta red ya reservó el máximo de nombres de sala permitido. Inicia sesión con uno de ellos.',
		quota: 'Has usado los minutos de llamada de tu plan este mes. Se renuevan el día 1 (UTC).',
		daily: 'Ya usaste los minutos de llamada gratis de hoy. Se renuevan a medianoche (UTC).',
		monthly: 'La capacidad gratuita de Diceri de este mes se agotó. Se renueva el día 1 (UTC).',
		busy: 'Diceri está al máximo de su capacidad ahora mismo. Vuelve a intentarlo en unos minutos.',
		timeLeft: (time: string) => `Quedan ${time}`,
		timeUp: 'Se acabó tu tiempo de llamada gratis',
		timeUpHint: 'Has usado los minutos gratis de este mes. Se renuevan el día 1 (UTC).'
	},
	terms: {
		title: 'Términos de uso',
		updated: 'Última actualización: 27 de noviembre de 2026',
		notice:
			'Al crear tu sala aceptas los términos de uso, incluido que guardemos la grabación de tu voz, cifrada, para recrear tu voz en las llamadas.',
		read: 'Leer los términos de uso',
		sections: [
			{
				heading: 'Qué es Diceri',
				body: 'Diceri es un servicio de videollamadas con traducción de voz en directo, gestionado como software libre y de código abierto (AGPL-3.0). Cada cuenta tiene una sala en diceri.com/@username. Usar Diceri implica aceptar estos términos.'
			},
			{
				heading: 'Tu cuenta',
				body: 'Te registras con un nombre de usuario y una contraseña; no se pide ningún correo electrónico. No compartas tu contraseña con nadie. Solo se pueden crear unas pocas cuentas por red, y el plan gratuito incluye una cantidad mensual de minutos de llamada.'
			},
			{
				heading: 'Tu voz',
				body: 'Para unirte a una llamada grabas unos 10 segundos de tu voz. La usamos para crear una copia de tu voz con nuestro proveedor de voz, para que la gente oiga tus traducciones con tu propia voz. Guardamos la grabación con tu cuenta, cifrada, para poder recrear tu voz en llamadas posteriores, también con otro proveedor de voz si cambiamos. Solo se usa para decir tus propias traducciones, nunca para hablar en nombre de otra persona, y nunca se vende ni se comparte para otros fines.'
			},
			{
				heading: 'Borrar tu voz',
				body: 'Puedes borrar tu grabación en cualquier momento con "Borrar mi voz" en tu menú @; también se borra con tu cuenta. La copia de tu voz creada en el proveedor de voz se conserva para tus llamadas mientras tu grabación esté guardada, y se borra junto con ella.'
			},
			{
				heading: 'Qué guardamos y qué no',
				body: 'Guardamos tu nombre de usuario, una forma protegida de tu contraseña, la mascota de tu sala, tu plan, cuántos minutos de llamada usaste, una huella cifrada de la red desde la que te registraste (nunca tu dirección IP) y la grabación de tu voz. No grabamos las llamadas ni guardamos lo que se dijo: las transcripciones y los subtítulos se quedan en tu navegador.'
			},
			{
				heading: 'Proveedores de servicios',
				body: 'Las llamadas pasan por LiveKit; ElevenLabs, Inworld, Soniox y Google Gemini transcriben, traducen y locutan lo que se dice. Solo reciben lo necesario para prestar la llamada.'
			},
			{
				heading: 'Uso justo',
				body: 'No uses Diceri para infringir la ley, acosar a personas ni suplantar a nadie. Podemos limitar o cerrar las cuentas que abusen del servicio. Diceri se ofrece tal cual, sin garantías.'
			},
			{
				heading: 'Cambios',
				body: 'Podemos actualizar estos términos; la fecha de arriba indica la última versión. Preguntas: abre un issue en github.com/julianosig/diceri.com.'
			}
		]
	},
	footer: { freeSoftware: 'Software libre', sourceCode: 'Código fuente', terms: 'Términos de uso' }
};
