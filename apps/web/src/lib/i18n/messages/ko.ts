import type { Messages } from './en.ts';

export const ko: Messages = {
	meta: { title: 'Diceri — 양방향 음성 번역이 되는 Google Meet 대안' },
	header: {
		signOut: '로그아웃',
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
		label: '로그인하거나 내 방 사용자 이름 만들기',
		usernamePlaceholder: 'yourname',
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
	footer: { freeSoftware: '자유 소프트웨어', sourceCode: '소스 코드' }
};
