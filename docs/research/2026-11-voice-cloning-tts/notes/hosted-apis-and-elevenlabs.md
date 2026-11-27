# Hosted voice-cloning TTS APIs (group B) and the ElevenLabs baseline

Research date: 2026-11-27. Scope: Fish Audio, MiniMax Speech, Resemble AI, Deepgram, Google
(Chirp 3 Instant Custom Voice, Gemini TTS), OpenAI custom voices, Azure Personal Voice, and
ElevenLabs. Pages fetched on the research date unless noted. Where a page was undated, that is
flagged.

## Cloning model and voice limits (zero-shot per request vs create-a-voice; caps; gating)

### Takeaway
Only Fish Audio (inline `references`) and Google Chirp 3 Instant Custom Voice (client-held
"voice cloning key", no server-stored voice, no cap on keys) avoid a stored-voice quota. MiniMax
creates a stored voice but has generous slot counts (10 to 800+) and auto-deletes unused clones
after 7 days. ElevenLabs, OpenAI (20 voices per org) and Azure use a stored voice. Google, OpenAI
(recording-based) and Azure are gated behind sales or an approval form. Deepgram has no
self-serve cloning, and Resemble's hosted TTS appears not to be sold to new customers.

### Cited Findings
**ElevenLabs (baseline)**
- Monthly voice operations ("creating a new voice or editing an existing voice"): Free 55,
  Starter 65, Creator 95, Pro 290, Scale 1,040, Business 1,040, Enterprise unlimited. Operations
  reset when credits renew (monthly on most paid plans). Page undated; it does not say whether
  deleting counts or whether API use is exempt. —
  [ElevenLabs help: voice add/edit limit](https://elevenlabs.io/docs/help-center/account/general/is-there-a-limit-on-how-many-times-i-can-edit-add-voices)
- Custom voice slots: Free 3, Starter 10, Creator 30, Pro 160, Scale 660, Business 660. Voice
  Library voices do not consume slots; to get more slots "you would need to upgrade your account".
  — [ElevenLabs help: voice slots per tier](https://elevenlabs.io/docs/help-center/account/general/how-many-voice-slots-do-i-get-per-tier-and-how-can-i-increase-it)
- The models overview does not describe any way to synthesize from reference audio without first
  creating a voice. —
  [ElevenLabs models](https://elevenlabs.io/docs/overview/models)

**Fish Audio**
- The TTS API accepts `references` (array of audio + transcript pairs) for zero-shot cloning per
  request; the body must be MessagePack because JSON cannot carry raw bytes. The WebSocket API
  also accepts reference audio in place of `reference_id`. Fish's guidance: a pre-created voice
  model (`reference_id`) gives better quality and lower latency than inline references. —
  [Fish TTS API reference](https://docs.fish.audio/api-reference/endpoint/openapi-v1/text-to-speech),
  [Fish WebSocket TTS](https://docs.fish.audio/text-to-speech/text-to-speech-ws),
  [Fish TTS best practices](https://docs.fish.audio/resources/best-practices/text-to-speech)
- Consumer subscription voice slots (Free 3 public; Plus 10 private, 1 professional; Pro
  unlimited, 5 professional). The API is billed separately, pay-as-you-go. —
  [Fish plans](https://fish.audio/plan/)
- The API pricing/rate-limit page states no voice-model creation limits. —
  [Fish pricing and rate limits](https://docs.fish.audio/developer-guide/models-pricing/pricing-and-rate-limits)

**MiniMax Speech**
- Create-a-voice flow (upload file, then clone to a `voice_id`). A cloned voice not used in a T2A
  call within 7 days is deleted (official docs, per search summary; I could not open the
  voice-clone guide directly). Inputs mp3/m4a/wav, up to 20 MB. —
  [MiniMax voice clone guide](https://platform.minimax.io/docs/guides/speech-voice-clone),
  [MiniMax voice cloning intro](https://platform.minimax.io/docs/api-reference/voice-cloning-intro)
- Voice slots by Audio Package: Starter 10, Standard 100, Pro 250, Scale 500, Business 800,
  custom pricing beyond. — [MiniMax pricing](https://platform.minimax.io/docs/guides/pricing)
- Search summary claimed reference "clips under 8 seconds", which conflicts with my recollection
  of a 10 s minimum; treat as unverified (see Gaps).

**Google Chirp 3 HD Instant Custom Voice**
- Allow-listed only; request access through sales. Requires a consent statement (provided script
  only, up to 10 s, same language) plus up to 10 s of reference audio. Produces a "voice cloning
  key" (a text string) stored client-side and sent with each request; no limit on number of
  keys, but creation is limited to 10 per minute per project; one key can be used concurrently. —
  [Chirp 3 Instant Custom Voice docs](https://docs.cloud.google.com/text-to-speech/docs/chirp3-instant-custom-voice)

**Google Gemini TTS**
- No cloning information found; see Gaps.

**OpenAI custom voices**
- Recording-based voices are limited to eligible customers (contact sales); up to 20 recording
  voices and 100 prompt-generated voices per organization. Flow: consent recording of an exact
  phrase → `/v1/audio/voice_consents` → consent id → sample (≤ 60 s, ≤ 10 MiB) to
  `/v1/audio/voices`. Recording voices work with the TTS API, Realtime API and Chat Completions
  audio; prompt-generated voices only with GPT-Live. The guide does not name `gpt-4o-mini-tts`. —
  [OpenAI custom voices guide](https://developers.openai.com/api/docs/guides/custom-voices)

**Azure Personal Voice**
- Create a `speakerProfileId` from a verbal consent statement + 5–90 s sample; training < 5 s.
  API access "restricted to eligible customers and approved use cases" via intake form
  (aka.ms/customneural); restricted use cases per transparency note. Doc dated 2026-09-09. —
  [Azure personal voice overview](https://learn.microsoft.com/en-us/azure/ai-services/speech-service/personal-voice-overview)

**Deepgram**
- Pricing page lists custom voices as "Contact sales"; no official self-serve cloning doc found;
  a third-party review states Aura has no voice cloning. —
  [Deepgram pricing](https://www.deepgram.com/pricing),
  [rywalker review (third-party)](https://rywalker.com/research/deepgram-aura)

**Resemble AI**
- resemble.ai/pricing now lists only detection, watermarking and identity products (Flex $0,
  Team $350/mo, Business $1,000/mo, Enterprise); no TTS or cloning. —
  [Resemble pricing](https://www.resemble.ai/pricing/)
- An October 2026 third-party review says Resemble no longer sells voice generation to new
  customers. — [Voiceflow review](https://www.voiceflow.com/blog/resemble-ai)
- A Resemble changelog URL titled "voice-cloning-pricing-first-clone-free-then-2-each-2026"
  exists (not opened). —
  [Resemble changelog](https://www.resemble.ai/changelogs/voice-cloning-pricing-first-clone-free-then-2-each-2026)

### Inferences
- For Diceri (one clone per consenting caller per call, deleted at call end, §9 / ADR 0006),
  ElevenLabs' monthly add/edit cap is the binding constraint: even Business (1,040/mo) allows
  only ~1,040 consenting speaker sessions per month unless Enterprise. Slot counts matter less
  if voices are deleted after each call.
- Fish inline references and Google's client-held keys fit "clone per call, discard after" best:
  nothing (Fish inline) or no server-side voice object (Google) must be created/deleted. Fish
  trades a little latency/quality for inline mode; a hybrid (create model at call start, delete at
  end) is possible if Fish has no creation cap (unconfirmed).
- MiniMax's 7-day auto-delete matches the retention goal but Diceri should still delete
  explicitly if an API exists (not verified).
- Google, OpenAI and Azure are gated; approval timelines are unknown and may block a small team.
  Resemble and Deepgram are effectively out of scope for hosted cloning.

### Gaps
- Whether ElevenLabs API voice creation counts against the add/edit cap (very likely, not stated)
  and whether deleting a voice counts.
- Fish Audio: any cap on voice-model creation via API; whether `references` per request is
  billed differently.
- MiniMax: exact reference-audio length bounds (10 s–5 min vs "< 8 s"), explicit delete API.
- Gemini TTS: whether any voice cloning exists (I found no source; my prior belief is that it
  offers only prebuilt voices).
- Azure Personal Voice profile limits per resource and approval timeline.

## Price per million characters with a cloned voice; extra cost per clone

### Takeaway
Cheapest cloned-voice synthesis among those verified: Fish Audio s2-pro / s2.1-pro at $15 per
million UTF-8 bytes (no per-clone fee found). ElevenLabs API is $40/M (Flash/Turbo, v4 Turbo) to
$80/M (Multilingual v2, v3, v4). Google Instant Custom Voice ~$60/M (from a garbled snippet).
MiniMax $60/M (Turbo) or $100/M (HD) plus $1.50 per clone. Azure and OpenAI prices not verified.

### Cited Findings
- ElevenLabs API per 1k chars: Flash/Turbo $0.04, v4 Turbo $0.04, v3 Conversational $0.04,
  Multilingual v2 $0.08, v3 $0.08, v4 $0.08 (i.e. $40 or $80 per million). Subscriptions: Free $0
  (10k chars on v4/v3/MLv2, 20k on Flash), Starter $6 (75k/150k), Creator $22 (275k/550k), Pro $99
  (1.238M/2.475M), Scale $299 (3.738M/7.475M), Business $990 (12.375M/24.75M). —
  [ElevenLabs API pricing](https://elevenlabs.io/pricing/api)
- Fish Audio per million UTF-8 bytes: `s2.1-pro` $15, `s2-pro` $15, `s1` $15 (deprecated,
  retires 2026-12-31, routes to s2.1-pro), `s2.1-pro-free` $0, `drama-3` $50 ($10 promo until
  2026-11-30). Voice design $0.01/request. Note: bytes, not characters, so non-Latin scripts
  (CJK ~3 bytes/char, Cyrillic/Arabic/Greek ~2) cost proportionally more. —
  [Fish pricing and rate limits](https://docs.fish.audio/developer-guide/models-pricing/pricing-and-rate-limits)
- MiniMax per million characters: speech-2.8-hd $100, speech-2.8-turbo $60 (current); legacy
  speech-2.6-hd / 02-hd $100, speech-2.6-turbo / 02-turbo $60. Rapid Voice Cloning $1.50 per voice,
  charged at first synthesis use (not at creation). — [MiniMax pricing](https://platform.minimax.io/docs/guides/pricing)
- Google: Chirp 3 HD $30/M chars; Instant custom voice $60/M chars, no free tier (search snippet
  of the pricing page; the page itself would not render for me). —
  [Google TTS pricing](https://cloud.google.com/text-to-speech/pricing)
- Deepgram Aura-2: $0.030 per 1k chars ($0.027 Growth) for stock voices. —
  [Deepgram pricing](https://www.deepgram.com/pricing)
- Azure Personal Voice: billed per 1M characters (real-time and batch); voice creation free;
  profile storage billed (pages conflict: per 1,000 profiles/month vs per profile/day); dollar
  figures not rendered. — [Azure Speech pricing](https://azure.microsoft.com/en-us/pricing/details/cognitive-services/speech-services/)
- OpenAI: recording-based voice pricing not stated in the custom voices guide. —
  [OpenAI custom voices guide](https://developers.openai.com/api/docs/guides/custom-voices)
- Resemble (third-party, conflicting): Flex $0.0005/s TTS, $2/mo per Rapid clone, $5/mo per Pro
  clone (Sept 2026 listing) vs "no longer sold to new customers" (Oct 2026). Hosted Chatterbox
  on third parties: DeepInfra chatterbox-turbo $1.00/M chars; fal.ai chatterboxhd $0.04/1k. —
  [frontdeskreview](https://frontdeskreview.com/software/ai-voice-tts/resemble-ai/),
  [Voiceflow](https://www.voiceflow.com/blog/resemble-ai),
  [DeepInfra Chatterbox Turbo](https://deepinfra.com/ResembleAI/chatterbox-turbo/voice),
  [fal.ai Chatterbox HD](https://fal.ai/models/resemble-ai/chatterboxhd/text-to-speech/llms.txt)

### Inferences
- Versus ElevenLabs' ~$50/M blended rate cited in the brief, Fish is ~3x cheaper for Latin-script
  languages and roughly at parity for CJK (byte billing). MiniMax Turbo and Google ICV are
  slightly more expensive than ElevenLabs Flash ($40/M); MiniMax's $1.50 per clone dominates for
  short calls (e.g. a caller speaking 3,000 translated chars costs $0.18 synthesis + $1.50 clone).
- Open-weights Chatterbox via third-party hosts ($1/M) is an order of magnitude cheaper but is a
  different category (covered by other researchers presumably).

### Gaps
- Azure Personal Voice and OpenAI custom-voice dollar rates; Gemini TTS token prices (pricing page
  did not render).
- Volume discounts for Fish/MiniMax beyond list price.

## Languages and cross-lingual cloning

### Takeaway
Language coverage for a cloned voice: Azure Personal Voice ~91 languages (auto-detect),
ElevenLabs v4/v4 Turbo 90+ (v3 70+, Flash 32), Fish ~80 (S2.1 Pro reportedly 83), MiniMax 40,
Google ICV 30 locales (with limited documented cross-lingual pairs), OpenAI consent in 16
languages. Only Fish publishes per-language WER/SIM for cloning against ElevenLabs and MiniMax.

### Cited Findings
- ElevenLabs: Flash v2.5 32 languages, Turbo v2.5 32, Multilingual v2 29, v3 70+, v4 90+,
  v4 Turbo 90+, v3 Conversational 70+. — [ElevenLabs models](https://elevenlabs.io/docs/overview/models)
- Fish S2: pre-training on ~80 languages and dialects; multilingual eval on 24 languages. —
  [Fish Audio S2 technical report](https://arxiv.org/html/2603.08823v2)
- Fish S2.1 Pro reportedly 83 languages (secondary source). —
  [testingcatalog](https://www.testingcatalog.com/fish-audio-launches-s2-1-pro-with-support-for-83-languages/)
- MiniMax: 40 languages. — [MiniMax API overview](https://platform.minimax.io/docs/api-reference/api-overview)
- Google ICV: 30 locales (Arabic, Bengali, Chinese, Dutch, English AU/IN/UK/US, French CA/FR,
  German, Gujarati, Hindi, Indonesian, Italian, Japanese, Kannada, Korean, Malayalam, Marathi,
  Polish, Portuguese BR, Russian, Tamil, Telugu, Thai, Turkish, Vietnamese, Spanish ES/US).
  Documented cross-lingual: keys created in en-US can synthesize de-DE, es-US/ES, fr-CA/FR, pt-BR.
  — [Chirp 3 ICV docs](https://docs.cloud.google.com/text-to-speech/docs/chirp3-instant-custom-voice)
- Azure Personal Voice: speak "any of the 91 languages supported across 100+ locales", automatic
  sentence-level language detection, no locale tag required. —
  [Azure personal voice overview](https://learn.microsoft.com/en-us/azure/ai-services/speech-service/personal-voice-overview)
- OpenAI: consent phrases in 16 languages (de, en, es, fr, hi, id, it, ja, ko, nl, pl, pt, ru, uk,
  vi, zh); GPT-Live custom voices English accents only. —
  [OpenAI custom voices guide](https://developers.openai.com/api/docs/guides/custom-voices)
- Deepgram Aura-2: few languages (Dec 2025 changelog added nl, de, fr, it, ja; reviews say ~7
  total). — [Deepgram changelog 2025-12-12](https://developers.deepgram.com/changelog/2025/12/12)

### Inferences
- No provider covers all ~57 Diceri languages with a documented cloned voice except possibly
  Azure (91), ElevenLabs v4 (90+) and Fish (~80); MiniMax (40) and Google ICV (30) would need a
  stock-voice fallback for the rest.
- Google ICV's documented cross-lingual support is from en-US only; a pt-BR speaker heard in
  Japanese is not documented.

### Gaps
- No vendor publishes cross-lingual speaker-similarity numbers for e.g. pt→ja; Fish's tables are
  same-language cloning. A Diceri-specific listening test is needed.
- Exact language lists vs Diceri's 57 were not cross-checked.

## Streaming, formats, time-to-first-audio, concurrency

### Takeaway
All candidates stream. Fish (WebSocket, TTFA < 100 ms model-side, concurrency 5/15/50 by
prepaid tier), ElevenLabs (Flash and v4 Turbo ~100 ms model latency, concurrency 4–40 by plan),
MiniMax (WebSocket; no official TTFA; RPM 10–800 by package), Google ICV (streaming API). No
independent, network-inclusive TTFA numbers were verified.

### Cited Findings
- ElevenLabs: Flash v2.5 ~100 ms, v4 Turbo ~100 ms, v3 Conversational ~165 ms median model latency
  (excluding app/network); v4 Turbo and v3 Conversational on Text-to-Dialogue WebSocket. —
  [ElevenLabs models](https://elevenlabs.io/docs/overview/models)
- ElevenLabs concurrency 4/6/10/20/30/40 listed in tier order Free→Business (mapping inferred by
  page structure). — [ElevenLabs API pricing](https://elevenlabs.io/pricing/api)
- Fish: concurrency limits by prepaid total: < $100 → 5, ≥ $100 → 15, ≥ $1,000 → 50, Enterprise
  custom; shared across keys, STT counts; 429 without Retry-After. —
  [Fish pricing and rate limits](https://docs.fish.audio/developer-guide/models-pricing/pricing-and-rate-limits)
- Fish S2: TTFA "below 100 ms" (abstract) / "as low as 100 ms" (Section 5); RTF 0.195 on one H200.
  These are self-hosted model numbers, not API round-trip. —
  [Fish S2 technical report](https://arxiv.org/html/2603.08823v2)
- Fish `latency` parameter: "balanced" (default, faster) vs "normal" (higher quality). —
  [Fish TTS best practices](https://docs.fish.audio/resources/best-practices/text-to-speech)
- MiniMax: WebSocket T2A available; official guide gives no TTFA. Third-party WaveSpeed claims
  sub-250 ms for Speech-2.6 Turbo (unofficial). RPM by Audio Package: Starter 10, Standard 50,
  Pro 200, Scale 500, Business 800. —
  [MiniMax WebSocket guide](https://platform.minimax.io/docs/guides/speech-t2a-websocket),
  [MiniMax pricing](https://platform.minimax.io/docs/guides/pricing),
  [WaveSpeed (third-party)](https://wavespeed.ai/docs/docs-api/minimax/speech-2.6-turbo)
- Google ICV: streaming supported; outputs LINEAR16, A-law, mu-law, OGG Opus, PCM. —
  [Chirp 3 ICV docs](https://docs.cloud.google.com/text-to-speech/docs/chirp3-instant-custom-voice)

### Inferences
- Fish's 5-concurrent Starter tier is too small for multi-room production; $100 prepaid unlocks 15.
- MiniMax's RPM limits (10 RPM on Starter) are tight for sentence-by-sentence synthesis across
  many simultaneous calls.

### Gaps
- Measured end-to-end TTFA over the public internet from the Diceri host for each API.
- Output formats for Fish/MiniMax/ElevenLabs were not re-verified in this pass.

## Commercial terms, consent, retention, residency

### Takeaway
Google, OpenAI and Azure enforce a recorded consent statement from the speaker before cloning;
this aligns with ADR 0006 but adds call-start friction (reading a fixed phrase). OpenAI voice
creation is not compatible with Zero Data Retention. Google ICV offers EU/Asia regions; Azure is
regional. Fish/MiniMax/ElevenLabs residency and reference-audio retention were not verified.

### Cited Findings
- Google ICV: consent recording of a provided script required; regions global, us, eu,
  asia-southeast1, asia-northeast1, europe-west2. —
  [Chirp 3 ICV docs](https://docs.cloud.google.com/text-to-speech/docs/chirp3-instant-custom-voice)
- OpenAI: consent phrase must match exactly and come from the same speaker; no children's
  voices; must disclose AI-generated speech and preserve watermarks/provenance; voice creation
  not ZDR-compatible. — [OpenAI custom voices guide](https://developers.openai.com/api/docs/guides/custom-voices)
- Azure: verbal consent statement required acknowledging the resource owner will create and use
  the voice; "no unapproved use case allowed". —
  [Azure personal voice overview](https://learn.microsoft.com/en-us/azure/ai-services/speech-service/personal-voice-overview)
- MiniMax: unused clones deleted after 7 days. —
  [MiniMax voice clone guide](https://platform.minimax.io/docs/guides/speech-voice-clone)

### Inferences
- Diceri's consent step could record the provider's required phrase in the caller's language,
  which doubles as the reference sample source only if the provider allows (Google says same
  environment, separate recording).

### Gaps
- Fish Audio, MiniMax and ElevenLabs terms on consent, reference-audio retention, training use,
  and data residency (not fetched within the tool budget).

## Published quality metrics

### Takeaway
Fish's own S2 report beats ElevenLabs and MiniMax on speaker similarity in 17/24 languages and
WER in 11/24 on the MiniMax Multilingual test set, but ElevenLabs' result there is likely an
older model. Arena (Artificial Analysis) snapshots conflict across 2026 and do not measure
cloned voices.

### Cited Findings
- Fish S2 vs MiniMax vs ElevenLabs (MiniMax Multilingual Testset), WER % / SIM: English
  1.620/2.164/2.339 and 0.797/0.756/0.613; Japanese 2.760/3.519/10.646 and 0.796/0.776/0.738;
  Chinese 0.730/2.252/16.026 and 0.816/0.780/0.677; Vietnamese 7.410/0.880/73.415 and
  0.740/0.743/0.369; Thai 4.230/2.701/73.936 and 0.786/0.800/0.588. MiniMax and ElevenLabs keep an
  advantage in some low-resource languages. Seed-TTS-Eval WER test-en 0.99, test-zh 0.54. —
  [Fish S2 technical report](https://arxiv.org/html/2603.08823v2) (vendor-authored)
- Artificial Analysis arena snapshots: ElevenLabs v3 Elo 1177 (April 2026 export); Fish S2 Pro
  1,123 on the open-weights provider-voice board; a Sept 2026 article puts Cartesia Sonic 3.6 top
  at 1279. These conflict and are dated. —
  [magichour export](https://magichour.ai/model-leaderboard/text-to-speech),
  [AA open-weights board](https://artificialanalysis.ai/text-to-speech/leaderboard/provider-voice/open-weights),
  [digitalapplied Sept 2026](https://www.digitalapplied.com/blog/best-text-to-speech-models-september-2026-ranked-priced)
- Community test (FLEURS refs, en/de/ar/es/zh): S2 Pro strong de/ar speaker similarity but
  slower than some rivals. — [HF discussion](https://huggingface.co/fishaudio/s2-pro/discussions/26)

### Inferences
- The ElevenLabs numbers in Fish's table (e.g. 73% WER in Thai/Vietnamese) suggest an older model
  (probably Multilingual v2, which lacks those languages), so they say little about v4.

### Gaps
- No independent cloning-specific benchmark (cross-lingual SIM) covering all candidates.
- Live AA leaderboard not fetched; current ranks unverified.
