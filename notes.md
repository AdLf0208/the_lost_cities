# The Lost Cities — port notes (slices 1–12)

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

## Slice 8 — the seventh, twelve-layer first rail descent (0.1.7)

### The part and both palette sources as read from the cached jar (source first)

Everything below was read from `ports/_shared/licence-cache/the-lost-cities.jar`
(sha `666cd25d…`, verified before any edit):

- `data/lostcities/lostcities/parts/rails_down1.json`: **5,211 B**, sha256
  `822a38fa5420ab545f78248a216a937e0f1f8dc78d6a258ffbae45da8340411e`. The
  brief's byte guess was wrong; the artefact is the authority. It is `xsize: 16`,
  `zsize: 16`, `refpalette: "rails"`, **12 slices** (911 non-space cells), every
  row 16 chars. The exact non-space char census is `:` 30, `_` 26, `e` 27, `f` 3,
  `g` 2, `j` 18, `y` 279, `~` 526 — measured from the jar, not from the brief.
  Embedded byte-identically in `runtime::RAILS_DOWN1_PART_JSON` (verified by
  sha256 after rustfmt).
- The part's `refpalette` is `palettes/rails.json`, whose twelve entries are the
  same verbatim set slice 4 already carries in `RAILS_PALETTE_JSON`; `y` is the
  jar's `variant: stonebrick` entry and `g`/`~`/space come from
  `palettes/common.json`. No id or char was invented.
- `:` and `_` are **absent** from `RAILS_PALETTE_JSON`. Both are copied verbatim
  from the jar's `palettes/default.json`: `:` → `minecraft:iron_bars`, `_` →
  `minecraft:smooth_stone_slab[type=bottom]`. They are merged into a new
  `runtime::RAILS_DOWN1_PALETTE_JSON` (the 17 rail entries plus these two, byte
  for byte). No other decoded part writes `iron_bars`: the street/building/rail
  palettes only name it as a `damaged` field, which the decoder does not place.
  That makes it a clean in-game differential.

No new decoder form, SDK API or manifest key: `variant`, `frompalette`, weighted
`blocks` and generic multi-layer bounds already exist. `lib.rs` lays the part at
y=100 (`DOWN1_BASE_Y`), spanning y=100..111 above the street (y=90) — the
dimension's min_y=0 leaves no room below the 18-layer descent's y=4..21, so the
sealed band above the street is the only free one. Its block ids are unioned into
the same one-time `block_key` request. The seven parts do not collide: the
highest existing part before this slice was the street at y=90.

### Red verbatim

`evidence/the_lost_cities/down1-test-red.log`, before the part or its palette
existed: three `error[E0425]` — *"cannot find value `RAILS_DOWN1_PART_JSON` in
this scope"* (twice, at the 12-layer shape assertion and at the id resolver) and
*"cannot find value `RAILS_DOWN1_PALETTE_JSON` in this scope"* — each with
rustc's *similarly named constant `RAILS_DOWN2_PART_JSON`* hint —
`error: could not compile 'the_lost_cities' (lib test) due to 3 previous errors`,
build-slot exit **101**. The new tests then passed green: **21 passed / 0
failed**, including `down1_tests::down1_chars_resolve_to_the_jar_iron_bars_and_bottom_slab`,
which pins the artefact's 30 `:` / 26 `_` counts and asserts none of the first six
parts writes an iron bar.

### Both fits lines on the delivered 0.1.7 sha

```
The Lost Cities:
0a875da  made for
The Lost Cities:
0b14f8a  fits
```

Delivered `the_lost_cities.hmod` **34,134 B**, SHA-256
`9f2c2e28d8eaec9252187bea4c2fed3c079c02f4c1d48718b67597101923ac3f`, 17 entries.
The second green line is honest: the registrar/`ChunkAccess` surface exists in
both cached builds; only the clean-room decoder and embedded jar data are ours.

## Slice 9 — the eighth, six-layer second building part (0.1.8)

### The part and both palette sources as read from the cached jar (source first)

Everything below was read from `ports/_shared/licence-cache/the-lost-cities.jar`
(sha `666cd25d…`, verified before any edit); the brief's measurements were checked
against the artefact and matched:

- `data/lostcities/lostcities/parts/building1_2.json`: **2,618 B**, sha256
  `b2130f36f887c90c4b1602f04e3d70bcff0835ac5b176b070ca78e94ff264f74`. It is
  `xsize: 16`, `zsize: 16`, no `refpalette`, **6 slices**, 998 non-space cells, every
  row 16 chars. Exact census: `#` 812, `a` 144, `@` 24, `l` 6, `Q` 3, `*` 2, `:` 2,
  `T` 2, `F` 2, `1` 1. The three `Q` cells are at (x,y,z) (10,1,1), (11,1,1),
  (12,1,1) and the two `:` cells at (2,2,10), (3,2,10) — read from the bytes, not
  guessed. Embedded byte-identically in `runtime::BUILDING_1_2_PART_JSON`
  (verified by sha256 after rustfmt).
- `data/lostcities/lostcities/palettes/default.json`: `Q` →
  `minecraft:quartz_block` with `damaged: minecraft:iron_bars`, and `:` →
  `minecraft:iron_bars`. Both entries are copied verbatim from the jar.
- The part's other chars are the standard-style groups `styles/standard.json`
  selects together: `#` (variant stonebrick), `a`/`@` (glass) and `1`/`T`/`F`/
  `l`/`*`/space from `common` — exactly slice 3's
  `runtime::BUILDING_PALETTE_JSON` (`D` and `L` are declared there but unused by
  this part). No block id or char was invented; the merged
  `runtime::BUILDING_1_2_PALETTE_JSON` is those 11 building entries plus the two
  verbatim `default.json` entries (13 total).

`Q`/quartz is placed by no other decoded part: the street, rail and descent
palettes never name quartz, and the first building part has no `Q` char. That
makes it the clean in-game differential.

No new decoder form, SDK API or manifest key: `variant`, `frompalette`, weighted
`blocks` and generic multi-layer bounds already exist. `lib.rs` lays the part at
y=116 (`BUILDING12_BASE_Y`), spanning y=116..121 above the first descent
(y=100..111), and unions its block ids into the same one-time `block_key`
request.

### Red verbatim

`evidence/the_lost_cities/building12-test-red.log`, before the part or its
palette existed: three `error[E0425]` — *"cannot find value
`BUILDING_1_2_PART_JSON` in this scope"* (twice, at the six-layer shape assertion
and at the id resolver, each with rustc's *similarly named constant
`BUILDING_PART_JSON`* hint) and *"cannot find value
`BUILDING_1_2_PALETTE_JSON` in this scope"* (with the `BUILDING_PALETTE_JSON`
hint) — `error: could not compile 'the_lost_cities' (lib test) due to 3 previous
errors`, build-slot exit **101**. The new tests then passed green:
**23 passed / 0 failed**, including
`building1_2_tests::building12_chars_resolve_to_the_jar_quartz_block`, which pins
the artefact's 3 `Q` / 2 `:` counts and asserts none of the first seven parts
writes a quartz block.

### Both fits lines on the delivered 0.1.8 sha

```
The Lost Cities:
0a875da  made for
The Lost Cities:
0b14f8a  fits
```

Delivered `the_lost_cities.hmod` **35,434 B**, SHA-256
`00515ddedd40fcab8c97d68442f7bd12e75a3044946a5bcc5e293747e7ebe877`, 17 entries.
The second green line is honest: the registrar/`ChunkAccess` surface exists in
both cached builds; only the clean-room decoder and embedded jar data are ours.
No type or variant named `Block`/`Item` was introduced (the `fits` false-fail
trap), and a local type or variant named `Block`/`Item` would false-fail it.

## Slice 10 — the ninth, six-layer city centre (0.1.9)

### The part and its palette sources as read from the cached jar (source first)

Everything below was read from `ports/_shared/licence-cache/the-lost-cities.jar`
(sha `666cd25d…`, verified before any edit); the brief's measurements matched
the artefact:

- `data/lostcities/lostcities/parts/center00_1.json`: **2,618 B**, sha256
  `5ade2e7903e012f38fb9e9559ccda6aa51ef4c9856d2d365b891c118acebdc54`. It is
  `xsize: 16`, `zsize: 16`, no `refpalette`, **6 slices**, 680 non-space cells,
  every row 16 chars. Exact census: `#` 338, `a` 126, `Q` 107, `S` 97, `)` 12 —
  measured from the bytes, not from the brief. The twelve `)` cells are at
  (x,y,z) (13,0,14), (14,0,14), (13,1,9), (14,1,9), (13,2,10), (14,2,10),
  (13,3,11), (14,3,11), (13,4,12), (14,4,12), (13,5,13), (14,5,13). Embedded
  byte-identically in `runtime::CENTER00_1_PART_JSON` (verified by sha256 after
  rustfmt). It is an **escaped** string literal, as in 0.1.8: every brick row
  begins `"#`, which would close a `r#"..."#` raw string immediately.
- `data/lostcities/lostcities/palettes/default.json`: `Q` →
  `minecraft:quartz_block` with `damaged: minecraft:iron_bars`, `S` →
  `minecraft:smooth_stone_slab[type=double]` with the same `damaged` field, and
  the one genuinely new entry `)` →
  `minecraft:quartz_stairs[facing=south,half=bottom,shape=straight]`. All three
  are copied verbatim from the jar.
- The part's other chars are the standard-style groups `styles/standard.json`
  selects together: `#` (variant stonebrick) and `a` (glass) come from the
  carried `runtime::BUILDING_PALETTE_JSON`. The merged
  `runtime::CENTER00_1_PALETTE_JSON` is those 11 building entries plus the 3
  verbatim `default.json` entries (14 total). No block id or char was invented.

Quartz stairs are placed by no other decoded part: the eight earlier parts place
slabs, glass, bookshelves, rails, iron bars and quartz_block, never stairs. That
makes `)` the clean in-game differential. No new decoder form, SDK API or
manifest key: `variant`, `frompalette`, weighted `blocks` and generic
multi-layer bounds already exist. `lib.rs` lays the part at y=128 (`CENTER001_BASE_Y`,
spanning y=128..133 above building1_2 at y=116..121) and unions its block ids
into the same one-time `block_key` request.

### Red verbatim

`evidence/the_lost_cities/center001-test-red.log`, before the part or its
palette existed: three `error[E0425]: cannot find value` — *"cannot find value
`CENTER00_1_PART_JSON` in this scope"* (twice, at the six-layer shape assertion
and at the id resolver) and *"cannot find value `CENTER00_1_PALETTE_JSON` in
this scope"* — `error: could not compile 'the_lost_cities' (lib test) due to 3
previous errors`, build-slot exit **101**. The new tests then passed green:
**25 passed / 0 failed** (up from 23), including
`center00_1_tests::center001_chars_resolve_to_the_jar_quartz_stairs`, which pins
the artefact's 12 `)` / 107 `Q` / 97 `S` counts and asserts none of the first
eight parts writes a quartz stair.

### Both fits lines on the delivered 0.1.9 sha

```
The Lost Cities:
0a875da  made for
The Lost Cities:
0b14f8a  fits
```

Delivered `the_lost_cities.hmod` **37,092 B**, SHA-256
`ebeca9f22eb117d2b512d9bde1df315e6aa5120f96c22a8a8304e56ad57155d7`, 17 entries.
The second green line is honest: the registrar/`ChunkAccess` surface exists in
both cached builds; only the clean-room decoder and embedded jar data are ours.
No local type or variant named `Block`/`Item` was introduced (the `fits`
false-fail trap).

## Slice 11 — the tenth, six-layer shopping-open front (0.1.10)

### The part and its palette as read from the cached jar (source first)

Everything below was read from `ports/_shared/licence-cache/the-lost-cities.jar`
(sha `666cd25d…`, verified before any edit); the brief's measurements matched
the artefact:

- `data/lostcities/lostcities/parts/shopping_open01_1.json`: **2,697 B**, sha256
  `c16f5ff65b9a32b168b8c5bb5edfbdaa970a994bc1434f4a35679b30b4a4ed5d`. It is
  `xsize: 16`, `zsize: 16`, **6 slices**, 12 non-space cells, every row 16 chars.
  Exact census: `w` 12 only; the twelve `w` cells are at x=1, z=0 or 14, y=0..5 —
  read from the bytes, not guessed. A private `meta` array (`dontconnect: true`)
  is carried byte-identically and ignored by the decoder like any unknown key.
  The whole part is embedded verbatim as `runtime::SHOPPING_OPEN01_1_PART_JSON`
  (verified by sha256 after rustfmt). It is a **raw** string literal: no row
  contains the `"#` sequence that would close `r#"..."#` (unlike 0.1.8/0.1.9).
- `data/lostcities/lostcities/palettes/default.json`: the one genuinely new
  entry `w` → `minecraft:cobblestone_wall`, copied verbatim into
  `runtime::SHOPPING_OPEN01_1_PALETTE_JSON`. Spaces are air and are skipped by
  the decoder; no id or char was invented.

Cobblestone walls are placed by no other decoded part (the nine earlier parts
write slabs, glass, bookshelves, rails, iron bars, quartz_block and
quartz_stairs), so `w` is the clean in-game differential. No new decoder form,
SDK API or manifest key: `variant`, `frompalette`, weighted `blocks` and generic
multi-layer bounds already exist. `lib.rs` lays the part at y=140 (`SHOPPING1_BASE_Y`,
spanning y=140..145 above the city centre at y=128..133) and unions its block id
into the same one-time `block_key` request.

### Red verbatim

`evidence/the_lost_cities/shopping1-test-red.log`, before the part or its
palette existed: three `error[E0425]: cannot find value` — *"cannot find value
`SHOPPING_OPEN01_1_PART_JSON` in this scope"* (twice, at the six-layer shape
assertion and at the id resolver) and *"cannot find value
`SHOPPING_OPEN01_1_PALETTE_JSON` in this scope"* — `error: could not compile
'the_lost_cities' (lib test) due to 3 previous errors`, build-slot exit **101**.
The new tests then passed green: **27 passed / 0 failed** (up from 25), including
`shopping_open01_1_tests::shopping1_chars_resolve_to_the_jar_cobblestone_wall`,
which pins the artefact's 12 `w` cells and asserts none of the first nine parts
writes a cobblestone wall.

### Both fits lines on the delivered 0.1.10 sha

```
The Lost Cities:
0a875da  made for
The Lost Cities:
0b14f8a  fits
```

Delivered `the_lost_cities.hmod` **38,460 B**, SHA-256
`a729fbd163b15a7ef1f87804c67adfb508b46f404f262d21be2f270d87a97e9f`, 17 entries.
The second green line is honest: the registrar/`ChunkAccess` surface exists in
both cached builds; only the clean-room decoder and embedded jar data are ours.
No type or variant named `Block`/`Item` was introduced (the `fits` false-fail
trap).

## Slice 12 — the eleventh, seven-layer stone stair (0.1.11)

### The part and its palette as read from the cached jar (source first)

Everything below was read from `ports/_shared/licence-cache/the-lost-cities.jar`
(sha `666cd25d…`, verified before any edit); the brief's measurements matched
the artefact exactly:

- `data/lostcities/lostcities/parts/stairsnormal.json`: **1,484 B**, sha256
  `c6d17e44238f625378d2f8e474aba1a1ea48c755200ecd5214b3b75d9bb387bc`. It is
  `xsize: 6`, `zsize: 11`, no `refpalette`, **7 slices** (every slice 11 rows of
  six chars), 138 non-space cells. Exact census: `x` 102, `/` 24, `_` 12 —
  measured from the bytes. The twelve `/` cells are at x=5,y=0,z=6..9;
  x=4,y=1,z=6..9; x=3,y=2,z=6..9; x=2,y=3,z=6..9; x=1,y=4,z=6..9 and
  x=0,y=5,z=6..9. A private `meta` array (`z1`=5, `z2`=10) is carried
  byte-identically and ignored by the decoder like any unknown key. The whole
  part is embedded byte-identically in `runtime::STAIRSNORMAL_PART_JSON`
  (sha verified in the source after rustfmt **and** in the packed archive).
- `data/lostcities/lostcities/palettes/default.json`: three verbatim entries —
  `x` → the jar's `variant: stonebrick` entry (resolved through the same
  `variants/stonebrick.json` as every earlier part), `_` →
  `minecraft:smooth_stone_slab[type=bottom]`, and the one new id `/` →
  `minecraft:stone_stairs[facing=west,half=bottom,shape=straight]`. They are
  copied byte-for-byte into `runtime::STAIRSNORMAL_PALETTE_JSON`. No id or char
  was invented.

Stone stairs are placed by no other decoded part (the ten earlier parts write
slabs, glass, bookshelves, rails, iron bars, quartz_block, quartz_stairs and
cobblestone walls), so `/` is the clean in-game differential. No new decoder
form, SDK API or manifest key: `variant`, `frompalette`, weighted `blocks` and
the generic row/slice bounds already validate a non-16 footprint. `lib.rs`
lays the part at y=152 (`STAIRSNORMAL_BASE_Y`, spanning y=152..158 above the
shopping front at y=140..145) and unions its block ids into the same one-time
`block_key` request.

### Red verbatim

`evidence/the_lost_cities/stairsnormal-test-red.log`, before the part or its
palette existed: three `error[E0425]: cannot find value` — *"cannot find value
`STAIRSNORMAL_PART_JSON` in this scope"* (twice, at the 7-layer shape assertion
and at the id resolver) and *"cannot find value
`STAIRSNORMAL_PALETTE_JSON` in this scope"* — `error: could not compile
'the_lost_cities' (lib test) due to 3 previous errors`, build-slot exit **101**.
The new tests then passed green: **29 passed / 0 failed** (up from 27),
including `stairsnormal_tests::stairsnormal_chars_resolve_to_the_jar_stone_stairs`,
which pins the artefact's 24 `/` / 12 `_` / 138-cell census and asserts none of
the first ten parts writes a stone stair.

### Both fits lines on the delivered 0.1.11 sha

```
The Lost Cities:
0a875da  made for
The Lost Cities:
0b14f8a  fits
```

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
  clean-room decoder and decodes **eleven parts** (`street_straight`,
  `building1_1`, `rails_horizontal`, `rails_bend`, `rails_3split`, `rails_down2`, `rails_down1`, `building1_2`, `center00_1`,
  `shopping_open01_1`, `stairsnormal`) plus their
  palette/variant entries. The **other 197 parts**, 38 buildings,
  31 palettes, 15 multibuildings, 12 variants, 6 scattered and all
  `citystyles`/`styles`/`worldstyles` are still blocked — no decoder was written
  for them.
- **`blocked(no-structure-type-for-cities)`** — `StructuresRegistrar` exists, but
  the jar has **no** `worldgen/structure/*.json` for a jigsaw the SDK could start;
  the cities are assembled by Java code. Registering an empty `StructureType` would
  be an invented structure, so none is registered.
- **Not in the artefact:** no item, block, model, recipe, advancement, biome or
  structure files (census above).
- **Deferred:** the remaining 198 parts, spheres/rail dungeons, the config
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
| 322 internal generator JSON files | 11 parts decoded / 197 `blocked(no-city-data-decoder)` | clean-room decoder for `street_straight`, the six-layer `building1_1`, the six-layer `rails_horizontal`, the six-layer `rails_bend`, the six-layer `rails_3split`, the eighteen-layer `rails_down2`, the twelve-layer `rails_down1`, the six-layer `building1_2`, the six-layer `center00_1`, the six-layer `shopping_open01_1` and the seven-layer 6x11 `stairsnormal`, with `variant`/`frompalette`/weighted/multi-layer support; the other 197 parts and all buildings/palettes/multibuildings have no decoder; not packed |
| horizontal rail line (`parts/rails_horizontal.json` + `palettes/rails.json`) | **tested** | 45 east-west rails + 3 powered rails decoded from the artefact's ids; laid at y=54; in-game fill-replace of `minecraft:rail` at y=50..64 → **48 blocks**, vanilla-noise Overworld → **0**, `RAILS_SELFTEST=1` → `EXIT:1` |
| rail bend (`parts/rails_bend.json` + `palettes/rails.json`) | **tested** | 9 north_south + 2 powered north_south + 1 north_west + 1 south_west + 14 east_west + 2 powered east_west rails decoded from the artefact's ids; laid at y=36; in-game fill-replace of `minecraft:rail[shape=north_south]` at y=30..44 → **10 blocks** (9 in-chunk + 1 from the inclusive 17×17 window), vanilla-noise Overworld → **0**, `BEND_SELFTEST=1` → `EXIT:1` |
| three-way rail split (`parts/rails_3split.json` + `palettes/rails.json`) | **tested** | 1 north_east + 1 south_east + 9 north_south + 2 powered north_south + 26 east_west + 4 powered east_west rails decoded from the artefact's ids (byte-identical part, sha `b0c8bd61…`); laid at y=18; in-game fill-replace of `minecraft:rail[shape=north_east]` at y=12..26 → **1 block**, vanilla-noise Overworld → **0**, `SPLIT_SELFTEST=1` → `EXIT:1` |
| eighteen-layer rail descent (`parts/rails_down2.json` + `palettes/rails.json`) | **tested** | 21 `j` ascending_east + 15 `i` powered ascending_east + 12 `f` powered east_west rails (7,779-byte part, sha `f2f6eba…`, byte-identical); laid at y=4..21; in-game fill-replace of `minecraft:rail[shape=ascending_east]` at y=0..24 → **21 blocks**, vanilla-noise Overworld → **0**, `DOWN2_SELFTEST=1` → `EXIT:1` |
| twelve-layer first rail descent (`parts/rails_down1.json` + merged palette) | **tested** | 30 `:` iron_bars + 26 `_` bottom slabs decoded from the artefact's ids (5,211-byte part, sha `822a38fa…`, byte-identical; two `palettes/default.json` entries added verbatim); laid at y=100..111; in-game fill-replace of `minecraft:iron_bars` at y=96..112 → **>0 blocks**, vanilla-noise Overworld → **0**, `DOWN1_SELFTEST=1` → `EXIT:1` |
| second six-layer building (`parts/building1_2.json` + merged palette) | **tested** | 3 `Q` quartz_block + 2 `:` iron_bars decoded from the artefact's ids (2,618-byte part, sha `b2130f36…`, byte-identical; two `palettes/default.json` entries added verbatim); laid at y=116..121; in-game fill-replace of `minecraft:quartz_block` at y=112..122 → **>0 blocks**, vanilla-noise Overworld → **0**, `BUILDING12_SELFTEST=1` → `EXIT:1` |
| nine-layer city centre (`parts/center00_1.json` + merged palette) | **tested** | 12 `)` quartz_stairs + 107 `Q` quartz_block + 97 `S` double slabs decoded from the artefact's ids (2,618-byte part, sha `5ade2e79…`, byte-identical escaped literal; three `palettes/default.json` entries added verbatim, `)` the only new block id); laid at y=128..133; in-game fill-replace of `minecraft:quartz_stairs` at y=124..134 → **12 blocks**, vanilla-noise Overworld → **0**, `CENTER001_SELFTEST=1` → `EXIT:1` |
| six-layer shopping-open front (`parts/shopping_open01_1.json` + merged palette) | **tested** | 12 `w` cobblestone_walls decoded from the artefact's verbatim `palettes/default.json` entry (2,697-byte part, sha `c16f5ff…`, byte-identical raw literal with a private `meta` key the decoder ignores); laid at y=140..145; in-game fill-replace of `minecraft:cobblestone_wall` at y=136..150 → **18 blocks** (12 in-part plus neighbours in the inclusive 17×17 window), vanilla-noise Overworld → **0**, `SHOPPING1_SELFTEST=1` → `EXIT:1` |
| seven-layer stone stair (`parts/stairsnormal.json` + verbatim palette entries) | **tested** | 24 `/` stone_stairs + 12 `_` bottom slabs decoded from the artefact's verbatim `palettes/default.json` entries (1,484-byte part, sha `c6d17e44…`, byte-identical raw literal with a private `meta` key the decoder ignores; `x` resolves through the jar stonebrick variant); laid at y=152..158; in-game fill-replace of `minecraft:stone_stairs` at y=152..158 → **28 blocks** (24 in-part plus neighbours in the inclusive 17×17 window), vanilla-noise Overworld → **0**, `STAIRS_SELFTEST=1` → `EXIT:1` |
| structure types assembled in Java | `blocked(no-structure-type-for-cities)` | no carried structure JSON for a jigsaw; none invented |
| 5 legacy `.lang` files, 18 GUI textures, Java classes, editor/commands/GUI | not ported (out of tier) | no carried art; no equivalent hook or screen at this pin |
| items/blocks/models/recipes/advancements/biomes | not in artefact | census above |

## Verification (fresh, 2026-11-24, version 0.1.11)

Pin `0a875da…`, mixins `e59da909…`, package **`the_lost_cities.hmod` 40,042 B,
SHA-256 `0acc5d54dba2f6fb42bd3c767e761a66a715d329e9a617bf4d30edad98a0edfe`,
17 entries**. Full table in [`VALIDATION.md`](VALIDATION.md).

- `tools/carry.py --check` green against the final tree (9 carried, 2 loot tables
  shape-upgraded); `check-recipe-types.py`/`check-manifest-deps.py` clean (no
  recipe type; only vanilla block ids named in code).
- Slices 1–12 were each red first: the dimension-contract tests, slice 2's 11
  unresolved decoder symbols, slice 3's 11 unresolved variant/building symbols,
  slice 4's 3 unresolved rail symbols, slice 5's 2 unresolved
  `RAILS_BEND_PART_JSON`, slice 6's 2 unresolved `RAILS_3SPLIT_PART_JSON`,
  slice 7's 2 unresolved `RAILS_DOWN2_PART_JSON`, slice 8's 3 unresolved
  `RAILS_DOWN1_PART_JSON`/`RAILS_DOWN1_PALETTE_JSON`, slice 9's 3 unresolved
  `BUILDING_1_2_PART_JSON`/`BUILDING_1_2_PALETTE_JSON`, slice 10's 3 unresolved
  `CENTER00_1_PART_JSON`/`CENTER00_1_PALETTE_JSON`, slice 11's 3 unresolved
  `SHOPPING_OPEN01_1_PART_JSON`/`SHOPPING_OPEN01_1_PALETTE_JSON` and slice 12's
  3 unresolved `STAIRSNORMAL_PART_JSON`/`STAIRSNORMAL_PALETTE_JSON` symbols
  above. Final suite: **29 passed / 0 failed**; `cargo clippy --all-targets
  -- -D warnings` clean (0 warnings); `rustfmt --check` clean. `sdk-test.log`
  regenerated for 0.1.11 **after** the final build; the 0.1.10 log retired to
  `retired-0.1.10/`.
- `cargo horizon-mod check` on the package: *0.1.11,
  native-1/0a875da…, 40,042 bytes, sha 0acc5d54…*. No new API item or SDK
  surface was needed (132 API items).
- `fits --build 0a875da`: **`made for`**. `fits --build 0b14f8a`: **`fits`** —
  the chunk-generator registrar and `DimensionSettings` exist in both cached
  builds (only the Rust decoder and embedded jar data are ours), so the second
  green line is honest. No local type or variant named `Block`/`Item` was
  introduced (the false-fail trap).
- Launcher native build: **`Compiling the_lost_cities (1 of 1)`**, tag
  `native-0a875da-c8ac0678fbc4`; `native-mods.json` names **0.1.11** (checked
  before the legs, so a concurrent build's prune cannot make the legs test a
  stale mod). No stale the_lost_cities `native-*` tag remained before the build
  (audited `versions`, `stale-tags` and `retired`); no exit 10 from a concurrent
  native build occurred.
- `verify-sdk-package.py the_lost_cities` → **1/1** (incl. "package matches the
  tree" — README/manifest/Cargo/src edited before the final build, and the
  1,484-byte stair part verified byte-identically inside the packed archive);
  `check-e2e-logs.py the_lost_cities-e2e --strict` →
  `logs=15 ok=15 missing-exit=0 bad-exit=0`;
  `check-skills.py --notes ports/the_lost_cities --strict` → PASS;
  `tools/carry.py --check` → 9 carried; `check-recipe-types.py`/`check-manifest-deps.py`
  clean. All 15 legs were re-run against 0.1.11 with `E2E_TMP` taken from
  `launcher-game.sh env` (`/home/adlf/.cache/horizon-e2e/scratch`, not `/tmp`).

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
ctl command "execute in lostcities:lostcity run fill 0 96 0 16 112 16 minecraft:air replace minecraft:iron_bars"
   -> {"chat":["Successfully filled 30 block(s)"],"ok":true}    # first descent's : char at y=100..111
ctl command "fill 0 96 0 16 112 16 minecraft:air replace minecraft:iron_bars"  # Overworld console
   -> {"chat":["No blocks were filled"],"ok":true}              # vanilla-noise control = 0
ctl command "execute in lostcities:lostcity run fill 0 112 0 16 122 16 minecraft:air replace minecraft:quartz_block"
   -> {"chat":["Successfully filled 3 block(s)"],"ok":true}     # second building's Q char at y=116..121
ctl command "fill 0 112 0 16 122 16 minecraft:air replace minecraft:quartz_block"  # Overworld console
   -> {"chat":["No blocks were filled"],"ok":true}              # vanilla-noise control = 0
ctl command "execute in lostcities:lostcity run fill 0 124 0 16 134 16 minecraft:air replace minecraft:quartz_stairs"
   -> {"chat":["Successfully filled 12 block(s)"],"ok":true}    # city centre's ) char at y=128..133
ctl command "fill 0 124 0 16 134 16 minecraft:air replace minecraft:quartz_stairs"  # Overworld console
   -> {"chat":["No blocks were filled"],"ok":true}              # vanilla-noise control = 0
ctl command "execute in lostcities:lostcity run fill 0 136 0 16 150 16 minecraft:air replace minecraft:cobblestone_wall"
   -> {"chat":["Successfully filled 18 block(s)"],"ok":true}    # shopping part's w char at y=140..145
ctl command "fill 0 136 0 16 150 16 minecraft:air replace minecraft:cobblestone_wall"  # Overworld console
   -> {"chat":["No blocks were filled"],"ok":true}              # vanilla-noise control = 0
ctl command "execute in lostcities:lostcity run fill 0 152 0 16 158 16 minecraft:air replace minecraft:stone_stairs"
   -> {"chat":["Successfully filled 28 block(s)"],"ok":true}    # stair part's / char at y=152..158
ctl command "fill 0 152 0 16 158 16 minecraft:air replace minecraft:stone_stairs"  # Overworld console
   -> {"chat":["No blocks were filled"],"ok":true}              # vanilla-noise control = 0
ctl command "execute if block 50 80 50 #lostcities:notbreakable"
   -> {"chat":["Test passed"],"ok":true}
ctl command "execute if block 50 80 50 #lostcities:no_such_tag"
   -> "Unknown block tag 'lostcities:no_such_tag'"
ctl command "loot give @p loot lostcities:chests/lostcitychest"  -> Dropped 3 [Diamond]
ctl command "loot give @p loot lostcities:chests/raildungeonchest" -> Dropped 4 items
ctl command "loot give @p loot lostcities:chests/no_such_table"
   -> "Can't find element '…' in registry 'minecraft:loot_table'"
PASS: the lostcity dimension answered, all eleven decoded parts authored their own blocks
      in it (136 slabs at y=90, 4 bookshelves at y=72, 48 east_west rails at y=54..59,
      10 north_south rails at y=36..41, 1 north_east rail at y=18..23, 21
      ascending_east rails at y=4..21, 30 iron bars at y=100..111, 3 quartz
      blocks at y=116..121, 12 quartz stairs at y=128..133, 18 cobblestone
      walls at y=140..145 and 28 stone stairs at y=152..158 — each 0 on vanilla
      noise), #lostcities:notbreakable matched bedrock, and both carried loot
      tables yielded items
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
inverted assertion expected 0` + `EXIT:1`; `DOWN1_SELFTEST=1` ends
`FAIL: DOWN1_SELFTEST: the first descent authored 30 iron-bar(s); the inverted
assertion expected 0` + `EXIT:1`; `BUILDING12_SELFTEST=1` ends
`FAIL: BUILDING12_SELFTEST: the part authored 3 quartz block(s); the inverted
assertion expected 0` + `EXIT:1`; `CENTER001_SELFTEST=1` ends
`FAIL: CENTER001_SELFTEST: the part authored 12 quartz staircase(s); the inverted
assertion expected 0` + `EXIT:1`; `SHOPPING1_SELFTEST=1` ends
`FAIL: SHOPPING1_SELFTEST: the part authored 18 cobblestone wall(s); the inverted
assertion expected 0` + `EXIT:1`; `STAIRS_SELFTEST=1` ends
`FAIL: STAIRS_SELFTEST: the part authored 28 stone stair(s); the inverted
assertion expected 0` + `EXIT:1`. The pre-fix loot counterfactual
(`Dropped 0 items`) is preserved in `the_lost_cities/loot-probe.log`.
Logs: `ports/_shared/evidence/the_lost_cities-e2e/e2e{,-control,-selftest,-tag-selftest,-city-selftest,-building-selftest,-rails-selftest,-bend-selftest,-split-selftest,-down2-selftest,-down1-selftest,-building12-selftest,-center001-selftest,-shopping1-selftest,-stairs-selftest}.log`;
the 0.1.2 logs are retired to `retired-0.1.2/`.

## Next slice

**Family:** decode the next non-16 stair part, `parts/stairsbig.json`. Measured
from the cached jar before this proposal (sha `666cd25d…`): **1,596 B**,
`xsize: 6`, `zsize: 12`, no `refpalette`, **7 slices** (every row six chars,
every slice twelve rows), 180 non-space cells. Exact census: `x` 132, `/` 36,
`_` 12 — all three chars are already declared verbatim in the carried
`STAIRSNORMAL_PALETTE_JSON` (`x` = jar stonebrick variant, `_` = bottom
smooth-stone slab, `/` = `minecraft:stone_stairs[facing=west,half=bottom,
shape=straight]`). The agent must re-read the artefact, record its sha256 and
embed the bytes byte-identically before any edit. This part reuses the same id
as `stairsnormal`, so the clean differential is a **disjoint height band**, not
a new block id.

**Gap line:** the coverage table above, *"11 parts decoded / 197
`blocked(no-city-data-decoder)` … the other 197 parts and all buildings/
palettes/multibuildings have no decoder"*. The new slice moves the count to 12 / 196.

**Tier fit:** `deep` worldgen; the registrar/`ChunkAccess` surface and the
generic `variant`/multi-layer decoder are proven by slices 2–12. One more part
with the already-carved 6-wide footprint needs no new decoder form, SDK API,
renderer or manifest key. Place it at y=164..170, the next sealed band above
`stairsnormal` (y=152..158) inside the dimension's 256 height. Red first: a
`stairsbig_tests` module with a 7-layer shape assertion and a `/`/stone-stairs
id assertion against missing `STAIRSBIG_PART_JSON` (reusing
`STAIRSNORMAL_PALETTE_JSON`) must fail to compile.

**Concrete `ctl` assertion:** run
`ctl command "execute in lostcities:lostcity run fill 0 164 0 16 170 16 minecraft:air replace minecraft:stone_stairs"`
and assert a parsed count `> 0` (36 `/` cells; the earlier stair part's 28 are
at y=152..158, outside this band), the same fill on the vanilla-noise Overworld
answers `No blocks were filled` (0), and `STAIRSBIG_SELFTEST=1` inverts the
count and ends `EXIT:1`. All fifteen existing legs stay green
(`check-e2e-logs.py the_lost_cities-e2e --strict` → 16/16 logs).
