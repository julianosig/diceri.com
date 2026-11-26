import type { Messages } from './en.ts';

export const hi: Messages = {
	meta: { title: 'Diceri — वही कॉल जिसे आप जानते हैं, हर भाषा में' },
	header: {
		signOut: 'साइन आउट',
		language: 'भाषा',
		toDark: 'डार्क थीम पर जाएँ',
		toLight: 'लाइट थीम पर जाएँ'
	},
	hero: {
		headline: 'वही वीडियो कॉल जिसे आप पहले से जानते हैं।',
		promise: 'अब हर भाषा में।',
		illustration:
			'चित्र: एक व्यक्ति बोलता है, बीच में Diceri अनुवाद करता है, और दूसरा व्यक्ति उसे अपनी भाषा में सुनता है।'
	},
	onboarding: {
		label: 'लॉग इन करें या अपने रूम का यूज़रनेम लें',
		usernamePlaceholder: 'yourname',
		checking: (handle: string) => `@${handle} की जाँच हो रही है…`,
		available: (handle: string) => `@${handle} उपलब्ध है — इसे ले लें`,
		welcomeBack: (handle: string) => `फिर से स्वागत है, @${handle}`,
		invalid: '3–20 अंग्रेज़ी छोटे अक्षर (a–z), अंक या _, जो किसी अक्षर से शुरू हों।',
		checkFailed: 'नाम की जाँच नहीं हो सकी। फिर से कोशिश करें।',
		password: 'पासवर्ड',
		repeatPassword: 'पासवर्ड दोबारा लिखें',
		newPasswordPlaceholder: (min: number) => `कम से कम ${String(min)} अक्षर`,
		passwordPlaceholder: 'आपका पासवर्ड',
		repeatPlaceholder: 'फिर से लिखें',
		passwordsMatch: 'पासवर्ड मेल खाते हैं',
		tooShort: (min: number) => `कम से कम ${String(min)} अक्षरों का इस्तेमाल करें।`,
		tooLong: (max: number) => `ज़्यादा से ज़्यादा ${String(max)} अक्षरों का इस्तेमाल करें।`,
		mismatch: 'पासवर्ड मेल नहीं खाते।',
		create: 'मेरा रूम बनाएँ',
		creating: 'बन रहा है…',
		signIn: 'साइन इन',
		signingIn: 'साइन इन हो रहा है…',
		taken: 'यह नाम अभी-अभी किसी ने ले लिया।',
		wrongPassword: 'गलत पासवर्ड।',
		failed: 'कुछ गड़बड़ हो गई। फिर से कोशिश करें।'
	},
	mascots: {
		legend: 'रूम मैस्कॉट',
		names: {
			wizard: 'जादूगर',
			robot: 'रोबोट',
			astronaut: 'अंतरिक्ष यात्री',
			alien: 'एलियन',
			ninja: 'निंजा',
			dragon: 'ड्रैगन',
			knight: 'योद्धा',
			ghost: 'भूत',
			cybercat: 'साइबर बिल्ली',
			slime: 'स्लाइम',
			pirate: 'अंतरिक्ष समुद्री डाकू',
			gamer: 'गेमर'
		}
	},
	openRoom: {
		livesAt: (username: string) => `आपका रूम यहाँ है: diceri.com/@${username}`,
		whoGetsIn: 'कौन आ सकता है',
		approval: 'हर व्यक्ति को मेरी मंज़ूरी चाहिए',
		open: 'कोई भी जुड़ सकता है',
		openHint: 'रूम खुला रहने तक, जो भी आपका @ जानता है वह सीधे आ सकता है।',
		submit: 'मेरा रूम खोलें',
		submitting: 'खुल रहा है…',
		failed: 'आपका रूम नहीं खुल सका। फिर से कोशिश करें।'
	},
	roomOffline: {
		liveTitle: (h: string) => `@${h} अभी लाइव नहीं है`,
		liveBody: (h: string) =>
			`@${h} के कॉल शुरू करने पर यह रूम खुलता है। थोड़ी देर बाद आएँ — या कुछ ही सेकंड में अपना रूम बनाएँ।`,
		createOwn: 'अपना रूम बनाएँ',
		tryAgain: 'फिर से कोशिश करें',
		freeTitle: (h: string) => `@${h} अभी भी उपलब्ध है!`,
		freeBody: 'अभी तक किसी ने यह रूम नहीं लिया है। यह आपका हो सकता है।',
		claim: (h: string) => `@${h} लें`,
		home: 'होम पर वापस जाएँ'
	},
	footer: { freeSoftware: 'फ्री सॉफ़्टवेयर', sourceCode: 'सोर्स कोड' }
};
