import type { Messages } from './en.ts';

export const ja: Messages = {
	meta: { title: 'Diceri — 双方向の音声翻訳ができる Google Meet の代わり' },
	header: {
		signOut: 'ログアウト',
		language: '言語',
		toDark: 'ダークテーマに切り替え',
		toLight: 'ライトテーマに切り替え'
	},
	hero: {
		headline: 'Google Meet の代わりに。',
		promise: '自分の言語で話して、自分の言語で聞ける。',
		voice: '相手には、相手の言語で、あなた自身の声で届きます。',
		openSource: 'オープンソースです',
		illustration: 'イラスト：一人が話し、真ん中で Diceri が翻訳し、もう一人が自分の言語で聞きます。'
	},
	onboarding: {
		label: 'ログイン、またはルームのユーザー名を取得',
		usernamePlaceholder: 'yourname',
		checking: (handle: string) => `@${handle} を確認中…`,
		available: (handle: string) => `@${handle} は使えます — 取得しましょう`,
		welcomeBack: (handle: string) => `おかえりなさい、@${handle}`,
		invalid: '英小文字（a–z）、数字、_ で 3〜20 文字。英字で始めてください。',
		checkFailed: '名前を確認できませんでした。もう一度お試しください。',
		password: 'パスワード',
		repeatPassword: 'パスワード（確認）',
		newPasswordPlaceholder: (min: number) => `${String(min)} 文字以上`,
		passwordPlaceholder: 'パスワード',
		repeatPlaceholder: 'もう一度入力',
		passwordsMatch: 'パスワードが一致しました',
		tooShort: (min: number) => `${String(min)} 文字以上にしてください。`,
		tooLong: (max: number) => `${String(max)} 文字以内にしてください。`,
		mismatch: 'パスワードが一致しません。',
		create: 'ルームを作成',
		creating: '作成中…',
		signIn: 'ログイン',
		signingIn: 'ログイン中…',
		taken: 'その名前はたった今使われました。',
		wrongPassword: 'パスワードが違います。',
		failed: '問題が発生しました。もう一度お試しください。'
	},
	mascots: {
		legend: 'ルームのマスコット',
		names: {
			wizard: '魔法使い',
			robot: 'ロボット',
			astronaut: '宇宙飛行士',
			alien: 'エイリアン',
			ninja: '忍者',
			dragon: 'ドラゴン',
			knight: '騎士',
			ghost: 'おばけ',
			cybercat: 'サイバー猫',
			slime: 'スライム',
			pirate: '宇宙海賊',
			gamer: 'ゲーマー'
		}
	},
	openRoom: {
		whoGetsIn: '入室できる人',
		approval: '一人ずつ承認する',
		open: '誰でも参加できる',
		openHint: 'ルームが開いている間、あなたの @ を知っている人はすぐに入れます。',
		submit: 'ルームを開く',
		submitting: '開いています…',
		failed: 'ルームを開けませんでした。もう一度お試しください。'
	},
	roomOffline: {
		liveTitle: (h: string) => `@${h} は今ライブではありません`,
		liveBody: (h: string) =>
			`このルームは @${h} が通話を始めると開きます。またあとで来るか、数秒で自分のルームを作りましょう。`,
		createOwn: '自分のルームを作る',
		tryAgain: 'もう一度試す',
		freeTitle: (h: string) => `@${h} はまだ空いています！`,
		freeBody: 'このルームはまだ誰のものでもありません。あなたのものにできます。',
		claim: (h: string) => `@${h} を取得`,
		home: 'ホームに戻る'
	},
	footer: { freeSoftware: 'フリーソフトウェア', sourceCode: 'ソースコード' }
};
