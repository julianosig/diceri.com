/**
 * What people read while recording their voice (~10 s): plain conversation. Consent to
 * keeping the recording comes from the terms of use (ADR 0015), not from this text.
 */
const SCRIPTS: Partial<Record<string, string>> = {
	pt: 'Olá! É um prazer te conhecer. Fico muito feliz que a gente possa conversar hoje, mesmo falando idiomas diferentes, e espero que esta chamada seja ótima para nós dois.',
	en: "Hi! It's great to meet you. I'm really happy we can talk today, even though we speak different languages, and I hope this call goes well for both of us.",
	es: '¡Hola! Es un placer conocerte. Me alegra mucho que podamos hablar hoy, aunque hablemos idiomas diferentes, y espero que esta llamada salga muy bien para los dos.',
	fr: "Bonjour ! Ravi de te rencontrer. Je suis très content que nous puissions parler aujourd'hui, même dans des langues différentes, et j'espère que cet appel se passera bien pour nous deux.",
	de: 'Hallo! Schön, dich kennenzulernen. Ich freue mich sehr, dass wir heute reden können, auch wenn wir verschiedene Sprachen sprechen, und ich hoffe, dieser Anruf wird für uns beide gut.',
	it: 'Ciao! È un piacere conoscerti. Sono molto felice che possiamo parlare oggi, anche se parliamo lingue diverse, e spero che questa chiamata vada benissimo per entrambi.'
};

const ENGLISH = SCRIPTS.en ?? '';

/** Script in the given language (by base language), English otherwise. */
export function voiceScript(language: string): string {
	const base = language.split('-')[0]?.toLowerCase() ?? 'en';
	return SCRIPTS[base] ?? ENGLISH;
}
