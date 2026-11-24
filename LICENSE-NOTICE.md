# Licence notice — `the_lost_cities` (The Lost Cities)

This port carries data from **The Lost Cities** (mod id `lostcities`), authored
by **McJty** (`McJtyMods`).

## Upstream project

- Repository: <https://github.com/McJtyMods/LostCities> — the URL declared by
  the jar and by the source map; credited in this port's README.
- Modrinth project `8VDCUS3b`, version `26.3-12.0.2` (NeoForge).
- The campaign's port repository is the home of this Rust rewrite, not upstream.

## What the artefacts declare — read, not guessed

| Evidence | Value |
|---|---|
| Jar `META-INF/neoforge.mods.toml` → `license` | `MIT License https://github.com/McJtyMods/LostCities/blob/1.21_neo/LICENSE` |
| Jar `META-INF/neoforge.mods.toml` → `authors` | `McJty` |
| Modrinth project `8VDCUS3b` licence | MIT |
| Dispatch gate (`dispatch-gate.py the_lost_cities`) | exit 0 (MIT, rank 0) |

MIT permits redistribution with the copyright and permission notice, so this
package ships `LICENSE` and this notice. No asset is regenerated because no art
is carried in slice 1: the 18 GUI icons belong to a config screen this port
does not implement, and they are deliberately **not** packed.

## What travels with this package

- `data/lostcities/tags/block/*.json` — six block tags, **verbatim**.
- `data/lostcities/loot_table/chests/*.json` — two chest loot tables, with one
  documented 1.21.1 → 26.3 entry-type fix (see `notes.md`).
- `assets/lostcities/lang/en_us.json` — the language file, **verbatim**.
- `src/` — the clean-room Rust glue registering the `lostcities:lostcity`
  dimension from the jar's `dimension_type` values.

No Java source, no `.class` file, no Mojang file and no art from the jar is
shipped. The cached jar under `ports/_shared/licence-cache/` is evidence and is
never packed. `PROVENANCE.md` records the exact byte comparison.

## Obligations that follow from MIT

Redistribution and modification are permitted; the notice travels with the
package; the software is provided **as is**, without warranty. The port is a
**partial** implementation: city generation is explicitly blocked, as recorded
in `notes.md`, and is not claimed as ported behaviour.
