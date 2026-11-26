/** One line of the home banner: said in one language, heard in another. */
export interface SpokenLine {
	/** Language name in that language, shown on the bubble. */
	language: string;
	text: string;
	/** BCP 47 tag, for the `lang` attribute (correct fonts and screen readers). */
	lang: string;
}

export interface LanguagePair {
	said: SpokenLine;
	heard: SpokenLine;
}

/** The banner cycles through these: the same sentence, translated, in many pairs. */
export const LANGUAGE_PAIRS: readonly LanguagePair[] = [
	{
		said: { language: 'Português', text: 'Olá! Tudo bem?', lang: 'pt-BR' },
		heard: { language: '日本語', text: 'こんにちは！元気？', lang: 'ja' }
	},
	{
		said: { language: 'English', text: 'Nice to meet you!', lang: 'en' },
		heard: { language: 'Español', text: '¡Mucho gusto!', lang: 'es' }
	},
	{
		said: { language: 'Français', text: 'On commence ?', lang: 'fr' },
		heard: { language: '한국어', text: '시작할까요?', lang: 'ko' }
	},
	{
		said: { language: 'Deutsch', text: 'Hallo zusammen!', lang: 'de' },
		heard: { language: 'Italiano', text: 'Ciao a tutti!', lang: 'it' }
	},
	{
		said: { language: 'Español', text: '¿Me escuchas bien?', lang: 'es' },
		heard: { language: 'English', text: 'Can you hear me?', lang: 'en' }
	},
	{
		said: { language: '中文', text: '你好！', lang: 'zh' },
		heard: { language: 'Português', text: 'Olá!', lang: 'pt-BR' }
	}
];

/** One full say → translate → hear cycle, in milliseconds (matches the CSS timeline). */
export const CYCLE_MS = 6000;

export function nextPair(index: number): number {
	return (index + 1) % LANGUAGE_PAIRS.length;
}
