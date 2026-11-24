# The Lost Cities — port notes (slices 1–2)

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

## Slice 2 — the first city chunk generator (0.1.1)

### R11 gate — measured, passes for the mod's own dimension

The brief required proving that registering a chunk generator on the mod's own
dimension does **not** need the R11-refused `[native] level_reset` manifest key.
Measured on the pin (`ports/_shared/sdk/0a875da/`, 2026-11-24):

- `grep -oi 'level_reset\|levels_reset\|reset_vanilla' surface.json` → **0 hits**.
  `RuntimeLevels::reset_vanilla` exists only in nightly `f1bb66f…`, not our pin.
- `register_chunk_generator` is a plain `Registrar` method in the `register`
  hook (`world/chunk_generator.rs:76-85`, `parts/dimensions.rs:47-49`) — it takes
  a `ChunkGenerator`, needs **no manifest key**, and names a generator id that a
  registered dimension's `DimensionSettings::generator` field selects
  (`world/dimension.rs:103-106`). `register_dimension` names the level stem.
- The manifest stays `game`/`horizon`/`mixins` only — no `level_reset` key.

**Ruling:** the gate passes. A mod-owned level gets the generator through the
registrar, not through a vanilla level reset. `blocked(level_reset:R11)` still
holds **only** for replacing vanilla's three levels (the Java mixins' effect),
which this port deliberately does not do.

### The one part decoded — schema as read, with source

Read verbatim from the cached jar
(`ports/_shared/licence-cache/the-lost-cities.jar`, sha `666cd25d…`):

- `data/lostcities/lostcities/parts/street_straight.json` (478 B): `xsize: 16`,
  `zsize: 16`, one `slices` layer of 16 strings of 16 chars. Rows 0–3 and 12–15
  are `b` (street base); rows 4–11 are `S` (street). A space is air.
- `data/lostcities/lostcities/palettes/default.json`: `S` →
  `minecraft:smooth_stone_slab[type=double]` (with a `damaged: minecraft:iron_bars`
  field), `b` → `minecraft:structure_void`. Both entries are embedded verbatim
  as `runtime::STREET_PART_JSON`/`STREET_PALETTE_JSON`.
- `data/lostcities/lostcities/citystyles/citystyle_border.json` names the
  `street`/`streetbase` chars `S`/`b`, confirming the role of each char.

`runtime.rs` contains a clean-room JSON decoder (no crate dependency, no Java
translation) with validated rules: a row's char count must equal `xsize`, a
slice's row count must equal `zsize`, every non-space char must resolve in the
palette, `blocks` arrays use deterministic weighted picks, and `id[prop=val]`
splits into state properties. `lib.rs` requests the two block keys once,
registers `lostcities:lostcity` as a `ChunkGenerator`, and `build_terrain`
writes the 16×16 part at y=90 in every chunk of that level with no vanilla
stages. The dimension switches to the pin-required `Fixed` biome source: the
jar's multi-noise preset is not expressible for a mod-owned generator
(`world/dimension.rs:358-360`, recorded as a deviation).

**Red verbatim** (`evidence/the_lost_cities/decoder-test-red.log`, before the
decoder existed): eleven `error[E0425]/[E0433]` — *"cannot find value
`STREET_PART_JSON` in this scope"*, *"cannot find function `parse_json` in this
scope"*, *"cannot find type `Part` in this scope"*, *"cannot find function
`decode_city` in this scope"*, *"cannot find function `split_block` in this
scope"* — `error: could not compile ... due to 11 previous errors`. The
part-size/bounds/palette rules then failed as assertions before the glue:
the decoder's first green run also caught a wrong test expectation (256 cells,
not 128 — the `b` border is a block too), fixed to assert 128 slab + 128 void.

### Both fits lines on the delivered 0.1.1 sha

```
The Lost Cities:
0a875da  made for
The Lost Cities:
0b14f8a  fits
```
Delivered `the_lost_cities.hmod` **22,266 B**, SHA-256
`ae8b3ab10fb9992b7a54d738e93fd79a356bf57508b1339701171ed367929777`, 17 entries.

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
- **`blocked(level_reset:R11)`** — the Java mod's `mixin` classes replace
  **vanilla's three levels** with cities. The pin has 0 matches for
  `levels_reset`/`RuntimeLevels::reset_vanilla`, and R11 refuses a
  `[native] level_reset` manifest key (`var/autopilot/PLAN.md`, 2026-11-24).
  Slice 2's measured gate above proves this does not block a chunk generator on
  the mod's **own** registered level; it still blocks vanilla-level replacement,
  which this port does not attempt. The nightly `f1bb66f353cf…` adds the surface
  but is not our pin.
- **`blocked(no-city-data-decoder)`** — of the 322
  `data/lostcities/lostcities/*.json` files (private
  `parts`/`buildings`/`palettes`/`multibuildings`/… schema; the pin's
  `mod_content.rs` worldgen bundle names only vanilla kinds), slice 2 writes a
  clean-room decoder and decodes **one part** (`street_straight`) + two palette
  entries. The **other 207 parts**, 38 buildings, 31 palettes, 15 multibuildings,
  13 variants, 6 scattered and all `citystyles`/`styles`/`worldstyles` are still
  blocked — no decoder was written for them.
- **`blocked(no-structure-type-for-cities)`** — `StructuresRegistrar` exists, but
  the jar has **no** `worldgen/structure/*.json` for a jigsaw the SDK could start;
  the cities are assembled by Java code. Registering an empty `StructureType` would
  be an invented structure, so none is registered.
- **Not in the artefact:** no item, block, model, recipe, advancement, biome or
  structure files (census above).
- **Deferred:** the remaining 207 parts, spheres/rail dungeons, the config
  GUI/18 GUI textures, the editor/commands/network classes.

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
| city chunk generator (part `street_straight` + 2 palette entries) | **tested** | clean-room decoder + 5 unit tests; `register_chunk_generator`; fill-replace of the palette's slab in the generated level → **136 blocks**, vanilla-noise Overworld control → **0**, `CITY_SELFTEST=1` inverts → `EXIT:1` |
| block tags (6 files) | **tested** | verbatim carry + data contract; `#lostcities:notbreakable` matches bedrock in game, unknown tag id refused |
| chest loot tables (2 files) | **tested** | shape fix + data contract; both tables yield 3–4 items in game, unknown loot id refused |
| language file (`en_us.json`) | carried | verbatim, data contract; no in-game text assertion (the key is a generator name, not surfaced by the pin) |
| custom features (2) + biome modifiers (2) | `blocked(configured-feature-not-registered)` | 0 API matches for a feature/placement registrar; not packed |
| vanilla terrain replacement / city mixins | `blocked(level_reset:R11)` | `surface.json` 0 matches; R11 refuses the manifest key; only the mod's own level is generated, not vanilla's |
| 322 internal generator JSON files | 1 part decoded / 207 `blocked(no-city-data-decoder)` | clean-room decoder for `street_straight` + `S`/`b` palette entries; the other 207 parts and all buildings/palettes/multibuildings have no decoder; not packed |
| structure types assembled in Java | `blocked(no-structure-type-for-cities)` | no carried structure JSON for a jigsaw; none invented |
| 5 legacy `.lang` files, 18 GUI textures, Java classes, editor/commands/GUI | not ported (out of tier) | no carried art; no equivalent hook or screen at this pin |
| items/blocks/models/recipes/advancements/biomes | not in artefact | census above |

## Verification (fresh, 2026-11-24, version 0.1.1)

Pin `0a875da…`, mixins `e59da909…`, package **`the_lost_cities.hmod` 22,266 B,
SHA-256 `ae8b3ab10fb9992b7a54d738e93fd79a356bf57508b1339701171ed367929777`,
17 entries**. Full table in [`VALIDATION.md`](VALIDATION.md).

- `tools/carry.py --check` green against the final tree (9 carried, 2 loot tables
  shape-upgraded); `check-recipe-types.py`/`check-manifest-deps.py` clean (no
  recipe type; only vanilla block ids named in code).
- Slice 1's two dimension-contract tests were red first; slice 2's decoder tests
  were red first (11 unresolved symbols above, then 256-vs-128 expectation).
  Final suite: **8 passed / 0 failed**; `cargo clippy --all-targets -- -D warnings`
  clean (0 warnings); `rustfmt --check` clean. `sdk-test.log` regenerated for
  0.1.1 **after** the final build; the 0.1.0 logs retired to `retired-0.1.0/`.
- `cargo horizon-mod check` (source): *131 API items, Made for 0a875da*. Package
  check: *0.1.1, native-1/0a875da…, 22,266 bytes, sha ae8b3ab1…*.
- `fits --build 0a875da`: **`made for`**. `fits --build 0b14f8a`: **`fits`** —
  the chunk-generator registrar and `DimensionSettings` exist in both cached
  builds (only the new Rust decoder is ours), so the second green line is honest.
- Launcher native build: **`Compiling the_lost_cities (1 of 1)`**, tag
  `native-0a875da-cae68fb44f23`; `native-mods.json` names **0.1.1** (checked before
  the legs, so a concurrent build's prune cannot make the legs test a stale mod).
- `verify-sdk-package.py the_lost_cities` → **1/1** (incl. "package matches the
  tree" — README/manifest edited before the final build);
  `check-e2e-logs.py the_lost_cities-e2e --strict` →
  `logs=5 ok=5 missing-exit=0 bad-exit=0`;
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
ctl command "execute in lostcities:lostcity run fill 0 60 0 16 90 16 minecraft:air replace minecraft:smooth_stone_slab"
   -> {"chat":["Successfully filled 136 block(s)"],"ok":true}   # generator-authored
ctl command "fill 0 60 0 16 90 16 minecraft:air replace minecraft:smooth_stone_slab"  # Overworld console
   -> {"chat":["No blocks were filled"],"ok":true}              # vanilla-noise control = 0
ctl command "execute if block 50 80 50 #lostcities:notbreakable"
   -> {"chat":["Test passed"],"ok":true}
ctl command "execute if block 50 80 50 #lostcities:no_such_tag"
   -> "Unknown block tag 'lostcities:no_such_tag'"
ctl command "loot give @p loot lostcities:chests/lostcitychest"  -> Dropped 3–4 items
ctl command "loot give @p loot lostcities:chests/raildungeonchest" -> Dropped 3–4 items
ctl command "loot give @p loot lostcities:chests/no_such_table"
   -> "Can't find element '…' in registry 'minecraft:loot_table'"
PASS: the lostcity dimension answered, #lostcities:notbreakable matched bedrock,
      both carried loot tables yielded items, and the generator authored 136 slabs
      where vanilla noise authored 0
EXIT:0
```

Fail-capable legs: the same leg on the **unmodded control game** ends
`FAIL: the registered dimension did not answer … Unknown dimension` + `EXIT:1`;
`SELFTEST=1` (invert the dimension assertion) ends
`FAIL: SELFTEST: the dimension answered` + `EXIT:1`; `TAG_SELFTEST=1` ends
`FAIL: TAG_SELFTEST: the tag matched` + `EXIT:1`; `CITY_SELFTEST=1` ends
`FAIL: CITY_SELFTEST: the generator authored 136 slab(s); the inverted assertion
expected 0` + `EXIT:1`. The pre-fix loot counterfactual (`Dropped 0 items`) is
preserved in `the_lost_cities/loot-probe.log`.
Logs: `ports/_shared/evidence/the_lost_cities-e2e/e2e{,-control,-selftest,-tag-selftest,-city-selftest}.log`.

## Next slice

**Family:** decode a **second, multi-slice building part** —
`parts/building1_1.json` (16×16, **6 slices**, chars `#`/`a`/`@`/`F`/`D`/`L`/`l`/`1`/`*`)
— and place it at a second Y alongside the street, using palettes
`bricks_standard` + `common`.

**Gap line:** from the coverage table above, *"1 part decoded / 207
`blocked(no-city-data-decoder)` … the other 207 parts and all buildings/palettes/
multibuildings have no decoder"*. The generic decoder already written handles
`slices`, weighted `blocks` and `id[prop=val]`; this part additionally needs the
`{"variant": "stonebrick"}` entry form (`bricks_standard.json` char `#`), which
resolves through the jar's `styles`/variant files and is not yet handled.

**Tier fit:** `deep` worldgen; the registrar and `ChunkAccess` surface is already
proven by slice 2, so this is pure decoder extension (variant resolution) plus
one more generator placement — no new API, renderer or manifest key. It grows the
same clean-room decoder with red-first tests for the variant entry and multi-layer
bounds, then a second `build_terrain` placement at a different Y.

**Concrete `ctl` assertion:** after placing the building part, run
`ctl command "execute in lostcities:lostcity run fill 0 70 0 16 84 16 minecraft:air replace minecraft:bookshelf"`
(the `L` char of `palettes/common.json` resolves to `minecraft:bookshelf`, which
the street part never writes) and assert the parsed count `> 0`; the same fill on
the vanilla-noise Overworld must answer `No blocks were filled` (count 0);
`BUILDING_SELFTEST=1` inverts the count and must end `EXIT:1`. The four existing
legs plus `CITY_SELFTEST` stay green.
