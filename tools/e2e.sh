#!/usr/bin/env bash
# In-game check for The Lost Cities slice 1: the registered lostcity dimension,
# the carried block tag, and the registry differential. Run through the slot:
#
#   eval "$(ports/tools/launcher-game.sh env the_lost_cities)"
#   BUILD_LOCAL=1 ports/tools/build-slot.sh run --job TLC \
#       --port ports/the_lost_cities --step e2e -- bash -lc '<eval> && tools/e2e.sh'
#
# Legs:
#   default       -> PASS EXIT:0  (dimension exists and accepts a block; tag matches bedrock)
#   GAME=$CONTROL -> FAIL EXIT:1  (the unmodded game knows neither the dimension nor the tag)
#   SELFTEST=1    -> FAIL EXIT:1  (inverted dimension assertion)
#   TAG_SELFTEST=1-> FAIL EXIT:1  (inverted tag assertion)
#   CITY_SELFTEST=1 -> FAIL EXIT:1 (inverted generator-authored block count)
#   BUILDING_SELFTEST=1 -> FAIL EXIT:1 (inverted bookshelf count of the six-layer part)
#   RAILS_SELFTEST=1 -> FAIL EXIT:1 (inverted rail count of the horizontal rail line)
#   BEND_SELFTEST=1  -> FAIL EXIT:1 (inverted north_south rail count of the bend)
#   SPLIT_SELFTEST=1 -> FAIL EXIT:1 (inverted north_east rail count of the 3split)
#   DOWN2_SELFTEST=1 -> FAIL EXIT:1 (inverted ascending_east rail count of the descent)
#   DOWN1_SELFTEST=1 -> FAIL EXIT:1 (inverted iron-bar count of the first descent)
set -euo pipefail
export LC_ALL=C

CAMPAIGN=/home/adlf/Desktop/all_the_mods_10_rust_rewrite
PORT=$CAMPAIGN/ports/the_lost_cities
CTL=${CTL:-}
GAME=${GAME:-}
RES=${RES:-${HORIZON_RES:-/home/adlf/Desktop/horizon/.local/horizon-client/26.3}}
SOCK=${SOCK:-/tmp/opencode/the_lost_cities-e2e.sock}
E2E_TMP=${E2E_TMP:-/tmp/opencode}
SELFTEST=${SELFTEST:-0}
TAG_SELFTEST=${TAG_SELFTEST:-0}
CITY_SELFTEST=${CITY_SELFTEST:-0}
BUILDING_SELFTEST=${BUILDING_SELFTEST:-0}
RAILS_SELFTEST=${RAILS_SELFTEST:-0}
BEND_SELFTEST=${BEND_SELFTEST:-0}
SPLIT_SELFTEST=${SPLIT_SELFTEST:-0}
DOWN2_SELFTEST=${DOWN2_SELFTEST:-0}
DOWN1_SELFTEST=${DOWN1_SELFTEST:-0}

[ -n "$CTL" ] && [ -n "$GAME" ] || { echo "missing CTL/GAME: eval \"\$(ports/tools/launcher-game.sh env the_lost_cities)\"" >&2; exit 1; }
[ -x "$CTL" ] || { echo "missing horizon-ctl: $CTL" >&2; exit 1; }
[ -x "$GAME" ] || { echo "missing horizon-client: $GAME" >&2; exit 1; }
[ -d "$RES/assets" ] || { echo "missing prepared resources: $RES" >&2; exit 1; }

say() { echo "== $* =="; }
fail() { echo "FAIL: $*" >&2; exit 1; }

SCRATCH=$(mktemp -d "$E2E_TMP/tlc-home.XXXXXX")
export HORIZON_HOME=$SCRATCH/game
export HORIZON_LAUNCHER_HOME=$SCRATCH/launcher
export XDG_DATA_HOME=$SCRATCH/data
export XDG_CONFIG_HOME=$SCRATCH/config
export HORIZON_NO_AUDIO=1
mkdir -p "$(dirname "$SOCK")" "$SCRATCH"
rm -f "$SOCK"
cd "$SCRATCH"

ctl() { "$CTL" --socket "$SOCK" "$@"; }
end() { ctl command "execute in minecraft:the_end run $1"; }
lc() { ctl command "execute in lostcities:lostcity run $1"; }

cleanup() {
    rc=$?
    ctl quit >/dev/null 2>&1 || true
    sleep 1
    rm -rf "$SCRATCH" || true
    echo "EXIT:$rc"
}
trap cleanup EXIT

say "launch (offscreen, creative, The End, seed 20261105)"
if ! "$CTL" --socket "$SOCK" launch \
    --game "$GAME" --world LostCitiesE2E --seed 20261105 --creative --offscreen --wait \
    --game-dir "$SCRATCH/game" --resources "$RES" \
    > "$SCRATCH/launch.json" 2> "$SCRATCH/launch.err"; then
    echo "launch reply: $(cat "$SCRATCH/launch.json" 2>/dev/null)" >&2
    tail -25 "$SCRATCH/game/logs/latest.log" >&2 || true
    fail "the game did not launch"
fi
jq -c '{dimension, creative, ok: .launch.ok}' "$SCRATCH/launch.json" || true
ctl wait-ready 180 || fail "the game did not become ready"

# A carried tag whose members cannot resolve must not be a silent data refusal.
LOG=$(ctl logs --lines 8000 2>/dev/null || true)
for bad in "Couldn't parse data file" "invalid world generation field" "Unknown recipe type"; do
    if grep -qi "$bad" <<<"$LOG"; then
        grep -i "$bad" <<<"$LOG" >&2
        fail "the level opened with '$bad' in the log"
    fi
done
echo "the level opened without a data-file refusal"

say "The End arena (stone floor at y=79)"
end "forceload add -16 -16 96 80" >/dev/null
end "tp @s 8.5 80 8.5" >/dev/null
ctl wait 40
end "fill -16 79 -16 80 79 80 minecraft:stone" >/dev/null
for y in 80 81 82 83 84 85 86 87 88 89 90; do end "fill -16 $y -16 80 $y 80 minecraft:air" >/dev/null; done
end "gamerule spawn_mobs false" >/dev/null
ctl wait 5

say "registered dimension lostcities:lostcity (only the package supplies it)"
dim_reply=$(lc "say LC_DIM_PRESENT" 2>&1 || true)
dim_chat=$(echo "$dim_reply" | jq -r '.chat[0] // ""' 2>/dev/null || echo "")
echo "dimension reply: $(echo "$dim_reply" | jq -c . 2>/dev/null | head -c 180)"
if [ "$SELFTEST" = 1 ]; then
    grep -q 'LC_DIM_PRESENT' <<<"$dim_chat" \
        && fail "SELFTEST: the dimension answered; the inverted assertion expected no answer"
    echo "SELFTEST: no dimension reply (unexpected)"
else
    grep -q 'LC_DIM_PRESENT' <<<"$dim_chat" \
        || fail "the registered dimension did not answer: $dim_reply"
fi
# The dimension is a real level, not only a name: a block written in it is read
# back. A brand-new level has no loaded chunks, so force-load one first.
lc "forceload add -1 -1 1 1" >/dev/null || fail "forceload in lostcity failed"
ctl wait 40
lc "setblock 0 70 0 minecraft:stone" >/dev/null || fail "setblock in lostcity failed"
ctl wait 5
block_reply=$(lc "fill 0 70 0 0 70 0 minecraft:air replace minecraft:stone" 2>&1 || true)
block_chat=$(echo "$block_reply" | jq -r '.chat[0] // ""' 2>/dev/null || echo "")
echo "dimension block reply: $block_chat"
[ "$SELFTEST" = 1 ] || grep -q 'Successfully filled 1 block' <<<"$block_chat" \
    || fail "the dimension level did not hold the written stone: $block_reply"

# Slice 2: the dimension's own chunk generator lays the decoded jar part. The
# assertion replaces the exact block the part's palette resolves to
# (`minecraft:smooth_stone_slab[type=double]`, see palettes/default.json); the
# brief's illustrative `minecraft:stone` is not in any jar palette, so using it
# would invent content. The same fill on the vanilla-noise Overworld is the
# control: natural terrain has no smooth stone slabs there, so its count is 0.
say "deterministic city generator authors blocks in its own dimension"
city_reply=$(lc "fill 0 60 0 16 90 16 minecraft:air replace minecraft:smooth_stone_slab" 2>&1 || true)
city_chat=$(echo "$city_reply" | jq -r '.chat[0] // ""' 2>/dev/null || echo "")
echo "city generator fill: $(echo "$city_reply" | jq -c . 2>/dev/null | head -c 180)"
city_count=$(echo "$city_chat" | grep -oE 'Successfully filled [0-9]+ block' | grep -oE '[0-9]+' || echo 0)
city_count=${city_count:-0}
echo "generator-authored slab count: $city_count"
vanilla_reply=$(ctl command "fill 0 60 0 16 90 16 minecraft:air replace minecraft:smooth_stone_slab" 2>&1 || true)
vanilla_chat=$(echo "$vanilla_reply" | jq -r '.chat[0] // ""' 2>/dev/null || echo "")
echo "vanilla-noise control fill: $(echo "$vanilla_reply" | jq -c . 2>/dev/null | head -c 180)"
vanilla_count=$(echo "$vanilla_chat" | grep -oE 'Successfully filled [0-9]+ block' | grep -oE '[0-9]+' || echo 0)
vanilla_count=${vanilla_count:-0}
echo "vanilla-noise slab count: $vanilla_count"
if [ "$CITY_SELFTEST" = 1 ]; then
    [ "$city_count" -eq 0 ] \
        || fail "CITY_SELFTEST: the generator authored $city_count slab(s); the inverted assertion expected 0"
    echo "CITY_SELFTEST: no generator-authored blocks (unexpected)"
else
    [ "$city_count" -gt 0 ] \
        || fail "the chunk generator authored no city slabs: $city_reply"
    [ "$vanilla_count" -eq 0 ] \
        || fail "the vanilla-noise control unexpectedly had $vanilla_count slab(s): $vanilla_reply"
fi

# Slice 3: the second, six-layer building part is laid at BUILDING_BASE_Y=72.
# Its `L` char resolves to `minecraft:bookshelf` in the merged jar palette, which
# the street part never writes; fill-replacing that id in the generator's level
# must report a count > 0. The same fill on the vanilla-noise Overworld is the
# control (no bookshelves in natural terrain), and BUILDING_SELFTEST inverts it.
say "six-layer building part authors bookshelves at the second Y"
bldg_reply=$(lc "fill 0 70 0 16 84 16 minecraft:air replace minecraft:bookshelf" 2>&1 || true)
bldg_chat=$(echo "$bldg_reply" | jq -r '.chat[0] // ""' 2>/dev/null || echo "")
echo "building bookshelf fill: $(echo "$bldg_reply" | jq -c . 2>/dev/null | head -c 180)"
bldg_count=$(echo "$bldg_chat" | grep -oE 'Successfully filled [0-9]+ block' | grep -oE '[0-9]+' || echo 0)
bldg_count=${bldg_count:-0}
echo "building-authored bookshelf count: $bldg_count"
vanilla_bldg_reply=$(ctl command "fill 0 70 0 16 84 16 minecraft:air replace minecraft:bookshelf" 2>&1 || true)
vanilla_bldg_chat=$(echo "$vanilla_bldg_reply" | jq -r '.chat[0] // ""' 2>/dev/null || echo "")
echo "vanilla-noise bookshelf control: $(echo "$vanilla_bldg_reply" | jq -c . 2>/dev/null | head -c 180)"
vanilla_bldg_count=$(echo "$vanilla_bldg_chat" | grep -oE 'Successfully filled [0-9]+ block' | grep -oE '[0-9]+' || echo 0)
vanilla_bldg_count=${vanilla_bldg_count:-0}
echo "vanilla-noise bookshelf count: $vanilla_bldg_count"
if [ "$BUILDING_SELFTEST" = 1 ]; then
    [ "$bldg_count" -eq 0 ] \
        || fail "BUILDING_SELFTEST: the building authored $bldg_count bookshelf(s); the inverted assertion expected 0"
    echo "BUILDING_SELFTEST: no building-authored bookshelves (unexpected)"
else
    [ "$bldg_count" -gt 0 ] \
        || fail "the chunk generator authored no building bookshelves: $bldg_reply"
    [ "$vanilla_bldg_count" -eq 0 ] \
        || fail "the vanilla-noise control unexpectedly had $vanilla_bldg_count bookshelf(s): $vanilla_bldg_reply"
fi

# Slice 4: the third, six-layer rail part is laid at RAILS_BASE_Y=54. Its `e`
# char resolves to `minecraft:rail[shape=east_west]` in palettes/rails.json — the
# artefact's id, not the brief's illustrative north_south example — and neither
# the street nor the building writes a rail. The same fill on the vanilla-noise
# Overworld must report `No blocks were filled`; RAILS_SELFTEST inverts it.
say "six-layer horizontal rail line authors rails at the third Y"
rails_reply=$(lc "fill 0 50 0 16 64 16 minecraft:air replace minecraft:rail" 2>&1 || true)
rails_chat=$(echo "$rails_reply" | jq -r '.chat[0] // ""' 2>/dev/null || echo "")
echo "rail fill: $(echo "$rails_reply" | jq -c . 2>/dev/null | head -c 180)"
rails_count=$(echo "$rails_chat" | grep -oE 'Successfully filled [0-9]+ block' | grep -oE '[0-9]+' || echo 0)
rails_count=${rails_count:-0}
echo "rail-authored count: $rails_count"
vanilla_rails_reply=$(ctl command "fill 0 50 0 16 64 16 minecraft:air replace minecraft:rail" 2>&1 || true)
vanilla_rails_chat=$(echo "$vanilla_rails_reply" | jq -r '.chat[0] // ""' 2>/dev/null || echo "")
echo "vanilla-noise rail control: $(echo "$vanilla_rails_reply" | jq -c . 2>/dev/null | head -c 180)"
vanilla_rails_count=$(echo "$vanilla_rails_chat" | grep -oE 'Successfully filled [0-9]+ block' | grep -oE '[0-9]+' || echo 0)
vanilla_rails_count=${vanilla_rails_count:-0}
echo "vanilla-noise rail count: $vanilla_rails_count"
if [ "$RAILS_SELFTEST" = 1 ]; then
    [ "$rails_count" -eq 0 ] \
        || fail "RAILS_SELFTEST: the rail part authored $rails_count rail(s); the inverted assertion expected 0"
    echo "RAILS_SELFTEST: no rail-authored blocks (unexpected)"
else
    [ "$rails_count" -gt 0 ] \
        || fail "the chunk generator authored no rails: $rails_reply"
    [ "$vanilla_rails_count" -eq 0 ] \
        || fail "the vanilla-noise control unexpectedly had $vanilla_rails_count rail(s): $vanilla_rails_reply"
fi

# Slice 5: the fourth, six-layer rail bend is laid at BEND_BASE_Y=36. Its `k`
# char resolves to `minecraft:rail[shape=north_south]` in palettes/rails.json —
# the artefact's id, and a shape the horizontal line never writes (that one is
# east_west). So a north_south count at y=30..44 can only come from the bend.
# The same fill on the vanilla-noise Overworld must report `No blocks were
# filled`; BEND_SELFTEST inverts it.
say "six-layer rail bend authors north_south rails at the fourth Y"
bend_reply=$(lc "fill 0 30 0 16 44 16 minecraft:air replace minecraft:rail[shape=north_south]" 2>&1 || true)
bend_chat=$(echo "$bend_reply" | jq -r '.chat[0] // ""' 2>/dev/null || echo "")
echo "north_south rail bend fill: $(echo "$bend_reply" | jq -c . 2>/dev/null | head -c 180)"
bend_count=$(echo "$bend_chat" | grep -oE 'Successfully filled [0-9]+ block' | grep -oE '[0-9]+' || echo 0)
bend_count=${bend_count:-0}
echo "bend-authored north_south rail count: $bend_count"
vanilla_bend_reply=$(ctl command "fill 0 30 0 16 44 16 minecraft:air replace minecraft:rail[shape=north_south]" 2>&1 || true)
vanilla_bend_chat=$(echo "$vanilla_bend_reply" | jq -r '.chat[0] // ""' 2>/dev/null || echo "")
echo "vanilla-noise north_south rail control: $(echo "$vanilla_bend_reply" | jq -c . 2>/dev/null | head -c 180)"
vanilla_bend_count=$(echo "$vanilla_bend_chat" | grep -oE 'Successfully filled [0-9]+ block' | grep -oE '[0-9]+' || echo 0)
vanilla_bend_count=${vanilla_bend_count:-0}
echo "vanilla-noise north_south rail count: $vanilla_bend_count"
if [ "$BEND_SELFTEST" = 1 ]; then
    [ "$bend_count" -eq 0 ] \
        || fail "BEND_SELFTEST: the bend authored $bend_count north_south rail(s); the inverted assertion expected 0"
    echo "BEND_SELFTEST: no bend-authored north_south rails (unexpected)"
else
    [ "$bend_count" -gt 0 ] \
        || fail "the chunk generator authored no north_south rails: $bend_reply"
    [ "$vanilla_bend_count" -eq 0 ] \
        || fail "the vanilla-noise control unexpectedly had $vanilla_bend_count north_south rail(s): $vanilla_bend_reply"
fi

# Slice 6: the fifth, six-layer rail split is laid at SPLIT_BASE_Y=18. Its `n`
# char resolves to `minecraft:rail[shape=north_east]` in palettes/rails.json —
# a shape none of the other four decoded parts writes (the horizontal line is
# east_west; the bend is north_south/north_west/south_west). So a north_east
# count at y=12..26 can only come from this part. The same fill on the
# vanilla-noise Overworld must report `No blocks were filled`; SPLIT_SELFTEST
# inverts it.
say "six-layer rail split authors north_east rails at the fifth Y"
split_reply=$(lc "fill 0 12 0 16 26 16 minecraft:air replace minecraft:rail[shape=north_east]" 2>&1 || true)
split_chat=$(echo "$split_reply" | jq -r '.chat[0] // ""' 2>/dev/null || echo "")
echo "north_east rail split fill: $(echo "$split_reply" | jq -c . 2>/dev/null | head -c 180)"
split_count=$(echo "$split_chat" | grep -oE 'Successfully filled [0-9]+ block' | grep -oE '[0-9]+' || echo 0)
split_count=${split_count:-0}
echo "split-authored north_east rail count: $split_count"
vanilla_split_reply=$(ctl command "fill 0 12 0 16 26 16 minecraft:air replace minecraft:rail[shape=north_east]" 2>&1 || true)
vanilla_split_chat=$(echo "$vanilla_split_reply" | jq -r '.chat[0] // ""' 2>/dev/null || echo "")
echo "vanilla-noise north_east rail control: $(echo "$vanilla_split_reply" | jq -c . 2>/dev/null | head -c 180)"
vanilla_split_count=$(echo "$vanilla_split_chat" | grep -oE 'Successfully filled [0-9]+ block' | grep -oE '[0-9]+' || echo 0)
vanilla_split_count=${vanilla_split_count:-0}
echo "vanilla-noise north_east rail count: $vanilla_split_count"
if [ "$SPLIT_SELFTEST" = 1 ]; then
    [ "$split_count" -eq 0 ] \
        || fail "SPLIT_SELFTEST: the split authored $split_count north_east rail(s); the inverted assertion expected 0"
    echo "SPLIT_SELFTEST: no split-authored north_east rails (unexpected)"
else
    [ "$split_count" -gt 0 ] \
        || fail "the chunk generator authored no north_east rails: $split_reply"
    [ "$vanilla_split_count" -eq 0 ] \
        || fail "the vanilla-noise control unexpectedly had $vanilla_split_count north_east rail(s): $vanilla_split_reply"
fi

# Slice 7: the sixth, eighteen-layer rail descent is laid at DOWN2_BASE_Y=4.
# Its `j` char resolves to `minecraft:rail[shape=ascending_east]` in
# palettes/rails.json — a shape none of the other five decoded parts writes
# (they use east_west, north_south, north_west, south_west, north_east and
# south_east). So an ascending_east count at y=0..24 can only come from this
# part. The same fill on the vanilla-noise Overworld must report `No blocks
# were filled`; DOWN2_SELFTEST inverts it.
say "eighteen-layer rail descent authors ascending_east rails at the sixth Y"
down2_reply=$(lc "fill 0 0 0 16 24 16 minecraft:air replace minecraft:rail[shape=ascending_east]" 2>&1 || true)
down2_chat=$(echo "$down2_reply" | jq -r '.chat[0] // ""' 2>/dev/null || echo "")
echo "ascending_east rail descent fill: $(echo "$down2_reply" | jq -c . 2>/dev/null | head -c 180)"
down2_count=$(echo "$down2_chat" | grep -oE 'Successfully filled [0-9]+ block' | grep -oE '[0-9]+' || echo 0)
down2_count=${down2_count:-0}
echo "descent-authored ascending_east rail count: $down2_count"
vanilla_down2_reply=$(ctl command "fill 0 0 0 16 24 16 minecraft:air replace minecraft:rail[shape=ascending_east]" 2>&1 || true)
vanilla_down2_chat=$(echo "$vanilla_down2_reply" | jq -r '.chat[0] // ""' 2>/dev/null || echo "")
echo "vanilla-noise ascending_east rail control: $(echo "$vanilla_down2_reply" | jq -c . 2>/dev/null | head -c 180)"
vanilla_down2_count=$(echo "$vanilla_down2_chat" | grep -oE 'Successfully filled [0-9]+ block' | grep -oE '[0-9]+' || echo 0)
vanilla_down2_count=${vanilla_down2_count:-0}
echo "vanilla-noise ascending_east rail count: $vanilla_down2_count"
if [ "$DOWN2_SELFTEST" = 1 ]; then
    [ "$down2_count" -eq 0 ] \
        || fail "DOWN2_SELFTEST: the descent authored $down2_count ascending_east rail(s); the inverted assertion expected 0"
    echo "DOWN2_SELFTEST: no descent-authored ascending_east rails (unexpected)"
else
    [ "$down2_count" -gt 0 ] \
        || fail "the chunk generator authored no ascending_east rails: $down2_reply"
    [ "$vanilla_down2_count" -eq 0 ] \
        || fail "the vanilla-noise control unexpectedly had $vanilla_down2_count ascending_east rail(s): $vanilla_down2_reply"
fi

# Slice 8: the seventh, twelve-layer first rail descent is laid at
# DOWN1_BASE_Y=100 (spanning y=100..111, above the street at y=90). Its `:`
# char resolves to `minecraft:iron_bars` in palettes/default.json — a block no
# other decoded part writes (their palettes only name iron bars as a `damaged`
# field, which the decoder never places). So an iron-bar count at y=96..112 can
# only come from this part. The same fill on the vanilla-noise Overworld must
# report `No blocks were filled`; DOWN1_SELFTEST inverts it.
say "twelve-layer first rail descent authors iron bars at the seventh Y"
down1_reply=$(lc "fill 0 96 0 16 112 16 minecraft:air replace minecraft:iron_bars" 2>&1 || true)
down1_chat=$(echo "$down1_reply" | jq -r '.chat[0] // ""' 2>/dev/null || echo "")
echo "iron-bar first descent fill: $(echo "$down1_reply" | jq -c . 2>/dev/null | head -c 180)"
down1_count=$(echo "$down1_chat" | grep -oE 'Successfully filled [0-9]+ block' | grep -oE '[0-9]+' || echo 0)
down1_count=${down1_count:-0}
echo "first-descent-authored iron-bar count: $down1_count"
vanilla_down1_reply=$(ctl command "fill 0 96 0 16 112 16 minecraft:air replace minecraft:iron_bars" 2>&1 || true)
vanilla_down1_chat=$(echo "$vanilla_down1_reply" | jq -r '.chat[0] // ""' 2>/dev/null || echo "")
echo "vanilla-noise iron-bar control: $(echo "$vanilla_down1_reply" | jq -c . 2>/dev/null | head -c 180)"
vanilla_down1_count=$(echo "$vanilla_down1_chat" | grep -oE 'Successfully filled [0-9]+ block' | grep -oE '[0-9]+' || echo 0)
vanilla_down1_count=${vanilla_down1_count:-0}
echo "vanilla-noise iron-bar count: $vanilla_down1_count"
if [ "$DOWN1_SELFTEST" = 1 ]; then
    [ "$down1_count" -eq 0 ] \
        || fail "DOWN1_SELFTEST: the first descent authored $down1_count iron-bar(s); the inverted assertion expected 0"
    echo "DOWN1_SELFTEST: no first-descent-authored iron bars (unexpected)"
else
    [ "$down1_count" -gt 0 ] \
        || fail "the chunk generator authored no iron bars: $down1_reply"
    [ "$vanilla_down1_count" -eq 0 ] \
        || fail "the vanilla-noise control unexpectedly had $vanilla_down1_count iron-bar(s): $vanilla_down1_reply"
fi

say "carried block tag #lostcities:notbreakable matches bedrock"
end "setblock 50 80 50 minecraft:bedrock" >/dev/null
ctl wait 5
tag_reply=$(ctl command "execute if block 50 80 50 #lostcities:notbreakable" 2>&1 || true)
tag_chat=$(echo "$tag_reply" | jq -r '.chat[0] // ""' 2>/dev/null || echo "")
echo "tag reply: $(echo "$tag_reply" | jq -c . 2>/dev/null | head -c 160)"
if [ "$TAG_SELFTEST" = 1 ]; then
    grep -q 'Test passed' <<<"$tag_chat" \
        && fail "TAG_SELFTEST: the tag matched; the inverted assertion expected no match"
    echo "TAG_SELFTEST: no match (unexpected)"
else
    grep -q 'Test passed' <<<"$tag_chat" \
        || fail "the carried tag did not match bedrock: $tag_reply"
fi
# Differential: an unknown tag is refused by the registry. `ctl command`
# answers ok:true for a refused command, so the chat text is the assertion.
unknown=$(ctl command "execute if block 50 80 50 #lostcities:no_such_tag" 2>&1 || true)
unknown_chat=$(echo "$unknown" | jq -r '.chat[0] // ""' 2>/dev/null || echo "")
echo "unknown tag differential: $unknown_chat"
grep -q 'Unknown block tag' <<<"$unknown_chat" \
    || fail "an unknown tag id was not refused: $unknown"

say "carried loot tables yield items (26.3 entry shape)"
inv_before=$(ctl inventory 2>/dev/null | jq -r '[.slots[]? | select(.id != null)] | length' 2>/dev/null || echo 0)
echo "inventory before loot: $inv_before"
loot_reply=$(ctl command "loot give @p loot lostcities:chests/lostcitychest" 2>&1 || true)
loot_chat=$(echo "$loot_reply" | jq -r '.chat[0] // ""' 2>/dev/null || echo "")
echo "lostcitychest: $loot_chat"
grep -qE 'Dropped [1-9][0-9]* (items|\[)' <<<"$loot_chat" \
    || fail "the carried city chest yielded no items (the 1.21.1 entry shape defect): $loot_reply"
loot2_reply=$(ctl command "loot give @p loot lostcities:chests/raildungeonchest" 2>&1 || true)
loot2_chat=$(echo "$loot2_reply" | jq -r '.chat[0] // ""' 2>/dev/null || echo "")
echo "raildungeonchest: $loot2_chat"
grep -qE 'Dropped [1-9][0-9]* (items|\[)' <<<"$loot2_chat" \
    || fail "the carried rail dungeon chest yielded no items: $loot2_reply"
ctl wait 3 >/dev/null
inv_after=$(ctl inventory 2>/dev/null | jq -r '[.slots[]? | select(.id != null)] | length' 2>/dev/null || echo 0)
echo "inventory after loot: $inv_after (informational: creative-mode loot give drops at feet)"
unknown_loot=$(ctl command "loot give @p loot lostcities:chests/no_such_table" 2>&1 || true)
unknown_loot_chat=$(echo "$unknown_loot" | jq -r '.chat[0] // ""' 2>/dev/null || echo "")
echo "unknown loot differential: $unknown_loot_chat"
grep -q "Can't find element" <<<"$unknown_loot_chat" \
    || fail "an unknown loot table id was not refused: $unknown_loot"

say "carried data summary"
echo "tags=$(find "$PORT/data/lostcities/tags" -name '*.json' | wc -l) " \
     "loot=$(find "$PORT/data/lostcities/loot_table" -name '*.json' | wc -l) " \
     "lang=$(find "$PORT/assets/lostcities/lang" -name '*.json' | wc -l)"

echo "PASS: the lostcity dimension answered, all seven decoded parts authored their own blocks in it (slabs, bookshelves, east_west rails, the bend's north_south rails, the split's north_east rails, the descent's ascending_east rails and the first descent's iron bars), #lostcities:notbreakable matched bedrock, and both carried loot tables yielded items"
