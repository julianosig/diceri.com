import type { Messages } from './en.ts';

export const es: Messages = {
	meta: { title: 'Diceri — alternativa a Google Meet con traducción de voz bidireccional' },
	header: {
		signOut: 'Cerrar sesión',
		language: 'Idioma',
		toDark: 'Cambiar al tema oscuro',
		toLight: 'Cambiar al tema claro'
	},
	hero: {
		headline: 'La alternativa a Google Meet',
		promise: 'donde hablas y escuchas en tu propio idioma.',
		voice: 'Te escuchan en su idioma, con tu propia voz.',
		openSource: 'Código abierto en GitHub',
		illustration:
			'Ilustración: una persona habla, Diceri traduce en el medio y la otra persona lo oye en su propio idioma.'
	},
	onboarding: {
		label: 'Inicia sesión o reserva el nombre de tu sala',
		usernamePlaceholder: 'tunombre',
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
	footer: { freeSoftware: 'Software libre', sourceCode: 'Código fuente' }
};
