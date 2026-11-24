# The Lost Cities — a Horizon 26.3 port

> Generate cities all over the world. Upstream: **McJty**,
> <https://github.com/McJtyMods/LostCities> (MIT).
> The cached artefact is The Lost Cities `26.3-12.0.2` for NeoForge.

This is a **clean-room Rust rewrite** of The Lost Cities for Horizon (Minecraft
Java 26.3), pinned to SDK `0a875da91dbb0fdcb7ba8c42c11341aa229af51a` and
horizon-mixins `e59da9094f0c2fdda6c627fe9eff22dec6a906f0`. It is a first
publication at version **0.1.0**; the city-generator slice shipped as **0.1.1**, the
second multi-layer building part as **0.1.2**, the horizontal rail line as **0.1.3**,
the rail bend as **0.1.4**, the three-way rail split as **0.1.5** and the
eighteen-layer rail descent as **0.1.6**.

## What this version implements

- Carries the mod's six block tags (`lostcities:lights`, `notbreakable`,
  `easybreakable`, `foliage`, `needspoi`, `rotatable`) verbatim from the jar.
- Carries the two chest loot tables (`lostcitychest`, `raildungeonchest`) and the
  `en_us.json` language file. The loot tables need one 26.3 entry-type fix
  (`"item"` → `"minecraft:item"`), recorded under *Upstream defects fixed* in
  `notes.md`; both yield 3–4 items in game.
- Registers the **`lostcities:lostcity` dimension** from the jar's
  `dimension_type/lostcity.json` values through `Registrar::register_dimension`
  (min_y 0, height 256, ambient light 0, the jar's sky/fog/cloud colours and
  spawn rules). The jar's dimension namespace is declared in the manifest's
  `namespaces` so the carried tags load.
- **A deterministic city chunk generator** (0.1.1): the dimension's generator
  id is `lostcities:lostcity`, registered with `Registrar::register_chunk_generator`.
  A clean-room JSON decoder in `src/runtime.rs` reads the jar's
  `parts/street_straight.json` with its `S`/`b` palette entries, and
  `build_terrain` lays that 16x16 part across the level at y=90 with no vanilla
  stages. Registering a chunk generator needs no `[native] level_reset` manifest
  key: that key is only for resetting vanilla levels, which this port does not
  do (R11 gate passes — see `notes.md`).
- **A second, six-layer building part** (0.1.2): `parts/building1_1.json`
  (16x16, six slices) is decoded with a merged palette copied from
  `bricks_standard`, `common`, `glass_full` and `glass_side_variant_bricks` — the
  groups `styles/standard.json` selects together — and laid at y=72. Its `#`
  char is a `{"variant":"stonebrick"}` entry resolved through the jar's
  `variants/stonebrick.json`, and `@` uses the jar's `frompalette` form. The
  decoder now handles `variant`, `frompalette`, weighted `blocks` and multi-layer
  bounds; no new SDK API or manifest key is used.
- **A third, six-layer horizontal rail line** (0.1.3): `parts/rails_horizontal.json`
  (16x16, six slices, non-space chars `T`/`e`/`f`/`g`/`y`/`~`) is decoded with a
  palette carried verbatim from `palettes/rails.json` (all twelve rail entries),
  `y` from `palettes/default.json` (a `variant: stonebrick`) and `T`/`g`/`~`/space
  from `palettes/common.json`, and laid at y=54. The artefact's `e` is
  `minecraft:rail[shape=east_west]` and its `f` is
  `minecraft:powered_rail[powered=true,shape=east_west]`; the brief's illustrative
  ids were wrong and were not used. The in-game assertion fill-replaces
  `minecraft:rail` at y=50..64, where vanilla noise fills nothing.
- **A fourth, six-layer rail bend** (0.1.4): `parts/rails_bend.json`
  (16x16, six slices, `refpalette: "rails"`) is decoded with the same verbatim
  `palettes/rails.json` entries and laid at y=36. Its `k`/`r`/`s`/`m` chars
  resolve to `minecraft:rail[shape=north_south]`, `rail[shape=north_west]`,
  `rail[shape=south_west]` and `powered_rail[powered=true,shape=north_south]`
  — rail shapes no other decoded part writes. The in-game assertion fill-replaces
  `rail[shape=north_south]` at y=30..44, where vanilla noise fills nothing.
  This time the brief's example ids matched the artefact; they were checked
  against the jar before the carry.

- **A fifth, six-layer three-way rail split** (0.1.5): `parts/rails_3split.json`
   (16x16, six slices, `refpalette: "rails"`, embedded byte-identically from the
   cached jar, sha256 `b0c8bd61…`) is decoded with the same verbatim
   `palettes/rails.json` entries and laid at y=18. Its `n`/`q` chars resolve to
   `minecraft:rail[shape=north_east]` and `rail[shape=south_east]` — the two rail
   shapes none of the other four decoded parts writes. The in-game assertion
   fill-replaces `rail[shape=north_east]` at y=12..26, where vanilla noise fills
   nothing. No new decoder form, SDK API or manifest key was needed.

- **A sixth, eighteen-layer rail descent** (0.1.6): `parts/rails_down2.json`
   (16x16, **18 slices**, `refpalette: "rails"`, embedded byte-identically from
   the cached jar, sha256 `f2f6eba…`) is decoded with the same verbatim
   `palettes/rails.json` entries and laid at y=4. Its `j`/`i` chars resolve to
   `minecraft:rail[shape=ascending_east]` and
   `minecraft:powered_rail[powered=true,shape=ascending_east]` — shapes none of
   the other five decoded parts writes. The in-game assertion fill-replaces
   `rail[shape=ascending_east]` at y=0..24, where vanilla noise fills nothing.
   The decoder's layer bound is generic, so no new decoder form, SDK API or
   manifest key was needed.

## What it does **not** implement

- **The other 202 parts, 38 buildings, 31 palettes, 15 multibuildings, the
  spheres/rail dungeons and Java-assembled structures** — still
  `blocked(no-city-data-decoder)` / `blocked(no-structure-type-for-cities)`:
  this port decodes exactly six parts and the palette entries they use.
- **Replacing vanilla terrain in the Overworld** — `blocked(level_reset:R11)`:
  the Java mod's mixins replace vanilla levels; the pin has no `levels_reset`
  surface and R11 refuses the manifest key. This port generates cities only in
  its own registered `lostcities:lostcity` level.
- **The `lostcities:lostcity` and `lostcities:spheres` features and their two
  biome modifiers** — `configured-feature-not-registered`: the pin has no
  feature/placement-modifier registrar (0 API matches), so those data files are
  deliberately not packed.
- The config GUI, its 18 textures, the editor/commands/network classes — out of
  tier for this port.

## Install

Build with `cargo horizon-mod build` against the pinned SDK and install the
resulting `the_lost_cities.hmod` with a Horizon launcher using the same pin.
See `notes.md` for the full jar inventory, the measured registrar surface, the
coverage table and the verification evidence; `PROVENANCE.md` accounts for
every carried byte.

## Layout

```
horizon-mod.toml       flat SDK manifest (api = "0.2", [native] pins, namespaces)
Cargo.toml             SDK-only dependency (registry rule R9)
src/lib.rs             the one impl Hooks: dimension + city chunk generator
src/runtime.rs         pure dimension facts, JSON part/palette decoder + tests
data/lostcities/       carried tags and loot tables
assets/lostcities/     carried language file
tools/carry.py         jar -> 26.3 carry and its --check data contract
tools/e2e.sh           in-game, fail-capable assertions
```
