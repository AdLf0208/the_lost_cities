# The Lost Cities — port notes (slices 1–7)

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

## Slice 3 — the second, six-layer building part (0.1.2)

### The part and its variant/palette as read from the jar (source first)

Everything below was read from the cached jar
(`ports/_shared/licence-cache/the-lost-cities.jar`, sha `666cd25d…`) before any
code was written; the brief's illustrative char list was not trusted:

- `data/lostcities/lostcities/parts/building1_1.json`: `xsize: 16`, `zsize: 16`,
  **6 slices**. The exact non-space char set is `#` `a` `@` `F` `D` `L` `l` `1`
  `*` `T` (the brief omitted `T` and the space cells; the artefact wins). Slice 0
  and slices 4–5 are all `#`; slices 1–3 are the walls (`a`/`@`/`#`), the `L`
  bookshelf, `l` ladder, `F` weighted furniture, `D` dirt, `1` spawner, `*` potted
  cactus and `T` wall torch. Embedded verbatim as `runtime::BUILDING_PART_JSON`.
- `data/lostcities/lostcities/palettes/bricks_standard.json`: char `#` is
  **`{"variant": "stonebrick"}`**, not a `block`. Its
  `data/lostcities/lostcities/variants/stonebrick.json` is a weighted list
  (`cracked_stone_bricks` 9, `mossy_stone_bricks` 8, `stone_bricks` 1000);
  embedded verbatim as `runtime::STONEBRICK_VARIANT_JSON`.
- `data/lostcities/lostcities/palettes/common.json` supplies `1`/`T`/`F`/`D`/
  `l`/`L`/`*`/space verbatim — in particular `L` → `minecraft:bookshelf`, the
  char the in-game assertion replaces.
- `a` and `@` are **not** in those two palettes. `styles/standard.json`
  selects `common`, `default`, one `bricks_*` and one `glass_full*`/`glass_pane*`
  and one `glass_side_variant_*` palette *together*, so the port carries the
  exact `a` → `minecraft:glass` entry from `palettes/glass_full.json` and the
  exact `@` → `{"frompalette": "#"}` entry from
  `palettes/glass_side_variant_bricks.json` into the merged
  `runtime::BUILDING_PALETTE_JSON` (every entry byte-copied from its jar source,
  comment citing each). No block id or char was invented.

The decoder grew two `PaletteEntry` forms — `Variant(name)` resolved through the
variant table, and `FromPalette(ch)` followed with a cycle bound — plus a
`VariantTable::from_files` reader and six-layer bound checks in `Part::from_json`.
`lib.rs` lays the street at y=90 and the building at y=72 (`BUILDING_BASE_Y`),
requesting the union of block keys from both parts once. No new SDK API or
manifest key: the registrar and `ChunkAccess` surface is slice 2's.

**Red verbatim** (`evidence/the_lost_cities/building-test-red.log`, before the
variant/part data existed): 11 `error[E0425]/[E0433]` — *"cannot find value
`STONEBRICK_VARIANT_JSON` in this scope"*, *"cannot find value `BUILDING_PART_JSON`
in this scope"*, *"cannot find value `BUILDING_PALETTE_JSON` in this scope"*,
*"cannot find type `VariantTable` in this scope"*, *"cannot find function
`decode_city_with_variants` in this scope"* (plus the one-cell test fixtures) —
`error: could not compile ... due to 11 previous errors`.

### Both fits lines on the delivered 0.1.2 sha

```
The Lost Cities:
0a875da  made for
The Lost Cities:
0b14f8a  fits
```
Delivered `the_lost_cities.hmod` **25,576 B**, SHA-256
`9e3220960cc2fd173026339ccf79048f3e09a5dbe202c8119d5c44eefa6057db`, 17 entries.
The second green line is honest: the registrar/`ChunkAccess` API exists in both
cached builds; only the clean-room decoder and its embedded jar data are ours.

## Slice 4 — the third, six-layer horizontal rail part (0.1.3)

### The part and its palettes as read from the jar (source first)

Everything below was read from the cached jar
(`ports/_shared/licence-cache/the-lost-cities.jar`, sha `666cd25d…`) before any
code was written. **The brief's char list was wrong, so the artefact won:**

- `data/lostcities/lostcities/parts/rails_horizontal.json`: `xsize: 16`,
  `zsize: 16`, `refpalette: "rails"`, **6 slices**. The exact non-space char set
  is `T` `e` `f` `g` `y` `~` — the same six the brief named — and every row is
  16 chars. Slice 1's three rail rows are `eeeeeeefeeeeeeee` (7 `e`, one `f`,
  8 `e`), so the part writes 45 east-west rails and 3 powered rails. Embedded
  verbatim as `runtime::RAILS_PART_JSON`.
- `data/lostcities/lostcities/palettes/rails.json`: the brief's example ids were
  **not** what the jar says. The artefact declares `k` →
  `minecraft:rail[shape=north_south]`, **`e` → `minecraft:rail[shape=east_west]`**,
  `f` → `minecraft:powered_rail[powered=true,shape=east_west]`, plus `j`/`q`/`s`/
  `r`/`n`/`m`/`E`/`H`/`i` for the other rail shapes. All twelve entries are
  carried verbatim into `runtime::RAILS_PALETTE_JSON` (the part's `refpalette`).
- `y` is the `{"variant": "stonebrick"}` entry of `palettes/default.json`
  (resolved through the same jar `variants/stonebrick.json` as slice 3); `T`
  (`wall_torch[facing=north]`), `g` (`redstone_torch[lit=true]`), `~`
  (`structure_void`) and space (`minecraft:air`) are copied verbatim from
  `palettes/common.json`. No id or char was invented.

The decoder needed no new form — `variant`, `frompalette`, weighted `blocks`
and six-layer bounds already exist. `lib.rs` lays the part at y=54
(`RAILS_BASE_Y`) alongside the street (90) and building (72) and requests the
union of all three parts' block keys once. No new SDK API or manifest key.

**Red verbatim** (`evidence/the_lost_cities/rails-test-red.log`, before the rail
data existed): `error[E0425]: cannot find value 'RAILS_PART_JSON' in this scope`
(twice) and `cannot find value 'RAILS_PALETTE_JSON' in this scope` —
`error: could not compile ... due to 3 previous errors`, build-slot exit 101.
The new tests then passed green: **13 passed / 0 failed**, including
`rails_tests::rail_chars_resolve_to_the_jar_rail_ids` which asserts the artefact's
`e`/`f` ids and the 45/3 counts.

### Both fits lines on the delivered 0.1.3 sha

```
The Lost Cities:
0a875da  made for
The Lost Cities:
0b14f8a  fits
```
Delivered `the_lost_cities.hmod` **27,133 B**, SHA-256
`857e338962757c3ecc4b209d90c5b86e3819a7a0c62b783c7465befa6daf9e63`, 17 entries.
The second line is honest: the registrar/`ChunkAccess` API exists in both
cached builds; only the clean-room decoder and its embedded jar data are ours.

## Slice 5 — the fourth, six-layer rail bend (0.1.4)

### The part as read from the cached jar (source first)

Read verbatim from `ports/_shared/licence-cache/the-lost-cities.jar` (sha
`666cd25d…`) before any edit: `data/lostcities/lostcities/parts/rails_bend.json`,
**2,643 B**, sha256 `9969f02e8c9b566de15616eecb5598dbea4a4ff5528c5b8792fd4143497e3232`.
It is `xsize: 16`, `zsize: 16`, `refpalette: "rails"`, **six slices**, each a
16×16 layer with 134 non-space cells. Its exact char census is `k` 9, `m` 2,
`r` 1, `s` 1, `e` 14, `f` 2, `g` 2, `y` 488, `~` 285.

**Correction record:** for the first time the brief's example ids matched the
artefact — `palettes/rails.json` really says `k` → `minecraft:rail[shape=north_south]`,
`r` → `rail[shape=north_west]`, `s` → `rail[shape=south_west]`, `m` →
`powered_rail[powered=true,shape=north_south]`, beside `e` → `rail[shape=east_west]`
and `f` → `powered_rail[powered=true,shape=east_west]`. They were still checked
against the jar before the carry; the whole 2,643-byte part is embedded
byte-identically in `runtime::RAILS_BEND_PART_JSON` (verified by sha256), and the
part reuses slice 4's verbatim `RAILS_PALETTE_JSON` and the same jar
`variants/stonebrick.json`. No id or char was invented.

No new decoder form, SDK API or manifest key: `variant`, `frompalette`, weighted
`blocks` and six-layer bounds already exist. `lib.rs` lays the bend at y=36
(`BEND_BASE_Y`) alongside street (90), building (72) and horizontal line (54),
and unions its block ids into the same one-time `block_key` request.

### Red verbatim

`evidence/the_lost_cities/bend-test-red.log`, before the part existed: two
`error[E0425]: cannot find value 'RAILS_BEND_PART_JSON' in this scope` (at the
six-layer shape assertion and at the rail-shape resolver), with rustc's
*similarly named constant `RAILS_PART_JSON`* hint —
`error: could not compile 'the_lost_cities' (lib test) due to 2 previous errors`,
build-slot exit **101**. The new tests then passed green: **15 passed / 0
failed**, including `bend_tests::bend_chars_resolve_to_the_jar_rail_shapes`,
which pins the artefact's 9 `k` / 2 `m` / 1 `r` / 1 `s` / 14 `e` / 2 `f` counts
and asserts the horizontal line writes **zero** north_south rails.

### Both fits lines on the delivered 0.1.4 sha

```
The Lost Cities:
0a875da  made for
The Lost Cities:
0b14f8a  fits
```

## Slice 6 — the fifth, six-layer three-way rail split (0.1.5)

### The part as read from the cached jar (source first)

Read verbatim from `ports/_shared/licence-cache/the-lost-cities.jar` (sha
`666cd25d…`) before any edit: `data/lostcities/lostcities/parts/rails_3split.json`,
**2,643 B**, sha256 `b0c8bd61f1f3b3641b9bf20a57087429123b7de9353dd1fd3129e40717777eb0`.
It is `xsize: 16`, `zsize: 16`, `refpalette: "rails"`, six slices. Its exact
non-space char census is `y` 606, `~` 415, `e` 26, `k` 9, `g` 8, `f` 4, `T` 2,
`m` 2, `n` 1, `q` 1 (1,074 cells). The palette ids were checked against the
artefact, not the brief: `palettes/rails.json` really says `n` →
`minecraft:rail[shape=north_east]` and `q` → `minecraft:rail[shape=south_east]`.
Neither shape is written by the street, building, horizontal line or bend; the
`shape(8,5)`/`shape(8,9)` cells of slice 1 carry them. The whole 2,643-byte part
is embedded byte-identically in `runtime::RAILS_3SPLIT_PART_JSON` (verified by
sha256 after rustfmt) and reuses slice 4's verbatim `RAILS_PALETTE_JSON` plus the
same jar `variants/stonebrick.json`. No id or char was invented.

No new decoder form, SDK API or manifest key: `variant`, `frompalette`, weighted
`blocks` and six-layer bounds already exist. `lib.rs` lays the split at y=18
(`SPLIT_BASE_Y`) alongside street (90), building (72), line (54) and bend (36),
and unions its block ids into the same one-time `block_key` request.

### Red verbatim

`evidence/the_lost_cities/split-test-red.log`, before the part existed: two
`error[E0425]: cannot find value 'RAILS_3SPLIT_PART_JSON' in this scope` (at the
six-layer shape assertion and at the rail-shape resolver), with rustc's
*similarly named constant `RAILS_BEND_PART_JSON`* hint —
`error: could not compile 'the_lost_cities' (lib test) due to 2 previous errors`,
build-slot exit **101**. The new tests then passed green: **17 passed / 0
failed**, including `split_tests::split_chars_resolve_to_the_jar_rail_shapes`,
which pins the artefact's 1 `n` / 1 `q` / 9 `k` / 2 `m` / 26 `e` / 4 `f` / 0 `r` /
0 `s` counts and asserts none of the first four parts writes either east-facing
shape.

### Both fits lines on the delivered 0.1.5 sha

```
The Lost Cities:
0a875da  made for
The Lost Cities:
0b14f8a  fits
```

## Slice 7 — the sixth, eighteen-layer rail descent (0.1.6)

### The part as read from the cached jar (source first)

Read verbatim from `ports/_shared/licence-cache/the-lost-cities.jar` (sha
`666cd25d…`, verified before any edit):
`data/lostcities/lostcities/parts/rails_down2.json`, **7,779 B**, sha256
`f2f6ebabe2ae71dc6c094496de0b90ebda97415ecc619b7c7b8c78161e65be89`. It is
`xsize: 16`, `zsize: 16`, `refpalette: "rails"`, **18 slices** (not the six of
the earlier parts), every row 16 chars. Its exact non-space char census is
`y` 504, `~` 420, `j` 21, `i` 15, `f` 12, `g` 6 (978 cells). The palette ids were
checked against the **jar's** `palettes/rails.json`, not the brief: it says
`j` → `minecraft:rail[shape=ascending_east]` and
`i` → `minecraft:powered_rail[powered=true,shape=ascending_east]`, two shapes
none of the first five decoded parts writes. The whole 7,779-byte part is
embedded byte-identically in `runtime::RAILS_DOWN2_PART_JSON` (verified by
sha256 after rustfmt) and reuses slice 4's verbatim `RAILS_PALETTE_JSON` plus
the same jar `variants/stonebrick.json`. No id or char was invented. The
decoder's layer loop is generic (`Part::from_json` checks every slice against
`zsize`/`xsize`), so 18 layers needed no new decoder form, SDK API or manifest
key.

`lib.rs` lays the descent at y=4 (`DOWN2_BASE_Y`), spanning y=4..21 alongside
street (90), building (72), line (54), bend (36) and split (18), and unions its
block ids into the same one-time `block_key` request.

### Red verbatim

`evidence/the_lost_cities/down2-test-red.log`, before the part existed: two
`error[E0425]: cannot find value 'RAILS_DOWN2_PART_JSON' in this scope` (at the
18-layer shape assertion and at the rail-shape resolver), with rustc's
*similarly named constant `RAILS_BEND_PART_JSON`* hint —
`error: could not compile 'the_lost_cities' (lib test) due to 2 previous errors`,
build-slot exit **101**. The new tests then passed green: **19 passed / 0
failed**, including `down2_tests::down2_chars_resolve_to_the_jar_ascending_rail_shapes`,
which pins the artefact's 21 `j` / 15 `i` / 12 `f` counts and asserts none of
the first five parts writes either ascending_east shape.

### Both fits lines on the delivered 0.1.6 sha

```
The Lost Cities:
0a875da  made for
The Lost Cities:
0b14f8a  fits
```

### Harness defect fixed (not a port defect)

The first 0.1.6 default e2e leg failed at the **pre-existing** loot assertion
with `Dropped 1 [Diamond Sword]`: 26.3's `loot give` names the dropped item
instead of saying `items` when the random table yields exactly one, while the
regex only matched `Dropped N items`. The regex now accepts both
`Dropped N items` and `Dropped N [Item...]` (still refusing `Dropped 0 items`),
so the assertion remains fail-capable. No package file changed; the leg was
re-run green (4 and 3 items).

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
  `mod_content.rs` worldgen bundle names only vanilla kinds), the port writes a
  clean-room decoder and decodes **six parts** (`street_straight`,
  `building1_1`, `rails_horizontal`, `rails_bend`, `rails_3split`, `rails_down2`) plus their palette/variant
  entries. The **other 202 parts**, 38 buildings,
  31 palettes, 15 multibuildings, 12 variants, 6 scattered and all
  `citystyles`/`styles`/`worldstyles` are still blocked — no decoder was written
  for them.
- **`blocked(no-structure-type-for-cities)`** — `StructuresRegistrar` exists, but
  the jar has **no** `worldgen/structure/*.json` for a jigsaw the SDK could start;
  the cities are assembled by Java code. Registering an empty `StructureType` would
  be an invented structure, so none is registered.
- **Not in the artefact:** no item, block, model, recipe, advancement, biome or
  structure files (census above).
- **Deferred:** the remaining 202 parts, spheres/rail dungeons, the config
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
| 322 internal generator JSON files | 6 parts decoded / 202 `blocked(no-city-data-decoder)` | clean-room decoder for `street_straight`, the six-layer `building1_1`, the six-layer `rails_horizontal`, the six-layer `rails_bend`, the six-layer `rails_3split` and the eighteen-layer `rails_down2`, with `variant`/`frompalette`/weighted/multi-layer support; the other 202 parts and all buildings/palettes/multibuildings have no decoder; not packed |
| horizontal rail line (`parts/rails_horizontal.json` + `palettes/rails.json`) | **tested** | 45 east-west rails + 3 powered rails decoded from the artefact's ids; laid at y=54; in-game fill-replace of `minecraft:rail` at y=50..64 → **48 blocks**, vanilla-noise Overworld → **0**, `RAILS_SELFTEST=1` → `EXIT:1` |
| rail bend (`parts/rails_bend.json` + `palettes/rails.json`) | **tested** | 9 north_south + 2 powered north_south + 1 north_west + 1 south_west + 14 east_west + 2 powered east_west rails decoded from the artefact's ids; laid at y=36; in-game fill-replace of `minecraft:rail[shape=north_south]` at y=30..44 → **10 blocks** (9 in-chunk + 1 from the inclusive 17×17 window), vanilla-noise Overworld → **0**, `BEND_SELFTEST=1` → `EXIT:1` |
| three-way rail split (`parts/rails_3split.json` + `palettes/rails.json`) | **tested** | 1 north_east + 1 south_east + 9 north_south + 2 powered north_south + 26 east_west + 4 powered east_west rails decoded from the artefact's ids (byte-identical part, sha `b0c8bd61…`); laid at y=18; in-game fill-replace of `minecraft:rail[shape=north_east]` at y=12..26 → **1 block**, vanilla-noise Overworld → **0**, `SPLIT_SELFTEST=1` → `EXIT:1` |
| eighteen-layer rail descent (`parts/rails_down2.json` + `palettes/rails.json`) | **tested** | 21 `j` ascending_east + 15 `i` powered ascending_east + 12 `f` powered east_west rails (7,779-byte part, sha `f2f6eba…`, byte-identical); laid at y=4..21; in-game fill-replace of `minecraft:rail[shape=ascending_east]` at y=0..24 → **21 blocks**, vanilla-noise Overworld → **0**, `DOWN2_SELFTEST=1` → `EXIT:1` |
| structure types assembled in Java | `blocked(no-structure-type-for-cities)` | no carried structure JSON for a jigsaw; none invented |
| 5 legacy `.lang` files, 18 GUI textures, Java classes, editor/commands/GUI | not ported (out of tier) | no carried art; no equivalent hook or screen at this pin |
| items/blocks/models/recipes/advancements/biomes | not in artefact | census above |

## Verification (fresh, 2026-11-24, version 0.1.6)

Pin `0a875da…`, mixins `e59da909…`, package **`the_lost_cities.hmod` 31,914 B,
SHA-256 `21b6f29c68148e6834cdc6ba01b5f1a329f83003f67171371ec9948b466ac6c9`,
17 entries**. Full table in [`VALIDATION.md`](VALIDATION.md).

- `tools/carry.py --check` green against the final tree (9 carried, 2 loot tables
  shape-upgraded); `check-recipe-types.py`/`check-manifest-deps.py` clean (no
  recipe type; only vanilla block ids named in code).
- Slices 1–7 were each red first: the dimension-contract tests, slice 2's 11
  unresolved decoder symbols, slice 3's 11 unresolved variant/building symbols,
  slice 4's 3 unresolved rail symbols, slice 5's 2 unresolved
  `RAILS_BEND_PART_JSON`, slice 6's 2 unresolved `RAILS_3SPLIT_PART_JSON` and
  slice 7's 2 unresolved `RAILS_DOWN2_PART_JSON` symbols above. Final suite:
  **19 passed / 0 failed**; `cargo clippy --all-targets -- -D warnings` clean
  (0 warnings); `rustfmt --check` clean. `sdk-test.log` regenerated for 0.1.6
  **after** the final build; earlier logs retired to `retired-0.1.0/` and
  `retired-0.1.2/`.
- `cargo horizon-mod check` on the package: *0.1.6,
  native-1/0a875da…, 31,914 bytes, sha 21b6f29c…*. No new API item or SDK
  surface was needed.
- `fits --build 0a875da`: **`made for`**. `fits --build 0b14f8a`: **`fits`** —
  the chunk-generator registrar and `DimensionSettings` exist in both cached
  builds (only the Rust decoder and embedded jar data are ours), so the second
  green line is honest. No local type or variant named `Block`/`Item` was
  introduced (the false-fail trap).
- Launcher native build: **`Compiling the_lost_cities (1 of 1)`**, tag
  `native-0a875da-0c2873eab4cb`; `native-mods.json` names **0.1.6** (checked
  before the legs, so a concurrent build's prune cannot make the legs test a
  stale mod). The previous the_lost_cities `native-*` tag (0.1.5,
  `native-0a875da-ff899e672547`) was removed before the build; no exit 10 from a
  concurrent native build occurred.
- `verify-sdk-package.py the_lost_cities` → **1/1** (incl. "package matches the
  tree" — README/manifest/Cargo/src edited before the final build);
  `check-e2e-logs.py the_lost_cities-e2e --strict` →
  `logs=10 ok=10 missing-exit=0 bad-exit=0`;
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
ctl command "execute in lostcities:lostcity run fill 0 70 0 16 84 16 minecraft:air replace minecraft:bookshelf"
   -> {"chat":["Successfully filled 4 block(s)"],"ok":true}     # six-layer building's L char
ctl command "fill 0 70 0 16 84 16 minecraft:air replace minecraft:bookshelf"  # Overworld console
   -> {"chat":["No blocks were filled"],"ok":true}              # vanilla-noise control = 0
ctl command "execute in lostcities:lostcity run fill 0 50 0 16 64 16 minecraft:air replace minecraft:rail"
   -> {"chat":["Successfully filled 48 block(s)"],"ok":true}    # rail line's e char at y=54..59
ctl command "fill 0 50 0 16 64 16 minecraft:air replace minecraft:rail"  # Overworld console
   -> {"chat":["No blocks were filled"],"ok":true}              # vanilla-noise control = 0
ctl command "execute in lostcities:lostcity run fill 0 30 0 16 44 16 minecraft:air replace minecraft:rail[shape=north_south]"
   -> {"chat":["Successfully filled 10 block(s)"],"ok":true}    # bend's k char at y=36..41
ctl command "fill 0 30 0 16 44 16 minecraft:air replace minecraft:rail[shape=north_south]"  # Overworld console
   -> {"chat":["No blocks were filled"],"ok":true}              # vanilla-noise control = 0
ctl command "execute in lostcities:lostcity run fill 0 12 0 16 26 16 minecraft:air replace minecraft:rail[shape=north_east]"
   -> {"chat":["Successfully filled 1 block(s)"],"ok":true}     # split's n char at y=18..23
ctl command "fill 0 12 0 16 26 16 minecraft:air replace minecraft:rail[shape=north_east]"  # Overworld console
   -> {"chat":["No blocks were filled"],"ok":true}              # vanilla-noise control = 0
ctl command "execute in lostcities:lostcity run fill 0 0 0 16 24 16 minecraft:air replace minecraft:rail[shape=ascending_east]"
   -> {"chat":["Successfully filled 21 block(s)"],"ok":true}    # descent's j char at y=4..21
ctl command "fill 0 0 0 16 24 16 minecraft:air replace minecraft:rail[shape=ascending_east]"  # Overworld console
   -> {"chat":["No blocks were filled"],"ok":true}              # vanilla-noise control = 0
ctl command "execute if block 50 80 50 #lostcities:notbreakable"
   -> {"chat":["Test passed"],"ok":true}
ctl command "execute if block 50 80 50 #lostcities:no_such_tag"
   -> "Unknown block tag 'lostcities:no_such_tag'"
ctl command "loot give @p loot lostcities:chests/lostcitychest"  -> Dropped 4 items
ctl command "loot give @p loot lostcities:chests/raildungeonchest" -> Dropped 3 items
ctl command "loot give @p loot lostcities:chests/no_such_table"
   -> "Can't find element '…' in registry 'minecraft:loot_table'"
PASS: the lostcity dimension answered, all six decoded parts authored their own blocks
      in it (136 slabs at y=90, 4 bookshelves at y=72, 48 east_west rails at y=54..59,
      10 north_south rails at y=36..41, 1 north_east rail at y=18..23 and 21
      ascending_east rails at y=4..21 — each 0 on vanilla noise),
      #lostcities:notbreakable matched bedrock, and both carried loot tables yielded items
EXIT:0
```

Fail-capable legs: the same leg on the **unmodded control game** ends
`FAIL: the registered dimension did not answer … Unknown dimension` + `EXIT:1`;
`SELFTEST=1` (invert the dimension assertion) ends
`FAIL: SELFTEST: the dimension answered` + `EXIT:1`; `TAG_SELFTEST=1` ends
`FAIL: TAG_SELFTEST: the tag matched` + `EXIT:1`; `CITY_SELFTEST=1` ends
`FAIL: CITY_SELFTEST: the generator authored 136 slab(s); the inverted assertion
expected 0` + `EXIT:1`; `BUILDING_SELFTEST=1` ends
`FAIL: BUILDING_SELFTEST: the building authored 4 bookshelf(s); the inverted
assertion expected 0` + `EXIT:1`; `RAILS_SELFTEST=1` ends
`FAIL: RAILS_SELFTEST: the rail part authored 48 rail(s); the inverted assertion
expected 0` + `EXIT:1`; `BEND_SELFTEST=1` ends
`FAIL: BEND_SELFTEST: the bend authored 10 north_south rail(s); the inverted
assertion expected 0` + `EXIT:1`; `SPLIT_SELFTEST=1` ends
`FAIL: SPLIT_SELFTEST: the split authored 1 north_east rail(s); the inverted
assertion expected 0` + `EXIT:1`; `DOWN2_SELFTEST=1` ends
`FAIL: DOWN2_SELFTEST: the descent authored 21 ascending_east rail(s); the
inverted assertion expected 0` + `EXIT:1`. The pre-fix loot counterfactual
(`Dropped 0 items`) is preserved in `the_lost_cities/loot-probe.log`.
Logs: `ports/_shared/evidence/the_lost_cities-e2e/e2e{,-control,-selftest,-tag-selftest,-city-selftest,-building-selftest,-rails-selftest,-bend-selftest,-split-selftest,-down2-selftest}.log`;
the 0.1.2 logs are retired to `retired-0.1.2/`.

## Next slice

**Family:** decode the first rail descent `parts/rails_down1.json`, the last
rail part with an in-game-unique block id. Measured from the cached jar before
this proposal: `rails_down1.json` is `xsize: 16`, `zsize: 16`,
`refpalette: "rails"`, **12 slices** (911 non-space cells) with exact char
census `:` 30, `_` 26, `e` 27, `f` 3, `g` 2, `j` 18, `y` 279, `~` 526. The
`:` and `_` chars are **not** in `RAILS_PALETTE_JSON`; both are copied verbatim
from the jar's `palettes/default.json` (`:` → `minecraft:iron_bars`, `_` →
`minecraft:smooth_stone_slab[type=bottom]`). No other decoded part writes
`iron_bars` (the building palette only names it as a `damaged` field, which the
decoder does not place), so it is a clean differential. Six rail parts remain
(`down1`, `flat`, `horizontal_end`, `horizontal_water`, `vertical`,
`vertical_water`); `rails_flat`, `rails_horizontal_end` and `rails_vertical`
need no new palette entry but write only shapes already asserted elsewhere, so
`down1` is the stronger next slice.

**Gap line:** the coverage table above, *"6 parts decoded / 202
`blocked(no-city-data-decoder)` … the other 202 parts and all buildings/
palettes/multibuildings have no decoder"*. The new slice adds one more part and
two more verbatim palette entries, moving the count to 7 / 201.

**Tier fit:** `deep` worldgen; the registrar/`ChunkAccess` surface and the
`variant`/`frompalette`/weighted/multi-layer decoder are proven by slices 2–7,
and one more part needs no new SDK API, renderer or manifest key. Place it at a
seventh Y that does not overwrite the 18-layer descent (e.g. y=100..111, above
the street at y=90..95 — the dimension's min_y=0 leaves no room below y=4).
Red-first test: write a `down1_tests` module with a 12-layer shape assertion and
`:`/`_` id assertions against a missing `RAILS_DOWN1_PART_JSON` constant; the
compile must fail (as slice 7's did) before the part is carried.

**Concrete `ctl` assertion:** after placing `down1`, run
`ctl command "execute in lostcities:lostcity run fill 0 96 0 16 112 16 minecraft:air replace minecraft:iron_bars"`
and assert a parsed count `> 0` (30 `:` cells in the part; no other decoded
part places iron bars), the same fill on the vanilla-noise Overworld answers
`No blocks were filled` (count 0), and `DOWN1_SELFTEST=1` inverts the count and
ends `EXIT:1`. All ten existing legs stay green (`check-e2e-logs.py
the_lost_cities-e2e --strict` → 11/11 logs).
