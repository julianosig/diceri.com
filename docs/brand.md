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
`static/logo-dark.svg` (dark) are standalone copies for sharing.

## Themes

Light and dark are design-system themes: every colour is a token in
`apps/web/src/routes/layout.css`, and the `dark` class on `<html>` swaps them.
`ThemeToggle` switches and remembers the choice; without one, the system preference
wins. An inline script in `app.html` applies it before first paint.

## Home illustration

`static/hero/diceri-banner.{webp,gif}` (+ a still frame for reduced motion): a 6:1 strip
where "Olá!" from Brazil goes through Diceri and arrives as "こんにちは!" in Japan.
Generated with Codex as a 4-frame sheet stacked vertically (the first, taller 2×2
version was cut to this strip to keep the promise above the fold).

## Promise

> The call you already know. **Now in every language.**
> Talk the way you always do. Everyone hears you live in their own language — in your
> own voice.

It anchors on something everyone has used (a video call) and names the moat: live
translation spoken back in the speaker's own (consented, cloned) voice.

## Mark

`apps/web/src/lib/assets/favicon.svg`: a twenty-sided die
whose outline is a speech bubble. Speaking is the product; the d20 is a nod to the
geek culture of the room mascots. Violet `#7c3aed` and blue `#2563eb` facets with
white gaps. The concept was explored with Codex image generation and redrawn by hand
as a 1 kB vector so it stays sharp from favicon to header.

## Room mascots

`apps/web/static/mascots/<name>.gif`: twelve **original** characters inspired by
pop-culture archetypes (wizard, robot, astronaut, alien, ninja, dragon, knight, ghost,
cyber cat, slime, space pirate, gamer). They are deliberately not based on any existing
copyrighted or trademarked character.

How they were made, so they can be extended consistently:

1. Codex (`codex exec`, image generation) produced a 6-frame horizontal sprite sheet
   per character (6 × 256 px, flat chibi vector style, thick outlines, one solid pastel
   background per mascot, same pose and size in every frame).
2. The frames were re-centred and encoded as a looping 128 px GIF (140 ms per frame,
   one shared 128-colour palette), 14–35 kB each.

Adding a mascot: generate it the same way, add the name to `Mascot` in
`crates/domain/src/mascot.rs` and to `MASCOTS` in
`apps/web/src/lib/features/room/mascots.ts`, and drop the GIF in `static/mascots/`.
