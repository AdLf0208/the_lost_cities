# The Lost Cities — validation record (v0.1.11)

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
`minecraft:stone_stairs[facing=west,half=bottom,shape=straight]` entry.
Cargo and the manifest agree at 0.1.11.
The orchestrator publishes; this port does not touch registry/PR/ledger.

## The deliverable

| | |
|---|---|
| package | `ports/the_lost_cities/the_lost_cities.hmod` |
| size | **40,042 B** |
| SHA-256 | `0acc5d54dba2f6fb42bd3c767e761a66a715d329e9a617bf4d30edad98a0edfe` |
| version | 0.1.11 (manifest + Cargo.toml) |
| entries | 17 (6 tags + 2 loot tables + 1 lang + manifest/Cargo/src/docs/uses.json) |
| packed Cargo.lock | none |
| runtime | `src/runtime.rs` byte-identical in tree and package (`98f5ce583915`) |
| embedded bend part | 2,643 B, sha256 `9969f02e8c9b566de15616eecb5598dbea4a4ff5528c5b8792fd4143497e3232`, byte-identical to the jar's `data/lostcities/lostcities/parts/rails_bend.json` |
| embedded split part | 2,643 B, sha256 `b0c8bd61f1f3b3641b9bf20a57087429123b7de9353dd1fd3129e40717777eb0`, byte-identical to the jar's `parts/rails_3split.json` |
| embedded descent part | 7,779 B, sha256 `f2f6ebabe2ae71dc6c094496de0b90ebda97415ecc619b7c7b8c78161e65be89`, byte-identical to the jar's `parts/rails_down2.json` (18 slices) |
| embedded first descent part | 5,211 B, sha256 `822a38fa5420ab545f78248a216a937e0f1f8dc78d6a258ffbae45da8340411e`, byte-identical to the jar's `parts/rails_down1.json` (12 slices, 911 non-space cells) |
| embedded second building part | 2,618 B, sha256 `b2130f36f887c90c4b1602f04e3d70bcff0835ac5b176b070ca78e94ff264f74`, byte-identical to the jar's `parts/building1_2.json` (6 slices, 998 non-space cells) |
| embedded city-centre part | 2,618 B, sha256 `5ade2e7903e012f38fb9e9559ccda6aa51ef4c9856d2d365b891c118acebdc54`, byte-identical to the jar's `parts/center00_1.json` (6 slices, 680 non-space cells, escaped literal) |
| embedded shopping part | 2,697 B, sha256 `c16f5ff65b9a32b168b8c5bb5edfbdaa970a994bc1434f4a35679b30b4a4ed5d`, byte-identical to the jar's `parts/shopping_open01_1.json` (6 slices, 12 `w` cells, raw literal with a private `meta` key) |
| embedded stair part | 1,484 B, sha256 `c6d17e44238f625378d2f8e474aba1a1ea48c755200ecd5214b3b75d9bb387bc`, byte-identical to the jar's `parts/stairsnormal.json` (6x11, 7 slices, 138 cells: `x` 102 / `/` 24 / `_` 12) — verified in the source after rustfmt **and inside the packed archive** |

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
| unit tests | `cargo test --offline` | **29 passed, 0 failed** | `evidence/the_lost_cities/sdk-test.log` |
| lints | `cargo clippy --all-targets --offline -- -D warnings` | exit 0, 0 warnings | `sdk-clippy-021.log` |
| format | `rustfmt --edition 2024 --check src/{lib,runtime}.rs` | clean | `sdk-fmt-021.log` |
| SDK check (source) | `cargo horizon-mod check` | *132 API items; Made for 0a875da* | `sdk-check-021.log` |
| package check | `cargo horizon-mod check the_lost_cities.hmod` | *0.1.11, native-1/0a875da…, 40,042 bytes, sha 0acc5d54…* | `sdk-package-check-021.log` |
| fits (pin) | `cargo horizon-mod fits … --build 0a875da` | **`made for`**, exit 0 | `sdk-fits-021-0a875da.log` |
| fits (other build) | `cargo horizon-mod fits … --build 0b14f8a` | **`fits`**, exit 0 — registrar + DimensionSettings exist in both builds; only the Rust decoder/embedded jar data are ours | `sdk-fits-021-0b14f8a.log` |
| package gate | `verify-sdk-package.py the_lost_cities` | **1/1**, incl. "package matches the tree" | — |
| skills gate | `check-skills.py --notes ports/the_lost_cities --strict` | PASS | — |
| recipe types / manifest deps | `check-recipe-types.py` / `check-manifest-deps.py` | ok / no SDK package | — |

README.md, horizon-mod.toml, Cargo.toml and `src/` were edited **before** the
final build, so the whole-archive-vs-tree gate is green; `sdk-test.log` was
regenerated for 0.1.11 after the build (the 0.1.10 log retired to
`retired-0.1.10/`). The 1,484-byte stair part and its three
`palettes/default.json` entries were read from the jar before the edit and
verified byte-identically in the source after rustfmt and in the packed archive
(sha `c6d17e44…`). No type or variant named `Block`/`Item` was introduced (the
`fits` false-fail trap). No new API item, manifest key or decoder form was
needed.

Every Rust step went through `ports/tools/build-slot.sh` with
`CARGO_INCREMENTAL=0`, no `--target-dir`, no `BUILD_LOCAL=1`: test/lint/fmt/
check/build/package steps forwarded to `AdLf-PC` (the test RED log shows
`forwarding test … to the remote runner` and paths under `/home/deepseek/…`).
Only the launcher native build and the e2e legs were local by design
(`--offscreen` has no renderer on the far box). Every leg used `E2E_TMP` from
`launcher-game.sh env` (`/home/adlf/.cache/horizon-e2e/scratch`), never `/tmp`.
No lease was backgrounded and no turn ended with a shell outstanding.

## In-game legs (`ports/_shared/evidence/the_lost_cities-e2e/`)

| leg | result | log |
|---|---|---|
| modded game | **PASS** + `EXIT:0` — dimension answers; a written stone block reads back; **136 smooth-stone slabs** at y=90, **4 bookshelves** at y=72, **48 east_west rails** at y=54..59, **10 north_south rails** at y=36..41, **1 north_east rail** at y=18..23, **21 ascending_east rails** at y=4..21, **30 iron bars** at y=100..111, **3 quartz blocks** at y=116..121, **12 quartz stairs** at y=128..133, **18 cobblestone walls** at y=140..145 and **28 stone stairs** at y=152..158, all 0 on the vanilla-noise Overworld; `#lostcities:notbreakable` → `Test passed`; both loot tables → items; unknown tag/loot ids refused | `e2e.log` |
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
| log gate | `check-e2e-logs.py the_lost_cities-e2e --strict` → `logs=15 ok=15 missing-exit=0 bad-exit=0` | — |

The in-game count is 28 stone stairs: the part's 24 `/` cells plus adjacent
chunks inside the inclusive 17×17 fill window. The unit test pins the per-part
count at 24.

The launcher native build printed **`Compiling the_lost_cities (1 of 1)`**, tag
`native-0a875da-c8ac0678fbc4`, and `native-mods.json` named version **0.1.11**
before any leg ran. No stale the_lost_cities `native-*` tag remained before the
build (`versions`, `stale-tags` and `retired` were audited), so no leg could test
a stale export. No exit 10 from a concurrent native build occurred.

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
BUILD_LOCAL=1 ports/tools/build-slot.sh run --job TLC-e2e --port ports/the_lost_cities --step e2e -- bash ports/the_lost_cities/tools/e2e.sh
# plus GAME=$CONTROL_GAME, SELFTEST=1, TAG_SELFTEST=1, CITY_SELFTEST=1,
#       BUILDING_SELFTEST=1, RAILS_SELFTEST=1, BEND_SELFTEST=1, SPLIT_SELFTEST=1,
#       DOWN2_SELFTEST=1, DOWN1_SELFTEST=1, BUILDING12_SELFTEST=1,
#       CENTER001_SELFTEST=1, SHOPPING1_SELFTEST=1, STAIRS_SELFTEST=1
```
