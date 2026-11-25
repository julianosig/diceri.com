import { isCaptionForListener, type CaptionMessage } from './captions.ts';

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

	constructor(
		private readonly listenerLanguage: string,
		private readonly speakerLanguage: (speaker: string) => string | undefined
	) {}

	push(message: CaptionMessage): void {
		const speakerLanguage = this.speakerLanguage(message.speaker);
		if (!isCaptionForListener(message, this.listenerLanguage, speakerLanguage)) return;
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
