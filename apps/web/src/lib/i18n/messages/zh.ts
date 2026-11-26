import type { Messages } from './en.ts';

/** Simplified Chinese. */
export const zh: Messages = {
	meta: { title: 'Diceri — 支持双向语音翻译的 Google Meet 替代方案' },
	header: {
		signOut: '退出登录',
		language: '语言',
		toDark: '切换到深色主题',
		toLight: '切换到浅色主题'
	},
	hero: {
		headline: 'Google Meet 的替代方案',
		promise: '用自己的语言说，也用自己的语言听。',
		voice: '对方用他们的语言听到你——用你自己的声音。',
		illustration: '插图：一个人说话，Diceri 在中间翻译，另一个人用自己的语言听到。'
	},
	onboarding: {
		label: '登录或领取你的房间用户名',
		usernamePlaceholder: 'yourname',
		checking: (handle: string) => `正在检查 @${handle}…`,
		available: (handle: string) => `@${handle} 可用 — 立即领取`,
		welcomeBack: (handle: string) => `欢迎回来，@${handle}`,
		invalid: '3–20 个小写字母（a–z）、数字或 _，以字母开头。',
		checkFailed: '无法检查该名称，请重试。',
		password: '密码',
		repeatPassword: '确认密码',
		newPasswordPlaceholder: (min: number) => `至少 ${String(min)} 个字符`,
		passwordPlaceholder: '你的密码',
		repeatPlaceholder: '再输入一次',
		passwordsMatch: '两次密码一致',
		tooShort: (min: number) => `请至少使用 ${String(min)} 个字符。`,
		tooLong: (max: number) => `最多使用 ${String(max)} 个字符。`,
		mismatch: '两次输入的密码不一致。',
		create: '创建我的房间',
		creating: '正在创建…',
		signIn: '登录',
		signingIn: '正在登录…',
		taken: '这个名称刚刚被占用了。',
		wrongPassword: '密码错误。',
		failed: '出了点问题，请重试。'
	},
	mascots: {
		legend: '房间吉祥物',
		names: {
			wizard: '巫师',
			robot: '机器人',
			astronaut: '宇航员',
			alien: '外星人',
			ninja: '忍者',
			dragon: '龙',
			knight: '骑士',
			ghost: '幽灵',
			cybercat: '赛博猫',
			slime: '史莱姆',
			pirate: '太空海盗',
			gamer: '玩家'
		}
	},
	openRoom: {
		whoGetsIn: '谁可以进入',
		approval: '我逐个批准',
		open: '任何人都可加入',
		openHint: '房间开放期间，知道你 @ 的人可以直接进入。',
		submit: '打开我的房间',
		submitting: '正在打开…',
		failed: '无法打开你的房间，请重试。'
	},
	roomOffline: {
		liveTitle: (h: string) => `@${h} 现在不在线`,
		liveBody: (h: string) =>
			`@${h} 发起通话时，这个房间就会打开。稍后再来，或者几秒钟内创建你自己的房间。`,
		createOwn: '创建我自己的房间',
		tryAgain: '重试',
		freeTitle: (h: string) => `@${h} 还没有被领取！`,
		freeBody: '还没有人领取这个房间，它可以是你的。',
		claim: (h: string) => `领取 @${h}`,
		home: '返回首页'
	},
	footer: { freeSoftware: '自由软件', sourceCode: '源代码' }
};
