# Streaming Speech-to-Text (STT/ASR) for a Live Two-Party Multilingual Voice-Translation Call (state as of 2026-11-25)

Context: each participant's mic is one stream. The pipeline is STT, then MT, then voice-cloned TTS. What matters most: low-latency partial and final tokens, fast and reliable end-of-utterance detection, strong pt-BR/EN/ES accuracy, auto language ID and code-switching, cost per stream-minute, and a plain WebSocket API that Rust can drive.

Read this first: the market changed several times in November 2026. Soniox STT v6 shipped Nov 3, ElevenLabs Scribe v3 Live shipped Nov 10, and AssemblyAI-STT-3.7-Realtime shipped Nov 17. The independent benchmarks below (AA, June 2026; Coval, Sep-Oct 2026) do not cover those three releases yet. Many search results were aggregator summaries. Figures I could not trace to a primary page are labeled "unverified".

## 1. Which streaming STT models are current (Nov 2026)?

### Takeaway
The current flagship streaming models are AssemblyAI-STT-3.7-Realtime and -Realtime-Agent, Soniox stt-rt-v6, ElevenLabs Scribe v3 Live (succeeding Scribe v2 Realtime), Deepgram Flux / Flux Multilingual and Nova-3, Cartesia Ink-2, OpenAI GPT-Realtime-Whisper, Google Chirp 3 and Gemini 3.5 Transcribe Live, Speechmatics, Gladia Solaria 1, and Mistral Voxtral Mini Transcribe Realtime. Open-weight streaming options are NVIDIA Nemotron 3.5 ASR Streaming, Voxtral Mini 4B Realtime, Kyutai STT and Qwen3-ASR. Many of these were released or replaced in the last 30 days.

### Cited Findings
- **AssemblyAI:** AssemblyAI-STT-3.7-Realtime ($0.30/hr) and AssemblyAI-STT-3.7-Realtime-Agent ($0.45/hr) were released 2026-11-17. Universal-3.6 Pro Realtime (Sep 2026) is now "previous generation". Universal-Streaming English and Multilingual ($0.15/hr) were "replaced by AssemblyAI-STT-3.7-Realtime" on Nov 17. Universal-3.5 Pro Realtime retires 2027-02-01. — [AssemblyAI pricing.md, updated 2026-11-10](https://www.assemblyai.com/pricing.md); [AssemblyAI models.md](https://www.assemblyai.com/llms/models.md)
- **AssemblyAI language coverage:** the 3.7 Realtime, 3.7 Realtime-Agent and 3.6 Pro Realtime models cover 32 languages including Portuguese, with automatic language detection. 3.6 Pro does native code-switching in one stream. Legacy Universal-Streaming Multilingual covered only 6 languages (EN, ES, DE, FR, PT, IT). The streaming docs name no pt-BR vs pt-PT variant. — [AssemblyAI models.md](https://www.assemblyai.com/llms/models.md)
- **Soniox:** "Soniox STT v6" (real-time and async) was announced 2026-11-03. The real-time model is `stt-rt-v6`. It claims 60+ languages, one unified model with free mixing and mid-sentence switching, better diarization and language ID, and better endpointing. All of this is vendor claim with no numbers. — [Soniox v6 blog](https://soniox.com/blog/soniox-stt-v6); [Soniox RT translation docs](https://docs.soniox.com/stt/rt/real-time-translation). STT RT v5 retires 2026-12-15 and its traffic is routed to v6. — [Coval, 2026-10-28](https://www.coval.ai/blog/best-speech-to-text-providers-in-2026-independent-benchmarks-and-how-to-choose/)
- **Deepgram:** Flux Multilingual (`flux-general-multi`) went GA on 2026-04-29. It covers 10 languages (EN, ES, FR, DE, HI, RU, PT, JA, IT, NL), supports mid-call language switching, and has built-in turn detection. — [Deepgram blog](https://deepgram.com/blog/introducing-flux-multilingual); [SiliconANGLE 2026-04-29](https://siliconangle.com/2026/04/29/deepgram-expands-flux-10-languages-mid-call-switching-voice-agents/). Nova-3 Portuguese was improved for pt-BR and pt-PT in May 2026, per a Deepgram changelog quoted in search results. — [Deepgram changelog](https://developers.deepgram.com/changelogs/voice-agent-changelog)
- **ElevenLabs:** Scribe v3 and Scribe v3 Live launched 2026-11-10, reportedly at $0.22/hr for Live. They claim about 27% fewer word errors and "90+ languages" with a multilingual mode for mid-sentence switching. Only secondary sources confirmed this, so it is unverified. — [letsdatascience](https://letsdatascience.com/news/elevenlabs-launches-scribe-v3-and-v3-live-speech-to-text-mod-82ce234a); [ElevenLabs realtime STT page](https://elevenlabs.io/realtime-speech-to-text)
- **OpenAI:** GPT-Realtime-Whisper costs $0.017/min and is billed by audio duration. It launched in May 2026 alongside GPT-Realtime-2 and GPT-Realtime-Translate. Realtime sessions are capped at 60 minutes. — [OpenAI model page](https://developers.openai.com/api/docs/models/gpt-realtime-whisper); [aipricing.guru, May 2026](https://www.aipricing.guru/news/openai-realtime-2-voice-models-api-pricing-impact-may-2026/)
- **Other models in Coval's Sep-Oct 2026 streaming table:** Google Gemini 3.5 Transcribe Live, Google Chirp 3 and Chirp 2, Speechmatics "Linden 1", Cartesia Ink 2, Gladia Solaria 1, Mistral Voxtral Mini Transcribe Realtime 2602, Inworld STT 1, Nari Labs Qwen3 ASR Fast, Reson8 resonant-1 and Realtime Global, SpaceXAI Grok STT, Smallest Pulse, and Gradium. — [Coval](https://www.coval.ai/blog/best-speech-to-text-providers-in-2026-independent-benchmarks-and-how-to-choose/)
- **Unverified aggregator claims:** "Meta Muse Voice Transcribe" (3.1% on AA streaming, Sep 2026) and "Microsoft MAI-Transcribe-2-Streaming" (2.5% WER and 128 ms finalization, said to top AA on 2026-10-01). These came only from search snippets ([tech-insider](https://tech-insider.org/speech-to-text-api-comparison-2026/), [cartesia.ai learn page](https://www.cartesia.ai/learn/best-speech-to-text-models)). I could not confirm them on artificialanalysis.ai.

### Inferences
- AssemblyAI's offering today is STT-3.7-Realtime. The "Universal-Streaming" product the user first looked at is legacy as of Nov 17 2026, so any evaluation should target 3.7.
- No independent benchmark yet covers Soniox v6, Scribe v3 Live or AssemblyAI 3.7. Treat every ranking below as one generation stale for those three vendors.

### Gaps
- I did not fetch the live Artificial Analysis streaming leaderboard, so the Meta Muse and MAI-Transcribe-2 claims are unconfirmed.
- I did not verify Azure Speech, Google Chirp 3 or Gladia streaming pricing and features first-hand.
- I could not confirm whether Scribe v3 Live is GA on the realtime WebSocket or how it is priced.

## 2. Independent benchmarks: WER, latency, end-of-turn, multilingual and pt-BR

### Takeaway
On English streaming, the top accuracy tier is AssemblyAI U3.5/3.6 Pro (2.2-2.4%), followed by Qwen3-ASR, Gemini, Cartesia Ink-2 and Scribe v2 Realtime (about 2.6-4.3%). Soniox and Deepgram are the fastest to finalize (about 20-100 ms) at moderately higher WER. Soniox's WER varies wildly between benchmarks (1.3% to 11.9%). No independent streaming benchmark for pt-BR or Spanish exists, so run your own test.

### Cited Findings
- **Artificial Analysis AA-WER Streaming (published 2026-06-01; English-centric sets AgentTalk, Earnings22, VoxPopuli; latency measured from Silero-VAD end of speech):**
  - Cartesia Ink-2 (semantic endpoints): 3.59% final WER at 0.21 s.
  - ElevenLabs Scribe v2 Realtime: 3.64% at 0.14 s final, 3.65% at 0.13 s first partial. Price $6.50 per 1k minutes.
  - Cartesia Ink-2 (external endpoints): 3.66% at 0.09 s. Price $4 per 1k minutes.
  - AssemblyAI U3 Realtime Pro: 4.46% at 0.47 s partial.
  - Deepgram Nova-3: about 6.7% at 0.06 s.
  - Deepgram Flux: 7.36% at 0.020 s.
  - Soniox: 11.9% at about 0.05-0.06 s. Price $2 per 1k minutes, the cheapest.
  - Voxtral Mini Realtime: 5.3% final.
  - OpenAI GPT Realtime Whisper: 5.1% final.
  - Pareto frontier: Cartesia, ElevenLabs, Deepgram.
  - — [Artificial Analysis](https://artificialanalysis.ai/articles/new-streaming-speech-to-text-benchmark-aa-wer-streaming)
- **Coval (30 days to 2026-10-28; English only; p50/p95 time-to-final from shared VAD):**
  - AssemblyAI Universal 3.6 Pro: 2.2% WER, 106/222 ms, $0.45/hr, 2.7% on phone codec.
  - Qwen3 ASR Fast (Nari Labs): 2.6%, 45/69 ms, $0.12/hr.
  - Gemini 3.5 Transcribe Live: 3.0%, 361 ms avg.
  - Chirp 3: 3.2%, 766 ms.
  - Cartesia Ink 2: 3.4%, 150 ms.
  - Speechmatics Linden 1: 3.9%, 272 ms.
  - GPT Realtime Whisper: 4.0%, 557 ms.
  - Scribe v2 Realtime: 4.3%, 120 ms.
  - Inworld STT 1: 4.3%, 70 ms.
  - Soniox STT RT v5: 4.8%, 37/70 ms, $0.12/hr, 6.9% on phone codec.
  - Deepgram Nova 3: 5.2%, 86 ms.
  - Voxtral Mini RT 2602: 5.3%, 438 ms.
  - Deepgram Flux: 5.7%, 99/165 ms, 8.0% on phone codec.
  - AssemblyAI Universal Streaming (legacy): 5.9%.
  - Gladia Solaria 1: 6.2%, 436 ms.
  - Deepgram Flux Multilingual: 7.4%, 99 ms.
  - Parakeet TDT 0.6B v3: 11.0%.
  - Nemotron 3.5 ASR Streaming (Together): 13.0%.
  - Coval does not measure TTFT and has no per-language data. — [Coval](https://www.coval.ai/blog/best-speech-to-text-providers-in-2026-independent-benchmarks-and-how-to-choose/)
- **Coval single-model page:** Scribe v2 Realtime 4.2% WER, ranked 15th of 30, measured 2026-11-07. — [Coval benchmarks](https://benchmarks.coval.ai/models/scribe_v2_realtime)
- **Pipecat/Daily benchmark, per a search snippet:** Soniox stt-rt-v5 1.27% vs Scribe v2 Realtime 3.12%. Not fetched; unverified. A vendor-hosted AA table lists "Soniox v4" at 4.49% and 0.062 s. These conflict with AA's 11.9% for Soniox. — [search summary referencing Pipecat; no primary URL retrieved]
- **Turn detection:**
  - Deepgram Flux emits model-based end-of-turn events with a configurable confidence threshold plus a silence timeout. — [Coval](https://www.coval.ai/blog/best-speech-to-text-providers-in-2026-independent-benchmarks-and-how-to-choose/)
  - AssemblyAI 3.6 Pro uses punctuation and entity-aware endpointing (defaults: 100 ms min silence, 1000 ms max, VAD 0.3), with a median end-of-turn latency of 537 ms (vendor).
  - AssemblyAI 3.7-Realtime-Agent finalizes 86 ms after speech ends (was 172 ms), has 12% fewer false end-of-turns, and only 1.0% of words change between partial and final (was 3.2%). All vendor figures.
  - AssemblyAI 3.7-Realtime (non-agent) is "not tuned for turn-taking".
  - — [AssemblyAI models.md](https://www.assemblyai.com/llms/models.md)
  - Soniox `enable_endpoint_detection` finalizes all pending tokens as soon as the speaker stops. — [Soniox docs](https://docs.soniox.com/stt/rt/real-time-translation)
- **Code-switching:** AssemblyAI 3.7-Realtime reports 7.35% WER across five code-switched pairs (vendor). Early access saw German-English issues on 3.6 Pro. — [AssemblyAI models.md](https://www.assemblyai.com/llms/models.md)
- **Portuguese and Spanish (all vendor or offline):**
  - AssemblyAI Universal-3.5 Pro: ES about 7.6%, PT about 7.9%, vs about 9.8% for Universal Streaming (vendor, March 2026). — [AssemblyAI streaming benchmarks](https://www.assemblyai.com/docs/streaming/benchmarks)
  - Nemotron 3.5 streaming on FLEURS: ES about 4.1%, PT 5.48% (self-reported). — [HF model card](https://huggingface.co/nvidia/nemotron-3.5-asr-streaming-0.6b)
  - An arXiv Iberian benchmark (about Sep 2026) finds most systems cluster in a narrow WER band for ES and PT, so these languages discriminate poorly between systems. Voxtral Realtime scored 48.5% macro WER there, partly due to offline evaluation. — [arXiv 2609.36920](https://arxiv.org/html/2609.36920v1)
  - On FLEURS (offline), Azure MAI-Transcribe-2 has the best macro average and Qwen-Audio-3.0 is best on ES and PT. — [Qwen-Audio-3.0 report](https://arxiv.org/pdf/2609.07549)
- **Note on Coval's WER spread:** Coval shows only about 2-3 WER points between top and mid providers on English. Its FAQ recommends testing in your own target language. — [Coval](https://www.coval.ai/blog/best-speech-to-text-providers-in-2026-independent-benchmarks-and-how-to-choose/)

### Inferences
- In this pipeline the downstream MT step forgives small WER differences (about 1-2 points). Finalization latency and turn/segment boundary quality matter more for perceived lag, because translation and TTS wait for each segment.
- Soniox's inconsistent results suggest it is sensitive to test set and configuration. Its pt-BR quality must be tested directly.
- Deepgram Flux Multilingual (7.4%) is noticeably less accurate than Flux EN on English. Its value is turn detection, not accuracy.

### Gaps
- No independent streaming WER for pt-BR or ES across providers was found.
- No independent end-of-turn F1 or false-cutoff benchmark was found.
- No TTFT data for most providers beyond AA's June partials.
- The HF Open ASR Leaderboard multilingual track was not fetched, and it is offline-only.

## 3. Pricing, concurrency, regions, API ergonomics, partials, diarization, Rust

### Takeaway
Per stream, streaming STT now costs between $0.12/hr (Soniox, translation and diarization included) and about $1/hr (OpenAI). AssemblyAI 3.7 is $0.30-0.45/hr and bills by session time, idle included. Deepgram Flux Multilingual is about $0.47/hr. Every serious vendor exposes a JSON-over-WebSocket API with interim or non-final results, so Rust works with tokio-tungstenite. I found no official Rust SDKs.

### Cited Findings
- **AssemblyAI** — [pricing.md, 2026-11-10](https://www.assemblyai.com/pricing.md); [models.md](https://www.assemblyai.com/llms/models.md)
  - Prices: 3.7-Realtime $0.30/hr, 3.7-Realtime-Agent $0.45/hr, 3.6 Pro $0.45/hr, legacy Universal-Streaming $0.15/hr.
  - Add-ons: streaming diarization +$0.12/hr, PII +$0.12/hr, general prompting (beta) +$0.05/hr, Voice Focus +$0.10/hr (Agent only). Keyterms (up to 100) are free and can be updated mid-stream.
  - Billing is by WebSocket session duration, idle time included, and add-ons bill on the whole session.
  - Concurrency is 100 new streams/min on pay-as-you-go (5 on free tier).
  - Regions: US (`streaming.assemblyai.com`) and EU (`streaming.eu.assemblyai.com`) at the same price. Endpoint is `wss://streaming.assemblyai.com/v3/ws`.
  - No streaming translation; translation is pre-recorded only.
  - 3.7-Realtime emits about one partial per second. 3.6 Pro "only emits finals after a silence threshold".
- **Soniox** — [Soniox pricing](https://soniox.com/pricing); [Soniox docs](https://docs.soniox.com/stt/rt/real-time-translation)
  - stt-rt-v6 costs $0.12/hr (about $0.002/min), priced as $2 per 1M input audio tokens plus $4 per 1M output text tokens ("updated June 2026").
  - Translation, diarization, language ID and formatting are included at no extra charge.
  - No concurrency limit is stated ("hundreds of thousands of concurrent streams" is marketing).
  - Tokens carry `is_final`. Non-final tokens are replaced on each response and final tokens are appended.
  - Official SDKs are Python and Node; Rust is not mentioned. Regions are not stated.
- **Deepgram** (page undated; marked "Promo pricing" with no end date) — [Deepgram pricing](https://deepgram.com/pricing)
  - Pay-as-you-go / Growth: Flux EN $0.0065 / $0.0057 per min (was $0.0077), Flux Multilingual $0.0078 / $0.0068, Nova-3 Mono $0.0048 / $0.0042, Nova-3 Multi $0.0058 / $0.0050 (was $0.0092).
  - Streaming diarization +$0.0020/min.
  - Concurrency is 150 streaming connections on pay-as-you-go and 225 on Growth.
- **ElevenLabs:** Scribe v2 Realtime price conflicts. AA lists $6.50 per 1k min ($0.39/hr) — [AA](https://artificialanalysis.ai/articles/new-streaming-speech-to-text-benchmark-aa-wer-streaming). Secondary sources say Scribe v2 Realtime and v3 Live are both $0.22/hr — [datastudios](https://www.datastudios.org/post/elevenlabs-scribe-v3-live-speech-to-text-27-percent-fewer-errors-0-20-per-hour); [techjacksolutions](https://techjacksolutions.com/ai-tools/elevenlabs/scribe-3-pricing-and-api/). The price may have been cut at the v3 launch; check [elevenlabs.io/pricing/api](https://elevenlabs.io/pricing/api).
- **Cartesia Ink-2:** $4 per 1k min ($0.24/hr). — [AA](https://artificialanalysis.ai/articles/new-streaming-speech-to-text-benchmark-aa-wer-streaming)
- **OpenAI GPT-Realtime-Whisper:** $0.017/min ($1.02/hr) with a 60-minute session cap. A competitor page says it has no speaker labels in realtime transcription sessions. — [OpenAI](https://developers.openai.com/api/docs/models/gpt-realtime-whisper); [Soniox comparison](https://soniox.com/pricing/stt/soniox-vs-openai-whisper)
- **Speechmatics:** real-time Pro is about $0.24/hr (standard) or $0.43/hr (enhanced), and translation is a +$0.65/hr bolt-on. A competitor page lists "RT Enhanced + Translation" at $1.19/hr (July 2026). These come from aggregator and competitor sources, so they are unverified and conflicting. — [activepieces](https://www.activepieces.com/blog/speechmatics-pricing-2026-plans-rates-and-limits); [Soniox compare](https://soniox.com/compare/translate/pricing)
- **Inworld STT 1** is $0.10-0.15/hr and **Qwen3 ASR Fast** (Nari Labs) is $0.12/hr. — [Coval](https://www.coval.ai/blog/best-speech-to-text-providers-in-2026-independent-benchmarks-and-how-to-choose/)

### Inferences
- Per-participant-minute STT costs, one stream per speaker:
  - Soniox: $0.0020
  - Legacy AssemblyAI Universal-Streaming: $0.0025
  - Cartesia: about $0.0040
  - Scribe v3 Live: about $0.0037 at $0.22/hr, or $0.0065 at $0.39/hr
  - AssemblyAI 3.7-Realtime: $0.0050 ($0.0070 with diarization)
  - Deepgram Nova-3 Multi: $0.0058
  - AssemblyAI 3.7-Agent or 3.6 Pro: $0.0075
  - Deepgram Flux Multi: $0.0078
  - Speechmatics with translation: about $0.018-0.020
  - OpenAI: $0.017
- Diarization is unnecessary here because each participant has their own mic stream. Leave it off, since it is a paid add-on at AssemblyAI and Deepgram.
- With AssemblyAI's session billing, keep sockets open only while a participant is in the call, or reconnect on VAD after long silences.
- Rust: all of these vendors use JSON over WebSocket with binary PCM frames, which is straightforward with `tokio-tungstenite`. AssemblyAI's endpoint and Soniox's token protocol are documented. There are no first-party Rust SDKs, so plan to write a thin client (an inference; Rust SDK availability was not exhaustively checked).

### Gaps
- Regions and data residency for Soniox and ElevenLabs.
- Soniox's official concurrency caps.
- Whether Deepgram's promo pricing has an end date.
- Live verification of ElevenLabs and Speechmatics pricing pages.
- Gladia, Google and Azure streaming pricing were not checked.

## 4. Providers with native streaming translation, and its quality

### Takeaway
Soniox is the only major STT vendor that bundles real-time translation at no extra charge (one-way or two-way, 60+ languages, starting before the sentence ends). That fits a two-party call well. Speechmatics sells translation as an extra. OpenAI offers a separate GPT-Realtime-Translate model. AssemblyAI has no streaming translation. No independent quality benchmark exists for any of these streaming translators.

### Cited Findings
- **Soniox:**
  - Two modes: `"type":"one_way"` with `target_language`, and `"type":"two_way"` with `language_a` and `language_b`.
  - Translation tokens stream in chunks after the transcription tokens and are not aligned one-to-one with them. Translation starts before the sentence ends.
  - Model `stt-rt-v6`.
  - — [Soniox docs](https://docs.soniox.com/stt/rt/real-time-translation)
  - Vendor claims "3,600 language pairs", better handling of dates, numbers and currencies, and an adaptive wait before translating (for German and Korean word order). No metrics are published. — [Soniox v6 blog](https://soniox.com/blog/soniox-stt-v6); [Soniox speech-translation](https://soniox.com/speech-translation)
- **Speechmatics:** translation is a bolt-on (about +$0.65/hr) and reportedly only to and from English, per a competitor page, so unverified. — [Soniox vs Speechmatics](https://soniox.com/compare/soniox-vs-speechmatics); [activepieces](https://www.activepieces.com/blog/speechmatics-pricing-2026-plans-rates-and-limits)
- **OpenAI:** GPT-Realtime-Translate launched in May 2026 alongside GPT-Realtime-Whisper. I found no pricing or quality details. — [aipricing.guru](https://www.aipricing.guru/news/openai-realtime-2-voice-models-api-pricing-impact-may-2026/)
- **AssemblyAI:** translation is not offered for streaming. — [AssemblyAI pricing.md](https://www.assemblyai.com/pricing.md)

### Inferences
- Soniox two-way mode maps directly onto a two-party call: one stream per participant with `language_a`/`language_b` set to the pair. It removes the separate MT hop (about 100-400 ms and extra cost), and the result is still text, so it can feed your own voice-clone TTS.
- Soniox translation output is a black box. For quality control, you might still run your own LLM MT on the final source-text segments and use Soniox's translation only as a low-latency draft. Choose by A/B testing on pt-BR to and from EN/ES.

### Gaps
- No independent BLEU/COMET or human evaluation of Soniox, Speechmatics or OpenAI streaming translation.
- No translation latency figures in milliseconds.
- Soniox's per-language support for pt-BR was not shown on the pages fetched.

## 5. Self-hosted open-weight streaming STT: viability and GPU cost

### Takeaway
Open-weight streaming models are viable but trail the best APIs on accuracy (Coval: Nemotron 3.5 streaming 13.0% and Parakeet 0.6B v3 11.0% on English, vs 2-5% for top APIs). Qwen3-ASR is the exception at about 2.6-3.0% when hosted dedicated. With Soniox at $0.12/hr, self-hosting only pays off at high, steady concurrency or for data-sovereignty needs.

### Cited Findings
- **NVIDIA Nemotron 3.5 ASR Streaming 0.6B** (June 2026):
  - 40 language-locales, cache-aware streaming, OpenMDW-1.1 license (open weights; training data includes proprietary Riva data).
  - NVIDIA claims 240 real-time streams per H100 at 80 ms chunks, or 560 at 320 ms.
  - FLEURS PT 5.48% and ES about 4.1% (self-reported).
  - — [MarkTechPost](https://www.marktechpost.com/2026/06/06/nvidia-releases-nemotron-3-5-asr-a-600m-parameter-cache-aware-streaming-model-transcribing-40-language-locales-in-real-time/); [HF blog](https://huggingface.co/blog/nvidia/nemotron-speech-asr-scaling-voice-agents); [HF model card](https://huggingface.co/nvidia/nemotron-3.5-asr-streaming-0.6b); [wavect review](https://wavect.io/blog/nvidia-nemotron-3-5-asr-production-review/)
  - Coval measured it at 13.0% WER in English via Together AI. — [Coval](https://www.coval.ai/blog/best-speech-to-text-providers-in-2026-independent-benchmarks-and-how-to-choose/)
- **Mistral Voxtral Mini 4B Realtime:** Apache-2.0, 13 languages, configurable delay of 240 ms to 2.4 s, served via vLLM `/v1/realtime`. At 480 ms delay it is on par with Whisper (paper). — [arXiv 2602.11298](https://arxiv.org/html/2602.11298v1); [pinggy](https://pinggy.io/blog/best_open_source_self_hosted_speech_to_text_models/)
- **Kyutai STT:** the 1B model is English/French only (0.5 s delay), CC-BY-4.0. That rules it out for pt-BR and ES. — [pinggy](https://pinggy.io/blog/best_open_source_self_hosted_speech_to_text_models/)
- **Qwen3 ASR:** 1.7B on Baseten dedicated scores 3.0% WER with 37 ms finalization, and the Nari Labs hosted "Fast" variant scores 2.6% at 45 ms for $0.12/hr. Both English, per Coval. — [Coval](https://www.coval.ai/blog/best-speech-to-text-providers-in-2026-independent-benchmarks-and-how-to-choose/)

### Inferences
- Rough break-even: at an assumed H100 rental of about $2-3/hr (an assumption; not sourced here) serving about 240 streams, the compute cost is about $0.01/stream-hour. That is below Soniox's $0.12/hr only at full utilization, and it ignores ops, autoscaling, idle GPUs and lower accuracy. For a startup with bursty two-person calls, the API is cheaper overall.
- Self-hosting is a reasonable phase-2 fallback or sovereignty option, using Qwen3-ASR or Nemotron 3.5 for pt/es/en, after pt-BR evaluation.

### Gaps
- No independent per-GPU throughput or cost benchmark.
- No streaming pt-BR WER for open models.
- Qwen3-ASR streaming license and language details were not fetched.

## 6. Ranked recommendation for this use case (two-party pt-BR/EN/ES live translation call)

### Takeaway
**Primary: Soniox stt-rt-v6.** It costs $0.002 per participant-minute with translation included. Run two-way translation per speaker stream with endpoint detection on, use non-final tokens to drive speculative MT and TTS, and use final tokens to commit. **Fallback / A-B challenger: AssemblyAI-STT-3.7-Realtime** ($0.005/min, about $0.30/hr; or the -Agent variant at $0.0075/min for better end-of-turn) with separate LLM MT. Its accuracy is independently top-ranked and it has US and EU endpoints. **Turn-detection-centric alternative: Deepgram Flux Multilingual** ($0.0078/min, 10 languages including PT). **Worth benchmarking: ElevenLabs Scribe v3 Live** (about $0.0037/min if the $0.22/hr price is confirmed), especially if ElevenLabs is already the voice-clone TTS vendor.

### Cited Findings
- Soniox is $0.12/hr with translation, diarization and language ID included. It has the lowest finalization latency (37 ms p50, Coval) and native two-way translation. — [Soniox pricing](https://soniox.com/pricing); [Coval](https://www.coval.ai/blog/best-speech-to-text-providers-in-2026-independent-benchmarks-and-how-to-choose/); [Soniox docs](https://docs.soniox.com/stt/rt/real-time-translation)
- AssemblyAI U3.6 Pro is the most accurate streaming model in Coval's 30-model test (2.2%). STT-3.7 adds 32 languages with auto language ID, and Agent finalizes 86 ms after speech ends (vendor). — [Coval](https://www.coval.ai/blog/best-speech-to-text-providers-in-2026-independent-benchmarks-and-how-to-choose/); [AssemblyAI models.md](https://www.assemblyai.com/llms/models.md)
- Deepgram Flux has the fastest AA latency (20 ms) and model-based end-of-turn, but higher WER (Flux Multi 7.4%). — [AA](https://artificialanalysis.ai/articles/new-streaming-speech-to-text-benchmark-aa-wer-streaming); [Coval](https://www.coval.ai/blog/best-speech-to-text-providers-in-2026-independent-benchmarks-and-how-to-choose/)

### Inferences
- **Cost per participant-minute** (STT only, one stream per speaker; a two-person call costs twice this per call-minute):

  | Rank | Option | STT $/participant-min | Translation | Notes |
  |---|---|---|---|---|
  | 1 | Soniox stt-rt-v6 (two-way translation) | ~$0.0020 | Included | Lowest cost, lowest latency. Benchmark WER is inconsistent (1.3-11.9%). v6 not yet independently benchmarked. |
  | 2 | AssemblyAI-STT-3.7-Realtime | ~$0.0050 (session-billed) | Not included; add LLM MT (~$0.0005-0.002/min, estimate) | Best independent English accuracy (U3.6). EU region available. |
  | 2b | AssemblyAI-STT-3.7-Realtime-Agent | ~$0.0075 | Not included | Better end-of-turn: 86 ms final, 1% partial churn (vendor). |
  | 3 | Deepgram Flux Multilingual | ~$0.0078 PAYG (promo) | Not included | Best built-in turn events. Weaker multilingual WER. |
  | 4 | ElevenLabs Scribe v3 Live | ~$0.0037 (unverified; may be $0.0065) | Not included | Strong accuracy on v2. Possible single-vendor deal with ElevenLabs TTS. |
  | - | OpenAI GPT-Realtime-Whisper | $0.017 | Separate GPT-Realtime-Translate | Expensive, 557 ms finalization (Coval). Not recommended. |
  | - | Speechmatics with translation | ~$0.018-0.020 | Bolt-on, English-pivot | Expensive. |

- **Compared with the original AssemblyAI choice:** AssemblyAI is a strong, defensible fallback. It is about 2.5x Soniox's STT price, it charges for idle session time, and it has no streaming translation, so it needs a separate MT hop that adds latency and cost. Its advantage is the strongest independent accuracy evidence.
- **Before committing, run a 1-2 day bake-off:** feed the same 30-60 minutes of real pt-BR/EN/ES conversational audio, with some code-switching, to Soniox v6, AssemblyAI 3.7(-Agent), Flux Multi and Scribe v3 Live. Measure:
  - WER and CER per language
  - time from VAD end to final
  - partial churn
  - false end-of-turn rate
  - Soniox translation quality vs your LLM MT, judged by COMET or an LLM judge
- **Architecture hint:** keep the STT layer behind a Rust trait (`connect`, `send_pcm`, a stream of `{text, is_final, lang, end_of_turn}`) so vendors can be swapped. Every candidate is a plain WebSocket.

### Gaps
- The recommendation rests on English benchmarks plus vendor claims. pt-BR performance, and the quality of the three November 2026 releases, are unverified and need the bake-off above.
- The LLM MT cost estimate is a rough inference that was not researched here.
