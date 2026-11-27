# Open-weight, self-hostable zero-shot voice-cloning TTS (as of November 2026)

Scope: models Diceri (AGPL-3.0, commercial use) could self-host to re-speak a caller's translated
speech in that caller's own voice from a ~10 s recording, in many languages, with streaming.
Research date: 2026-11-27. All figures are vendor/model-card self-reports unless stated otherwise;
no independent head-to-head benchmark across these models was found.

## 1. Licenses: code AND weights, commercial use, AGPL compatibility

### Takeaway
Only a handful of strong cloning models are both open-weight and commercially usable: **Qwen3-TTS
(Apache-2.0)**, **VoxCPM2 (Apache-2.0)**, **Fun-CosyVoice3 (Apache-2.0)**, **Chatterbox Multilingual
(MIT, but retired/unmaintained since Aug 2026 and watermarked)**, and **Audio8-TTS (Apache-2.0,
unverified)**. Higgs Audio v2 is commercial with a 100k-annual-active-user cap and branding duties.
Many popular models are non-commercial: F5-TTS, OmniVoice weights, Voxtral TTS, OpenAudio S1-mini,
Higgs v3, XTTS-v2 (CPML), and IndexTTS (custom bilibili license, commercial by contact only).

### Cited Findings
- **Qwen3-TTS (Alibaba Qwen)**: Apache-2.0. Base cloning models come in 0.6B and 1.7B. Released 2026-01-22 — [HF card](https://huggingface.co/Qwen/Qwen3-TTS-12Hz-1.7B-Base); [GitHub](https://github.com/QwenLM/Qwen3-TTS)
- **VoxCPM2 (OpenBMB)**: "VoxCPM model weights and code are open-sourced under the Apache-2.0 license". 2B parameters, MiniCPM-4 backbone — [GitHub](https://github.com/OpenBMB/VoxCPM)
- **Fun-CosyVoice3-0.5B-2512 (Alibaba FunAudioLLM)**: Apache-2.0. The card also says its content is "for academic purposes only and is intended to demonstrate technical capabilities". That disclaimer is about the demo content, not a license term — [HF card](https://huggingface.co/FunAudioLLM/Fun-CosyVoice3-0.5B-2512)
- **Chatterbox (Resemble AI)**: code and weights MIT. Every output carries an imperceptible Perth neural watermark — [HF card](https://huggingface.co/ResembleAI/chatterbox). "As of August 2026, Chatterbox is retired research and is no longer maintained". It stays public as an MIT archive with no support or updates — [GitHub](https://github.com/resemble-ai/chatterbox)
- **Chatterbox Turbo**: MIT, 350M, English only — [HF card](https://huggingface.co/ResembleAI/chatterbox-turbo)
- **Higgs Audio v2 (Boson AI)**: "Boson Higgs Audio 2 Community License", based on the Meta Llama 3 Community License — [LICENSE](https://huggingface.co/bosonai/higgs-audio-v2-generation-3B-base/blob/main/LICENSE). Key terms:
  - Commercial use is royalty-free.
  - Products with **more than 100,000 annual active users** must request an expanded license, which Boson grants at its sole discretion.
  - You must display "Built with Higgs Materials licensed from Boson AI USA, Inc."
  - Derivative models must have names starting with "Higgs Audio 2".
  - The Llama Acceptable Use Policy applies.
  - Outputs may not be used to improve other LLMs.
  - Starting IP litigation terminates the license.
  - Users must indemnify Boson.
- **Higgs Audio v3 / "Higgs TTS 3"** (4B): "Boson Higgs Audio v3 Research and Non-Commercial License". "Production / hosted / revenue-generating use requires a separate commercial license". The repo sidebar shows Apache-2.0, but that covers only the code — [GitHub](https://github.com/boson-ai/higgs-audio)
- **F5-TTS**: weights CC-BY-NC-4.0 (trained on the Emilia dataset) — [HF card](https://huggingface.co/SWivid/F5-TTS). Non-commercial.
- **OmniVoice (k2-fsa)**: code Apache-2.0, but "The pre-trained model is licensed under the CC-BY-NC due to constraints from its training data (e.g., Emilia)" — [HF card](https://huggingface.co/k2-fsa/OmniVoice). Third-party blogs disagree on its license, and the model card is authoritative — [search summary of BrightCoding vs others](https://www.blog.brightcoding.dev/2026/09/02/k2-fsaomnivoice-600-language-tts-with-voice-cloning)
- **Mistral Voxtral TTS** (4B, released 2026-03-26): weights CC BY-NC 4.0. Earlier Voxtral STT models were Apache-2.0. Commercial use goes through Mistral's API — [Mistral](https://mistral.ai/news/voxtral-tts/); [Slator](https://slator.com/mistral-text-to-speech-model/); [arXiv 2603.25551](https://arxiv.org/abs/2603.25551)
- **Fish Audio OpenAudio S1-mini** (0.5B): CC-BY-NC-SA-4.0. The full S1 (4B) is proprietary — [HF card](https://huggingface.co/fishaudio/openaudio-s1-mini)
- **Fish Audio S2-Pro**: weights under the "Fish Audio Research License". Since 2026-10-13 the code is under the "Fish Audio Community License", which covers revenue under $500K and **excludes hosted TTS services**. Commercial self-hosting needs a paid license — [Cekura license-checked roundup](https://www.cekura.ai/blogs/open-source-tts) (secondary source, not verified against the Fish repo)
- **Coqui XTTS-v2**: Coqui Public Model License (CPML) — [HF card](https://huggingface.co/coqui/XTTS-v2). The card does not state commercial terms (see Gaps).
- **IndexTTS 2 / 2.5 (bilibili)**: "bilibili Model Use License Agreement". For commercial use, write to indexspeech@bilibili.com — [GitHub](https://github.com/index-tts/index-tts)
- **Orpheus TTS (Canopy)**: Apache-2.0 code and weights. It is built on Llama-3.2-3B-Instruct, so Llama license terms may also apply. Gated on Hugging Face — [Cekura](https://www.cekura.ai/blogs/open-source-tts)
- **VibeVoice (Microsoft)**: MIT, but the model cards say research and development only, and the README advises against commercial use. The original TTS code was removed in 2025 — [Cekura](https://www.cekura.ai/blogs/open-source-tts)
- **Kokoro-82M**: Apache-2.0, but it has **no voice cloning** (54 preset voices) — [Cekura](https://www.cekura.ai/blogs/open-source-tts)
- **Audio8-TTS-Realistic (Edge0)**: 0.6B, 23 languages, weights and code reported as Apache-2.0 — [MarkTechPost, 2026-10-29](https://www.marktechpost.com/2026/10/29/edge0-audio8-tts-realistic-0-6b-voice-cloning/) (secondary source; repo not checked)

### Inferences
- **AGPL and the model.** Running an Apache-2.0 or MIT model behind Diceri's AGPL service raises no compatibility problem: both licenses are permissive and AGPL-compatible. NC licenses (CC-BY-NC, CC-BY-NC-SA, CPML, Higgs v3, Voxtral) are incompatible with a commercial product whether or not the model weights are "distributed".
- **Higgs Audio v2.** Three things make it a poor fit for an open-source product:
  - The 100k-AAU cap is at Boson's discretion.
  - The "Built with Higgs" notice is mandatory.
  - The Llama AUP applies.
  - Its terms are also "additional restrictions" of a kind the AGPL forbids. That only matters if Diceri bundled or redistributed the weights inside the AGPL work. Calling the model as a separate service is the safer path.
- **Emilia-trained models.** Emilia is a recurring source of NC restrictions (F5-TTS, OmniVoice). For any new model, check whether its training data included Emilia (the original NC release) or only Emilia-YODAS / commercially licensed data.
- **Chatterbox watermark.** The Perth watermark is a plus for the consent posture (ADR 0006). But the project being retired means no fixes, for example for RTX 50-series incompatibility (reported by Cekura).

### Gaps
- **CPML's exact commercial terms.** CPML is widely described as non-commercial, with a commercial license previously sold by Coqui (which shut down). I did not fetch the CPML text in this session.
- **Weights licenses not fetched:** Zonos (Zyphra), Spark-TTS, Llasa, MaskGCT, and Sesame CSM.
- **Fish S2 license text:** not verified from Fish's own repo.
- **Audio8-TTS:** its license was not verified from a primary source.

## 2. Languages, cross-lingual cloning, reference length

### Takeaway
No commercially licensed open model covers ~57 languages with cloning.
- **VoxCPM2 (30 languages)** and **Chatterbox Multilingual / Audio8 (23)** come closest.
- **Qwen3-TTS (10)** and **CosyVoice3 (9)** have the best published multilingual cloning metrics but cover few languages.
- The broadest models (OmniVoice 600+, Higgs v3 100+, Fish S2 80+) are all non-commercial.

A realistic self-hosted design is a router: several models by language, plus a hosted API for the long tail.

### Cited Findings
- **Qwen3-TTS**: 10 languages (zh, en, ja, ko, de, fr, ru, pt, es, it). It clones from a **3 s** reference — [HF card](https://huggingface.co/Qwen/Qwen3-TTS-12Hz-1.7B-Base). For cross-lingual cloning it has the lowest mixed error rate in 8 of 12 language pairs against CosyVoice3. For example, zh→ko is 4.82 vs 14.4 — [GitHub](https://github.com/QwenLM/Qwen3-TTS)
- **VoxCPM2**: 30 languages, including ar, zh, en, ja, ko, hi, vi and sw, plus 9 Chinese dialects. The reference is a "short reference clip" with no duration given — [GitHub](https://github.com/OpenBMB/VoxCPM)
- **Chatterbox Multilingual V3**: 23 languages: ar, da, de, el, en, es, fi, fr, he, hi, it, ja, ko, ms, nl, no, pl, pt, ru, sv, sw, tr, zh — [HF card](https://huggingface.co/ResembleAI/chatterbox)
  - It also has finetunes for pt-BR, LatAm Spanish, Spain Spanish, pt-PT, zh and hi.
  - If the reference clip's language differs from the target language, output may inherit the clip's accent. The suggested mitigation is `cfg_weight=0`. This matters for cross-lingual translation — [GitHub](https://github.com/resemble-ai/chatterbox)
  - Turbo's example uses a ~10 s reference clip — [HF card](https://huggingface.co/ResembleAI/chatterbox-turbo)
- **Fun-CosyVoice3**: 9 languages (zh, en, ja, ko, de, es, fr, it, ru) plus 18+ Chinese dialects. It supports "multi-lingual/cross-lingual zero-shot voice cloning". No pt-BR — [HF card](https://huggingface.co/FunAudioLLM/Fun-CosyVoice3-0.5B-2512)
- **XTTS-v2**: 17 languages (en, es, fr, de, it, pt, pl, tr, ru, nl, cs, ar, zh-cn, ja, hu, ko, hi). It clones from a **6 s** clip — [HF card](https://huggingface.co/coqui/XTTS-v2)
- **IndexTTS-2.5**: zh, en, ja, es, ar — [GitHub](https://github.com/index-tts/index-tts)
- **OpenAudio S1-mini**: 13 languages — [HF card](https://huggingface.co/fishaudio/openaudio-s1-mini)
- **Voxtral TTS**: 9 languages (en, fr, de, es, nl, pt, it, hi, ar). It clones from about 3 s — [Slator](https://slator.com/mistral-text-to-speech-model/); [The Decoder](https://the-decoder.com/mistrals-first-open-weight-tts-model-voxtral-clones-voices-from-three-seconds-of-audio-across-nine-languages/)
- **OmniVoice**: 600+ languages. It recommends a 3–10 s reference and warns that longer clips slow inference and may reduce quality — [GitHub](https://github.com/k2-fsa/OmniVoice)
- **Higgs v3**: "100+ languages" — [GitHub](https://github.com/boson-ai/higgs-audio). The Higgs v2 card is tagged with 4 languages — [HF card](https://huggingface.co/bosonai/higgs-audio-v2-generation-3B-base)
- **Fish S2-Pro**: 80+ languages. It needs a 10–30 s reference — [Cekura](https://www.cekura.ai/blogs/open-source-tts)
- **Orpheus**: primarily English, with 7 multilingual research pairs. The base model is not trained for zero-shot cloning. **VibeVoice Realtime** is mainly English, with experimental voices in 9 languages. **Sesame CSM** is weak outside English — [Cekura](https://www.cekura.ai/blogs/open-source-tts)

### Inferences
- **Diceri's core list** is en, pt-BR, es, fr, de, it, ru, ja, ko, zh, hi, ar.
  - **Chatterbox Multilingual** and **VoxCPM2** cover all 12 under commercial-friendly licenses.
  - **Qwen3-TTS** covers 10 of the 12, missing hi and ar.
  - **CosyVoice3** covers 9 of the 12, missing pt, hi and ar.
- Diceri's ~10 s recording is enough for every model listed. Several need only 3 s, and OmniVoice warns against going longer.

### Gaps
- **Missing language lists:** Audio8-TTS's 23 languages and the full VoxCPM2 30-language list were not captured.
- **Cross-lingual quality** (Portuguese speaker → Japanese output) is published only by Qwen3-TTS and CosyVoice3. Chatterbox flags accent leakage.

## 3. Published quality metrics, and comparison with ElevenLabs

### Takeaway
On their own reported numbers, the best commercially licensed open models match or beat ElevenLabs on intelligibility and speaker similarity.
- **Seed-TTS-eval:** Qwen3-TTS reports WER 1.24 (en) and 0.77 (zh); VoxCPM2 reports SIM 75–80%.
- **Against ElevenLabs:** Qwen3-TTS reports higher SIM than ElevenLabs in all 10 of its languages, and Voxtral reports a 68.4% human-preference win rate over ElevenLabs Flash v2.5.

All of these are vendor-run evaluations, and SIM scoring setups differ between papers.

### Cited Findings
- **Qwen3-TTS-12Hz-1.7B-Base**: Seed-TTS WER 1.24 (test-en) and 0.77 (test-zh) — [HF card](https://huggingface.co/Qwen/Qwen3-TTS-12Hz-1.7B-Base)
  - On Qwen's multilingual test set, SIM (1.7B) ranges from 0.714 (fr) to 0.817 (it, pt) — [GitHub](https://github.com/QwenLM/Qwen3-TTS)
  - On the same set, ElevenLabs scores 0.535–0.738 and MiniMax 0.628–0.805.
  - English WER: Qwen 0.934, ElevenLabs 2.339, MiniMax 2.164. Chinese WER: ElevenLabs 16.026 — [HF card](https://huggingface.co/Qwen/Qwen3-TTS-12Hz-1.7B-Base)
- **VoxCPM2** — [GitHub](https://github.com/OpenBMB/VoxCPM)
  - Seed-TTS-eval: test-en WER 1.84% / SIM 75.3%; test-zh CER 0.97% / SIM 79.5%; test-hard CER 8.13% / SIM 75.3%.
  - CV3-eval: ja CER 5.96.
  - An internal 30-language ASR test reports a 1.68% average error, scored by a Gemini model.
- **Fun-CosyVoice3-0.5B** — [HF card](https://huggingface.co/FunAudioLLM/Fun-CosyVoice3-0.5B-2512)
  - test-zh: CER 1.21% / SIM 78.0%.
  - test-en: WER 2.24% / SIM 71.8%.
  - The RL variant reaches en WER 1.68% and zh CER 0.81%.
- **Higgs Audio v2**: Seed-TTS Eval WER 2.44 / SIM 67.70. EmergentTTS-Eval win rate over gpt-4o-mini-tts: 75.71% (Emotions) and 55.71% (Questions) — [HF card](https://huggingface.co/bosonai/higgs-audio-v2-generation-3B-base)
- **OpenAudio S1-mini**: English WER 0.011, CER 0.005 and speaker *distance* 0.380, measured with gpt-4o-transcribe and pyannote. This is not comparable to SIM-o — [HF card](https://huggingface.co/fishaudio/openaudio-s1-mini)
- **Voxtral TTS**: 68.4% win rate over ElevenLabs Flash v2.5 in human evaluations of multilingual voice cloning (non-commercial weights) — [arXiv 2603.25551](https://arxiv.org/abs/2603.25551) via [search summary](https://mistral.ai/news/voxtral-tts/)
- **Chatterbox**: Resemble claims it is "consistently preferred" over ElevenLabs in side-by-side Podonos tests. No numbers appear on the card — [HF card](https://huggingface.co/ResembleAI/chatterbox). The repo links Podonos comparisons of Turbo vs ElevenLabs Turbo v2.5, Cartesia Sonic 3 and VibeVoice 7B, but the results are not in the text — [GitHub](https://github.com/resemble-ai/chatterbox)

### Inferences
- **Qwen3-TTS has the strongest published multilingual cloning evidence among the commercially licensed options.** The weak point is its 10-language coverage.
- **ElevenLabs' low SIM in Qwen's table** may partly reflect how ElevenLabs was prompted (instant clone settings). Treat it as vendor framing, not a neutral result.
- **Comparability:** none of these numbers come from a shared third-party leaderboard. Diceri should run its own small A/B on real call audio: noisy laptop microphones and cross-lingual pairs.

### Gaps
- **No independent multilingual cloning benchmark** covering these models was found. Possible sources would be TTS Arena or Artificial Analysis open-weight entries, which were not fetched.
- **Missing metrics:** no MOS numbers were captured, and no Chatterbox Multilingual or Audio8 WER/SIM numbers were found.

## 4. Streaming, time-to-first-audio, RTF, throughput

### Takeaway
- **Native streaming with ~100–150 ms claimed first-audio latency:** Qwen3-TTS (97 ms) and CosyVoice3 (150 ms, bi-streaming).
- **Streaming API, latency not given in ms:** VoxCPM2 (RTF 0.13–0.30 on an RTX 4090).
- **No documented streaming:** Chatterbox's open models. Its "sub-200 ms" applies only to Resemble's paid service.

Published concurrency or throughput per GPU is almost absent.

### Cited Findings
- **Qwen3-TTS**: "end-to-end synthesis latency as low as 97ms", with the first packet after a single input character. No GPU is named — [GitHub](https://github.com/QwenLM/Qwen3-TTS)
  - vLLM-Omni has day-0 support but **offline inference only**. Online serving and streaming are listed as future work.
  - No throughput numbers are published.
- **CosyVoice3**: bi-streaming (text-in, audio-out) at "latency as low as 150ms" — [HF card](https://huggingface.co/FunAudioLLM/Fun-CosyVoice3-0.5B-2512)
- **VoxCPM2**: `generate_streaming` API. RTF about 0.30 on an RTX 4090 in PyTorch and about 0.13 with Nano-vLLM. About 8 GB VRAM — [GitHub](https://github.com/OpenBMB/VoxCPM)
- **IndexTTS**: RTF on an RTX 4090 with KV cache is 0.2065 (2.5, bf16) and 0.3257 (2, fp16) — [GitHub](https://github.com/index-tts/index-tts)
- **OmniVoice**: single H100, fp16, 32 steps — [GitHub](https://github.com/k2-fsa/OmniVoice)
  - RTF 0.0899 at batch 1, 0.0367 with FlashInfer and CUDA graph, and 0.0115 at batch 8.
  - Streaming is not mentioned.
- **Chatterbox**: no streaming mentioned — [GitHub](https://github.com/resemble-ai/chatterbox)
  - Nano is "3x faster than realtime on 8 CPU cores".
  - Turbo distils the decoder from 10 steps to 1, but publishes no RTF.
- **Orpheus**: about 200 ms, or about 100 ms with streamed text input. **VibeVoice Realtime**: about 300 ms first audible. **Fish S2-Pro**: about 100 ms first audio and RTF 0.195 on an H200 (vendor), while the hosted S2.1-Pro measured 321 ms (Cekura). Voxtral TTS needs a GPU with at least 16 GB. Fish S2 needs at least 24 GB VRAM — [Cekura](https://www.cekura.ai/blogs/open-source-tts)
- **Higgs v3**: streaming is served through SGLang-Omni (`sgl-omni serve`), with no latency or throughput figures — [GitHub](https://github.com/boson-ai/higgs-audio)

### Inferences
- **Streams per GPU (single-stream estimate).** An RTF of 0.2 means one GPU generates 5 s of audio per second at batch 1. A live call only needs real-time playback, so an RTX 4090 or L4-class GPU could in principle serve about 3–5 simultaneous speakers per model replica at RTF 0.13–0.3. Batching (vLLM, SGLang, OmniVoice batch 8) can raise this. This is my estimate, not a measured figure.
- **L4 vs 4090.** An L4 is generally slower than an RTX 4090, so RTFs measured on a 4090 will be worse on an L4. No L4 or A10G numbers were found for these models.

### Gaps
- **No measured numbers for the target setup:** no TTFA on L4, A10G or H100 for Qwen3-TTS or Chatterbox Multilingual, and no published concurrent-streams-per-GPU figure for any of these models.

## 5. Self-hosting cost vs hosted APIs

### Takeaway
GPU-hours are cheap: an L4 or 4090 costs $0.34–0.89/h on RunPod and $0.80/h on Modal. Per-character cost is dominated by **utilization**, because live calls need warm GPUs.

At full utilization, self-hosting works out to low single-digit dollars per million characters. Hosted open models are $20–25 per million characters (fal Chatterbox, Kokoro). MiniMax costs $60–100 per million.

### Cited Findings
- **Modal serverless** (per second), with $30 per month of free credits on the Starter plan — [Modal pricing](https://modal.com/pricing):
  - T4: $0.59/h
  - L4: $0.80/h
  - A10: $1.10/h
  - L40S: $1.95/h
  - A100-80GB: $2.50/h
  - H100: $3.95/h
- **RunPod on-demand pods** (community / secure cloud) — [RunPod pricing](https://www.runpod.io/pricing):
  - L4: $0.44 / $0.69
  - RTX 4090: $0.34 / $0.89
  - RTX 5090: $0.69 / $1.39
  - L40S: $0.79 / $1.09
  - A100-80GB PCIe: $1.19 / $2.09
  - H100 SXM: $2.69 / $4.39
- **RunPod serverless** — [RunPod pricing](https://www.runpod.io/pricing):
  - L4: $0.75/h
  - 4090: $1.34/h
  - L40S: $1.89/h
  - A100: $2.99/h
  - H100: $5.89/h
- **fal serverless GPUs** (list price, with lower "as low as" rates): H100 $4.50/h ($2.49 as low as) and RTX PRO 6000 $4.00/h ($1.99) — [fal pricing](https://fal.ai/pricing)
- **fal hosted TTS** — [fal pricing](https://fal.ai/pricing):
  - Chatterbox: **$0.025 per 1,000 characters**, i.e. $25 per million.
  - Chatterbox speech-to-speech: $0.015 per minute.
  - Kokoro: $0.02 per 1,000 characters.
  - **Index TTS 2.0: $0.002 per second of audio.**
  - MiniMax Speech 2.8 HD / Turbo: $0.10 / $0.06 per 1,000 characters.
  - Inworld TTS-1.5 Max: $0.01 per 1,000 characters.
  - No F5, XTTS or Qwen3-TTS listing.
- **DeepInfra**: the pricing page lists no TTS models, only STT. Voxtral STT is $0.001–0.003 per minute — [DeepInfra pricing](https://deepinfra.com/pricing)

### Inferences
All figures below are my calculations, not sourced. I assume about 15 characters per second of speech, so 1M characters ≈ 18.5 h of audio.
- **Raw compute at RTF 0.2, batch 1, 100% busy:** about 3.7 GPU-h per 1M characters.
  - RunPod 4090 community ($0.34/h): about $1.3 per 1M characters.
  - Modal L4 ($0.80/h, and RTF likely worse on an L4): about $3–6.
- **At a realistic 10–25% utilization for bursty live calls:** about $5–40 per 1M characters. This is roughly the fal hosted Chatterbox price ($25 per 1M).
- **fal Index TTS 2.0** at $0.002/s works out to about $133 per 1M characters. Index TTS is also licensed for commercial use only on request from bilibili.
- **Always-on floor:** one 24/7 GPU costs about $250–650 per month (RunPod L4 or 4090 at $0.34–0.89/h × 730 h). This is the floor for a warm, low-latency self-hosted pool.
- **Serverless cold starts.** Model load on Modal or RunPod conflicts with low time-to-first-audio unless a minimum number of containers is kept warm, which is billed.
- **Licensing of hosted versions.** Third parties hosting an MIT model (Chatterbox on fal) is fine. No third party was found selling hosted Qwen3-TTS or VoxCPM2, which would be the most attractive license-plus-quality combinations.

### Gaps
- **Not checked:** Replicate, Baseten and Together pricing for TTS. DeepInfra's /models/text-to-speech page was not fetched.
- **ElevenLabs pricing** for the comparison is out of scope here and is presumably covered by another researcher's notes.

## 6. Maturity, maintenance, known issues

### Takeaway
- **Qwen3-TTS (Jan 2026)** and **VoxCPM2 (2026)** are active, backed by large labs, and have serving integrations (vLLM-Omni, Nano-vLLM).
- **Chatterbox** is popular but officially retired as of Aug 2026.
- **XTTS-v2** is legacy: Coqui is defunct and the license is CPML.

Hallucination and stability data is thin everywhere.

### Cited Findings
- **Chatterbox**:
  - Retired and unmaintained since Aug 2026 — [GitHub](https://github.com/resemble-ai/chatterbox)
  - Reported RTX 50-series incompatibility — [Cekura](https://www.cekura.ai/blogs/open-source-tts)
  - V3 claims fewer hallucinations, with no measurements published — [GitHub](https://github.com/resemble-ai/chatterbox)
- **Qwen3-TTS**: released 2026-01-22, with a tech report (arXiv 2601.15621). vLLM-Omni online serving and streaming are still listed as future work — [GitHub](https://github.com/QwenLM/Qwen3-TTS)
- **VoxCPM release history** — [GitHub](https://github.com/OpenBMB/VoxCPM):
  - 0.5B: Sep 2025
  - 1.5: Dec 2025
  - VoxCPM2: April 2026, although the README's arXiv id 2606.06928 suggests June, so the date is inconsistent.
  - Runtimes: Nano-vLLM and llama.cpp-omni.
- **IndexTTS release history**: 1.0 (2025-03), 1.5 (2025-05), 2 (2025-09-08) and 2.5 (2026-08-10). Duration control was "not yet enabled" in v2 and arrives in 2.5 — [GitHub](https://github.com/index-tts/index-tts)
- **CosyVoice3**: released Dec 2025. The repo moved to QwenAudio/CosyVoice — [HF card](https://huggingface.co/FunAudioLLM/Fun-CosyVoice3-0.5B-2512); [Cekura](https://www.cekura.ai/blogs/open-source-tts)
- **Kokoro**: the documentation is thin — [Cekura](https://www.cekura.ai/blogs/open-source-tts)
- **Newer 2026 entrants:**
  - Audio8-TTS-Realistic (Oct 2026) — [MarkTechPost](https://www.marktechpost.com/2026/10/29/edge0-audio8-tts-realistic-0-6b-voice-cloning/)
  - OmniVoice (arXiv 2604.00688, NC weights) — [HF](https://huggingface.co/k2-fsa/OmniVoice)
  - Higgs v3 (NC) — [GitHub](https://github.com/boson-ai/higgs-audio)
  - Voxtral TTS (Mar 2026, NC) — [Mistral](https://mistral.ai/news/voxtral-tts/)

### Inferences
- **Shortlist for commercial self-hosting in Diceri:**
  1. Qwen3-TTS (best evidence, 10 languages)
  2. VoxCPM2 (30 languages, Apache)
  3. CosyVoice3 (low latency, 9 languages)
  4. Chatterbox Multilingual (23 languages, MIT, frozen)
- **Avoid for commercial use:** F5-TTS, OmniVoice, Voxtral TTS, OpenAudio/Fish, Higgs v3, XTTS-v2 and IndexTTS, unless a commercial license is bought.

### Gaps
- **No production users** were identified for any model.
- **No long-form or hallucination rate measurements** were found.
- **Zonos, Spark-TTS, Llasa, MaskGCT, Sesame CSM details** (license, languages, metrics) were not researched within the tool budget.
