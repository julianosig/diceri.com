# Multilingual quality of real-time streaming STT (state as of 2026-11-25)

Context: a live voice-translation video call where each participant has their own mic stream and may speak any of many languages. The current primary pick is Soniox stt-rt-v6, with AssemblyAI-STT-3.7-Realtime as challenger (see ../../2026-11-voice-translation-stack/notes/streaming-stt.md). These notes add the per-language evidence.

**Source caveat, read first.** The only per-language streaming table that covers several vendors with fresh models (Soniox v6, Scribe v3 Live, Gemini 3.5 Live, AssemblyAI 3.7) is AssemblyAI's own benchmark report. AssemblyAI ran it (runs 2026-08-25 to 2026-11-14), streamed audio in real time with the language fixed and competitors on default settings, and marks some cells "≈" without defining the mark. It is a vendor benchmark, but it is detailed, reproducible in method, and the only multi-vendor per-language streaming data that exists. Every other per-language figure is either vendor self-reported for its own model or offline.

## 1. Per-language streaming WER/CER across providers

### Takeaway
Across 18 major languages on FLEURS (streaming, language fixed), Google Gemini 3.5 Transcribe Live has the best mean (6.0%), followed by ElevenLabs Scribe v3 Live (6.8%), Soniox RT v6 (6.9%) and AssemblyAI 3.7 Realtime/Agent (7.3-7.5%). Deepgram Nova-3 (15.0%) and OpenAI GPT Realtime (11.8%) are clearly behind. The leaders are within about 1 point of each other on average, but which one wins changes by language. AssemblyAI is strongest on Western European languages, Soniox and Gemini on Hindi, Arabic, Vietnamese and Finnish. AssemblyAI is weak on Vietnamese (11-12%) and Arabic (11.2%). On Common Voice the order flips: AssemblyAI 3.7 is best at 6.8%, and Gemini drops to 8.9%.

### Cited Findings
- **FLEURS streaming, 18 languages (AssemblyAI benchmark, runs 2026-08-25 to 2026-11-14; vendor-run; WER%, CER for ja/zh/ko; ≈ as published):** — [AssemblyAI benchmarks](https://www.assemblyai.com/benchmarks)

  | System | Mean | es | fr | de | it | pt | nl | da | sv | fi | ru | tr | ar | he | hi | vi | ja* | zh* | ko* |
  |---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
  | Google Gemini 3.5 Transcribe Live | 6.0 | 3.8 | 5.4 | 4.7 | 3.2 | 4.7 | 5.0 | 9.1 | 7.3 | 7.5 | 5.3 | 5.3 | 6.7 | 18.9 | 5.0 | 5.2 | 3.6 | 5.7 | 2.3 |
  | ElevenLabs Scribe v3 Live | 6.8 | 3.9 | 5.8 | 5.5 | 3.6 | 5.5 | 5.1 | 9.0 | 7.7 | 7.8 | 7.3 | 5.9 | 7.6 | 20.7 | 9.3 | 5.9 | 3.4 | 5.5 | 2.4 |
  | Soniox Realtime v6 | 6.9 | 4.5 | 7.0 | 6.4 | 4.4 | 5.8 | 7.2 | 9.9 | 9.2 | 6.7 | 6.6 | 6.8 | 6.8 | 18.0 | 6.2 | 6.5 | 3.9 | 5.6 | 2.4 |
  | AssemblyAI-STT-3.7-Realtime-Agent | 7.3 | 4.0 | 4.8 | 4.8 | 3.6 | 5.3 | 5.6 | 11.4 | 6.2 | 9.9 | 7.0 | 8.0 | 11.2 | 18.0 | 7.4 | 11.3 | 3.8 | 7.3 | 2.9 |
  | Soniox Realtime (v5) | 7.5 | 5.0 | 7.8 | 6.9 | 4.9 | 6.2 | 7.7 | 11.4 | 10.3 | 8.0 | 7.3 | 7.3 | 7.2 | 19.2 | 6.3 | 7.0 | 4.0 | 5.8 | 2.6 |
  | AssemblyAI-STT-3.7-Realtime | 7.5 | 4.0 | 4.7 | 4.8 | 3.7 | 5.3 | 5.9 | 11.5 | 6.5 | 10.0 | 7.3 | 8.2 | 11.2 | 18.5 | 7.3 | 11.9 | 4.0 | 7.3 | 3.0 |
  | AssemblyAI Universal-3.6 Pro RT | 7.9 | 4.1 | 5.0 | 5.0 | 4.0 | 5.5 | 5.9 | 12.7 | 6.6 | 10.8 | 7.5 | 8.6 | 12.1 | 18.2 | 7.5 | 13.9 | 4.0 | 7.7 | 2.8 |
  | Meta Muse Voice Transcribe | 9.5 | 6.5 | 8.0 | 6.4 | 6.0 | 8.7 | 8.6 | 13.6 | 11.0 | 10.7 | 10.1 | 10.5 | 8.9 | 27.0 | 8.1 | 6.5 | 8.0 | 8.0 | 4.8 |
  | OpenAI GPT Realtime | 11.8 | 5.7 | 11.8 | 11.2 | 5.9 | 8.8 | 10.1 | 14.7 | 12.4 | 12.4 | 10.4 | 14.3 | 10.5 | 34.5 | 10.4 | 11.4 | 11.3 | 10.9 | 6.3 |
  | Azure (model not specified) | 12.0 | 7.7 | 11.4 | 9.1 | 8.3 | 10.9 | 12.9 | 19.8 | 16.5 | 15.6 | 13.1 | 15.2 | 12.5 | 27.6 | 8.6 | 9.4 | 6.5 | 6.3 | 5.4 |
  | Deepgram Nova-3 | 15.0 | 6.7 | 10.9 | 12.1 | 8.3 | 11.9 | 11.8 | 27.0 | 18.3 | 20.0 | 10.6 | 15.5 | 22.9 | 30.2 | 20.4 | 15.2 | 10.7 | 10.7 | 6.3 |

- **Systems covering fewer than 18 languages (same report; FLEURS mean over each system's own languages, so not comparable):** — [AssemblyAI benchmarks](https://www.assemblyai.com/benchmarks)
  - Zoom Scribe Live: 5.8 (8 languages)
  - Reson8: 5.8 (7)
  - Gradium: 7.5 (4)
  - Mistral Voxtral Mini: 8.4 (12)
  - Smallest Pulse: 10.8 (11)
  - xAI Grok Voice Transcribe 2.0: 11.3 (17)
  - Alebex: 11.7 (17)
  - Deepgram Flux: 12.5 (9)
  - Speechmatics Enhanced/Standard, Gladia Solaria, Cartesia Ink-Whisper and AWS were run on 14-17 languages, but I could not recover their per-language values or means from the page text.
- **Common Voice streaming means (same report; 18 languages unless noted):** — [AssemblyAI benchmarks](https://www.assemblyai.com/benchmarks)
  - AssemblyAI 3.7-Realtime: 6.8
  - AssemblyAI 3.7-Realtime-Agent: 6.9
  - AWS Transcribe: 7.2
  - AssemblyAI U3.6 Pro: 7.2
  - Scribe v3 Live: 7.4
  - Soniox RT v6: 8.4
  - Gemini 3.5 Live: 8.9
  - Soniox RT v5: 9.1
  - Azure: 9.5
  - Meta Muse: 9.9
  - Gladia Solaria: 15.5
  - Deepgram Flux: 13.2 (9 languages)
  - Mistral Voxtral Mini: 13.0 (12 languages)
- **AssemblyAI's own multilingual summary (vendor):** 3.7-Realtime is 11.72% on a 30-language average vs 12.25% for 3.6 Pro. The test set is not named, and the language list has 32 entries. — [AssemblyAI models.md, updated 2026-11-17](https://www.assemblyai.com/llms/models.md)
- **Google (vendor, 2026-08-26):** Gemini 3.5 Transcribe scores 5.50% FLEURS streaming vs 5.04% non-streaming, and 4.0% streaming on Artificial Analysis. No per-language figures; the only comparison is against Chirp 3. — [Google blog](https://blog.google/innovation-and-ai/models-and-research/gemini-models/gemini-3-5-transcribe/)
- **ElevenLabs (vendor, 2026-11-10, updated 2026-11-14):** Scribe v3 Live has 43% lower semantic WER and 31% lower WER than Scribe v2 Realtime across 102 languages, macro-averaged per language. Largest gains: Greek -77%, Danish -54%, Croatian -45%, Hindi -36%. Absolute per-language WER appears only in charts, and no named competitors are compared. — [ElevenLabs blog](https://elevenlabs.io/blog/scribe-v3). A secondary source quotes ElevenLabs docs as "44% lower semantic WER on FLEURS across 73 languages", which conflicts with the 102 figure. — [alphasignal](https://alphasignal.ai/news/elevenlabs-scribe-v3-cuts-live-transcription-errors-by-43-across-102-languages)
- **Soniox (vendor, 2026-11-03):** the v6 launch post gives no per-language numbers at all. — [Soniox v6 blog](https://soniox.com/blog/soniox-stt-v6)
- **Pipecat/Daily realtime benchmark (November 2026; semantic WER; 1,000 real voice-agent samples from smart-turn-data-v3.1):**
  - Soniox stt-rt-v6: 0.74% (1st)
  - Meta Muse: 0.80%
  - AssemblyAI U3.6 Pro: 0.89%
  - Azure: 0.98%
  - Speechmatics Linden-1: 1.11%
  - Cartesia Ink-2: 1.14%
  - Deepgram Nova-3: 1.32%
  - Gemini 3.5 Transcribe Live: 1.93%
  - GPT-Realtime-Whisper: 2.41%
  - Scribe v2 Realtime: 3.05%
  - Nemotron 3.5 multilingual: 4.54%
  - This is mostly English and not per-language. I read it on Soniox's site, which re-hosts it. — [Soniox benchmarks page](https://soniox.com/benchmarks)
- **Qwen-Audio-3.0-ASR (offline, vendor; arXiv 2609.07549v2, Sep 2026):**
  - FLEURS WER/CER for Qwen-Audio-3.0 vs Azure vs GPT-4o-Transcribe vs Gemini 3.1 Pro:
    - es: 1.23 / 2.06 / 2.13 / 2.62
    - pt: 1.87 / 2.59 / 2.80 / 3.46
    - ja (CER): 1.61 / 2.08 / 2.98 / 3.95
    - id: 2.89 / 3.12 / 3.73 / 3.72
    - vi: 4.95 / 2.79 / 3.23 / 4.70
    - th (CER): 6.73 / 4.65 / 4.99 / 6.05
    - ko (CER): 4.94 / 3.89 / 4.06 / 4.24
    - ms: 8.79 / N/A / 4.21 / 4.22
    - fil: 11.48 / N/A / 7.42 / 8.33
  - The streaming variant (Qwen-Audio-3.0-ASR-Streaming) reports zh CER 9.24% at 400 ms latency vs 8.03% offline, and en WER 13.38% vs 11.78% on the report's internal sets. No per-language streaming results for competitors. — [arXiv 2609.07549v2](https://arxiv.org/html/2609.07549v2)
- **NVIDIA Nemotron 3.5 ASR Streaming (self-reported FLEURS):** ES about 4.1%, PT 5.48%, 40 language-locales. — [HF model card](https://huggingface.co/nvidia/nemotron-3.5-asr-streaming-0.6b)

### Inferences
- **Per-language winners on FLEURS streaming (AssemblyAI data):**
  - Gemini 3.5 Live wins or ties on es, it, pt, nl, ru, tr, hi, vi and ko.
  - AssemblyAI 3.7 wins on fr, de and sv.
  - Soniox v6 wins on fi, he (tie) and is second on ar and hi.
  - Scribe v3 Live wins on da and ja, and ties on zh.
  - For most European languages the top four are within about 1-2 points of each other. That is likely inside the noise MT can absorb.
- **Real separators** are the languages where a provider falls clearly off:
  - AssemblyAI on vi (11-12% vs 5-6.5%) and ar (11.2% vs 6.7-6.8%).
  - Scribe v3 Live on hi (9.3% vs 5.0-6.2%).
  - Soniox on Nordic languages (sv 9.2%) and fr/nl (7.0/7.2%).
- **Soniox v6 is the most even across languages.** Its worst-vs-best gap against the leader is small everywhere: no outright cliffs, just a consistent 0.5-2 point lag on Western European languages. The v5 to v6 gain is about 0.6 points of mean WER.
- **Dataset sensitivity is large.** Gemini is best on FLEURS (read speech) but 7th on Common Voice. AssemblyAI is mid on FLEURS but best on Common Voice. Neither set is conversational, so do not over-weight a 1-point gap.

### Gaps
- No independent (non-vendor) per-language streaming benchmark across providers. Artificial Analysis publishes no per-language breakdown; its AA-WER is English-centric, with v2 dated 2026-02-17 ([AA](https://artificialanalysis.ai/articles/mai-transcribe-1-everything-you-need-to-know)).
- I could not recover per-language values for Speechmatics, Gladia, Cartesia or Deepgram Flux from the AssemblyAI page text.
- "Azure" in AssemblyAI's table is not identified as MAI-Transcribe-2-Streaming, so its 12.0 may reflect a classic Azure Speech model.
- No per-language data for Cartesia Ink-2, GPT-Realtime-Whisper (only "GPT Realtime"), or Nemotron beyond es/pt.
- Languages outside the 18 (pl, uk, id, th, bn, ur, sw, fa, ta, etc.) have no multi-vendor streaming data at all.
- Voice Arena runs a phone-call STT leaderboard on 7 languages (US English, Hindi, Bangla, pt-BR, Vietnamese, Romanian, Arabic). Secondary reports say Sarvam leads Hindi and Bangla and Microsoft leads Romanian and pt-BR. I could not fetch the primary page (403/unavailable). — [HF VoiceArena](https://huggingface-gw-kz.just-ai.com/VoiceArena)

## 2. Exact supported language lists (streaming)

### Takeaway
For streaming, the breadth ranking is: ElevenLabs Scribe v3 Live (about 90-102) > Gemini 3.5 Transcribe Live (85+, preview) > Soniox v6 (60) = MAI-Transcribe-2-Streaming (60, preview) > Deepgram Nova-3 multi (about 55) > Nemotron 3.5 (40 locales) > AssemblyAI 3.7 (32) > Qwen-Audio-3.0 (30 plus Chinese dialects) > Deepgram Flux Multilingual (10). AssemblyAI is missing several top-30 languages: Polish, Ukrainian, Indonesian, Thai, Bengali, Swahili, Tamil, Malay and Greek.

### Cited Findings
- **Soniox (RT and async, transcription and translation between any pair; 60 languages):** af, sq, ar, az, eu, be, bn, bs, bg, ca, zh, hr, cs, da, nl, en, et, fi, fr, gl, de, el, gu, he, hi, hu, id, it, ja, kn, kk, ko, lv, lt, mk, ms, ml, mr, no, fa, pl, pt, pa, ro, ru, sr, sk, sl, es, sw, sv, tl, ta, te, th, tr, uk, ur, vi, cy. Cantonese is not listed separately. — [Soniox supported languages](https://docs.soniox.com/stt/concepts/supported-languages)
- **AssemblyAI-STT-3.7-Realtime and -Agent (32):** Afrikaans, Arabic, Cantonese, Catalan, Danish, Dutch, English, Estonian, Finnish, French, Galician, German, Hebrew, Hindi, Italian, Japanese, Korean, Mandarin, Marathi, Norwegian (Bokmål and Nynorsk), Persian, Portuguese, Romanian, Russian, Spanish, Swedish, Turkish, Urdu, Vietnamese, Xhosa, Zulu. Auto-detection works across all of them. — [AssemblyAI models.md, 2026-11-17](https://www.assemblyai.com/llms/models.md)
- **Deepgram:**
  - Flux Multilingual (10): en, es, fr, de, hi, ru, pt, ja, it, nl.
  - Nova-3 multilingual codes include zh, zh-HK (Cantonese), ar, th, vi, id, ko, uk, pl, tr and bn.
  - Swahili and Urdu are monolingual Nova-3 only.
  - — [Deepgram models & languages](https://developers.deepgram.com/docs/models-languages-overview)
  - A changelog says `language=multi` now serves a 55-language single model for streaming and pre-recorded, with code-switching and per-word language tags. — [Deepgram changelog](https://developers.deepgram.com/changelog) (via search snippet; exact date not confirmed)
- **Google Gemini 3.5 Transcribe Live:** 85+ languages with auto-detect. Status is public preview (launched 2026-08-26). Model id is `gemini-3.5-transcribe-live` in the Gemini API and `gemini-3.5-transcribe-live-preview` on Vertex. — [Google blog](https://blog.google/innovation-and-ai/models-and-research/gemini-models/gemini-3-5-transcribe/); [Gemini API docs](https://ai.google.dev/gemini-api/docs/models/gemini-3.5-transcribe)
- **Microsoft MAI-Transcribe-2-Streaming:**
  - 60 languages with continuous automatic language detection.
  - Released 2026-10-01 as a public preview with no SLA ("not recommended for production").
  - Uses an OpenAI-Realtime-compatible WebSocket or Speech SDK 1.52+.
  - Price conflicts between sources: $0.36/hr (Microsoft model page, per snippet) vs $0.54/hr introductory (other source).
  - — [Microsoft MAI-Transcribe-2](https://microsoft.ai/models/mai-transcribe-2/); [testingcatalog](https://testingcatalog.com/microsoft-launches-mai-transcribe-2-streaming); [Azure catalog](https://ai.azure.com/catalog/models/MAI-Transcribe-2-Streaming)
- **ElevenLabs Scribe v3 Live:** evaluated on 102 languages, per vendor. Other sources say "90+" or "73" for the Live figure. — [ElevenLabs blog](https://elevenlabs.io/blog/scribe-v3); [letsdatascience](https://letsdatascience.com/news/elevenlabs-launches-scribe-v3-and-v3-live-speech-to-text-mod-82ce234a)
- **Qwen-Audio-3.0-ASR (30 + 16 Chinese dialect varieties):** zh, ja, ko, vi, th, id, ms, tl, hi, ar, en, fr, de, es, pt, ru, it, nl, sv, da, fi, el, pl, cs, hu, ro, bg, hr, sk, no. — [arXiv 2609.07549v2](https://arxiv.org/html/2609.07549v2)
- **Nemotron 3.5 ASR Streaming:** 40 language-locales. — [MarkTechPost](https://www.marktechpost.com/2026/06/06/nvidia-releases-nemotron-3-5-asr-a-600m-parameter-cache-aware-streaming-model-transcribing-40-language-locales-in-real-time/)

### Inferences
- **Coverage of a practical top-35 list** (en, zh, hi, es, fr, ar, bn, pt, ru, ur, id, de, ja, sw, mr, te, tr, ta, vi, ko, it, th, fa, pl, uk, nl, ro, el, cs, hu, sv, he, ms, tl, yue):
  - Soniox v6 covers all except Cantonese as a separate language.
  - AssemblyAI 3.7 misses about 13 (bn, id, sw, te, ta, th, pl, uk, el, cs, hu, ms, tl).
  - Flux Multilingual covers 7.
- If "any language" is a real product requirement, AssemblyAI cannot be the single provider. Soniox, Scribe v3 Live, Gemini Live and MAI-Transcribe-2 are the only realistic single-provider candidates. Of these, Soniox is the only GA one that also bundles translation.

### Gaps
- Full Scribe v3 Live and Gemini 3.5 Live language tables were not fetched.
- The exact list of Deepgram's 55-language multi model is not confirmed.
- No language list was found for Speechmatics, Gladia Solaria or Cartesia Ink-2 (Speechmatics and Gladia are known to cover 50+/100+ batch, but I did not verify streaming).

## 3. Automatic language ID and code-switching in streaming

### Takeaway
Every leading model now offers in-stream auto language ID, so the speaker's language does not need to be known upfront. Measured code-switching quality, in AssemblyAI's vendor-run 5-pair EN↔{ES,DE,FR,IT,PT} streaming test, ranks AssemblyAI 3.7 first (6.67%), then Scribe v3 Live (8.32%), AWS (9.13%) and Gemini 3.5 Live (10.16%), with OpenAI GPT Realtime far behind (37.35%). Soniox was not shown in that table. Real conversational code-switching (the Miami ES-EN set) is much harder for everyone, at 24-41%.

### Cited Findings
- **Realtime code-switching (AssemblyAI report, runs 2026-08-20 to 2026-11-14, vendor-run; WER):** — [AssemblyAI benchmarks](https://www.assemblyai.com/benchmarks)

  | System | 5-pair mean | EN↔ES | EN↔DE | EN↔FR | EN↔IT | EN↔PT | Miami conv. ES-EN |
  |---|---|---|---|---|---|---|---|
  | AssemblyAI-STT-3.7-Realtime | 6.67 | 9.16 | 5.46 | 6.58 | 5.53 | 6.61 | 24.5 |
  | AssemblyAI U3.6 Pro RT | 7.20 | 9.59 | 6.13 | 7.23 | 6.03 | 7.00 | 24.3 |
  | AssemblyAI 3.7-Realtime-Agent | 7.49 | 10.74 | 6.57 | 6.93 | 6.23 | 6.96 | 28.9 |
  | ElevenLabs Scribe v3 Live | 8.32 | 10.84 | 7.65 | 8.32 | 7.19 | 7.57 | 25.7 |
  | AWS Transcribe Streaming | 9.13 | 11.33 | 8.12 | 9.33 | 7.05 | 9.81 | 28.0 |
  | Gemini 3.5 Transcribe Live | 10.16 | 14.40 | 8.29 | 10.05 | 8.25 | 9.79 | 40.7 |
  | OpenAI GPT Realtime | 37.35 | 39.35 | 35.46 | 37.87 | 36.30 | 37.77 | 53.3 |

  The table covered 24 systems; Soniox's row was not in the portion I could read. The pair sets are built from public read-speech corpora.
- **Independent code-switching study (ServiceNow on HF, 2026-06-09; ES-EN, FR-EN, CA-FR-EN, DE-EN; synthetic TTS audio; streaming vs batch mode not stated):** — [HF blog](https://huggingface.co/blog/ServiceNow-AI/code-switching)
  - Scribe v2 and AssemblyAI Universal-3 Pro tie on ES-EN, and Scribe is narrowly ahead on the other pairs. Scribe v2 ranks first on semantic metrics.
  - Scribe v2, Gemini 3 Flash and AssemblyAI show the smallest code-switching penalties.
  - Deepgram Nova-3 Multi is last or second-to-last on semantic error.
  - Errors concentrate on the embedded English spans.
  - Whisper without a language parameter tends to translate into English instead of transcribing.
- **Soniox:**
  - Per-token `language` tags are enabled with `enable_language_identification`. The model aims for sentence-level coherence, so an embedded foreign word stays tagged with the sentence language.
  - In real time, a language "may be temporarily misclassified" and tags can be revised.
  - Language hints are recommended to improve accuracy, and there is a separate "language restrictions" feature to avoid unwanted languages.
  - — [Soniox language identification](https://docs.soniox.com/stt/concepts/language-identification)
  - The v6 announcement claims improved language ID and mid-sentence switching, with no numbers. — [Soniox v6 blog](https://soniox.com/blog/soniox-stt-v6)
- **AssemblyAI:** 3.7-Realtime auto-detects across 32 languages in one stream, and AssemblyAI says it can replace a "language router in front of two streaming integrations". 3.6 Pro had German-English issues in early access, and its docs conflict on whether `language_code(s)` constrains it. — [AssemblyAI models.md](https://www.assemblyai.com/llms/models.md)
- **Deepgram:** `language=multi` gives code-switching with per-word language tags, and 100 ms endpointing is recommended. Flux Multi takes `language_hint`. — [Deepgram code-switching docs](https://developers.deepgram.com/docs/multilingual-code-switching)
- **Gemini 3.5 Live:** auto-detects 85+ languages, handles "live language switches", and accepts BCP-47 codes to bias recognition. — [Google blog](https://blog.google/innovation-and-ai/models-and-research/gemini-models/gemini-3-5-transcribe/); [Gemini Live transcription docs](https://ai.google.dev/gemini-api/docs/live-api/live-transcribe?hl=en)
- **MAI-Transcribe-2-Streaming:** continuous automatic language detection without specifying the language upfront. — [testingcatalog](https://testingcatalog.com/microsoft-launches-mai-transcribe-2-streaming)

### Inferences
- No source measures language-ID accuracy itself, such as time-to-correct-language or the first-utterance misclassification rate. That is the key risk for an "unknown language" caller.
- **Mitigation:** the product knows each participant's profile or UI language. Pass it as a language hint or restriction (Soniox `language_hints`, Deepgram `language_hint`, Gemini BCP-47) plus English. That turns open-set language ID into a 2-3-way choice, which is much more reliable.
- **Cost of misclassification:** each participant's stream is mostly monolingual, so code-switching matters less than correct language ID at the start of a turn. A misdetected first utterance produces garbage that MT and TTS then voice.

### Gaps
- No independent streaming language-ID accuracy benchmark.
- No code-switching data for non-European pairs (hi-en "Hinglish", zh-en, ar-en, tl-en), which are the most common real code-switching cases.
- Soniox's code-switching score is not visible in the data retrieved.

## 4. Tonal and low-resource language performance

### Takeaway
Among streaming providers there is decent evidence only for zh, ja, ko, vi, ar, he and hi (the AssemblyAI table). On those, Gemini 3.5 Live, Soniox v6 and Scribe v3 Live are close, and AssemblyAI is weak on vi and ar. Hebrew is bad for everyone (18-21%). For th, yue, sw, ur, bn, id, ta and Arabic dialects there is no multi-vendor streaming data. Offline, Qwen-Audio-3.0 is strongest on ja/ko/id/es/pt/th (Common Voice) and Azure on vi/th (FLEURS).

### Cited Findings
- **Streaming FLEURS** (Gemini 3.5 Live / Scribe v3 Live / Soniox v6 / AssemblyAI 3.7 RT / Deepgram Nova-3), all from the AssemblyAI report — [AssemblyAI benchmarks](https://www.assemblyai.com/benchmarks):
  - zh CER: 5.7 / 5.5 / 5.6 / 7.3 / 10.7
  - vi WER: 5.2 / 5.9 / 6.5 / 11.9 / 15.2
  - ar WER: 6.7 / 7.6 / 6.8 / 11.2 / 22.9
  - hi WER: 5.0 / 9.3 / 6.2 / 7.3 / 20.4
  - he WER: 18.9 / 20.7 / 18.0 / 18.5 / 30.2
- **Offline FLEURS** (Qwen-Audio-3.0 / Azure / GPT-4o-Transcribe / Gemini 3.1 Pro):
  - th CER: 6.73 / 4.65 / 4.99 / 6.05
  - vi WER: 4.95 / 2.79 / 3.23 / 4.70
  - id WER: 2.89 / 3.12 / 3.73 / 3.72
  - Common Voice th CER: Qwen 1.56 vs Azure 2.39 vs Gemini 12.33
  - — [arXiv 2609.07549v2](https://arxiv.org/html/2609.07549v2)
- **Qwen-Audio-3.0** covers 16 Chinese dialect varieties across eight dialect regions, making it the only source found with explicit dialect coverage (relevant to Cantonese and other varieties). — [arXiv 2609.07549v2](https://arxiv.org/html/2609.07549v2)
- **ElevenLabs Scribe v3 (batch)** improved Swahili by 34%, Tamil by 29% and Thai by 28% vs v2. These are relative reductions with no absolute WER, per vendor. — [ElevenLabs blog](https://elevenlabs.io/blog/scribe-v3)
- **Cantonese support:** AssemblyAI 3.7 lists it. Deepgram Nova-3 has `zh-HK`. Soniox does not list it separately. — [AssemblyAI models.md](https://www.assemblyai.com/llms/models.md); [Deepgram](https://developers.deepgram.com/docs/models-languages-overview); [Soniox](https://docs.soniox.com/stt/concepts/supported-languages)
- **Hindi and Bangla on real phone calls (Voice Arena):** Sarvam (an Indic specialist) reportedly leads. This is a secondary report and was not verified. — [search summary; HF VoiceArena](https://huggingface-gw-kz.just-ai.com/VoiceArena)
- **HF Open ASR Leaderboard:** its multilingual tab is still mostly European and offline. It recently added Hindi (Monsoon hi-IN) scored with OIWER, but no Swahili, Bengali, Urdu, Arabic or Southeast Asian results. — [HF blog source](https://raw.githubusercontent.com/huggingface/blog/main/open-asr-leaderboard-global-south.md)

### Inferences
- For CJK, the top streaming providers are effectively tied (zh about 5.5-5.7, ja 3.4-3.9, ko 2.3-2.4), so choose on other factors.
- For vi and ar, avoid AssemblyAI and Deepgram.
- For hi, avoid Scribe v3 Live and Deepgram, and consider Sarvam if Hindi or Bangla is a key market.
- For th, sw, ur, bn, ta and id, the evidence is a vacuum. Soniox, Scribe and Gemini all claim support, so a small in-house test set (30-60 min per language) is the only way to decide.

### Gaps
- No streaming data for Thai, Cantonese, Swahili, Urdu, Bengali, Tamil, Indonesian, Polish or Ukrainian across vendors.
- No Arabic dialect (Egyptian, Gulf, Levantine, Maghrebi) streaming data. FLEURS Arabic is mostly MSA-like read speech.
- Sarvam's streaming API, languages and pricing were not researched.

## 5. Routing strategy: best STT per language?

### Takeaway
Routing is technically practical, because every candidate is JSON over WebSocket and the participant's language is usually known from their profile. On current evidence the per-language gains over Soniox v6 are mostly 0.5-2 WER points on European languages, which MT largely absorbs. Routing pays off only for specific weak spots: Hebrew and Nordic languages, plus offline-benchmark leaders for Thai and Vietnamese. A single provider (Soniox v6) with an optional "premium route" for a few high-value languages is the pragmatic design.

### Cited Findings
- AssemblyAI pitches 3.7-Realtime's in-stream language ID as a way to retire a "language router in front of two streaming integrations", which implies routers are a real production pattern. — [AssemblyAI models.md](https://www.assemblyai.com/llms/models.md)
- Per-language spreads among the top four streaming systems on FLEURS are mostly 0.5-2 points. The exceptions are the vi, ar and hi gaps noted in section 4. — [AssemblyAI benchmarks](https://www.assemblyai.com/benchmarks)
- Prices per stream-hour (from the prior notes):
  - Soniox: $0.12 with translation included
  - AssemblyAI 3.7: $0.30
  - Scribe v3 Live: about $0.22, unverified
  - Deepgram Flux Multi: about $0.47
  - Gemini 3.5 Live: pricing not found
  - — [prior notes](../../2026-11-voice-translation-stack/notes/streaming-stt.md)

### Inferences
- **Routing design:**
  - Choose the STT per participant stream at join time, keyed on the declared or profile language. Fall back to auto language ID within the first N seconds, then re-route on a confident mismatch.
  - Keep a Rust `SttProvider` trait. Each route adds a vendor contract, a WebSocket client, a monitoring dimension and an endpointing behavior to tune. In particular, turn and finalization latency differs per vendor, which changes the feel of the call per language.
  - Routing also breaks Soniox's bundled translation for routed languages, which then need a separate MT hop.
- **Routing table, based on the vendor-run FLEURS and Common Voice streaming data plus language coverage** (single best and fallback):

  | Language group | Best streaming STT | Fallback | Evidence note |
  |---|---|---|---|
  | EN | AssemblyAI 3.7 / U3.6 Pro (Coval 2.2%) or Soniox v6 (Pipecat 0.74%) | Scribe v3 Live | English independent benchmarks conflict; all top-tier |
  | ES, PT (pt-BR), IT | Gemini 3.5 Live (FLEURS best) or AssemblyAI 3.7 (CV best) | Soniox v6 (+0.5-1.3 pts) | Gaps under 1.5 pts; ES/PT discriminate poorly (arXiv 2609.36920) |
  | FR, DE, NL | AssemblyAI 3.7 (fr 4.7, de 4.8) | Gemini 3.5 Live / Scribe v3 Live | Soniox 1.5-2.3 pts behind |
  | Nordic (sv, da, fi, no) | AssemblyAI 3.7 for sv, Scribe v3 Live / Gemini for da, Soniox v6 for fi | Scribe v3 Live | Soniox weak on sv (9.2) |
  | RU, TR | Gemini 3.5 Live (5.3 / 5.3) | Soniox v6 (6.6 / 6.8) | AssemblyAI weaker on tr (8.2) |
  | PL, UK, CS, EL, HU, RO, other Slavic/Balkan | Soniox v6 | Scribe v3 Live | AssemblyAI lacks pl/uk/cs/el/hu; no per-language data |
  | Hebrew | Soniox v6 (18.0) | AssemblyAI 3.7 Agent (18.0) | All poor (18-21%) |
  | Arabic (MSA) | Gemini 3.5 Live (6.7) / Soniox v6 (6.8) | Scribe v3 Live (7.6) | Avoid AssemblyAI (11.2), Deepgram (22.9); dialects unmeasured |
  | Hindi / Urdu / Marathi | Gemini 3.5 Live (hi 5.0) / Soniox v6 (6.2) | AssemblyAI 3.7 (7.3); consider Sarvam for hi/bn | Avoid Scribe v3 Live for hi (9.3) |
  | Bengali, Tamil, Telugu, other Indic | Soniox v6 (coverage) | Gemini 3.5 Live / Sarvam | No streaming data |
  | Mandarin (zh) | Scribe v3 Live / Soniox v6 / Gemini (5.5-5.7 CER, tie) | Qwen-Audio-3.0-Streaming (self-host / Alibaba) | AssemblyAI 7.3 |
  | Cantonese (yue) | AssemblyAI 3.7 (explicit support) or Gemini 3.5 Live | Qwen-Audio-3.0 (dialects) / Deepgram zh-HK | Soniox doesn't list yue; no WER data |
  | Japanese, Korean | Scribe v3 Live (ja 3.4) / Gemini (ko 2.3) | Soniox v6 (3.9 / 2.4) | All within 0.5 pt |
  | Vietnamese | Gemini 3.5 Live (5.2) | Soniox v6 (6.5) | Avoid AssemblyAI (11.9) |
  | Thai, Indonesian, Malay, Tagalog | Soniox v6 or Gemini 3.5 Live (coverage; no streaming data) | Scribe v3 Live | Offline leaders: Azure (th/vi), Qwen (id); AssemblyAI lacks all four |
  | Persian | Soniox v6 / AssemblyAI 3.7 | Scribe v3 Live | No data |
  | Swahili, African languages (zu, xh, af, yo, am) | Soniox v6 (sw) / AssemblyAI 3.7 (zu, xh, af) | Scribe v3 Live / Gemini (broadest lists) | No data |

- **Gemini 3.5 Live is best on FLEURS for many languages but is public preview,** with 7th place on Common Voice and the worst conversational code-switching score among the leaders (40.7% on Miami). Treat it as a candidate to watch rather than a production route today.

### Gaps
- No pricing found for Gemini 3.5 Transcribe Live.
- No GA date for MAI-Transcribe-2-Streaming.
- No per-language finalization latency data, which matters as much as WER for routing.
- All routing evidence rests on one vendor-run benchmark (AssemblyAI's) with read-speech sets. The table should be validated with in-house conversational audio before shipping routes.

## Overall recommendation (single provider vs routed)

### Takeaway
**Single provider: Soniox stt-rt-v6.** It is the broadest GA streaming coverage with bundled translation (60 languages), it is near the top in every language with data (FLEURS mean 6.9 vs leader 6.0, with no cliffs), it is the cheapest ($0.12/hr), and it placed 1st in Pipecat's November 2026 realtime test. **Routed: keep Soniox as default and add AssemblyAI-STT-3.7-Realtime for FR/DE/NL/SV and the EN↔European code-switching case, if the 1.5-2.5 point gain justifies a second vendor and a separate MT hop for those streams.** Re-evaluate Gemini 3.5 Transcribe Live (best FLEURS mean) and MAI-Transcribe-2-Streaming once they leave preview.

### Cited Findings
- **FLEURS streaming means:** Gemini 3.5 Live 6.0, Scribe v3 Live 6.8, Soniox v6 6.9, AssemblyAI 3.7 RT 7.5. **Common Voice means:** AssemblyAI 3.7 RT 6.8, Scribe v3 Live 7.4, Soniox v6 8.4, Gemini 8.9. — [AssemblyAI benchmarks (vendor-run)](https://www.assemblyai.com/benchmarks)
- **Language counts:** Soniox 60, AssemblyAI 3.7 32, Flux Multi 10, Gemini Live 85+ (preview), MAI-Transcribe-2-Streaming 60 (preview). — [Soniox](https://docs.soniox.com/stt/concepts/supported-languages); [AssemblyAI](https://www.assemblyai.com/llms/models.md); [Deepgram](https://developers.deepgram.com/docs/models-languages-overview); [Google](https://blog.google/innovation-and-ai/models-and-research/gemini-models/gemini-3-5-transcribe/); [testingcatalog](https://testingcatalog.com/microsoft-launches-mai-transcribe-2-streaming)
- **Pipecat realtime semantic WER (Nov 2026):** Soniox v6 0.74% (1st of 23). — [Soniox benchmarks page](https://soniox.com/benchmarks)

### Inferences
- **Strongest single-provider challenger: ElevenLabs Scribe v3 Live.** It has the second-best FLEURS mean, the second-best code-switching score, and the broadest language list (about 100). Its weaknesses are Hindi, the lack of bundled translation, and unverified pricing. If the voice-clone TTS is ElevenLabs, a one-vendor deal is plausible.
- **Do not use as the single provider:**
  - AssemblyAI 3.7, which lacks 13 of the top-35 languages and is weak on vi and ar.
  - Deepgram, whose multilingual means are 12.5-15.0.
  - OpenAI GPT Realtime, with 37% code-switching WER.
- **Validation plan:** build a 30-60 minute conversational test set for each of about 10 priority languages (pt-BR, es, en, hi, ar, zh, vi, id, th, ru). Run Soniox v6, Scribe v3 Live, AssemblyAI 3.7 and Gemini 3.5 Live, and measure WER/CER, language-ID-first-utterance accuracy and finalization latency. Add routes only where a provider beats Soniox by more than about 2 points.

### Gaps
- Every per-language number for Soniox, Scribe, Gemini and AssemblyAI comes from one vendor (AssemblyAI). An independent multilingual streaming benchmark does not exist as of 2026-11-25.
