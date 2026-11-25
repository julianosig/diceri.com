/** Languages offered in the lobby: the Tier 1 set from docs/decisions/0005. */
export const LANGUAGES = [
	{ code: 'pt-BR', label: 'Português (Brasil)' },
	{ code: 'en', label: 'English' },
	{ code: 'es', label: 'Español' },
	{ code: 'fr', label: 'Français' },
	{ code: 'de', label: 'Deutsch' },
	{ code: 'it', label: 'Italiano' },
	{ code: 'nl', label: 'Nederlands' },
	{ code: 'pl', label: 'Polski' },
	{ code: 'ru', label: 'Русский' },
	{ code: 'uk', label: 'Українська' },
	{ code: 'tr', label: 'Türkçe' },
	{ code: 'ar', label: 'العربية' },
	{ code: 'hi', label: 'हिन्दी' },
	{ code: 'zh', label: '中文' },
	{ code: 'ja', label: '日本語' },
	{ code: 'ko', label: '한국어' },
	{ code: 'vi', label: 'Tiếng Việt' },
	{ code: 'id', label: 'Bahasa Indonesia' }
] as const;

const KEY = 'diceri:language';

/** Last chosen language, else the browser's, else English. */
export function loadLanguage(): string {
	try {
		const saved = localStorage.getItem(KEY);
		if (saved && LANGUAGES.some((l) => l.code === saved)) return saved;
	} catch {
		// Storage blocked: fall through to the browser language.
	}
	return matchBrowserLanguage(navigator.language);
}

export function saveLanguage(code: string): void {
	try {
		localStorage.setItem(KEY, code);
	} catch {
		// Storage blocked: nothing to remember.
	}
}

export function matchBrowserLanguage(tag: string): string {
	const exact = LANGUAGES.find((l) => l.code.toLowerCase() === tag.toLowerCase());
	const primary = tag.split('-')[0]?.toLowerCase();
	const byPrimary = LANGUAGES.find((l) => l.code.split('-')[0] === primary);
	return (exact ?? byPrimary ?? LANGUAGES[1]).code;
}

/** Human label for a language code, or the code itself when unknown. */
export function languageLabel(code: string): string {
	return LANGUAGES.find((l) => l.code === code)?.label ?? code;
}
