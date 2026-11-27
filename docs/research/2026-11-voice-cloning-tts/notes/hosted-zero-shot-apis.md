# Hosted voice-cloning TTS APIs, group A: Mistral Voxtral, Inworld, Cartesia Sonic, Gradium, Hume Octave (as of 2026-11-27)

Scope: hosted APIs that can speak in a caller's voice from a short reference recording, for Diceri (live
voice translation, ~57 target languages, streaming, low time-to-first-audio), as a replacement for
ElevenLabs (Creator plan: ~$50/M chars, 95 voice creations/month, 30 stored voices).

**Headline status:** Hume's TTS (Octave) API was shut down on 2026-11-13 and is no longer an option.
Mistral's original Voxtral TTS (26.03) is deprecated since 2026-10-31 in favour of Voxtral Mini TTS 3.
Inworld TTS-1.5 is deprecated in favour of Realtime TTS-2 / TTS-2 Flash.

## Cloning model: per-request zero-shot vs stored voice; voice limits; rate and concurrency limits

### Takeaway
Only **Mistral Voxtral** officially documents true per-request zero-shot cloning (`ref_audio`, "the voice
isn't saved"), which removes any voice-creation/storage quota from the design. Inworld, Cartesia and Gradium
all require creating a stored voice first, but their quotas are far larger than ElevenLabs Creator's (Inworld
100-30,000 custom voices by plan; Gradium 3-1,000; Cartesia no published instant-clone cap), and deleting a
voice frees the slot (documented for Gradium). Concurrency is the tighter constraint at low tiers
(Cartesia 3-15 TTS streams, Gradium 2-20, Inworld 5-500 requests).

### Cited Findings
**Mistral Voxtral (Voxtral Mini TTS 3, `voxtral-mini-tts-3-0`; realtime variant `voxtral-mini-tts-realtime-3-0`)**
- One-off cloning: send a base64-encoded MP3 or WAV as `ref_audio`; "The voice isn't saved." A URL isn't accepted. — [Mistral docs: Voice cloning](https://docs.mistral.ai/studio/audio/text-to-speech/voice-cloning)
- Saved voices are optional: create a voice from the recording and pass `voice_id`; saved voices carry a `retention_notice` field (default 30 days, min 1, max 365). No limit on number of saved voices is stated. — [Mistral docs: Voice cloning](https://docs.mistral.ai/studio/audio/text-to-speech/voice-cloning)
- `ref_audio` replaces `voice_id` per request; `conversation_history` (contextual TTS) needs a Voxtral 3 preset voice and cannot be combined with `ref_audio`. — [Mistral docs: Speech generation](https://docs.mistral.ai/studio/audio/text-to-speech/generate-speech)
- Reference length: Mistral docs say as little as 2-3 s; the Voxtral TTS paper says the encoder accepts 3-30 s references and works best on 3-25 s. No official maximum for `ref_audio` found in Voxtral 3 docs. — [Mistral TTS overview (older docs)](https://docs.mistral.ai/capabilities/audio/text_to_speech); [arXiv 2603.25551](https://arxiv.org/html/2603.25551v2)
- Professional cloning ("Pro Voice") recommends 30 min to 3 h of clean audio. — [Mistral docs: Voice cloning](https://docs.mistral.ai/studio/audio/text-to-speech/voice-cloning)
- The original Voxtral TTS (v26.03) is "deprecated since October 31, 2026"; Voxtral Mini TTS 3 recommended for new integrations. — [Mistral model card voxtral-tts-26-03](https://docs.mistral.ai/models/voxtral-tts-26-03)
- Voxtral Mini TTS 3 released 2026-10-20 (secondary sources; some describe it as public preview). — [Voxtral 3 explainers, e.g. digitalapplied](https://www.digitalapplied.com/blog/voxtral-3-transcription-tts-models-price-preview)
- Rate limits / concurrency for TTS: not found in public docs.

**Inworld (Realtime TTS-2 `inworld-tts-2`, Realtime TTS-2 Flash `inworld-tts-2-flash`)**
- Instant cloning uses 5-15 s of audio; Professional Voice Cloning (preview) needs 10+ min. — [Inworld: cross-lingual voice cloning (2026-05-28, checked 2026-09-02)](https://inworld.ai/resources/cross-lingual-voice-cloning)
- API flow is two-step: `POST /voices/v1/voices:clone` with reference audio returns a `voiceId`, then use it in TTS (search-snippet of Inworld docs; the docs page itself is behind a login and could not be fetched). — [Inworld docs (login-gated)](https://docs.inworld.ai/tts/capabilities/voice-cloning)
- Custom voices per plan: On-Demand 100, Creator 500, Builder 3,000, Developer 10,000, Growth 30,000, Enterprise custom. Professional clones: 2/4/10/25/40/100+. No per-month creation cap and no per-clone fee stated. — [Inworld pricing](https://inworld.ai/pricing)
- Concurrent requests: On-Demand 5, Creator 10, Builder 50, Developer 150, Growth 500 (est. concurrent user sessions "typically at least 4x" that: 20/40/200/600/2,000). — [Inworld pricing](https://inworld.ai/pricing)
- Instant cloning available on both TTS-2 and TTS-2 Flash; Professional Voice Cloning and steering on TTS-2 only. — [Inworld TTS models](https://docs.inworld.ai/tts/tts-models); [Inworld TTS release notes](https://docs.inworld.ai/release-notes/tts.md)
- No per-request zero-shot (reference audio inline) option found.

**Cartesia (Sonic 3.6 / 3.5 / 3)**
- Instant cloning: 10 s is enough to start; up to 60 s helps retain accent (60 s limit applies to Sonic 3.6+, older models use the first 10 s); file ≤ 16 MB; created via dashboard or Clone Voice API with clip, name and language. — [Cartesia docs: Clone voices](https://docs.cartesia.ai/build-with-cartesia/capability-guides/clone-voices)
- TTS WebSocket voice specifier accepts only a voice `id`; "Embeddings: not accepted in this API version" (i.e. no inline per-request reference). — [Cartesia API reference: TTS WebSocket](https://docs.cartesia.ai/api-reference/tts/websocket)
- Instant cloning available from Pro plan up, no slot limit listed; Professional clones: Startup 2, Scale 4. Clone training free. — [Cartesia pricing](https://cartesia.ai/pricing)
- TTS concurrency: Free 2, Pro 3, Startup 5, Scale 15, Enterprise custom (separate pools per product). — [Cartesia pricing](https://cartesia.ai/pricing)

**Gradium**
- Clones are created by uploading audio (`POST /api/voices/`, ≥ 10 s recommended) and used as `voice_id`; per-request reference audio not documented. New clones are "enhanced" by default (`enhance=false` to opt out). — [Gradium docs: Custom voices](https://docs.gradium.ai/guides/voices/custom-voices.md)
- Custom voices (clones + Voice Design share allowance): Free 3, XS 10, S 25, M 100, L 1,000, Enterprise unlimited; Pro clones M 5, L 20. Deleting a voice frees its slot immediately (unless downloaded). Exceeding returns "409 Custom voice limit reached". — [Gradium docs: Limits](https://docs.gradium.ai/guides/limits.md)
- TTS concurrency (simultaneous sessions): Free 2, XS 5, S 10, M 15, L 20. Session duration 3,000 s (L: 7,200 s). Request-rate limits not stated. — [Gradium docs: Limits](https://docs.gradium.ai/guides/limits.md)

**Hume Octave 2**
- Hume changelog (quoted by third parties): "Hume is sunsetting the TTS and EVI APIs. Access ends November 13, 2026 at 12:01 a.m. EST"; account data permanently deleted after that date. Hume's developer docs now only cover Audio Decisions / Expression Measurement. — [Versely (third party, quoting Hume)](https://www.versely.studio/blog/hume-octave-2-emotional-tts-2026); [Developers Digest](https://www.developersdigest.tech/voice-agents/hume-evi); [dev.hume.ai overview now shows Audio Decisions](https://dev.hume.ai/voice/docs/text-to-speech-tts/overview)
- Historical (pre-shutdown): cloning from as little as 15 s of audio. — [Hume TTS FAQ (cached)](https://dev.hume.ai/voice/docs/text-to-speech-tts/faq)

### Inferences
- Voxtral's non-stored `ref_audio` maps directly to Diceri's ADR 0006 consent rule ("clones deleted when the call ends"): nothing to delete, no quota. It costs bandwidth/latency per request (base64 clip each call) unless a saved voice with `retention_notice` = 1 day is used.
- For Gradium and Inworld, a create-on-join / delete-on-leave pattern keeps stored voices well under quota; Gradium explicitly frees slots on delete. Inworld's 100 custom voices even on the free On-Demand tier is ~3x ElevenLabs Creator's 30.
- Hume should be dropped from the shortlist.

### Gaps
- Mistral TTS rate/concurrency limits and maximum `ref_audio` length for Voxtral 3: not published in pages fetched.
- Inworld voice-cloning docs are login-gated; could not confirm whether inline reference audio is possible or whether there are clone-creation rate limits.
- Cartesia instant-clone count limits per plan not published (pricing page lists none).

## Price per million characters with a cloned voice; per-clone charges

### Takeaway
All four live providers are cheaper than ElevenLabs' ~$50/M: Voxtral Mini TTS 3 $13/M flat; Inworld TTS-2
Flash $7-15/M and TTS-2 $12.50-25/M by plan; Gradium $36-50/M; Cartesia ~$37-50/M on plans ($38-65/M
overage). None charges extra for instant clones; Gradium charges for Pro clones (1M credits + 1.2 credits/char).

### Cited Findings
- **Mistral Voxtral Mini TTS 3:** $13 per 1M characters. — [Mistral: Voxtral 3 announcement](https://mistral.ai/news/voxtral-3/)
- Old Voxtral TTS (26.03, deprecated): $0.016/1k chars = $16/M. — [Mistral: Voxtral TTS announcement](https://mistral.ai/news/voxtral-tts/); [OpenRouter listing](https://openrouter.ai/mistralai/voxtral-mini-tts-2603)
- No separate per-clone fee for Mistral found (none mentioned in docs fetched).
- **Inworld per 1M chars (TTS-2 / TTS-2 Flash):** On-Demand $0/mo: $25 / $15; Creator $25/mo: $20 / $10; Builder $100/mo: $17.50 / $9; Developer $300/mo: $15 / $8; Growth $1,500/mo: $12.50 / $7; Enterprise "as low as $5" / "sub-$5". Overage at the plan rate; no per-clone charge stated. — [Inworld pricing](https://inworld.ai/pricing)
- **Cartesia:** 1 credit per character; Pro $5/mo 100K credits; Startup $49/mo 1.25M; Scale $299/mo 8M; overage $65 / $45 / $38 per 1M credits (Pro/Startup/Scale). Clone training free; clone speech costs 1 credit/char. Free plan: no commercial use, no instant cloning. — [Cartesia pricing](https://cartesia.ai/pricing)
  - Derived: Startup ≈ $39.2/M, Scale ≈ $37.4/M included; Artificial Analysis lists Sonic at $49/M. — [Artificial Analysis TTS leaderboard](https://artificialanalysis.ai/text-to-speech/leaderboard)
- **Gradium:** 1 credit per TTS char; per 1M credits (monthly billing) XS $50, S $39, M $37, L $36; plans XS $8-9/mo (180k credits), S $33 (1M), M $249 (8M), L $667 (22M) — page shows internal inconsistencies between monthly and yearly figures. Overage per 100k credits $5.50 / $4.20 / $3.90 / $3.64. Pro clone: one-time 1M credits per clone, then 1.2 credits/char. Free tier: no commercial use. — [Gradium pricing](https://gradium.ai/pricing)
- **Hume Octave 2 (historical):** Artificial Analysis lists $87.50/M; Hume never published a flat rate ("defined by your subscription tier"). Now moot. — [Artificial Analysis](https://artificialanalysis.ai/text-to-speech/leaderboard); [Versely](https://www.versely.studio/blog/hume-octave-2-emotional-tts-2026)

### Inferences
- At Diceri volumes in the low millions of chars/month, Inworld TTS-2 Flash ($8-10/M) and Voxtral 3 ($13/M) are ~4-6x cheaper than ElevenLabs; Cartesia and Gradium are roughly ElevenLabs-priced but without its voice caps.

### Gaps
- Mistral volume discounts / enterprise pricing not found.

## Languages and cross-lingual cloning

### Takeaway
Against a ~57-language target: Inworld claims 200+ languages/locales (but only 15 at "production quality",
90+ experimental at TTS-2 launch); Cartesia Sonic 3.6 covers 44; Voxtral Mini TTS 3 35+ languages; Gradium
only 5 (en, fr, de, es, pt). All except Cartesia's per-language clone design state cross-lingual voice reuse;
Cartesia requires "Add Voice Accents" to make a clone speak other languages natively. No provider publishes
cross-lingual speaker-similarity numbers for a pt→ja-style pair.

### Cited Findings
- **Voxtral Mini TTS 3:** "35+ languages and 50+ dialects"; preset voices in 18 languages; page names 21 languages explicitly. — [Mistral: Voxtral 3](https://mistral.ai/news/voxtral-3/); [Mistral on X](https://x.com/MistralAI/status/2112574621182775528)
- Voxtral cloning: "A cloned voice isn't limited to the language of its recording." — [Mistral docs: Voice cloning](https://docs.mistral.ai/studio/audio/text-to-speech/voice-cloning)
- Old Voxtral TTS: 9 languages (en, fr, es, pt, it, nl, de, hi, ar), "cross-lingual voice cloning and code-mixing". — [Mistral TTS docs (deprecated section)](https://docs.mistral.ai/capabilities/audio/text_to_speech)
- **Inworld:** TTS-2 and TTS-2 Flash support "200+ languages and locales" (same voices across both). — [Inworld TTS models](https://docs.inworld.ai/tts/tts-models)
- At TTS-2 launch (2026-05-05): "production-quality synthesis in 15 languages with experimental support for 90+ more, cross-lingual voice reuse, voice localization". — [Inworld TTS release notes](https://docs.inworld.ai/release-notes/tts.md)
- Inworld itself cautions: "The coverage number is not a quality guarantee"; each launch language needs native-speaker review; one cloned voice can be localized across 200+ languages with identity "recognizable". — [Inworld: cross-lingual voice cloning](https://inworld.ai/resources/cross-lingual-voice-cloning)
- **Cartesia Sonic 3.6:** 44 languages (Odia and Urdu new in 3.6); WebSocket `language` enum covers ~50 base languages plus regional variants. — [Cartesia models](https://docs.cartesia.ai/build-with-cartesia/tts-models/latest); [Cartesia TTS WebSocket](https://docs.cartesia.ai/api-reference/tts/websocket)
- Cartesia cross-lingual: "Each clone is built from one language. To make it speak other languages natively, clone it in its original language first, then add languages with the Add Voice Accents API." — [Cartesia docs: Clone voices](https://docs.cartesia.ai/build-with-cartesia/capability-guides/clone-voices)
- **Gradium:** TTS languages en, fr, de, es, pt; clone language must be one of these and match the recording. — [Gradium llm-info](https://gradium.ai/llm-info); [Gradium docs: Custom voices](https://docs.gradium.ai/guides/voices/custom-voices.md)
- **Hume Octave 2 (historical):** 11 languages (en, ja, ko, es, fr, pt, it, de, ru, hi, ar). — [Hume TTS FAQ (cached)](https://dev.hume.ai/voice/docs/text-to-speech-tts/faq)

### Inferences
- None of these alone credibly covers 57 languages at production quality. Inworld has the widest nominal coverage; Cartesia (44) and Voxtral 3 (35+) need a fallback for the remainder. Gradium is unsuitable as the main provider (5 languages).
- Cross-lingual cloning quality must be measured in-house (e.g. pt source → ja/en target, speaker-similarity + native listener review).

### Gaps
- Full per-language lists for Voxtral 3 and Inworld GA vs experimental were not retrieved.
- No published cross-lingual speaker-similarity metrics for any of the providers.

## Streaming, output formats, time-to-first-audio

### Takeaway
All live providers stream. Cartesia and Gradium offer WebSocket streaming with raw PCM up to 48 kHz;
Inworld has HTTP streaming including an OpenAI-compatible SSE endpoint (WebSocket not confirmed in fetched
pages); Mistral has `stream: true` and a realtime model variant. Vendor TTFA claims: Inworld Flash 20 ms P90
server-side, Gradium ~50 ms, Cartesia sub-90 ms, Inworld TTS-2 100 ms P90, Voxtral ~0.8 s end-to-end (old
model, pcm).

### Cited Findings
- **Mistral:** `stream: true` returns audio chunks; formats mp3 (default), wav, pcm, flac, opus; sample rate not stated. — [Mistral docs: Speech generation](https://docs.mistral.ai/studio/audio/text-to-speech/generate-speech)
- Old Voxtral TTS: ~90 ms model processing; end-to-end TTFA via API ~0.8 s for pcm, ~3 s for mp3. — [Mistral TTS docs](https://docs.mistral.ai/capabilities/audio/text_to_speech)
- Voxtral codec is 24 kHz mono; paper: 70 ms latency at concurrency 1, 552 ms at 32 on one H200 (self-hosted, 500-char input). — [arXiv 2603.25551](https://arxiv.org/html/2603.25551v2)
- Third-party integration notes Mistral `pcm` is float32 little-endian (not s16) — unofficial. — [mozilla-ai/any-llm PR](https://github.com/mozilla-ai/any-llm/pull/1573)
- **Inworld:** TTS-2 Flash P90 TTFA 20 ms server-side (excl. network), ~5x faster than TTS-2 (100 ms P90). — [Inworld TTS models](https://docs.inworld.ai/tts/tts-models); [release notes](https://docs.inworld.ai/release-notes/tts.md)
- Inworld OpenAI-compatible `POST /v1/audio/speech` (2026-09-11): mp3, opus, flac, wav, pcm; SSE streaming via `stream_format: "sse"`. — [Inworld release notes](https://docs.inworld.ai/release-notes/tts.md)
- **Cartesia:** WebSocket streaming with `context_id`, continuations, `max_buffer_delay_ms` (0-5000, default 3000), flush/cancel; raw `pcm_f32le`, `pcm_s16le`, `pcm_mulaw`, `pcm_alaw` at 8/16/22.05/24/44.1/48 kHz. — [Cartesia TTS WebSocket](https://docs.cartesia.ai/api-reference/tts/websocket)
- Cartesia vendor claim "sub-90ms" TTS latency (model-side); Vapi measured Sonic 3.5 median first audio 128 ms incl. network; a competitor (Orcarouter) measured Sonic 3.6 median TTFB 143 ms. — [Cartesia Sonic 3.6 blog](https://cartesia.ai/blog/sonic-3.6/); [Vapi Humanness Index Sonic 3.5](https://humannessindex.vapi.ai/models/cartesia-sonic-3-5); [Orcarouter (competitor)](https://www.orcarouter.ai/blog/assemblyai-tts-1-vs-cartesia-sonic-3-6)
- **Gradium:** WebSocket (`tts_realtime`) and REST; WAV/raw PCM 48 kHz s16 mono (80 ms chunks), Ogg Opus, PCM at 8/16/22.05/24/44.1/48 kHz, μ-law/A-law 8 kHz. — [Gradium docs: Limits](https://docs.gradium.ai/guides/limits.md); [Gradium llms.txt](https://docs.gradium.ai/llms.txt)
- Gradium TTFA ~50 ms (vendor, 2026-10-01); 53 ms P50 on Coval TTS leaderboard (as read by Gradium, 2026-11-19). — [Gradium llm-info](https://gradium.ai/llm-info)

### Inferences
- Cartesia and Gradium fit a LiveKit agent pipeline most directly (WebSocket, s16 PCM at 24/48 kHz). Voxtral's per-request base64 reference adds upload size to every request; measured TTFA with `ref_audio` should be benchmarked.

### Gaps
- Voxtral Mini TTS 3 TTFA and sample rate not officially published; Inworld WebSocket TTS endpoint and PCM sample rates not confirmed in fetched pages.
- No independent TTFA measurement with a *cloned* voice for any provider.

## Commercial use, consent policy, data retention, hosting region

### Takeaway
All paid tiers allow commercial use (Cartesia and Gradium free tiers do not). Gradium is EU-based (Paris) with
EU/US data residency and zero data retention on paid plans; Mistral offers EU/US/global endpoints and
customer-controlled retention, and its non-stored `ref_audio` is the cleanest for ADR 0006. Consent: Mistral's
terms forbid cloning without explicit consent; Inworld recommends scoped consent; no explicit Cartesia/Gradium
consent clause was retrieved.

### Cited Findings
- **Mistral:** available via EU, US and global API endpoints; customers can control whether audio is retained. — [Mistral: Voxtral 3](https://mistral.ai/news/voxtral-3/)
- Mistral usage terms (per search snippet of docs): not authorized to "impersonate others, clone voices without explicit consent". — [Mistral TTS docs (search result)](https://docs.mistral.ai/studio-api/audio/text_to_speech)
- Old Voxtral open weights are CC BY-NC 4.0 (self-hosting non-commercial). — [Mistral model card](https://docs.mistral.ai/models/voxtral-tts-26-03)
- **Inworld:** TTS-2 and Flash both list zero data retention. — [Inworld TTS models](https://docs.inworld.ai/tts/tts-models)
- Inworld guidance: consent should cover languages, markets, channels, use cases, duration, storage, sublicensing, revocation; approval for one language doesn't extend to others. — [Inworld: cross-lingual voice cloning](https://inworld.ai/resources/cross-lingual-voice-cloning)
- **Cartesia:** commercial license from Pro up; Free not. — [Cartesia pricing](https://cartesia.ai/pricing)
- **Gradium:** commercial use on XS and above (not Free); zero data retention on XS+; EU or US data residency on S+; SOC 2 Type 2, ISO 27001:2022, HDS v2; founded Sept 2025 by Kyutai researchers, Paris + SF. — [Gradium pricing](https://gradium.ai/pricing); [Gradium llm-info](https://gradium.ai/llm-info); [Gradium docs: Data residency](https://docs.gradium.ai/guides/data-residency.md)
- **Hume:** account data permanently deleted after 2026-11-13. — [Versely](https://www.versely.studio/blog/hume-octave-2-emotional-tts-2026)

### Inferences
- For LGPD/GDPR, Gradium (EU residency + ZDR) and Mistral (EU endpoint, non-stored clone) are the easiest to document in `docs/data-inventory.md`.

### Gaps
- Cartesia and Gradium voice-cloning consent clauses and reference-audio retention periods not retrieved; Cartesia hosting region not found.
- Inworld hosting region not found.

## Published quality metrics

### Takeaway
On the Artificial Analysis TTS arena (Nov 2026 snapshot, undated page), Cartesia Sonic 3.6 ranks #5 (Elo 1297),
Inworld TTS-2 #10 (1272), Voxtral 3 #20 (1211), Inworld Flash #19 (1216), Gradium (Oct 2026) #28 (1174), Hume
Octave 2 #68 (1060); ElevenLabs v4 Turbo is #1 (1343) and Flash v2.5 #58 (1075). Arena Elo measures preset-voice
naturalness, not cloning similarity. Only Mistral publishes speaker-similarity numbers (vendor paper).

### Cited Findings
- Artificial Analysis leaderboard: Cartesia Sonic 3.6 #5 Elo 1297 ($49/M); Inworld Realtime TTS-2 #10 1272 ($20.83/M); Inworld TTS-2 Flash #19 1216 ($10.42/M); Mistral Voxtral 3 TTS #20 1211 ($13/M); Gradium TTS (Oct 2026) #28 1174 ($47.78/M); Voxtral TTS (old) #51 1094; Hume Octave 2 #68 1060; ElevenLabs v4 Turbo #1 1343 ($40/M), Eleven v4 #2 1340, Flash v2.5 #58 1075 ($50/M). — [Artificial Analysis TTS leaderboard](https://artificialanalysis.ai/text-to-speech/leaderboard)
- Voxtral TTS (old 26.03) paper, vendor-reported: speaker similarity (ECAPA-TDNN cosine) e.g. pt 0.785 vs ElevenLabs v3 0.571 / Flash v2.5 0.642; en 0.786 vs 0.484/0.489; SEED-TTS 0.628 vs 0.392/0.413. WER pt 1.02%, en 0.63%. Human preference for zero-shot cloning vs ElevenLabs Flash v2.5: 68.4% win; per-language win rate pt 74.4%, es 87.8%, nl 49.4%. — [arXiv 2603.25551](https://arxiv.org/html/2603.25551v2)
- Voxtral 3 announcement reports only internal evals judged by Gemini; no speaker similarity / WER. — [Mistral: Voxtral 3](https://mistral.ai/news/voxtral-3/)
- Gradium (vendor): 1.7% avg WER on Coval leaderboard (30-day window, 2026-11-19); 1.11% avg WER on MiniMax Multilingual test set across 5 languages (April 2026). — [Gradium llm-info](https://gradium.ai/llm-info)
- Inworld TTS 1.5 claimed 40% WER reduction vs TTS-1 (no absolute numbers). — [Inworld release notes](https://docs.inworld.ai/release-notes/tts.md)
- Third-party claim that Sonic 3.6 led both Artificial Analysis speech arenas at launch (Aug 2026) — since overtaken by ElevenLabs v4 per current leaderboard. — [MarkTechPost](https://www.marktechpost.com/2026/08/18/cartesia-ships-sonic-3-6-a-streaming-tts-model-that-now-leads-both-artificial-analysis-speech-arenas/)

### Inferences
- Quality ranking among live options: Cartesia > Inworld TTS-2 > Voxtral 3 ≈ Inworld Flash > Gradium, but this is preset-voice naturalness; cloning fidelity and cross-lingual similarity need in-house tests.

### Gaps
- No independent cloning-similarity benchmark covering these providers. Artificial Analysis page has no capture date.
