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
#   BUILDING12_SELFTEST=1 -> FAIL EXIT:1 (inverted quartz count of the second building)
#   CENTER001_SELFTEST=1 -> FAIL EXIT:1 (inverted quartz-stairs count of the city centre)
#   SHOPPING1_SELFTEST=1 -> FAIL EXIT:1 (inverted cobblestone-wall count of the shopping part)
#   STAIRS_SELFTEST=1   -> FAIL EXIT:1 (inverted stone-stairs count of the stair part)
#   STAIRSBIG_SELFTEST=1 -> FAIL EXIT:1 (inverted stone-stairs count of the large stair)
#   BUILDING13_SELFTEST=1-> FAIL EXIT:1 (inverted water count of the third building)
#   CENTER002_SELFTEST=1 -> FAIL EXIT:1 (inverted chest count of the second city centre)
#   LIBRARY1002_SELFTEST=1 -> FAIL EXIT:1 (inverted oak-fence count of the library)
#   RAILDUNGEON1_SELFTEST=1 -> FAIL EXIT:1 (inverted iron-block count of the rail dungeon)
#   RAILDUNGEON2_SELFTEST=1 -> FAIL EXIT:1 (inverted 3-cell iron-block count of the second rail dungeon)
#   RAILEND_SELFTEST=1 -> FAIL EXIT:1 (inverted east-west rail count of the rail end cap)
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
BUILDING12_SELFTEST=${BUILDING12_SELFTEST:-0}
CENTER001_SELFTEST=${CENTER001_SELFTEST:-0}
SHOPPING1_SELFTEST=${SHOPPING1_SELFTEST:-0}
STAIRS_SELFTEST=${STAIRS_SELFTEST:-0}
STAIRSBIG_SELFTEST=${STAIRSBIG_SELFTEST:-0}
BUILDING13_SELFTEST=${BUILDING13_SELFTEST:-0}
CENTER002_SELFTEST=${CENTER002_SELFTEST:-0}
LIBRARY1002_SELFTEST=${LIBRARY1002_SELFTEST:-0}
RAILDUNGEON1_SELFTEST=${RAILDUNGEON1_SELFTEST:-0}
RAILDUNGEON2_SELFTEST=${RAILDUNGEON2_SELFTEST:-0}
RAILEND_SELFTEST=${RAILEND_SELFTEST:-0}

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

# Slice 9: the eighth, six-layer second building part is laid at
# BUILDING12_BASE_Y=116 (spanning y=116..121, above the first descent at
# y=100..111). Its `Q` char resolves to `minecraft:quartz_block` in
# palettes/default.json — a block no other decoded part writes (their palettes
# never name quartz) — so a quartz count at y=112..122 can only come from this
# part. The same fill on the vanilla-noise Overworld must report `No blocks
# were filled`; BUILDING12_SELFTEST inverts it.
say "six-layer second building part authors quartz blocks at the eighth Y"
b12_reply=$(lc "fill 0 112 0 16 122 16 minecraft:air replace minecraft:quartz_block" 2>&1 || true)
b12_chat=$(echo "$b12_reply" | jq -r '.chat[0] // ""' 2>/dev/null || echo "")
echo "quartz second-building fill: $(echo "$b12_reply" | jq -c . 2>/dev/null | head -c 180)"
b12_count=$(echo "$b12_chat" | grep -oE 'Successfully filled [0-9]+ block' | grep -oE '[0-9]+' || echo 0)
b12_count=${b12_count:-0}
echo "second-building-authored quartz count: $b12_count"
vanilla_b12_reply=$(ctl command "fill 0 112 0 16 122 16 minecraft:air replace minecraft:quartz_block" 2>&1 || true)
vanilla_b12_chat=$(echo "$vanilla_b12_reply" | jq -r '.chat[0] // ""' 2>/dev/null || echo "")
echo "vanilla-noise quartz control: $(echo "$vanilla_b12_reply" | jq -c . 2>/dev/null | head -c 180)"
vanilla_b12_count=$(echo "$vanilla_b12_chat" | grep -oE 'Successfully filled [0-9]+ block' | grep -oE '[0-9]+' || echo 0)
vanilla_b12_count=${vanilla_b12_count:-0}
echo "vanilla-noise quartz count: $vanilla_b12_count"
if [ "$BUILDING12_SELFTEST" = 1 ]; then
    [ "$b12_count" -eq 0 ] \
        || fail "BUILDING12_SELFTEST: the part authored $b12_count quartz block(s); the inverted assertion expected 0"
    echo "BUILDING12_SELFTEST: no quartz-authored blocks (unexpected)"
else
    [ "$b12_count" -gt 0 ] \
        || fail "the chunk generator authored no quartz blocks: $b12_reply"
    [ "$vanilla_b12_count" -eq 0 ] \
        || fail "the vanilla-noise control unexpectedly had $vanilla_b12_count quartz block(s): $vanilla_b12_reply"
fi

# Slice 10: the ninth, six-layer city centre is laid at CENTER001_BASE_Y=128
# (spanning y=128..133, above the second building at y=116..121). Its `)`
# char resolves to `minecraft:quartz_stairs[facing=south,half=bottom,shape=straight]`
# from palettes/default.json — a block no other decoded part writes — so a
# quartz-stairs count at y=124..134 can only come from this part. The same fill
# on the vanilla-noise Overworld must report `No blocks were filled`;
# CENTER001_SELFTEST inverts it.
say "six-layer city-centre part authors quartz stairs at the ninth Y"
c001_reply=$(lc "fill 0 124 0 16 134 16 minecraft:air replace minecraft:quartz_stairs" 2>&1 || true)
c001_chat=$(echo "$c001_reply" | jq -r '.chat[0] // ""' 2>/dev/null || echo "")
echo "quartz-stairs city-centre fill: $(echo "$c001_reply" | jq -c . 2>/dev/null | head -c 180)"
c001_count=$(echo "$c001_chat" | grep -oE 'Successfully filled [0-9]+ block' | grep -oE '[0-9]+' || echo 0)
c001_count=${c001_count:-0}
echo "city-centre-authored quartz-stairs count: $c001_count"
vanilla_c001_reply=$(ctl command "fill 0 124 0 16 134 16 minecraft:air replace minecraft:quartz_stairs" 2>&1 || true)
vanilla_c001_chat=$(echo "$vanilla_c001_reply" | jq -r '.chat[0] // ""' 2>/dev/null || echo "")
echo "vanilla-noise quartz-stairs control: $(echo "$vanilla_c001_reply" | jq -c . 2>/dev/null | head -c 180)"
vanilla_c001_count=$(echo "$vanilla_c001_chat" | grep -oE 'Successfully filled [0-9]+ block' | grep -oE '[0-9]+' || echo 0)
vanilla_c001_count=${vanilla_c001_count:-0}
echo "vanilla-noise quartz-stairs count: $vanilla_c001_count"
if [ "$CENTER001_SELFTEST" = 1 ]; then
    [ "$c001_count" -eq 0 ] \
        || fail "CENTER001_SELFTEST: the part authored $c001_count quartz staircase(s); the inverted assertion expected 0"
    echo "CENTER001_SELFTEST: no quartz-stairs-authored blocks (unexpected)"
else
    [ "$c001_count" -gt 0 ] \
        || fail "the chunk generator authored no quartz stairs: $c001_reply"
    [ "$vanilla_c001_count" -eq 0 ] \
        || fail "the vanilla-noise control unexpectedly had $vanilla_c001_count quartz staircase(s): $vanilla_c001_reply"
fi

# Slice 11: the tenth, six-layer shopping-open front is laid at
# SHOPPING1_BASE_Y=140 (spanning y=140..145, above the city centre at y=128..133).
# Its `w` char resolves to `minecraft:cobblestone_wall` in palettes/default.json —
# a block no other decoded part writes — so a cobblestone-wall count at y=136..150
# can only come from this part. The same fill on the vanilla-noise Overworld must
# report `No blocks were filled`; SHOPPING1_SELFTEST inverts it.
say "six-layer shopping-open part authors cobblestone walls at the tenth Y"
shop1_reply=$(lc "fill 0 136 0 16 150 16 minecraft:air replace minecraft:cobblestone_wall" 2>&1 || true)
shop1_chat=$(echo "$shop1_reply" | jq -r '.chat[0] // ""' 2>/dev/null || echo "")
echo "cobblestone-wall shopping-part fill: $(echo "$shop1_reply" | jq -c . 2>/dev/null | head -c 180)"
shop1_count=$(echo "$shop1_chat" | grep -oE 'Successfully filled [0-9]+ block' | grep -oE '[0-9]+' || echo 0)
shop1_count=${shop1_count:-0}
echo "shopping-part-authored cobblestone-wall count: $shop1_count"
vanilla_shop1_reply=$(ctl command "fill 0 136 0 16 150 16 minecraft:air replace minecraft:cobblestone_wall" 2>&1 || true)
vanilla_shop1_chat=$(echo "$vanilla_shop1_reply" | jq -r '.chat[0] // ""' 2>/dev/null || echo "")
echo "vanilla-noise cobblestone-wall control: $(echo "$vanilla_shop1_reply" | jq -c . 2>/dev/null | head -c 180)"
vanilla_shop1_count=$(echo "$vanilla_shop1_chat" | grep -oE 'Successfully filled [0-9]+ block' | grep -oE '[0-9]+' || echo 0)
vanilla_shop1_count=${vanilla_shop1_count:-0}
echo "vanilla-noise cobblestone-wall count: $vanilla_shop1_count"
if [ "$SHOPPING1_SELFTEST" = 1 ]; then
    [ "$shop1_count" -eq 0 ] \
        || fail "SHOPPING1_SELFTEST: the part authored $shop1_count cobblestone wall(s); the inverted assertion expected 0"
    echo "SHOPPING1_SELFTEST: no cobblestone-wall-authored blocks (unexpected)"
else
    [ "$shop1_count" -gt 0 ] \
        || fail "the chunk generator authored no cobblestone walls: $shop1_reply"
    [ "$vanilla_shop1_count" -eq 0 ] \
        || fail "the vanilla-noise control unexpectedly had $vanilla_shop1_count cobblestone wall(s): $vanilla_shop1_reply"
fi

# Slice 12: the eleventh, seven-layer stone stair is laid at STAIRSNORMAL_BASE_Y=152
# (spanning y=152..158, above the shopping front at y=140..145). Its `/` char
# resolves to `minecraft:stone_stairs[facing=west,half=bottom,shape=straight]`
# from palettes/default.json — a block no other decoded part writes — so a
# stone-stairs count at y=152..158 can only come from this part. The same fill
# on the vanilla-noise Overworld must report `No blocks were filled`;
# STAIRS_SELFTEST inverts it.
say "seven-layer stone stair authors stone stairs at the eleventh Y"
stairs_reply=$(lc "fill 0 152 0 16 158 16 minecraft:air replace minecraft:stone_stairs" 2>&1 || true)
stairs_chat=$(echo "$stairs_reply" | jq -r '.chat[0] // ""' 2>/dev/null || echo "")
echo "stone-stairs stair-part fill: $(echo "$stairs_reply" | jq -c . 2>/dev/null | head -c 180)"
stairs_count=$(echo "$stairs_chat" | grep -oE 'Successfully filled [0-9]+ block' | grep -oE '[0-9]+' || echo 0)
stairs_count=${stairs_count:-0}
echo "stair-part-authored stone-stairs count: $stairs_count"
vanilla_stairs_reply=$(ctl command "fill 0 152 0 16 158 16 minecraft:air replace minecraft:stone_stairs" 2>&1 || true)
vanilla_stairs_chat=$(echo "$vanilla_stairs_reply" | jq -r '.chat[0] // ""' 2>/dev/null || echo "")
echo "vanilla-noise stone-stairs control: $(echo "$vanilla_stairs_reply" | jq -c . 2>/dev/null | head -c 180)"
vanilla_stairs_count=$(echo "$vanilla_stairs_chat" | grep -oE 'Successfully filled [0-9]+ block' | grep -oE '[0-9]+' || echo 0)
vanilla_stairs_count=${vanilla_stairs_count:-0}
echo "vanilla-noise stone-stairs count: $vanilla_stairs_count"
if [ "$STAIRS_SELFTEST" = 1 ]; then
    [ "$stairs_count" -eq 0 ] \
        || fail "STAIRS_SELFTEST: the part authored $stairs_count stone stair(s); the inverted assertion expected 0"
    echo "STAIRS_SELFTEST: no stone-stairs-authored blocks (unexpected)"
else
    [ "$stairs_count" -gt 0 ] \
        || fail "the chunk generator authored no stone stairs: $stairs_reply"
    [ "$vanilla_stairs_count" -eq 0 ] \
        || fail "the vanilla-noise control unexpectedly had $vanilla_stairs_count stone stair(s): $vanilla_stairs_reply"
fi

# Slice 13: the twelfth, seven-layer large stone stair is laid at
# STAIRSBIG_BASE_Y=164 (spanning y=164..170, above the first stair at y=152..158).
# It shares the `minecraft:stone_stairs` id with the earlier stair part, so the
# differential is a DISJOINT HEIGHT BAND: the 36 `/` cells of this part are at
# y=164..170 while the earlier part's 24 are at y=152..158 and its fill has
# already replaced them. The same fill on the vanilla-noise Overworld must
# report `No blocks were filled`; STAIRSBIG_SELFTEST inverts the count.
say "seven-layer large stone stair authors stone stairs at the twelfth Y"
stairsbig_reply=$(lc "fill 0 164 0 16 170 16 minecraft:air replace minecraft:stone_stairs" 2>&1 || true)
stairsbig_chat=$(echo "$stairsbig_reply" | jq -r '.chat[0] // ""' 2>/dev/null || echo "")
echo "stone-stairs large-stair-part fill: $(echo "$stairsbig_reply" | jq -c . 2>/dev/null | head -c 180)"
stairsbig_count=$(echo "$stairsbig_chat" | grep -oE 'Successfully filled [0-9]+ block' | grep -oE '[0-9]+' || echo 0)
stairsbig_count=${stairsbig_count:-0}
echo "large-stair-part-authored stone-stairs count: $stairsbig_count"
vanilla_stairsbig_reply=$(ctl command "fill 0 164 0 16 170 16 minecraft:air replace minecraft:stone_stairs" 2>&1 || true)
vanilla_stairsbig_chat=$(echo "$vanilla_stairsbig_reply" | jq -r '.chat[0] // ""' 2>/dev/null || echo "")
echo "vanilla-noise large-stair stone-stairs control: $(echo "$vanilla_stairsbig_reply" | jq -c . 2>/dev/null | head -c 180)"
vanilla_stairsbig_count=$(echo "$vanilla_stairsbig_chat" | grep -oE 'Successfully filled [0-9]+ block' | grep -oE '[0-9]+' || echo 0)
vanilla_stairsbig_count=${vanilla_stairsbig_count:-0}
echo "vanilla-noise large-stair stone-stairs count: $vanilla_stairsbig_count"
if [ "$STAIRSBIG_SELFTEST" = 1 ]; then
    [ "$stairsbig_count" -eq 0 ] \
        || fail "STAIRSBIG_SELFTEST: the part authored $stairsbig_count stone stair(s); the inverted assertion expected 0"
    echo "STAIRSBIG_SELFTEST: no stone-stairs-authored blocks (unexpected)"
else
    [ "$stairsbig_count" -gt 0 ] \
        || fail "the chunk generator authored no large-part stone stairs: $stairsbig_reply"
    [ "$vanilla_stairsbig_count" -eq 0 ] \
        || fail "the vanilla-noise control unexpectedly had $vanilla_stairsbig_count stone stair(s): $vanilla_stairsbig_reply"
fi

# Slice 14: the thirteenth, six-layer third building part is laid at
# BUILDING13_BASE_Y=176 (spanning y=176..181, above the large stair at y=164..170).
# Its `W` char resolves to `minecraft:water` in palettes/common.json — a block no
# other decoded part writes — so a water count at y=176..181 can only come from
# this part. The same fill on the vanilla-noise Overworld must report `No blocks
# were filled`; BUILDING13_SELFTEST inverts the count.
say "six-layer third building part authors water at the thirteenth Y"
b13_reply=$(lc "fill 0 176 0 16 181 16 minecraft:air replace minecraft:water" 2>&1 || true)
b13_chat=$(echo "$b13_reply" | jq -r '.chat[0] // ""' 2>/dev/null || echo "")
echo "water third-building fill: $(echo "$b13_reply" | jq -c . 2>/dev/null | head -c 180)"
b13_count=$(echo "$b13_chat" | grep -oE 'Successfully filled [0-9]+ block' | grep -oE '[0-9]+' || echo 0)
b13_count=${b13_count:-0}
echo "third-building-authored water count: $b13_count"
vanilla_b13_reply=$(ctl command "fill 0 176 0 16 181 16 minecraft:air replace minecraft:water" 2>&1 || true)
vanilla_b13_chat=$(echo "$vanilla_b13_reply" | jq -r '.chat[0] // ""' 2>/dev/null || echo "")
echo "vanilla-noise water control: $(echo "$vanilla_b13_reply" | jq -c . 2>/dev/null | head -c 180)"
vanilla_b13_count=$(echo "$vanilla_b13_chat" | grep -oE 'Successfully filled [0-9]+ block' | grep -oE '[0-9]+' || echo 0)
vanilla_b13_count=${vanilla_b13_count:-0}
echo "vanilla-noise water count: $vanilla_b13_count"
if [ "$BUILDING13_SELFTEST" = 1 ]; then
    [ "$b13_count" -eq 0 ] \
        || fail "BUILDING13_SELFTEST: the part authored $b13_count water block(s); the inverted assertion expected 0"
    echo "BUILDING13_SELFTEST: no water-authored blocks (unexpected)"
else
    [ "$b13_count" -gt 0 ] \
        || fail "the chunk generator authored no water blocks: $b13_reply"
    [ "$vanilla_b13_count" -eq 0 ] \
        || fail "the vanilla-noise control unexpectedly had $vanilla_b13_count water block(s): $vanilla_b13_reply"
fi

# Slice 15: the fourteenth, six-layer second city centre is laid at
# CENTER002_BASE_Y=188 (spanning y=188..193, above the third building at
# y=176..181). Its `C` char resolves to `minecraft:chest[facing=north]` in
# palettes/common.json — a block no other decoded part writes — so a chest
# count at y=188..193 can only come from this part. The same fill on the
# vanilla-noise Overworld must report `No blocks were filled`;
# CENTER002_SELFTEST inverts the count.
say "six-layer second city centre authors a chest at the fourteenth Y"
c002_state=$(lc "execute if block 3 189 11 minecraft:chest[facing=north]" 2>&1 || true)
echo "north-facing chest read-back (informational): $(echo "$c002_state" | jq -c . 2>/dev/null | head -c 180)"
c002_reply=$(lc "fill 0 188 0 16 193 16 minecraft:air replace minecraft:chest" 2>&1 || true)
c002_chat=$(echo "$c002_reply" | jq -r '.chat[0] // ""' 2>/dev/null || echo "")
echo "chest second-centre fill: $(echo "$c002_reply" | jq -c . 2>/dev/null | head -c 180)"
c002_count=$(echo "$c002_chat" | grep -oE 'Successfully filled [0-9]+ block' | grep -oE '[0-9]+' || echo 0)
c002_count=${c002_count:-0}
echo "second-centre-authored chest count: $c002_count"
vanilla_c002_reply=$(ctl command "fill 0 188 0 16 193 16 minecraft:air replace minecraft:chest" 2>&1 || true)
vanilla_c002_chat=$(echo "$vanilla_c002_reply" | jq -r '.chat[0] // ""' 2>/dev/null || echo "")
echo "vanilla-noise chest control: $(echo "$vanilla_c002_reply" | jq -c . 2>/dev/null | head -c 180)"
vanilla_c002_count=$(echo "$vanilla_c002_chat" | grep -oE 'Successfully filled [0-9]+ block' | grep -oE '[0-9]+' || echo 0)
vanilla_c002_count=${vanilla_c002_count:-0}
echo "vanilla-noise chest count: $vanilla_c002_count"
if [ "$CENTER002_SELFTEST" = 1 ]; then
    [ "$c002_count" -eq 0 ] \
        || fail "CENTER002_SELFTEST: the part authored $c002_count chest block(s); the inverted assertion expected 0"
    echo "CENTER002_SELFTEST: no chest-authored blocks (unexpected)"
else
    [ "$c002_count" -gt 0 ] \
        || fail "the chunk generator authored no chest blocks: $c002_reply"
    [ "$vanilla_c002_count" -eq 0 ] \
        || fail "the vanilla-noise control unexpectedly had $vanilla_c002_count chest block(s): $vanilla_c002_reply"
fi

# Slice 16: the fifteenth, six-layer library part is laid at
# LIBRARY1002_BASE_Y=200 (spanning y=200..205, above the second city centre at
# y=188..193). Its `.` char resolves to `minecraft:oak_fence` and its `-` char to
# `minecraft:oak_pressure_plate` in palettes/common.json — two block ids no
# other decoded part writes — so counts at y=200..205 can only come from this
# part. The property/shape read-backs at (5,201,7) and (5,202,7) pin the exact
# states. Each `-` plate sits directly on a `.` fence, so the plate fill must run
# FIRST: removing the supporting fences with air drops the plates in game (a
# measured harness fact, not a port defect). The same fills on the vanilla-noise
# control level must report `No blocks were filled`; LIBRARY1002_SELFTEST inverts
# the fence count.
say "six-layer library part authors an oak fence and pressure plate at the fifteenth Y"
lib_state=$(lc "execute if block 5 201 7 minecraft:oak_fence" 2>&1 || true)
lib_state_chat=$(echo "$lib_state" | jq -r '.chat[0] // ""' 2>/dev/null || echo "")
echo "oak fence read-back at (5,201,7): $(echo "$lib_state" | jq -c . 2>/dev/null | head -c 180)"
plate_state=$(lc "execute if block 5 202 7 minecraft:oak_pressure_plate" 2>&1 || true)
plate_state_chat=$(echo "$plate_state" | jq -r '.chat[0] // ""' 2>/dev/null || echo "")
echo "oak pressure plate read-back at (5,202,7): $(echo "$plate_state" | jq -c . 2>/dev/null | head -c 180)"
if [ "$LIBRARY1002_SELFTEST" != 1 ]; then
    grep -q 'Test passed' <<<"$lib_state_chat" \
        || fail "the library part did not author a fence at (5,201,7): $lib_state"
    grep -q 'Test passed' <<<"$plate_state_chat" \
        || fail "the library part did not author a pressure plate at (5,202,7): $plate_state"
fi
plate_reply=$(lc "fill 0 200 0 16 205 16 minecraft:air replace minecraft:oak_pressure_plate" 2>&1 || true)
plate_chat=$(echo "$plate_reply" | jq -r '.chat[0] // ""' 2>/dev/null || echo "")
echo "oak-pressure-plate library fill: $(echo "$plate_reply" | jq -c . 2>/dev/null | head -c 180)"
plate_count=$(echo "$plate_chat" | grep -oE 'Successfully filled [0-9]+ block' | grep -oE '[0-9]+' || echo 0)
plate_count=${plate_count:-0}
echo "library-authored oak-pressure-plate count: $plate_count"
vanilla_plate_reply=$(ctl command "fill 0 200 0 16 205 16 minecraft:air replace minecraft:oak_pressure_plate" 2>&1 || true)
vanilla_plate_chat=$(echo "$vanilla_plate_reply" | jq -r '.chat[0] // ""' 2>/dev/null || echo "")
echo "vanilla-noise oak-pressure-plate control: $(echo "$vanilla_plate_reply" | jq -c . 2>/dev/null | head -c 180)"
vanilla_plate_count=$(echo "$vanilla_plate_chat" | grep -oE 'Successfully filled [0-9]+ block' | grep -oE '[0-9]+' || echo 0)
vanilla_plate_count=${vanilla_plate_count:-0}
echo "vanilla-noise oak-pressure-plate count: $vanilla_plate_count"
lib_reply=$(lc "fill 0 200 0 16 205 16 minecraft:air replace minecraft:oak_fence" 2>&1 || true)
lib_chat=$(echo "$lib_reply" | jq -r '.chat[0] // ""' 2>/dev/null || echo "")
echo "oak-fence library fill: $(echo "$lib_reply" | jq -c . 2>/dev/null | head -c 180)"
lib_count=$(echo "$lib_chat" | grep -oE 'Successfully filled [0-9]+ block' | grep -oE '[0-9]+' || echo 0)
lib_count=${lib_count:-0}
echo "library-authored oak-fence count: $lib_count"
vanilla_lib_reply=$(ctl command "fill 0 200 0 16 205 16 minecraft:air replace minecraft:oak_fence" 2>&1 || true)
vanilla_lib_chat=$(echo "$vanilla_lib_reply" | jq -r '.chat[0] // ""' 2>/dev/null || echo "")
echo "vanilla-noise oak-fence control: $(echo "$vanilla_lib_reply" | jq -c . 2>/dev/null | head -c 180)"
vanilla_lib_count=$(echo "$vanilla_lib_chat" | grep -oE 'Successfully filled [0-9]+ block' | grep -oE '[0-9]+' || echo 0)
vanilla_lib_count=${vanilla_lib_count:-0}
echo "vanilla-noise oak-fence count: $vanilla_lib_count"
if [ "$LIBRARY1002_SELFTEST" = 1 ]; then
    [ "$lib_count" -eq 0 ] \
        || fail "LIBRARY1002_SELFTEST: the part authored $lib_count oak-fence block(s); the inverted assertion expected 0"
    echo "LIBRARY1002_SELFTEST: no fence-authored blocks (unexpected)"
else
    [ "$lib_count" -gt 0 ] \
        || fail "the chunk generator authored no oak fences: $lib_reply"
    [ "$vanilla_lib_count" -eq 0 ] \
        || fail "the vanilla-noise control unexpectedly had $vanilla_lib_count oak-fence block(s): $vanilla_lib_reply"
    [ "$plate_count" -gt 0 ] \
        || fail "the chunk generator authored no oak pressure plates: $plate_reply"
    [ "$vanilla_plate_count" -eq 0 ] \
        || fail "the vanilla-noise control unexpectedly had $vanilla_plate_count oak-pressure-plate block(s): $vanilla_plate_reply"
fi

# Slice 17: the sixteenth, six-layer rail-dungeon part is laid at
# RAILDUNGEON1_BASE_Y=212 (spanning y=212..217, above the library at
# y=200..205). Its `!` char resolves the jar's weighted treasure list in
# palettes/common.json (diamond 15 / gold 20 / emerald 15 / iron 1000)
# deterministically to `minecraft:iron_block`, a block id no other decoded
# part writes, so a fill-replace count at y=212..217 can only come from this
# part. The exact read-back at (10,214,6) pins the block; iron blocks need no
# supporting neighbour, so the plate-before-fence support order does not apply.
# The same fill on the vanilla-noise control level must report `No blocks were
# filled`; RAILDUNGEON1_SELFTEST inverts the count.
say "six-layer rail dungeon part authors an iron block at the sixteenth Y"
rd_state=$(lc "execute if block 10 214 6 minecraft:iron_block" 2>&1 || true)
rd_state_chat=$(echo "$rd_state" | jq -r '.chat[0] // ""' 2>/dev/null || echo "")
echo "iron-block read-back at (10,214,6): $(echo "$rd_state" | jq -c . 2>/dev/null | head -c 180)"
if [ "$RAILDUNGEON1_SELFTEST" != 1 ]; then
    grep -q 'Test passed' <<<"$rd_state_chat" \
        || fail "the rail dungeon part did not author an iron block at (10,214,6): $rd_state"
fi
rd_reply=$(lc "fill 0 212 0 16 217 16 minecraft:air replace minecraft:iron_block" 2>&1 || true)
rd_chat=$(echo "$rd_reply" | jq -r '.chat[0] // ""' 2>/dev/null || echo "")
echo "rail-dungeon iron-block fill: $(echo "$rd_reply" | jq -c . 2>/dev/null | head -c 180)"
rd_count=$(echo "$rd_chat" | grep -oE 'Successfully filled [0-9]+ block' | grep -oE '[0-9]+' || echo 0)
rd_count=${rd_count:-0}
echo "rail-dungeon-authored iron-block count: $rd_count"
vanilla_rd_reply=$(ctl command "fill 0 212 0 16 217 16 minecraft:air replace minecraft:iron_block" 2>&1 || true)
vanilla_rd_chat=$(echo "$vanilla_rd_reply" | jq -r '.chat[0] // ""' 2>/dev/null || echo "")
echo "vanilla-noise iron-block control: $(echo "$vanilla_rd_reply" | jq -c . 2>/dev/null | head -c 180)"
vanilla_rd_count=$(echo "$vanilla_rd_chat" | grep -oE 'Successfully filled [0-9]+ block' | grep -oE '[0-9]+' || echo 0)
vanilla_rd_count=${vanilla_rd_count:-0}
echo "vanilla-noise iron-block count: $vanilla_rd_count"
if [ "$RAILDUNGEON1_SELFTEST" = 1 ]; then
    [ "$rd_count" -eq 0 ] \
        || fail "RAILDUNGEON1_SELFTEST: the part authored $rd_count iron block(s); the inverted assertion expected 0"
    echo "RAILDUNGEON1_SELFTEST: no iron-block-authored blocks (unexpected)"
else
    [ "$rd_count" -gt 0 ] \
        || fail "the chunk generator authored no iron blocks: $rd_reply"
    [ "$vanilla_rd_count" -eq 0 ] \
        || fail "the vanilla-noise control unexpectedly had $vanilla_rd_count iron block(s): $vanilla_rd_reply"
fi

# Slice 18: the seventeenth, six-layer second rail dungeon is laid at
# RAILDUNGEON2_BASE_Y=224 (spanning y=224..229, above the first dungeon at
# y=212..217). Its `!` char resolves the jar's same weighted treasure list in
# palettes/common.json (diamond 15 / gold 20 / emerald 15 / iron 1000)
# deterministically to `minecraft:iron_block`, but this part has only three `!`
# cells at (9,2,8)/(9,2,9)/(9,2,10). Because the block id is shared with the
# lower dungeon, the differential is a DISJOINT HEIGHT BAND: the earlier leg has
# already replaced the seventeen lower iron blocks, so the y=224..229 fill must
# report exactly three. The exact read-back at (9,226,8) pins the cell; iron
# blocks need no supporting neighbour. The same fill on the vanilla-noise
# control level must report `No blocks were filled`; RAILDUNGEON2_SELFTEST
# inverts the count.
say "six-layer second rail dungeon authors three iron blocks at the seventeenth Y"
rd2_state=$(lc "execute if block 9 226 8 minecraft:iron_block" 2>&1 || true)
rd2_state_chat=$(echo "$rd2_state" | jq -r '.chat[0] // ""' 2>/dev/null || echo "")
echo "iron-block read-back at (9,226,8): $(echo "$rd2_state" | jq -c . 2>/dev/null | head -c 180)"
if [ "$RAILDUNGEON2_SELFTEST" != 1 ]; then
    grep -q 'Test passed' <<<"$rd2_state_chat" \
        || fail "the second rail dungeon did not author an iron block at (9,226,8): $rd2_state"
fi
rd2_reply=$(lc "fill 0 224 0 16 229 16 minecraft:air replace minecraft:iron_block" 2>&1 || true)
rd2_chat=$(echo "$rd2_reply" | jq -r '.chat[0] // ""' 2>/dev/null || echo "")
echo "second-rail-dungeon iron-block fill: $(echo "$rd2_reply" | jq -c . 2>/dev/null | head -c 180)"
rd2_count=$(echo "$rd2_chat" | grep -oE 'Successfully filled [0-9]+ block' | grep -oE '[0-9]+' || echo 0)
rd2_count=${rd2_count:-0}
echo "second-rail-dungeon-authored iron-block count: $rd2_count"
vanilla_rd2_reply=$(ctl command "fill 0 224 0 16 229 16 minecraft:air replace minecraft:iron_block" 2>&1 || true)
vanilla_rd2_chat=$(echo "$vanilla_rd2_reply" | jq -r '.chat[0] // ""' 2>/dev/null || echo "")
echo "vanilla-noise iron-block control (second band): $(echo "$vanilla_rd2_reply" | jq -c . 2>/dev/null | head -c 180)"
vanilla_rd2_count=$(echo "$vanilla_rd2_chat" | grep -oE 'Successfully filled [0-9]+ block' | grep -oE '[0-9]+' || echo 0)
vanilla_rd2_count=${vanilla_rd2_count:-0}
echo "vanilla-noise iron-block count (second band): $vanilla_rd2_count"
if [ "$RAILDUNGEON2_SELFTEST" = 1 ]; then
    [ "$rd2_count" -eq 0 ] \
        || fail "RAILDUNGEON2_SELFTEST: the part authored $rd2_count iron block(s); the inverted assertion expected 0"
    echo "RAILDUNGEON2_SELFTEST: no iron-block-authored blocks (unexpected)"
else
    [ "$rd2_count" -eq 3 ] \
        || fail "the second rail dungeon authored $rd2_count iron block(s), expected exactly 3: $rd2_reply"
    [ "$vanilla_rd2_count" -eq 0 ] \
        || fail "the vanilla-noise control unexpectedly had $vanilla_rd2_count iron block(s): $vanilla_rd2_reply"
fi

# Slice 19: the eighteenth, six-layer rail end cap is laid at RAILEND_BASE_Y=236
# (spanning y=236..241, above rail_dungeon2 at y=224..229). Its `e` char resolves
# through the verbatim palettes/rails.json entry to
# minecraft:rail[shape=east_west], the same id the lower horizontal line writes.
# The lower line's 45 in-chunk cells were already cleared at y=54..59, so the
# y=236..241 fill can only count this part's 33 in-chunk `e` cells plus the three
# the inclusive 17x17 window picks up from the x=16 rim of chunk (1,0) — exactly
# 36, the same rim calibration that makes the lower line read 48 (= 45 + 3).
# The property-exact read-back at (0,237,5) pins one in-chunk `e` cell; the
# vanilla-noise control must report `No blocks were filled`; RAILEND_SELFTEST
# inverts the count.
say "six-layer rail end cap authors east-west rails at the eighteenth Y"
rend_state=$(lc "execute if block 0 237 5 minecraft:rail[shape=east_west]" 2>&1 || true)
rend_state_chat=$(echo "$rend_state" | jq -r '.chat[0] // ""' 2>/dev/null || echo "")
echo "east-west rail read-back at (0,237,5): $(echo "$rend_state" | jq -c . 2>/dev/null | head -c 180)"
if [ "$RAILEND_SELFTEST" != 1 ]; then
    grep -q 'Test passed' <<<"$rend_state_chat" \
        || fail "the rail end cap did not author an east-west rail at (0,237,5): $rend_state"
fi
rend_reply=$(lc "fill 0 236 0 16 241 16 minecraft:air replace minecraft:rail" 2>&1 || true)
rend_chat=$(echo "$rend_reply" | jq -r '.chat[0] // ""' 2>/dev/null || echo "")
echo "rail-end-cap rail fill: $(echo "$rend_reply" | jq -c . 2>/dev/null | head -c 180)"
rend_count=$(echo "$rend_chat" | grep -oE 'Successfully filled [0-9]+ block' | grep -oE '[0-9]+' || echo 0)
rend_count=${rend_count:-0}
echo "rail-end-cap-authored rail count: $rend_count"
vanilla_rend_reply=$(ctl command "fill 0 236 0 16 241 16 minecraft:air replace minecraft:rail" 2>&1 || true)
vanilla_rend_chat=$(echo "$vanilla_rend_reply" | jq -r '.chat[0] // ""' 2>/dev/null || echo "")
echo "vanilla-noise rail control (eighteenth band): $(echo "$vanilla_rend_reply" | jq -c . 2>/dev/null | head -c 180)"
vanilla_rend_count=$(echo "$vanilla_rend_chat" | grep -oE 'Successfully filled [0-9]+ block' | grep -oE '[0-9]+' || echo 0)
vanilla_rend_count=${vanilla_rend_count:-0}
echo "vanilla-noise rail count (eighteenth band): $vanilla_rend_count"
if [ "$RAILEND_SELFTEST" = 1 ]; then
    [ "$rend_count" -eq 0 ] \
        || fail "RAILEND_SELFTEST: the part authored $rend_count rail(s); the inverted assertion expected 0"
    echo "RAILEND_SELFTEST: no rail-authored blocks (unexpected)"
else
    [ "$rend_count" -eq 36 ] \
        || fail "the rail end cap authored $rend_count rail(s), expected exactly 36: $rend_reply"
    [ "$vanilla_rend_count" -eq 0 ] \
        || fail "the vanilla-noise control unexpectedly had $vanilla_rend_count rail(s): $vanilla_rend_reply"
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

echo "PASS: the lostcity dimension answered, all eighteen decoded parts authored their own blocks in it (slabs, bookshelves, east_west rails, the bend's north_south rails, the split's north_east rails, the descent's ascending_east rails, the first descent's iron bars, the second building's quartz blocks, the city centre's quartz stairs, the shopping part's cobblestone walls, the stair part's stone stairs, the large stair's stone stairs in its own height band, the third building's water, the second city centre's north-facing chest, the library's oak fences and oak pressure plates, the rail dungeon's seventeen iron blocks, the second rail dungeon's three iron blocks in its own height band and the rail end cap's east-west rails in its own height band), #lostcities:notbreakable matched bedrock, and both carried loot tables yielded items"
