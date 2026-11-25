/**
 * What people read while recording their voice (~10 s). The first sentence is
 * the spoken consent required by docs/decisions/0006-voice-cloning-consent.md.
 */
const SCRIPTS: Partial<Record<string, string>> = {
	pt: 'Eu autorizo o Diceri a usar a minha voz para me traduzir nesta chamada. Olá! É um prazer te conhecer, e fico feliz que a gente possa conversar hoje, mesmo em idiomas diferentes.',
	en: "I agree to Diceri using my voice to translate me in this call. Hi! It's great to meet you, and I'm happy we can talk today, even in different languages.",
	es: 'Autorizo a Diceri a usar mi voz para traducirme en esta llamada. ¡Hola! Es un placer conocerte, y me alegra que podamos hablar hoy, aunque sea en idiomas diferentes.',
	fr: "J'autorise Diceri à utiliser ma voix pour me traduire pendant cet appel. Bonjour ! Ravi de te rencontrer, et content que nous puissions parler aujourd'hui, même dans des langues différentes.",
	de: 'Ich erlaube Diceri, meine Stimme zu nutzen, um mich in diesem Anruf zu übersetzen. Hallo! Schön, dich kennenzulernen, und ich freue mich, dass wir heute reden können, auch in verschiedenen Sprachen.',
	it: 'Autorizzo Diceri a usare la mia voce per tradurmi in questa chiamata. Ciao! È un piacere conoscerti, e sono felice che possiamo parlare oggi, anche in lingue diverse.'
};

const ENGLISH =
	"I agree to Diceri using my voice to translate me in this call. Hi! It's great to meet you, and I'm happy we can talk today, even in different languages.";

/** Script in the given language (by base language), English otherwise. */
export function voiceScript(language: string): string {
	const base = language.split('-')[0]?.toLowerCase() ?? 'en';
	return SCRIPTS[base] ?? ENGLISH;
}
