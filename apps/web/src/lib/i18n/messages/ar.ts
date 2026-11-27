import type { Messages } from './en.ts';

/**
 * Written right to left; `locales.ts` sets `dir="rtl"` on the page. Latin handles and
 * addresses are wrapped in Unicode isolates (FSI … PDI) so `@ana` does not flip.
 */
const ltr = (text: string) => `\u2068${text}\u2069`;

export const ar: Messages = {
	meta: { title: 'Diceri — بديل Google Meet مع ترجمة صوتية في الاتجاهين' },
	header: {
		signOut: 'تسجيل الخروج',
		deleteVoice: 'احذف صوتي',
		deleteVoiceConfirm: 'هل تريد حذف تسجيل صوتك المحفوظ؟ ستسجّل تسجيلًا جديدًا في مكالمتك القادمة.',
		voiceDeleted: 'تم حذف تسجيل صوتك.',
		language: 'اللغة',
		toDark: 'التبديل إلى السمة الداكنة',
		toLight: 'التبديل إلى السمة الفاتحة'
	},
	hero: {
		headline: 'البديل لـ Google Meet',
		promise: 'حيث تتحدث وتستمع بلغتك أنت.',
		voice: 'يسمع الجميع في الاجتماع صوتك بلغتهم.',
		openSource: 'نحن مفتوحو المصدر',
		illustration: 'رسم توضيحي: شخص يتحدث، وDiceri يترجم في المنتصف، والشخص الآخر يسمعه بلغته.'
	},
	onboarding: {
		label: 'اسم مستخدم الغرفة',
		mascotTitle: 'اختر تميمة غرفتك',
		mascotHint: 'ترحّب بكل من يزور غرفتك',
		continue: 'متابعة',
		back: 'رجوع',
		usernamePlaceholder: 'room_username',
		checking: (handle: string) => `جارٍ التحقق من ${ltr(`@${handle}`)}…`,
		available: (handle: string) => `${ltr(`@${handle}`)} متاح — احجزه الآن`,
		welcomeBack: (handle: string) => `مرحبًا بعودتك، ${ltr(`@${handle}`)}`,
		invalid: 'من 3 إلى 20 حرفًا لاتينيًا صغيرًا (a–z) أو أرقامًا أو _، ويبدأ بحرف.',
		checkFailed: 'تعذّر التحقق من الاسم. حاول مرة أخرى.',
		password: 'كلمة المرور',
		repeatPassword: 'أعد كتابة كلمة المرور',
		newPasswordPlaceholder: (min: number) => `${String(min)} أحرف على الأقل`,
		passwordPlaceholder: 'كلمة مرورك',
		repeatPlaceholder: 'اكتبها مرة أخرى',
		passwordsMatch: 'كلمتا المرور متطابقتان',
		tooShort: (min: number) => `استخدم ${String(min)} أحرف على الأقل.`,
		tooLong: (max: number) => `استخدم ${String(max)} حرفًا كحد أقصى.`,
		tooEasy: 'سهلة التخمين جدًا. جرّب بضع كلمات غير مترابطة، مثل blue-otter-sings.',
		mismatch: 'كلمتا المرور غير متطابقتين.',
		create: 'أنشئ غرفتي',
		creating: 'جارٍ الإنشاء…',
		signIn: 'تسجيل الدخول',
		signingIn: 'جارٍ تسجيل الدخول…',
		taken: 'حُجز هذا الاسم للتو.',
		wrongPassword: 'كلمة المرور غير صحيحة.',
		failed: 'حدث خطأ ما. حاول مرة أخرى.'
	},
	mascots: {
		legend: 'تميمة الغرفة',
		names: {
			wizard: 'ساحر',
			robot: 'روبوت',
			astronaut: 'رائد فضاء',
			alien: 'كائن فضائي',
			ninja: 'نينجا',
			dragon: 'تنين',
			knight: 'فارس',
			ghost: 'شبح',
			cybercat: 'قط سايبر',
			slime: 'هلام',
			pirate: 'قرصان فضاء',
			gamer: 'لاعب'
		}
	},
	openRoom: {
		whoGetsIn: 'من يدخل',
		approval: 'أوافق على كل شخص',
		open: 'يمكن لأي شخص الانضمام',
		openHint: 'كل من يعرف @ الخاص بك يدخل مباشرة ما دامت الغرفة مفتوحة.',
		submit: 'افتح غرفتي',
		submitting: 'جارٍ الفتح…',
		failed: 'تعذّر فتح غرفتك. حاول مرة أخرى.'
	},
	roomOffline: {
		liveTitle: (h: string) => `${ltr(`@${h}`)} ليس في بث مباشر الآن`,
		liveBody: (h: string) =>
			`تُفتح هذه الغرفة عندما يبدأ ${ltr(`@${h}`)} مكالمة. عُد قريبًا — أو افتح غرفتك الخاصة في ثوانٍ.`,
		createOwn: 'أنشئ غرفتي الخاصة',
		tryAgain: 'حاول مرة أخرى',
		freeTitle: (h: string) => `${ltr(`@${h}`)} ما زال متاحًا!`,
		freeBody: 'لم يحجز أحد هذه الغرفة بعد. يمكن أن تكون لك.',
		claim: (h: string) => `احجز ${ltr(`@${h}`)}`,
		home: 'العودة إلى الصفحة الرئيسية'
	},
	lobby: {
		joinRoomTitle: (handle: string) => `انضم إلى ${ltr(`@${handle}`)}`,
		joinRoomHint:
			'للانضمام إلى هذه المكالمة، سجّل الدخول باسم المستخدم الخاص بك في Diceri، أو أنشئ واحدًا مجانًا إن كنت جديدًا. يستغرق ذلك دقيقة، ويتيح للجميع سماعك بلغتهم وبصوتك أنت.',
		camera: 'الكاميرا',
		cameraOn: 'تشغيل الكاميرا',
		cameraOff: 'إيقاف الكاميرا',
		cameraBlocked: 'اسمح بالكاميرا في متصفحك لعرض الفيديو.',
		cameraUnavailable: 'لم يتم العثور على كاميرا.',
		joinTitle: 'انضم إلى الاجتماع',
		yourName: 'اسمك',
		hearEveryoneIn: 'اسمع الجميع بلغة',
		microphone: 'الميكروفون',
		systemDefault: 'الإعداد الافتراضي للنظام',
		microphonePreview: 'معاينة الميكروفون',
		joinNow: 'انضم الآن',
		askToJoin: 'اطلب الانضمام',
		joining: 'جارٍ الانضمام…',
		micBlocked: 'تم حظر الوصول إلى الميكروفون. اسمح به من شريط العنوان في المتصفح.',
		micUnavailable: 'تعذّر الوصول إلى الميكروفون.',
		waitingToJoin: 'بانتظار الانضمام',
		deny: 'رفض',
		admit: 'قبول',
		wantsToJoin: (name: string) => `${ltr(name)} يريد الانضمام إلى هذه المكالمة`,
		hears: (language: string) => `(يسمع بـ${language})`,
		dismiss: 'إغلاق',
		cantJoin: 'لا يمكنك الانضمام إلى هذا الاجتماع',
		declined: 'رفض المضيف طلبك.',
		somethingWrong: 'حدث خطأ ما',
		back: 'رجوع',
		asking: 'جارٍ طلب الدخول…',
		willJoin: 'ستنضم إلى الاجتماع عندما يسمح لك أحد بالدخول.',
		cancel: 'إلغاء'
	},
	voice: {
		requiredHint: (seconds: number) =>
			`اقرأ جملة قصيرة لمدة ${String(seconds)} ثوانٍ: سيسمع الجميع ترجماتك بصوتك أنت. يُحفظ التسجيل مشفّرًا مع حسابك.`,
		pro: 'Pro',
		paidSoon: 'قريبًا كميزة مدفوعة.',
		section: 'صوتك',
		recording: 'جارٍ التسجيل · اقرأ بصوت عالٍ',
		secondsLeft: (seconds: number) => `${String(seconds)} ث`,
		done: 'تم',
		keepReading: (seconds: number) => `واصل القراءة (${String(seconds)} ث على الأقل)`,
		creating: 'جارٍ إنشاء صوتك…',
		ready: 'صوتك جاهز',
		redo: 'إعادة',
		readyHint: 'ستبدو الترجمات بصوتك. يُحفظ التسجيل مشفّرًا مع حسابك.',
		keptReady: 'صوتك المحفوظ جاهز',
		keptHint: 'نستخدم التسجيل المحفوظ مع حسابك. احذفه في أي وقت من قائمة @ الخاصة بك.',
		title: 'اجعل الترجمات بصوتك',
		hint: (seconds: number) =>
			`اقرأ جملة قصيرة لمدة ${String(seconds)} ثانية. اختياري، ويُحفظ التسجيل مشفّرًا مع حسابك.`,
		tryAgain: 'حاول مرة أخرى',
		record: (seconds: number) => `سجّل صوتي (${String(seconds)} ث)`,
		noVoice: 'لم نتمكن من سماعك بوضوح. اقرأ الجملة بصوت عالٍ وحاول مرة أخرى.',
		micNeeded: 'يلزم الوصول إلى الميكروفون لتسجيل صوتك.',
		failed: 'تعذّر إنشاء صوتك.'
	},
	call: {
		turnOnCamera: 'تشغيل الكاميرا',
		turnOffCamera: 'إيقاف الكاميرا',
		turnOnMic: 'تشغيل الميكروفون',
		turnOffMic: 'إيقاف الميكروفون',
		leaveCall: 'مغادرة المكالمة',
		justLeave: 'مغادرة المكالمة فقط',
		endForEveryone: 'إنهاء المكالمة للجميع',
		people: 'الأشخاص',
		audioSettings: 'إعدادات الصوت',
		guest: 'ضيف',
		enableAudio: 'انقر لتفعيل صوت المشاركين الآخرين',
		connecting: 'جارٍ الاتصال…',
		inCall: 'في المكالمة',
		closePanel: 'إغلاق اللوحة',
		you: (name: string) => `${ltr(name)} (أنت)`,
		micOff: 'الميكروفون متوقف',
		hearsEveryoneIn: 'يسمع الجميع بلغة',
		hears: (language: string) => `يسمع بـ${language}`,
		meetingDetails: 'تفاصيل الاجتماع',
		meetingReady: 'اجتماعك جاهز',
		close: 'إغلاق',
		shareLink: 'شارك هذا الرابط مع الشخص الذي تريد التحدث معه. سيُطلب منك السماح له بالدخول.',
		copyLink: 'نسخ رابط الاجتماع',
		linkCopied: 'تم نسخ الرابط',
		captions: 'الترجمة النصية',
		filterUnsupported: 'فلتر الضوضاء المحسّن غير مدعوم في هذا المتصفح.',
		filterFailed: 'فشل فلتر الضوضاء المحسّن.',
		micUnpublished: 'تعذّر بث الميكروفون.'
	},
	audio: {
		microphone: 'الميكروفون',
		systemDefault: 'الإعداد الافتراضي للنظام',
		noiseAndEcho: 'الضوضاء والصدى',
		aiFilter: 'فلتر الضوضاء بالذكاء الاصطناعي (Krisp)',
		filterOff: 'إيقاف',
		filterNoise: 'الضوضاء',
		filterVoice: 'صوتي فقط',
		filterOffHint: 'معالجة المتصفح فقط.',
		filterNoiseHint: 'يزيل ضوضاء الخلفية ويُبقي كل الأصوات.',
		filterVoiceHint: 'يزيل أيضًا أصوات الأشخاص الآخرين الذين يتحدثون بالقرب منك (بتكلفة منفصلة).',
		echo: 'إلغاء الصدى',
		echoHint: 'يمنع صوت الشخص الآخر من العودة عبر الميكروفون.',
		suppression: 'كتم الضوضاء في المتصفح',
		suppressionOff: 'متوقف أثناء تشغيل فلتر الذكاء الاصطناعي.',
		suppressionHint: 'فلتر أساسي مدمج.',
		autoGain: 'الكسب التلقائي',
		autoGainHint: 'يحافظ على ثبات مستوى صوتك.',
		outgoingLevel: 'المستوى الصادر (بعد الفلترة)',
		muted: 'مكتوم',
		hearMyself: 'اسمع نفسي',
		headphones: 'استخدم سماعات الرأس، وإلا سيسمعك الآخرون مرتين.',
		monitorOff: 'إيقاف',
		local: 'محلي',
		viaServer: 'عبر الخادم',
		monitorOffHint: 'لا تسمع نفسك.',
		localHint: 'اسمع الميكروفون بعد فلتر الضوضاء فورًا. لتقييم جودة الفلتر.',
		viaServerHint: 'اسمع نفسك بعد رحلة ذهاب وإياب عبر خادم الوسائط. لتقييم الإرسال.',
		connection: 'الاتصال',
		direction: 'الاتجاه',
		bitrate: 'معدل البت',
		loss: 'الفقد',
		jitter: 'التذبذب',
		rtt: 'RTT',
		sending: 'الإرسال',
		receiving: 'الاستقبال'
	},
	meeting: {
		left: 'غادرت الاجتماع',
		endedByMe: 'أنهيت المكالمة للجميع',
		endedByHost: 'أنهى المضيف المكالمة',
		rejoin: 'إعادة الانضمام',
		home: 'العودة إلى الشاشة الرئيسية'
	},
	transcript: {
		title: 'النص المكتوب',
		copy: 'نسخ',
		download: 'تنزيل',
		empty: 'لم يُقل شيء أثناء وجودك في المكالمة.',
		copied: 'تم نسخ النص'
	},
	plans: {
		usedThisMonth: (used: number, total: number) =>
			`${String(used)} من ${String(total)} د مستخدمة هذا الشهر`,
		freeLine: (minutes: number) => `مجاني: ${String(minutes)} د من المكالمات كل شهر.`
	},
	limits: {
		network: 'حجزت هذه الشبكة الحد الأقصى المسموح من أسماء الغرف. سجّل الدخول بأحدها بدلًا من ذلك.',
		quota: 'استخدمت دقائق المكالمات في خطتك لهذا الشهر. تتجدد في اليوم الأول من كل شهر (UTC).',
		daily: 'استخدمت دقائق المكالمات المجانية لهذا اليوم. تتجدد عند منتصف الليل (UTC).',
		monthly: 'نفدت السعة المجانية لـ Diceri هذا الشهر. تتجدد في اليوم الأول من كل شهر (UTC).',
		busy: 'وصل Diceri إلى أقصى سعته الآن. حاول مرة أخرى بعد بضع دقائق.',
		timeLeft: (time: string) => `متبقٍ ${time}`,
		timeUp: 'انتهى وقت مكالمتك المجانية',
		timeUpHint: 'استخدمت الدقائق المجانية لهذا الشهر. تتجدد في اليوم الأول من كل شهر (UTC).'
	},
	terms: {
		title: 'شروط الاستخدام',
		updated: 'آخر تحديث: 27 نوفمبر 2026',
		notice:
			'بإنشاء غرفتك فإنك توافق على شروط الاستخدام، بما في ذلك الاحتفاظ بتسجيل صوتك مشفّرًا لإعادة إنشاء صوتك في المكالمات.',
		read: 'اقرأ شروط الاستخدام',
		sections: [
			{
				heading: 'ما هو Diceri',
				body: `Diceri خدمة مكالمات فيديو مع ترجمة صوتية فورية، تُدار كبرمجيات حرة ومفتوحة المصدر (AGPL-3.0). لكل حساب غرفة واحدة على ${ltr('diceri.com/@username')}. استخدامك لـ Diceri يعني قبولك لهذه الشروط.`
			},
			{
				heading: 'حسابك',
				body: 'تسجّل باسم مستخدم وكلمة مرور؛ لا يُطلب أي بريد إلكتروني. لا تشارك كلمة مرورك مع أحد. لا يمكن إنشاء سوى عدد قليل من الحسابات لكل شبكة، وتتضمن الخطة المجانية رصيدًا شهريًا من دقائق المكالمات.'
			},
			{
				heading: 'صوتك',
				body: 'للانضمام إلى مكالمة تسجّل نحو 10 ثوانٍ من صوتك. نستخدمه لإنشاء نسخة من صوتك لدى مزوّد الصوت لدينا، كي يسمع الآخرون ترجماتك بصوتك أنت. نحتفظ بالتسجيل مع حسابك مشفّرًا، كي نتمكن من إعادة إنشاء صوتك في المكالمات اللاحقة، بما في ذلك لدى مزوّد صوت آخر إذا غيّرناه. لا يُستخدم إلا لنطق ترجماتك أنت، ولا يُستخدم أبدًا للتحدث نيابةً عن أي شخص آخر، ولا يُباع أبدًا ولا يُشارَك لأي أغراض أخرى.'
			},
			{
				heading: 'حذف صوتك',
				body: 'يمكنك حذف تسجيلك في أي وقت عبر «احذف صوتي» في قائمة @ الخاصة بك؛ ويُحذف أيضًا مع حسابك. أما النسخ التي تُنشأ لدى مزوّد الصوت لمكالمة ما فتُحذف عند انتهاء تلك المكالمة.'
			},
			{
				heading: 'ما نحتفظ به وما لا نحتفظ به',
				body: 'نحتفظ باسم المستخدم الخاص بك، وصيغة محمية من كلمة مرورك، وتميمة غرفتك، وخطتك، وعدد دقائق المكالمات التي استخدمتها، وبصمة مشفّرة للشبكة التي سجّلت منها (وليس عنوان IP الخاص بك أبدًا)، وتسجيل صوتك. لا نسجّل المكالمات ولا نحتفظ بما قيل: تبقى النصوص المفرّغة والترجمة النصية في متصفحك.'
			},
			{
				heading: 'مزوّدو الخدمة',
				body: 'تمرّ المكالمات عبر LiveKit؛ ويُفرَّغ الكلام نصيًا ويُترجم ويُنطق بواسطة ElevenLabs وSoniox وGoogle Gemini. ولا يتلقّون إلا ما يلزم لتقديم المكالمة.'
			},
			{
				heading: 'الاستخدام العادل',
				body: 'لا تستخدم Diceri لمخالفة القانون أو مضايقة الآخرين أو انتحال شخصية أي أحد. قد نقيّد الحسابات التي تسيء استخدام الخدمة أو نغلقها. يُقدَّم Diceri كما هو، دون أي ضمانات.'
			},
			{
				heading: 'التغييرات',
				body: `قد نحدّث هذه الشروط؛ ويشير التاريخ أعلاه إلى أحدث نسخة. للأسئلة: افتح issue على ${ltr('github.com/julianosig/diceri.com')}.`
			}
		]
	},
	footer: { freeSoftware: 'برمجيات حرة', sourceCode: 'الشيفرة المصدرية', terms: 'شروط الاستخدام' }
};
