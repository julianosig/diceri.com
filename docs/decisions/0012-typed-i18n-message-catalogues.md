---
status: accepted
date: 2026-11-26
---

# Typed i18n message catalogues without a library

## Context and problem statement

Diceri sells "the call you know, in every language", so its own interface cannot be
English only. AGENTS.md §2 already reserved UI copy for i18n message files. We need
translations for the main languages, a language picker, right-to-left support (Arabic),
and no extra weight for people who never switch.

## Considered options

- Paraglide JS (compiled messages, tree-shaken).
- svelte-i18n / i18next (runtime ICU strings, stores).
- A small typed module of our own: one TypeScript catalogue per language.

## Decision outcome

Chosen option: "a small typed module", because the copy is small (one page today), the
compiler can already prove every language is complete, and it adds no dependency or
build step to keep up with SvelteKit 3.

- `apps/web/src/lib/i18n/messages/en.ts` is the source; its shape is the `Messages`
  type, and every other catalogue is typed `Messages`, so a missing key fails
  `svelte-check`. Interpolation is plain functions (`available(handle)`).
- English ships in the main bundle; every other language is a lazy chunk.
- The language is the saved choice (`localStorage` `diceri-locale`), else the browser's
  (`navigator.languages`, primary subtag), else English. The root layout awaits it before
  the first render, and sets `lang` and `dir` on `<html>`.
- Languages: en, es, pt, fr, de, it, ru, ja, ko, zh (Simplified), hi, ar (RTL).

### Consequences

- Good, because adding a string is adding a key to `en.ts`; the compiler lists every
  catalogue that still needs it.
- Good, because a visitor downloads one extra ~2 kB chunk at most.
- Bad, because plurals and gender need hand-written functions (no ICU). Revisit with
  Paraglide if the copy grows enough to need translators' tooling.
- Bad, because translations are written by maintainers; native review is welcome.

### Confirmation

`svelte-check` fails on an incomplete catalogue; `locales.spec.ts` loads every language
and checks browser-preference matching.
