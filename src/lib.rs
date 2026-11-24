//! The Lost Cities for Horizon 26.3 — the one `impl Hooks` at the crate root.
//!
//! Slice 1 registers the `lostcities:lostcity` dimension from the jar's
//! `dimension_type` values and carries its block tags, loot tables and language
//! file. All pure values live in [`runtime`]; this file is only typed glue to the SDK.
use mod_api::{self, BiomeSourceKind, DimensionSettings, Hooks, Registrar, RegistrationContext};

mod runtime;

#[derive(Default)]
struct Mod;

#[mod_api::horizon_mod]
impl Hooks for Mod {
    fn register(&self, registrar: &mut dyn Registrar, _context: &RegistrationContext) {
        // The dimension type is registered from the jar's values, not from a
        // data pack: `data/<ns>/dimension*/` is not read at this pin, and the
        // jar's dimension type is a 1.21.1 codec that 26.3 would not accept.
        let facts = runtime::dimension();
        let settings = DimensionSettings {
            min_y: facts.min_y,
            height: facts.height,
            logical_height: facts.logical_height,
            has_skylight: facts.has_skylight,
            has_ceiling: facts.has_ceiling,
            generator: facts.generator.into(),
            noise_settings: facts.noise_settings.into(),
            biome_source: BiomeSourceKind::MultiNoise,
            biome_preset: facts.biome_preset.into(),
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
        registrar.register_dimension("lostcities:lostcity", settings);
    }
}
