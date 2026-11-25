import { decideRequest, pendingRequests, type PendingRequest } from './lobby-api.ts';

const POLL_MS = 2000;

/** The host's view of guests waiting to join, refreshed by polling. */
export class HostRequests {
	pending = $state<PendingRequest[]>([]);
	/** Requests the host closed the toast for (still listed in People). */
	dismissed = $state<string[]>([]);
	error = $state<string | undefined>();
	#timer: ReturnType<typeof setInterval> | undefined;

	constructor(
		private readonly roomId: string,
		private readonly hostKey: string
	) {}

	/** Newest request not dismissed, shown as a toast. */
	get toast(): PendingRequest | undefined {
		return this.pending.findLast((request) => !this.dismissed.includes(request.id));
	}

	start(): void {
		this.stop();
		void this.refresh();
		this.#timer = setInterval(() => {
			void this.refresh();
		}, POLL_MS);
	}

	stop(): void {
		clearInterval(this.#timer);
	}

	async refresh(): Promise<void> {
		try {
			this.pending = await pendingRequests(this.roomId, this.hostKey);
			this.error = undefined;
		} catch (cause) {
			this.error = cause instanceof Error ? cause.message : String(cause);
		}
	}

	async decide(requestId: string, admit: boolean): Promise<void> {
		this.pending = this.pending.filter((request) => request.id !== requestId);
		await decideRequest(this.roomId, this.hostKey, requestId, admit);
		await this.refresh();
	}

	dismiss(requestId: string): void {
		this.dismissed = [...this.dismissed, requestId];
	}
}
