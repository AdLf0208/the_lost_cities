//! Pure logic for The Lost Cities port: the values of the jar's
//! `data/lostcities/dimension_type/lostcity.json` and `dimension/lostcity.json`,
//! unpacked into plain data so they can be checked with `cargo test` alone.
//!
//! No `mod_api` or game types live here. `src/lib.rs` maps these facts into one
//! `DimensionSettings` and registers it as `lostcities:lostcity`.

/// The dimension values read from the jar (1.21.1 NeoForge `26.3-12.0.2`).
#[derive(Clone, Debug, PartialEq)]
pub struct DimensionFacts {
    pub min_y: i32,
    pub height: i32,
    pub logical_height: i32,
    pub has_skylight: bool,
    pub has_ceiling: bool,
    pub coordinate_scale: f64,
    pub ambient_light: f32,
    pub monster_spawn_light_level: (i32, i32),
    pub monster_spawn_block_light_limit: i32,
    pub can_start_raid: bool,
    pub respawn_anchor_works: bool,
    pub nether_portal_spawns_piglin: bool,
    pub sky_color: &'static str,
    pub fog_color: &'static str,
    pub cloud_height: f32,
    pub generator: &'static str,
    pub noise_settings: &'static str,
    pub biome_preset: &'static str,
}

impl Default for DimensionFacts {
    fn default() -> Self {
        Self {
            min_y: 0,
            height: 0,
            logical_height: 0,
            has_skylight: false,
            has_ceiling: false,
            coordinate_scale: 0.0,
            ambient_light: 0.0,
            monster_spawn_light_level: (0, 0),
            monster_spawn_block_light_limit: 0,
            can_start_raid: false,
            respawn_anchor_works: false,
            nether_portal_spawns_piglin: false,
            sky_color: "",
            fog_color: "",
            cloud_height: 0.0,
            generator: "",
            noise_settings: "",
            biome_preset: "",
        }
    }
}

/// The jar's dimension facts, read from `dimension_type/lostcity.json` and
/// `dimension/lostcity.json`. The generator is the literal one the dimension
/// file declares; the Java mod's mixin replaces it with the city generator at
/// runtime, which this pin cannot express (`blocked(level_reset:R11)`).
#[must_use]
pub fn dimension() -> DimensionFacts {
    DimensionFacts {
        min_y: 0,
        height: 256,
        logical_height: 256,
        has_skylight: true,
        has_ceiling: false,
        coordinate_scale: 1.0,
        ambient_light: 0.0,
        // The jar declares a constant 2, represented as a degenerate range.
        monster_spawn_light_level: (2, 2),
        monster_spawn_block_light_limit: 2,
        can_start_raid: false,
        respawn_anchor_works: true,
        nether_portal_spawns_piglin: true,
        sky_color: "#78a7ff",
        fog_color: "#c0d8ff",
        cloud_height: 192.33,
        generator: "minecraft:noise",
        noise_settings: "minecraft:overworld",
        biome_preset: "minecraft:overworld",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dimension_type_matches_the_carried_jar() {
        // Every value below is read verbatim from
        // `data/lostcities/dimension_type/lostcity.json` in the cached jar; the
        // generator triple is from `data/lostcities/dimension/lostcity.json`.
        let d = dimension();
        assert_eq!(d.min_y, 0, "wrong min_y");
        assert_eq!(d.height, 256, "wrong height");
        assert_eq!(d.logical_height, 256, "wrong logical_height");
        assert!(d.has_skylight, "wrong has_skylight");
        assert!(!d.has_ceiling, "wrong has_ceiling");
        assert_eq!(d.coordinate_scale, 1.0, "wrong coordinate_scale");
        assert_eq!(d.ambient_light, 0.0, "wrong ambient_light");
        // The jar's `monster_spawn_light_level` is the constant 2, not a range.
        assert_eq!(
            d.monster_spawn_light_level,
            (2, 2),
            "wrong spawn light level"
        );
        assert_eq!(
            d.monster_spawn_block_light_limit, 2,
            "wrong block light limit"
        );
        assert!(!d.can_start_raid, "wrong can_start_raid");
        assert!(d.respawn_anchor_works, "wrong respawn_anchor_works");
        assert!(
            d.nether_portal_spawns_piglin,
            "wrong nether_portal_spawns_piglin"
        );
        assert_eq!(d.sky_color, "#78a7ff", "wrong sky_color");
        assert_eq!(d.fog_color, "#c0d8ff", "wrong fog_color");
        assert_eq!(d.cloud_height, 192.33, "wrong cloud_height");
        assert_eq!(d.generator, "minecraft:noise", "wrong generator");
        assert_eq!(
            d.noise_settings, "minecraft:overworld",
            "wrong noise_settings"
        );
        assert_eq!(d.biome_preset, "minecraft:overworld", "wrong biome_preset");
    }

    #[test]
    fn dimension_height_is_a_multiple_of_16_in_the_jar() {
        let d = dimension();
        assert_eq!(
            d.height % 16,
            0,
            "the pin's DimensionSettings codec refuses this height"
        );
        assert!(
            d.height > 0 && d.logical_height <= d.height,
            "invalid dimension height"
        );
    }
}
