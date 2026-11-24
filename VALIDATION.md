# The Lost Cities — validation record (v0.1.3)

Pin **`0a875da91dbb0fdcb7ba8c42c11341aa229af51a`**, mixins
`e59da9094f0c2fdda6c627fe9eff22dec6a906f0`, game 26.3 (Nightly
`nightly-2026-11-20-0a875da`), `api = "0.2"`.

**Version 0.1.3** — an update of our first publication (0.1.0, PR #1861); 0.1.1
added the first deterministic city chunk generator, 0.1.2 the six-layer building
part, and 0.1.3 the six-layer horizontal rail line from `parts/rails_horizontal.json`
with ids carried verbatim from `palettes/rails.json`. Cargo and the manifest
agree at 0.1.3. The orchestrator publishes; this port does not touch
registry/PR/ledger.

## The deliverable

| | |
|---|---|
| package | `ports/the_lost_cities/the_lost_cities.hmod` |
| size | **27,133 B** |
| SHA-256 | `857e338962757c3ecc4b209d90c5b86e3819a7a0c62b783c7465befa6daf9e63` |
| version | 0.1.3 (manifest + Cargo.toml) |
| entries | 17 (6 tags + 2 loot tables + 1 lang + manifest/Cargo/src/docs/uses.json) |
| packed Cargo.lock | none |
| runtime | `src/runtime.rs` byte-identical in tree and package (`c36fa6e71718`) |

## Gates (fresh, against the delivered bytes)

| gate | command | result | log |
|---|---|---|---|
| data contract | `python3 tools/carry.py --check` | 6 verbatim tags, 2 loot tables shape-upgraded, 1 verbatim lang, blocked families absent | `evidence/the_lost_cities/carry-contract-013.log` |
| decoder red (slice 2) | `cargo test --offline` before the decoder | **FAIL**, 11 unresolved symbols | `evidence/the_lost_cities/decoder-test-red.log` |
| building red (slice 3) | `cargo test --offline` before variant/building data | **FAIL**, 11 unresolved symbols | `evidence/the_lost_cities/building-test-red.log` |
| rail red (slice 4) | `cargo test --offline` before rail data | **FAIL**, 3 unresolved symbols (`RAILS_PART_JSON` ×2, `RAILS_PALETTE_JSON`), build-slot exit 101 | `evidence/the_lost_cities/rails-test-red.log` |
| unit tests | `cargo test --offline` | **13 passed, 0 failed** | `evidence/the_lost_cities/sdk-test.log` |
| lints | `cargo clippy --all-targets --offline -- -D warnings` | exit 0, 0 warnings | `evidence/the_lost_cities/sdk-clippy.log` |
| format | `rustfmt --edition 2024 --check src/{lib,runtime}.rs` | clean | `evidence/the_lost_cities/sdk-fmt.log` |
| SDK check (source) | `cargo horizon-mod check` | *132 API items; Made for 0a875da* | `evidence/the_lost_cities/sdk-check.log` |
| package check | `cargo horizon-mod check the_lost_cities.hmod` | *0.1.3, native-1/0a875da…, 27,133 bytes, sha 857e3389…* | `evidence/the_lost_cities/sdk-package-check.log` |
| fits (pin) | `cargo horizon-mod fits … --build 0a875da` | **`made for`**, exit 0 | `evidence/the_lost_cities/sdk-fits.log` |
| fits (other build) | `cargo horizon-mod fits … --build 0b14f8a` | **`fits`**, exit 0 — registrar + DimensionSettings exist in both builds; only the Rust decoder/embedded jar data are ours | `evidence/the_lost_cities/sdk-fits-other.log` |
| package gate | `verify-sdk-package.py the_lost_cities` | **1/1**, incl. "package matches the tree" | — |
| skills gate | `check-skills.py --notes ports/the_lost_cities --strict` | PASS | — |

README.md, horizon-mod.toml, Cargo.toml and `src/` were edited **before** the
final build, so the whole-archive-vs-tree gate is green; `sdk-test.log` was
regenerated for 0.1.3 after the build (0.1.2 log retired to
`evidence/the_lost_cities/retired-0.1.2/`). No type or variant named
`Block`/`Item` was introduced (the R9-era `fits` false-fail trap).

Every Rust step went through `ports/tools/build-slot.sh` with
`CARGO_INCREMENTAL=0`, no `--target-dir`. Compile/test/check/lint/fmt/build and
package steps forwarded to `AdLf-PC` (no `BUILD_LOCAL=1`); only the launcher
native build and the e2e legs were local (`BUILD_LOCAL=1`). No lease was
backgrounded and no turn ended with a shell outstanding.

## In-game legs (`ports/_shared/evidence/the_lost_cities-e2e/`)

| leg | result | log |
|---|---|---|
| modded game | **PASS** + `EXIT:0` — dimension answers; a written stone block reads back; **136 smooth-stone slabs** at y=90, **4 bookshelves** at y=72 and **48 rails** at y=54..59, all 0 on the vanilla-noise Overworld; `#lostcities:notbreakable` → `Test passed`; both loot tables → items; unknown tag/loot ids refused | `e2e.log` |
| control (same Nightly, no mod) | **FAIL** + `EXIT:1` — `Unknown dimension 'lostcities:lostcity'` | `e2e-control.log` |
| `SELFTEST=1` (invert dimension assertion) | **FAIL** + `EXIT:1` — `SELFTEST: the dimension answered` | `e2e-selftest.log` |
| `TAG_SELFTEST=1` (invert tag assertion) | **FAIL** + `EXIT:1` — `TAG_SELFTEST: the tag matched` | `e2e-tag-selftest.log` |
| `CITY_SELFTEST=1` (invert slab count) | **FAIL** + `EXIT:1` — `CITY_SELFTEST: the generator authored 136 slab(s); the inverted assertion expected 0` | `e2e-city-selftest.log` |
| `BUILDING_SELFTEST=1` (invert bookshelf count) | **FAIL** + `EXIT:1` — `BUILDING_SELFTEST: the building authored 4 bookshelf(s); the inverted assertion expected 0` | `e2e-building-selftest.log` |
| `RAILS_SELFTEST=1` (invert rail count) | **FAIL** + `EXIT:1` — `RAILS_SELFTEST: the rail part authored 48 rail(s); the inverted assertion expected 0` | `e2e-rails-selftest.log` |
| log gate | `check-e2e-logs.py the_lost_cities-e2e --strict` → `logs=7 ok=7 missing-exit=0 bad-exit=0` | — |

The launcher native build printed **`Compiling the_lost_cities (1 of 1)`**, tag
`native-0a875da-afbe05a47b8d`, and `native-mods.json` named version **0.1.3**
before any leg ran. No stale the_lost_cities `native-*` tag was present at the
start of the build and no exit 10 from a concurrent native build occurred. The
0.1.2 e2e logs are retired to `retired-0.1.2/`.

## Re-run

```sh
# ladder (compile steps forward to AdLf-PC automatically)
ports/tools/build-slot.sh run --job TLC --port ports/the_lost_cities --step test  -- cargo test --offline
ports/tools/build-slot.sh run --job TLC --port ports/the_lost_cities --step lint -- cargo clippy --all-targets --offline -- -D warnings
ports/tools/build-slot.sh run --job TLC --port ports/the_lost_cities --step check -- cargo horizon-mod check
ports/tools/build-slot.sh run --job TLC --port ports/the_lost_cities --step build -- bash -c 'cd ports/the_lost_cities && rm -f Cargo.lock && cargo horizon-mod build && cp target/horizon-mod/the_lost_cities.hmod the_lost_cities.hmod'
ports/tools/build-slot.sh run --job TLC --port ports/the_lost_cities --step check -- cargo horizon-mod check $PWD/ports/the_lost_cities/the_lost_cities.hmod
ports/tools/build-slot.sh run --job TLC --port ports/the_lost_cities --step check -- cargo horizon-mod fits $PWD/ports/the_lost_cities/the_lost_cities.hmod --build 0a875da
ports/tools/build-slot.sh run --job TLC --port ports/the_lost_cities --step check -- cargo horizon-mod fits $PWD/ports/the_lost_cities/the_lost_cities.hmod --build 0b14f8a
BUILD_LOCAL=1 ports/tools/build-slot.sh run --job TLC --port ports/the_lost_cities --step build -- ports/tools/launcher-game.sh build the_lost_cities
# legs (block in the lease until EXIT:)
eval "$(ports/tools/launcher-game.sh env the_lost_cities)"
BUILD_LOCAL=1 ports/tools/build-slot.sh run --job TLC-e2e --port ports/the_lost_cities --step e2e -- bash ports/the_lost_cities/tools/e2e.sh
# plus GAME=$CONTROL_GAME, SELFTEST=1, TAG_SELFTEST=1, CITY_SELFTEST=1, BUILDING_SELFTEST=1, RAILS_SELFTEST=1
```
