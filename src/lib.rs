//! The Lost Cities for Horizon 26.3 — the one `impl Hooks` at the crate root.
//!
//! Slice 1 registers the `lostcities:lostcity` dimension from the jar's
//! `dimension_type` values and carries its block tags, loot tables and language
//! file. Slice 2 attaches a deterministic city `ChunkGenerator` to that same
//! dimension: one jar part (`street_straight`) decoded by [`runtime`] into
//! placements, written through the pin's `ChunkAccess` API. All pure values and
//! the JSON decoder live in [`runtime`]; this file is only typed glue to the SDK.
use std::collections::HashMap;

use mod_api::{
    self, BiomeSourceKind, BlockKey, ChunkAccess, ChunkGenerator, DimensionSettings, Hooks,
    Registrar, RegistrationContext, VanillaStages,
};

mod runtime;

/// The name shared by the registered generator and the dimension's generator
/// id. The jar's own dimension file names its custom generator
/// `lostcities:lostcity`; it is manifest-owned, so the pin accepts it.
const CITY_GENERATOR: &str = "lostcities:lostcity";

/// The y at which the street part is laid. Above the Overworld noise surface,
/// so a vanilla-noise control at this y finds no city blocks.
const CITY_BASE_Y: i32 = 90;

/// The second building part is laid below the street. Its six slices span
/// y=72..77, inside the in-game fill window (70..84); the street never writes a
/// bookshelf, so the building's `L` char is the differential assertion.
const BUILDING_BASE_Y: i32 = 72;

/// The third part — the horizontal rail line — is laid below the building. Its
/// six slices span y=54..59, inside the rail fill window (50..64); neither the
/// street nor the building writes a rail, so `minecraft:rail` is differential.
const RAILS_BASE_Y: i32 = 54;

/// The fourth part — the rail bend — is laid below the horizontal line. Its six
/// slices span y=36..41, inside the bend fill window (30..44). It is the only
/// decoded part that writes `north_south` rails: the horizontal line writes
/// `east_west`, so a `north_south` count in the level is the in-game assertion.
const BEND_BASE_Y: i32 = 36;

/// The fifth part — the three-way rail split — is laid below the bend. Its six
/// slices span y=18..23, inside the split fill window (12..26). It is the only
/// decoded part that writes the `north_east` and `south_east` rail shapes: the
/// horizontal line writes `east_west`, the bend `north_south`/`north_west`/
/// `south_west`, so an `north_east` count in the level is the in-game assertion.
const SPLIT_BASE_Y: i32 = 18;

/// The sixth part — the eighteen-layer rail descent — is laid below the split.
/// Its 18 slices span y=4..21, inside the descent fill window (0..24). It is the
/// only decoded part that writes the `ascending_east` rail shapes: the other
/// five write east_west, north_south, north_west, south_west, north_east and
/// south_east, so an `ascending_east` count in the level is the in-game assertion.
const DOWN2_BASE_Y: i32 = 4;

/// The seventh part — the twelve-layer first rail descent — is laid above the
/// street (y=90), at y=100..111. The dimension's min_y=0 leaves no room below
/// the 18-layer descent's y=4..21, so the only free sealed band is above it.
/// Its `:` char resolves to `minecraft:iron_bars`, which no other decoded part
/// writes (their palettes only name iron bars as a `damaged` field), so an
/// iron-bar count at y=96..112 is the in-game assertion.
const DOWN1_BASE_Y: i32 = 100;

/// The eighth part — the second six-layer building — is laid above the first
/// descent at y=116..121, the next sealed band inside the dimension's 256
/// height (the street at 90 and the first descent at 100..111 are below it).
/// Its `Q` char resolves to `minecraft:quartz_block` from `palettes/default.json`,
/// which no other decoded part writes, so a quartz count at y=112..122 is the
/// in-game assertion.
const BUILDING12_BASE_Y: i32 = 116;

#[derive(Default)]
struct Mod;

/// The stateless city generator: the eight decoded parts plus process-stable
/// block keys, requested once at registration. Two worldgen workers may fill
/// neighbouring chunks at once, so nothing here is mutable.
struct CityGenerator {
    street: Vec<runtime::Placement>,
    building: Vec<runtime::Placement>,
    rails: Vec<runtime::Placement>,
    bend: Vec<runtime::Placement>,
    split: Vec<runtime::Placement>,
    down2: Vec<runtime::Placement>,
    down1: Vec<runtime::Placement>,
    building12: Vec<runtime::Placement>,
    keys: HashMap<String, BlockKey>,
}

impl CityGenerator {
    /// Write one decoded part into the chunk at its own base y.
    fn write_part(
        &self,
        chunk: &mut dyn ChunkAccess,
        placements: &[runtime::Placement],
        base_y: i32,
    ) {
        let pos = chunk.pos();
        for place in placements {
            let Some(&key) = self.keys.get(&place.block) else {
                continue;
            };
            let state = if place.properties.is_empty() {
                chunk.state(key)
            } else {
                let properties: Vec<(&str, &str)> = place
                    .properties
                    .iter()
                    .map(|(k, v)| (k.as_str(), v.as_str()))
                    .collect();
                chunk.state_with(key, &properties)
            };
            let world = [
                pos.x.saturating_mul(16).saturating_add(place.dx as i32),
                base_y.saturating_add(place.dy as i32),
                pos.z.saturating_mul(16).saturating_add(place.dz as i32),
            ];
            chunk.set_block(world, state);
        }
    }
}

impl ChunkGenerator for CityGenerator {
    fn build_terrain(&self, chunk: &mut dyn ChunkAccess) {
        if chunk.level() != CITY_GENERATOR {
            return;
        }
        self.write_part(chunk, &self.street, CITY_BASE_Y);
        self.write_part(chunk, &self.building, BUILDING_BASE_Y);
        self.write_part(chunk, &self.rails, RAILS_BASE_Y);
        self.write_part(chunk, &self.bend, BEND_BASE_Y);
        self.write_part(chunk, &self.split, SPLIT_BASE_Y);
        self.write_part(chunk, &self.down2, DOWN2_BASE_Y);
        self.write_part(chunk, &self.down1, DOWN1_BASE_Y);
        self.write_part(chunk, &self.building12, BUILDING12_BASE_Y);
    }

    fn vanilla_stages(&self) -> VanillaStages {
        // The parts are the level's whole terrain: no vanilla surface, carvers,
        // structures or features are applied over them.
        VanillaStages::default()
    }
}

#[mod_api::horizon_mod]
impl Hooks for Mod {
    fn register(&self, registrar: &mut dyn Registrar, _context: &RegistrationContext) {
        let street = runtime::decode_city(runtime::STREET_PART_JSON, runtime::STREET_PALETTE_JSON)
            .expect("the embedded street part and palette decode");
        // The building's `#` char is a `variant` palette entry; resolve it
        // through the jar's variants/stonebrick.json before decoding.
        let variants =
            runtime::VariantTable::from_files(&[("stonebrick", runtime::STONEBRICK_VARIANT_JSON)])
                .expect("the embedded jar variant decodes");
        let building = runtime::decode_city_with_variants(
            runtime::BUILDING_PART_JSON,
            runtime::BUILDING_PALETTE_JSON,
            &variants,
        )
        .expect("the embedded building part and merged palette decode");
        // The rail part's `y` char is the same jar `variant: stonebrick` entry,
        // so the variant table built above resolves it too. Its `e`/`f` chars
        // come from `palettes/rails.json` through the part's `refpalette`.
        let rails = runtime::decode_city_with_variants(
            runtime::RAILS_PART_JSON,
            runtime::RAILS_PALETTE_JSON,
            &variants,
        )
        .expect("the embedded rail part and merged palette decode");
        // The bend reuses the same jar `palettes/rails.json` entries and the
        // same stonebrick variant as the horizontal line; only its part bytes
        // are new. Its `k`/`r`/`s`/`m` chars resolve to rail shapes no other
        // decoded part writes.
        let bend = runtime::decode_city_with_variants(
            runtime::RAILS_BEND_PART_JSON,
            runtime::RAILS_PALETTE_JSON,
            &variants,
        )
        .expect("the embedded rail bend part and merged palette decode");
        // The split reuses the same jar `palettes/rails.json` entries (including
        // the `n`/`q` north_east/south_east shapes no other part writes) and the
        // same stonebrick variant; only its part bytes are new.
        let split = runtime::decode_city_with_variants(
            runtime::RAILS_3SPLIT_PART_JSON,
            runtime::RAILS_PALETTE_JSON,
            &variants,
        )
        .expect("the embedded rail split part and merged palette decode");
        // The descent reuses the same jar `palettes/rails.json` entries — in
        // particular `j`/`i`, the ascending_east rail shapes no other decoded
        // part writes — and the same stonebrick variant; its 18 slices span a
        // taller y range than the earlier six-layer parts.
        let down2 = runtime::decode_city_with_variants(
            runtime::RAILS_DOWN2_PART_JSON,
            runtime::RAILS_PALETTE_JSON,
            &variants,
        )
        .expect("the embedded rail descent part and merged palette decode");
        // The first descent reuses the same jar `palettes/rails.json` entries —
        // in particular `j`, the ascending_east rail — and the same stonebrick
        // variant. Its `:`/`_` chars are the two `palettes/default.json` entries
        // absent from the plain rail palette, carried in a merged palette below.
        let down1 = runtime::decode_city_with_variants(
            runtime::RAILS_DOWN1_PART_JSON,
            runtime::RAILS_DOWN1_PALETTE_JSON,
            &variants,
        )
        .expect("the embedded first descent part and merged palette decode");
        // The eighth part is the second six-layer building. Its `#`/`a`/`@`
        // chars use the standard building palette and stonebrick variant above;
        // `Q` and `:` are the two `palettes/default.json` entries added to its
        // merged palette (quartz_block and iron_bars). No new decoder form, SDK
        // API or manifest key is needed.
        let building12 = runtime::decode_city_with_variants(
            runtime::BUILDING_1_2_PART_JSON,
            runtime::BUILDING_1_2_PALETTE_JSON,
            &variants,
        )
        .expect("the embedded second building part and merged palette decode");

        // Request each distinct block id once. The registrar checks it against
        // the game's registry at load; an unknown id refuses the mod by name.
        let mut blocks: Vec<String> = street
            .iter()
            .chain(building.iter())
            .chain(rails.iter())
            .chain(bend.iter())
            .chain(split.iter())
            .chain(down2.iter())
            .chain(down1.iter())
            .chain(building12.iter())
            .map(|p| p.block.clone())
            .collect();
        blocks.sort();
        blocks.dedup();
        let keys = blocks
            .into_iter()
            .map(|id| {
                let key = registrar.block_key(&id);
                (id, key)
            })
            .collect();
        registrar.register_chunk_generator(
            CITY_GENERATOR,
            CityGenerator {
                street,
                building,
                rails,
                bend,
                split,
                down2,
                down1,
                building12,
                keys,
            },
        );

        // The dimension type's values come from the jar's dimension_type file
        // (`runtime::dimension()`); only the generator and the biome source
        // change. The pin requires `Fixed` for a mod-owned chunk generator.
        let facts = runtime::dimension();
        let settings = DimensionSettings {
            min_y: facts.min_y,
            height: facts.height,
            logical_height: facts.logical_height,
            has_skylight: facts.has_skylight,
            has_ceiling: facts.has_ceiling,
            generator: CITY_GENERATOR.into(),
            noise_settings: facts.noise_settings.into(),
            biome: "minecraft:plains".into(),
            biome_source: BiomeSourceKind::Fixed,
            coordinate_scale: facts.coordinate_scale,
            ambient_light: facts.ambient_light,
            monster_spawn_light_level: facts.monster_spawn_light_level,
            monster_spawn_block_light_limit: facts.monster_spawn_block_light_limit,
            can_start_raid: facts.can_start_raid,
            respawn_anchor_works: facts.respawn_anchor_works,
            nether_portal_spawns_piglin: facts.nether_portal_spawns_piglin,
            sky_color: Some(facts.sky_color.into()),
            fog_color: Some(facts.fog_color.into()),
            cloud_height: Some(facts.cloud_height),
            ..DimensionSettings::default()
        };
        registrar.register_dimension(CITY_GENERATOR, settings);
    }
}
