# Brand and assets

## Name

**Diceri** is a morphological variation of Latin *dīcere*, "to say, to speak,
to declare", chosen as a domain hack for `diceri.com`.

## Logo

The lockup is one SVG: the mark, the wordmark **Diceri** and its meaning underneath,
*from Latin dicere: to speak*. All text is outlines (Inter, SIL OFL 1.1, converted with
opentype.js into `apps/web/src/lib/components/ui/logo-paths.ts`), so it renders the same
everywhere without fonts. `Logo.svelte` inlines it and follows the theme (text in the ink
colour, facet gaps in the surface colour); `static/logo.svg` (light) and
`static/logo-dark.svg` (dark) are standalone copies for sharing. `layout="stacked"`
puts the wordmark centred under the mark (used in the home banner).

## Themes

Light and dark are design-system themes: every colour is a token in
`apps/web/src/routes/layout.css`, and the `dark` class on `<html>` swaps them.
`ThemeToggle` switches and remembers the choice; without one, the system preference
wins. An inline script in `app.html` applies it before first paint.

## Home illustration

A 6:1 strip (`HeroAnimation.svelte`), all vector or code, so it has no background and stays
sharp at any size. The two people are Codex drawings with a transparent background,
traced to SVG with `vtracer` (`src/lib/assets/hero/{left,right}-person.svg`, ~45 kB each
gzipped, imported so the build hashes their names); one pair serves both themes. Before tracing, the drawing gets a binary alpha and the
soft light halo the generator leaves along the outline is painted in the outline colour
(at 2× size), otherwise the trace shows broken, jagged edges on light backgrounds. The
middle is the real mark with the wordmark under it (`Logo layout="stacked"`), with a CSS
halo, sparkles and dotted SVG links that flow. The speech (`HeroSpeech.svelte`) is HTML
on top: the speaker talks (the drawing has no mouth; a resting smile and an open,
moving mouth plus voice waves are drawn in code,
`HeroPerson.svelte`), a line said on the left travels into Diceri, Diceri glows, and the same line
arrives translated at the listener, labelled with each language. Pairs live in
`language-pairs.ts`; add one there, no new art needed. Reduced motion shows both lines
at once.

## Promise

> **The Google Meet alternative**
> **where you talk and listen in your own language.**
> Everyone in the meeting hears your voice in their language.

Written to the ICP in the second person. The first line is what people already search
for and compare against (a nominative, comparative use of the name: no Google logo,
colours or implied affiliation); the second says the mechanism both ways in words a
first call proves; the line under it names the moat, live translation spoken in the
speaker's own (consented, cloned) voice. Each line stays on one line in every language
(`Headline.svelte` sizes it from its width).

## Mark

`apps/web/src/lib/assets/favicon.svg`: a twenty-sided die
whose outline is a speech bubble. Speaking is the product; the d20 is a nod to the
geek culture of the room mascots. Violet `#7c3aed` and blue `#2563eb` facets; the
gaps take the background colour (white on light tabs, near-black on dark ones). The concept was explored with Codex image generation and redrawn by hand
as a 1 kB vector so it stays sharp from favicon to header.

## Room mascots

`apps/web/static/mascots/{webp,gif}/<name>.*` (transparent): twelve **original** characters inspired by
pop-culture archetypes (wizard, robot, astronaut, alien, ninja, dragon, knight, ghost,
cyber cat, slime, space pirate, gamer). They are deliberately not based on any existing
copyrighted or trademarked character.

How they were made, so they can be extended consistently:

1. Codex (`codex exec`, image generation) produced a 6-frame horizontal sprite sheet
   per character (6 × 256 px, flat chibi vector style, thick outlines, one solid pastel
   background per mascot, same pose and size in every frame).
2. The solid background was removed by flood fill from the frame edges (it stops at the
   dark outline, so light colours inside a character survive), the frames re-centred and
   encoded as looping transparent animated WebP at 128 px (`webp/`, 12–35 kB) and at the
   native 250 px (`webp-2x/`, 15–60 kB), plus a 128 px GIF with 1-bit transparency as
   fallback, 140 ms per frame. `MascotAvatar` offers both WebPs in `srcset`, so small
   avatars stay light and large or high-density ones stay sharp. Shown at most 128 px.

Adding a mascot: generate it the same way, add the name to `Mascot` in
`crates/domain/src/mascot.rs` and to `MASCOTS` in
`apps/web/src/lib/features/room/mascots.ts`, and drop the files in `static/mascots/webp/`, `webp-2x/` and `gif/`.
