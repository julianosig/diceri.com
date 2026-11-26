import { DEFAULT_LOCALE, direction, isLocale, matchLocale, type Locale } from './locales.ts';
import { MESSAGE_LOADERS } from './message-loaders.ts';
import { en, type Messages } from './messages/en.ts';

/** Where the visitor's language choice is kept. */
export const LOCALE_STORAGE_KEY = 'diceri-locale';

function savedLocale(): Locale | null {
	try {
		const saved = localStorage.getItem(LOCALE_STORAGE_KEY);
		return isLocale(saved) ? saved : null;
	} catch {
		return null;
	}
}

/**
 * The interface language: the saved choice, else the browser's, else English. A global
 * client preference, so module-level state is the right place. `t` is the active
 * message catalogue; components read `i18n.t.<section>.<key>`.
 */
class I18nState {
	locale = $state<Locale>(DEFAULT_LOCALE);
	t = $state.raw<Messages>(en);

	/** Picks the starting language; awaited before first render so nothing flashes. */
	async init(): Promise<void> {
		await this.apply(savedLocale() ?? matchLocale(navigator.languages));
	}

	/** Switches language and remembers the choice. */
	async set(locale: Locale): Promise<void> {
		await this.apply(locale);
		try {
			localStorage.setItem(LOCALE_STORAGE_KEY, locale);
		} catch {
			// Storage can be blocked (private mode); the choice then lasts for this page.
		}
	}

	private async apply(locale: Locale): Promise<void> {
		this.t = await MESSAGE_LOADERS[locale]();
		this.locale = locale;
		document.documentElement.lang = locale;
		document.documentElement.dir = direction(locale);
	}
}

export const i18n = new I18nState();
