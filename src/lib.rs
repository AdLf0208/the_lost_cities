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

#[derive(Default)]
struct Mod;

/// The stateless city generator: the three decoded parts plus process-stable
/// block keys, requested once at registration. Two worldgen workers may fill
/// neighbouring chunks at once, so nothing here is mutable.
struct CityGenerator {
    street: Vec<runtime::Placement>,
    building: Vec<runtime::Placement>,
    rails: Vec<runtime::Placement>,
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

        // Request each distinct block id once. The registrar checks it against
        // the game's registry at load; an unknown id refuses the mod by name.
        let mut blocks: Vec<String> = street
            .iter()
            .chain(building.iter())
            .chain(rails.iter())
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
