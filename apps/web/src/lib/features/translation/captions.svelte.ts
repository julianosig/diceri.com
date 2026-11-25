import { isCaptionForListener, type CaptionMessage } from './captions.ts';
import type { Listener } from './translation-routing.ts';

export interface CaptionLine {
	speaker: string;
	phrase: number;
	text: string;
	final: boolean;
}

const MAX_LINES = 3;

/** The latest captions a listener should see, one line per speaker phrase. */
export class CaptionFeed {
	lines = $state<CaptionLine[]>([]);
	/** Language last detected per speaker, learned from their final captions. */
	#detected: Record<string, string> = {};

	constructor(
		private readonly listener: () => Listener,
		private readonly onDetected: (speaker: string, language: string) => void
	) {}

	detectedLanguage(speaker: string): string | undefined {
		return this.#detected[speaker];
	}

	push(message: CaptionMessage): void {
		if (message.kind === 'original' && this.#detected[message.speaker] !== message.language) {
			this.#detected[message.speaker] = message.language;
			this.onDetected(message.speaker, message.language);
		}
		const speakerLanguage = this.#detected[message.speaker];
		if (!isCaptionForListener(message, this.listener(), speakerLanguage)) return;
		const line: CaptionLine = {
			speaker: message.speaker,
			phrase: message.phrase,
			text: message.text,
			final: message.kind !== 'partial'
		};
		const rest = this.lines.filter(
			(l) => !(l.speaker === message.speaker && l.phrase === message.phrase)
		);
		this.lines = [...rest, line].slice(-MAX_LINES);
	}
}
