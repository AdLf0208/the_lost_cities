# The Lost Cities — validation record (first slice, v0.1.0)

Pin **`0a875da91dbb0fdcb7ba8c42c11341aa229af51a`**, mixins
`e59da9094f0c2fdda6c627fe9eff22dec6a906f0`, game 26.3 (Nightly
`nightly-2026-11-20-0a875da`), launcher 0.2.92, `api = "0.2"`.

**Version 0.1.0** — a first publication: no upstream entry on `main`, no open
PR, dispatch gate exit 0; no prior record to supersede. Cargo and the manifest
agree at 0.1.0.

## The deliverable

| | |
|---|---|
| package | `ports/the_lost_cities/the_lost_cities.hmod` |
| size | **13,214 B** |
| SHA-256 | `6bbe5b775c2a2db1d4078c0bb88ebca50ca8f5aa81bc957736ca9875320dd89d` |
| version | 0.1.0 (manifest + Cargo.toml) |
| entries | 17 (6 tags + 2 loot tables + 1 lang + manifest/Cargo/src/docs/uses.json) |
| packed Cargo.lock | none |
| runtime | `src/runtime.rs` byte-identical in tree and package (`d6df8b2f45b0…`) |

## Gates (fresh, against the delivered bytes)

| gate | command | result | log |
|---|---|---|---|
| data contract (red) | `python3 tools/carry.py --check` before the carry | **FAIL**, 9 missing files | `evidence/the_lost_cities/carry-contract-red.log` |
| data contract (green) | `python3 tools/carry.py --check` | 6 verbatim tags, 2 loot tables shape-upgraded, 1 verbatim lang, blocked families absent | `evidence/the_lost_cities/carry-contract.log` |
| unit tests (red) | `cargo test --offline`, stub facts | **2 failed**, `wrong height: left 0 right 256` + `invalid dimension height` | `evidence/the_lost_cities/unit-test-red.log` |
| unit tests | `cargo test --offline` | **2 passed, 0 failed** | `evidence/the_lost_cities/sdk-test.log` |
| lints | `cargo clippy --all-targets --offline -- -D warnings` | exit 0, 0 warnings | `evidence/the_lost_cities/sdk-clippy.log` |
| format | `rustfmt --edition 2024 --check src/{lib,runtime}.rs` | clean | `evidence/the_lost_cities/sdk-fmt.log` |
| SDK check (source) | `cargo horizon-mod check` | *30 API items; Made for 0a875da* | `evidence/the_lost_cities/sdk-check.log` |
| package check | `cargo horizon-mod check the_lost_cities.hmod` | *0.1.0, native-1/0a875da…, 13,214 bytes, sha 6bbe5b77…* | `evidence/the_lost_cities/sdk-package-check.log` |
| fits (pin) | `cargo horizon-mod fits … --build 0a875da` | **`made for`**, exit 0 | `evidence/the_lost_cities/sdk-fits.log` |
| fits (other build) | `cargo horizon-mod fits … --build 0b14f8a` | **`fits`**, exit 0 — the dimension registrar exists in both cached builds, so this is the honest result, not a tautology about a new API | `evidence/the_lost_cities/sdk-fits-other.log` |
| package gate | `verify-sdk-package.py the_lost_cities` | **1/1**, including "package matches the tree" | — |
| skills gate | `check-skills.py --notes ports/the_lost_cities --strict` | PASS | — |
| skipped gates | `check-recipe-types.py`, `check-manifest-deps.py` | no recipe types; no other registry mod named in code | — |

Every Rust step ran through `ports/tools/build-slot.sh` with
`CARGO_INCREMENTAL=0`, no `--target-dir`. Compile/test/check/build steps
forwarded to the repaired remote automatically; only the launcher game build
and the e2e legs were local (`BUILD_LOCAL=1`) because the launcher and its
managed Nightly install live on this machine.

## In-game legs (`ports/_shared/evidence/the_lost_cities-e2e/`)

| leg | result | log |
|---|---|---|
| modded game | **PASS** + `EXIT:0` — `execute in lostcities:lostcity run say` → `LC_DIM_PRESENT`; a stone block written in that level is read back; `#lostcities:notbreakable` → `Test passed` on bedrock; both carried loot tables → `Dropped 3–4 items`; unknown tag/loot ids refused | `e2e.log` |
| control (same Nightly, no mod) | **FAIL** + `EXIT:1` — `Unknown dimension 'lostcities:lostcity'` | `e2e-control.log` |
| `SELFTEST=1` (invert dimension assertion) | **FAIL** + `EXIT:1` — `SELFTEST: the dimension answered` | `e2e-selftest.log` |
| `TAG_SELFTEST=1` (invert tag assertion) | **FAIL** + `EXIT:1` — `TAG_SELFTEST: the tag matched` | `e2e-tag-selftest.log` |
| pre-fix loot counterfactual | **`Dropped 0 items`** on the unfixed package — the 1.21.1 entry-type defect, fixed in the delivered data | `evidence/the_lost_cities/loot-probe.log` |
| log gate | `check-e2e-logs.py the_lost_cities-e2e --strict` → `logs=4 ok=4 missing-exit=0 bad-exit=0` | — |

One `native-build` attempt was refused by a **concurrent native build from
another agent** (`error: A native build or cache cleanup is running`, exit 10).
The retry succeeded (`native-0a875da-fcbca9be0aa3`, `launcher-build.log`) and
**all four e2e legs above were re-run against that final export**, whose
`native-mods.json` names the delivered 0.1.0 mod.

## Re-run

```sh
# ladder (each a separate lease; compile steps forward automatically)
ports/tools/build-slot.sh run --job TLC --port ports/the_lost_cities --step test  -- cargo test --offline
ports/tools/build-slot.sh run --job TLC --port ports/the_lost_cities --step lint -- cargo clippy --all-targets --offline -- -D warnings
ports/tools/build-slot.sh run --job TLC --port ports/the_lost_cities --step check -- cargo horizon-mod check
ports/tools/build-slot.sh run --job TLC --port ports/the_lost_cities --step build -- bash -c 'rm -f Cargo.lock && cargo horizon-mod build && cp target/horizon-mod/the_lost_cities.hmod the_lost_cities.hmod'
BUILD_LOCAL=1 ports/tools/build-slot.sh run --job TLC --port ports/the_lost_cities --step build -- ports/tools/launcher-game.sh build the_lost_cities
BUILD_LOCAL=1 ports/tools/build-slot.sh run --job TLC --port ports/the_lost_cities --step e2e -- \
  bash -lc 'cd ports/the_lost_cities && eval "$(/home/adlf/Desktop/all_the_mods_10_rust_rewrite/ports/tools/launcher-game.sh env the_lost_cities)" && tools/e2e.sh'
```
