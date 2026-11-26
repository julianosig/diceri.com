import { en, type Messages } from './messages/en.ts';
import type { Locale } from './locales.ts';

/**
 * English ships with the app (it is the fallback); every other language is its own
 * chunk, loaded only when someone picks it.
 */
export const MESSAGE_LOADERS: Record<Locale, () => Promise<Messages>> = {
	en: () => Promise.resolve(en),
	es: async () => (await import('./messages/es.ts')).es,
	pt: async () => (await import('./messages/pt.ts')).pt,
	fr: async () => (await import('./messages/fr.ts')).fr,
	de: async () => (await import('./messages/de.ts')).de,
	it: async () => (await import('./messages/it.ts')).it,
	ru: async () => (await import('./messages/ru.ts')).ru,
	ja: async () => (await import('./messages/ja.ts')).ja,
	ko: async () => (await import('./messages/ko.ts')).ko,
	zh: async () => (await import('./messages/zh.ts')).zh,
	hi: async () => (await import('./messages/hi.ts')).hi,
	ar: async () => (await import('./messages/ar.ts')).ar
};
