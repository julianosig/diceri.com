import { i18n } from '#lib/i18n/index.ts';

// The app is a client-rendered SPA: media APIs only exist in the browser.
export const ssr = false;
export const prerender = false;

/** The interface language loads before the first render, so no English flashes first. */
export async function load(): Promise<void> {
	await i18n.init();
}
