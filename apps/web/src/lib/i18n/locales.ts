/**
 * The interface languages, each named in itself for the picker. Chosen to reach the most
 * people: the most spoken first languages plus the main languages of the web.
 */
export const LOCALES = [
	{ code: 'en', name: 'English', dir: 'ltr' },
	{ code: 'es', name: 'Español', dir: 'ltr' },
	{ code: 'pt', name: 'Português', dir: 'ltr' },
	{ code: 'fr', name: 'Français', dir: 'ltr' },
	{ code: 'de', name: 'Deutsch', dir: 'ltr' },
	{ code: 'it', name: 'Italiano', dir: 'ltr' },
	{ code: 'ru', name: 'Русский', dir: 'ltr' },
	{ code: 'ja', name: '日本語', dir: 'ltr' },
	{ code: 'ko', name: '한국어', dir: 'ltr' },
	{ code: 'zh', name: '中文', dir: 'ltr' },
	{ code: 'hi', name: 'हिन्दी', dir: 'ltr' },
	{ code: 'ar', name: 'العربية', dir: 'rtl' }
] as const;

export type Locale = (typeof LOCALES)[number]['code'];

export const DEFAULT_LOCALE: Locale = 'en';

export function isLocale(value: unknown): value is Locale {
	return LOCALES.some((locale) => locale.code === value);
}

export function direction(locale: Locale): 'ltr' | 'rtl' {
	return LOCALES.find((entry) => entry.code === locale)?.dir ?? 'ltr';
}

/**
 * The first supported language among the browser's preferences, matched on the primary
 * subtag (`pt-BR` → `pt`, `zh-TW` → `zh`), else English.
 */
export function matchLocale(preferred: readonly string[]): Locale {
	for (const tag of preferred) {
		const primary = tag.toLowerCase().split('-')[0];
		if (isLocale(primary)) return primary;
	}
	return DEFAULT_LOCALE;
}
