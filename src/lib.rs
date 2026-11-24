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

/// The y at which the one decoded part is laid. Above the Overworld noise
/// surface, so a vanilla-noise control at this y finds no city blocks.
const CITY_BASE_Y: i32 = 90;

#[derive(Default)]
struct Mod;

/// The stateless city generator: decoded placements plus process-stable block
/// keys, requested once at registration. Two worldgen workers may fill
/// neighbouring chunks at once, so nothing here is mutable.
struct CityGenerator {
    placements: Vec<runtime::Placement>,
    keys: HashMap<String, BlockKey>,
}

impl ChunkGenerator for CityGenerator {
    fn build_terrain(&self, chunk: &mut dyn ChunkAccess) {
        if chunk.level() != CITY_GENERATOR {
            return;
        }
        let pos = chunk.pos();
        for place in &self.placements {
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
                CITY_BASE_Y.saturating_add(place.dy as i32),
                pos.z.saturating_mul(16).saturating_add(place.dz as i32),
            ];
            chunk.set_block(world, state);
        }
    }

    fn vanilla_stages(&self) -> VanillaStages {
        // The part is the level's whole terrain: no vanilla surface, carvers,
        // structures or features are applied over it.
        VanillaStages::default()
    }
}

#[mod_api::horizon_mod]
impl Hooks for Mod {
    fn register(&self, registrar: &mut dyn Registrar, _context: &RegistrationContext) {
        let placements =
            runtime::decode_city(runtime::STREET_PART_JSON, runtime::STREET_PALETTE_JSON)
                .expect("the embedded jar part and palette decode");

        // Request each distinct block id once. The registrar checks it against
        // the game's registry at load; an unknown id refuses the mod by name.
        let mut blocks: Vec<String> = placements.iter().map(|p| p.block.clone()).collect();
        blocks.sort();
        blocks.dedup();
        let keys = blocks
            .into_iter()
            .map(|id| {
                let key = registrar.block_key(&id);
                (id, key)
            })
            .collect();
        registrar.register_chunk_generator(CITY_GENERATOR, CityGenerator { placements, keys });

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
