You are an independent, skeptical reviewer. Today is 2026-11-25. Do NOT simply agree with the documents; your job is to find where they are wrong, outdated or suboptimal. Use web search to verify current facts (models, prices, benchmarks, language lists) yourself.

Product: Diceri, a Google-Meet-like app. Anonymous rooms joined by shared link (1:1 initially). Each participant's speech is captured; the other listener chooses which language to hear them in, and should hear it in a voice that sounds like the original speaker (consented voice clone). It will become a paid product and must be competitive (vs Google Meet translation, Microsoft Teams interpreter, Palabra, Kudo, Wordly, ElevenLabs, etc.).

Read these files in the repo (read-only):
- docs/architecture.md
- docs/decisions/0004-livekit-media-plane.md
- docs/decisions/0005-speech-translation-provider-stack.md
- docs/decisions/0006-voice-cloning-consent.md
- docs/research/2026-11-multilingual-stack/report.md
- docs/research/2026-11-voice-translation-stack/report.md
(notes/ subfolders contain sources if you need them)

Evaluate, with evidence and dates, the balance of: multilingual support (breadth and per-language quality), voice fidelity (speaker similarity, cross-lingual accent), translation quality, latency/performance (glass-to-glass), reliability, and unit economics for a paid product.

Answer precisely:
1. Architecture: is cascade (streaming STT -> LLM MT -> voice-clone streaming TTS) + LiveKit SFU + Rust translator agent the best choice vs managed/end-to-end speech-to-speech (Gemini 3.5 Live Translate, OpenAI gpt-realtime-translate, ElevenLabs realtime/agents, Palabra, Azure), or a hybrid? Any architectural flaw (latency budget, turn-taking, overlap/barge-in, echo, original-audio ducking, scaling, Rust vs Python agent)?
2. For each stage, is the chosen default the best balance? STT: Soniox stt-rt-v6. MT: Gemini 3.5 Flash-Lite (minimal thinking). TTS: ElevenLabs eleven_v4_turbo. Name any better or missing candidate with evidence (incl. anything released in Oct-Nov 2026).
3. Are the fallbacks and per-language routing rules sound? Anything over- or under-engineered?
4. Biggest risks to user experience and to competitiveness once charging, and what to change.
5. Any factual errors you found in the documents (cite the line/claim and the correct fact with source).

Output: a concise report in English with sections 1-5, a final verdict (Keep / Change) per stage, and a confidence level per verdict. Cite URLs for every factual claim. Do not modify any files.
