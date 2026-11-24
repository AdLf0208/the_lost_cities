# The Lost Cities — validation record (v0.1.18)

Pin **`0a875da91dbb0fdcb7ba8c42c11341aa229af51a`**, mixins
`e59da9094f0c2fdda6c627fe9eff22dec6a906f0`, game 26.3 (Nightly
`nightly-2026-11-20-0a875da`), `api = "0.2"`.

**Version 0.1.11** — an update of our first publication (0.1.0, PR #1861); 0.1.1
added the first deterministic city chunk generator, 0.1.2 the six-layer building
part, 0.1.3 the six-layer horizontal rail line, 0.1.4 the six-layer rail
bend from `parts/rails_bend.json`, 0.1.5 the three-way rail split from
`parts/rails_3split.json`, 0.1.6 the eighteen-layer rail descent from
`parts/rails_down2.json`, 0.1.7 the twelve-layer first rail descent from
`parts/rails_down1.json`, 0.1.8 the second six-layer building from
`parts/building1_2.json` (whose `Q` char adds a verbatim
`palettes/default.json` entry, `minecraft:quartz_block`), 0.1.9 the
six-layer city centre from `parts/center00_1.json`, whose `)` char adds the
verbatim `minecraft:quartz_stairs[facing=south,half=bottom,shape=straight]`
entry, 0.1.10 the six-layer shopping-open front from
`parts/shopping_open01_1.json`, whose `w` char adds the verbatim
`minecraft:cobblestone_wall` entry, and 0.1.11 the seven-layer 6x11 stone
stair from `parts/stairsnormal.json`, whose `/` char adds the verbatim
`minecraft:stone_stairs[facing=west,half=bottom,shape=straight]` entry, and 0.1.12
the seven-layer 6x12 large stone stair from `parts/stairsbig.json`, which reuses
the same verbatim stair palette entries (only the part bytes are new) and is
proven in the disjoint y=164..170 height band, and 0.1.13 the third six-layer
building from `parts/building1_3.json`, whose `W` char adds the verbatim
`palettes/common.json` entry `minecraft:water`, proven at y=176..181, and 0.1.14
the second six-layer city centre from `parts/center00_2.json`, whose `C` char
adds the verbatim `palettes/common.json` entry `minecraft:chest[facing=north]`
(`loot` ignored like `mob`), proven at y=188..193 with a north-facing chest
read-back, and 0.1.15 the fifteen-part six-layer library from
`parts/library10_2.json`, whose `.` and `-` chars add the verbatim
`palettes/common.json` entries `minecraft:oak_fence` and
`minecraft:oak_pressure_plate`, proven at y=200..205, and 0.1.16 the six-layer
rail dungeon from `parts/rail_dungeon1.json`, whose `!` char resolves the jar's
weighted treasure list deterministically to `minecraft:iron_block`, proven at
y=212..217, and 0.1.17 the second six-layer rail dungeon from
`parts/rail_dungeon2.json`, whose three `!` cells resolve the same weighted list
to `minecraft:iron_block` (and whose two `F` cells resolve to `minecraft:cobweb`
via the verbatim common furniture list), proven at y=224..229 with exactly three
iron blocks in the disjoint upper band, and 0.1.18 the eighteen-part six-layer
rail end cap from `parts/rails_horizontal_end.json`, whose 33 `e` cells reuse the
verbatim rail palette at y=236..241 and read 36 in game (33 in-chunk plus 3 from
the fill window's x=16 rim, calibrated against the lower line's 48 = 45 + 3).
Cargo and the manifest agree at 0.1.18.
The orchestrator publishes; this port does not touch registry/PR/ledger.

## The deliverable

| | |
|---|---|
| package | `ports/the_lost_cities/the_lost_cities.hmod` |
| size | **51,306 B** |
| SHA-256 | `4708adf3cf42fda0364426d5f36c65808671b7c526b5175d98fac8ee0bffecb3` |
| version | 0.1.18 (manifest + Cargo.toml) |
| entries | 17 (6 tags + 2 loot tables + 1 lang + manifest/Cargo/src/docs/uses.json) |
| packed Cargo.lock | none |
| runtime | `src/runtime.rs` byte-identical in tree and package (`3a2bd7b608ce`) |
| embedded bend part | 2,643 B, sha256 `9969f02e8c9b566de15616eecb5598dbea4a4ff5528c5b8792fd4143497e3232`, byte-identical to the jar's `data/lostcities/lostcities/parts/rails_bend.json` |
| embedded split part | 2,643 B, sha256 `b0c8bd61f1f3b3641b9bf20a57087429123b7de9353dd1fd3129e40717777eb0`, byte-identical to the jar's `parts/rails_3split.json` |
| embedded descent part | 7,779 B, sha256 `f2f6ebabe2ae71dc6c094496de0b90ebda97415ecc619b7c7b8c78161e65be89`, byte-identical to the jar's `parts/rails_down2.json` (18 slices) |
| embedded first descent part | 5,211 B, sha256 `822a38fa5420ab545f78248a216a937e0f1f8dc78d6a258ffbae45da8340411e`, byte-identical to the jar's `parts/rails_down1.json` (12 slices, 911 non-space cells) |
| embedded second building part | 2,618 B, sha256 `b2130f36f887c90c4b1602f04e3d70bcff0835ac5b176b070ca78e94ff264f74`, byte-identical to the jar's `parts/building1_2.json` (6 slices, 998 non-space cells) |
| embedded city-centre part | 2,618 B, sha256 `5ade2e7903e012f38fb9e9559ccda6aa51ef4c9856d2d365b891c118acebdc54`, byte-identical to the jar's `parts/center00_1.json` (6 slices, 680 non-space cells, escaped literal) |
| embedded shopping part | 2,697 B, sha256 `c16f5ff65b9a32b168b8c5bb5edfbdaa970a994bc1434f4a35679b30b4a4ed5d`, byte-identical to the jar's `parts/shopping_open01_1.json` (6 slices, 12 `w` cells, raw literal with a private `meta` key) |
| embedded stair part | 1,484 B, sha256 `c6d17e44238f625378d2f8e474aba1a1ea48c755200ecd5214b3b75d9bb387bc`, byte-identical to the jar's `parts/stairsnormal.json` (6x11, 7 slices, 138 cells: `x` 102 / `/` 24 / `_` 12) — verified in the source after rustfmt **and inside the packed archive** |
| embedded large-stair part | 1,596 B, sha256 `ae9dc82a2450c587cad447355021b5985549d4d17702de6e38b7b31a2cfe3a2c`, byte-identical to the jar's `parts/stairsbig.json` (6x12, 7 slices, 180 cells: `x` 132 / `/` 36 / `_` 12, private `meta` key ignored) — verified in the source after rustfmt **and inside the packed archive** |
| embedded third-building part | 2,618 B, sha256 `93cd0a5bac82a1e54b838f70a2e5366db4d4e54f33f087271426050f26ef9974`, byte-identical to the jar's `parts/building1_3.json` (16x16, 6 slices, 1,206 cells: `#` 784 / `a` 186 / `W` 168 / `:` 30 / `@` 24 / `l` 6 / `L` 3 / `F` 2 / `1` 2 / `T` 1; escaped literal, brick rows begin `"#`) — verified in the source after rustfmt **and inside the packed archive**; its one new `W` → `minecraft:water` entry is copied verbatim from `palettes/common.json` |
| embedded second-centre part | 2,618 B, sha256 `3fdc948a024ae6100518f23c434137f27462b597a003f00cfd2e47ba9b99349e`, byte-identical to the jar's `parts/center00_2.json` (16x16, 6 slices, 771 cells: `#` 420 / `a` 126 / `Q` 107 / `S` 97 / `)` 12 / `1` 5 / `C` 2 / `4` 1 / `T` 1; escaped literal, brick rows begin `"#`) — verified in the source after rustfmt **and inside the packed archive**; its two new `palettes/common.json` entries `C` → chest and `4` → spawner are copied verbatim |
| embedded library part | 2,618 B, sha256 `e52c4556d19cf3eb5617161852cba5a0f632f7b5794a6acc3e888976d5fcc30b`, byte-identical to the jar's `parts/library10_2.json` (16x16, 6 slices, 680 cells: `#` 361 / `Q` 144 / `a` 75 / `S` 56 / `L` 30 / `-` 6 / `.` 6 / `T` 2; escaped literal, brick rows begin `"#`) — verified in the source after rustfmt **and inside the packed archive**; its two new `palettes/common.json` entries `.` → oak fence and `-` → oak pressure plate are copied verbatim onto the carried centre palette |
| embedded rail-dungeon part | 2,643 B, sha256 `1216e3346e3be52eb2b5f93b39816e9f0305c45101fb393614f198d95204225c`, byte-identical to the jar's `parts/rail_dungeon1.json` (16x16, 6 slices, 1,536 cells: `y` 1127 / `~` 382 / `!` 17 / `C` 4 / `1` 3 / `4` 3; raw literal with `refpalette: rails`) — verified in the source after rustfmt **and inside the packed archive**; its one new `!` weighted entry (diamond/gold/emerald/iron) is copied verbatim from `palettes/common.json` and resolves all 17 cells to `minecraft:iron_block` |
| embedded second rail-dungeon part | 2,643 B, sha256 `6b5081f14b4cbf30d5cc771c9bd3d5ed8ae82d8dad8b3bfa4190fb9b218bb2f5`, byte-identical to the jar's `parts/rail_dungeon2.json` (16x16, 6 slices, 1,536 cells: `y` 1148 / `~` 380 / `1` 3 / `!` 3 / `F` 2; raw literal with `refpalette: rails`) — verified in the source after rustfmt **and inside the packed archive**; it reuses the first dungeon's verbatim 19-entry palette (`!` → iron, `F` → cobweb) with **no new id or palette entry** |
| embedded rail end-cap part | 2,643 B, sha256 `6e302fa0ba450e26eae406b1f9219817a5e5bd41cd46fbc11e32c5587d3cd529`, byte-identical to the jar's `parts/rails_horizontal_end.json` (16x16, 6 slices, 942 cells: `y` 589 / `~` 314 / `e` 33 / `f` 3 / `g` 2 / `T` 1; raw literal with `refpalette: rails`) — verified in the source after rustfmt **and inside the packed archive**; all six chars are already carried by `RAILS_PALETTE_JSON`, so **no new id or palette entry** |

## Gates (fresh, against the delivered bytes)

| gate | command | result | log |
|---|---|---|---|
| data contract | `python3 tools/carry.py --check` | 6 verbatim tags, 2 loot tables shape-upgraded, 1 verbatim lang, blocked families absent | — |
| decoder red (slice 2) | `cargo test --offline` before the decoder | **FAIL**, 11 unresolved symbols | `evidence/the_lost_cities/decoder-test-red.log` |
| building red (slice 3) | before variant/building data | **FAIL**, 11 unresolved symbols | `building-test-red.log` |
| rail red (slice 4) | before rail data | **FAIL**, 3 unresolved symbols | `rails-test-red.log` |
| bend red (slice 5) | before bend data | **FAIL**, 2 unresolved | `bend-test-red.log` |
| split red (slice 6) | before split data | **FAIL**, 2 unresolved | `split-test-red.log` |
| descent red (slice 7) | before descent data | **FAIL**, 2 unresolved | `down2-test-red.log` |
| first-descent red (slice 8) | before down1 data | **FAIL**, 3 unresolved, exit 101 | `down1-test-red.log` |
| second-building red (slice 9) | before building1_2 data | **FAIL**, 3 unresolved, exit 101 | `building12-test-red.log` |
| city-centre red (slice 10) | before center00_1 data | **FAIL**, 3 unresolved, exit 101 | `center001-test-red.log` |
| shopping red (slice 11) | before shopping_open01_1 data | **FAIL**, 3 unresolved, exit 101 | `shopping1-test-red.log` |
| stair red (slice 12) | before stairsnormal data | **FAIL**, 3 unresolved `STAIRSNORMAL_PART_JSON`/`STAIRSNORMAL_PALETTE_JSON`, exit 101 | `stairsnormal-test-red.log` |
| large-stair red (slice 13) | before stairsbig data | **FAIL**, 2 unresolved `STAIRSBIG_PART_JSON`, exit 101 | `stairsbig-test-red.log` |
| third-building red (slice 14) | before building1_3 data | **FAIL**, 3 unresolved `BUILDING_1_3_PART_JSON`/`BUILDING_1_3_PALETTE_JSON`, exit 101 | `building13-test-red.log` |
| second-centre red (slice 15) | before center00_2 data | **FAIL**, 3 unresolved `CENTER00_2_PART_JSON`/`CENTER00_2_PALETTE_JSON`, exit 101 | `center002-test-red.log` |
| library red (slice 16) | before library10_2 data | **FAIL**, 3 unresolved `LIBRARY_10_2_PART_JSON`/`LIBRARY_10_2_PALETTE_JSON`, exit 101 | `library1002-test-red.log` |
| rail-dungeon red (slice 17) | before rail_dungeon1 data | **FAIL**, 3 unresolved `RAIL_DUNGEON1_PART_JSON`/`RAIL_DUNGEON1_PALETTE_JSON`, exit 101 | `raildungeon1-test-red.log` |
| second rail-dungeon red (slice 18) | before rail_dungeon2 data | **FAIL**, 3 unresolved `RAIL_DUNGEON2_PART_JSON`/`RAIL_DUNGEON2_PALETTE_JSON`, exit 101 | `raildungeon2-test-red.log` |
| rail end-cap red (slice 19) | before rails_horizontal_end data | **FAIL**, 2 unresolved `RAILS_HORIZONTAL_END_PART_JSON`, exit 101 | `rails-end-test-red.log` |
| unit tests | `cargo test --offline` | **43 passed, 0 failed** | `evidence/the_lost_cities/sdk-test.log` (regenerated after the final build) |
| lints | `cargo clippy --all-targets --offline -- -D warnings` | exit 0, 0 warnings | `sdk-clippy-028.log` |
| format | `rustfmt --edition 2024 --check src/{lib,runtime}.rs` | clean | `sdk-fmt-check-028.log` |
| SDK check (source) | `cargo horizon-mod check` | *132 API items; Made for 0a875da* | `sdk-check-028.log` |
| package check | `cargo horizon-mod check the_lost_cities.hmod` | *0.1.18, native-1/0a875da…, 51,306 bytes, sha 4708adf3…* | `sdk-package-check-028.log` |
| fits (pin) | `cargo horizon-mod fits … --build <full 0a875da sha>` | **`made for`**, exit 0 | `sdk-fits-028-0a875da.log` |
| fits (other build) | `cargo horizon-mod fits … --build <full 0b14f8a sha>` | **`fits`**, exit 0 — registrar + DimensionSettings exist in both builds; only the Rust decoder/embedded jar data are ours | `sdk-fits-028-0b14f8a.log` |
| package gate | `verify-sdk-package.py the_lost_cities` | **1/1**, incl. "package matches the tree" | — |
| skills gate | `check-skills.py --notes ports/the_lost_cities --strict` | PASS | — |
| recipe types / manifest deps | `check-recipe-types.py` / `check-manifest-deps.py` | ok / no SDK package | — |

README.md, horizon-mod.toml, Cargo.toml and `src/` were edited **before** the
final build, so the whole-archive-vs-tree gate is green; `sdk-test.log` was
regenerated for 0.1.18 after the build. The 2,643-byte `rail_dungeon1` part and
its one new `palettes/common.json` entry (`!` → weighted diamond/gold/emerald/
iron, resolving to `minecraft:iron_block`) were read from the jar before the
edit and verified byte-identically in the source after rustfmt and in the
packed archive (sha `1216e334…`). The 2,643-byte `rail_dungeon2` part (sha
`6b5081f1…`) and its resolved ids (`!` → iron, `F` → cobweb) were likewise read
from the jar before the edit, embedded byte-identically and checked after
rustfmt and inside the packed archive; it reuses the first dungeon's verbatim
palette, so no new id or entry was introduced. The 2,643-byte
`rails_horizontal_end` part (sha `6e302fa0…`) was likewise read from the jar
before the edit, embedded byte-identically and checked after rustfmt and inside
the packed archive; all six of its chars are already carried by
`RAILS_PALETTE_JSON`, so it also introduces no new id or entry. No type or
variant named `Block`/`Item` was introduced (the `fits` false-fail trap). No new
API item, manifest key or decoder form was needed.

Both `fits` runs used `HORIZON_SDK_BASE=.sdk-mirror` — a local mirror of the in-tree signed
`sdk-<sha>/` assets (`sdk.json`, `surface.json`, `SHA256SUMS`, `.sig`,
`api-source.tar.xz`) — and the **full 40-hex build sha**. The two
`made for`/`fits` lines were computed from those real cached surfaces, not
guessed.

Every Rust step went through `ports/tools/build-slot.sh` with
`CARGO_INCREMENTAL=0`, no `--target-dir`, no `BUILD_LOCAL=1`: test/lint/fmt/
check/build/package steps forwarded to `AdLf-PC` (the test RED log shows
`forwarding test … to the remote runner` and paths under `/home/deepseek/…`).
The launcher native build's first forwarded attempt failed on the far box
(`libxkbcommon-x11.so.0` absent), so `BUILD_LOCAL=1` was set **for that step
only** and it was re-run locally; the e2e legs are local by design (`--offscreen`
has no renderer on the far box). A forwarded `rustfmt` apply does
not sync source edits back, so formatting was applied locally and then verified
green through the slot. Every leg used `E2E_TMP` from `launcher-game.sh env`
(`/home/adlf/.cache/horizon-e2e/scratch`), never `/tmp`.
No lease was backgrounded and no turn ended with a shell outstanding.

## In-game legs (`ports/_shared/evidence/the_lost_cities-e2e/`)

| leg | result | log |
|---|---|---|
| modded game | **PASS** + `EXIT:0` — dimension answers; a written stone block reads back; **136 smooth-stone slabs** at y=90, **4 bookshelves** at y=72, **48 east_west rails** at y=54..59, **10 north_south rails** at y=36..41, **1 north_east rail** at y=18..23, **21 ascending_east rails** at y=4..21, **30 iron bars** at y=100..111, **3 quartz blocks** at y=116..121, **12 quartz stairs** at y=128..133, **18 cobblestone walls** at y=140..145, **28 stone stairs** at y=152..158, **42 stone stairs** at y=164..170 (the large stair's own band) **168 water blocks** at y=176..181, **2 chests** at y=188..193 (the `execute if block 3 189 11 minecraft:chest[facing=north]` read-back answered `Test passed`) and **6 oak fences + 6 oak pressure plates** at y=200..205 (property-exact read-backs at (5,201,7)/(5,202,7) answered `Test passed`; plate fill runs before fence fill because each plate sits on a fence) and **17 iron blocks** at y=212..217 (property-exact read-back at (10,214,6) answered `Test passed`) and **exactly 3 iron blocks** at y=224..229 (property-exact read-back at (9,226,8) answered `Test passed`, the lower band already cleared by the earlier leg) and **exactly 36 east-west rails** at y=236..241 (33 in-chunk `e` cells plus 3 from the inclusive 17×17 window's x=16 rim; property-exact read-back at (0,237,5) answered `Test passed`), all 0 on the vanilla-noise control; `#lostcities:notbreakable` → `Test passed`; both loot tables → items; unknown tag/loot ids refused | `e2e.log` |
| control (same Nightly, no mod) | **FAIL** + `EXIT:1` — `Unknown dimension 'lostcities:lostcity'` | `e2e-control.log` |
| `SELFTEST=1` (invert dimension assertion) | **FAIL** + `EXIT:1` — `SELFTEST: the dimension answered` | `e2e-selftest.log` |
| `TAG_SELFTEST=1` | **FAIL** + `EXIT:1` — `TAG_SELFTEST: the tag matched` | `e2e-tag-selftest.log` |
| `CITY_SELFTEST=1` | **FAIL** + `EXIT:1` — `CITY_SELFTEST: the generator authored 136 slab(s); the inverted assertion expected 0` | `e2e-city-selftest.log` |
| `BUILDING_SELFTEST=1` | **FAIL** + `EXIT:1` — `BUILDING_SELFTEST: the building authored 4 bookshelf(s); the inverted assertion expected 0` | `e2e-building-selftest.log` |
| `RAILS_SELFTEST=1` | **FAIL** + `EXIT:1` — `RAILS_SELFTEST: the rail part authored 48 rail(s); the inverted assertion expected 0` | `e2e-rails-selftest.log` |
| `BEND_SELFTEST=1` | **FAIL** + `EXIT:1` — `BEND_SELFTEST: the bend authored 10 north_south rail(s); the inverted assertion expected 0` | `e2e-bend-selftest.log` |
| `SPLIT_SELFTEST=1` | **FAIL** + `EXIT:1` — `SPLIT_SELFTEST: the split authored 1 north_east rail(s); the inverted assertion expected 0` | `e2e-split-selftest.log` |
| `DOWN2_SELFTEST=1` | **FAIL** + `EXIT:1` — `DOWN2_SELFTEST: the descent authored 21 ascending_east rail(s); the inverted assertion expected 0` | `e2e-down2-selftest.log` |
| `DOWN1_SELFTEST=1` | **FAIL** + `EXIT:1` — `DOWN1_SELFTEST: the first descent authored 30 iron-bar(s); the inverted assertion expected 0` | `e2e-down1-selftest.log` |
| `BUILDING12_SELFTEST=1` | **FAIL** + `EXIT:1` — `BUILDING12_SELFTEST: the part authored 3 quartz block(s); the inverted assertion expected 0` | `e2e-building12-selftest.log` |
| `CENTER001_SELFTEST=1` | **FAIL** + `EXIT:1` — `CENTER001_SELFTEST: the part authored 12 quartz staircase(s); the inverted assertion expected 0` | `e2e-center001-selftest.log` |
| `SHOPPING1_SELFTEST=1` | **FAIL** + `EXIT:1` — `SHOPPING1_SELFTEST: the part authored 18 cobblestone wall(s); the inverted assertion expected 0` | `e2e-shopping1-selftest.log` |
| `STAIRS_SELFTEST=1` | **FAIL** + `EXIT:1` — `STAIRS_SELFTEST: the part authored 28 stone stair(s); the inverted assertion expected 0` | `e2e-stairs-selftest.log` |
| `STAIRSBIG_SELFTEST=1` | **FAIL** + `EXIT:1` — `STAIRSBIG_SELFTEST: the part authored 42 stone stair(s); the inverted assertion expected 0` | `e2e-stairsbig-selftest.log` |
| `BUILDING13_SELFTEST=1` | **FAIL** + `EXIT:1` — `BUILDING13_SELFTEST: the part authored 168 water block(s); the inverted assertion expected 0` | `e2e-building13-selftest.log` |
| `CENTER002_SELFTEST=1` | **FAIL** + `EXIT:1` — `CENTER002_SELFTEST: the part authored 2 chest block(s); the inverted assertion expected 0` | `e2e-center002-selftest.log` |
| `LIBRARY1002_SELFTEST=1` | **FAIL** + `EXIT:1` — `LIBRARY1002_SELFTEST: the part authored 6 oak-fence block(s); the inverted assertion expected 0` | `e2e-library1002-selftest.log` |
| `RAILDUNGEON1_SELFTEST=1` | **FAIL** + `EXIT:1` — `RAILDUNGEON1_SELFTEST: the part authored 17 iron block(s); the inverted assertion expected 0` | `e2e-raildungeon1-selftest.log` |
| `RAILDUNGEON2_SELFTEST=1` | **FAIL** + `EXIT:1` — `RAILDUNGEON2_SELFTEST: the part authored 3 iron block(s); the inverted assertion expected 0` | `e2e-raildungeon2-selftest.log` |
| `RAILEND_SELFTEST=1` | **FAIL** + `EXIT:1` — `RAILEND_SELFTEST: the part authored 36 rail(s); the inverted assertion expected 0` | `e2e-railend-selftest.log` |
| log gate | `check-e2e-logs.py the_lost_cities-e2e --strict` → `logs=22 ok=22 missing-exit=0 bad-exit=0` | — |

The in-game counts are 28 stone stairs at y=152..158 (the part's 24 `/` cells
plus neighbours inside the inclusive 17×17 fill window), 42 stone stairs at
y=164..170 (the large stair's 36 `/` cells plus neighbours in its own window)
and 168 water blocks at y=176..181 (the third building's 168 `W` cells, exactly
— the water body is enclosed by glass/stonebrick so it does not spread into the
fill window), and 2 chests at y=188..193 (the two `C` cells, with a property-exact
`chest[facing=north]` read-back in the same level), and 6 oak fences + 6 oak
pressure plates at y=200..205 (the part's `.`/`-` cells, exactly — first run
reported 0 plates because the fence fill removed their support; with the fills
ordered plate-first both report 6). The unit tests pin the per-part counts at
24, 36, 168, 2, 6 and 6; the disjoint height bands keep the stair assertions
independent even though both parts write the same id, and water, chests, oak
fences and oak pressure plates are ids no other decoded part writes. The
rail dungeon's 17 iron blocks at y=212..217 are an id no other decoded part
writes (iron bars appear only as a `damaged` field, which the decoder does not
place). The second rail dungeon's three iron blocks at y=224..229 share that id
with the first dungeon but occupy a disjoint height band; the e2e asserts the
exact count 3, not merely a positive count. The rail end cap's 33 in-chunk
`e` cells at y=236..241 read 36 in game: the inclusive 17×17 fill window also
takes the x=16 cell from chunk (1,0) in each of the part's three rail rows,
exactly as the lower horizontal line reads 48 = 45 + 3 and the bend 10 = 9 + 1;
the unit test pins 33 and the e2e pins the measured 36. All 22 legs were run
sequentially, each blocking in its own slot until `EXIT:`; no harness defect
surfaced this slice and no leg needed a re-run.

The launcher native build printed **`Compiling the_lost_cities (1 of 1)`**, new
tag `native-0a875da-b173507aacf6`, and `native-mods.json` named version
**0.1.18** before any leg ran. The previous 0.1.17 tag
`native-0a875da-8e4c8de453c1` also carried the mod after the build, so it was
moved aside to `~/.cache/horizon-e2e/retired-tags/`; exactly one active `native-*`
tag then carried the_lost_cities (0.1.18), so no leg could test a stale export.
No exit 10 from a concurrent native build occurred. As in 0.1.16/0.1.17 the
launcher step alone ran with `BUILD_LOCAL=1` because the far box lacks
`libxkbcommon-x11.so.0`; every Rust step forwarded normally.

## Re-run

```sh
# ladder (compile steps forward to AdLf-PC automatically, no BUILD_LOCAL=1)
ports/tools/build-slot.sh run --job TLC --port ports/the_lost_cities --step test  -- cargo test --offline
ports/tools/build-slot.sh run --job TLC --port ports/the_lost_cities --step lint -- cargo clippy --all-targets --offline -- -D warnings
ports/tools/build-slot.sh run --job TLC --port ports/the_lost_cities --step fmt  -- rustfmt --edition 2024 --check src/lib.rs src/runtime.rs
ports/tools/build-slot.sh run --job TLC --port ports/the_lost_cities --step check -- cargo horizon-mod check
ports/tools/build-slot.sh run --job TLC --port ports/the_lost_cities --step build -- bash -c 'rm -f Cargo.lock && cargo horizon-mod build && cp target/horizon-mod/the_lost_cities.hmod the_lost_cities.hmod'
ports/tools/build-slot.sh run --job TLC --port ports/the_lost_cities --step check -- cargo horizon-mod check $PWD/ports/the_lost_cities/the_lost_cities.hmod
ports/tools/build-slot.sh run --job TLC --port ports/the_lost_cities --step check -- cargo horizon-mod fits $PWD/ports/the_lost_cities/the_lost_cities.hmod --build 0a875da
ports/tools/build-slot.sh run --job TLC --port ports/the_lost_cities --step check -- cargo horizon-mod fits $PWD/ports/the_lost_cities/the_lost_cities.hmod --build 0b14f8a
BUILD_LOCAL=1 ports/tools/build-slot.sh run --job TLC --port ports/the_lost_cities --step build -- ports/tools/launcher-game.sh build the_lost_cities
# legs (block in the lease until EXIT:); E2E_TMP from launcher-game.sh env
eval "$(ports/tools/launcher-game.sh env the_lost_cities)"
ports/tools/build-slot.sh run --job TLC-e2e --port ports/the_lost_cities --step e2e -- bash ports/the_lost_cities/tools/e2e.sh
# plus GAME=$CONTROL_GAME, SELFTEST=1, TAG_SELFTEST=1, CITY_SELFTEST=1,
#       BUILDING_SELFTEST=1, RAILS_SELFTEST=1, BEND_SELFTEST=1, SPLIT_SELFTEST=1,
#       DOWN2_SELFTEST=1, DOWN1_SELFTEST=1, BUILDING12_SELFTEST=1,
#       CENTER001_SELFTEST=1, SHOPPING1_SELFTEST=1, STAIRS_SELFTEST=1,
#       STAIRSBIG_SELFTEST=1, BUILDING13_SELFTEST=1, CENTER002_SELFTEST=1,
#       LIBRARY1002_SELFTEST=1, RAILDUNGEON1_SELFTEST=1, RAILDUNGEON2_SELFTEST=1,
#       RAILEND_SELFTEST=1
```
