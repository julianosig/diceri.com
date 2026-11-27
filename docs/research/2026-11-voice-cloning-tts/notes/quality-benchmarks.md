# Independent quality evaluations of voice-cloning TTS (as of 2026-11-27)

Scope: naturalness/preference, zero-shot speaker similarity (SIM), pronunciation (WER/CER) per
language, cross-lingual cloning, and time to first audio (TTFA), for ElevenLabs versus the
alternatives, for a live voice-translation call product (pt-BR, en, es, fr, de, it, ja, ko, zh, hi, ar).

Source labels used below: **[INDEPENDENT]** = third-party benchmark operator or academic paper not
selling the systems compared; **[VENDOR]** = run or published by a company whose own model is in the
comparison; **[COPIED]** = numbers re-used from another paper, not re-measured.

## 1. Artificial Analysis arenas: current Elo ranks, price and latency columns

### Takeaway
On Artificial Analysis (AA), the only large independent blind-preference arena with ongoing updates,
ElevenLabs' new **Eleven v4 / v4 Turbo** sit at #1-#2 with the providers' own native voices. In the
**Controlled Voice** arena, where every model clones the same 8 English voices (the closest proxy to
our use case), they drop to #6-#7. Alibaba Qwen-Audio-3.1-TTS-Plus, HeyGen Voice and Speechify Simba 3.5
lead there, and Cartesia Sonic 3.6 and Inworld Realtime TTS-2 are statistically tied with ElevenLabs.
Both arenas are **English only** and say nothing about pt-BR, ja, zh, hi or ar.

### Cited Findings
**Provider Voice arena (native voices, read 2026-11-27; no "last updated" date on the page) [INDEPENDENT]**
— [AA leaderboard](https://artificialanalysis.ai/text-to-speech/leaderboard)
- Method: Elo from blind pairwise votes. Listeners hear the same text from two models and pick the more
  natural one. The anchor is Zonos-v0.1 = 1000. The page gives no prompts or languages (filters only for
  US/UK accent). It shows no latency column. Prices are USD per 1M characters at default API settings.
- Selected rows (rank, model, Elo [95% CI], samples, $/1M chars):
  - 1 Eleven v4 Turbo 1343 [1329–1357], 2,927, $40; 2 Eleven v4 1340 [1326–1354], 3,198, $80
  - 3 StepFun StepAudio 3 TTS 1310; 4 Alibaba Qwen-Audio-3.1-TTS-Plus 1299 ($19.31)
  - 5 Cartesia Sonic 3.6 1297 [1285–1309], 3,877, $49; 6 Speechify Simba 3.5 1296 ($6.58)
  - 7 Google Gemini 3.8 Flash TTS 1286 ($16.49); 9 Fish Audio Drama 3 1278 ($15)
  - 10 Inworld Realtime TTS-2 1272 [1259–1285], $20.83; 11 Gemini 3.8 Flash-Lite TTS 1265 ($11.03)
  - 16 ElevenLabs v3 Conversational 1219 ($50); 19 Inworld Realtime TTS-2 Flash 1216 ($10.42)
  - 20 Mistral Voxtral 3 TTS 1211 ($13); 23 Eleven v3 1194 ($100); 25 Fish Audio S2.1 Pro 1181
  - 28 Gradium TTS (Oct 2026) 1174 ($47.78); 29 MiniMax Speech 2.8 HD 1173 ($100); 36 Speech 2.8 Turbo 1151 ($60)
  - 44 Resemble Chatterbox HD 1112 ($40); 50 ElevenLabs Turbo v2.5 1094; 51 Voxtral TTS (v1) 1094
  - 52 ElevenLabs Multilingual v2 1091 ($100); 58 ElevenLabs Flash v2.5 1075 ($50)
  - 62 OpenAI GPT-Realtime-2 1070; 67 Google Chirp 3: HD 1062 ($30); 68 Hume Octave 2 1060 ($87.50)
  - 71 Deepgram Flux TTS 1053 ($45); 81 Boson Higgs Audio V3 1035; 87 Chatterbox 1021
  - 96 Alibaba Qwen3 TTS (open) 932; 97 Coqui XTTS v2 917
  - OpenAI gpt-4o-mini-tts does not appear in the list as extracted.

**Controlled Voice arena (same 8 cloned voices, 4 US + 4 UK English, anchor OpenAudio S1 = 1000) [INDEPENDENT]**
— [AA Controlled Voice](https://artificialanalysis.ai/text-to-speech/leaderboard/controlled-voice)
- 1 Qwen-Audio-3.1-TTS-Plus 1208 [1190–1226]; 2 HeyGen Voice 1190; 3 Simba 3.5 1185; 4 HeyGen Voice Turbo 1185 ($4)
- 5 Qwen-Audio-3.1-TTS-Flash 1172; **6 Eleven v4 Turbo 1158 [1147–1169]**; **7 Eleven v4 1155 [1144–1166]**
- 8 Cartesia Sonic 3.6 1154 [1143–1165]; 9 Inworld Realtime TTS-2 1150 [1139–1161]; 10 StepAudio 3 1138
- 17 Eleven v3 1075; 19 Fish Audio Drama 3 1072; 20 Voxtral 3 TTS 1069; 21 v3 Conversational 1068
- 22 Inworld TTS-2 Flash 1067; 25/26 Gemini 3.8 Flash-Lite / Flash 1059 / 1055; 27 Gradium (Sep 2026) 1054
- 29/30 MiniMax Speech 2.8 HD / Turbo 1031 / 1029; 33 Fish S2 Pro 1012; **34 Multilingual v2 1011**
- **40 Flash v2.5 986**; **41 Turbo v2.5 982**; 43 Higgs Audio V3 965; 45 Sonic 3 962
- 47 Chatterbox 942; 51 XTTS v2 847; 52 OpenVoice v2 827
- The page does not say how voices were cloned (reference length, transcript or not).
- The arena launched 2026-07-08. At launch Cartesia Sonic 3.5 was #1 at 1,122, Eleven v3 1,088,
  Inworld TTS-2 preview 1,070, Fish S2 Pro 1,034, Voxtral 1,024, Chatterbox 930 —
  [AlphaSignal, 2026-07-08](https://alphasignal.ai/news/artificial-analysis-fixes-the-benchmark-that-let-bad-tts-models-win).
- HeyGen Voice was added 2026-10-09 and went straight to #1 then. A secondary report says it ranked only
  10th of 29 on AA pronunciation robustness (secondary, not verified on AA) —
  [OrcaRouter blog](https://www.orcarouter.ai/blog/heygen-voice-tops-controlled-voice-arena).

**AA Pronunciation Robustness benchmark [INDEPENDENT]** —
[AA methodology](https://artificialanalysis.ai/methodology/text-to-speech/pronunciation-robustness-overview)
- Four categories: contextual disambiguation (heteronyms), text normalization (shorthand), sequence
  fidelity (codes, emails, paths), and term pronunciation (brands, places, technical names). The text is
  sent unnormalized. At least 3 blind third-party reviewers judge each clip Yes/No. The score is the Yes share.
- The methodology page shows no per-model scores, and its examples are English only. (An unverified
  LinkedIn claim that Eleven v4 is #1 on it is listed under Gaps.)

### Inferences
- With **cloned** voices, the gap between ElevenLabs v4 and Cartesia Sonic 3.6 / Inworld TTS-2 is inside
  the confidence intervals (1158 vs 1154 vs 1150). Alibaba Qwen-Audio-3.1 and HeyGen are measurably ahead.
- The older ElevenLabs low-latency models a product would actually use today (Flash v2.5, Turbo v2.5) rank
  near the anchor in the cloned-voice arena (≈ 982-986). Many cheaper models beat them, including Voxtral 3,
  Inworld TTS-2 Flash, Fish Drama 3, Gemini 3.8 and MiniMax 2.8.
- The native-voice arena rewards voice choice as much as synthesis. For our product the Controlled Voice
  arena is the more relevant of the two.

### Gaps
- Neither AA arena publishes per-language results. All voices and prompts in the Controlled arena are
  English. No independent arena covers pt-BR, ja, zh, hi or ar cloning.
- AA's "samples" column is not defined as votes vs appearances.
- Per-model AA Pronunciation Robustness scores could not be retrieved. A LinkedIn post (not verified) claims
  Eleven v4 is #1 there, and a blog says HeyGen Voice is 10th of 29.
- The Hugging Face TTS Arena V2 leaderboard did not load
  ([HF Space](https://huggingface.co/spaces/TTS-AGI/TTS-Arena-V2)), so I have no current numbers from it.
- The Labelbox speech leaderboard found was last updated 2025-03-11 (stale). Cartesia notes that ElevenLabs is
  a Labelbox customer —
  [Labelbox leaderboard](https://labelbox.com/leaderboards/speech-generation/),
  [Cartesia vs ElevenLabs](https://cartesia.ai/blog/cartesia-vs-elevenlabs).

## 2. Speaker similarity and WER/CER per language (incl. pt, ja, zh, ar, hi)

### Takeaway
No independent, multi-language cloning benchmark that re-measures current ElevenLabs models exists. The
only per-language ElevenLabs numbers are vendor-run. They come from two test sets: the MiniMax 24-language
test with **Multilingual v2** (May 2025), and Mistral's 9-language test with **Flash v2.5 and v3**
(Mar/Apr 2026). On both, every competitor's **automatic speaker similarity** to the reference voice beats
ElevenLabs, often by 0.1-0.3 cosine. ElevenLabs is competitive or best on WER in European languages,
including Portuguese, but weak on Japanese and Chinese (Multilingual v2), and v3 is weak on Hindi.

### Cited Findings
**MiniMax-Speech paper (arXiv 2505.07916, May 2025) [VENDOR — MiniMax]** —
[arXiv](https://arxiv.org/html/2505.07916)
- Test set: 24 languages, 100 sentences per language, two Common Voice speakers (one M, one F), zero-shot.
  ASR: Whisper-large-v3 (Paraformer-zh for Chinese). SIM: WavLM-large SV model (Seed-TTS-eval protocol).
- WER (MiniMax vs ElevenLabs Multilingual v2), SIM (MiniMax vs EL):
  - Chinese: WER 2.252 vs **16.026**; SIM 0.780 vs 0.677
  - English: 2.164 vs 2.339; 0.756 vs 0.613
  - Japanese: 3.519 vs **10.646**; 0.776 vs 0.738
  - Korean: 1.747 vs 1.865; 0.776 vs 0.700
  - Arabic: 1.665 vs 1.666; 0.736 vs 0.706
  - Spanish: 1.029 vs 1.084; 0.762 vs 0.615
  - French: 4.099 vs 5.216; 0.628 vs 0.535
  - Italian: 1.543 vs 1.743; 0.699 vs 0.579
  - **Portuguese: 1.877 vs 1.331 (EL better); 0.805 vs 0.711**
  - German: 1.906 vs 0.572 (EL better); 0.733 vs 0.614
  - **Hindi: 6.962 vs 5.827 (EL better); 0.818 vs 0.730**
  - Vietnamese 0.880 vs 73.415 and Thai 2.701 vs 73.936 (EL Multilingual v2 essentially fails)
- Seed-TTS-eval: MiniMax zero-shot test-zh CER 0.83 / SIM 0.783, test-en WER 1.65 / SIM 0.692
  (ground truth 1.25/0.750 and 2.14/0.730).

**Qwen3-TTS technical report (arXiv 2601.15621, Jan 2026) [VENDOR — Alibaba]** —
[arXiv](https://arxiv.org/html/2601.15621)
- Its MiniMax and ElevenLabs columns are **identical to the MiniMax paper** (e.g. zh 16.026, pt 1.331),
  so they are **[COPIED]**. The report does not say so.
- Qwen3-TTS-12Hz-1.7B-Base (open weights): WER zh 0.928, en 0.934, de 1.235, it 0.948, **pt 1.526**,
  es 1.126, **ja 3.823**, ko 1.755, fr 2.858, ru 3.212. SIM zh 0.799, en 0.775, de 0.775, it 0.817,
  **pt 0.817**, es 0.814, **ja 0.788**, ko 0.799, fr 0.714, ru 0.792. It is higher than ElevenLabs on SIM in all
  10 languages. On WER, EL wins de, pt, es (vs some variants) and ru.
- Seed-TTS-eval WER (zh / en): Seed-TTS 1.12/2.25, F5-TTS 1.56/1.83, CosyVoice 3 0.71/1.45,
  MiniMax-Speech 0.83/1.65, Qwen3-TTS-12Hz-1.7B 0.77/1.24. No Arabic or Hindi.

**Voxtral TTS paper (arXiv 2603.25551v2, 2026-04-06) [VENDOR — Mistral]** —
[arXiv](https://arxiv.org/html/2603.25551v2)
- Measured ElevenLabs **v3** and **Flash v2.5** directly. WER is from Voxtral Mini Transcribe v2 (Mistral's own
  ASR, a possible bias), SIM is ECAPA-TDNN, naturalness is UTMOS-v2. Format below is Voxtral / EL v3 / EL Flash v2.5:
  - Arabic: WER 2.68 / 3.67 / 2.86; SIM 0.746 / 0.546 / 0.539
  - Hindi: WER 4.99 / **8.71** / 5.08; SIM 0.839 / 0.707 / 0.679
  - Portuguese: WER 1.02 / 0.92 / 1.15; SIM 0.785 / 0.571 / 0.642
  - English: WER 0.63 / 0.48 / 0.33; SIM 0.786 / 0.484 / 0.489
  - Spanish: 0.51 / 0.87 / 0.49; SIM 0.762 / 0.443 / 0.541
  - French: 3.22 / 2.34 / 2.26; SIM 0.587 / 0.339 / 0.378
  - German: 0.83 / 0.45 / 1.08; SIM 0.721 / 0.457 / 0.489
  - Italian: 1.32 / 0.58 / 0.55; SIM 0.739 / 0.527 / 0.485
- The paper itself says Flash v2.5 "performs better on most automated metrics" (WER), while Voxtral leads on
  SIM in every row.
- Human eval of zero-shot cloning, Voxtral win rate vs ElevenLabs Flash v2.5: overall **68.4%**; es 87.8,
  hi 79.8, pt 74.4, ar 72.9, de 72.0, en 60.8, it 57.1, fr 54.4, nl 49.4. Each language used 2 voices and
  60 prompts, rated side by side by native-speaker annotators (Mistral's blog says 3 per pair) on
  naturalness, accent adherence and similarity.
  [Mistral blog](https://mistral.ai/news/voxtral-tts/)
- Flagship (non-cloned) voices: Voxtral vs EL v3 51.0% (explicit emotion), 55.4% (implicit). The paper
  calls Gemini 2.5 Flash TTS "the strongest model" (Voxtral only 35-37% vs Gemini).

**Confucius4-TTS (arXiv 2608.11650, 2026-08-12, NetEase Youdao) [VENDOR — NetEase; EL numbers COPIED]** —
[arXiv](https://arxiv.org/html/2608.11650v1)
- Re-uses the MiniMax 24-language set for 11 languages. It adds open/commercial baselines (run with reference
  transcripts): Qwen3-TTS, Fish Audio S2, OmniVoice, VoxCPM2.
  - Portuguese WER / SIM: Qwen3-TTS 1.53/0.805, Fish S2 1.57/0.777, OmniVoice 1.83/0.866,
    VoxCPM2 1.71/0.842 (vs EL Multilingual v2 1.33/0.711 copied).
  - Japanese CER / SIM: Qwen3 3.82/0.771, Fish S2 3.52/0.753, OmniVoice 3.59/0.821, VoxCPM2 3.51/0.825
    (EL 10.65/0.738).
  - Spanish: Fish S2 0.95/0.734, OmniVoice 0.81/0.814 (EL 1.08/0.615).
  - No Arabic or Hindi.
- Seed-TTS-eval (en WER/SIM; zh CER/SIM): Qwen3-TTS 1.24/0.714, 0.77/0.770; Fish S2 1.79/0.643,
  0.98/0.737; OmniVoice 1.62/0.740, 0.87/0.777; VoxCPM2 1.70/0.752, 0.97/0.793.

### Inferences
- **Portuguese:** ElevenLabs (Multilingual v2 and v3) has the lowest or near-lowest WER in every vendor
  table. Its weak point is speaker similarity, where it loses to every competitor by roughly 0.07-0.2.
- **Japanese / Chinese:** Multilingual v2 is clearly weak (ja CER 10.6, zh 16.0). Qwen3-TTS, MiniMax,
  Fish S2 and CosyVoice 3 are far better. I found no per-language ja/zh numbers for Flash v2.5 or v3/v4.
- **Hindi / Arabic:** only MiniMax (vs Multilingual v2) and Mistral (vs v3 and Flash v2.5) report them.
  ElevenLabs Flash v2.5 is close to Voxtral on WER (hi 5.08 vs 4.99, ar 2.86 vs 2.68), but EL v3 is worse
  on Hindi (8.71). Qwen3-TTS, Fish S2 and the open Chinese models report no hi/ar numbers.
- Automatic SIM systematically favours models that copy the reference timbre, and ElevenLabs may trade
  similarity for "clean" studio-like output. The perceptual study in §4 also found EL clones less similar.
  Several numbers are therefore consistent.
- Every per-language comparison with ElevenLabs is vendor-run and uses Common Voice references (noisy,
  read speech). Treat the WER/SIM tables as directional only.

### Gaps
- No independent per-language benchmark of Eleven v4, Cartesia Sonic 3.x, Inworld TTS-2, Google
  Gemini/Chirp 3, OpenAI gpt-4o-mini-tts, Hume Octave, Deepgram Aura or Gradium cloning in pt/ja/zh/hi/ar.
- No pt-BR-specific result: Common Voice Portuguese mixes pt-PT and pt-BR, and none of the papers says which.
- IndexTTS2 / IndexTTS 2.5, Higgs Audio and XTTS lack comparable multilingual SIM/WER tables against
  ElevenLabs in the sources read. The IndexTTS 2.5 report claims the lowest average cross-lingual WER over five
  target languages (vendor) — [arXiv 2601.03888](https://arxiv.org/html/2601.03888).
- The X-Voice 30-language paper claims WER on par with Qwen3-TTS (vendor/academic, not read in full) —
  [arXiv 2605.05611](https://arxiv.org/html/2605.05611v2).

## 3. Cross-lingual cloning (voice from language A speaking language B) and known weaknesses

### Takeaway
Cross-lingual numbers exist almost only for open/Chinese research models in zh/en/ja/ko directions.
The only human-rated cross-lingual comparison including ElevenLabs (Eleven v3) is a NetEase paper. There,
EL ranks 2nd-4th of 6, ahead of MiniMax 2.8 HD and Qwen3-TTS, but behind the authors' model and VoxCPM2.
Accent leakage (source accent carrying into target speech) is the main documented failure mode. Accent
attenuation (clones sound more "standard" than the speaker) is a separate documented effect.

### Cited Findings
- Human eval, cross-lingual cloning, overall average rank (1 = best, of 6) **[VENDOR — NetEase]**.
  Eleven v3 scored zh→en 2.95, en→zh 2.62, zh→ko 3.65, zh→ja 2.30. MiniMax Speech-2.8-HD: 4.15 / 4.17 /
  2.95 / 4.25. Qwen3-TTS: 4.43 / 4.28 / 5.85 / 5.85. VoxCPM2: 1.95 / 2.16 / 3.80 / 4.30 —
  [Confucius4-TTS](https://arxiv.org/html/2608.11650v1).
- CV3-Eval cross-lingual error rates, e.g. ja→zh CER: CosyVoice 2 48.1, CosyVoice 3-1.5B 6.78,
  OmniVoice 52.64, VoxCPM2 4.20. ko→en WER: CosyVoice 3-1.5B 13.7, OmniVoice 3.91 —
  [Confucius4-TTS](https://arxiv.org/html/2608.11650v1). Large failures occur on specific directions.
- Qwen3-TTS-12Hz-1.7B cross-lingual mixed error rate: zh→en 2.77, en→zh 4.77, zh→ja 8.40, en→ja 7.21,
  zh→ko 4.82. CosyVoice 3: zh→en 2.98, zh→ja 7.08, zh→ko 14.4 [VENDOR] —
  [Qwen3-TTS report](https://arxiv.org/html/2601.15621).
- MiniMax cross-lingual (Chinese speakers → other languages), zero-shot WER / SIM: Arabic 1.446/0.619,
  French 4.497/0.586, Vietnamese 0.659/0.692. One-shot raises SIM but also WER [VENDOR] —
  [MiniMax-Speech](https://arxiv.org/html/2505.07916).
- Accent leakage is defined as source-language phonetics persisting in target speech (IWSLT 2026
  cross-lingual track). Region-aware masking work reports that the unadapted baseline "exhibits strong accent
  leakage" and that suppressing accent can cost speaker timbre —
  [IWSLT 2026](https://aclanthology.org/2026.iwslt-1.8.pdf),
  [arXiv 2610.07524](https://arxiv.org/pdf/2610.07524).
- **Independent perceptual study (arXiv 2604.01562, 2026-04-02)** on Mandarin standard vs accented speakers
  cloned with ElevenLabs, MiniMax and AnyVoice (model versions not stated; ~20 s enrolment; 67 native
  listeners retained of 138). ElevenLabs clones diverged more from originals in ECAPA embeddings (~0.21 above
  within-speaker baseline; the others showed none). Listeners rated AnyVoice (β = 3.34) and MiniMax (β = 3.51)
  clones significantly **more similar** than ElevenLabs (p < .001). All clones were rated **more intelligible**
  than the originals, especially for accented speakers, which the authors read as "accent attenuation" —
  [arXiv 2604.01562](https://arxiv.org/html/2604.01562v1).
- Independent study (arXiv 2605.16578): 86 non-native English speakers cloned with ElevenLabs, Coqui XTTS and
  Chatterbox. It argues these models "systematically apply style transfer" to source voices (secondary reports
  say clones were rated more humanlike than originals; not verified) —
  [arXiv 2605.16578](https://arxiv.org/html/2605.16578v2).
- ElevenLabs Multilingual v2 essentially fails on Vietnamese and Thai (WER ~73%) in MiniMax's test
  [VENDOR], an example of hallucinated or garbled output in unsupported languages —
  [MiniMax-Speech](https://arxiv.org/html/2505.07916).

### Inferences
- For translation calls, the user hears their own voice in a language they may not speak, so slight accent
  attenuation is acceptable or even desirable. The risk is the opposite: source accent leaking into the
  target, or low identity similarity.
- Evidence points to ElevenLabs favouring intelligibility and cleanliness over identity: lower SIM in
  every vendor table, and lower perceived similarity in the one independent listener study.
  Competitors (MiniMax, Qwen, Voxtral) favour timbre match.

### Gaps
- No independent cross-lingual benchmark covers pt-BR↔en/es, or any direction into Hindi or Arabic, with
  commercial APIs.
- No systematic, independent data on number/date/name handling across languages. AA Pronunciation
  Robustness is English-only and its per-model scores were not retrievable.
- No hallucination-rate (skips, repeats, babble) metrics found for Eleven v3/v4, Cartesia or Inworld.

## 4. Latency: time to first audio (TTFA)

### Takeaway
Two independent operators (AA and Coval) measure perceived TTFA including leading silence. Their numbers
disagree on ordering. In both, Inworld TTS-2 and ElevenLabs' fast tier (Flash v2.5 / v4 Turbo) are among
the fastest high-quality options at ~170-260 ms. Cartesia Sonic 3.6 runs ~270-440 ms. Flagship Eleven v4
(653 ms), Voxtral 3 (455-576 ms), Gemini 3.1 Flash TTS (908-975 ms) and OpenAI gpt-4o-mini-tts (P95 3.9 s)
are slow. All measurements use English text with stock voices, not cloned voices or other languages.

### Cited Findings
- **AA TTFA leaderboard** (article 2026-11-16) **[INDEPENDENT]**. It measures from full-text submit to the first
  audible 10 ms window above −40 dBFS. Each model runs once a day with 12 prompts in 4 length bands and a random
  English voice; the score is a 7-day rolling mean; region is not stated. Results: Speechify Simba 3.5 116 ms;
  Inworld Realtime TTS-2 186 ms; Eleven v4 Turbo 258 ms; Fish Drama 3 258 ms; Cartesia Sonic 3.6 287 ms;
  Eleven v4 653 ms; Mistral Voxtral 3 455-576 ms; Gemini 3.1 Flash TTS 908-975 ms. Medians: 376 ms over
  65 streaming models, 2.0 s over 20 buffered models —
  [AlphaSignal summary of AA](https://alphasignal.ai/news/artificial-analysis-tracks-how-fast-voice-ai-actually-starts-speaking).
  The AA live page was not read directly.
- **Coval TTS leaderboard**, read 2026-09-08 and reported by Gradium, a competitor whose page republishes
  it **[INDEPENDENT operator, VENDOR-relayed]**. Perceived TTFA P50 (P95), 480 samples per model, 30 English
  prompts, connection setup excluded; avg WER in brackets:
  - Inworld TTS Flash 2: 75 (111) ms [5.3%]; Inworld TTS 2: 170 (230) [4.5%]
  - ElevenLabs Flash v2.5: 185 (225) [6.5%]; Gradium: 214 (346) [4.9%]
  - Cartesia Sonic 3.5: 269 (346) [5.8%]; Deepgram Aura-2: 290 (522) [5.0%]
  - Fish Audio S2.1 Pro: 293 (586) [4.7%]; ElevenLabs v3 Conversational: 320 (393) [4.3%]
  - Cartesia Sonic 3.6: 440 (696) [5.3%]; OpenAI GPT-4o mini TTS: P95 3,927 ms [4.9%]
  - Source: [Gradium latency benchmark 2026](https://gradium.ai/content/tts-latency-benchmark-2026)
- Gradium's own test (Paris, 2026-03-24, 100 queries) **[VENDOR]**: ElevenLabs Turbo v2.5 P50 257 ms,
  Flash v2.5 277 ms (excluding connection setup); Gradium 214 ms —
  [Gradium](https://gradium.ai/content/tts-latency-benchmark-2026).
- Picovoice's open benchmark **[VENDOR — Picovoice Orca]** reports ElevenLabs first-token-to-speech of
  1,470 ms (LLM-streaming simulation) on one page and 335 ms on another, which is inconsistent. It does not
  include Cartesia —
  [Picovoice benchmark](https://picovoice.ai/docs/benchmark/tts-latency),
  [GitHub](https://www.github.com/Picovoice/tts-latency-benchmark).
- Mistral reports Voxtral TTS 70 ms model latency at concurrency 1 on an H200 (331 ms at concurrency 16),
  self-hosted, not end-to-end API TTFA [VENDOR] —
  [Voxtral paper](https://arxiv.org/html/2603.25551v2).
- Cartesia's 2024 self-comparison (Sonic 199 ms vs ElevenLabs 832 ms) is outdated [VENDOR] —
  [Cartesia blog](https://cartesia.ai/blog/cartesia-vs-elevenlabs).

### Inferences
- Within the real-time budget of a translation call (STT + MT + TTS), the realistic candidates at
  ≤ ~300 ms TTFA are: ElevenLabs Flash v2.5 / v4 Turbo, Inworld TTS-2 (and Flash), Cartesia Sonic 3.5/3.6,
  Fish Drama 3 / S2.1 Pro, Gradium and Deepgram Aura-2. Eleven v4 (non-Turbo), Gemini TTS, Voxtral 3 via API
  and gpt-4o-mini-tts are too slow on these measurements.
- Cartesia Sonic 3.6's TTFA differs between AA (287 ms) and Coval (440 ms), likely because of region or
  network. Measure from our own host region before committing.

### Gaps
- No independent TTFA for cloned voices (instant clones may add latency) or for non-English text.
- No independent TTFA found for MiniMax Speech 2.8, Hume Octave 2 or Google Chirp 3 HD.
- The AA per-model latency table itself was not fetched, only a dated summary of it.

## 5. Methodology caveats (dates, who ran what, sample sizes)

### Takeaway
The only genuinely independent, current, large-sample data (AA arenas and TTFA, Coval) is English-only.
Every multilingual WER/SIM comparison with ElevenLabs is vendor-run. Most re-use one 2025 MiniMax
measurement of a now-superseded model (Multilingual v2).

### Cited Findings
- The AA arenas are blind pairwise Elo with ~1-8k samples per model and 95% CIs of about ±11-18 Elo, English
  prompts only. The Controlled Voice arena uses 8 English cloned voices and has run since 2026-07-08 —
  [AA leaderboard](https://artificialanalysis.ai/text-to-speech/leaderboard),
  [AA Controlled Voice](https://artificialanalysis.ai/text-to-speech/leaderboard/controlled-voice),
  [AlphaSignal](https://alphasignal.ai/news/artificial-analysis-fixes-the-benchmark-that-let-bad-tts-models-win).
- The MiniMax 24-language set uses 100 sentences × 2 Common Voice speakers per language. The ElevenLabs
  model is Multilingual v2, measured around May 2025 by MiniMax —
  [MiniMax-Speech](https://arxiv.org/html/2505.07916).
- The Qwen3-TTS (Jan 2026) and Confucius4-TTS (Aug 2026) ElevenLabs/MiniMax columns are copied from the
  MiniMax paper (Confucius4 states this; Qwen's values are identical) —
  [Qwen3-TTS](https://arxiv.org/html/2601.15621),
  [Confucius4-TTS](https://arxiv.org/html/2608.11650v1).
- Mistral's ElevenLabs comparison is the only recent multilingual one measured fresh (v3 and Flash v2.5).
  But it uses Mistral's own ASR for WER and only 2 voices per language, with an unstated annotator count for
  cloning — [Voxtral paper](https://arxiv.org/html/2603.25551v2).
- Different SIM models (WavLM-large SV vs ECAPA-TDNN) give different absolute scales. Do not compare SIM across
  papers (e.g. EL Portuguese 0.711 in MiniMax vs 0.571/0.642 in Mistral).
- Hugging Face's TTS Arena notes that WER-like objective metrics are unreliable measures of quality, hence
  its listener votes — [HF blog](https://huggingface.co/blog/arena-tts).

### Inferences
- To decide whether a provider "matches ElevenLabs" in pt-BR, ja, zh, hi and ar for our product, public data
  is insufficient. A small in-house blind test is needed: the same caller references, cross-lingual
  pt-BR↔en/es and so on, native raters, plus WER via our own STT and SIM via a fixed SV model.
- The public evidence supports shortlisting these against ElevenLabs Flash v2.5 / v4 Turbo:
  - Inworld TTS-2: arena parity, fastest TTFA
  - Cartesia Sonic 3.6: arena parity
  - Qwen-Audio-3.1 or open Qwen3-TTS: best cloned-voice Elo, strong ja/zh, no hi/ar data
  - Mistral Voxtral 3: strong SIM, hi/ar/pt covered, slower TTFA
  - MiniMax Speech 2.8: broad language coverage, strong SIM, lower arena Elo

### Gaps
- No public data on clone quality from short, noisy in-call references (vs clean Common Voice or studio clips).
- No independent multilingual arena (non-English listeners) exists as of this date in the sources found.
