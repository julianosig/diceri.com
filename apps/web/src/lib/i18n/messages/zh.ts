import type { Messages } from './en.ts';

/** Simplified Chinese. */
export const zh: Messages = {
	meta: { title: 'Diceri — 支持双向语音翻译的 Google Meet 替代方案' },
	header: {
		signOut: '退出登录',
		deleteVoice: '删除我的声音',
		deleteVoiceConfirm: '删除已保存的声音录音？你将在下次通话时重新录制。',
		voiceDeleted: '你的声音录音已删除。',
		language: '语言',
		toDark: '切换到深色主题',
		toLight: '切换到浅色主题'
	},
	hero: {
		headline: 'Google Meet 的替代方案',
		promise: '用自己的语言说，也用自己的语言听。',
		voice: '会议中的每个人都用自己的语言听到你的声音。',
		openSource: '我们是开源的',
		illustration: '插图：一个人说话，Diceri 在中间翻译，另一个人用自己的语言听到。'
	},
	onboarding: {
		label: '房间用户名',
		mascotTitle: '选择你的房间吉祥物',
		mascotHint: '它会迎接每一位访客',
		continue: '继续',
		back: '返回',
		usernamePlaceholder: 'room_username',
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
		tooEasy: '太容易被猜到了。试试几个不相关的词，比如 blue-otter-sings。',
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
	lobby: {
		joinRoomTitle: (handle: string) => `加入 @${handle}`,
		joinRoomHint:
			'要加入这次通话，请用你的 Diceri 用户名登录；如果是新用户，可以免费创建一个。只需一分钟，大家就能用自己的语言、听到你本人的声音。',
		camera: '摄像头',
		cameraOn: '打开摄像头',
		cameraOff: '关闭摄像头',
		cameraBlocked: '请在浏览器中允许使用摄像头以显示你的视频。',
		cameraUnavailable: '未找到摄像头。',
		joinTitle: '加入会议',
		yourName: '你的名字',
		hearEveryoneIn: '用以下语言听所有人',
		microphone: '麦克风',
		systemDefault: '系统默认',
		microphonePreview: '麦克风预览',
		joinNow: '立即加入',
		askToJoin: '请求加入',
		joining: '正在加入…',
		micBlocked: '麦克风访问被阻止。请在浏览器地址栏中允许访问。',
		micUnavailable: '无法访问麦克风。',
		waitingToJoin: '等待加入',
		deny: '拒绝',
		admit: '允许',
		wantsToJoin: (name: string) => `${name} 想加入此通话`,
		hears: (language: string) => `（收听${language}）`,
		dismiss: '关闭',
		cantJoin: '你无法加入此会议',
		declined: '主持人拒绝了你的请求。',
		somethingWrong: '出了点问题',
		back: '返回',
		asking: '正在请求进入…',
		willJoin: '有人允许后，你就会加入会议。',
		cancel: '取消'
	},
	voice: {
		requiredHint: (seconds: number) =>
			`朗读一句短句 ${String(seconds)} 秒：大家会用你自己的声音听到你的翻译。录音会加密保存在你的账户中。`,
		pro: 'Pro',
		paidSoon: '即将作为付费功能推出。',
		section: '你的声音',
		recording: '录音中 · 请朗读',
		secondsLeft: (seconds: number) => `${String(seconds)} 秒`,
		done: '完成',
		keepReading: (seconds: number) => `继续朗读（至少 ${String(seconds)} 秒）`,
		creating: '正在生成你的声音…',
		ready: '你的声音已就绪',
		redo: '重录',
		readyHint: '翻译将使用你的声音。录音会加密保存在你的账户中。',
		keptReady: '你保存的声音已就绪',
		keptHint: '正在使用保存在你账户中的录音。可随时在 @ 菜单中删除。',
		title: '让翻译听起来像你',
		hint: (seconds: number) =>
			`朗读一句短句，持续 ${String(seconds)} 秒。可选；录音会加密保存在你的账户中。`,
		tryAgain: '重试',
		record: (seconds: number) => `录制我的声音（${String(seconds)} 秒）`,
		noVoice: '没能听清你的声音。请大声朗读这句话，然后重试。',
		micNeeded: '录制声音需要麦克风权限。',
		failed: '无法生成你的声音。'
	},
	call: {
		turnOnCamera: '打开摄像头',
		turnOffCamera: '关闭摄像头',
		turnOnMic: '打开麦克风',
		turnOffMic: '关闭麦克风',
		leaveCall: '离开通话',
		justLeave: '仅离开通话',
		endForEveryone: '为所有人结束通话',
		people: '参与者',
		audioSettings: '音频设置',
		guest: '访客',
		enableAudio: '点击以开启其他参与者的音频',
		connecting: '正在连接…',
		inCall: '通话中',
		closePanel: '关闭面板',
		you: (name: string) => `${name}（你）`,
		micOff: '麦克风已关闭',
		hearsEveryoneIn: '收听所有人的语言',
		hears: (language: string) => `收听${language}`,
		meetingDetails: '会议详情',
		meetingReady: '你的会议已准备就绪',
		close: '关闭',
		shareLink: '把这个链接分享给你想交谈的人。对方加入时会请你允许。',
		copyLink: '复制会议链接',
		linkCopied: '链接已复制',
		captions: '字幕',
		filterUnsupported: '此浏览器不支持增强降噪。',
		filterFailed: '增强降噪出错。',
		micUnpublished: '无法发布麦克风。'
	},
	audio: {
		microphone: '麦克风',
		systemDefault: '系统默认',
		noiseAndEcho: '噪音和回声',
		aiFilter: 'AI 降噪（Krisp）',
		filterOff: '关闭',
		filterNoise: '降噪',
		filterVoice: '仅我的声音',
		filterOffHint: '仅使用浏览器处理。',
		filterNoiseHint: '去除背景噪音，保留所有人声。',
		filterVoiceHint: '同时去除附近其他人的说话声（单独计费）。',
		echo: '回声消除',
		echoHint: '防止对方的声音通过你的麦克风传回去。',
		suppression: '浏览器噪音抑制',
		suppressionOff: 'AI 降噪开启时关闭。',
		suppressionHint: '基础内置过滤。',
		autoGain: '自动增益',
		autoGainHint: '保持音量稳定。',
		outgoingLevel: '输出电平（过滤后）',
		muted: '已静音',
		hearMyself: '听自己的声音',
		headphones: '请使用耳机，否则大家会听到你两次。',
		monitorOff: '关闭',
		local: '本地',
		viaServer: '经服务器',
		monitorOffHint: '你听不到自己的声音。',
		localHint: '即时收听降噪后的麦克风声音，用于判断降噪效果。',
		viaServerHint: '收听经媒体服务器往返后的声音，用于判断传输质量。',
		connection: '连接',
		direction: '方向',
		bitrate: '比特率',
		loss: '丢包',
		jitter: '抖动',
		rtt: 'RTT',
		sending: '发送',
		receiving: '接收'
	},
	meeting: {
		left: '你已离开会议',
		endedByMe: '你已为所有人结束通话',
		endedByHost: '主持人已结束通话',
		rejoin: '重新加入',
		home: '返回主屏幕'
	},
	transcript: {
		title: '文字记录',
		copy: '复制',
		download: '下载',
		empty: '你在通话期间没有人发言。',
		copied: '文字记录已复制'
	},
	plans: {
		betaLabel: '公开测试',
		betaSpots: (left: number, total: number) => `${String(total)} 个名额，还剩 ${String(left)} 个`,
		betaFull: '目前名额已满',
		usedThisMonth: (used: number, total: number) =>
			`本月已用 ${String(used)}/${String(total)} 分钟`,
		freeLine: (minutes: number) => `免费：每月 ${String(minutes)} 分钟通话。`
	},
	limits: {
		beta: 'Diceri 正在进行公开测试，目前名额已满。请稍后再试。',
		network: '这个网络领取的房间用户名已达上限。请用其中一个登录。',
		quota: '你本月套餐的通话分钟数已用完，将在每月 1 日（UTC）重置。',
		daily: '你今天的免费通话分钟数已用完，将在午夜（UTC）重置。',
		monthly: 'Diceri 本月的免费额度已用完，将在每月 1 日（UTC）重置。',
		busy: 'Diceri 当前已满负荷，请几分钟后再试。',
		timeLeft: (time: string) => `剩余 ${time}`,
		timeUp: '你的免费通话时间已用完',
		timeUpHint: '你本月的免费分钟数已用完，将在每月 1 日（UTC）重置。'
	},
	terms: {
		title: '使用条款',
		updated: '最后更新：2026 年 11 月 27 日',
		notice: '创建房间即表示你同意使用条款，包括加密保存你的声音录音，以便在通话中重现你的声音。',
		read: '阅读使用条款',
		sections: [
			{
				heading: 'Diceri 是什么',
				body: 'Diceri 是一项带实时语音翻译的视频通话服务，以自由开源软件（AGPL-3.0）的形式运营。每个账户在 diceri.com/@username 拥有一个房间。使用 Diceri 即表示接受本条款。'
			},
			{
				heading: '你的账户',
				body: '你用用户名和密码注册，无需提供电子邮箱。请勿将密码告诉他人。每个网络只能创建少量账户，免费套餐每月包含一定的通话分钟数。'
			},
			{
				heading: '你的声音',
				body: '加入通话时，你需要录制约 10 秒的声音。我们用它在我们的语音服务商处创建你声音的副本，让大家用你自己的声音听到你的翻译。我们将录音加密后与你的账户一起保存，以便在之后的通话中重现你的声音，包括在我们更换语音服务商时使用新的服务商。录音仅用于说出你自己的翻译，绝不会用来替其他任何人说话，也绝不会出售或为其他目的共享。'
			},
			{
				heading: '删除你的声音',
				body: '你可以随时通过 @ 菜单中的“删除我的声音”删除录音；删除账号时录音也会一并删除。在语音服务商处创建的你的声音副本，会在录音保存期间留作通话使用，并随录音一起删除。'
			},
			{
				heading: '我们保存什么，不保存什么',
				body: '我们保存你的用户名、经过保护处理的密码、房间吉祥物、套餐、已用的通话分钟数、你注册时所用网络的加密指纹（绝不保存你的 IP 地址）以及你的声音录音。我们不录制通话，也不保存通话内容：转写文本和字幕只保留在你的浏览器中。'
			},
			{
				heading: '服务提供商',
				body: '通话通过 LiveKit 进行；语音由 ElevenLabs、Inworld、Soniox 和 Google Gemini 进行转写、翻译和朗读。它们只会收到提供通话所必需的内容。'
			},
			{
				heading: '合理使用',
				body: '请勿使用 Diceri 违反法律、骚扰他人或冒充任何人。我们可能会限制或关闭滥用服务的账户。Diceri 按“现状”提供，不作任何保证。'
			},
			{
				heading: '变更',
				body: '我们可能会更新本条款；上方日期为最新版本。如有疑问，请在 github.com/julianosig/diceri.com 提交 issue。'
			}
		]
	},
	footer: { freeSoftware: '自由软件', sourceCode: '源代码', terms: '使用条款' }
};
