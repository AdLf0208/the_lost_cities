# The Lost Cities — a Horizon 26.3 port

> Generate cities all over the world. Upstream: **McJty**,
> <https://github.com/McJtyMods/LostCities> (MIT).
> The cached artefact is The Lost Cities `26.3-12.0.2` for NeoForge.

This is a **clean-room Rust rewrite** of The Lost Cities for Horizon (Minecraft
Java 26.3), pinned to SDK `0a875da91dbb0fdcb7ba8c42c11341aa229af51a` and
horizon-mixins `e59da9094f0c2fdda6c627fe9eff22dec6a906f0`. It is a first
publication at version **0.1.0**.

## What this version implements

- Carries the mod's six block tags (`lostcities:lights`, `notbreakable`,
  `easybreakable`, `foliage`, `needspoi`, `rotatable`) verbatim from the jar.
- Carries the two chest loot tables (`lostcitychest`, `raildungeonchest`) and the
  `en_us.json` language file. The loot tables need one 26.3 entry-type fix
  (`"item"` → `"minecraft:item"`), recorded under *Upstream defects fixed* in
  `notes.md`; both yield 3–4 items in game.
- Registers the **`lostcities:lostcity` dimension** from the jar's
  `dimension_type/lostcity.json` values through `Registrar::register_dimension`
  (min_y 0, height 256, ambient light 0, multi-noise Overworld biomes, the
  jar's sky/fog/cloud colours and spawn rules). The jar's dimension namespace
  is declared in the manifest's `namespaces` so the carried tags load.

## What it does **not** implement

- **City terrain generation** — `blocked(level_reset:R11)`: the Java mod's
  mixins and 322 internal `parts`/`buildings`/`palettes` JSON files replace
  vanilla terrain; the pin has no `levels_reset` surface and R11 refuses the
  manifest key. The registered dimension uses the vanilla noise generator
  declared in the jar's own dimension file — **no cities are generated**.
- **The `lostcities:lostcity` and `lostcities:spheres` features and their two
  biome modifiers** — `configured-feature-not-registered`: the pin has no
  feature/placement-modifier registrar (0 API matches), so those data files are
  deliberately not packed.
- The 322 internal generator JSON files — `blocked(no-city-data-decoder)`: no
  decoder for McJty's private schema at this pin.
- The config GUI, its 18 textures, the editor/commands/network classes and the
  structure types assembled in Java — out of tier for a first data slice.

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
src/lib.rs             the one impl Hooks: register() for the dimension
src/runtime.rs         pure dimension facts + unit tests
data/lostcities/       carried tags and loot tables
assets/lostcities/     carried language file
tools/carry.py         jar -> 26.3 carry and its --check data contract
tools/e2e.sh           in-game, fail-capable assertions
```
