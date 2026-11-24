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

// --- City data decoder -----------------------------------------------------
//
// The jar's 322 `data/lostcities/lostcities/*.json` files use McJty's private
// `parts`/`palettes` schema, which the pin's `mod_content.rs` worldgen bundle
// does not decode (`blocked(no-city-data-decoder)`). This is a clean-room
// decoder for that schema, written from the observed JSON shape, not translated
// from Java. Slice 1 decodes exactly one part (`street_straight`) and the two
// palette entries it uses; the remaining 207 parts stay blocked.

/// A minimal JSON value, enough for the part/palette files. No dependency is
/// added (registry rule R9: the SDK path crate is the only dependency).
#[derive(Clone, Debug, PartialEq)]
pub enum Json {
    Null,
    Bool(bool),
    Num(f64),
    Str(String),
    Arr(Vec<Json>),
    Obj(Vec<(String, Json)>),
}

impl Json {
    fn get(&self, key: &str) -> Option<&Json> {
        match self {
            Json::Obj(entries) => entries.iter().find(|(k, _)| k == key).map(|(_, v)| v),
            _ => None,
        }
    }

    fn as_usize(&self) -> Option<usize> {
        match self {
            Json::Num(n) if *n >= 0.0 && n.fract() == 0.0 => Some(*n as usize),
            _ => None,
        }
    }

    fn as_str(&self) -> Option<&str> {
        match self {
            Json::Str(s) => Some(s),
            _ => None,
        }
    }

    fn as_arr(&self) -> Option<&[Json]> {
        match self {
            Json::Arr(a) => Some(a),
            _ => None,
        }
    }
}

struct Parser<'a> {
    bytes: &'a [u8],
    pos: usize,
}

impl<'a> Parser<'a> {
    fn new(input: &'a str) -> Self {
        Self {
            bytes: input.as_bytes(),
            pos: 0,
        }
    }

    fn ws(&mut self) {
        while self
            .bytes
            .get(self.pos)
            .is_some_and(u8::is_ascii_whitespace)
        {
            self.pos += 1;
        }
    }

    fn byte(&mut self) -> Result<u8, String> {
        let b = *self.bytes.get(self.pos).ok_or("unexpected end of JSON")?;
        self.pos += 1;
        Ok(b)
    }

    fn expect(&mut self, want: u8) -> Result<(), String> {
        let got = self.byte()?;
        if got == want {
            Ok(())
        } else {
            Err(format!(
                "expected '{}' but found '{}' at {}",
                want as char,
                got as char,
                self.pos - 1
            ))
        }
    }

    fn literal(&mut self, word: &str) -> Result<(), String> {
        if self.bytes[self.pos..].starts_with(word.as_bytes()) {
            self.pos += word.len();
            Ok(())
        } else {
            Err(format!("expected {word} at {}", self.pos))
        }
    }

    fn string(&mut self) -> Result<String, String> {
        self.expect(b'"')?;
        let mut out = Vec::new();
        loop {
            let b = self.byte()?;
            match b {
                b'"' => break,
                b'\\' => {
                    let e = self.byte()?;
                    match e {
                        b'"' => out.push(b'"'),
                        b'\\' => out.push(b'\\'),
                        b'/' => out.push(b'/'),
                        b'n' => out.push(b'\n'),
                        b't' => out.push(b'\t'),
                        b'r' => out.push(b'\r'),
                        _ => return Err(format!("unsupported JSON escape \\{}", e as char)),
                    }
                }
                _ => out.push(b),
            }
        }
        String::from_utf8(out).map_err(|_| "invalid UTF-8 in JSON string".to_string())
    }

    fn number(&mut self) -> Result<Json, String> {
        let start = self.pos;
        while self
            .bytes
            .get(self.pos)
            .is_some_and(|b| b.is_ascii_digit() || matches!(b, b'-' | b'+' | b'.' | b'e' | b'E'))
        {
            self.pos += 1;
        }
        let text = std::str::from_utf8(&self.bytes[start..self.pos])
            .map_err(|_| "invalid JSON number".to_string())?;
        text.parse::<f64>()
            .map(Json::Num)
            .map_err(|_| format!("invalid JSON number: {text}"))
    }

    fn value(&mut self) -> Result<Json, String> {
        self.ws();
        match self
            .bytes
            .get(self.pos)
            .copied()
            .ok_or("unexpected end of JSON")?
        {
            b'{' => {
                self.expect(b'{')?;
                let mut entries = Vec::new();
                self.ws();
                if self.bytes.get(self.pos) == Some(&b'}') {
                    self.pos += 1;
                    return Ok(Json::Obj(entries));
                }
                loop {
                    self.ws();
                    let key = self.string()?;
                    self.ws();
                    self.expect(b':')?;
                    let value = self.value()?;
                    entries.push((key, value));
                    self.ws();
                    match self.byte()? {
                        b',' => continue,
                        b'}' => break,
                        other => {
                            return Err(format!(
                                "expected ',' or '}}' but found '{}'",
                                other as char
                            ));
                        }
                    }
                }
                Ok(Json::Obj(entries))
            }
            b'[' => {
                self.expect(b'[')?;
                let mut items = Vec::new();
                self.ws();
                if self.bytes.get(self.pos) == Some(&b']') {
                    self.pos += 1;
                    return Ok(Json::Arr(items));
                }
                loop {
                    items.push(self.value()?);
                    self.ws();
                    match self.byte()? {
                        b',' => continue,
                        b']' => break,
                        other => {
                            return Err(format!(
                                "expected ',' or ']' but found '{}'",
                                other as char
                            ));
                        }
                    }
                }
                Ok(Json::Arr(items))
            }
            b'"' => self.string().map(Json::Str),
            b't' => {
                self.literal("true")?;
                Ok(Json::Bool(true))
            }
            b'f' => {
                self.literal("false")?;
                Ok(Json::Bool(false))
            }
            b'n' => {
                self.literal("null")?;
                Ok(Json::Null)
            }
            b'-' | b'0'..=b'9' => self.number(),
            other => Err(format!(
                "unexpected byte '{}' at {}",
                other as char, self.pos
            )),
        }
    }
}

/// Parse the small JSON subset the part and palette files use.
pub fn parse_json(input: &str) -> Result<Json, String> {
    let mut p = Parser::new(input);
    let value = p.value()?;
    p.ws();
    if p.pos != p.bytes.len() {
        return Err(format!(
            "{} trailing byte(s) after JSON",
            p.bytes.len() - p.pos
        ));
    }
    Ok(value)
}

/// One part: an `xsize` by `zsize` footprint of `slices` layers, bottom first.
/// Each layer holds exactly `zsize` rows of exactly `xsize` palette characters;
/// a space is air. Measured from `parts/street_straight.json` in the jar.
#[derive(Clone, Debug, PartialEq)]
pub struct Part {
    pub xsize: usize,
    pub zsize: usize,
    pub slices: Vec<Vec<String>>,
}

impl Part {
    pub fn from_json(value: &Json) -> Result<Self, String> {
        let xsize = value
            .get("xsize")
            .and_then(Json::as_usize)
            .ok_or("part: xsize must be a non-negative integer")?;
        let zsize = value
            .get("zsize")
            .and_then(Json::as_usize)
            .ok_or("part: zsize must be a non-negative integer")?;
        if xsize == 0 || zsize == 0 {
            return Err("part: xsize and zsize must both be positive".to_string());
        }
        let layers = value
            .get("slices")
            .and_then(Json::as_arr)
            .ok_or("part: slices must be an array")?;
        let mut slices = Vec::with_capacity(layers.len());
        for (y, layer) in layers.iter().enumerate() {
            let rows = layer
                .as_arr()
                .ok_or_else(|| format!("part: slice {y} is not an array"))?;
            if rows.len() != zsize {
                return Err(format!(
                    "part: slice {y} has {} row(s), expected zsize {zsize}",
                    rows.len()
                ));
            }
            let mut parsed = Vec::with_capacity(zsize);
            for (z, row) in rows.iter().enumerate() {
                let text = row
                    .as_str()
                    .ok_or_else(|| format!("part: slice {y} row {z} is not a string"))?;
                let width = text.chars().count();
                if width != xsize {
                    return Err(format!(
                        "part: slice {y} row {z} has {width} char(s), expected xsize {xsize}"
                    ));
                }
                parsed.push(text.to_string());
            }
            slices.push(parsed);
        }
        if slices.is_empty() {
            return Err("part: no slices".to_string());
        }
        Ok(Self {
            xsize,
            zsize,
            slices,
        })
    }

    /// Resolve every non-space cell into a placement. A char with no palette
    /// entry is an error: a silent skip would drop part of the city.
    pub fn placements(&self, palette: &Palette) -> Result<Vec<Placement>, String> {
        let mut out = Vec::new();
        for (y, layer) in self.slices.iter().enumerate() {
            for (z, row) in layer.iter().enumerate() {
                for (x, ch) in row.chars().enumerate() {
                    if ch == ' ' {
                        continue;
                    }
                    let salt =
                        ((x as u64) << 42) ^ ((y as u64) << 28) ^ ((z as u64) << 8) ^ (ch as u64);
                    let block = palette.resolve(ch, salt).ok_or_else(|| {
                        format!("part: char '{ch}' at ({x},{y},{z}) has no palette entry")
                    })?;
                    let (id, properties) = split_block(block);
                    out.push(Placement {
                        dx: x,
                        dy: y,
                        dz: z,
                        block: id.to_string(),
                        properties,
                    });
                }
            }
        }
        Ok(out)
    }
}

/// How one palette char resolves: one block, or a weighted choice of blocks.
#[derive(Clone, Debug, PartialEq)]
pub enum PaletteEntry {
    Single(String),
    Weighted(Vec<(u32, String)>),
}

/// The `palette` array of one palette file, keyed by its single-character id.
#[derive(Clone, Debug, PartialEq)]
pub struct Palette {
    pub entries: Vec<(char, PaletteEntry)>,
}

impl Palette {
    pub fn from_json(value: &Json) -> Result<Self, String> {
        let list = value
            .get("palette")
            .and_then(Json::as_arr)
            .ok_or("palette: missing palette array")?;
        let mut entries = Vec::new();
        for item in list {
            let text = item
                .get("char")
                .and_then(Json::as_str)
                .ok_or("palette: an entry has no char")?;
            let mut chars = text.chars();
            let ch = chars.next().ok_or("palette: an empty char")?;
            if chars.next().is_some() {
                return Err(format!("palette: char '{text}' is not one character"));
            }
            if let Some(block) = item.get("block").and_then(Json::as_str) {
                entries.push((ch, PaletteEntry::Single(block.to_string())));
            } else if let Some(blocks) = item.get("blocks").and_then(Json::as_arr) {
                let mut choices = Vec::with_capacity(blocks.len());
                for choice in blocks {
                    let weight = choice
                        .get("random")
                        .and_then(Json::as_usize)
                        .ok_or("palette: a weighted entry has no random weight")?;
                    let block = choice
                        .get("block")
                        .and_then(Json::as_str)
                        .ok_or("palette: a weighted entry has no block")?;
                    choices.push((weight as u32, block.to_string()));
                }
                if choices.is_empty() {
                    return Err(format!("palette: char '{ch}' has an empty weighted list"));
                }
                entries.push((ch, PaletteEntry::Weighted(choices)));
            } else {
                return Err(format!("palette: char '{ch}' has neither block nor blocks"));
            }
        }
        Ok(Self { entries })
    }

    /// Resolve `ch` to a block id, using `salt` for a stable weighted pick.
    pub fn resolve(&self, ch: char, salt: u64) -> Option<&str> {
        let entry = self
            .entries
            .iter()
            .find(|(c, _)| *c == ch)
            .map(|(_, e)| e)?;
        match entry {
            PaletteEntry::Single(block) => Some(block),
            PaletteEntry::Weighted(choices) => {
                let total: u64 = choices.iter().map(|(w, _)| u64::from(*w)).sum();
                if total == 0 {
                    return choices.first().map(|(_, b)| b.as_str());
                }
                let mut pick =
                    (0xcbf2_9ce4_8422_2325_u64 ^ salt).wrapping_mul(0x0100_0000_01b3) % total;
                for (weight, block) in choices {
                    if pick < u64::from(*weight) {
                        return Some(block);
                    }
                    pick -= u64::from(*weight);
                }
                choices.last().map(|(_, b)| b.as_str())
            }
        }
    }
}

/// One block to write, relative to its part's footprint.
#[derive(Clone, Debug, PartialEq)]
pub struct Placement {
    pub dx: usize,
    pub dy: usize,
    pub dz: usize,
    pub block: String,
    pub properties: Vec<(String, String)>,
}

/// Split `minecraft:block[a=b,c=d]` into its id and property pairs.
pub fn split_block(text: &str) -> (&str, Vec<(String, String)>) {
    if let Some(open) = text.find('[') {
        let id = &text[..open];
        let body = text[open + 1..].trim_end_matches(']');
        let properties = body
            .split(',')
            .filter_map(|pair| {
                let mut parts = pair.splitn(2, '=');
                Some((parts.next()?.to_string(), parts.next()?.to_string()))
            })
            .collect();
        (id, properties)
    } else {
        (text, Vec::new())
    }
}

/// Decode one part with one palette into worldgen placements.
pub fn decode_city(part_json: &str, palette_json: &str) -> Result<Vec<Placement>, String> {
    let part = Part::from_json(&parse_json(part_json)?)?;
    let palette = Palette::from_json(&parse_json(palette_json)?)?;
    part.placements(&palette)
}

/// `parts/street_straight.json` from the cached jar, verbatim: a 16x16 single
/// layer whose middle eight rows are the street char `S` and whose borders are
/// `b`. Both chars are defined by `palettes/default.json` below.
pub const STREET_PART_JSON: &str = r#"{
  "xsize": 16,
  "zsize": 16,
  "slices": [
    [
      "bbbbbbbbbbbbbbbb",
      "bbbbbbbbbbbbbbbb",
      "bbbbbbbbbbbbbbbb",
      "bbbbbbbbbbbbbbbb",
      "SSSSSSSSSSSSSSSS",
      "SSSSSSSSSSSSSSSS",
      "SSSSSSSSSSSSSSSS",
      "SSSSSSSSSSSSSSSS",
      "SSSSSSSSSSSSSSSS",
      "SSSSSSSSSSSSSSSS",
      "SSSSSSSSSSSSSSSS",
      "SSSSSSSSSSSSSSSS",
      "bbbbbbbbbbbbbbbb",
      "bbbbbbbbbbbbbbbb",
      "bbbbbbbbbbbbbbbb",
      "bbbbbbbbbbbbbbbb"
    ]
  ]
}"#;

/// The `S` and `b` entries of `palettes/default.json`, verbatim (the `damaged`
/// field is carried for provenance; the decoder only needs `char` and `block`).
pub const STREET_PALETTE_JSON: &str = r#"{
  "palette": [
    {
      "char": "S",
      "block": "minecraft:smooth_stone_slab[type=double]",
      "damaged": "minecraft:iron_bars"
    },
    {
      "char": "b",
      "block": "minecraft:structure_void"
    }
  ]
}"#;

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

#[cfg(test)]
mod decoder_tests {
    use super::*;

    // A part whose single row is one char short of its declared xsize.
    const SHORT_ROW: &str = r#"{"xsize":4,"zsize":1,"slices":[[ "abc" ]]}"#;
    // A part whose single slice has one row too few for its declared zsize.
    const SHORT_SLICE: &str = r#"{"xsize":4,"zsize":2,"slices":[[ "abcD" ]]}"#;
    // A part with a char the palette does not define.
    const UNMAPPED: &str = r#"{"xsize":4,"zsize":1,"slices":[[ "ab?D" ]]}"#;
    const TINY_PAL: &str = r#"{"palette":[{"char":"a","block":"minecraft:stone"},{"char":"b","block":"minecraft:stone"}]}"#;
    const WEIGHTED_PAL: &str = r#"{"palette":[{"char":"a","blocks":[{"random":32,"block":"minecraft:stone"},{"random":1000,"block":"minecraft:dirt"}]}]}"#;

    #[test]
    fn part_rejects_a_row_shorter_than_xsize() {
        let j = parse_json(SHORT_ROW).expect("test JSON parses");
        let err = Part::from_json(&j).expect_err("a short row must be refused");
        assert!(err.contains("expected xsize 4"), "wrong refusal: {err}");
    }

    #[test]
    fn part_rejects_a_slice_with_too_few_rows() {
        let j = parse_json(SHORT_SLICE).expect("test JSON parses");
        let err = Part::from_json(&j).expect_err("a short slice must be refused");
        assert!(err.contains("expected zsize 2"), "wrong refusal: {err}");
    }

    #[test]
    fn decode_refuses_a_char_absent_from_the_palette() {
        let err = decode_city(UNMAPPED, TINY_PAL)
            .expect_err("an unmapped char must be refused, not silently skipped");
        assert!(err.contains("no palette entry"), "wrong refusal: {err}");
    }

    #[test]
    fn embedded_street_part_decodes_to_the_palette_slab() {
        let placements = decode_city(STREET_PART_JSON, STREET_PALETTE_JSON)
            .expect("the embedded jar part decodes");
        // All 256 cells are non-space: 8 street rows of `S` and 8 border rows
        // of `b`. The street rows resolve to the double slab; the borders to
        // structure_void, exactly as `palettes/default.json` declares.
        assert_eq!(placements.len(), 256, "16 rows x 16 columns");
        assert_eq!(
            placements
                .iter()
                .filter(|p| p.block == "minecraft:smooth_stone_slab")
                .count(),
            128,
            "8 street rows x 16 columns"
        );
        assert_eq!(
            placements
                .iter()
                .filter(|p| p.block == "minecraft:structure_void")
                .count(),
            128,
            "8 border rows x 16 columns"
        );
        assert!(placements.iter().all(|p| p.dy == 0));
        let street = placements
            .iter()
            .find(|p| p.block == "minecraft:smooth_stone_slab")
            .unwrap();
        assert!((4..12).contains(&street.dz));
        assert_eq!(
            street.properties,
            vec![("type".to_string(), "double".to_string())]
        );
    }

    #[test]
    fn weighted_palette_choice_is_deterministic() {
        let p = Palette::from_json(&parse_json(WEIGHTED_PAL).expect("test JSON parses"))
            .expect("weighted palette parses");
        for salt in 0..64 {
            assert_eq!(p.resolve('a', salt), p.resolve('a', salt));
        }
    }

    #[test]
    fn block_properties_are_split_from_the_id() {
        let (id, props) = split_block("minecraft:redstone_torch[lit=true]");
        assert_eq!(id, "minecraft:redstone_torch");
        assert_eq!(props, vec![("lit".to_string(), "true".to_string())]);
    }
}
