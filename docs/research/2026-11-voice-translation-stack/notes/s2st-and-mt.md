# End-to-End Speech-to-Speech Translation (S2ST) vs Cascade, and the MT Layer: State as of November 2026

Scope: a live two-party video-call app (Meet-like, anonymous join by link). Each listener picks the language they hear. Translated speech should sound like the original speaker.
Research date: 2026-11-25. Every claim is tagged **[vendor]** (vendor-stated), **[3rd-party]** (secondary or aggregator), **[independent]** (peer-reviewed or academic eval) or **[vendor-run benchmark]** (a vendor tested itself against competitors).

---

## Q1. Which end-to-end or managed S2ST options exist as of Nov 2026, which preserve the speaker's voice, and how are they licensed?

### Takeaway
Two hyperscaler models arrived in 2026, each a single API call that takes audio in and returns translated audio:
- **Google Gemini 3.5 Live Translate** (preview, June 2026). It preserves prosody and pitch. It also powers Google Meet's 63-language translation.
- **OpenAI gpt-realtime-translate** (May 2026, $0.034/min). There is no documented voice preservation.

Both cost about $0.035/min. Neither guarantees an identity-preserving clone. For open weights, Kyutai Hibiki-Zero (Feb 2026, 3B) has the strongest published speaker-similarity numbers, but it only translates into English. Meta Seamless is non-commercial. Most other vendors (Speechmatics, Soniox, AssemblyAI, Gladia) offer text-only translation, and you add TTS yourself.

### Cited Findings

**Google, Gemini 3.5 Live Translate and Google Meet**
- Released 2026-06-09 as `gemini-3.5-live-translate-preview`, a public preview via the Gemini Live API (WebSocket) and AI Studio. No GA date and no SLA. [vendor via 3rd-party] — [ChatForest builder guide](https://chatforest.com/builders-log/gemini-3-5-live-translate-speech-to-speech-70-languages-live-api-builder-guide/); [LiveLingo guide](https://www.livelingo.io/guides/gemini-3-5-live-translate)
- It covers 70+ languages and auto-detects the source language, so only the target is set (`targetLanguageCode`). Each session translates one direction. Audio is 16 kHz PCM in, 24 kHz PCM out, sent in roughly 100 ms chunks. Optional text transcripts are available. All output carries a SynthID watermark. [3rd-party] — [ChatForest](https://chatforest.com/builders-log/gemini-3-5-live-translate-speech-to-speech-70-languages-live-api-builder-guide/); [Slator](https://slator.com/google-ai-live-speech-translation-gemini-3-5-live-translate/)
- **Voice:** it is audio-to-audio and carries the source speaker's pitch, intonation and pacing into the output. [vendor] A third-party guide warns that it "should not be treated as a guaranteed identity-preserving voice clone" [3rd-party], and Google itself says voice replication "can be inconsistent". — [ChatForest](https://chatforest.com/builders-log/gemini-3-5-live-translate-speech-to-speech-70-languages-live-api-builder-guide/); [AssemblyAI comparison, 2026-11-18](https://www.assemblyai.com/blog/best-real-time-speech-translation-apis)
- **Preview pricing:** input audio $3.50/M tokens (32 tok/s, about $0.0053/min) plus output audio $21/M tokens (25 tok/s, about $0.0315/min), about **$0.0368/min** in total. [3rd-party reading of the Google rate card] — [ChatForest](https://chatforest.com/builders-log/gemini-3-5-live-translate-speech-to-speech-70-languages-live-api-builder-guide/); [AssemblyAI](https://www.assemblyai.com/blog/best-real-time-speech-translation-apis)
- **Session limits:** 131k input-token context, so about 68 minutes of audio. Live API audio sessions default to a 15-minute cap unless extended. [3rd-party] — [ChatForest](https://chatforest.com/builders-log/gemini-3-5-live-translate-speech-to-speech-70-languages-live-api-builder-guide/)
- **Google Meet (2026-10-22):** speech translation now covers any combination of 63 languages, on web and mobile, using Gemini 3.5 Live Translate. It is not available in live streams, recordings, CSE meetings, dial-in or interop calls, or with adaptive audio. [vendor] — [Google Workspace Updates](https://workspaceupdates.googleblog.com/2026/10/real-time-speech-translation-in-google-Meet-expands-to-63-languages-with-automatic-language-detection.html)
- **Earlier Meet status:** EN↔ES/FR/DE/PT/IT reached GA for business customers in early 2026. Sources disagree on the date (Jan 27 vs Feb 4). Plan caps are reported at 90 min per meeting on base plans and up to 8 h on higher tiers. [3rd-party, unconfirmed] — [search summary of pasqualepillitteri.it / jotme.io](https://pasqualepillitteri.it/en/news/30355/google-meet-speech-translation-63-languages)

**OpenAI, gpt-realtime-translate**
- Launched 2026-05-07 alongside gpt-realtime-2 and gpt-realtime-whisper. It translates 70+ input languages into **13 output languages** "while keeping pace with the speaker". **$0.034/min**, billed by audio duration. [vendor] — [OpenAI announcement](https://openai.com/index/advancing-voice-intelligence-with-new-models-in-the-api/); [model page](https://developers.openai.com/api/docs/models/gpt-realtime-translate)
- It auto-detects the source language and returns translated audio plus transcript deltas while source audio is still arriving. It runs on a dedicated endpoint. [vendor] — [OpenAI cookbook translation guide](https://developers.openai.com/cookbook/examples/voice_solutions/realtime_translation_guide)
- **Voice:** the announcement says nothing about preserving the speaker's voice. [vendor] A third-party benchmark reports no speaker attribution and no voice selection. [vendor-run benchmark] — [OpenAI](https://openai.com/index/advancing-voice-intelligence-with-new-models-in-the-api/); [LiveLingo benchmark](https://www.livelingo.io/research/benchmark-2026)
- OpenAI publishes no formal quality or latency numbers. The only cited evidence is a customer quote: BolnaAI saw "12.5% lower WER" on Hindi, Tamil and Telugu. [vendor] — [OpenAI](https://openai.com/index/advancing-voice-intelligence-with-new-models-in-the-api/)
- Also available on Microsoft Foundry (Global Standard). [vendor] — [Microsoft Learn](https://learn.microsoft.com/en-us/azure/foundry/openai/concepts/gpt-realtime-translate)
- **Companion pricing:** gpt-realtime-whisper streaming STT costs $0.017/min. gpt-realtime-2 costs $32/M audio-in tokens and $64/M audio-out tokens. [vendor] — [OpenAI](https://openai.com/index/advancing-voice-intelligence-with-new-models-in-the-api/)

**Kyutai Hibiki and Hibiki-Zero (open weights)**
- Hibiki-Zero (arXiv 2602.11072, Feb 2026; ICML 2026) has 3B parameters and is decoder-only and multistream (Mimi codec, 12.5 Hz). It translates FR/ES/PT/DE into **English only**, and Italian was added with under 1,000 h of data. It does voice transfer and outputs timestamped text. Training uses sentence-level alignment plus GRPO RL, with no word-level alignment needed. [independent/academic] — [arXiv](https://arxiv.org/html/2602.11072v1); [HF model card](https://huggingface.co/kyutai/hibiki-zero-3b-pytorch-bf16); [ICML poster](https://icml.cc/virtual/2026/poster/66094)
- Real-time on GPU with batching, but no GPU model or throughput figures are given. [academic] — [arXiv](https://arxiv.org/html/2602.11072v1)
- NaturalFlow (arXiv 2606.13121, 2026) builds on Hibiki to reduce disruptive pauses in simultaneous S2ST output. — [arXiv](https://arxiv.org/pdf/2606.13121)

**Meta Seamless (SeamlessStreaming / SeamlessExpressive)**
- SeamlessM4T and SeamlessStreaming weights are licensed CC-BY-NC 4.0. SeamlessExpressive (the voice- and prosody-preserving model) is under the "Seamless License", which allows only "Noncommercial Research Uses" and bars using outputs commercially. **It cannot be used in a commercial product.** — [Meta Seamless License](https://ai.meta.com/resources/models-and-libraries/seamless-license/); [HF seamless-streaming card](https://huggingface.co/facebook/seamless-streaming/blob/main/README.md?code=true)
- On Hibiki-Zero's benchmark, Seamless scores far lower on human-rated audio quality, speaker similarity and naturalness (see Q2). [academic] — [arXiv](https://arxiv.org/html/2602.11072v1)

**Microsoft Azure Live Interpreter**
- Live Interpreter includes **personal voice**, meaning output in the speaker's own voice. [3rd-party] — [AssemblyAI](https://www.assemblyai.com/blog/best-real-time-speech-translation-apis)
- **Reported per-component pricing:** input audio about $1.00/h, standard audio output $1.50/h, custom-voice audio output $2.00/h, text output $10/M chars. The older S1 speech translation tier is $2.50/h for up to 2 targets. Personal voice requires Microsoft approval (limited access). [3rd-party, not confirmed against the official page] — [search summary of afeayazilim.com / blocksentient.com](https://afeayazilim.com/en/blog-detail/microsofts-new-live-interpreter-api); [GitHub demo](https://github.com/honestypugh2/live-interpreter-api-demo)

**Other vendors**
- **Palabra.ai:** claims under 1 s for two-way speech translation, 60–70+ languages, voice cloning from about 3 s of audio, WebRTC and WebSocket APIs, and **$0.04/min** for API speech-to-speech. [vendor] Third-party sources list conflicting plan prices ($29–$150/month). — [Palabra](https://www.palabra.ai/); [Palabra TTS](https://www.palabra.ai/text-to-speech); [AssemblyAI](https://www.assemblyai.com/blog/best-real-time-speech-translation-apis)
- **DeepL Voice API:** GA on 2026-02-02 for API Pro. It translates into up to 5 targets from about 45–48 languages. STT is GA, speech-to-speech is in **beta**, pricing is contract-based and unpublished, and voice preservation is not mentioned. [vendor] — [DeepL press release](https://www.deepl.com/en/press-release/deepl_launches_voice_api_for_real_time_speech_transcription_and_translation); [AWS Marketplace](https://aws.amazon.com/marketplace/pp/prodview-ocvlpuamzgr36)
- **Text-only translation (no TTS):** [vendor pricing via 3rd-party] — [AssemblyAI](https://www.assemblyai.com/blog/best-real-time-speech-translation-apis)
  - Speechmatics: about 34 languages, $0.65/h add-on (about $1.10/h in total). Its docs warn translations can take up to 5 s to finalize.
  - Soniox: 60 languages, any pair, $2/M audio-in tokens and $4/M text-out tokens.
  - Gladia: €0.05/h add-on.
  - AssemblyAI: $0.30/h plus LLM Gateway, with translation about 100 ms after the final turn.
- **Krisp Voice Translation:** 61 languages, any pair, $3–4/h, sub-second claimed. [vendor] — [AssemblyAI](https://www.assemblyai.com/blog/best-real-time-speech-translation-apis)

### Inferences
- Gemini 3.5 Live Translate is the only managed E2E option at commodity price (about $0.037/min) that claims voice and prosody carry-over across 70+ languages. Google's own product (Meet) uses it, which suggests it is good enough for production calls.
- OpenAI's model is the same price but supports only 13 output languages and has no voice preservation. For "sound like the speaker" it is effectively a fixed-voice translator.
- The only managed products that do genuine voice cloning are Azure (personal voice, gated and per-hour pricing) and Palabra (cloning, $0.04/min). Otherwise you build a cascade with a cloning TTS.
- Open weights are not viable for production today. Hibiki-Zero is X→EN only (no EN→X), and Seamless is non-commercial.

### Gaps
- I did not verify official pricing pages for ElevenLabs real-time translation/dubbing, Sanas, Timekettle, Kudo, Wordly, Zoom or Teams. No source was retrieved.
- Hibiki-Zero's weights license is not stated in the paper. Earlier Kyutai releases used permissive licenses, but I did not confirm this, so check the HF card.
- I found no evidence of a Hibiki-Zero EN→X model or a 2026 Meta Seamless successor.
- The Azure Live Interpreter official price page was not confirmed. Whether personal voice bills at the $2/h custom rate is ambiguous.

---

## Q2. Benchmarks: ASR-BLEU/COMET, latency, speaker similarity, naturalness MOS, and independent comparisons

### Takeaway
The only rigorous S2ST benchmark with speaker similarity and MOS is Kyutai's (academic, open systems only). There, Hibiki-Zero beats Seamless decisively on voice similarity and naturalness at a LAAL of about 3 s. For commercial APIs, the only head-to-head is a small, vendor-run LiveLingo test (June 2026):
- Gemini 3.5 Live Translate: about 3 s utterance-end to audio, quality 4.93/5.
- OpenAI gpt-realtime-translate: 0.7 s to first audio but lag balloons on dense speech, quality 4.53/5.
- A Whisper + GPT-4o-mini cascade: 2.7 s.

No independent benchmark of commercial S2ST voice similarity exists.

### Cited Findings

**Hibiki-Zero paper (arXiv 2602.11072), short-form / long-form results** [academic, authors' own system vs baselines] — [arXiv](https://arxiv.org/html/2602.11072v1)

| System | ASR-BLEU (short / long) | Speaker sim. (short / long) | LAAL (short / long) | End offset |
|---|---|---|---|---|
| Hibiki-Zero FR→EN | 34.6 / 28.7 | 49.5 / 61.3 | 2.8 s / 6.1 s | 2.1 s / 2.3 s |
| Hibiki-Zero ES→EN | 33.9 / 31.5 | 57.0 / 64.6 | 3.1 s / 5.6 s | — |
| Seamless FR→EN | 32.8 / 23.9 | 19.1 / 44.4 | 2.8 s / 6.2 s | 2.4–3.2 s across languages |
| Hibiki (2025) FR→EN | 31.8 / 26.4 | 35.7 / 52.8 | 3.5 s / 6.8 s | 2.5 s / 2.6 s |

- Hibiki-Zero COMET is 78.9–82.0 on short-form and 42.3–43.7 on long-form.
- **Human MOS (0–100), FR→EN:** Hibiki-Zero scored 64.5 on audio quality, 70.0 on speaker similarity and 67.2 on naturalness. Seamless scored 11.4, 21.1 and 21.2. Hibiki (2025) scored 62.9, 44.7 and 57.0. German Hibiki-Zero scored 73.5, 65.3 and 69.9. [academic]
- **LiveLingo Real-Time Voice Translation Benchmark** (published 2026-06-04, addenda June 9–10). [vendor-run benchmark] — [LiveLingo](https://www.livelingo.io/research/benchmark-2026)
  - Set-up: three 120 s VOA clips (zh/es/pt→en) plus 120 authored utterances (en→es/zh/ja/de), scored by LLM judges on a 0–5 scale.
  - Median latency after the speaker stops:
    - LiveLingo: 1,518 ms.
    - Whisper-large + GPT-4o-mini cascade: 2,720 ms.
    - Azure: 4,755 ms.
    - Google STT v2 + Translate v3 chain: 26,736 ms, caused by `latest_long` batch finalization.
    - **Gemini 3.5 Live Translate:** about 3.1 s to finished audio, and 2,947 ms to first audio.
    - **OpenAI gpt-realtime-translate:** about 3.8 s to finished audio but **711 ms to first audio**. Lag grew to **20.3 s on dense audio**.
  - Comprehension (0–5): LiveLingo 4.96, **Gemini 4.93**, Google chain 4.77, Azure 4.65, Whisper + GPT-4o-mini 4.63, **OpenAI 4.53**.
  - Stability (erasures per 120 s): Whisper + GPT-4o-mini 22, Azure 121, Google chain 353.
  - **Code-switching:** when the zh clip switched to English at 86 s, both Gemini and OpenAI stopped translating, leaving 34 s untranslated.
  - Gemini voice replication "can drift in multi-speaker audio". OpenAI has no voice selection.
- **IWSLT and other academic work:** IWSLT simultaneous-track data in the search results was from 2024 (findings, arXiv 2411.05088), and I did not extract it. — [IWSLT 2024 findings](https://arxiv.org/pdf/2411.05088)

### Inferences
- **Latency:** E2E systems do not dramatically beat a well-built cascade on "speaker stops → translation done". Both sit around 2.5–3.5 s in the only commercial comparison. E2E wins on time-to-first-audio, at least OpenAI does.
- **Quality:** Gemini 3.5 Live Translate scores at the top on comprehension. OpenAI's translate model is weakest and drifts on continuous speech, which is a risk for monologue-heavy calls.
- **Voice similarity:** Kyutai's data shows modern E2E models can reach a human-rated speaker similarity of about 65–70/100. No published number exists for Gemini, Azure personal voice or Palabra cloning, so this is an open evaluation question you must test yourself.

### Gaps
- No independent (non-vendor) head-to-head of Gemini 3.5 Live Translate vs gpt-realtime-translate vs Palabra vs Azure.
- No speaker-similarity metric for any commercial system.
- I did not retrieve IWSLT 2025 or 2026 simultaneous-track results.

---

## Q3. MT layer for the cascade: LLMs vs DeepL vs Google; latency, quality, cost and simultaneous strategies

### Takeaway
At WMT25 (2025), human evaluation ranked Gemini 2.5 Pro first overall. GPT-4.1 and Claude 4 were in a close second tier, and specialist systems (e.g., Hunyuan-MT) were competitive. For short spoken segments latency dominates, so use a fast "Flash/mini"-class LLM or DeepL. No rigorous per-segment latency benchmark exists, and the figures that do exist are vendor-stated.

### Cited Findings
- **WMT25 General MT human evaluation.** [independent/academic; snippet-level extraction, check the tables] — [WMT25 findings](https://aclanthology.org/2025.wmt-1.22.pdf)
  - Overall, the best system in human evaluation was **Gemini 2.5 Pro**.
  - EN→RU: Gemini-2.5-Pro 83.4, Shy-hunyuan-MT 80.2, GPT-4.1 76.2, Claude-4 75.9.
  - EN→Bhojpuri: Gemini-2.5-Pro 94.9 (human reference 92.6), Claude-4 83.2, GPT-4.1 82.8.
  - One unattributed table showed GPT-4.1 ahead (77.0 vs 60.6 vs 55.7), so rankings vary by pair.
  - Automatic metrics did not always match human rankings.
- **DeepL's latency claim:** "GPT-5.2 is 4x slower than DeepL, Claude Opus 4.6 is 6x slower, Gemini 3.1 Pro is 29x slower". This compares frontier and pro models on full output, not Flash-class models or streaming TTFT. [vendor marketing] — [DeepL on X](https://x.com/DeepLcom/status/2093352915486339437)
- **TTFT for translation:** Gemini 0.15–0.40 s; ChatGPT and DeepSeek 0.20–0.50 s. No methodology given. [vendor] — [MachineTranslation.com](https://www.machinetranslation.com/blog/time-to-first-token-latency-of-llms)
- **Blended latency:** GPT-4.1 about 1.2 s; Gemini 2.5 Flash about 0.8 s. No test conditions given. [3rd-party] — [Eden AI](https://www.edenai.co/post/best-machine-translation-apis)
- **Speech-to-text translation latency in a production STT:** AssemblyAI says translation from a small LLM lands about 100 ms after the final turn. [vendor] — [AssemblyAI](https://www.assemblyai.com/blog/best-real-time-speech-translation-apis)
- In LiveLingo's test, the Whisper + GPT-4o-mini cascade had the second-best latency (2.7 s) and few erasures (22 per 120 s). This shows a small-LLM MT stage is not the bottleneck. [vendor-run benchmark] — [LiveLingo](https://www.livelingo.io/research/benchmark-2026)
- **Simultaneous strategies:**
  - LLMs can act as zero-shot context-aware simultaneous translators. — [arXiv 2406.13476](https://arxiv.org/pdf/2406.13476)
  - TransLLaMa is an LLM-based simultaneous translation system. — [arXiv 2402.04636](https://arxiv.org/pdf/2402.04636)
  - Hibiki-Zero learns its read/write policy with RL (GRPO) instead of wait-k on aligned data. — [arXiv](https://arxiv.org/html/2602.11072v1)

### Inferences
- **Recommended MT:** a Flash-class Gemini model (e.g., the current Gemini Flash / Flash-Lite) or GPT-5.x-mini/nano. Use a short rolling context of the last 2–4 segments plus a glossary of names, and translate on STT endpoint/final segments.
  - Expected time to first token is 150–400 ms, and short-segment completion is about 300–800 ms.
  - Token cost per call-minute is negligible (well under $0.001/min). Spoken speech runs about 150 words/min, which is about 200–300 tokens per minute each way.
- **DeepL as an alternative:** consider DeepL API for European pairs where it has an edge on latency and determinism.
- **Avoid incremental re-translation of partials into TTS.** Erasure counts (Azure 121, Google 353) show it causes visible flicker. Spoken output cannot be retracted, so commit only on stable segments (endpoint or clause boundary). Partial translations can still go to captions.

### Gaps
- I did not retrieve current per-token prices for Gemini Flash, GPT-5.x mini, Claude Haiku, Groq or Cerebras-hosted models, DeepL API ($/M chars), or Google Translation LLM. Pull them from the vendor pricing pages.
- No independent per-segment latency benchmark for MT APIs.
- No WMT 2026 results (WMT26 runs in late 2026 and was not found).

---

## Q4. Overall latency budget, techniques, and reference architectures (Meet, Teams, Palabra, LiveKit/Pipecat)

### Takeaway
A realistic speech-in to translated-speech-out time is **about 1.5–3 s after the end of each phrase**, for both a tuned cascade and a managed E2E model. The voice-to-voice figures under 1 s that voice-agent frameworks advertise do not carry over, because translation must wait for enough source content. Cloning TTS adds only about 100–300 ms to first audio.

### Cited Findings
- Measured "speaker stops → translation" ranges from 1.5 s (best) to 2.7 s (small-LLM cascade) and about 3–3.8 s (Gemini and OpenAI E2E). — [LiveLingo](https://www.livelingo.io/research/benchmark-2026)
- Academic simultaneous S2ST runs at a LAAL of about 2.8–3.1 s and an end offset of about 2–2.6 s on short-form. — [arXiv Hibiki-Zero](https://arxiv.org/html/2602.11072v1)
- **Voice-agent pipelines (not translation):** [3rd-party, unclear method] — [Kanopy Labs](https://kanopylabs.com/blog/openai-realtime-vs-livekit-vs-pipecat-voice)
  - LiveKit Agents (Deepgram + GPT-4o-mini + Cartesia): p50 550 ms / p95 850 ms voice-to-voice.
  - Pipecat (Deepgram + Groq Llama + Cartesia): 400 ms / 650 ms.
  - A matched stack is within about 50–100 ms between the two frameworks. — [Evalgent](https://www.evalgent.com/blog/livekit-vs-pipecat-latency)
  - A Pipecat-over-LiveKit transport showed desynchronization with multiple active audio tracks. This is relevant for per-listener translated tracks. — [Effloow](https://effloow.com/articles/pipecat-livekit-realtime-voice-latency-benchmark-2026)
  - Framework self-reported latency undercounts what the listener hears, so measure externally. — [arXiv 2611.19118](https://arxiv.org/pdf/2611.19118)
- **Cloning TTS time-to-first-audio:** [3rd-party] — [Cekura](https://www.cekura.ai/blogs/elevenlabs-vs-cartesia); [theplanettools](https://theplanettools.ai/compare/elevenlabs-vs-cartesia)
  - ElevenLabs Flash v2.5 measured at 177 ms in streaming (vendor claim about 75 ms).
  - Cartesia Sonic 3.6 measured at 270 ms (vendor claim about 40–90 ms).
  - Cartesia Sonic 3.5 went GA in May 2026 and added Professional Voice Clones in June 2026. — [invideo](https://invideo.io/blog/cartesia-sonic-ai-voice/)
  - Cartesia clones from about 3 s of audio. — [texttolab](https://texttolab.com/blog/cartesia-vs-elevenlabs)
  - Palabra TTS: 35 ms P90 TTFA, excluding network. — [Palabra TTS](https://www.palabra.ai/text-to-speech)
- **TTS pricing:** [3rd-party, conflicting]
  - ElevenLabs Flash v2.5 about $0.05 per 1K chars. — [fast.io](https://fast.io/resources/best-ai-text-to-speech-tools-2026/)
  - Cartesia about $1.68/h of audio on Scale. — [dupple](https://dupple.com/learn/cartesia-vs-elevenlabs)
- **Streaming STT:** gpt-realtime-whisper costs $0.017/min. — [OpenAI](https://openai.com/index/advancing-voice-intelligence-with-new-models-in-the-api/)
- **Google Meet reference architecture:** a Gemini 3.5 Live Translate audio-to-audio model with prosody carry-over, any-to-any among 63 languages, chosen per listener. — [Google Workspace Updates](https://workspaceupdates.googleblog.com/2026/10/real-time-speech-translation-in-google-Meet-expands-to-63-languages-with-automatic-language-detection.html)
- **Palabra:** WebRTC and WebSocket streaming with a captions API. Agora is reported as an enterprise client. — [Palabra](https://www.palabra.ai/); [Dealroom](https://app.dealroom.co/companies/palabra)

### Inferences

**Cascade latency budget** (per phrase, measured from the end of the source phrase):

| Stage | Budget |
|---|---|
| VAD / endpoint | 200–500 ms (or clause-boundary commit on partials) |
| STT final | 100–300 ms |
| MT, Flash-class LLM | 300–700 ms |
| Cloning-TTS time to first audio | 150–300 ms |
| WebRTC transport and jitter | 100–200 ms |
| **Total** | **about 0.9–2.0 s** |

For long monologues, commit clauses early, wait-k style, to keep lag bounded.

**Techniques:**
- Play the original audio ducked under the translation, at about −15 to −20 dB, so the listener keeps the speaker's presence.
- Show streaming partial captions immediately.
- Synthesize only committed segments.
- Use echo cancellation so translated audio isn't re-captured by the speaker's mic, since each client plays TTS locally. In practice this means server-side mixing per listener track, or relying on browser AEC because TTS plays through the same output device.

**Topology for a two-party call:** each participant has one source stream and one target language per listener. That means at most 2 translation directions, and if both listeners pick each other's language, exactly 2 sessions.

### Gaps
- I found no public architecture write-ups for Zoom, Teams Interpreter, Kudo or Wordly.
- I found no official LiveKit or Pipecat translation-with-cloning example (only general voice-agent examples).
- I found no measured end-to-end latency for a cascade that includes cloning TTS.

---

## Recommendation summary (for the report writer, derived from the findings above)

**Architecture: hybrid.**
- **Default path (most languages):** a cascade with streaming STT → Flash-class LLM MT → instant-clone streaming TTS (Cartesia Sonic or ElevenLabs Flash, cloned from the first 3–10 s of each participant's speech). This is the only path that gives:
  - genuine speaker identity, with consent captured at join;
  - control over glossary and context;
  - any-to-any language pairs;
  - text captions.
- **Fallback or alternative path:** Gemini 3.5 Live Translate. It is the simplest E2E option, top-rated for comprehension, prosody-preserving and about $0.037/min. A/B test it against the cascade. Its drawbacks are:
  - preview status with no SLA;
  - a 15-minute session default;
  - code-switching failures;
  - inconsistent voice likeness.
- **Do not use:**
  - Seamless (non-commercial license);
  - Hibiki-Zero (X→EN only);
  - gpt-realtime-translate as the primary (13 output languages, no voice preservation, lowest quality score, lag drift).

**MT model:** Gemini Flash-class (WMT25 leader family) or GPT-5.x-mini, with 2–4 segments of context and a per-call glossary. Use DeepL as a deterministic option for EU pairs. Keep the MT stage under 700 ms.

**Realistic latency:** about 1–2 s after the end of each phrase for a tuned cascade, and about 3 s for the managed E2E option as measured.

**Estimated cost per participant-minute** (for one translated direction heard by one listener; my calculation from the cited prices):

| Path | Components | Total |
|---|---|---|
| Cascade | STT about $0.005–0.017/min, MT under $0.001/min, cloning TTS about $0.03–0.08/min (assuming about 50% speaking time and about 700–900 chars per spoken minute at about $0.05/1K chars for ElevenLabs, less for Cartesia) | **about $0.03–0.10 per speaking-minute** |
| Gemini 3.5 Live Translate | — | **about $0.037/min of streamed audio** |
| OpenAI translate | — | $0.034/min |
| Palabra | — | $0.04/min |
| Azure Live Interpreter with custom voice | about $1 + $2 per hour | **about $0.05/min** |

Infrastructure (SFU, compute) is extra.
