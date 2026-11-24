# The Lost Cities — port notes (first slice)

Skills: superpowers/using-superpowers, brainstorming, writing-plans, test-driven-development, systematic-debugging, verification-before-completion; BeautifulRust/rust-best-practices

**Class + tier:** `content` / `deep` (worldgen: dimension, biome source, structures, city generation).
Ledger row `ports/ledger.csv:470`: `the_lost_cities,The Lost Cities,content,deep,MIT,pending,P1,,ports/the_lost_cities`;
source map `ports/_shared/source-map/atm10-sources.csv:459`
(`McJtyMods/LostCities`, Modrinth `8VDCUS3b`). Approach from
`horizon/docs/modpacks.md` §"Tiers"/§"Carry-over" (lines 58, 74, 183, 269-279, 505-541)
and `docs/mods.md`/`docs/native-mods.md` (content `.hmod` vs code, `api = "0.2"`):
carry the jar's data/assets that the pin can decode, plus pure registrar glue for the
families the SDK exposes; no Java translation, no decompilation. The precedent for an
honest data worldgen port at this pin is `ports/tectonic/` (first slice 2026-11-24).

**Pin:** Horizon `0a875da91dbb0fdcb7ba8c42c11341aa229af51a` + horizon-mixins
`e59da9094f0c2fdda6c627fe9eff22dec6a906f0`, `api = "0.2"`, game 26.3, package format 6.
Do not repin mid-wave: ten other agents build against `0a875da`, and the additive
nightly `f1bb66f353cf…` adds `levels_reset` while R11 refuses the manifest key.

**Licence: MIT (rank 0).** The jar's `META-INF/neoforge.mods.toml` says
`license="MIT License https://github.com/McJtyMods/LostCities/blob/1.21_neo/LICENSE"`
and `authors="McJty"`; Modrinth `8VDCUS3b` declares MIT. Dispatch gate exit 0; the
id is on neither upstream `main` nor any open PR, so this is a **first publication**
at version **0.1.0** (the orchestrator publishes; this port does not touch the
registry, PRs or ledger). Credit in README: McJty, <https://github.com/McJtyMods/LostCities>.

**Artifact:** `ports/_shared/licence-cache/the-lost-cities.jar` — **1,414,750 B**,
SHA-256 `666cd25d4915a2204693968d0052bbd78ea2e8df98c3537bad3943c078164bfd`
(verified 2026-11-24). The Modrinth version is `26.3-12.0.2` (NeoForge); the jar's
own `neoforge.mods.toml` declares a 26.3 version range. The jar is the authority.

## Jar inventory — measured with `unzip -Z1`/`unzip -l` before any family was chosen

741 entries, 686 files. The mod is a **Java core plus data**:

| Location | Files | What it is |
|---|---:|---|
| `data/lostcities/lostcities/**` | 322 | the Java generator's own JSON data format: `parts` 208, `buildings` 38, `palettes` 31, `multibuildings` 15, `variants` 13, `scattered` 6, `citystyles` 6, `styles` 5, `stuff` 4, `conditions` 4, `worldstyles` 3 |
| `mcjty/lostcities/**` | 321 | Java core: `worldgen` 215, `api` 27, `varia` 20, `gui` 20, `commands` 15, `setup` 13, `config` 12, `editor` 6, `network` 3, `datagen` 3, `mixin` 2, `data` 2 + root classes — analysis aid only |
| `data/lostcities/tags/block/*.json` | 6 | block tags: `easybreakable`, `foliage`, `lights`, `needspoi`, `notbreakable`, `rotatable` |
| `assets/lostcities/lang/*` | 6 | `en_us.json` (one key, `generator.lc_cavern`) + 5 legacy `.lang` files |
| `assets/lostcities/textures/gui/*.png` | 18 | GUI icons/config icon (the config screen is Java) |
| `data/lostcities/worldgen/feature/*.json` | 2 | `lostcities.json` (`{"type":"lostcities:lostcity"}`), `spheres.json` (`{"type":"lostcities:spheres"}`) — **custom Java feature types** |
| `data/lostcities/worldgen/placed_feature/*.json` | 2 | `lostcities`, `spheres` — `{"feature":"lostcities:…","placement":[{"type":"minecraft:count","count":1}]}` |
| `data/lostcities/neoforge/biome_modifier/*.json` | 2 | `neoforge:add_features` placing `lostcities:lostcities` at `raw_generation` and `lostcities:spheres` at `top_layer_modification` on `#minecraft:is_overworld` |
| `data/lostcities/loot_table/chests/*.json` | 2 | `lostcitychest`, `raildungeonchest` — vanilla-format loot pools of vanilla items (`type:"item"`, 1.21.1 `modifier` key) |
| `data/lostcities/dimension/lostcity.json` | 1 | `{"type":"lostcities:lostcity","generator":{"type":"minecraft:noise","seed":0,"settings":"minecraft:overworld","biome_source":{"type":"minecraft:multi_noise","preset":"overworld"}},"forge:use_server_seed":true}` |
| `data/lostcities/dimension_type/lostcity.json` | 1 | min_y 0, height 256, logical_height 256, ambient_light 0.0, skylight, no ceiling, coordinate_scale 1.0, `#minecraft:infiniburn_overworld`, monster_spawn_block_light_limit 2, respawn anchors/portal piglins true, raids false, sky `#78a7ff`, fog `#c0d8ff`, cloud height 192.33, `timelines: #minecraft:in_overworld`, `default_clock: minecraft:overworld` |
| `META-INF/*` | 3 | `neoforge.mods.toml`, `MANIFEST.MF`, `accesstransformer.cfg` |

**Census facts:** the mod registers **no** items, blocks, models, recipes,
advancements, structures (`worldgen/structure/`), biomes (`worldgen/biome/`) or
preset tags (`tags/worldgen/`). Those families are **not in the artefact** — the brief's
generic list is deliberately not invented here. Its cities, spheres, rail dungeons
and scattered buildings are generated by the Java `worldgen` classes reading the
322 internal JSON files, not by carried data files.

## SDK registrar surface — measured before choosing a family

Measured on `ports/_shared/sdk/0a875da/` (`surface.json` + `api-source/crates/**/*.rs`),
2026-11-24. Counts are `grep -oi <symbol> surface.json | wc -l`:

| Symbol | hits | Verdict |
|---|---:|---|
| `BiomeSource` / `register_biome_source` | 31 / 4 | **exists** — `BiomeSource::possible_biomes`, `noise_biome` (`world/biome_source.rs:17-29`) |
| `StructuresRegistrar` / `register_structure_type` | 6 / 3 | **exists** — `StructureType`, `StructurePiece` (`parts/structures.rs`, `world/structure_type.rs`) |
| `BiomeModifiersRegistrar` | 7 | **exists** — `declare_biome_modifier(path, BiomeModifier)` (`parts/biome_modifiers.rs:28-50`) |
| `BiomeRegionsRegistrar` / `register_chunk_generator` | 7 / 5 | **exist** — `ChunkGenerator::build_terrain` + `vanilla_stages` (`world/chunk_generator.rs`) |
| `register_dimension` (`DimensionSettings`) | 1 | **exists** — `parts/dimensions.rs:36`, `world/dimension.rs:92` with an Overworld-shaped `Default` |
| `register_density_function_type` | 3 | exists (Tectonic precedent) |
| `LevelRef::structures_at` / `StructureAt` | 3 / 14 | **exists** — newly added at this pin (`sdk.json` `changes`) |
| `register_feature` / `configured_feature` / `PlacementModifier` | **0 / 0 / 0** | **absent** — `configured-feature-not-registered` |
| `levels_reset` / `RuntimeLevels::reset_vanilla` | **0 / 0** | **absent** — `blocked(level_reset:R11)` |

This is stronger than the brief anticipated: the dimension, biome source, structure
type and biome modifier registrars **do** exist at this pin. The holes are the
feature registrar, the terrain reset, and the Java generator's own data format.

## Family choice (made after the two measurements above)

Slice 1 carries the verbatim block tags, the language file and the loot tables where
the 26.3 decoder accepts them, and registers the `lostcities:lostcity` dimension from
the jar's `dimension_type` values through `Registrar::register_dimension`. The
in-game assertions are a `ctl` read of the dimension id (only the package produces
it) and a carried block tag, each with a vanilla control and an inverted leg.
Everything else is `blocked(<symbol>)` with the evidence recorded below.

## Gaps / blocked symbols (measured, not guessed)

- **`configured-feature-not-registered`** — the two carried `worldgen/feature/*.json`
  name custom Java feature types `lostcities:lostcity` and `lostcities:spheres`, and
  the two `neoforge/biome_modifier/*.json` add those features to overworld biomes.
  `surface.json` has **0** matches for `register_feature`, `configured_feature` or
  `PlacementModifier`; `mod_content.rs:1892-1904` bundles mod features into
  `worldgen/features.json`, whose decoder uses a fixed vanilla type set, so a
  `lostcities:` type is skipped. The biome modifiers are inert until a feature id
  exists. The feature JSONs are carried for provenance but are **not** claimed as
  working features; the biome modifiers are not packed (adding a feature id the
  game cannot decode is a load-time refusal).
- **`blocked(level_reset:R11)`** — the Java mod's `mixin` classes and the 322
  internal generator JSONs replace vanilla terrain with cities. The pin has 0
  matches for `levels_reset`/`RuntimeLevels::reset_vanilla`, and R11 refuses a
  `[native] level_reset` manifest key (`var/autopilot/PLAN.md`, 2026-11-24). The
  new nightly `f1bb66f353cf…` adds the surface but is not our pin. The registered
  dimension therefore uses vanilla's `minecraft:noise` terrain (faithful to the
  dimension JSON's literal generator) with **no** city generation; recorded as a
  deviation, not flattened into a biome hack.
- **`blocked(no-city-data-decoder)`** — the 322 `data/lostcities/lostcities/*.json`
  files use McJty's private `parts`/`buildings`/`palettes`/`multibuildings`/… schema.
  `surface.json` has no decoder for any of those registry folders (0 matches), and
  `mod_content.rs`'s worldgen bundle list (lines 1892-1904) names only vanilla
  structure/feature/pool/processor kinds. They are not packed in slice 1.
- **`blocked(no-structure-type-for-cities)`** — `StructuresRegistrar` exists, but
  the jar has **no** `worldgen/structure/*.json` for a jigsaw the SDK could start;
  the cities are assembled by Java code. Registering an empty `StructureType` would
  be an invented structure, so none is registered.
- **Not in the artefact:** no item, block, model, recipe, advancement, biome or
  structure files (census above).
- **Deferred:** city generation (a `ChunkGenerator` over the carried part/palette
  data), the config GUI/18 GUI textures, the editor/commands/network classes.

## Upstream defects fixed

1. **The two loot tables' entries say `"type": "item"`** (1.21.1) while 26.3's
   loot decoder matches `"minecraft:item"` in both `expand` and `create`
   (`crates/horizon-world/src/loot.rs:1837`, `:1874` of the pinned staged source).
   Measured in game before the fix: `/loot give @p loot lostcities:chests/lostcitychest`
   answered **`Dropped 0 items`** (`ports/_shared/evidence/the_lost_cities/loot-probe.log`)
   and the inventory stayed empty — the data decoded but every entry was skipped.
   `tools/carry.py::upgrade_loot` rewrites that one field to `"minecraft:item"`;
   after the fix the same command answers **`Dropped 3–4 items`** for both tables
   in the e2e. Everything else in the tables (rolls, weights, `modifier`
   functions with `minecraft:`-qualified types) is already 26.3-shaped. A user
   now gets the chest's contents; the only change is the entry type spelling.

## Coverage table

| Family in the jar | State | Evidence / reason |
|---|---|---|
| `lostcities:lostcity` dimension + its dimension type | **tested** | `register_dimension` from the jar's 18 values (Rust unit test); in-game `execute in lostcities:lostcity run say` → `LC_DIM_PRESENT`, plus a block written/read in that level |
| block tags (6 files) | **tested** | verbatim carry + data contract; `#lostcities:notbreakable` matches bedrock in game, unknown tag id refused |
| chest loot tables (2 files) | **tested** | shape fix + data contract; both tables yield 3–4 items in game, unknown loot id refused |
| language file (`en_us.json`) | carried | verbatim, data contract; no in-game text assertion (the key is a generator name, not surfaced by the pin) |
| custom features (2) + biome modifiers (2) | `blocked(configured-feature-not-registered)` | 0 API matches for a feature/placement registrar; not packed |
| vanilla terrain replacement / city mixins | `blocked(level_reset:R11)` | `surface.json` 0 matches; R11 refuses the manifest key; not packed (recorded deviation) |
| 322 internal generator JSON files | `blocked(no-city-data-decoder)` | no decoder for McJty's private schema in the pin; not packed |
| structure types assembled in Java | `blocked(no-structure-type-for-cities)` | no carried structure JSON for a jigsaw; none invented |
| 5 legacy `.lang` files, 18 GUI textures, Java classes, editor/commands/GUI | not ported (out of tier) | no carried art; no equivalent hook or screen at this pin |
| items/blocks/models/recipes/advancements/biomes | not in artefact | census above |

## Verification (fresh, 2026-11-24)

Pin `0a875da…`, mixins `e59da909…`, package **`the_lost_cities.hmod` 13,214 B,
SHA-256 `6bbe5b775c2a2db1d4078c0bb88ebca50ca8f5aa81bc957736ca9875320dd89d`,
17 entries**. Full table in [`VALIDATION.md`](VALIDATION.md).

- `tools/carry.py --check` was **red before the carry** (9 missing files) and green
  after (9 carried, 2 loot tables shape-upgraded); re-run against the final tree.
- Rust data-contract test was **red first** (`wrong height: left 0 right 256`,
  `invalid dimension height`) and green after the jar values were filled in:
  **2 passed / 0 failed**; `cargo clippy --all-targets -- -D warnings` clean;
  `rustfmt --check` clean.
- `cargo horizon-mod check` (source): *30 API items, Made for 0a875da*. Package
  check: *0.1.0, native-1/0a875da…, 13,214 bytes, sha 6bbe5b77…*.
- `fits --build 0a875da`: **`made for`**. `fits --build 0b14f8a`: **`fits`** —
  the dimension registrar and `DimensionSettings` exist in both cached builds,
  unlike Tectonic's density API; a green second line is the honest result here.
- `verify-sdk-package.py the_lost_cities` → **1/1**;
  `check-e2e-logs.py the_lost_cities-e2e --strict` → `logs=4 ok=4 missing-exit=0 bad-exit=0`;
  `check-skills.py --notes ports/the_lost_cities --strict` → PASS.

### In-game `ctl` transcript (The End + lostcity, seed 20261105, offscreen, creative)

```
vanilla control: execute in lostcities:lostcity run say LC_DIM_PRESENT
   -> {"chat":["Unknown dimension 'lostcities:lostcity'"],"ok":true}
ctl command "execute in lostcities:lostcity run say LC_DIM_PRESENT"
   -> {"chat":["[Player…] LC_DIM_PRESENT"],"ok":true}
ctl command "execute in lostcities:lostcity run forceload add -1 -1 1 1"
ctl command "execute in lostcities:lostcity run setblock 0 70 0 minecraft:stone"
ctl command "execute in lostcities:lostcity run fill 0 70 0 0 70 0 minecraft:air replace minecraft:stone"
   -> {"chat":["Successfully filled 1 block(s)"],"ok":true}
ctl command "execute if block 50 80 50 #lostcities:notbreakable"
   -> {"chat":["Test passed"],"ok":true}
ctl command "execute if block 50 80 50 #lostcities:no_such_tag"
   -> "Unknown block tag 'lostcities:no_such_tag'"
ctl command "loot give @p loot lostcities:chests/lostcitychest"  -> Dropped 3–4 items
ctl command "loot give @p loot lostcities:chests/raildungeonchest" -> Dropped 3–4 items
ctl command "loot give @p loot lostcities:chests/no_such_table"
   -> "Can't find element '…' in registry 'minecraft:loot_table'"
PASS: the lostcity dimension answered, #lostcities:notbreakable matched bedrock,
      and both carried loot tables yielded items
EXIT:0
```

Fail-capable legs: the same leg on the **unmodded control game** ends
`FAIL: the registered dimension did not answer … Unknown dimension` + `EXIT:1`;
`SELFTEST=1` (invert the dimension assertion) ends
`FAIL: SELFTEST: the dimension answered` + `EXIT:1`; `TAG_SELFTEST=1` ends
`FAIL: TAG_SELFTEST: the tag matched` + `EXIT:1`. The pre-fix loot counterfactual
(`Dropped 0 items`) is preserved in `the_lost_cities/loot-probe.log`.
Logs: `ports/_shared/evidence/the_lost_cities-e2e/e2e{,-control,-selftest,-tag-selftest}.log`.

## Next slice

**Family:** a `ChunkGenerator` registered as `lostcities:lostcity` that places a
deterministic city from the carried `parts`/`palettes`/`buildings` data, replacing
the vanilla noise terrain in the dimension registered in slice 1.

**Gap line:** from this document, *"`blocked(level_reset:R11)` — the Java mod's
mixin classes and 322 internal generator JSONs replace vanilla terrain with
cities; the pin has 0 matches for levels_reset/RuntimeLevels::reset_vanilla and
R11 refuses the manifest key"* plus *"`blocked(no-city-data-decoder)` — no decoder
for McJty's private parts/buildings/palettes schema"*.

**Tier fit:** `deep` worldgen. The SDK's `ChunkGenerator::build_terrain(&mut dyn
ChunkAccess)` + `vanilla_stages()` and `Registrar::register_chunk_generator`
**do** exist at this pin (`world/chunk_generator.rs`), so a first faithful city
generator is code, not a data hack; the dimension id is already registered. No
renderer or new vanilla hook is needed. The private JSON schema still needs a
clean-room decoder in `runtime.rs` with red-first tests (start with one `part`
and its `palette`, not all 208 parts). If R11 still refuses a terrain reset,
keep the generator attached only to the mod's own dimension and re-record the
measurement rather than claiming vanilla terrain is Tectonic-style replacement.

**Concrete `ctl` assertion:** after registering the generator, run
`ctl command "execute in lostcities:lostcity run fill 0 60 0 16 90 16 minecraft:air replace minecraft:stone"`
and assert the parsed count is greater than 0 at a seed where the vanilla noise
control yields 0 (the chunk was force-loaded), proving the generator wrote city
blocks the vanilla generator would not. Control: the same `fill` on the unmodded
game finds no lostcity dimension (`Unknown dimension`); `SELFTEST=1` inverts the
count assertion and must exit 1.
