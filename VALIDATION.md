# The Lost Cities — validation record (v0.1.6)

Pin **`0a875da91dbb0fdcb7ba8c42c11341aa229af51a`**, mixins
`e59da9094f0c2fdda6c627fe9eff22dec6a906f0`, game 26.3 (Nightly
`nightly-2026-11-20-0a875da`), `api = "0.2"`.

**Version 0.1.6** — an update of our first publication (0.1.0, PR #1861); 0.1.1
added the first deterministic city chunk generator, 0.1.2 the six-layer building
part, 0.1.3 the six-layer horizontal rail line, 0.1.4 the six-layer rail
bend from `parts/rails_bend.json`, 0.1.5 the three-way rail split from
`parts/rails_3split.json`, and 0.1.6 the eighteen-layer rail descent from
`parts/rails_down2.json`, all with ids carried verbatim from `palettes/rails.json`.
Cargo and the manifest agree at 0.1.6. The orchestrator
publishes; this port does not touch registry/PR/ledger.

## The deliverable

| | |
|---|---|
| package | `ports/the_lost_cities/the_lost_cities.hmod` |
| size | **31,914 B** |
| SHA-256 | `21b6f29c68148e6834cdc6ba01b5f1a329f83003f67171371ec9948b466ac6c9` |
| version | 0.1.6 (manifest + Cargo.toml) |
| entries | 17 (6 tags + 2 loot tables + 1 lang + manifest/Cargo/src/docs/uses.json) |
| packed Cargo.lock | none |
| runtime | `src/runtime.rs` byte-identical in tree and package (`23dc15a3e92b`) |
| embedded bend part | 2,643 B, sha256 `9969f02e8c9b566de15616eecb5598dbea4a4ff5528c5b8792fd4143497e3232`, byte-identical to the jar's `data/lostcities/lostcities/parts/rails_bend.json` |
| embedded split part | 2,643 B, sha256 `b0c8bd61f1f3b3641b9bf20a57087429123b7de9353dd1fd3129e40717777eb0`, byte-identical to the jar's `data/lostcities/lostcities/parts/rails_3split.json` |
| embedded descent part | 7,779 B, sha256 `f2f6ebabe2ae71dc6c094496de0b90ebda97415ecc619b7c7b8c78161e65be89`, byte-identical to the jar's `data/lostcities/lostcities/parts/rails_down2.json` (18 slices) |

## Gates (fresh, against the delivered bytes)

| gate | command | result | log |
|---|---|---|---|
| data contract | `python3 tools/carry.py --check` | 6 verbatim tags, 2 loot tables shape-upgraded, 1 verbatim lang, blocked families absent | `evidence/the_lost_cities/carry-contract-014.log` |
| decoder red (slice 2) | `cargo test --offline` before the decoder | **FAIL**, 11 unresolved symbols | `evidence/the_lost_cities/decoder-test-red.log` |
| building red (slice 3) | `cargo test --offline` before variant/building data | **FAIL**, 11 unresolved symbols | `evidence/the_lost_cities/building-test-red.log` |
| rail red (slice 4) | `cargo test --offline` before rail data | **FAIL**, 3 unresolved symbols | `evidence/the_lost_cities/rails-test-red.log` |
| bend red (slice 5) | `cargo test --offline` before bend data | **FAIL**, 2 unresolved `RAILS_BEND_PART_JSON` symbols, build-slot exit 101 | `evidence/the_lost_cities/bend-test-red.log` |
| split red (slice 6) | `cargo test --offline` before split data | **FAIL**, 2 unresolved `RAILS_3SPLIT_PART_JSON` symbols, build-slot exit 101 | `evidence/the_lost_cities/split-test-red.log` |
| descent red (slice 7) | `cargo test --offline` before descent data | **FAIL**, 2 unresolved `RAILS_DOWN2_PART_JSON` symbols, build-slot exit 101 | `evidence/the_lost_cities/down2-test-red.log` |
| unit tests | `cargo test --offline` | **19 passed, 0 failed** | `evidence/the_lost_cities/sdk-test.log` |
| lints | `cargo clippy --all-targets --offline -- -D warnings` | exit 0, 0 warnings | `evidence/the_lost_cities/sdk-clippy.log` |
| format | `rustfmt --edition 2024 --check src/{lib,runtime}.rs` | clean | `evidence/the_lost_cities/sdk-fmt.log` |
| SDK check (source) | `cargo horizon-mod check` | *132 API items; Made for 0a875da* | `evidence/the_lost_cities/sdk-check.log` |
| package check | `cargo horizon-mod check the_lost_cities.hmod` | *0.1.6, native-1/0a875da…, 31,914 bytes, sha 21b6f29c…* | `evidence/the_lost_cities/sdk-package-check.log` |
| fits (pin) | `cargo horizon-mod fits … --build 0a875da` | **`made for`**, exit 0 | `evidence/the_lost_cities/sdk-fits.log` |
| fits (other build) | `cargo horizon-mod fits … --build 0b14f8a` | **`fits`**, exit 0 — registrar + DimensionSettings exist in both builds; only the Rust decoder/embedded jar data are ours | `evidence/the_lost_cities/sdk-fits-other.log` |
| package gate | `verify-sdk-package.py the_lost_cities` | **1/1**, incl. "package matches the tree" | — |
| skills gate | `check-skills.py --notes ports/the_lost_cities --strict` | PASS | — |

README.md, horizon-mod.toml, Cargo.toml and `src/` were edited **before** the
final build, so the whole-archive-vs-tree gate is green; `sdk-test.log` was
regenerated for 0.1.6 after the build. No type or variant named `Block`/`Item`
was introduced (the R9-era `fits` false-fail trap). The e2e harness's loot regex
was widened to accept 26.3's `Dropped 1 [Item]` single-drop wording as well as
`Dropped N items` (it still refuses `Dropped 0 items`); `tools/e2e.sh` is not a
packed file, so no rebuild was needed.

Every Rust step went through `ports/tools/build-slot.sh` with
`CARGO_INCREMENTAL=0`, no `--target-dir`, no `BUILD_LOCAL=1`: compile/test/
check/lint/fmt/build and package steps printed `build-slot: forwarding … to the
remote runner` and ran on `AdLf-PC`. Only the launcher native build and the e2e
legs were local by design (`--offscreen` has no renderer on the far box). No
lease was backgrounded and no turn ended with a shell outstanding.

## In-game legs (`ports/_shared/evidence/the_lost_cities-e2e/`)

| leg | result | log |
|---|---|---|
| modded game | **PASS** + `EXIT:0` — dimension answers; a written stone block reads back; **136 smooth-stone slabs** at y=90, **4 bookshelves** at y=72, **48 east_west rails** at y=54..59, **10 north_south rails** at y=36..41, **1 north_east rail** at y=18..23 and **21 ascending_east rails** at y=4..21, all 0 on the vanilla-noise Overworld; `#lostcities:notbreakable` → `Test passed`; both loot tables → items; unknown tag/loot ids refused | `e2e.log` |
| control (same Nightly, no mod) | **FAIL** + `EXIT:1` — `Unknown dimension 'lostcities:lostcity'` | `e2e-control.log` |
| `SELFTEST=1` (invert dimension assertion) | **FAIL** + `EXIT:1` — `SELFTEST: the dimension answered` | `e2e-selftest.log` |
| `TAG_SELFTEST=1` (invert tag assertion) | **FAIL** + `EXIT:1` — `TAG_SELFTEST: the tag matched` | `e2e-tag-selftest.log` |
| `CITY_SELFTEST=1` (invert slab count) | **FAIL** + `EXIT:1` — `CITY_SELFTEST: the generator authored 136 slab(s); the inverted assertion expected 0` | `e2e-city-selftest.log` |
| `BUILDING_SELFTEST=1` (invert bookshelf count) | **FAIL** + `EXIT:1` — `BUILDING_SELFTEST: the building authored 4 bookshelf(s); the inverted assertion expected 0` | `e2e-building-selftest.log` |
| `RAILS_SELFTEST=1` (invert east_west rail count) | **FAIL** + `EXIT:1` — `RAILS_SELFTEST: the rail part authored 48 rail(s); the inverted assertion expected 0` | `e2e-rails-selftest.log` |
| `BEND_SELFTEST=1` (invert north_south rail count) | **FAIL** + `EXIT:1` — `BEND_SELFTEST: the bend authored 10 north_south rail(s); the inverted assertion expected 0` | `e2e-bend-selftest.log` |
| `SPLIT_SELFTEST=1` (invert north_east rail count) | **FAIL** + `EXIT:1` — `SPLIT_SELFTEST: the split authored 1 north_east rail(s); the inverted assertion expected 0` | `e2e-split-selftest.log` |
| `DOWN2_SELFTEST=1` (invert ascending_east rail count) | **FAIL** + `EXIT:1` — `DOWN2_SELFTEST: the descent authored 21 ascending_east rail(s); the inverted assertion expected 0` | `e2e-down2-selftest.log` |
| log gate | `check-e2e-logs.py the_lost_cities-e2e --strict` → `logs=10 ok=10 missing-exit=0 bad-exit=0` | — |

The 10 north_south rails are the bend's 9 in-chunk `k` cells plus 1 from the
neighbouring chunk's row at z=16 (the fill window is inclusive 17×17) — the same
boundary effect that made the slab count 136 and the east_west rail count 48.
The unit test pins the per-part count at 9; the in-game count is the whole filled
window.

The launcher native build printed **`Compiling the_lost_cities (1 of 1)`**, tag
`native-0a875da-0c2873eab4cb`, and `native-mods.json` named version **0.1.6**
before any leg ran. The previous the_lost_cities tag
(`native-0a875da-ff899e672547`, 0.1.5) was removed before the build so no leg
could test a stale export. No exit 10 from a concurrent native build occurred.

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
# legs (block in the lease until EXIT:)
eval "$(ports/tools/launcher-game.sh env the_lost_cities)"
BUILD_LOCAL=1 ports/tools/build-slot.sh run --job TLC-e2e --port ports/the_lost_cities --step e2e -- bash ports/the_lost_cities/tools/e2e.sh
# plus GAME=$CONTROL_GAME, SELFTEST=1, TAG_SELFTEST=1, CITY_SELFTEST=1,
#       BUILDING_SELFTEST=1, RAILS_SELFTEST=1, BEND_SELFTEST=1, SPLIT_SELFTEST=1,
#       DOWN2_SELFTEST=1
```
