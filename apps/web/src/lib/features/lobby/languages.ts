import { LANGUAGES, matchBrowserLanguage } from '#lib/features/translation/index.ts';

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
