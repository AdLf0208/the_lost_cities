# The Lost Cities — validation record (v0.1.1)

Pin **`0a875da91dbb0fdcb7ba8c42c11341aa229af51a`**, mixins
`e59da9094f0c2fdda6c627fe9eff22dec6a906f0`, game 26.3 (Nightly
`nightly-2026-11-20-0a875da`), `api = "0.2"`.

**Version 0.1.1** — an update of our first publication (0.1.0, PR #1861), adding
the first deterministic city chunk generator. Cargo and the manifest agree at
0.1.1. The orchestrator publishes; this port does not touch registry/PR/ledger.

## The deliverable

| | |
|---|---|
| package | `ports/the_lost_cities/the_lost_cities.hmod` |
| size | **22,266 B** |
| SHA-256 | `ae8b3ab10fb9992b7a54d738e93fd79a356bf57508b1339701171ed367929777` |
| version | 0.1.1 (manifest + Cargo.toml) |
| entries | 17 (6 tags + 2 loot tables + 1 lang + manifest/Cargo/src/docs/uses.json) |
| packed Cargo.lock | none |
| runtime | `src/runtime.rs` byte-identical in tree and package (`17b1103ae27a…`) |

## Gates (fresh, against the delivered bytes)

| gate | command | result | log |
|---|---|---|---|
| data contract | `python3 tools/carry.py --check` | 6 verbatim tags, 2 loot tables shape-upgraded, 1 verbatim lang, blocked families absent | `evidence/the_lost_cities/carry-contract.log` |
| decoder red | `cargo test --offline` before the decoder | **FAIL**, 11 unresolved symbols (`STREET_PART_JSON`, `parse_json`, `Part`, `decode_city`, `split_block`) | `evidence/the_lost_cities/decoder-test-red.log` |
| unit tests | `cargo test --offline` | **8 passed, 0 failed** | `evidence/the_lost_cities/sdk-test.log` |
| lints | `cargo clippy --all-targets --offline -- -D warnings` | exit 0, 0 warnings | `evidence/the_lost_cities/sdk-clippy.log` |
| format | `rustfmt --edition 2024 --check src/{lib,runtime}.rs` | clean | `evidence/the_lost_cities/sdk-fmt.log` |
| SDK check (source) | `cargo horizon-mod check` | *131 API items; Made for 0a875da* | `evidence/the_lost_cities/sdk-check.log` |
| package check | `cargo horizon-mod check the_lost_cities.hmod` | *0.1.1, native-1/0a875da…, 22,266 bytes, sha ae8b3ab1…* | `evidence/the_lost_cities/sdk-package-check.log` |
| fits (pin) | `cargo horizon-mod fits … --build 0a875da` | **`made for`**, exit 0 | `evidence/the_lost_cities/sdk-fits.log` |
| fits (other build) | `cargo horizon-mod fits … --build 0b14f8a` | **`fits`**, exit 0 — registrar + DimensionSettings exist in both builds; only the Rust decoder is ours | `evidence/the_lost_cities/sdk-fits-other.log` |
| package gate | `verify-sdk-package.py the_lost_cities` | **1/1**, incl. "package matches the tree" | — |
| skills gate | `check-skills.py --notes ports/the_lost_cities --strict` | PASS | — |
| skipped gates | `check-recipe-types.py`, `check-manifest-deps.py` | ok; no recipe type, only vanilla block ids in code | — |

README.md and horizon-mod.toml were edited **before** the final build, so the
whole-archive-vs-tree gate is green; the 0.1.0 logs were retired to
`retired-0.1.0/` and `sdk-test.log` regenerated for 0.1.1 after the build.

Every Rust step went through `ports/tools/build-slot.sh` with
`CARGO_INCREMENTAL=0`, no `--target-dir`. Compile/test/check/build steps
forwarded to the repaired remote automatically; the `cargo horizon-mod` package
build, the launcher game build and the e2e legs were local (`BUILD_LOCAL=1`).

## In-game legs (`ports/_shared/evidence/the_lost_cities-e2e/`)

| leg | result | log |
|---|---|---|
| modded game | **PASS** + `EXIT:0` — dimension answers; a written stone block reads back; **city generator fills 136 smooth-stone slabs** in the level, vanilla-noise Overworld fills 0; `#lostcities:notbreakable` → `Test passed`; both loot tables → `Dropped 4 items`; unknown tag/loot ids refused | `e2e.log` |
| control (same Nightly, no mod) | **FAIL** + `EXIT:1` — `Unknown dimension 'lostcities:lostcity'` | `e2e-control.log` |
| `SELFTEST=1` (invert dimension assertion) | **FAIL** + `EXIT:1` — `SELFTEST: the dimension answered` | `e2e-selftest.log` |
| `TAG_SELFTEST=1` (invert tag assertion) | **FAIL** + `EXIT:1` — `TAG_SELFTEST: the tag matched` | `e2e-tag-selftest.log` |
| `CITY_SELFTEST=1` (invert generator count) | **FAIL** + `EXIT:1` — `CITY_SELFTEST: the generator authored 136 slab(s); the inverted assertion expected 0` | `e2e-city-selftest.log` |
| log gate | `check-e2e-logs.py the_lost_cities-e2e --strict` → `logs=5 ok=5 missing-exit=0 bad-exit=0` | — |

The launcher native build printed **`Compiling the_lost_cities (1 of 1)`**, tag
`native-0a875da-cae68fb44f23`, and `native-mods.json` named version **0.1.1**
before any leg ran — so the legs tested this build, not a stale export. No exit
10 from a concurrent native build occurred on this run.

## Re-run

```sh
# ladder (compile steps forward to the remote automatically)
ports/tools/build-slot.sh run --job TLC --port ports/the_lost_cities --step test  -- cargo test --offline
ports/tools/build-slot.sh run --job TLC --port ports/the_lost_cities --step lint -- cargo clippy --all-targets --offline -- -D warnings
ports/tools/build-slot.sh run --job TLC --port ports/the_lost_cities --step check -- cargo horizon-mod check
BUILD_LOCAL=1 ports/tools/build-slot.sh run --job TLC --port ports/the_lost_cities --step build -- bash -c 'rm -f Cargo.lock && cargo horizon-mod build && cp target/horizon-mod/the_lost_cities.hmod the_lost_cities.hmod'
BUILD_LOCAL=1 ports/tools/build-slot.sh run --job TLC --port ports/the_lost_cities --step check -- cargo horizon-mod fits the_lost_cities.hmod --build 0a875da
BUILD_LOCAL=1 ports/tools/build-slot.sh run --job TLC --port ports/the_lost_cities --step check -- cargo horizon-mod fits the_lost_cities.hmod --build 0b14f8a
BUILD_LOCAL=1 ports/tools/build-slot.sh run --job TLC --port ports/the_lost_cities --step build -- ports/tools/launcher-game.sh build the_lost_cities
# legs (block in the lease until EXIT:)
BUILD_LOCAL=1 ports/tools/build-slot.sh run --job TLC-e2e --port ports/the_lost_cities --step e2e -- \
  bash -lc 'cd ports/the_lost_cities && eval "$(/home/adlf/Desktop/all_the_mods_10_rust_rewrite/ports/tools/launcher-game.sh env the_lost_cities)" && tools/e2e.sh'
# plus GAME=$CONTROL_GAME, SELFTEST=1, TAG_SELFTEST=1, CITY_SELFTEST=1
```
