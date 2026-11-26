import type { Messages } from './en.ts';

export const pt: Messages = {
	meta: { title: 'Diceri — a chamada que você conhece, em todos os idiomas' },
	header: {
		signOut: 'Sair',
		language: 'Idioma',
		toDark: 'Mudar para o tema escuro',
		toLight: 'Mudar para o tema claro'
	},
	hero: {
		headline: 'A chamada que você já conhece.',
		promise: 'Agora em todos os idiomas.',
		illustration:
			'Ilustração: uma pessoa fala, o Diceri traduz no meio e a outra pessoa ouve no próprio idioma.'
	},
	onboarding: {
		label: 'Entre ou garanta o @ da sua sala',
		usernamePlaceholder: 'seunome',
		checking: (handle: string) => `Verificando @${handle}…`,
		available: (handle: string) => `@${handle} está livre — garanta o seu`,
		welcomeBack: (handle: string) => `Que bom te ver de novo, @${handle}`,
		invalid: '3 a 20 letras (a–z), números ou _, começando com uma letra.',
		checkFailed: 'Não foi possível verificar o nome. Tente de novo.',
		password: 'Senha',
		repeatPassword: 'Repita a senha',
		newPasswordPlaceholder: (min: number) => `Pelo menos ${String(min)} caracteres`,
		passwordPlaceholder: 'Sua senha',
		repeatPlaceholder: 'Digite de novo',
		passwordsMatch: 'As senhas coincidem',
		tooShort: (min: number) => `Use pelo menos ${String(min)} caracteres.`,
		tooLong: (max: number) => `Use no máximo ${String(max)} caracteres.`,
		mismatch: 'As senhas não coincidem.',
		create: 'Criar minha sala',
		creating: 'Criando…',
		signIn: 'Entrar',
		signingIn: 'Entrando…',
		taken: 'Esse nome acabou de ser escolhido por outra pessoa.',
		wrongPassword: 'Senha incorreta.',
		failed: 'Algo deu errado. Tente de novo.'
	},
	mascots: {
		legend: 'Mascote da sala',
		names: {
			wizard: 'Mago',
			robot: 'Robô',
			astronaut: 'Astronauta',
			alien: 'Alienígena',
			ninja: 'Ninja',
			dragon: 'Dragão',
			knight: 'Cavaleiro',
			ghost: 'Fantasma',
			cybercat: 'Gato cibernético',
			slime: 'Slime',
			pirate: 'Pirata espacial',
			gamer: 'Gamer'
		}
	},
	openRoom: {
		livesAt: (username: string) => `Sua sala fica em diceri.com/@${username}`,
		listed: 'Mostrar no painel ao vivo',
		listedHint: 'Quem estiver na página inicial vê sua sala e pode entrar.',
		whoGetsIn: 'Quem entra',
		approval: 'Eu aprovo cada pessoa',
		open: 'Qualquer um pode entrar',
		openHint: 'Quem souber o seu @ entra direto enquanto a sala estiver aberta.',
		submit: 'Abrir minha sala',
		submitting: 'Abrindo…',
		failed: 'Não foi possível abrir sua sala. Tente de novo.'
	},
	roomOffline: {
		liveTitle: (h: string) => `@${h} não está ao vivo agora`,
		liveBody: (h: string) =>
			`Esta sala abre quando @${h} começa uma chamada. Volte daqui a pouco — ou abra a sua própria sala em segundos.`,
		createOwn: 'Criar minha própria sala',
		tryAgain: 'Tentar de novo',
		freeTitle: (h: string) => `@${h} ainda está livre!`,
		freeBody: 'Ninguém escolheu esta sala ainda. Ela pode ser sua.',
		claim: (h: string) => `Quero o @${h}`,
		home: 'Voltar para o início'
	},
	footer: { freeSoftware: 'Software livre', sourceCode: 'Código-fonte' }
};
