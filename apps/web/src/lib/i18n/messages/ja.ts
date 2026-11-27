import type { Messages } from './en.ts';

export const ja: Messages = {
	meta: { title: 'Diceri — 双方向の音声翻訳ができる Google Meet の代わり' },
	header: {
		signOut: 'ログアウト',
		deleteVoice: '自分の声を削除',
		deleteVoiceConfirm: '保存された声の録音を削除しますか？次の通話で新しく録音します。',
		voiceDeleted: '声の録音を削除しました。',
		language: '言語',
		toDark: 'ダークテーマに切り替え',
		toLight: 'ライトテーマに切り替え'
	},
	hero: {
		headline: 'Google Meet の代わりに。',
		promise: '自分の言語で話して、自分の言語で聞ける。',
		voice: '会議の全員が、あなたの声を自分の言語で聞けます。',
		openSource: 'オープンソースです',
		illustration: 'イラスト：一人が話し、真ん中で Diceri が翻訳し、もう一人が自分の言語で聞きます。'
	},
	onboarding: {
		label: 'ルームのユーザー名',
		mascotTitle: 'ルームのマスコットを選んでください',
		mascotHint: '訪れる人を出迎えます',
		continue: '続ける',
		back: '戻る',
		usernamePlaceholder: 'room_username',
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
		tooEasy:
			'推測されやすいパスワードです。blue-otter-sings のように、関係のない単語を組み合わせてください。',
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
	lobby: {
		joinRoomTitle: (handle: string) => `@${handle} に参加`,
		joinRoomHint:
			'この通話に参加するには、Diceri のユーザー名でログインするか、初めての方は無料で作成してください。1分ほどで完了し、みなさんがそれぞれの言語で、あなた自身の声であなたの話を聞けるようになります。',
		camera: 'カメラ',
		cameraOn: 'カメラをオン',
		cameraOff: 'カメラをオフ',
		cameraBlocked: 'ビデオを表示するには、ブラウザでカメラを許可してください。',
		cameraUnavailable: 'カメラが見つかりません。',
		joinTitle: '会議に参加',
		yourName: 'あなたの名前',
		hearEveryoneIn: '全員の声を聞く言語',
		microphone: 'マイク',
		systemDefault: 'システムのデフォルト',
		microphonePreview: 'マイクのプレビュー',
		joinNow: '今すぐ参加',
		askToJoin: '参加をリクエスト',
		joining: '参加しています…',
		micBlocked:
			'マイクへのアクセスがブロックされました。ブラウザのアドレスバーから許可してください。',
		micUnavailable: 'マイクにアクセスできませんでした。',
		waitingToJoin: '参加待ち',
		deny: '拒否',
		admit: '承認',
		wantsToJoin: (name: string) => `${name} さんがこの通話への参加を希望しています`,
		hears: (language: string) => `（${language}で聞く）`,
		dismiss: '閉じる',
		cantJoin: 'この会議に参加できません',
		declined: 'ホストがリクエストを拒否しました。',
		somethingWrong: '問題が発生しました',
		back: '戻る',
		asking: '参加をリクエストしています…',
		willJoin: '誰かが承認すると会議に参加できます。',
		cancel: 'キャンセル'
	},
	voice: {
		requiredHint: (seconds: number) =>
			`短い文を${String(seconds)}秒間読み上げてください。翻訳はあなた自身の声で相手に届きます。録音は暗号化してアカウントに保存されます。`,
		pro: 'Pro',
		paidSoon: '近日、有料機能として提供予定です。',
		section: 'あなたの声',
		recording: '録音中 · 声に出して読んでください',
		secondsLeft: (seconds: number) => `${String(seconds)}秒`,
		done: '完了',
		keepReading: (seconds: number) => `読み続けてください（最低${String(seconds)}秒）`,
		creating: 'あなたの声を作成しています…',
		ready: 'あなたの声の準備ができました',
		redo: 'やり直す',
		readyHint: '翻訳があなたの声で聞こえます。録音は暗号化してアカウントに保存されます。',
		keptReady: '保存された声の準備ができました',
		keptHint: 'アカウントに保存された録音を使用しています。@ メニューからいつでも削除できます。',
		title: '翻訳をあなたの声にする',
		hint: (seconds: number) =>
			`短い文を${String(seconds)}秒間読み上げてください。任意で、録音は暗号化してアカウントに保存されます。`,
		tryAgain: 'もう一度試す',
		record: (seconds: number) => `自分の声を録音（${String(seconds)}秒）`,
		noVoice: 'はっきり聞き取れませんでした。文を声に出して読み、もう一度お試しください。',
		micNeeded: '声を録音するにはマイクへのアクセスが必要です。',
		failed: '声を作成できませんでした。'
	},
	call: {
		turnOnCamera: 'カメラをオン',
		turnOffCamera: 'カメラをオフ',
		turnOnMic: 'マイクをオンにする',
		turnOffMic: 'マイクをオフにする',
		leaveCall: '通話から退出',
		justLeave: '通話から退出するだけ',
		endForEveryone: '全員の通話を終了',
		people: '参加者',
		audioSettings: '音声設定',
		guest: 'ゲスト',
		enableAudio: 'クリックして他の参加者の音声を有効にする',
		connecting: '接続しています…',
		inCall: '通話中',
		closePanel: 'パネルを閉じる',
		you: (name: string) => `${name}（あなた）`,
		micOff: 'マイクはオフです',
		hearsEveryoneIn: '全員の声を聞く言語',
		hears: (language: string) => `${language}で聞く`,
		meetingDetails: '会議の詳細',
		meetingReady: '会議の準備ができました',
		close: '閉じる',
		shareLink: '話したい相手にこのリンクを共有してください。参加の承認を求められます。',
		copyLink: '会議のリンクをコピー',
		linkCopied: 'リンクをコピーしました',
		captions: '字幕',
		filterUnsupported: 'このブラウザは高度なノイズフィルタに対応していません。',
		filterFailed: '高度なノイズフィルタでエラーが発生しました。',
		micUnpublished: 'マイクの音声を送信できませんでした。'
	},
	audio: {
		microphone: 'マイク',
		systemDefault: 'システムのデフォルト',
		noiseAndEcho: 'ノイズとエコー',
		aiFilter: 'AI ノイズフィルタ（Krisp）',
		filterOff: 'オフ',
		filterNoise: 'ノイズ',
		filterVoice: '自分の声のみ',
		filterOffHint: 'ブラウザの処理のみ。',
		filterNoiseHint: '背景ノイズを除去し、すべての声を残します。',
		filterVoiceHint: '近くで話す他の人の声も除去します（別途課金）。',
		echo: 'エコーキャンセル',
		echoHint: '相手の声がマイクから戻るのを防ぎます。',
		suppression: 'ブラウザのノイズ抑制',
		suppressionOff: 'AI フィルタがオンの間はオフです。',
		suppressionHint: '基本的な内蔵フィルタ。',
		autoGain: '自動ゲイン',
		autoGainHint: '音量を一定に保ちます。',
		outgoingLevel: '送信レベル（フィルタ後）',
		muted: 'ミュート中',
		hearMyself: '自分の声を聞く',
		headphones: 'ヘッドホンを使ってください。そうしないと相手にあなたの声が二重に聞こえます。',
		monitorOff: 'オフ',
		local: 'ローカル',
		viaServer: 'サーバー経由',
		monitorOffHint: '自分の声は聞こえません。',
		localHint: 'ノイズフィルタ後のマイク音声をすぐに聞きます。フィルタの品質を確認できます。',
		viaServerHint: 'メディアサーバーを往復した自分の声を聞きます。伝送品質を確認できます。',
		connection: '接続',
		direction: '方向',
		bitrate: 'ビットレート',
		loss: 'ロス',
		jitter: 'ジッター',
		rtt: 'RTT',
		sending: '送信',
		receiving: '受信'
	},
	meeting: {
		left: '会議から退出しました',
		endedByMe: '全員の通話を終了しました',
		endedByHost: 'ホストが通話を終了しました',
		rejoin: '再参加',
		home: 'ホーム画面に戻る'
	},
	transcript: {
		title: '文字起こし',
		copy: 'コピー',
		download: 'ダウンロード',
		empty: '通話中、発言はありませんでした。',
		copied: '文字起こしをコピーしました'
	},
	plans: {
		usedThisMonth: (used: number, total: number) =>
			`今月${String(total)}分中${String(used)}分を使用`,
		freeLine: (minutes: number) => `無料：毎月${String(minutes)}分まで通話できます。`
	},
	limits: {
		beta: 'Diceri はクローズドベータ中で、現在すべての枠が埋まっています。時間をおいてもう一度お試しください。',
		network:
			'このネットワークで取得できるルーム名の上限に達しています。取得済みのルーム名でログインしてください。',
		quota: '今月のプランの通話時間を使い切りました。毎月1日(UTC)にリセットされます。',
		daily: '本日の無料通話時間を使い切りました。毎日午前0時（UTC）にリセットされます。',
		monthly: '今月のDiceriの無料枠は上限に達しました。毎月1日(UTC)にリセットされます。',
		busy: '現在Diceriは混み合っています。数分後にもう一度お試しください。',
		timeLeft: (time: string) => `残り${time}`,
		timeUp: '無料通話の時間が終了しました',
		timeUpHint: '今月の無料通話時間を使い切りました。毎月1日(UTC)にリセットされます。'
	},
	terms: {
		title: '利用規約',
		updated: '最終更新日：2026年11月27日',
		notice:
			'ルームを作成すると、通話であなたの声を再現するために声の録音を暗号化して保存することを含め、利用規約に同意したことになります。',
		read: '利用規約を読む',
		sections: [
			{
				heading: 'Diceri とは',
				body: 'Diceri は、リアルタイム音声翻訳付きのビデオ通話サービスで、フリーかつオープンソースのソフトウェア（AGPL-3.0）として運営されています。各アカウントは diceri.com/@username にルームを1つ持ちます。Diceri を利用することで、本規約に同意したものとみなされます。'
			},
			{
				heading: 'アカウント',
				body: 'ユーザー名とパスワードで登録します。メールアドレスは不要です。パスワードは他人に教えないでください。1つのネットワークから作成できるアカウントは数個までで、無料プランには毎月一定の通話時間（分）が含まれます。'
			},
			{
				heading: 'あなたの声',
				body: '通話に参加するには、約10秒間あなたの声を録音します。この録音を使って音声プロバイダーであなたの声のコピーを作成し、相手があなたの翻訳をあなた自身の声で聞けるようにします。録音は暗号化してアカウントに保存し、以降の通話であなたの声を再現できるようにします。音声プロバイダーを変更した場合は、新しいプロバイダーでも再現します。録音はあなた自身の翻訳を話すためだけに使われ、他の誰かの代わりに話すことには決して使われず、販売されたり他の目的で共有されたりすることもありません。'
			},
			{
				heading: '声の削除',
				body: '@ メニューの「自分の声を削除」から、いつでも録音を削除できます。アカウントを削除した場合も録音は削除されます。通話のために音声プロバイダーで作成されたコピーは、その通話が終わると削除されます。'
			},
			{
				heading: '保存するもの、しないもの',
				body: 'ユーザー名、保護された形式のパスワード、ルームのマスコット、プラン、使用した通話時間（分）、登録時のネットワークの暗号化されたフィンガープリント（IP アドレスそのものは保存しません）、そして声の録音を保存します。通話は録画・録音せず、話された内容も保存しません。文字起こしと字幕はあなたのブラウザー内にとどまります。'
			},
			{
				heading: 'サービス提供者',
				body: '通話は LiveKit を経由します。音声の文字起こし、翻訳、読み上げは ElevenLabs、Inworld、Soniox、Google Gemini が行います。これらの事業者は、通話の提供に必要な情報のみを受け取ります。'
			},
			{
				heading: '適正な利用',
				body: '法律に違反する行為、他人への嫌がらせ、なりすましに Diceri を使用しないでください。サービスを悪用するアカウントは、制限または停止することがあります。Diceri は現状のまま提供され、いかなる保証もありません。'
			},
			{
				heading: '変更',
				body: '本規約は更新されることがあります。上記の日付が最新版を示します。ご質問は github.com/julianosig/diceri.com で issue を作成してください。'
			}
		]
	},
	footer: { freeSoftware: 'フリーソフトウェア', sourceCode: 'ソースコード', terms: '利用規約' }
};
