/** Error thrown for non-2xx API responses (RFC 9457 problem details). */
export class ApiError extends Error {
	constructor(
		readonly status: number,
		readonly title: string,
		readonly detail?: string
	) {
		super(detail ?? `${String(status)} ${title}`);
		this.name = 'ApiError';
	}
}

type FetchFn = typeof fetch;

export interface RequestOptions {
	method?: 'GET' | 'POST';
	/** Sent as JSON. */
	body?: unknown;
	/** Sent as-is with its own content type (e.g. a recording); excludes `body`. */
	blob?: Blob;
	/** Host key or guest request key, sent as `x-diceri-key`. */
	accessKey?: string;
	fetchFn?: FetchFn;
}

async function send(path: string, options: RequestOptions): Promise<Response> {
	const headers = new Headers({ accept: 'application/json' });
	if (options.accessKey) headers.set('x-diceri-key', options.accessKey);
	if (options.body !== undefined) headers.set('content-type', 'application/json');
	if (options.blob) headers.set('content-type', options.blob.type || 'application/octet-stream');
	const fetchFn = options.fetchFn ?? fetch;
	const body =
		options.blob ?? (options.body === undefined ? undefined : JSON.stringify(options.body));
	const response = await fetchFn(`/api/v1${path}`, {
		method: options.method ?? 'GET',
		headers,
		body
	});
	if (!response.ok) {
		const problem = (await response.json().catch(() => ({}))) as {
			title?: string;
			detail?: string;
		};
		throw new ApiError(response.status, problem.title ?? response.statusText, problem.detail);
	}
	return response;
}

/**
 * Sends a request to the Rust API and parses the JSON body. Requests are
 * same-origin: the Vite server proxies `/api`, production serves both.
 */
export async function requestJson<T>(path: string, options: RequestOptions = {}): Promise<T> {
	return (await (await send(path, options)).json()) as T;
}

/** Like `requestJson` for endpoints that answer 204 No Content. */
export async function requestVoid(path: string, options: RequestOptions = {}): Promise<void> {
	await send(path, options);
}
