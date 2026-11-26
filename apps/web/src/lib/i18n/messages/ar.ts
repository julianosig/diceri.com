import type { Messages } from './en.ts';

/**
 * Written right to left; `locales.ts` sets `dir="rtl"` on the page. Latin handles and
 * addresses are wrapped in Unicode isolates (FSI … PDI) so `@ana` does not flip.
 */
const ltr = (text: string) => `\u2068${text}\u2069`;

export const ar: Messages = {
	meta: { title: 'Diceri — المكالمة التي تعرفها، بكل اللغات' },
	header: {
		signOut: 'تسجيل الخروج',
		language: 'اللغة',
		toDark: 'التبديل إلى السمة الداكنة',
		toLight: 'التبديل إلى السمة الفاتحة'
	},
	hero: {
		headline: 'مكالمة الفيديو التي تعرفها.',
		promise: 'الآن بكل اللغات.',
		illustration: 'رسم توضيحي: شخص يتحدث، وDiceri يترجم في المنتصف، والشخص الآخر يسمعه بلغته.'
	},
	onboarding: {
		label: 'سجّل الدخول أو احجز اسم غرفتك',
		usernamePlaceholder: 'yourname',
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
	footer: { freeSoftware: 'برمجيات حرة', sourceCode: 'الشيفرة المصدرية' }
};
