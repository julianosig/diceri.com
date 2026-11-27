# Inworld instant voice cloning, TTS-2 streaming and per-plan limits (November 2026)

Scope: technical facts for Diceri's flow (about 10 s of 16 kHz mono WAV recorded in the browser,
cloned at join, used for streaming TTS in many languages during the call, deleted when the call
ends). All sources were fetched on 2026-11-27. Every docs.inworld.ai page used here is public.
Nothing behind a login was consulted. The Inworld Portal UI and account-specific limits were not
checked.

## 1. Cloning API: endpoint, request, response, latency, readiness

### Takeaway
`POST https://api.inworld.ai/voices/v1/voices:clone` takes JSON with base64 WAV or MP3 samples. A
transcription is optional (TTS-2 ignores it) and so is the language (`AUTO` detection). The call
synchronously returns a `voice.voiceId` in the form `{workspace}__{voice}`. The docs say cloning
takes "seconds" and is zero-shot with no training step, but they publish no latency figure and no
readiness or polling contract.

### Cited Findings
- Endpoint `POST https://api.inworld.ai/voices/v1/voices:clone`. The workspace segment is optional
  (`/voices/v1/workspaces/{workspace}/voices:clone` still works). Auth is
  `Authorization: Basic $INWORLD_API_KEY` and needs write permission. A key without `rw` on
  `voices` gets error code `7`. — [Clone a voice API reference](https://docs.inworld.ai/api-reference/voiceAPI/voiceservice/clone-voice)
- Body fields:
  - `displayName` (required).
  - `voiceSamples[]` (required). Each sample has `audioData`, base64 "binary audio", which accepts
    WAV and MP3, plus an optional `transcription` that is "generated automatically" if omitted.
  - Optional fields: `languageCode` (e.g. `en-US`, `vi`) or the legacy `langCode` (e.g. `EN_US`,
    `AUTO`). Send at most one of the two; omitting both auto-detects. Values outside the catalog
    return `INVALID_ARGUMENT`.
  - Optional fields: `description`, `tags`, and
    `audioProcessingConfig.removeBackgroundNoise`, which runs an isolation model that "may degrade
    already-clean audio".

  — [Clone a voice API reference](https://docs.inworld.ai/api-reference/voiceAPI/voiceservice/clone-voice)
- Length guidance: "works best with a 10-30 sec audio clip; longer clips will be cut off at 30 sec".
  Avoid mixing languages, whispering, coughing and other non-verbal sounds. **No sample rate or bit
  depth is specified.** — [Clone a voice API reference](https://docs.inworld.ai/api-reference/voiceAPI/voiceservice/clone-voice)
- Minimum length: "from as little as 3 seconds of audio". Longer audio improves similarity up to
  30 s. Clones made before 2026-08-15 keep a 15 s prompt limit. Uploads accept wav, mp3 and webm up
  to 12 MB (that page describes Portal uploads; the API reference lists only WAV/MP3). — [Voice cloning guide](https://docs.inworld.ai/tts/voice-cloning); [Instant voice cloning](https://docs.inworld.ai/tts/instant-voice-cloning.md)
- Response:
  - `voice` holds `voiceId` (for IVC voices, `{workspace}__{voice}`), `name`
    (`workspaces/{w}/voices/{v}`), `languageCode` (immutable after creation), `langCode` and
    `source` (`IVC` for a clone).
  - `audioSamplesValidated[]` holds processed audio ("up to 15 seconds"; per the docs this does not
    mean TTS-2 truncated the sample), `transcription` (used only by the legacy
    `inworld-tts-1.5-*` models, "TTS-2 does not use it"), `warnings[]` and `errors[]`.

  — [Clone a voice API reference](https://docs.inworld.ai/api-reference/voiceAPI/voiceservice/clone-voice)
- Do not set `gender`, `ageGroup` or `categories` at creation (they are ignored). Set them later
  with UpdateVoice. — [Clone a voice API reference](https://docs.inworld.ai/api-reference/voiceAPI/voiceservice/clone-voice)
- Latency and readiness:
  - Instant cloning "creates a personalized voice clone in seconds" and is "also known as zero-shot
    cloning".
  - Only Professional Voice Cloning has "an asynchronous training step".
  - Portal offers "Try your cloned voice" "once voice cloning completes".

  — [Instant voice cloning](https://docs.inworld.ai/tts/instant-voice-cloning.md); [Voice cloning guide](https://docs.inworld.ai/tts/voice-cloning)
- Official example: posts to the same endpoint with `displayName`, `langCode` and `voiceSamples`
  (base64 of the raw file bytes), then reads `result.voice.voiceId`. The CLI language choices are
  `EN_US, ZH_CN, KO_KR, JA_JP, RU_RU, AUTO, IT_IT, ES_ES, PT_BR, DE_DE, FR_FR, AR_SA, PL_PL, NL_NL`.
  — [inworld-api-examples example_voice_clone.py](https://raw.githubusercontent.com/inworld-ai/inworld-api-examples/main/tts/python/example_voice_clone.py)
- Node SDK: `cloneVoice()` in `@inworld/tts`. — [Instant voice cloning](https://docs.inworld.ai/tts/instant-voice-cloning.md)
- Consent: Portal requires a "Confirm you have the rights to clone the voice" checkbox. No consent
  field exists in the API. — [Voice cloning guide](https://docs.inworld.ai/tts/voice-cloning)

### Inferences
- A 10 s clip sits at the bottom of the recommended 10-30 s window. It is usable (minimum 3 s), but
  speaker similarity should improve with a longer recording.
- 16 kHz mono WAV is probably accepted, because WAV is a listed format and no sample-rate minimum is
  documented. This is not confirmed; test it.
- Since the call is synchronous and returns a `voiceId`, the voice is most likely usable right
  away. Diceri should still guard the first TTS call with a retry, because no readiness guarantee is
  documented.
- Send `languageCode` set to the speaker's lobby language rather than `AUTO`. A short clip can be
  misdetected, and `languageCode` cannot be changed after creation.
- `removeBackgroundNoise` should stay off for clean browser recordings, given the documented risk of
  degradation.
- Diceri's consent flow (ADR 0006) must live in Diceri. Inworld's API records no consent.

### Gaps
- No published clone-creation latency (p50/p90) and no readiness or polling semantics for
  API-created voices.
- Sample-rate, bit-depth and channel requirements are not documented.
- Whether the API (not only Portal) accepts webm is unclear: the API reference says WAV and MP3, the
  guide says wav, mp3 and webm.

## 2. Inline (zero-shot) reference audio per TTS request

### Takeaway
No. The documented TTS requests accept only a stored `voiceId` or a text-based `voiceDesign`. There
is no field for inline reference audio, so a stored voice must be created and later deleted.

### Cited Findings
- HTTP streaming request: "Set either `voiceId` or `voiceDesign`, not both". `voiceDesign` takes a
  text `designPrompt` of 7 to 1,024 characters. No reference-audio field is documented. — [Synthesize speech (stream) API reference](https://docs.inworld.ai/api-reference/ttsAPI/texttospeech/synthesize-speech-stream)
- The WebSocket `create` message requires `voiceId` and `modelId`. — [WebSocket API reference](https://docs.inworld.ai/api-reference/ttsAPI/texttospeech/synthesize-speech-websocket)
- "Zero-shot" in Inworld's docs means that instant cloning has no training step, not that audio is
  sent per request. — [Instant voice cloning](https://docs.inworld.ai/tts/instant-voice-cloning.md)

### Inferences
- Every participant needs a create → use → delete lifecycle, and each clone counts against the
  plan's custom-voice cap while it exists (see section 3).

### Gaps
- No undocumented or beta inline-audio option was found. This was not confirmed with Inworld
  support.

## 3. Delete, list, capacity and workspace scoping

### Takeaway
`DELETE /voices/v1/voices/{workspace}__{voice}` removes a clone and immediately frees capacity.
`GET /voices/v1/voices?filter=source="IVC"` (or `owned="true"`) lists clones in the key's workspace.
The custom-voice cap is per organization (shared across workspaces) and is queryable for free.

### Cited Findings
- `DELETE https://api.inworld.ai/voices/v1/voices/{voiceId}`, with `voiceId` in the form
  `{workspace}__{voice}`. It needs write permission and returns `{}` on 200. The workspace comes
  from the API key when the path omits it. — [Delete voice API reference](https://docs.inworld.ai/api-reference/voiceAPI/voiceservice/delete-voice)
- "Deleting private voices frees room immediately". Voices shared to the Voice Library cannot be
  deleted until they are removed from it (2-year notice period). — [Voice cloning guide](https://docs.inworld.ai/tts/voice-cloning)
- `GET https://api.inworld.ai/voices/v1/voices`: "The API key determines the workspace".
  - Filters (AIP-160): `source="IVC"`, `owned="true"`, and `lang_code`.
  - Pagination: `pageSize` up to 2000 and `pageToken`. Pagination is offset-based with no stable
    snapshot, so deduplicate on `voiceId`.
  - `orderBy` accepts `created_at` and other fields.

  — [List voices API reference](https://docs.inworld.ai/api-reference/voiceAPI/voiceservice/list-voices)
- `GET https://api.inworld.ai/voices/v1/voices:customVoiceCapacity` is free, needs read access, and
  returns `usedCustomVoices` / `maxCustomVoices`. At the cap, create requests fail with
  `400 INVALID_ARGUMENT` ("You have exceeded your plan's maximum number of custom voices (500)...").
  Existing voices keep working. — [Instant voice cloning](https://docs.inworld.ai/tts/instant-voice-cloning.md); [Voice cloning guide](https://docs.inworld.ai/tts/voice-cloning)
- The custom-voice limit is "shared across all workspaces in your organization". Instant clones,
  designed voices and copy-token voices all count toward it. — [Voice cloning guide](https://docs.inworld.ai/tts/voice-cloning)

### Inferences
- A sweeper job (list `source="IVC"` ordered by `created_at` and delete stale ones) is cheap
  insurance against leaked clones after crashes. Pair it with a capacity check at join.
- The cap is on voices that exist at the same time, not on clones created. With prompt deletion,
  the cap bounds concurrent cloned participants (e.g. On-Demand 100, Creator 500).

### Gaps
- No documented rate limit for delete or list calls.

## 4. Cross-lingual use, accent behaviour and language tiers

### Takeaway
A clone from Portuguese can speak Japanese, English or Arabic. All four (plus Spanish) are Tier 1,
the best-evaluated tier. By default the model tries to speak the target language natively,
**without** carrying over the original accent. Speaker similarity is best in the clone's own
language.

### Cited Findings
- Tier 1 ("most extensively evaluated... highest, most consistent quality"), 15 languages: ar, zh,
  nl, en, fr, de, he, hi, it, ja, ko, pl, pt, ru, es.
- Tier 2 has 79 languages and is "fully supported... results can vary more", e.g. el, sw, th, tr,
  vi. For Tier 2 the docs recommend cloning or designing the voice in the target language.
- Tier 3 has 376 languages, is "not evaluated", gets no text normalization, and expects numbers to
  be written as words.

  — [Multilingual capability](https://docs.inworld.ai/tts/capabilities/multilingual)
- Default locales: pt → pt-BR, ja → ja-JP, en → en-US, ar → ar-SA, es → es-ES. Regional codes
  (`en-GB`, `es-MX`) steer the accent, and unaccepted codes return 400. — [Multilingual capability](https://docs.inworld.ai/tts/capabilities/multilingual)
- "A voice delivers the best speaker similarity when it speaks the language it was cloned in". For
  another language, the model "makes a best-effort attempt to speak the target language natively,
  without carrying over the accent of the voice's original language". Keeping the accent requires
  explicit steering (`inworld-tts-2` only). — [Multilingual capability](https://docs.inworld.ai/tts/capabilities/multilingual)
- The `language` field (BCP-47) selects the spoken language, text normalization and the localized
  prompt. When it is omitted, the language is auto-detected and short inputs such as bare digits can
  be misdetected. — [Multilingual capability](https://docs.inworld.ai/tts/capabilities/multilingual)
- The TTS-2 launch (2026-05-05) introduced "Production-quality synthesis across 15 languages" plus
  experimental support for 90+ more, and "cross-lingual voice synthesis". — [TTS release notes](https://docs.inworld.ai/release-notes/tts)
- Language counts conflict between sources: the models page says "200+ languages and locales"
  ([TTS models](https://docs.inworld.ai/tts/tts-models)), while the release notes and Cloudflare say
  "15 production + 90+ experimental"
  ([Cloudflare model page, third-party](https://developers.cloudflare.com/ai/models/inworld/tts-2/)).
  The tier page reconciles these: 94 languages in Tiers 1-2 and 376 in Tier 3.

### Inferences
- Diceri should always send `language` set to the listener's language on each TTS context. Short
  translated utterances are exactly the case where auto-detection fails.
- Diceri's UI could mark Tier 1 languages as recommended and Tier 2/3 as lower quality.

### Gaps
- No published speaker-similarity numbers for cross-lingual output.

## 5. TTS-2 and TTS-2 Flash streaming: endpoints, formats, latency, text streaming

### Takeaway
There are two streaming transports. HTTP NDJSON (`POST /tts/v1/voice:stream`, ≤ 4,000 UTF-16 units)
and a bidirectional WebSocket (`wss://api.inworld.ai/tts/v1/voice:streamBidirectional`) with many
contexts per connection, ≤ 2,000 units per message, and server-side sentence batching (`autoMode`).
Output can be headerless `PCM`/`LINEAR16` at 8-48 kHz. Server-side P90 time to first audio is
100 ms for `inworld-tts-2` and 20 ms for `inworld-tts-2-flash`, excluding network.

### Cited Findings
- HTTP streaming `POST https://api.inworld.ai/tts/v1/voice:stream`:
  - `text` takes up to 4,000 UTF-16 units; longer input returns code 3.
  - `modelId` is required.
  - Encodings: `LINEAR16` (a WAV header in *every* chunk), `PCM` (headerless), `WAV` (a header in
    the first chunk only), plus MP3 (the default), OGG_OPUS, ALAW, MULAW and FLAC.
  - `sampleRateHertz` ranges 8000-48000. The default is 48000 (8000 for μ-law/A-law), "capped at the
    model's native rate".
  - The response is NDJSON (not SSE) with base64 `result.audioContent`. "Errors can arrive
    mid-stream after an HTTP 200".

  — [Synthesize speech (stream) API reference](https://docs.inworld.ai/api-reference/ttsAPI/texttospeech/synthesize-speech-stream)
- WebSocket `wss://api.inworld.ai/tts/v1/voice:streamBidirectional`:
  - One connection carries many contexts, each with its own voice and settings.
  - Client messages: `create`, `send_text` (≤ 2000 UTF-16 units, optional `flush_context`),
    `flush_context`, `close_context`, `cancel_context`.
  - Server messages: `contextCreated`, `audioChunk`, `flushCompleted`, `contextClosed`.
  - The session closes "after 10 minutes of inactivity across any context".

  — [WebSocket API reference](https://docs.inworld.ai/api-reference/ttsAPI/texttospeech/synthesize-speech-websocket)
- Text streaming on the WebSocket:
  - `autoMode` "Enable[s] server-controlled flushing and batching of buffered text".
  - `autoModeStrategy` is `CLIENT_SEGMENTED` (complete sentences) or `SENTENCE_BOUNDARY`
    (incremental tokens; supported by `inworld-tts-2` and `-flash`).
  - `bufferCharThreshold` defaults to 1000 (range 0-2000).
  - `maxBufferDelayMs` applies only when autoMode is off.
  - `synthesisContext` keeps up to 5 previous requests, 2,000 characters combined.
  - Word or character timestamps are available with SYNC/ASYNC transport.

  — [WebSocket API reference](https://docs.inworld.ai/api-reference/ttsAPI/texttospeech/synthesize-speech-websocket)
- `deliveryMode` (STABLE / BALANCED (default) / CREATIVE) exists only on `inworld-tts-2`.
  `speakingRate` ranges 0.5-1.5. — [WebSocket API reference](https://docs.inworld.ai/api-reference/ttsAPI/texttospeech/synthesize-speech-websocket)
- Models: `inworld-tts-2` (100 ms P90 TTFB, instant and professional clones, steering) and
  `inworld-tts-2-flash` (20 ms P90 TTFB, "5× faster", instant clones, no steering). Both figures are
  "measured server-side, excluding network latency". — [TTS models](https://docs.inworld.ai/tts/tts-models); [TTS release notes, 2026-08-09](https://docs.inworld.ai/release-notes/tts)
- Other latency claims: the Inworld TTS API page says "<250ms P90" (per a search snippet; the page
  was not fetched). The launch press release says "under 100ms at the 99th percentile" under
  Inworld's test conditions ([BusinessWire](https://www.businesswire.com/news/home/20260902226008/en/),
  search snippet). A third-party Vapi benchmark measured a median of 288 ms including network
  ([Vapi humanness index](https://humannessindex.vapi.ai/models/inworld-tts-2), search snippet, not
  fetched).
- An OpenAI-compatible `POST /v1/audio/speech` was added on 2026-09-11 (pcm, wav, opus, SSE, ...).
  — [TTS release notes](https://docs.inworld.ai/release-notes/tts)

### Inferences
- For LiveKit, request `PCM` at 48000 Hz, or at the agent's track rate, to skip WAV-header
  stripping. Avoid `LINEAR16` on streaming, because every chunk carries a header.
- One WebSocket per call (or per agent worker) with one context per (speaker, listener language)
  fits the many-contexts model well. Use `cancel_context` for barge-in.
- Translated text arrives in sentence-sized pieces, so `CLIENT_SEGMENTED` plus an explicit flush per
  segment likely gives the lowest latency. `SENTENCE_BOUNDARY` suits token streams from an LLM.

### Gaps
- Native output sample rates of TTS-2 and TTS-2 Flash are not documented ("capped at the model's
  native rate").
- WebSocket authentication is not described on the WebSocket reference page. It is presumably the
  same Basic key; verify this.

## 6. Rate and concurrency limits per plan; errors

### Takeaway
The limit that binds is **concurrent generations**: On-Demand 5, Creator 10, Builder 50,
Developer 150, Growth 500, Enterprise custom. This is per account and shared across keys. WebSocket
connections are capped at 10× that. Requests over the limit are "best-effort", and rejections are
HTTP 429 or WebSocket `code: 8`. Clone creation has "lower rate limits" than TTS, but **no number is
published** in the public docs. Custom voices that exist at the same time are capped by plan
(100 / 500 / 3,000 / 10,000 / 30,000).

### Cited Findings
- Concurrent generations by plan: On-Demand 5, Creator 10, Builder 50, Developer 150, Growth 500,
  Enterprise custom. The limits apply to HTTP and WebSocket. "Requests above these limits are
  best-effort based on available capacity." — [Concurrency limits](https://docs.inworld.ai/resources/concurrency-limits)
- Counting:
  - Each in-flight HTTP request counts as one generation.
  - Each *active* WebSocket context counts as one; an idle context stops counting.
  - Closing a context frees its slot once all audio has been returned.
  - WebSocket connections are limited to 10× the generation limit (50 / 100 / 500 / 1,500 / 5,000).
  - "You can typically handle at least 4x your concurrent generation limit in simultaneous
    conversations".

  — [Concurrency limits](https://docs.inworld.ai/resources/concurrency-limits)
- Errors:
  - Over the generation limit, HTTP returns `429` and WebSocket returns an error with
    `code: 8 (ResourceExhausted)`; the connection and other contexts are unaffected.
  - The error fires on `create_context`, or on the first message that resumes an idle context.
    Messages sent since then may be dropped.
  - Over the connection limit, the response is HTTP `429`.

  — [Concurrency limits](https://docs.inworld.ai/resources/concurrency-limits)
- Limits are per account, shared across keys. Per-second limits return `429` with body "Rate limit
  exceeded" and no `Retry-After`; per-minute limits may include one. The page recommends exponential
  backoff with jitter (base 1 s, max 30 s, about 5 retries). TTS limit categories are "concurrent
  generations, WebSocket connections, voice design and cloning limits", with numbers deferred to the
  pricing page. — [Rate limits](https://docs.inworld.ai/resources/rate-limits)
- Pricing page:

  | Plan | Monthly price | Concurrent requests | Est. user sessions | Custom voices | PVC | TTS-2 / Flash per 1M chars |
  | --- | --- | --- | --- | --- | --- | --- |
  | On-Demand | $0 | 5 | 20 | 100 | 2 | $25 / $15 |
  | Creator | $25 | 10 | 40 | 500 | 4 | $20 / $10 |
  | Builder | $100 | 50 | 200 | 3,000 | 10 | $17.50 / $9 |
  | Developer | $300 | 150 | 600 | 10,000 | 25 | $15 / $8 |
  | Growth | $1,500 | 500 | 2,000 | 30,000 | 40 | $12.50 / $7 |
  | Enterprise | custom | custom | custom | custom | 100+ | ≤ $5 / sub-$5 |

  On-Demand needs a payment method on file before creating a PVC. The pricing page states **no**
  clone-creation rate. — [Inworld pricing](https://inworld.ai/pricing)
- "Voice cloning has lower rate limits than regular speech synthesis". No figure is given. — [Instant voice cloning](https://docs.inworld.ai/tts/instant-voice-cloning.md); [Voice cloning guide](https://docs.inworld.ai/tts/voice-cloning)
- **Unverified:** a search-engine snippet of Inworld's founder-pricing page showed "Voice cloning API
  requests/min 5" and "Voice design API requests/min 5". Fetching the page did not show these rows,
  and the plan they belong to is unclear. — [Inworld founder pricing](https://inworld.ai/founder-pricing)
- **Possibly outdated:** an older docs path showed "TTS 100 requests per second" as an
  all-account default (search snippet only). The current rate-limits page uses plan-based
  concurrency instead. — [Rate limits (old path)](https://docs.inworld.ai/docs/resources/rate-limits)
- Async/batch synthesis (preview) has separate job-concurrency limits and rejects rather than queues.
  It is not relevant to realtime use. — [Job concurrency limits](https://docs.inworld.ai/resources/job-concurrency-limits) (search snippet)

### Inferences
- Concurrency is counted per *active* context. A two-person call where each listener hears one
  translated stream uses at most about 2 generations while both speak. On-Demand (5) therefore
  handles roughly 2 simultaneous active calls before best-effort applies, and Creator (10) about 5.
- If the unverified 5 clones/min limit is real, joins would bottleneck at 5 participants per minute
  per account. Diceri should queue or retry clone requests on 429 and fall back to a stock voice
  (already the no-consent path under ADR 0006) instead of blocking the join.

### Gaps
- Official per-plan clone-creation and voice-design request rates are not published in public docs.
  Confirm them with Inworld support or in the Portal (behind login).
- No documented rate limits for list, delete or capacity calls.
- No documented error code for exceeding the clone *rate* (presumably 429, but unconfirmed).
