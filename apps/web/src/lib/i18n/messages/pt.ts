import type { Messages } from './en.ts';

export const pt: Messages = {
	meta: { title: 'Diceri — alternativa ao Google Meet com tradução de voz nos dois sentidos' },
	header: {
		signOut: 'Sair',
		deleteVoice: 'Apagar minha voz',
		deleteVoiceConfirm:
			'Apagar a gravação salva da sua voz? Você vai gravar uma nova na sua próxima chamada.',
		voiceDeleted: 'A gravação da sua voz foi apagada.',
		voiceDeleteFailed: 'Não foi possível apagar sua voz agora. Nada foi apagado; tente de novo.',
		language: 'Idioma',
		toDark: 'Mudar para o tema escuro',
		toLight: 'Mudar para o tema claro'
	},
	hero: {
		headline: 'A alternativa ao Google Meet',
		promise: 'onde você fala e ouve no seu próprio idioma.',
		voice: 'Todos na reunião ouvem a sua voz no idioma deles.',
		openSource: 'Somos open source',
		illustration:
			'Ilustração: uma pessoa fala, o Diceri traduz no meio e a outra pessoa ouve no próprio idioma.'
	},
	onboarding: {
		label: 'Usuário da sala',
		mascotTitle: 'Escolha o mascote da sua sala',
		mascotHint: 'Ele recebe todo mundo que visita',
		continue: 'Continuar',
		back: 'Voltar',
		usernamePlaceholder: 'usuario_da_sala',
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
		tooEasy:
			'Fácil demais de adivinhar. Tente algumas palavras sem relação, como lontra-azul-canta.',
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
	lobby: {
		joinRoomTitle: (handle: string) => `Entrar na sala @${handle}`,
		joinRoomHint:
			'Para entrar nesta chamada, entre com seu nome de usuário do Diceri ou crie um grátis se for novo por aqui. Leva um minuto e faz todo mundo ouvir você no idioma deles, com a sua própria voz.',
		camera: 'Câmera',
		cameraOn: 'Ligar câmera',
		cameraOff: 'Desligar câmera',
		cameraBlocked: 'Permita a câmera no navegador para mostrar seu vídeo.',
		cameraUnavailable: 'Nenhuma câmera encontrada.',
		joinTitle: 'Entrar na reunião',
		yourName: 'Seu nome',
		hearEveryoneIn: 'Ouvir todos em',
		microphone: 'Microfone',
		systemDefault: 'Padrão do sistema',
		microphonePreview: 'Prévia do microfone',
		joinNow: 'Participar agora',
		askToJoin: 'Pedir para participar',
		joining: 'Entrando…',
		micBlocked: 'O acesso ao microfone foi bloqueado. Libere-o na barra de endereço do navegador.',
		micUnavailable: 'Não foi possível acessar o microfone.',
		waitingToJoin: 'Aguardando para entrar',
		deny: 'Recusar',
		admit: 'Aceitar',
		wantsToJoin: (name: string) => `${name} quer participar desta chamada`,
		hears: (language: string) => `(ouve em ${language})`,
		dismiss: 'Dispensar',
		cantJoin: 'Não é possível participar desta reunião',
		declined: 'Quem criou a reunião recusou seu pedido.',
		somethingWrong: 'Algo deu errado',
		back: 'Voltar',
		asking: 'Pedindo para entrar…',
		willJoin: 'Você vai entrar na reunião quando alguém permitir.',
		cancel: 'Cancelar'
	},
	voice: {
		requiredHint: (seconds: number) =>
			`Leia uma frase curta por ${String(seconds)} segundos: todos ouvem suas traduções com a sua própria voz. Guardada, criptografada, com a sua conta.`,
		pro: 'Pro',
		paidSoon: 'Em breve como recurso pago.',
		section: 'Sua voz',
		recording: 'Gravando · leia em voz alta',
		secondsLeft: (seconds: number) => `${String(seconds)} s`,
		done: 'Concluir',
		keepReading: (seconds: number) => `Continue lendo (mín. ${String(seconds)} s)`,
		creating: 'Criando sua voz…',
		ready: 'Sua voz está pronta',
		redo: 'Refazer',
		readyHint:
			'As traduções vão soar como você. A gravação fica guardada, criptografada, com a sua conta.',
		keptReady: 'Sua voz salva está pronta',
		keptHint: 'Usando a gravação guardada com a sua conta. Apague quando quiser no seu menu @.',
		title: 'Faça as traduções soarem como você',
		hint: (seconds: number) =>
			`Leia uma frase curta por ${String(seconds)} segundos. É opcional; fica guardada, criptografada, com a sua conta.`,
		tryAgain: 'Tentar de novo',
		record: (seconds: number) => `Gravar minha voz (${String(seconds)} s)`,
		noVoice: 'Não conseguimos ouvir com clareza. Leia a frase em voz alta e tente de novo.',
		micNeeded: 'É preciso acesso ao microfone para gravar sua voz.',
		failed: 'Não foi possível criar sua voz.'
	},
	call: {
		turnOnCamera: 'Ligar câmera',
		turnOffCamera: 'Desligar câmera',
		turnOnMic: 'Ativar microfone',
		turnOffMic: 'Desativar microfone',
		leaveCall: 'Sair da chamada',
		justLeave: 'Só sair da chamada',
		endForEveryone: 'Encerrar a chamada para todos',
		people: 'Pessoas',
		audioSettings: 'Configurações de áudio',
		guest: 'Convidado',
		enableAudio: 'Clique para ativar o áudio dos outros participantes',
		connecting: 'Conectando…',
		inCall: 'Na chamada',
		closePanel: 'Fechar painel',
		you: (name: string) => `${name} (Você)`,
		micOff: 'Microfone desativado',
		hearsEveryoneIn: 'Ouve todos em',
		hears: (language: string) => `ouve em ${language}`,
		meetingDetails: 'Detalhes da reunião',
		meetingReady: 'Sua reunião está pronta',
		close: 'Fechar',
		shareLink:
			'Compartilhe este link com a pessoa com quem você quer falar. Vamos pedir sua permissão para ela entrar.',
		copyLink: 'Copiar link da reunião',
		linkCopied: 'Link copiado',
		captions: 'Legendas',
		filterUnsupported: 'O filtro de ruído avançado não é compatível com este navegador.',
		filterFailed: 'O filtro de ruído avançado falhou.',
		micUnpublished: 'Não foi possível transmitir o microfone.'
	},
	audio: {
		microphone: 'Microfone',
		systemDefault: 'Padrão do sistema',
		noiseAndEcho: 'Ruído e eco',
		aiFilter: 'Filtro de ruído com IA (Krisp)',
		filterOff: 'Desligado',
		filterNoise: 'Ruído',
		filterVoice: 'Só a minha voz',
		filterOffHint: 'Só o processamento do navegador.',
		filterNoiseHint: 'Remove o ruído de fundo e mantém todas as vozes.',
		filterVoiceHint: 'Também remove outras pessoas falando por perto (cobrado à parte).',
		echo: 'Cancelamento de eco',
		echoHint: 'Impede que a voz da outra pessoa volte pelo seu microfone.',
		suppression: 'Supressão de ruído do navegador',
		suppressionOff: 'Desligada enquanto o filtro com IA estiver ativo.',
		suppressionHint: 'Filtro básico integrado.',
		autoGain: 'Ganho automático',
		autoGainHint: 'Mantém seu volume estável.',
		outgoingLevel: 'Nível de saída (após o filtro)',
		muted: 'Sem som',
		hearMyself: 'Ouvir minha voz',
		headphones: 'Use fones de ouvido, ou a sala vai te ouvir duas vezes.',
		monitorOff: 'Desligado',
		local: 'Local',
		viaServer: 'Pelo servidor',
		monitorOffHint: 'Você não ouve a própria voz.',
		localHint: 'Ouça seu microfone após o filtro de ruído, na hora. Serve para avaliar o filtro.',
		viaServerHint:
			'Ouça sua voz após uma ida e volta pelo servidor de mídia. Serve para avaliar a transmissão.',
		connection: 'Conexão',
		direction: 'Direção',
		bitrate: 'Taxa de bits',
		loss: 'Perda',
		jitter: 'Jitter',
		rtt: 'RTT',
		sending: 'Enviando',
		receiving: 'Recebendo'
	},
	meeting: {
		left: 'Você saiu da reunião',
		disconnected: 'Você foi desconectado',
		endedByMe: 'Você encerrou a chamada para todos',
		endedByHost: 'A chamada foi encerrada por quem a criou',
		rejoin: 'Participar de novo',
		home: 'Voltar para a tela inicial'
	},
	transcript: {
		title: 'Transcrição',
		copy: 'Copiar',
		download: 'Baixar',
		empty: 'Nada foi dito enquanto você estava na chamada.',
		copied: 'Transcrição copiada'
	},
	plans: {
		betaLeft: 'vagas restantes',
		betaDetail: (total: number) => `Beta público · ${String(total)} no total`,
		betaFull: 'Todas as vagas foram preenchidas por enquanto',
		usedThisMonth: (used: number, total: number) =>
			`${String(used)} de ${String(total)} min usados este mês`,
		freeLine: (minutes: number) => `Grátis: ${String(minutes)} min de chamadas todo mês.`
	},
	limits: {
		beta: 'O Diceri está em beta público e todas as vagas foram preenchidas por enquanto. Tente de novo mais tarde.',
		network: 'Esta rede já garantiu o máximo de nomes de sala permitido. Entre com um deles.',
		quota: 'Você usou os minutos de chamada do seu plano neste mês. Eles renovam no dia 1º (UTC).',
		daily: 'Você já usou os minutos grátis de chamada de hoje. Eles renovam à meia-noite (UTC).',
		monthly: 'A capacidade gratuita do Diceri neste mês acabou. Ela renova no dia 1º (UTC).',
		busy: 'O Diceri está no limite da capacidade agora. Tente de novo em alguns minutos.',
		timeLeft: (time: string) => `Faltam ${time}`,
		timeUp: 'Seu tempo de chamada grátis acabou',
		timeUpHint: 'Você usou os minutos grátis deste mês. Eles renovam no dia 1º (UTC).'
	},
	terms: {
		title: 'Termos de uso',
		updated: 'Última atualização: 27 de novembro de 2026',
		notice:
			'Ao criar sua sala, você aceita os termos de uso, incluindo guardar a gravação da sua voz, criptografada, para recriar sua voz nas chamadas.',
		read: 'Ler os termos de uso',
		sections: [
			{
				heading: 'O que é o Diceri',
				body: 'O Diceri é um serviço de videochamadas com tradução de voz ao vivo, mantido como software livre e de código aberto (AGPL-3.0). Cada conta tem uma sala em diceri.com/@username. Usar o Diceri significa aceitar estes termos.'
			},
			{
				heading: 'Sua conta',
				body: 'Você se cadastra com um nome de usuário e uma senha; nenhum e-mail é pedido. Não compartilhe sua senha com ninguém. Só é possível criar algumas contas por rede, e o plano gratuito inclui uma quantidade mensal de minutos de chamada.'
			},
			{
				heading: 'Sua voz',
				body: 'Para entrar em uma chamada, você grava cerca de 10 segundos da sua voz. Nós a usamos para criar uma cópia da sua voz no nosso provedor de voz, para que as pessoas ouçam suas traduções com a sua própria voz. Guardamos a gravação com a sua conta, criptografada, para podermos recriar sua voz em chamadas futuras, inclusive com outro provedor de voz se mudarmos. Ela é usada apenas para falar as suas próprias traduções, nunca para falar por outra pessoa, e nunca é vendida nem compartilhada para outros fins.'
			},
			{
				heading: 'Apagar sua voz',
				body: 'Você pode apagar sua gravação a qualquer momento em "Apagar minha voz", no menu do seu @; ela também é apagada junto com a sua conta. A cópia da sua voz feita no provedor de voz fica guardada para as suas chamadas enquanto a gravação existir, e é apagada junto com ela.'
			},
			{
				heading: 'O que guardamos e o que não guardamos',
				body: 'Guardamos seu nome de usuário, uma forma protegida da sua senha, o mascote da sua sala, seu plano, quantos minutos de chamada você usou, uma impressão digital criptografada da rede de onde você se cadastrou (nunca o seu endereço IP) e a gravação da sua voz. Não gravamos chamadas e não guardamos o que foi dito: transcrições e legendas ficam no seu navegador.'
			},
			{
				heading: 'Provedores de serviço',
				body: 'As chamadas passam pelo LiveKit; a fala é transcrita, traduzida e falada pela ElevenLabs, Inworld, Soniox e Google Gemini. Eles recebem apenas o necessário para realizar a chamada.'
			},
			{
				heading: 'Uso justo',
				body: 'Não use o Diceri para violar a lei, assediar pessoas ou se passar por alguém. Podemos limitar ou encerrar contas que abusem do serviço. O Diceri é fornecido como está, sem garantias.'
			},
			{
				heading: 'Alterações',
				body: 'Podemos atualizar estes termos; a data acima indica a versão mais recente. Dúvidas: abra uma issue em github.com/julianosig/diceri.com.'
			}
		]
	},
	footer: { freeSoftware: 'Software livre', sourceCode: 'Código-fonte', terms: 'Termos de uso' }
};
