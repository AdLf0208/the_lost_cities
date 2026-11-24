# The Lost Cities — a Horizon 26.3 port

> Generate cities all over the world. Upstream: **McJty**,
> <https://github.com/McJtyMods/LostCities> (MIT).
> The cached artefact is The Lost Cities `26.3-12.0.2` for NeoForge.

This is a **clean-room Rust rewrite** of The Lost Cities for Horizon (Minecraft
Java 26.3), pinned to SDK `0a875da91dbb0fdcb7ba8c42c11341aa229af51a` and
horizon-mixins `e59da9094f0c2fdda6c627fe9eff22dec6a906f0`. It is a first
publication at version **0.1.0**; the city-generator slice shipped as **0.1.1**, the
second multi-layer building part as **0.1.2**, the horizontal rail line as **0.1.3**,
the rail bend as **0.1.4**, the three-way rail split as **0.1.5**, the
eighteen-layer rail descent as **0.1.6**, the twelve-layer first rail descent
as **0.1.7**, the second six-layer building part as **0.1.8**, the six-layer
city centre as **0.1.9**, the six-layer shopping-open front as **0.1.10**, the
seven-layer stone stair as **0.1.11**, the seven-layer large stone stair as
**0.1.12**, the third six-layer building part as **0.1.13** and the second
six-layer city centre as **0.1.14**, the six-layer library part as **0.1.15**
and the six-layer rail-dungeon part as **0.1.16**, the second six-layer
rail-dungeon part as **0.1.17**, and the six-layer rail end cap as **0.1.18**.

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

- **A seventh, twelve-layer first rail descent** (0.1.7): `parts/rails_down1.json`
    (16x16, **12 slices**, 911 non-space cells, `refpalette: "rails"`, embedded
    byte-identically from the cached jar, sha256 `822a38fa…`) is decoded with a
    merged palette that adds the two `palettes/default.json` entries absent from
    the rail palette — `:` → `minecraft:iron_bars` and `_` →
    `minecraft:smooth_stone_slab[type=bottom]` — carried verbatim. It is laid at
    y=100..111, above the street, since min_y=0 leaves no room below the 18-layer
    descent's y=4..21. Iron bars are written by no other decoded part (their
    palettes only name it as a `damaged` field, which the decoder does not place),
    so the in-game assertion fill-replaces `minecraft:iron_bars` at y=96..112;
    vanilla noise fills nothing. No new decoder form, SDK API or manifest key.

- **An eighth, six-layer second building** (0.1.8): `parts/building1_2.json`
    (16x16, six slices, 998 non-space cells, embedded byte-identically from the
    cached jar, sha256 `b2130f36…`) is decoded with a merged palette that adds
    the two `palettes/default.json` entries absent from the building palette —
    `Q` → `minecraft:quartz_block` and `:` → `minecraft:iron_bars` — carried
    verbatim. It is laid at y=116..121, above the first descent. Quartz is
    written by no other decoded part, so the in-game assertion fill-replaces
    `minecraft:quartz_block` at y=112..122; vanilla noise fills nothing. No new
    decoder form, SDK API or manifest key.

- **A ninth, six-layer city centre** (0.1.9): `parts/center00_1.json`
    (16x16, six slices, 680 non-space cells, embedded byte-identically from the
    cached jar, sha256 `5ade2e79…`) is decoded with a merged palette that adds
    three `palettes/default.json` entries to the standard building groups —
    `Q` → `minecraft:quartz_block`, `S` → `minecraft:smooth_stone_slab[type=double]`
    and the one new `)` → `minecraft:quartz_stairs[facing=south,half=bottom,
    shape=straight]`, all carried verbatim. It is laid at y=128..133, above the
    second building. Quartz stairs are written by no other decoded part, so the
    in-game assertion fill-replaces `minecraft:quartz_stairs` at y=124..134;
    vanilla noise fills nothing. The part is embedded as an escaped string
    literal because each brick row begins `"#`, which would close a raw
    `r#"..."#` string. No new decoder form, SDK API or manifest key.

- **A tenth, six-layer shopping-open front** (0.1.10): `parts/shopping_open01_1.json`
    (16x16, six slices, 12 non-space cells, embedded byte-identically from the
    cached jar, sha256 `c16f5ff…`) is decoded with a palette carrying the single
    `w` → `minecraft:cobblestone_wall` entry copied verbatim from
    `palettes/default.json`. Its private `meta` key is ignored by the decoder like
    any other unknown key. It is laid at y=140..145, above the city centre.
    Cobblestone walls are written by no other decoded part, so the in-game
    assertion fill-replaces `minecraft:cobblestone_wall` at y=136..150; vanilla
    noise fills nothing. No new decoder form, SDK API or manifest key.

## What it does **not** implement

- **An eleventh, seven-layer stone stair** (0.1.11): `parts/stairsnormal.json`
  (6x11, seven slices, 138 non-space cells: `x` 102, `/` 24, `_` 12, embedded
  byte-identically from the jar, sha256 `c6d17e44…`) is decoded with a palette
  carrying three `palettes/default.json` entries verbatim — `x` is the jar's
  `variant: stonebrick` entry, `_` the bottom smooth-stone slab and the one new
  `/` → `minecraft:stone_stairs[facing=west,half=bottom,shape=straight]`. It is
  laid at y=152..158, above the shopping front. Stone stairs are written by no
  other decoded part, so the in-game assertion fill-replaces
  `minecraft:stone_stairs` at y=152..158; vanilla noise fills nothing. The
  generic decoder bounds already validate a non-16 footprint, so no new decoder
  form, SDK API or manifest key was needed.

- **A twelfth, seven-layer large stone stair** (0.1.12): `parts/stairsbig.json`
  (6x12, seven slices, 180 non-space cells: `x` 132, `/` 36, `_` 12, embedded
  byte-identically from the jar, sha256 `ae9dc82a…`) is decoded against the
  **same verbatim `STAIRSNORMAL_PALETTE_JSON`** — all three chars were already
  declared by the earlier stair part, so only the part bytes are new. It is laid
  at y=164..170, above the first stair. Because it shares the
  `minecraft:stone_stairs` id, the in-game assertion fill-replaces that id in
  the **disjoint** y=164..170 band (36 `/` cells, versus 24 in the earlier
  part's y=152..158 band); vanilla noise fills nothing. No new decoder form,
  SDK API or manifest key was needed.

- **A thirteenth, six-layer third building part** (0.1.13):
  `parts/building1_3.json` (16x16, six slices, 1,206 non-space cells: `#` 784,
  `a` 186, `W` 168, `:` 30, `@` 24, `l` 6, `L` 3, `F` 2, `1` 2, `T` 1, embedded
  byte-identically from the jar, sha256 `93cd0a5b…`) is decoded against a merged
  palette that adds one entry copied from `palettes/common.json`, `W` →
  `minecraft:water`, to the already-carried building palette. It is laid at
  y=176..181, above the large stair. Water is placed by no other decoded part,
  so the in-game assertion fill-replaces `minecraft:water` at y=176..181, with
  the vanilla-noise Overworld as control. No new decoder form, SDK API or
  manifest key was needed.

- **A fourteenth, six-layer second city centre** (0.1.14):
  `parts/center00_2.json` (16x16, six slices, 771 non-space cells: `#` 420,
  `a` 126, `Q` 107, `S` 97, `)` 12, `1` 5, `C` 2, `4` 1, `T` 1, embedded
  byte-identically from the jar, sha256 `3fdc948a…`) is decoded against a merged
  palette that adds two entries copied verbatim from `palettes/common.json`:
  `C` → `minecraft:chest[facing=north]` (its `loot` field ignored like `mob`)
  and `4` → `minecraft:spawner` (the block id already placed by `1`). It is laid
  at y=188..193, above the third building. A chest is placed by no other decoded
  part, so the in-game assertion fill-replaces `minecraft:chest` at y=188..193,
  with the vanilla-noise Overworld as control and `CENTER002_SELFTEST=1`
  inverting the count. No new decoder form, SDK API or manifest key was needed.

- **A fifteenth, six-layer library** (0.1.15):
  `parts/library10_2.json` (16x16, six slices, 680 non-space cells: `#` 361,
  `Q` 144, `a` 75, `S` 56, `L` 30, `-` 6, `.` 6, `T` 2, embedded
  byte-identically from the jar, sha256 `e52c4556…`) is decoded against a merged
  palette that adds two entries copied verbatim from `palettes/common.json`:
  `.` → `minecraft:oak_fence` and `-` → `minecraft:oak_pressure_plate`. It is
  laid at y=200..205, above the second city centre. Neither block is placed by
  any other decoded part, so the in-game assertion fill-replaces both ids at
  y=200..205 (with a property-exact fence read-back at (5,201,7)), with the
  vanilla-noise Overworld as control and `LIBRARY1002_SELFTEST=1` inverting the
  fence count. No new decoder form, SDK API or manifest key was needed.

- **A sixteenth, six-layer rail-dungeon part** (0.1.16):
  `parts/rail_dungeon1.json` (16x16, six slices, 1,536 non-space cells: `y` 1127,
  `~` 382, `!` 17, `C` 4, `1` 3, `4` 3, embedded byte-identically from the jar,
  sha256 `1216e334…`) is decoded against the carried centre palette plus the
  part's `refpalette: rails` `y`/`~` entries and one new weighted entry copied
  verbatim from `palettes/common.json`: `!` → diamond 15 / gold 20 / emerald
  15 / iron 1000, which the decoder resolves deterministically to
  `minecraft:iron_block`. It is laid at y=212..217, above the library. Iron
  blocks are placed by no other decoded part, so the in-game assertion
  fill-replaces `minecraft:iron_block` at y=212..217 with a property-exact
  read-back at (10,214,6), the vanilla-noise Overworld as control and
  `RAILDUNGEON1_SELFTEST=1` inverting the count. No new decoder form, SDK API
  or manifest key was needed.

- **A seventeenth, six-layer second rail-dungeon part** (0.1.17):
   `parts/rail_dungeon2.json` (16x16, six slices, 1,536 non-space cells: `y` 1148,
   `~` 380, `1` 3, `!` 3, `F` 2, embedded byte-identically from the jar,
   sha256 `6b5081f1…`) uses the same merged rail-dungeon palette as 0.1.16 — its
   `y`/`~` come from `refpalette: rails`, its `1`/`F`/`!` from the verbatim
   `palettes/common.json` entries (spawner; the weighted furniture list resolving
   to `minecraft:cobweb`; the weighted treasure list resolving deterministically
   to `minecraft:iron_block`) — so no new id or palette entry is introduced. The
   three `!` cells at (9,2,8)/(9,2,9)/(9,2,10) resolve to iron blocks. It is laid
   at y=224..229, a band disjoint from the first dungeon's y=212..217; the
   in-game assertion fill-replaces `minecraft:iron_block` there and requires
   exactly **3** blocks (versus 17 in the lower band, already cleared by the
   earlier leg), with a property-exact read-back at (9,226,8), the vanilla-noise
   Overworld as control and `RAILDUNGEON2_SELFTEST=1` inverting the count. No new
   decoder form, SDK API or manifest key was needed.

- **An eighteenth, six-layer rail end cap** (0.1.18):
  `parts/rails_horizontal_end.json` (16x16, six slices, 942 non-space cells: `y` 589,
  `~` 314, `e` 33, `f` 3, `g` 2, `T` 1, embedded byte-identically from the jar,
  sha256 `6e302fa0…`) uses the **same verbatim `RAILS_PALETTE_JSON`** as the
  lower rail line — all six of its chars are already carried, so no new id or
  palette entry is introduced. It is laid at y=236..241, a band disjoint from
  the horizontal line's y=54..59; the in-game assertion fill-replaces
  `minecraft:rail` there and requires exactly **36** blocks (33 in-chunk `e`
  cells plus 3 from the inclusive 17x17 fill window's x=16 rim, calibrated
  against the lower line's 48 = 45 + 3), with a property-exact read-back at
  (0,237,5), the vanilla-noise Overworld as control and `RAILEND_SELFTEST=1`
  inverting the count. No new decoder form, SDK API or manifest key was needed.

- **The other 190 parts, 38 buildings, 31 palettes, 15 multibuildings, the
  spheres and the remaining rail-dungeon parts, and Java-assembled structures**
  — still `blocked(no-city-data-decoder)` / `blocked(no-structure-type-for-cities)`:
  this port decodes exactly eighteen parts and the palette entries they use.
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
