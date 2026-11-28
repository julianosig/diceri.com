import type { Messages } from './en.ts';

export const ko: Messages = {
	meta: { title: 'Diceri — 양방향 음성 번역이 되는 Google Meet 대안' },
	header: {
		signOut: '로그아웃',
		deleteVoice: '내 목소리 삭제',
		deleteVoiceConfirm: '저장된 목소리 녹음을 삭제할까요? 다음 통화에서 새로 녹음하게 돼요.',
		voiceDeleted: '목소리 녹음이 삭제되었어요.',
		language: '언어',
		toDark: '다크 테마로 전환',
		toLight: '라이트 테마로 전환'
	},
	hero: {
		headline: 'Google Meet의 대안,',
		promise: '내 언어로 말하고, 내 언어로 듣는 곳.',
		voice: '회의의 모두가 당신의 목소리를 자신의 언어로 듣습니다.',
		openSource: '우리는 오픈 소스입니다',
		illustration:
			'일러스트: 한 사람이 말하면 가운데에서 Diceri가 번역하고, 다른 사람은 자신의 언어로 듣습니다.'
	},
	onboarding: {
		label: '방 사용자 이름',
		mascotTitle: '방 마스코트를 골라 주세요',
		mascotHint: '방문하는 모든 사람을 맞이해요',
		continue: '계속',
		back: '뒤로',
		usernamePlaceholder: 'room_username',
		checking: (handle: string) => `@${handle} 확인 중…`,
		available: (handle: string) => `@${handle} 사용 가능 — 지금 가져가세요`,
		welcomeBack: (handle: string) => `다시 오신 걸 환영해요, @${handle}`,
		invalid: '영문 소문자(a–z), 숫자, _ 로 3–20자, 영문자로 시작해야 합니다.',
		checkFailed: '이름을 확인할 수 없습니다. 다시 시도하세요.',
		password: '비밀번호',
		repeatPassword: '비밀번호 확인',
		newPasswordPlaceholder: (min: number) => `${String(min)}자 이상`,
		passwordPlaceholder: '비밀번호',
		repeatPlaceholder: '한 번 더 입력',
		passwordsMatch: '비밀번호가 일치합니다',
		tooShort: (min: number) => `${String(min)}자 이상 입력하세요.`,
		tooLong: (max: number) => `${String(max)}자 이하로 입력하세요.`,
		tooEasy:
			'너무 쉽게 추측할 수 있어요. blue-otter-sings처럼 서로 관련 없는 단어 몇 개를 써 보세요.',
		mismatch: '비밀번호가 일치하지 않습니다.',
		create: '내 방 만들기',
		creating: '만드는 중…',
		signIn: '로그인',
		signingIn: '로그인 중…',
		taken: '방금 다른 사람이 그 이름을 가져갔어요.',
		wrongPassword: '비밀번호가 틀렸습니다.',
		failed: '문제가 발생했습니다. 다시 시도하세요.'
	},
	mascots: {
		legend: '방 마스코트',
		names: {
			wizard: '마법사',
			robot: '로봇',
			astronaut: '우주비행사',
			alien: '외계인',
			ninja: '닌자',
			dragon: '드래곤',
			knight: '기사',
			ghost: '유령',
			cybercat: '사이버 고양이',
			slime: '슬라임',
			pirate: '우주 해적',
			gamer: '게이머'
		}
	},
	openRoom: {
		whoGetsIn: '입장 방식',
		approval: '한 명씩 승인',
		open: '누구나 참여 가능',
		openHint: '방이 열려 있는 동안 내 @를 아는 사람은 바로 들어옵니다.',
		submit: '내 방 열기',
		submitting: '여는 중…',
		failed: '방을 열 수 없습니다. 다시 시도하세요.'
	},
	roomOffline: {
		liveTitle: (h: string) => `@${h}님은 지금 라이브가 아니에요`,
		liveBody: (h: string) =>
			`@${h}님이 통화를 시작하면 이 방이 열려요. 잠시 후 다시 오거나, 몇 초 만에 내 방을 만들어 보세요.`,
		createOwn: '내 방 만들기',
		tryAgain: '다시 시도',
		freeTitle: (h: string) => `@${h}은(는) 아직 비어 있어요!`,
		freeBody: '아직 아무도 이 방을 가져가지 않았어요. 당신의 방이 될 수 있어요.',
		claim: (h: string) => `@${h} 가져가기`,
		home: '홈으로 돌아가기'
	},
	lobby: {
		joinRoomTitle: (handle: string) => `@${handle} 참여하기`,
		joinRoomHint:
			'이 통화에 참여하려면 Diceri 사용자 이름으로 로그인하거나, 처음이라면 무료로 만들어 주세요. 1분이면 충분하고, 모두가 각자의 언어로 내 목소리 그대로 듣게 돼요.',
		camera: '카메라',
		cameraOn: '카메라 켜기',
		cameraOff: '카메라 끄기',
		cameraBlocked: '영상을 보여 주려면 브라우저에서 카메라를 허용하세요.',
		cameraUnavailable: '카메라를 찾을 수 없어요.',
		joinTitle: '회의 참여하기',
		yourName: '이름',
		hearEveryoneIn: '모두의 말을 들을 언어',
		microphone: '마이크',
		systemDefault: '시스템 기본값',
		microphonePreview: '마이크 미리 듣기',
		joinNow: '지금 참여',
		askToJoin: '참여 요청',
		joining: '참여하는 중…',
		micBlocked: '마이크 접근이 차단되었습니다. 브라우저 주소 표시줄에서 허용하세요.',
		micUnavailable: '마이크에 접근할 수 없습니다.',
		waitingToJoin: '참여 대기 중',
		deny: '거부',
		admit: '수락',
		wantsToJoin: (name: string) => `${name} 님이 이 통화에 참여하려고 합니다`,
		hears: (language: string) => `(${language}(으)로 들음)`,
		dismiss: '닫기',
		cantJoin: '이 회의에 참여할 수 없습니다',
		declined: '호스트가 요청을 거부했습니다.',
		somethingWrong: '문제가 발생했습니다',
		back: '뒤로',
		asking: '입장을 요청하는 중…',
		willJoin: '누군가 수락하면 회의에 참여하게 됩니다.',
		cancel: '취소'
	},
	voice: {
		requiredHint: (seconds: number) =>
			`짧은 문장을 ${String(seconds)}초 동안 읽어 주세요. 모두가 내 목소리로 번역을 듣게 돼요. 녹음은 암호화되어 내 계정에 보관돼요.`,
		pro: 'Pro',
		paidSoon: '곧 유료 기능으로 제공됩니다.',
		section: '내 목소리',
		recording: '녹음 중 · 소리 내어 읽으세요',
		secondsLeft: (seconds: number) => `${String(seconds)}초`,
		done: '완료',
		keepReading: (seconds: number) => `계속 읽으세요 (최소 ${String(seconds)}초)`,
		creating: '목소리를 만드는 중…',
		ready: '목소리가 준비되었습니다',
		redo: '다시 하기',
		readyHint: '번역이 내 목소리로 들립니다. 녹음은 암호화되어 내 계정에 보관됩니다.',
		keptReady: '저장된 목소리가 준비되었습니다',
		keptHint: '내 계정에 보관된 녹음을 사용합니다. @ 메뉴에서 언제든지 삭제할 수 있습니다.',
		title: '번역을 내 목소리로 들려주기',
		hint: (seconds: number) =>
			`짧은 문장을 ${String(seconds)}초 동안 읽으세요. 선택 사항이며 녹음은 암호화되어 내 계정에 보관됩니다.`,
		tryAgain: '다시 시도',
		record: (seconds: number) => `내 목소리 녹음 (${String(seconds)}초)`,
		noVoice: '목소리가 잘 들리지 않았습니다. 문장을 소리 내어 읽고 다시 시도하세요.',
		micNeeded: '목소리를 녹음하려면 마이크 접근 권한이 필요합니다.',
		failed: '목소리를 만들 수 없습니다.'
	},
	call: {
		turnOnCamera: '카메라 켜기',
		turnOffCamera: '카메라 끄기',
		turnOnMic: '마이크 켜기',
		turnOffMic: '마이크 끄기',
		leaveCall: '통화에서 나가기',
		justLeave: '통화에서 나가기만 하기',
		endForEveryone: '모두의 통화 종료',
		people: '참여자',
		audioSettings: '오디오 설정',
		guest: '게스트',
		enableAudio: '클릭하여 다른 참여자의 오디오를 켜세요',
		connecting: '연결하는 중…',
		inCall: '통화 중',
		closePanel: '패널 닫기',
		you: (name: string) => `${name}(나)`,
		micOff: '마이크 꺼짐',
		hearsEveryoneIn: '모두의 말을 듣는 언어',
		hears: (language: string) => `${language}(으)로 들음`,
		meetingDetails: '회의 세부정보',
		meetingReady: '회의가 준비되었습니다',
		close: '닫기',
		shareLink: '대화하려는 상대에게 이 링크를 공유하세요. 상대를 들여보낼지 묻게 됩니다.',
		copyLink: '회의 링크 복사',
		linkCopied: '링크가 복사되었습니다',
		captions: '자막',
		filterUnsupported: '이 브라우저는 고급 소음 필터를 지원하지 않습니다.',
		filterFailed: '고급 소음 필터에 문제가 발생했습니다.',
		micUnpublished: '마이크를 송출할 수 없습니다.'
	},
	audio: {
		microphone: '마이크',
		systemDefault: '시스템 기본값',
		noiseAndEcho: '소음 및 에코',
		aiFilter: 'AI 소음 필터 (Krisp)',
		filterOff: '끄기',
		filterNoise: '소음',
		filterVoice: '내 목소리만',
		filterOffHint: '브라우저 처리만 사용합니다.',
		filterNoiseHint: '배경 소음을 없애고 모든 목소리는 유지합니다.',
		filterVoiceHint: '근처에서 말하는 다른 사람의 목소리도 없앱니다(별도 요금).',
		echo: '에코 제거',
		echoHint: '상대방의 목소리가 내 마이크로 되돌아가지 않게 합니다.',
		suppression: '브라우저 소음 억제',
		suppressionOff: 'AI 필터가 켜져 있는 동안 꺼집니다.',
		suppressionHint: '기본 내장 필터입니다.',
		autoGain: '자동 게인',
		autoGainHint: '음량을 일정하게 유지합니다.',
		outgoingLevel: '송출 레벨(필터 적용 후)',
		muted: '음소거됨',
		hearMyself: '내 목소리 듣기',
		headphones: '헤드폰을 사용하세요. 그렇지 않으면 상대방에게 내 목소리가 두 번 들립니다.',
		monitorOff: '끄기',
		local: '로컬',
		viaServer: '서버 경유',
		monitorOffHint: '내 목소리가 들리지 않습니다.',
		localHint: '소음 필터를 거친 마이크 소리를 바로 듣습니다. 필터 품질을 확인할 수 있습니다.',
		viaServerHint: '미디어 서버를 왕복한 내 목소리를 듣습니다. 전송 품질을 확인할 수 있습니다.',
		connection: '연결',
		direction: '방향',
		bitrate: '비트레이트',
		loss: '손실',
		jitter: '지터',
		rtt: 'RTT',
		sending: '송신',
		receiving: '수신'
	},
	meeting: {
		left: '회의에서 나왔습니다',
		endedByMe: '모두의 통화를 종료했습니다',
		endedByHost: '호스트가 통화를 종료했습니다',
		rejoin: '다시 참여',
		home: '홈 화면으로 돌아가기'
	},
	transcript: {
		title: '스크립트',
		copy: '복사',
		download: '다운로드',
		empty: '통화하는 동안 아무 말도 없었습니다.',
		copied: '스크립트가 복사되었습니다'
	},
	plans: {
		betaCaption: (total: number) => `공개 베타 남은 자리 (총 ${String(total)}자리)`,
		betaFull: '지금은 모든 자리가 찼어요',
		usedThisMonth: (used: number, total: number) =>
			`이번 달 ${String(total)}분 중 ${String(used)}분 사용`,
		freeLine: (minutes: number) => `무료: 매월 ${String(minutes)}분 통화.`
	},
	limits: {
		beta: 'Diceri는 공개 베타 중이며 지금은 모든 자리가 찼어요. 나중에 다시 시도해 주세요.',
		network:
			'이 네트워크에서 만들 수 있는 방 이름의 최대 개수에 도달했어요. 기존 방 이름으로 로그인하세요.',
		quota: '이번 달 요금제의 통화 시간을 모두 사용했어요. 매월 1일(UTC)에 초기화돼요.',
		daily: '오늘의 무료 통화 시간을 모두 사용했습니다. 자정(UTC)에 다시 충전됩니다.',
		monthly: '이번 달 Diceri 무료 용량이 모두 소진됐어요. 매월 1일(UTC)에 초기화돼요.',
		busy: '지금은 Diceri 사용량이 많습니다. 몇 분 후에 다시 시도해 주세요.',
		timeLeft: (time: string) => `${time} 남음`,
		timeUp: '무료 통화 시간이 끝났습니다',
		timeUpHint: '이번 달 무료 통화 시간을 모두 사용했어요. 매월 1일(UTC)에 초기화돼요.'
	},
	terms: {
		title: '이용약관',
		updated: '최종 업데이트: 2026년 11월 27일',
		notice:
			'방을 만들면 통화에서 내 목소리를 재현하기 위해 목소리 녹음을 암호화하여 보관하는 것을 포함한 이용약관에 동의하게 됩니다.',
		read: '이용약관 읽기',
		sections: [
			{
				heading: 'Diceri란',
				body: 'Diceri는 실시간 음성 번역 기능이 있는 화상 통화 서비스이며, 자유 오픈 소스 소프트웨어(AGPL-3.0)로 운영됩니다. 각 계정은 diceri.com/@username에 방 하나를 가집니다. Diceri를 이용하면 이 약관에 동의하는 것입니다.'
			},
			{
				heading: '내 계정',
				body: '사용자 이름과 비밀번호로 가입하며, 이메일은 요구하지 않습니다. 비밀번호는 다른 사람과 공유하지 마세요. 네트워크당 만들 수 있는 계정은 몇 개로 제한되며, 무료 요금제에는 매월 일정량의 통화 시간(분)이 포함됩니다.'
			},
			{
				heading: '내 목소리',
				body: '통화에 참여하려면 약 10초 동안 내 목소리를 녹음합니다. 이 녹음으로 음성 제공업체에서 내 목소리의 사본을 만들어, 다른 사람들이 내 번역을 내 목소리로 들을 수 있게 합니다. 녹음은 암호화하여 내 계정과 함께 보관하며, 이후 통화에서 내 목소리를 다시 만들 수 있도록 합니다. 음성 제공업체를 바꾸는 경우 다른 제공업체에서도 다시 만들 수 있습니다. 녹음은 내 번역을 말하는 데에만 사용되며, 다른 사람을 대신해 말하는 데에는 절대 사용되지 않고, 판매되거나 다른 목적으로 공유되지 않습니다.'
			},
			{
				heading: '내 목소리 삭제',
				body: '@ 메뉴의 "내 목소리 삭제"로 언제든지 녹음을 삭제할 수 있으며, 계정을 삭제해도 함께 삭제돼요. 음성 제공업체에서 만든 내 목소리의 복사본은 녹음이 저장되어 있는 동안 통화에 쓰이도록 보관되고, 녹음과 함께 삭제돼요.'
			},
			{
				heading: '보관하는 것과 보관하지 않는 것',
				body: '사용자 이름, 보호된 형태의 비밀번호, 방 마스코트, 요금제, 사용한 통화 시간(분), 가입한 네트워크의 암호화된 지문(IP 주소는 절대 보관하지 않음), 그리고 목소리 녹음을 보관합니다. 통화를 녹화하거나 녹음하지 않으며, 말한 내용도 보관하지 않습니다. 대화 기록과 자막은 내 브라우저에만 남습니다.'
			},
			{
				heading: '서비스 제공업체',
				body: '통화는 LiveKit을 통해 이루어지며, 음성의 받아쓰기, 번역, 음성 합성은 ElevenLabs, Inworld, Soniox, Google Gemini가 처리합니다. 이들은 통화 제공에 필요한 정보만 받습니다.'
			},
			{
				heading: '공정한 이용',
				body: '법을 어기거나, 다른 사람을 괴롭히거나, 다른 사람을 사칭하는 데 Diceri를 사용하지 마세요. 서비스를 악용하는 계정은 제한하거나 해지할 수 있습니다. Diceri는 있는 그대로 제공되며, 어떠한 보증도 하지 않습니다.'
			},
			{
				heading: '변경',
				body: '이 약관은 업데이트될 수 있으며, 위의 날짜가 최신 버전을 나타냅니다. 문의: github.com/julianosig/diceri.com에서 이슈를 열어 주세요.'
			}
		]
	},
	footer: { freeSoftware: '자유 소프트웨어', sourceCode: '소스 코드', terms: '이용약관' }
};
