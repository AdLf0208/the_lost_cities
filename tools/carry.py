#!/usr/bin/env python3
"""Carry The Lost Cities' expressible data from the cached jar into a 26.3 tree.

The jar ships a Java core plus data. This tool copies only the families the pinned
SDK actually decodes: the six block tags, the one JSON language file and the two
vanilla-format chest loot tables. The dimension is registered from Rust (its values
come from the jar's `dimension_type/lostcity.json`, pinned in `src/runtime.rs`), and
the custom feature types, biome modifiers, internal generator JSON and Java classes
are deliberately NOT packed — each has a `blocked(<symbol>)` row in `notes.md`.

`--check` is the data-contract test: before `--write` it fails (the tree is empty);
afterwards it fixes the exact file set, that every byte matches the jar, that the
loot tables use the 26.3 entry shape, and that the blocked families are absent.
It is re-run by the e2e so a re-carry cannot silently change the data.

Nothing here copies Java code or Mojang files. MIT permits carrying the art, but
the 18 GUI icons belong to a config screen this port does not implement, so no
art is carried in slice 1.

The two loot tables need one shape fix: the jar's entries say `"type": "item"`,
which 26.3's loot decoder (horizon-world `loot.rs`: `expand`/`create` match
`"minecraft:item"`) skips silently, making the table yield **0 items**. The port
rewrites that one field to `"minecraft:item"` — the smallest change that makes
the mod's own data mean what it says (recorded in notes.md).
"""
from __future__ import annotations

import argparse
import hashlib
import json
import zipfile
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
CAMPAIGN = ROOT.parent.parent
JAR = CAMPAIGN / "ports/_shared/licence-cache/the-lost-cities.jar"

# Exact carried set (measured from the jar, 2026-11-24).
TAGS = [
    "easybreakable", "foliage", "lights", "needspoi", "notbreakable", "rotatable",
]
LOOT = ["lostcitychest", "raildungeonchest"]
LANG = "en_us.json"

CARRIED = (
    [f"data/lostcities/tags/block/{n}.json" for n in TAGS]
    + [f"data/lostcities/loot_table/chests/{n}.json" for n in LOOT]
    + [f"assets/lostcities/lang/{LANG}"]
)

# Families present in the jar but not packed: see notes.md for the measurements.
BANNED = [
    "data/lostcities/lostcities",       # blocked(no-city-data-decoder)
    "data/lostcities/neoforge",         # blocked(configured-feature-not-registered)
    "data/lostcities/worldgen",         # blocked(configured-feature-not-registered)
    "data/lostcities/dimension",        # registered from Rust instead
    "data/lostcities/dimension_type",   # registered from Rust instead
]


def jar_bytes():
    with zipfile.ZipFile(JAR) as zf:
        return {name: zf.read(name) for name in CARRIED if name in zf.namelist()}


def upgrade_loot(doc):
    """1.21.1 `"type": "item"` -> 26.3 `"type": "minecraft:item"`.

    The rest of the table (rolls, weights, `modifier` functions with their
    `minecraft:`-qualified types) is already in the shape 26.3 decodes.
    """
    if not isinstance(doc, dict):
        return doc
    for pool in doc.get("pools", []):
        for entry in pool.get("entries", []):
            if isinstance(entry, dict) and entry.get("type") == "item":
                entry["type"] = "minecraft:item"
    return doc


def write_tree():
    bodies = jar_bytes()
    missing = [name for name in CARRIED if name not in bodies]
    if missing:
        raise SystemExit(f"jar is missing carried files: {missing}")
    fixed = 0
    for name, body in bodies.items():
        if "/loot_table/" in name:
            body = (json.dumps(upgrade_loot(json.loads(body)), indent=2,
                               sort_keys=True) + "\n").encode("utf-8")
            fixed += 1
        target = ROOT / name
        target.parent.mkdir(parents=True, exist_ok=True)
        target.write_bytes(body)
    print(f"carried {len(bodies)} files into {ROOT} ({fixed} loot tables shape-upgraded)")


def check_tree():
    problems = []
    bodies = jar_bytes()
    for name in CARRIED:
        target = ROOT / name
        if not target.is_file():
            problems.append(f"{name}: missing")
            continue
        got, want = target.read_bytes(), bodies.get(name)
        if want is None:
            problems.append(f"{name}: absent from the jar")
        elif "/loot_table/" in name:
            expected = (json.dumps(upgrade_loot(json.loads(want)), indent=2,
                                   sort_keys=True) + "\n").encode("utf-8")
            if got != expected:
                problems.append(f"{name}: loot 26.3 shape not applied")
        elif got != want:
            problems.append(
                f"{name}: not verbatim ({hashlib.sha256(got).hexdigest()[:12]} vs "
                f"{hashlib.sha256(want).hexdigest()[:12]})"
            )
    for banned in BANNED:
        if (ROOT / banned).exists():
            problems.append(f"{banned}: packed but blocked/not decoded")
    # The loot tables must be in the shape the pin's loot decoder reads:
    # `"minecraft:item"` entries with `"modifier"` functions (loot.rs:1837/1874).
    for name in [f"data/lostcities/loot_table/chests/{n}.json" for n in LOOT]:
        target = ROOT / name
        if not target.is_file():
            continue
        doc = json.loads(target.read_text())
        pools = doc.get("pools", [])
        if len(pools) != 1:
            problems.append(f"{name}: {len(pools)} pools, expected 1")
        for entry in pools[0].get("entries", []):
            if entry.get("type") != "minecraft:item":
                problems.append(f"{name}: entry type {entry.get('type')!r} is not decodable")
            if "modifier" not in entry:
                problems.append(f"{name}: entry has no modifier list")
    # The six tags must be non-empty and name the expected ids (a truncated
    # re-carry that silently drops members must fail here, not in game).
    for name in TAGS:
        target = ROOT / f"data/lostcities/tags/block/{name}.json"
        if not target.is_file():
            continue
        doc = json.loads(target.read_text())
        values = doc.get("values")
        if not isinstance(values, list) or not values:
            problems.append(f"tags/block/{name}: empty or missing values")
    if problems:
        for p in problems:
            print(f"FAIL {p}")
        print(f"carry contract FAILED: {len(problems)} problem(s)")
        return 1
    print(
        f"OK    {len(CARRIED)} carried files: {len(TAGS)} verbatim tags, {len(LOOT)} loot "
        "tables with the 26.3 entry-shape fix, 1 verbatim lang; blocked families absent"
    )
    return 0


def main(argv=None):
    ap = argparse.ArgumentParser()
    ap.add_argument("--write", action="store_true")
    ap.add_argument("--check", action="store_true")
    args = ap.parse_args(argv)
    if args.write:
        write_tree()
    if args.check or not args.write:
        return check_tree()
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
