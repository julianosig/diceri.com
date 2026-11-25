import type { CaptionMessage } from './captions.ts';
import { applyCaption, formatTranscript, type TranscriptEntry } from './transcript.ts';

/**
 * Everything said while this participant was in the call, originals and
 * translations, kept only in this browser (never stored on a server).
 */
export class TranscriptLog {
	entries = $state<TranscriptEntry[]>([]);

	record(message: CaptionMessage, speakerName: string): void {
		this.entries = applyCaption(this.entries, message, speakerName, Date.now());
	}

	toText(): string {
		return formatTranscript(this.entries);
	}
}
