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
    pub fn placements(
        &self,
        palette: &Palette,
        variants: &VariantTable,
    ) -> Result<Vec<Placement>, String> {
        let mut out = Vec::new();
        for (y, layer) in self.slices.iter().enumerate() {
            for (z, row) in layer.iter().enumerate() {
                for (x, ch) in row.chars().enumerate() {
                    if ch == ' ' {
                        continue;
                    }
                    let salt =
                        ((x as u64) << 42) ^ ((y as u64) << 28) ^ ((z as u64) << 8) ^ (ch as u64);
                    let block = palette.resolve(ch, salt, variants).ok_or_else(|| {
                        format!("part: char '{ch}' at ({x},{y},{z}) has no palette entry")
                    })?;
                    let (id, properties) = split_block(&block);
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

/// How one palette char resolves: one block, a weighted choice of blocks, a
/// named jar variant, or another char's resolved block (`frompalette`).
#[derive(Clone, Debug, PartialEq)]
pub enum PaletteEntry {
    Single(String),
    Weighted(Vec<(u32, String)>),
    Variant(String),
    FromPalette(char),
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
                entries.push((ch, PaletteEntry::Weighted(read_choices(blocks)?)));
            } else if let Some(name) = item.get("variant").and_then(Json::as_str) {
                entries.push((ch, PaletteEntry::Variant(name.to_string())));
            } else if let Some(from) = item.get("frompalette").and_then(Json::as_str) {
                let mut fc = from.chars();
                let fch = fc.next().ok_or("palette: empty frompalette")?;
                if fc.next().is_some() {
                    return Err(format!(
                        "palette: frompalette '{from}' is not one character"
                    ));
                }
                entries.push((ch, PaletteEntry::FromPalette(fch)));
            } else {
                return Err(format!(
                    "palette: char '{ch}' has neither block, blocks, variant nor frompalette"
                ));
            }
        }
        Ok(Self { entries })
    }

    /// Resolve `ch` to a block id, using `salt` for a stable weighted or variant
    /// pick. A `frompalette` entry follows another char, with a cycle bound.
    pub fn resolve(&self, ch: char, salt: u64, variants: &VariantTable) -> Option<String> {
        self.resolve_inner(ch, salt, variants, 0)
    }

    fn resolve_inner(
        &self,
        ch: char,
        salt: u64,
        variants: &VariantTable,
        depth: u8,
    ) -> Option<String> {
        if depth > 8 {
            return None;
        }
        let entry = self
            .entries
            .iter()
            .find(|(c, _)| *c == ch)
            .map(|(_, e)| e)?;
        match entry {
            PaletteEntry::Single(block) => Some(block.clone()),
            PaletteEntry::Weighted(choices) => pick_weighted(choices, salt).map(str::to_string),
            PaletteEntry::Variant(name) => variants.pick(name, salt).map(str::to_string),
            PaletteEntry::FromPalette(other) => {
                self.resolve_inner(*other, salt, variants, depth + 1)
            }
        }
    }
}

/// Read the jar's `{"random": N, "block": "..."}` weighted-block lists.
fn read_choices(blocks: &[Json]) -> Result<Vec<(u32, String)>, String> {
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
        return Err("palette: a weighted list is empty".to_string());
    }
    Ok(choices)
}

/// A deterministic pick from a jar weighted-block list.
fn pick_weighted(choices: &[(u32, String)], salt: u64) -> Option<&str> {
    let total: u64 = choices.iter().map(|(w, _)| u64::from(*w)).sum();
    if total == 0 {
        return choices.first().map(|(_, b)| b.as_str());
    }
    let mut pick = (0xcbf2_9ce4_8422_2325_u64 ^ salt).wrapping_mul(0x0100_0000_01b3) % total;
    for (weight, block) in choices {
        if pick < u64::from(*weight) {
            return Some(block);
        }
        pick -= u64::from(*weight);
    }
    choices.last().map(|(_, b)| b.as_str())
}

/// The jar's `variants/<name>.json` files: each is a `{"blocks": [...]}` list,
/// keyed here by the file name a palette's `variant` field refers to.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct VariantTable {
    entries: Vec<(String, Vec<(u32, String)>)>,
}

impl VariantTable {
    pub fn from_files(files: &[(&str, &str)]) -> Result<Self, String> {
        let mut entries = Vec::with_capacity(files.len());
        for (name, json) in files {
            let value = parse_json(json)?;
            let blocks = value
                .get("blocks")
                .and_then(Json::as_arr)
                .ok_or_else(|| format!("variant '{name}': missing blocks array"))?;
            entries.push(((*name).to_string(), read_choices(blocks)?));
        }
        Ok(Self { entries })
    }

    fn pick(&self, name: &str, salt: u64) -> Option<&str> {
        let choices = self
            .entries
            .iter()
            .find(|(n, _)| n == name)
            .map(|(_, c)| c)?;
        pick_weighted(choices, salt)
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

/// Decode one part with one palette into worldgen placements. The street part
/// has no variant entries, so an empty variant table is enough here.
pub fn decode_city(part_json: &str, palette_json: &str) -> Result<Vec<Placement>, String> {
    decode_city_with_variants(part_json, palette_json, &VariantTable::default())
}

/// Decode a part whose palette may contain `variant`/`frompalette` entries.
pub fn decode_city_with_variants(
    part_json: &str,
    palette_json: &str,
    variants: &VariantTable,
) -> Result<Vec<Placement>, String> {
    let part = Part::from_json(&parse_json(part_json)?)?;
    let palette = Palette::from_json(&parse_json(palette_json)?)?;
    part.placements(&palette, variants)
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

/// `variants/stonebrick.json` from the cached jar, verbatim. A `variant`
/// palette entry resolves through this weighted stone-brick list.
pub const STONEBRICK_VARIANT_JSON: &str = "{\n  \"blocks\": [\n    {\n      \"random\": 9,\n      \"block\": \"minecraft:cracked_stone_bricks\"\n    },\n    {\n      \"random\": 8,\n      \"block\": \"minecraft:mossy_stone_bricks\"\n    },\n    {\n      \"random\": 1000,\n      \"block\": \"minecraft:stone_bricks\"\n    }\n  ]\n}";

/// A one-cell part exercising only the `variant` entry form (bricks_standard
/// char `#`), for the red-first variant-resolution test.
#[cfg(test)]
pub const VARIANT_PART_JSON: &str = "{\"xsize\": 1, \"zsize\": 1, \"slices\": [[\"#\"]]}";
/// The `#` entry of `palettes/bricks_standard.json`, verbatim.
#[cfg(test)]
pub const VARIANT_PALETTE_JSON: &str = "{\"palette\": [{\"char\": \"#\", \"variant\": \"stonebrick\", \"damaged\": \"minecraft:iron_bars\"}]}";

/// `parts/building1_1.json` from the cached jar, verbatim: 16x16, six slices.
pub const BUILDING_PART_JSON: &str = "{\n  \"xsize\": 16,\n  \"zsize\": 16,\n  \"slices\": [\n    [\n      \"################\",\n      \"################\",\n      \"################\",\n      \"################\",\n      \"################\",\n      \"################\",\n      \"################\",\n      \"################\",\n      \"################\",\n      \"#######l########\",\n      \"##########1#####\",\n      \"################\",\n      \"################\",\n      \"################\",\n      \"################\",\n      \"################\"\n    ],\n    [\n      \"#aa@aaaaaaaa@aa#\",\n      \"a  #        #  a\",\n      \"a  #F       #  a\",\n      \"@  #F      L#  @\",\n      \"a  #D      L#  a\",\n      \"a  #        #  a\",\n      \"a              a\",\n      \"a  #        #  a\",\n      \"a  #        #  a\",\n      \"a  #   l    #  a\",\n      \"a  ##########  a\",\n      \"a              a\",\n      \"@              @\",\n      \"a              a\",\n      \"a     F        a\",\n      \"#aa@aaaaaaaa@aa#\"\n    ],\n    [\n      \"#aa@aaaaaaaa@aa#\",\n      \"a  #        #  a\",\n      \"a  #        #  a\",\n      \"@  #       L#  @\",\n      \"a  #*      L#  a\",\n      \"a  #        #  a\",\n      \"a              a\",\n      \"a  #        #  a\",\n      \"a  #        #  a\",\n      \"a  #   l    #  a\",\n      \"a  ##########  a\",\n      \"a       T      a\",\n      \"@              @\",\n      \"a              a\",\n      \"a              a\",\n      \"#aa@aaaaaaaa@aa#\"\n    ],\n    [\n      \"#aa@aaaaaaaa@aa#\",\n      \"a  #        #  a\",\n      \"a  #        #  a\",\n      \"@  #        #  @\",\n      \"a  #        #  a\",\n      \"a  #        #  a\",\n      \"a  #        #  a\",\n      \"a  #        #  a\",\n      \"a  #        #  a\",\n      \"a  #   l    #  a\",\n      \"a  ##########  a\",\n      \"a              a\",\n      \"@              @\",\n      \"a              a\",\n      \"a              a\",\n      \"#aa@aaaaaaaa@aa#\"\n    ],\n    [\n      \"################\",\n      \"################\",\n      \"################\",\n      \"################\",\n      \"################\",\n      \"################\",\n      \"################\",\n      \"################\",\n      \"################\",\n      \"#######l########\",\n      \"################\",\n      \"################\",\n      \"################\",\n      \"################\",\n      \"################\",\n      \"################\"\n    ],\n    [\n      \"################\",\n      \"################\",\n      \"################\",\n      \"################\",\n      \"################\",\n      \"################\",\n      \"################\",\n      \"################\",\n      \"################\",\n      \"#######l########\",\n      \"################\",\n      \"################\",\n      \"################\",\n      \"################\",\n      \"################\",\n      \"################\"\n    ]\n  ]\n}";

/// Merged palette for the building part. Every entry is copied verbatim from
/// its jar source: `#` from bricks_standard (a `variant`), `a` from glass_full,
/// `@` from glass_side_variant_bricks (`frompalette: "#"`), and `1`/`T`/`F`/
/// `D`/`l`/`L`/`*`/space from common. `styles/standard.json` selects exactly
/// these palette groups together, so this is the jar's own char -> block map.
pub const BUILDING_PALETTE_JSON: &str = "{\"palette\": [{\"char\": \"#\", \"variant\": \"stonebrick\", \"damaged\": \"minecraft:iron_bars\"}, {\"char\": \"a\", \"block\": \"minecraft:glass\"}, {\"char\": \"@\", \"frompalette\": \"#\"}, {\"char\": \"1\", \"block\": \"minecraft:spawner\", \"mob\": \"easymobs\"}, {\"char\": \"T\", \"block\": \"minecraft:wall_torch[facing=north]\", \"torch\": true}, {\"char\": \"F\", \"blocks\": [{\"random\": 25, \"block\": \"minecraft:furnace[facing=north]\"}, {\"random\": 20, \"block\": \"minecraft:crafting_table\"}, {\"random\": 7, \"block\": \"minecraft:brewing_stand\"}, {\"random\": 6, \"block\": \"minecraft:anvil[facing=north]\"}, {\"random\": 5, \"block\": \"minecraft:cauldron\"}, {\"random\": 2, \"block\": \"minecraft:enchanting_table\"}, {\"random\": 1000, \"block\": \"minecraft:cobweb\"}]}, {\"char\": \"D\", \"block\": \"minecraft:dirt\"}, {\"char\": \"l\", \"block\": \"minecraft:ladder[facing=north]\"}, {\"char\": \"L\", \"block\": \"minecraft:bookshelf\"}, {\"char\": \"*\", \"block\": \"minecraft:potted_cactus\"}, {\"char\": \" \", \"block\": \"minecraft:air\"}]}";

/// `parts/rails_horizontal.json` from the cached jar, verbatim: 16x16, six
/// slices, `refpalette: "rails"`. The exact non-space chars are `T`, `e`, `f`,
/// `g`, `y`, `~` — the same set the brief names, read from the artefact first.
pub const RAILS_PART_JSON: &str = r#"{
  "xsize": 16,
  "zsize": 16,
  "refpalette": "rails",
  "slices": [
    [
      "y              y",
      "y              y",
      "y              y",
      "yyyyyyyyyyyyyyyy",
      "yyyyyyyyyyyyyyyy",
      "yyyyyyyyyyyyyyyy",
      "yyyyyyyyyyyyyyyy",
      "yyyyyyyyyyyyyyyy",
      "yyyyyyyyyyyyyyyy",
      "yyyyyyyyyyyyyyyy",
      "yyyyyyyyyyyyyyyy",
      "yyyyyyyyyyyyyyyy",
      "y              y",
      "y              y",
      "y              y",
      "y               "
    ],
    [
      "y              y",
      "y              y",
      "y              y",
      "yyyyyyyyyyyyyyyy",
      "yyyyyyyyyyyyyyyy",
      "eeeeeeefeeeeeeee",
      "yyyyyyygyyyyyyyy",
      "eeeeeeefeeeeeeee",
      "yyyyyyygyyyyyyyy",
      "eeeeeeefeeeeeeee",
      "yyyyyyyyyyyyyyyy",
      "yyyyyyyyyyyyyyyy",
      "y              y",
      "y              y",
      "y              y",
      "y               "
    ],
    [
      "y              y",
      "y              y",
      "y              y",
      "yyyyyyyyyyyyyyyy",
      "~~~~~~~~~~~~~~~~",
      "~~~~~~~~~~~~~~~~",
      "~~~~~~~~~~~~~~~~",
      "~~~~~~~~~~~~~~~~",
      "~~~~~~~~~~~~~~~~",
      "~~~~~~~~~~~~~~~~",
      "~~~~~~~~~~~~~~~~",
      "yyyyyyyyyyyyyyyy",
      "y              y",
      "y              y",
      "y              y",
      "y               "
    ],
    [
      "y              y",
      "y              y",
      "y              y",
      "yyyyyyyyyyyyyyyy",
      "~~~~~~~~~~~~~~~~",
      "~~~~~~~~~~~~~~~~",
      "~~~~~~~~~~~~~~~~",
      "~~~~~~~~~~~~~~~~",
      "~~~~~~~~~~~~~~~~",
      "~~~~~~~~~~~~~~~~",
      "~~~~~~~~~~~~~~~~",
      "yyyyyyyyyyyyyyyy",
      "y              y",
      "y              y",
      "y              y",
      "y               "
    ],
    [
      "y              y",
      "y              y",
      "y              y",
      "yyyyyyyyyyyyyyyy",
      "~~~~~~~~T~~~~~~~",
      "~~~~~~~~~~~~~~~~",
      "~~~~~~~~~~~~~~~~",
      "~~~~~~~~~~~~~~~~",
      "~~~~~~~~~~~~~~~~",
      "~~~~~~~~~~~~~~~~",
      "~~~~~~~~~~~~~~~~",
      "yyyyyyyyyyyyyyyy",
      "y              y",
      "y              y",
      "y              y",
      "y               "
    ],
    [
      "y              y",
      "y              y",
      "y              y",
      "yyyyyyyyyyyyyyyy",
      "yyyyyyyyyyyyyyyy",
      "yyyyyyyyyyyyyyyy",
      "yyyyyyyyyyyyyyyy",
      "yyyyyyyyyyyyyyyy",
      "yyyyyyyyyyyyyyyy",
      "yyyyyyyyyyyyyyyy",
      "yyyyyyyyyyyyyyyy",
      "yyyyyyyyyyyyyyyy",
      "y              y",
      "y              y",
      "y              y",
      "y               "
    ]
  ]
}"#;

/// Merged palette for the rail part. Every entry is copied verbatim from its
/// jar source: all twelve entries of `palettes/rails.json` (the part's
/// `refpalette`), `y` from `palettes/default.json` (a `variant: stonebrick`),
/// and `T`, `g`, `~`, space from `palettes/common.json`. The brief's example ids
/// were wrong — the artefact says `e` is `rail[shape=east_west]` and `f` is
/// `powered_rail[powered=true,shape=east_west]`; both are carried as written.
pub const RAILS_PALETTE_JSON: &str = "{\"palette\": [{\"char\": \"k\",\"block\": \"minecraft:rail[shape=north_south]\"},{\"char\": \"e\",\"block\": \"minecraft:rail[shape=east_west]\"},{\"char\": \"j\",\"block\": \"minecraft:rail[shape=ascending_east]\"},{\"char\": \"q\",\"block\": \"minecraft:rail[shape=south_east]\"},{\"char\": \"s\",\"block\": \"minecraft:rail[shape=south_west]\"},{\"char\": \"r\",\"block\": \"minecraft:rail[shape=north_west]\"},{\"char\": \"n\",\"block\": \"minecraft:rail[shape=north_east]\"},{\"char\": \"m\",\"block\": \"minecraft:powered_rail[powered=true,shape=north_south]\"},{\"char\": \"E\",\"block\": \"minecraft:powered_rail[powered=true,shape=ascending_south]\"},{\"char\": \"H\",\"block\": \"minecraft:powered_rail[powered=true,shape=ascending_north]\"},{\"char\": \"f\",\"block\": \"minecraft:powered_rail[powered=true,shape=east_west]\"},{\"char\": \"i\",\"block\": \"minecraft:powered_rail[powered=true,shape=ascending_east]\"},{\"char\": \"y\",\"variant\": \"stonebrick\",\"damaged\": \"minecraft:iron_bars\"},{\"char\": \"T\",\"block\": \"minecraft:wall_torch[facing=north]\",\"torch\": true},{\"char\": \"g\",\"block\": \"minecraft:redstone_torch[lit=true]\"},{\"char\": \"~\",\"block\": \"minecraft:structure_void\"},{\"char\": \" \",\"block\": \"minecraft:air\"}]}";

/// `parts/rails_bend.json` from the cached jar, verbatim (2,643 B): 16x16, six
/// slices, `refpalette: "rails"`. Its exact non-space chars are `y`, `k`, `g`,
/// `m`, `e`, `f`, `r`, `s`, `~`. The brief's example ids were checked against the
/// artefact this time and match: `k` is north_south, `r` north_west, `s`
/// south_west and `m` powered north_south. No id or char was invented.
pub const RAILS_BEND_PART_JSON: &str = r#"{
  "xsize": 16,
  "zsize": 16,
  "refpalette": "rails",
  "slices": [
    [
      "      yyyyy     ",
      "      yyyyy     ",
      "      yyyyy     ",
      "yyyyyyyyyyy     ",
      "yyyyyyyyyyy     ",
      "yyyyyyyyyyy     ",
      "yyyyyyyyyyy     ",
      "yyyyyyyyyyy     ",
      "yyyyyyyyyyy     ",
      "yyyyyyyyyyy     ",
      "yyyyyyyyyyy     ",
      "yyyyyyyyyyy     ",
      "      yyyyy     ",
      "      yyyyy     ",
      "      yyyyy     ",
      "      yyyyy     "
    ],
    [
      "      yykyy     ",
      "      yykyy     ",
      "      yykyy     ",
      "yyyyyyyykyy     ",
      "yyyyyyygmyy     ",
      "eeeeeeefryy     ",
      "yyyyyyyyyyy     ",
      "yyyyyyyyyyy     ",
      "yyyyyyyyyyy     ",
      "eeeeeeefsyy     ",
      "yyyyyyygmyy     ",
      "yyyyyyyykyy     ",
      "      yykyy     ",
      "      yykyy     ",
      "      yykyy     ",
      "      yykyy     "
    ],
    [
      "      y~~~y     ",
      "      y~~~y     ",
      "      y~~~y     ",
      "yyyyyyy~~~y     ",
      "~~~~~~~~~~y     ",
      "~~~~~~~~~~y     ",
      "~~~~~~~~~~y     ",
      "~~~~~~~~yyy     ",
      "~~~~~~~~~~y     ",
      "~~~~~~~~~~y     ",
      "~~~~~~~~~~y     ",
      "yyyyyyy~~~y     ",
      "      y~~~y     ",
      "      y~~~y     ",
      "      y~~~y     ",
      "      y~~~y     "
    ],
    [
      "      y~~~y     ",
      "      y~~~y     ",
      "      y~~~y     ",
      "yyyyyyy~~~y     ",
      "~~~~~~~~~~y     ",
      "~~~~~~~~~~y     ",
      "~~~~~~~~~~y     ",
      "~~~~~~~~yyy     ",
      "~~~~~~~~~~y     ",
      "~~~~~~~~~~y     ",
      "~~~~~~~~~~y     ",
      "yyyyyyy~~~y     ",
      "      y~~~y     ",
      "      y~~~y     ",
      "      y~~~y     ",
      "      y~~~y     "
    ],
    [
      "      y~~~y     ",
      "      y~~~y     ",
      "      y~~~y     ",
      "yyyyyyy~~~y     ",
      "~~~~~~~~~~y     ",
      "~~~~~~~~~~y     ",
      "~~~~~~~~~~y     ",
      "~~~~~~~~yyy     ",
      "~~~~~~~~~~y     ",
      "~~~~~~~~~~y     ",
      "~~~~~~~~~~y     ",
      "yyyyyyy~~~y     ",
      "      y~~~y     ",
      "      y~~~y     ",
      "      y~~~y     ",
      "      y~~~y     "
    ],
    [
      "      yyyyy     ",
      "      yyyyy     ",
      "      yyyyy     ",
      "yyyyyyyyyyy     ",
      "yyyyyyyyyyy     ",
      "yyyyyyyyyyy     ",
      "yyyyyyyyyyy     ",
      "yyyyyyyyyyy     ",
      "yyyyyyyyyyy     ",
      "yyyyyyyyyyy     ",
      "yyyyyyyyyyy     ",
      "yyyyyyyyyyy     ",
      "      yyyyy     ",
      "      yyyyy     ",
      "      yyyyy     ",
      "      yyyyy     "
    ]
  ]
}"#;

/// `parts/rails_3split.json` from the cached jar, verbatim (2,643 B, sha256
/// `b0c8bd61...`): 16x16, six slices, `refpalette: "rails"`. Its exact non-space
/// chars are `y`, `k`, `g`, `m`, `n`, `f`, `e`, `q`, `~`, `T`; `n` and `q` are the
/// `rail[shape=north_east]` and `rail[shape=south_east]` entries of
/// `palettes/rails.json`, two shapes none of the first four parts writes. All
/// entries are copied byte-for-byte from the artefact, checked against its sha.
pub const RAILS_3SPLIT_PART_JSON: &str = r#"{
  "xsize": 16,
  "zsize": 16,
  "refpalette": "rails",
  "slices": [
    [
      "      yyyyy     ",
      "      yyyyy     ",
      "      yyyyy     ",
      "yyyyyyyyyyyyyyyy",
      "yyyyyyyyyyyyyyyy",
      "yyyyyyyyyyyyyyyy",
      "yyyyyyyyyyyyyyyy",
      "yyyyyyyyyyyyyyyy",
      "yyyyyyyyyyyyyyyy",
      "yyyyyyyyyyyyyyyy",
      "yyyyyyyyyyyyyyyy",
      "yyyyyyyyyyyyyyyy",
      "      yyyyy     ",
      "      yyyyy     ",
      "      yyyyy     ",
      "      yyyyy     "
    ],
    [
      "      yykyy     ",
      "      ygmgy     ",
      "      yykyy     ",
      "yyyyyyyykyyyyyyy",
      "yyyyyyyykyyyyyyy",
      "yyyyyyyynfeeeeee",
      "ygyyyyyyygyyyyyy",
      "efeeeeeeefeeeeee",
      "ygyyyyyyygyyyyyy",
      "yyyyyyyyqfeeeeee",
      "yyyyyyyykyyyyyyy",
      "yyyyyyyykyyyyyyy",
      "      yykyy     ",
      "      yykyy     ",
      "      ygmgy     ",
      "      yykyy     "
    ],
    [
      "      y~~~y     ",
      "      y~~~y     ",
      "      y~~~y     ",
      "yyyyyyy~~~yyyyyy",
      "~~~~~~~~~~~~~~~~",
      "~~~~~~~~~~~~~~~~",
      "~~~~~~~~~~~~~~~~",
      "~~~~~~~~~~~~~~~~",
      "~~~~~~~~~~~~~~~~",
      "~~~~~~~~~~~~~~~~",
      "~~~~~~~~~~~~~~~~",
      "yyyyyyy~~~yyyyyy",
      "      y~~~y     ",
      "      y~~~y     ",
      "      y~~~y     ",
      "      y~~~y     "
    ],
    [
      "      y~~~y     ",
      "      y~~~y     ",
      "      y~~~y     ",
      "yyyyyyy~~~yyyyyy",
      "~~~~~~~~~~~~~~~~",
      "~~~~~~~~~~~~~~~~",
      "~~~~~~~~~~~~~~~~",
      "~~~~~~~~~~~~~~~~",
      "~~~~~~~~~~~~~~~~",
      "~~~~~~~~~~~~~~~~",
      "~~~~~~~~~~~~~~~~",
      "yyyyyyy~~~yyyyyy",
      "      y~~~y     ",
      "      y~~~y     ",
      "      y~~~y     ",
      "      y~~~y     "
    ],
    [
      "      y~~~y     ",
      "      y~~~y     ",
      "      y~~~y     ",
      "yyyyyyy~~~yyyyyy",
      "~~~T~~~~~~~~~~~~",
      "~~~~~~~~~~~~~~~~",
      "~~~~~~~~~~~~~~~~",
      "~~~~~~~~~~~~~~~~",
      "~~~~~~~~~~~~~~~~",
      "~~~~~~~~~~~~~~~~",
      "~~~~~~~~~~~~~T~~",
      "yyyyyyy~~~yyyyyy",
      "      y~~~y     ",
      "      y~~~y     ",
      "      y~~~y     ",
      "      y~~~y     "
    ],
    [
      "      yyyyy     ",
      "      yyyyy     ",
      "      yyyyy     ",
      "yyyyyyyyyyyyyyyy",
      "yyyyyyyyyyyyyyyy",
      "yyyyyyyyyyyyyyyy",
      "yyyyyyyyyyyyyyyy",
      "yyyyyyyyyyyyyyyy",
      "yyyyyyyyyyyyyyyy",
      "yyyyyyyyyyyyyyyy",
      "yyyyyyyyyyyyyyyy",
      "yyyyyyyyyyyyyyyy",
      "      yyyyy     ",
      "      yyyyy     ",
      "      yyyyy     ",
      "      yyyyy     "
    ]
  ]
}"#;

/// `parts/rails_down2.json` from the cached jar, verbatim (7,779 B, sha256
/// `f2f6ebabe2ae71dc6c094496de0b90ebda97415ecc619b7c7b8c78161e65be89`):
/// 16x16, **eighteen** slices, `refpalette: "rails"`. Its exact non-space chars
/// are `y` 504, `~` 420, `j` 21, `i` 15, `f` 12, `g` 6 (978 cells). `j` and `i`
/// are the `rail[shape=ascending_east]` and
/// `powered_rail[powered=true,shape=ascending_east]` entries of
/// `palettes/rails.json`, two shapes none of the first five decoded parts
/// writes. The bytes below are copied straight from the artefact, checked
/// against its sha; the palette/variant entries are the same verbatim tables
/// the earlier rail parts use. No id or char was invented.
pub const RAILS_DOWN2_PART_JSON: &str = r#"{
  "xsize": 16,
  "zsize": 16,
  "refpalette": "rails",
  "slices": [
    [
      "                ",
      "                ",
      "                ",
      "yy              ",
      "yy              ",
      "yy              ",
      "yyy             ",
      "yy              ",
      "yyy             ",
      "yy              ",
      "yy              ",
      "yy              ",
      "                ",
      "                ",
      "                ",
      "                "
    ],
    [
      "                ",
      "                ",
      "                ",
      "yyy             ",
      "yyy             ",
      "fiy             ",
      "yyg             ",
      "fiy             ",
      "yyg             ",
      "fiy             ",
      "yyy             ",
      "yyy             ",
      "                ",
      "                ",
      "                ",
      "                "
    ],
    [
      "                ",
      "                ",
      "                ",
      "yyyy            ",
      "~~yy            ",
      "~~iy            ",
      "~~yy            ",
      "~~iy            ",
      "~~yy            ",
      "~~iy            ",
      "~~yy            ",
      "yyyy            ",
      "                ",
      "                ",
      "                ",
      "                "
    ],
    [
      "                ",
      "                ",
      "                ",
      "yyyyy           ",
      "~~~yy           ",
      "~~~jy           ",
      "~~~yy           ",
      "~~~jy           ",
      "~~~yy           ",
      "~~~jy           ",
      "~~~yy           ",
      "yyyyy           ",
      "                ",
      "                ",
      "                ",
      "                "
    ],
    [
      "                ",
      "                ",
      "                ",
      "yyyyyy          ",
      "~~~~yy          ",
      "~~~~jy          ",
      "~~~~yy          ",
      "~~~~jy          ",
      "~~~~yy          ",
      "~~~~jy          ",
      "~~~~yy          ",
      "yyyyyy          ",
      "                ",
      "                ",
      "                ",
      "                "
    ],
    [
      "                ",
      "                ",
      "                ",
      "yyyyyyy         ",
      "y~~~~yy         ",
      "y~~~~jy         ",
      "y~~~~yy         ",
      "y~~~~jy         ",
      "y~~~~yy         ",
      "y~~~~jy         ",
      "y~~~~yy         ",
      "yyyyyyy         ",
      "                ",
      "                ",
      "                ",
      "                "
    ],
    [
      "                ",
      "                ",
      "                ",
      " yyyyyyy        ",
      " y~~~~yy        ",
      " y~~~~jy        ",
      " y~~~~yy        ",
      " y~~~~jy        ",
      " y~~~~yy        ",
      " y~~~~jy        ",
      " y~~~~yy        ",
      " yyyyyyy        ",
      "                ",
      "                ",
      "                ",
      "                "
    ],
    [
      "                ",
      "                ",
      "                ",
      "  yyyyyyy       ",
      "  y~~~~yy       ",
      "  y~~~~iy       ",
      "  y~~~~yyy      ",
      "  y~~~~iy       ",
      "  y~~~~yyy      ",
      "  y~~~~iy       ",
      "  y~~~~yy       ",
      "  yyyyyyy       ",
      "                ",
      "                ",
      "                ",
      "                "
    ],
    [
      "                ",
      "                ",
      "                ",
      "   yyyyyyy      ",
      "   y~~~~yy      ",
      "   y~~~~iy      ",
      "   y~~~~yg      ",
      "   y~~~~iy      ",
      "   y~~~~yg      ",
      "   y~~~~iy      ",
      "   y~~~~yy      ",
      "   yyyyyyy      ",
      "                ",
      "                ",
      "                ",
      "                "
    ],
    [
      "                ",
      "                ",
      "                ",
      "    yyyyyyy     ",
      "    y~~~~yy     ",
      "    y~~~~iy     ",
      "    y~~~~yy     ",
      "    y~~~~iy     ",
      "    y~~~~yy     ",
      "    y~~~~iy     ",
      "    y~~~~yy     ",
      "    yyyyyyy     ",
      "                ",
      "                ",
      "                ",
      "                "
    ],
    [
      "                ",
      "                ",
      "                ",
      "     yyyyyyy    ",
      "     y~~~~yy    ",
      "     y~~~~jy    ",
      "     y~~~~yy    ",
      "     y~~~~jy    ",
      "     y~~~~yy    ",
      "     y~~~~jy    ",
      "     y~~~~yy    ",
      "     yyyyyyy    ",
      "                ",
      "                ",
      "                ",
      "                "
    ],
    [
      "                ",
      "                ",
      "                ",
      "      yyyyyyy   ",
      "      y~~~~yy   ",
      "      y~~~~jy   ",
      "      y~~~~yy y ",
      "      y~~~~jy   ",
      "      y~~~~yy y ",
      "      y~~~~jy   ",
      "      y~~~~yy   ",
      "      yyyyyyy   ",
      "                ",
      "                ",
      "                ",
      "                "
    ],
    [
      "                ",
      "                ",
      "                ",
      "       yyyyyyyyy",
      "       y~~~~yyyy",
      "       y~~~~jyyy",
      "       y~~~~yygy",
      "       y~~~~jyyy",
      "       y~~~~yygy",
      "       y~~~~jyyy",
      "       y~~~~yyyy",
      "       yyyyyyyyy",
      "                ",
      "                ",
      "                ",
      "                "
    ],
    [
      "                ",
      "                ",
      "                ",
      "        yyyyyyyy",
      "        y~~~~yyy",
      "        y~~~~fff",
      "        y~~~~yyy",
      "        y~~~~fff",
      "        y~~~~yyy",
      "        y~~~~fff",
      "        y~~~~yyy",
      "        yyyyyyyy",
      "                ",
      "                ",
      "                ",
      "                "
    ],
    [
      "                ",
      "                ",
      "                ",
      "         yyyyyyy",
      "         y~~~~~~",
      "         y~~~~~~",
      "         y~~~~~~",
      "         y~~~~~~",
      "         y~~~~~~",
      "         y~~~~~~",
      "         y~~~~~~",
      "         yyyyyyy",
      "                ",
      "                ",
      "                ",
      "                "
    ],
    [
      "                ",
      "                ",
      "                ",
      "          yyyyyy",
      "          y~~~~~",
      "          y~~~~~",
      "          y~~~~~",
      "          y~~~~~",
      "          y~~~~~",
      "          y~~~~~",
      "          y~~~~~",
      "          yyyyyy",
      "                ",
      "                ",
      "                ",
      "                "
    ],
    [
      "                ",
      "                ",
      "                ",
      "           yyyyy",
      "           y~~~~",
      "           y~~~~",
      "           y~~~~",
      "           y~~~~",
      "           y~~~~",
      "           y~~~~",
      "           y~~~~",
      "           yyyyy",
      "                ",
      "                ",
      "                ",
      "                "
    ],
    [
      "                ",
      "                ",
      "                ",
      "            yyyy",
      "            yyyy",
      "            yyyy",
      "            yyyy",
      "            yyyy",
      "            yyyy",
      "            yyyy",
      "            yyyy",
      "            yyyy",
      "                ",
      "                ",
      "                ",
      "                "
    ]
  ]
}"#;

/// `parts/rails_down1.json` from the cached jar, verbatim (5,211 B,
/// sha256 `822a38fa5420ab545f78248a216a937e0f1f8dc78d6a258ffbae45da8340411e`): 16x16, **twelve** slices, `refpalette: "rails"`. Its exact
/// non-space char census is `:` 30, `_` 26, `e` 27, `f` 3, `g` 2, `j` 18,
/// `y` 279, `~` 526 (911 cells). `:` and `_` are absent from
/// `palettes/rails.json`; both are copied verbatim from the jar's
/// `palettes/default.json` into `RAILS_DOWN1_PALETTE_JSON` below. No id or
/// char was invented — the bytes below are the artefact's, checked by sha256.
pub const RAILS_DOWN1_PART_JSON: &str = r#"{
  "xsize": 16,
  "zsize": 16,
  "refpalette": "rails",
  "slices": [
    [
      "                ",
      "                ",
      "                ",
      "y               ",
      "y               ",
      "y               ",
      "y               ",
      "y               ",
      "y               ",
      "y               ",
      "y               ",
      "y               ",
      "                ",
      "                ",
      "                ",
      "                "
    ],
    [
      "                ",
      "                ",
      "                ",
      "yy              ",
      "yy              ",
      "jy              ",
      "yy              ",
      "jy              ",
      "yy              ",
      "jy              ",
      "yy              ",
      "yy              ",
      "                ",
      "                ",
      "                ",
      "                "
    ],
    [
      "                ",
      "                ",
      "                ",
      "yyy             ",
      "~yy             ",
      "~jy             ",
      "~yy             ",
      "~jy             ",
      "~yy             ",
      "~jy             ",
      "~yy             ",
      "yyy             ",
      "                ",
      "                ",
      "                ",
      "                "
    ],
    [
      "                ",
      "                ",
      "                ",
      "yyyy            ",
      "~~yy            ",
      "~~jy            ",
      "~~yy            ",
      "~~jy            ",
      "~~yy            ",
      "~~jy            ",
      "~~yy            ",
      "yyyy            ",
      "                ",
      "                ",
      "                ",
      "                "
    ],
    [
      "                ",
      "                ",
      "                ",
      "yyyyy           ",
      "~~~yy           ",
      "~~~jy           ",
      "~~~yy           ",
      "~~~jy           ",
      "~~~yy           ",
      "~~~jy           ",
      "~~~yy           ",
      "yyyyy           ",
      "                ",
      "                ",
      "                ",
      "                "
    ],
    [
      "                ",
      "                ",
      "                ",
      "yyyyyy          ",
      "~~~~yy          ",
      "~~~~jy          ",
      "~~~~yy y        ",
      "~~~~jy          ",
      "~~~~yy y        ",
      "~~~~jy          ",
      "~~~~yy          ",
      "yyyyyy          ",
      "                ",
      "                ",
      "                ",
      "                "
    ],
    [
      "                ",
      "                ",
      "                ",
      "yyyyyyy         ",
      "y~~~~yyyyyyyyyyy",
      "y~~~~jyyyyyyyyyy",
      "y~~~~yygyyyyyyyy",
      "y~~~~jyyyyyyyyyy",
      "y~~~~yygyyyyyyyy",
      "y~~~~jyyyyyyyyyy",
      "y~~~~yyyyyyyyyyy",
      "yyyyyyy         ",
      "                ",
      "                ",
      "                ",
      "                "
    ],
    [
      "               y",
      "               y",
      "               y",
      " yyyyyy        y",
      " y~~~~yyyyyyyyyy",
      " y~~~~efeeeeeeee",
      " y~~~~yyyyyyyyyy",
      " y~~~~efeeeeeeee",
      " y~~~~yyyyyyyyyy",
      " y~~~~efeeeeeeee",
      " y~~~~yyyyyyyyyy",
      " yyyyyy        y",
      "               y",
      "               y",
      "               y",
      "               y"
    ],
    [
      "                ",
      "                ",
      "                ",
      " y::::y         ",
      " :~~~~~~~~~~~~~~",
      " :~~~~~~~~~~~~~~",
      " :~~~~~~~~~~~~~~",
      " :~~~~~~~~~~~~~~",
      " :~~~~~~~~~~~~~~",
      " :~~~~~~~~~~~~~~",
      " :~~~~~~~~~~~~~~",
      " y::::y         ",
      "                ",
      "                ",
      "                ",
      "                "
    ],
    [
      "                ",
      "                ",
      "                ",
      " y::::y         ",
      " :~~~~~~~~~~~~~~",
      " :~~~~~~~~~~~~~~",
      " :~~~~~~~~~~~~~~",
      " :~~~~~~~~~~~~~~",
      " :~~~~~~~~~~~~~~",
      " :~~~~~~~~~~~~~~",
      " :~~~~~~~~~~~~~~",
      " y::::y         ",
      "                ",
      "                ",
      "                ",
      "                "
    ],
    [
      "                ",
      "                ",
      "                ",
      " y~~~~y         ",
      " ~~~~~~~~~~~~~~~",
      " ~~~~~~~~~~~~~~~",
      " ~~~~~~~~~~~~~~~",
      " ~~~~~~~~~~~~~~~",
      " ~~~~~~~~~~~~~~~",
      " ~~~~~~~~~~~~~~~",
      " ~~~~~~~~~~~~~~~",
      " y~~~~y         ",
      "                ",
      "                ",
      "                ",
      "                "
    ],
    [
      "                ",
      "                ",
      "                ",
      " ______         ",
      " _~~~~_~~~~~~~~~",
      " _~~~~_~~~~~~~~~",
      " _~~~~_~~~~~~~~~",
      " _~~~~_~~~~~~~~~",
      " _~~~~_~~~~~~~~~",
      " _~~~~_~~~~~~~~~",
      " _~~~~_~~~~~~~~~",
      " ______         ",
      "                ",
      "                ",
      "                ",
      "                "
    ]
  ]
}"#;

/// Merged palette for the first rail descent: every entry of the part's
/// `refpalette` (`palettes/rails.json`, carried verbatim in
/// `RAILS_PALETTE_JSON` above), plus the two entries the earlier rail parts
/// never needed, copied byte-for-byte from `palettes/default.json`:
/// `:` -> `minecraft:iron_bars` and `_` ->
/// `minecraft:smooth_stone_slab[type=bottom]`. `y` is the same jar
/// `variant: stonebrick` entry and `g`/`~`/space come from `palettes/common.json`
/// through the existing rail palette. No id or char was invented.
pub const RAILS_DOWN1_PALETTE_JSON: &str = "{\"palette\":[{\"char\":\"k\",\"block\":\"minecraft:rail[shape=north_south]\"},{\"char\":\"e\",\"block\":\"minecraft:rail[shape=east_west]\"},{\"char\":\"j\",\"block\":\"minecraft:rail[shape=ascending_east]\"},{\"char\":\"q\",\"block\":\"minecraft:rail[shape=south_east]\"},{\"char\":\"s\",\"block\":\"minecraft:rail[shape=south_west]\"},{\"char\":\"r\",\"block\":\"minecraft:rail[shape=north_west]\"},{\"char\":\"n\",\"block\":\"minecraft:rail[shape=north_east]\"},{\"char\":\"m\",\"block\":\"minecraft:powered_rail[powered=true,shape=north_south]\"},{\"char\":\"E\",\"block\":\"minecraft:powered_rail[powered=true,shape=ascending_south]\"},{\"char\":\"H\",\"block\":\"minecraft:powered_rail[powered=true,shape=ascending_north]\"},{\"char\":\"f\",\"block\":\"minecraft:powered_rail[powered=true,shape=east_west]\"},{\"char\":\"i\",\"block\":\"minecraft:powered_rail[powered=true,shape=ascending_east]\"},{\"char\":\"y\",\"variant\":\"stonebrick\",\"damaged\":\"minecraft:iron_bars\"},{\"char\":\"T\",\"block\":\"minecraft:wall_torch[facing=north]\",\"torch\":true},{\"char\":\"g\",\"block\":\"minecraft:redstone_torch[lit=true]\"},{\"char\":\"~\",\"block\":\"minecraft:structure_void\"},{\"char\":\" \",\"block\":\"minecraft:air\"},{\"char\":\":\",\"block\":\"minecraft:iron_bars\"},{\"char\":\"_\",\"block\":\"minecraft:smooth_stone_slab[type=bottom]\"}]}";

/// `parts/building1_2.json` from the cached jar, verbatim (2,618 B, sha256
/// `b2130f36f887c90c4b1602f04e3d70bcff0835ac5b176b070ca78e94ff264f74`):
/// 16x16, six slices, 998 non-space cells. Its exact char census is `#` 812,
/// `a` 144, `@` 24, `l` 6, `Q` 3, `*` 2, `:` 2, `T` 2, `F` 2, `1` 1 — measured
/// from the artefact, not from the brief. `Q` is absent from every other
/// decoded part, so its `minecraft:quartz_block` is the clean differential.
pub const BUILDING_1_2_PART_JSON: &str = "{\n  \"xsize\": 16,\n  \"zsize\": 16,\n  \"slices\": [\n    [\n      \"################\",\n      \"################\",\n      \"################\",\n      \"################\",\n      \"################\",\n      \"################\",\n      \"################\",\n      \"################\",\n      \"################\",\n      \"#######l########\",\n      \"####1###########\",\n      \"################\",\n      \"################\",\n      \"################\",\n      \"################\",\n      \"################\"\n    ],\n    [\n      \"#aa@aaaaaaaa@aa#\",\n      \"a  F      QQQ  a\",\n      \"a              a\",\n      \"@              @\",\n      \"a              a\",\n      \"a             Fa\",\n      \"a              a\",\n      \"a              a\",\n      \"a              a\",\n      \"a      l       a\",\n      \"a###########  #a\",\n      \"a              a\",\n      \"@              @\",\n      \"a              a\",\n      \"a              a\",\n      \"#aa@aaaaaaaa@aa#\"\n    ],\n    [\n      \"#aa@aaaaaaaa@aa#\",\n      \"a         **   a\",\n      \"a              a\",\n      \"@              @\",\n      \"a              a\",\n      \"a              a\",\n      \"a              a\",\n      \"a              a\",\n      \"a              a\",\n      \"a    T l       a\",\n      \"a#::########  #a\",\n      \"a    T         a\",\n      \"@              @\",\n      \"a              a\",\n      \"a              a\",\n      \"#aa@aaaaaaaa@aa#\"\n    ],\n    [\n      \"#aa@aaaaaaaa@aa#\",\n      \"a              a\",\n      \"a              a\",\n      \"@              @\",\n      \"a              a\",\n      \"a              a\",\n      \"a              a\",\n      \"a              a\",\n      \"a              a\",\n      \"a      l       a\",\n      \"a##############a\",\n      \"a              a\",\n      \"@              @\",\n      \"a              a\",\n      \"a              a\",\n      \"#aa@aaaaaaaa@aa#\"\n    ],\n    [\n      \"################\",\n      \"################\",\n      \"################\",\n      \"################\",\n      \"################\",\n      \"################\",\n      \"################\",\n      \"################\",\n      \"################\",\n      \"#######l########\",\n      \"################\",\n      \"################\",\n      \"################\",\n      \"################\",\n      \"################\",\n      \"################\"\n    ],\n    [\n      \"################\",\n      \"################\",\n      \"################\",\n      \"################\",\n      \"################\",\n      \"################\",\n      \"################\",\n      \"################\",\n      \"################\",\n      \"#######l########\",\n      \"################\",\n      \"################\",\n      \"################\",\n      \"################\",\n      \"################\",\n      \"################\"\n    ]\n  ]\n}";

/// Merged palette for the second building part: every entry of the standard
/// building palette (`BUILDING_PALETTE_JSON`, the groups `styles/standard.json`
/// selects together) plus the two `palettes/default.json` entries this part
/// needs and the earlier building part never used: `Q` -> `minecraft:quartz_block`
/// (with its jar `damaged` field) and `:` -> `minecraft:iron_bars`, copied
/// verbatim from the jar. No id or char was invented.
pub const BUILDING_1_2_PALETTE_JSON: &str = "{\"palette\": [{\"char\": \"#\", \"variant\": \"stonebrick\", \"damaged\": \"minecraft:iron_bars\"}, {\"char\": \"a\", \"block\": \"minecraft:glass\"}, {\"char\": \"@\", \"frompalette\": \"#\"}, {\"char\": \"1\", \"block\": \"minecraft:spawner\", \"mob\": \"easymobs\"}, {\"char\": \"T\", \"block\": \"minecraft:wall_torch[facing=north]\", \"torch\": true}, {\"char\": \"F\", \"blocks\": [{\"random\": 25, \"block\": \"minecraft:furnace[facing=north]\"}, {\"random\": 20, \"block\": \"minecraft:crafting_table\"}, {\"random\": 7, \"block\": \"minecraft:brewing_stand\"}, {\"random\": 6, \"block\": \"minecraft:anvil[facing=north]\"}, {\"random\": 5, \"block\": \"minecraft:cauldron\"}, {\"random\": 2, \"block\": \"minecraft:enchanting_table\"}, {\"random\": 1000, \"block\": \"minecraft:cobweb\"}]}, {\"char\": \"D\", \"block\": \"minecraft:dirt\"}, {\"char\": \"l\", \"block\": \"minecraft:ladder[facing=north]\"}, {\"char\": \"L\", \"block\": \"minecraft:bookshelf\"}, {\"char\": \"*\", \"block\": \"minecraft:potted_cactus\"}, {\"char\": \" \", \"block\": \"minecraft:air\"}, {\"char\": \"Q\", \"block\": \"minecraft:quartz_block\", \"damaged\": \"minecraft:iron_bars\"}, {\"char\": \":\", \"block\": \"minecraft:iron_bars\"}]}";

/// `parts/center00_1.json` from the cached jar, verbatim (2,618 B, sha256
/// `5ade2e7903e012f38fb9e9559ccda6aa51ef4c9856d2d365b891c118acebdc54`):
/// 16x16, six slices, 680 non-space cells. Exact census: `#` 338, `a` 126,
/// `Q` 107, `S` 97, `)` 12 — measured from the artefact, not from the brief.
/// The `)` cells are at (x,y,z) (13,0,14), (14,0,14), (13,1,9), (14,1,9),
/// (13,2,10), (14,2,10), (13,3,11), (14,3,11), (13,4,12), (14,4,12),
/// (13,5,13), (14,5,13). Written as an escaped literal because each brick row
/// begins `"#`, which would close a `r#"..."#` raw string immediately.
pub const CENTER00_1_PART_JSON: &str = "{\n  \"xsize\": 16,\n  \"zsize\": 16,\n  \"slices\": [\n    [\n      \"##   #aaaa#   ##\",\n      \"######QSQS######\",\n      \" #SQSQSQSQSQSQSQ\",\n      \" #QSQSQSQSQSQSQS\",\n      \" #SQSQSQSQSQSQSQ\",\n      \"##QSQSQSQSQSQSQS\",\n      \"aQSQSQSQSQSQSQSQ\",\n      \"aSQSQSQSQSQSQSQS\",\n      \"aQSQSQSQSQSQSQSQ\",\n      \"aSQSQSQSQSQSQSQS\",\n      \"##SQSQSQSQSQS  Q\",\n      \" #QSQSQSQSQSQ  S\",\n      \" #SQSQSQSQSQS  Q\",\n      \" #QSQSQSQSQSQ  S\",\n      \"##SQSQSQSQSQS))Q\",\n      \"##QSQSQSQSQSQSQS\"\n    ],\n    [\n      \"##   #aaaa#   ##\",\n      \"##aaa#    #aaa##\",\n      \" a              \",\n      \" a              \",\n      \" a              \",\n      \"##              \",\n      \"a               \",\n      \"a               \",\n      \"a               \",\n      \"a            )) \",\n      \"##          Q  Q\",\n      \" a          Q  Q\",\n      \" a          Q  Q\",\n      \" a          Q  Q\",\n      \"##          Q  Q\",\n      \"##              \"\n    ],\n    [\n      \"##   #aaaa#   ##\",\n      \"##aaa#    #aaa##\",\n      \" a              \",\n      \" a              \",\n      \" a              \",\n      \"##              \",\n      \"a               \",\n      \"a               \",\n      \"a               \",\n      \"a               \",\n      \"##          a))a\",\n      \" a          a  a\",\n      \" a          a  a\",\n      \" a          a  a\",\n      \"##          a  a\",\n      \"##              \"\n    ],\n    [\n      \"##   #aaaa#   ##\",\n      \"##aaa#    #aaa##\",\n      \" a              \",\n      \" a              \",\n      \" a              \",\n      \"##              \",\n      \"a               \",\n      \"a               \",\n      \"a               \",\n      \"a               \",\n      \"##          a  a\",\n      \" a          a))a\",\n      \" a          a  a\",\n      \" a          a  a\",\n      \"##          a  a\",\n      \"##              \"\n    ],\n    [\n      \"##   #aaaa#   ##\",\n      \"##aaa#    #aaa##\",\n      \" a              \",\n      \" a              \",\n      \" a              \",\n      \"##              \",\n      \"a               \",\n      \"a               \",\n      \"a               \",\n      \"a               \",\n      \"##          a  a\",\n      \" a          a  a\",\n      \" a          a))a\",\n      \" a          a  a\",\n      \"##          a  a\",\n      \"##              \"\n    ],\n    [\n      \"##   #aaaa#   ##\",\n      \"################\",\n      \" ###############\",\n      \" ###############\",\n      \" ###############\",\n      \"################\",\n      \"a###############\",\n      \"a###############\",\n      \"a###############\",\n      \"a###############\",\n      \"#############  #\",\n      \" ############  #\",\n      \" ############  #\",\n      \" ############))#\",\n      \"#############  #\",\n      \"################\"\n    ]\n  ]\n}";

/// Merged palette for the city-centre part: the standard building groups
/// (`BUILDING_PALETTE_JSON`, selected together by `styles/standard.json`) plus
/// three entries copied verbatim from the jar's `palettes/default.json`: `Q` ->
/// `minecraft:quartz_block` (with its `damaged` field), `S` ->
/// `minecraft:smooth_stone_slab[type=double]` (with its `damaged` field), and
/// the one new entry `)` -> `minecraft:quartz_stairs[facing=south,half=bottom,
/// shape=straight]`. No id or char was invented.
pub const CENTER00_1_PALETTE_JSON: &str = "{\"palette\": [{\"char\": \"#\", \"variant\": \"stonebrick\", \"damaged\": \"minecraft:iron_bars\"}, {\"char\": \"a\", \"block\": \"minecraft:glass\"}, {\"char\": \"@\", \"frompalette\": \"#\"}, {\"char\": \"1\", \"block\": \"minecraft:spawner\", \"mob\": \"easymobs\"}, {\"char\": \"T\", \"block\": \"minecraft:wall_torch[facing=north]\", \"torch\": true}, {\"char\": \"F\", \"blocks\": [{\"random\": 25, \"block\": \"minecraft:furnace[facing=north]\"}, {\"random\": 20, \"block\": \"minecraft:crafting_table\"}, {\"random\": 7, \"block\": \"minecraft:brewing_stand\"}, {\"random\": 6, \"block\": \"minecraft:anvil[facing=north]\"}, {\"random\": 5, \"block\": \"minecraft:cauldron\"}, {\"random\": 2, \"block\": \"minecraft:enchanting_table\"}, {\"random\": 1000, \"block\": \"minecraft:cobweb\"}]}, {\"char\": \"D\", \"block\": \"minecraft:dirt\"}, {\"char\": \"l\", \"block\": \"minecraft:ladder[facing=north]\"}, {\"char\": \"L\", \"block\": \"minecraft:bookshelf\"}, {\"char\": \"*\", \"block\": \"minecraft:potted_cactus\"}, {\"char\": \" \", \"block\": \"minecraft:air\"}, {\"char\": \"Q\", \"block\": \"minecraft:quartz_block\", \"damaged\": \"minecraft:iron_bars\"}, {\"char\": \"S\", \"block\": \"minecraft:smooth_stone_slab[type=double]\", \"damaged\": \"minecraft:iron_bars\"}, {\"char\": \")\", \"block\": \"minecraft:quartz_stairs[facing=south,half=bottom,shape=straight]\"}]}";

/// `parts/shopping_open01_1.json` from the cached jar, verbatim (2,697 B, sha256
/// `c16f5ff65b9a32b168b8c5bb5edfbdaa970a994bc1434f4a35679b30b4a4ed5d`):
/// 16x16, six slices, a private `meta` key the decoder ignores. Its only non-space
/// char is `w` (12 cells at x=1, z=0 or 14, y=0..5); every other cell is air.
/// Written as a raw string: no row contains the `"#` sequence that would end it.
pub const SHOPPING_OPEN01_1_PART_JSON: &str = r#"{
  "meta": [
    {
      "key": "dontconnect",
      "boolean": true
    }
  ],
  "xsize": 16,
  "zsize": 16,
  "slices": [
    [
      " w              ",
      "                ",
      "                ",
      "                ",
      "                ",
      "                ",
      "                ",
      "                ",
      "                ",
      "                ",
      "                ",
      "                ",
      "                ",
      "                ",
      " w              ",
      "                "
    ],
    [
      " w              ",
      "                ",
      "                ",
      "                ",
      "                ",
      "                ",
      "                ",
      "                ",
      "                ",
      "                ",
      "                ",
      "                ",
      "                ",
      "                ",
      " w              ",
      "                "
    ],
    [
      " w              ",
      "                ",
      "                ",
      "                ",
      "                ",
      "                ",
      "                ",
      "                ",
      "                ",
      "                ",
      "                ",
      "                ",
      "                ",
      "                ",
      " w              ",
      "                "
    ],
    [
      " w              ",
      "                ",
      "                ",
      "                ",
      "                ",
      "                ",
      "                ",
      "                ",
      "                ",
      "                ",
      "                ",
      "                ",
      "                ",
      "                ",
      " w              ",
      "                "
    ],
    [
      " w              ",
      "                ",
      "                ",
      "                ",
      "                ",
      "                ",
      "                ",
      "                ",
      "                ",
      "                ",
      "                ",
      "                ",
      "                ",
      "                ",
      " w              ",
      "                "
    ],
    [
      " w              ",
      "                ",
      "                ",
      "                ",
      "                ",
      "                ",
      "                ",
      "                ",
      "                ",
      "                ",
      "                ",
      "                ",
      "                ",
      "                ",
      " w              ",
      "                "
    ]
  ]
}"#;

/// Merged palette for the shopping-open part: the single `w` entry copied
/// verbatim from the jar's `palettes/default.json` (`minecraft:cobblestone_wall`).
/// No other decoded part writes a cobblestone wall, so it is the clean in-game
/// differential. Spaces are air and are skipped by the decoder; no id invented.
pub const SHOPPING_OPEN01_1_PALETTE_JSON: &str = r#"{
  "palette": [
    {
      "char": "w",
      "block": "minecraft:cobblestone_wall"
    }
  ]
}"#;

/// `parts/stairsnormal.json` from the cached jar, verbatim (1,484 B, sha256
/// `c6d17e44238f625378d2f8e474aba1a1ea48c755200ecd5214b3b75d9bb387bc`):
/// **6x11**, seven slices, 138 non-space cells (`x` 102, `/` 24, `_` 12). Its
/// private `meta` array (`z1`/`z2` integers) is carried byte-identically and
/// ignored by the decoder like any unknown key. Written as a raw string: no
/// row contains the `"#` sequence that would end `r#"..."#`.
pub const STAIRSNORMAL_PART_JSON: &str = r#"{
  "meta": [
    {
      "integer": 5,
      "key": "z1"
    },
    {
      "integer": 10,
      "key": "z2"
    }
  ],
  "xsize": 6,
  "zsize": 11,
  "slices": [
    [
      "      ",
      "      ",
      "      ",
      "      ",
      "      ",
      "xxxxxx",
      "xxxxx/",
      "xxxxx/",
      "xxxxx/",
      "xxxxx/",
      "xxxxxx"
    ],
    [
      "      ",
      "      ",
      "      ",
      "      ",
      "      ",
      "xxxxx_",
      "xxxx/ ",
      "xxxx/ ",
      "xxxx/ ",
      "xxxx/ ",
      "xxxxx_"
    ],
    [
      "      ",
      "      ",
      "      ",
      "      ",
      "      ",
      "xxxx_ ",
      "xxx/  ",
      "xxx/  ",
      "xxx/  ",
      "xxx/  ",
      "xxxx_ "
    ],
    [
      "      ",
      "      ",
      "      ",
      "      ",
      "      ",
      "xxx_  ",
      "xx/   ",
      "xx/   ",
      "xx/   ",
      "xx/   ",
      "xxx_  "
    ],
    [
      "      ",
      "      ",
      "      ",
      "      ",
      "      ",
      "xx_   ",
      "x/    ",
      "x/    ",
      "x/    ",
      "x/    ",
      "xx_   "
    ],
    [
      "      ",
      "      ",
      "      ",
      "      ",
      "      ",
      "x_    ",
      "/     ",
      "/     ",
      "/     ",
      "/     ",
      "x_    "
    ],
    [
      "      ",
      "      ",
      "      ",
      "      ",
      "      ",
      "_     ",
      "      ",
      "      ",
      "      ",
      "      ",
      "_     "
    ]
  ]
}"#;

/// Merged palette for the stair part: the three `palettes/default.json`
/// entries its chars need, each copied verbatim from that jar file — `x` is the
/// jar's `variant: stonebrick` entry (resolved through the same
/// `variants/stonebrick.json` as every earlier part), `_` is the bottom
/// smooth-stone slab and `/` is the one new id,
/// `minecraft:stone_stairs[facing=west,half=bottom,shape=straight]`. No other
/// decoded part writes a stone stair, so `/` is the clean in-game differential.
pub const STAIRSNORMAL_PALETTE_JSON: &str = r#"{
  "palette": [
    {
      "char": "x",
      "variant": "stonebrick",
      "damaged": "minecraft:iron_bars"
    },
    {
      "char": "_",
      "block": "minecraft:smooth_stone_slab[type=bottom]"
    },
    {
      "char": "/",
      "block": "minecraft:stone_stairs[facing=west,half=bottom,shape=straight]"
    }
  ]
}"#;

/// `parts/stairsbig.json` from the cached jar, verbatim (1,596 B, sha256
/// `ae9dc82a2450c587cad447355021b5985549d4d17702de6e38b7b31a2cfe3a2c`):
/// **6x12**, seven slices, 180 non-space cells (`x` 132, `/` 36, `_` 12). Its
/// private `meta` array (`z1`/`z2` integers) is carried byte-identically and
/// ignored by the decoder like any unknown key. Written as a raw string: no
/// row contains the `"#` sequence that would end `r#"..."#`. All three chars
/// are already declared verbatim in `STAIRSNORMAL_PALETTE_JSON`, so only the
/// part bytes are new.
pub const STAIRSBIG_PART_JSON: &str = r#"{
  "meta": [
    {
      "integer": 4,
      "key": "z1"
    },
    {
      "integer": 11,
      "key": "z2"
    }
  ],
  "xsize": 6,
  "zsize": 12,
  "slices": [
    [
      "      ",
      "      ",
      "      ",
      "      ",
      "xxxxxx",
      "xxxxx/",
      "xxxxx/",
      "xxxxx/",
      "xxxxx/",
      "xxxxx/",
      "xxxxx/",
      "xxxxxx"
    ],
    [
      "      ",
      "      ",
      "      ",
      "      ",
      "xxxxx_",
      "xxxx/ ",
      "xxxx/ ",
      "xxxx/ ",
      "xxxx/ ",
      "xxxx/ ",
      "xxxx/ ",
      "xxxxx_"
    ],
    [
      "      ",
      "      ",
      "      ",
      "      ",
      "xxxx_ ",
      "xxx/  ",
      "xxx/  ",
      "xxx/  ",
      "xxx/  ",
      "xxx/  ",
      "xxx/  ",
      "xxxx_ "
    ],
    [
      "      ",
      "      ",
      "      ",
      "      ",
      "xxx_  ",
      "xx/   ",
      "xx/   ",
      "xx/   ",
      "xx/   ",
      "xx/   ",
      "xx/   ",
      "xxx_  "
    ],
    [
      "      ",
      "      ",
      "      ",
      "      ",
      "xx_   ",
      "x/    ",
      "x/    ",
      "x/    ",
      "x/    ",
      "x/    ",
      "x/    ",
      "xx_   "
    ],
    [
      "      ",
      "      ",
      "      ",
      "      ",
      "x_    ",
      "/     ",
      "/     ",
      "/     ",
      "/     ",
      "/     ",
      "/     ",
      "x_    "
    ],
    [
      "      ",
      "      ",
      "      ",
      "      ",
      "_     ",
      "      ",
      "      ",
      "      ",
      "      ",
      "      ",
      "      ",
      "_     "
    ]
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
        let variants = VariantTable::default();
        for salt in 0..64 {
            let pick = p
                .resolve('a', salt, &variants)
                .expect("weighted char resolves");
            assert!(
                pick == "minecraft:stone" || pick == "minecraft:dirt",
                "unexpected weighted pick: {pick}"
            );
        }
    }

    #[test]
    fn block_properties_are_split_from_the_id() {
        let (id, props) = split_block("minecraft:redstone_torch[lit=true]");
        assert_eq!(id, "minecraft:redstone_torch");
        assert_eq!(props, vec![("lit".to_string(), "true".to_string())]);
    }
}

/// Slice 3: the second, multi-slice building part and the `variant` palette
/// entry form. Both are read from the cached jar; the tests are written before
/// the decoder/data exist so the gap is proved red first.
#[cfg(test)]
mod building_tests {
    use super::*;

    /// The jar's `variants/stonebrick.json` resolves a `variant` entry to a
    /// weighted choice of stone-brick family blocks — not a literal id.
    #[test]
    fn variant_entry_resolves_through_the_variant_table() {
        let variants = VariantTable::from_files(&[("stonebrick", STONEBRICK_VARIANT_JSON)])
            .expect("the jar's stonebrick variant decodes");
        let placements =
            decode_city_with_variants(VARIANT_PART_JSON, VARIANT_PALETTE_JSON, &variants)
                .expect("a 'variant' palette entry resolves");
        assert_eq!(placements.len(), 1, "one cell, one block");
        assert!(
            matches!(
                placements[0].block.as_str(),
                "minecraft:stone_bricks"
                    | "minecraft:cracked_stone_bricks"
                    | "minecraft:mossy_stone_bricks"
            ),
            "the variant must resolve to a stone-brick family block, got {}",
            placements[0].block
        );
    }

    /// The embedded building part is measured from the jar: 16x16 footprint,
    /// six slices, all rows the declared width. This is the multi-layer bound
    /// the single-slice street part never exercised.
    #[test]
    fn embedded_building_part_has_six_16x16_layers() {
        let part = Part::from_json(&parse_json(BUILDING_PART_JSON).expect("jar part parses"))
            .expect("jar part decodes");
        assert_eq!(part.xsize, 16, "jar xsize");
        assert_eq!(part.zsize, 16, "jar zsize");
        assert_eq!(part.slices.len(), 6, "the building has six slices");
        for (y, layer) in part.slices.iter().enumerate() {
            assert_eq!(layer.len(), 16, "slice {y} has 16 rows");
            for (z, row) in layer.iter().enumerate() {
                assert_eq!(row.chars().count(), 16, "slice {y} row {z} is 16 chars");
            }
        }
    }

    /// The full building decodes with the merged jar palette: it contains the
    /// `L` -> bookshelf char (the in-game assertion), and placements span all
    /// six layers (dy 0 through 5).
    #[test]
    fn embedded_building_decodes_with_bookshelf_across_six_layers() {
        let variants = VariantTable::from_files(&[("stonebrick", STONEBRICK_VARIANT_JSON)])
            .expect("the jar's stonebrick variant decodes");
        let placements =
            decode_city_with_variants(BUILDING_PART_JSON, BUILDING_PALETTE_JSON, &variants)
                .expect("the embedded jar building and merged palette decode");
        assert!(
            placements.iter().any(|p| p.block == "minecraft:bookshelf"),
            "the L char must resolve to the palette's bookshelf"
        );
        assert_eq!(
            placements.iter().map(|p| p.dy).min(),
            Some(0),
            "the first layer is layer 0"
        );
        assert_eq!(
            placements.iter().map(|p| p.dy).max(),
            Some(5),
            "six slices span dy 0..5"
        );
    }
}

/// Slice 4: the third part, the horizontal rail line. Written before any rail
/// data exists so the decoder gap is proved red first. Every id asserted below
/// is read from the cached jar, not from the brief's illustrative char list.
#[cfg(test)]
mod rails_tests {
    use super::*;

    /// The jar's `parts/rails_horizontal.json` is 16x16 with six slices. This
    /// is the bound the single-slice street never exercised in the rail family.
    #[test]
    fn embedded_rails_part_has_six_16x16_layers() {
        let part = Part::from_json(&parse_json(RAILS_PART_JSON).expect("jar rail part parses"))
            .expect("jar rail part decodes");
        assert_eq!(part.xsize, 16, "jar xsize");
        assert_eq!(part.zsize, 16, "jar zsize");
        assert_eq!(part.slices.len(), 6, "the rail part has six slices");
        for (y, layer) in part.slices.iter().enumerate() {
            assert_eq!(layer.len(), 16, "slice {y} has 16 rows");
            for (z, row) in layer.iter().enumerate() {
                assert_eq!(row.chars().count(), 16, "slice {y} row {z} is 16 chars");
            }
        }
    }

    /// The rail chars resolve to the exact ids of `palettes/rails.json`. The
    /// brief's list (`e` = north_south, `f` = plain east_west rail) is wrong;
    /// the artefact says `e` = east_west rail and `f` = east_west powered rail.
    #[test]
    fn rail_chars_resolve_to_the_jar_rail_ids() {
        let variants = VariantTable::from_files(&[("stonebrick", STONEBRICK_VARIANT_JSON)])
            .expect("the jar's stonebrick variant decodes");
        let placements = decode_city_with_variants(RAILS_PART_JSON, RAILS_PALETTE_JSON, &variants)
            .expect("the embedded jar rail part and merged palette decode");
        let rail = placements
            .iter()
            .find(|p| p.dx == 6 && p.dy == 1 && p.dz == 5)
            .expect("the e char at (6,1,5)");
        assert_eq!(rail.block, "minecraft:rail", "the e char is a rail");
        assert_eq!(
            rail.properties,
            vec![("shape".to_string(), "east_west".to_string())],
            "the e char is the east_west rail of palettes/rails.json"
        );
        let powered = placements
            .iter()
            .find(|p| p.dx == 7 && p.dy == 1 && p.dz == 5)
            .expect("the f char at (7,1,5)");
        assert_eq!(
            powered.block, "minecraft:powered_rail",
            "the f char is a powered rail"
        );
        assert_eq!(
            powered.properties,
            vec![
                ("powered".to_string(), "true".to_string()),
                ("shape".to_string(), "east_west".to_string())
            ],
            "the f char is the powered east_west rail of palettes/rails.json"
        );
        // The differential block id of the in-game assertion: 15 `e` cells in
        // each of the three rail rows of slice 1, no other part writes a rail.
        assert_eq!(
            placements
                .iter()
                .filter(|p| p.block == "minecraft:rail")
                .count(),
            45,
            "three rows of 15 east_west rails"
        );
        assert_eq!(
            placements
                .iter()
                .filter(|p| p.block == "minecraft:powered_rail")
                .count(),
            3,
            "one powered rail in each of the three rows"
        );
    }
}

/// Slice 5: the fourth part, the rail bend. These tests are written before the
/// part data exists so the gap is proved red first: `RAILS_BEND_PART_JSON` is
/// unresolved until the part is carried verbatim from the cached jar. Every id
/// and count below is read from the artefact, not from the brief.
#[cfg(test)]
mod bend_tests {
    use super::*;

    /// The jar's `parts/rails_bend.json` is 16x16 with six slices, all rows the
    /// declared width — the same multi-layer bound as the other two six-slice
    /// parts, checked against the bend's own bytes.
    #[test]
    fn embedded_bend_part_has_six_16x16_layers() {
        let part =
            Part::from_json(&parse_json(RAILS_BEND_PART_JSON).expect("jar bend part parses"))
                .expect("jar bend part decodes");
        assert_eq!(part.xsize, 16, "jar xsize");
        assert_eq!(part.zsize, 16, "jar zsize");
        assert_eq!(part.slices.len(), 6, "the bend has six slices");
        for (y, layer) in part.slices.iter().enumerate() {
            assert_eq!(layer.len(), 16, "slice {y} has 16 rows");
            for (z, row) in layer.iter().enumerate() {
                assert_eq!(row.chars().count(), 16, "slice {y} row {z} is 16 chars");
            }
        }
    }

    /// The bend's rail chars resolve to the exact ids of `palettes/rails.json`:
    /// `k` north_south, `r` north_west, `s` south_west, `m` powered north_south,
    /// beside `e`/`f` east_west. The horizontal line writes only east_west
    /// rails, so a north_south count can only come from the bend — the
    /// differential the in-game assertion uses.
    #[test]
    fn bend_chars_resolve_to_the_jar_rail_shapes() {
        let variants = VariantTable::from_files(&[("stonebrick", STONEBRICK_VARIANT_JSON)])
            .expect("the jar's stonebrick variant decodes");
        let placements =
            decode_city_with_variants(RAILS_BEND_PART_JSON, RAILS_PALETTE_JSON, &variants)
                .expect("the embedded jar bend part and merged palette decode");

        let shape = |dx: usize, dz: usize| -> (String, Vec<(String, String)>) {
            let p = placements
                .iter()
                .find(|p| p.dx == dx && p.dy == 1 && p.dz == dz)
                .expect("a slice-1 bend cell at the expected coordinate");
            (p.block.clone(), p.properties.clone())
        };
        let (id, props) = shape(8, 0);
        assert_eq!(id, "minecraft:rail", "the k char is a rail");
        assert_eq!(
            props,
            vec![("shape".to_string(), "north_south".to_string())],
            "the k char is palettes/rails.json north_south rail"
        );
        let (id, props) = shape(8, 5);
        assert_eq!(id, "minecraft:rail", "the r char is a rail");
        assert_eq!(
            props,
            vec![("shape".to_string(), "north_west".to_string())],
            "the r char is palettes/rails.json north_west rail"
        );
        let (id, props) = shape(8, 9);
        assert_eq!(id, "minecraft:rail", "the s char is a rail");
        assert_eq!(
            props,
            vec![("shape".to_string(), "south_west".to_string())],
            "the s char is palettes/rails.json south_west rail"
        );
        let (id, props) = shape(8, 4);
        assert_eq!(id, "minecraft:powered_rail", "the m char is a powered rail");
        assert_eq!(
            props,
            vec![
                ("powered".to_string(), "true".to_string()),
                ("shape".to_string(), "north_south".to_string())
            ],
            "the m char is palettes/rails.json powered north_south rail"
        );

        // Exact char counts of the artefact's slice 1, cross-checked against
        // the jar: 9 k, 2 m, 1 r, 1 s, 14 e, 2 f.
        let count = |block: &str, shape: &str| {
            placements
                .iter()
                .filter(|p| {
                    p.block == block && p.properties.iter().any(|(k, v)| k == "shape" && v == shape)
                })
                .count()
        };
        assert_eq!(count("minecraft:rail", "north_south"), 9, "nine k cells");
        assert_eq!(
            count("minecraft:powered_rail", "north_south"),
            2,
            "two m cells"
        );
        assert_eq!(count("minecraft:rail", "north_west"), 1, "one r cell");
        assert_eq!(count("minecraft:rail", "south_west"), 1, "one s cell");
        assert_eq!(count("minecraft:rail", "east_west"), 14, "fourteen e cells");
        assert_eq!(
            count("minecraft:powered_rail", "east_west"),
            2,
            "two f cells"
        );

        // Differential: the horizontal line writes east_west rails only, so
        // its decoded placements contain no north_south rail to confuse the
        // in-game fill assertion.
        let horizontal = decode_city_with_variants(RAILS_PART_JSON, RAILS_PALETTE_JSON, &variants)
            .expect("the embedded horizontal rail part decodes");
        assert_eq!(
            horizontal
                .iter()
                .filter(|p| {
                    p.block == "minecraft:rail"
                        && p.properties
                            .iter()
                            .any(|(k, v)| k == "shape" && v == "north_south")
                })
                .count(),
            0,
            "the horizontal line must not write a north_south rail"
        );
    }
}

/// Slice 6: the fifth part, the three-way rail split. These tests are written
/// before the part data exists so the gap is proved red first:
/// `RAILS_3SPLIT_PART_JSON` is unresolved until the part is carried verbatim
/// from the cached jar. Every id and count below is read from the artefact —
/// in particular `n` = north_east and `q` = south_east from `palettes/rails.json`,
/// two shapes none of the first four decoded parts writes.
#[cfg(test)]
mod split_tests {
    use super::*;

    /// The jar's `parts/rails_3split.json` is 16x16 with six slices, all rows
    /// the declared width — the same multi-layer bound as the other parts.
    #[test]
    fn embedded_split_part_has_six_16x16_layers() {
        let part =
            Part::from_json(&parse_json(RAILS_3SPLIT_PART_JSON).expect("jar split part parses"))
                .expect("jar split part decodes");
        assert_eq!(part.xsize, 16, "jar xsize");
        assert_eq!(part.zsize, 16, "jar zsize");
        assert_eq!(part.slices.len(), 6, "the split has six slices");
        for (y, layer) in part.slices.iter().enumerate() {
            assert_eq!(layer.len(), 16, "slice {y} has 16 rows");
            for (z, row) in layer.iter().enumerate() {
                assert_eq!(row.chars().count(), 16, "slice {y} row {z} is 16 chars");
            }
        }
    }

    /// The split's `n` and `q` chars resolve to the exact north_east and
    /// south_east rail ids of `palettes/rails.json` — the two shapes the in-game
    /// fill assertion detects, and which no other decoded part writes. Every
    /// count is the artefact's per-part census, not the whole filled window.
    #[test]
    fn split_chars_resolve_to_the_jar_rail_shapes() {
        let variants = VariantTable::from_files(&[("stonebrick", STONEBRICK_VARIANT_JSON)])
            .expect("the jar's stonebrick variant decodes");
        let placements =
            decode_city_with_variants(RAILS_3SPLIT_PART_JSON, RAILS_PALETTE_JSON, &variants)
                .expect("the embedded jar split part and merged palette decode");

        let shape = |dx: usize, dz: usize| -> (String, Vec<(String, String)>) {
            let p = placements
                .iter()
                .find(|p| p.dx == dx && p.dy == 1 && p.dz == dz)
                .expect("a slice-1 split cell at the expected coordinate");
            (p.block.clone(), p.properties.clone())
        };
        let (id, props) = shape(8, 5);
        assert_eq!(id, "minecraft:rail", "the n char is a rail");
        assert_eq!(
            props,
            vec![("shape".to_string(), "north_east".to_string())],
            "the n char is palettes/rails.json north_east rail"
        );
        let (id, props) = shape(8, 9);
        assert_eq!(id, "minecraft:rail", "the q char is a rail");
        assert_eq!(
            props,
            vec![("shape".to_string(), "south_east".to_string())],
            "the q char is palettes/rails.json south_east rail"
        );

        // Exact per-part census of the artefact: 1 n, 1 q, 9 k, 2 m, 26 e,
        // 4 f, and no r (north_west) or s (south_west) char.
        let count = |block: &str, shape: &str| {
            placements
                .iter()
                .filter(|p| {
                    p.block == block && p.properties.iter().any(|(k, v)| k == "shape" && v == shape)
                })
                .count()
        };
        assert_eq!(count("minecraft:rail", "north_east"), 1, "one n cell");
        assert_eq!(count("minecraft:rail", "south_east"), 1, "one q cell");
        assert_eq!(count("minecraft:rail", "north_south"), 9, "nine k cells");
        assert_eq!(
            count("minecraft:powered_rail", "north_south"),
            2,
            "two m cells"
        );
        assert_eq!(
            count("minecraft:rail", "east_west"),
            26,
            "twenty-six e cells"
        );
        assert_eq!(
            count("minecraft:powered_rail", "east_west"),
            4,
            "four f cells"
        );
        assert_eq!(count("minecraft:rail", "north_west"), 0, "no r cells");
        assert_eq!(count("minecraft:rail", "south_west"), 0, "no s cells");

        // Differential: none of the first four decoded parts writes either
        // east-facing shape, so an in-game north_east/south_east rail count can
        // only come from this part.
        let has_east_shape = |ps: &[Placement]| {
            ps.iter().any(|p| {
                p.properties
                    .iter()
                    .any(|(k, v)| k == "shape" && (v == "north_east" || v == "south_east"))
            })
        };
        let street = decode_city(STREET_PART_JSON, STREET_PALETTE_JSON)
            .expect("the embedded street part decodes");
        let building =
            decode_city_with_variants(BUILDING_PART_JSON, BUILDING_PALETTE_JSON, &variants)
                .expect("the embedded building part decodes");
        let horizontal = decode_city_with_variants(RAILS_PART_JSON, RAILS_PALETTE_JSON, &variants)
            .expect("the embedded horizontal rail part decodes");
        let bend = decode_city_with_variants(RAILS_BEND_PART_JSON, RAILS_PALETTE_JSON, &variants)
            .expect("the embedded rail bend part decodes");
        assert!(
            !has_east_shape(&street),
            "the street must not write an east-facing rail"
        );
        assert!(
            !has_east_shape(&building),
            "the building must not write an east-facing rail"
        );
        assert!(
            !has_east_shape(&horizontal),
            "the horizontal line must not write an east-facing rail"
        );
        assert!(
            !has_east_shape(&bend),
            "the bend must not write an east-facing rail"
        );
    }
}

/// Slice 7: the sixth part, the eighteen-layer rail descent. These tests are
/// written before the part data exists so the gap is proved red first:
/// `RAILS_DOWN2_PART_JSON` is unresolved until the part is carried verbatim
/// from the cached jar. Every id and count below is read from the artefact —
/// in particular `j` = `rail[shape=ascending_east]` and `i` =
/// `powered_rail[powered=true,shape=ascending_east]` from `palettes/rails.json`,
/// two shapes none of the first five decoded parts writes.
#[cfg(test)]
mod down2_tests {
    use super::*;

    /// The jar's `parts/rails_down2.json` is 16x16 with eighteen slices, all
    /// rows the declared width — the same bound check as the other parts, just
    /// a taller stack spanning y=4..21 when laid at the sixth Y.
    #[test]
    fn embedded_down2_part_has_eighteen_16x16_layers() {
        let part =
            Part::from_json(&parse_json(RAILS_DOWN2_PART_JSON).expect("jar down2 part parses"))
                .expect("jar down2 part decodes");
        assert_eq!(part.xsize, 16, "jar xsize");
        assert_eq!(part.zsize, 16, "jar zsize");
        assert_eq!(part.slices.len(), 18, "the descent has eighteen slices");
        for (y, layer) in part.slices.iter().enumerate() {
            assert_eq!(layer.len(), 16, "slice {y} has 16 rows");
            for (z, row) in layer.iter().enumerate() {
                assert_eq!(row.chars().count(), 16, "slice {y} row {z} is 16 chars");
            }
        }
    }

    /// The descent's `j` and `i` chars resolve to the exact ascending_east rail
    /// ids of `palettes/rails.json` — the two shapes the in-game fill assertion
    /// detects, and which no other decoded part writes. Every count is the
    /// artefact's per-part census, not the whole filled window.
    #[test]
    fn down2_chars_resolve_to_the_jar_ascending_rail_shapes() {
        let variants = VariantTable::from_files(&[("stonebrick", STONEBRICK_VARIANT_JSON)])
            .expect("the jar's stonebrick variant decodes");
        let placements =
            decode_city_with_variants(RAILS_DOWN2_PART_JSON, RAILS_PALETTE_JSON, &variants)
                .expect("the embedded jar down2 part and merged palette decode");

        let cell = |dx: usize, dy: usize, dz: usize| -> (String, Vec<(String, String)>) {
            let p = placements
                .iter()
                .find(|p| p.dx == dx && p.dy == dy && p.dz == dz)
                .expect("a down2 cell at the expected coordinate");
            (p.block.clone(), p.properties.clone())
        };
        let (id, props) = cell(3, 3, 5);
        assert_eq!(id, "minecraft:rail", "the j char is a rail");
        assert_eq!(
            props,
            vec![("shape".to_string(), "ascending_east".to_string())],
            "the j char is palettes/rails.json ascending_east rail"
        );
        let (id, props) = cell(1, 1, 5);
        assert_eq!(id, "minecraft:powered_rail", "the i char is a powered rail");
        assert_eq!(
            props,
            vec![
                ("powered".to_string(), "true".to_string()),
                ("shape".to_string(), "ascending_east".to_string()),
            ],
            "the i char is palettes/rails.json powered ascending_east rail"
        );

        // Exact per-part census of the artefact's 978 non-space cells: 21 j,
        // 15 i, 12 f and 6 g. The y/~/space entries are already pinned by the
        // earlier parts' tests through the same variant/palette tables.
        let count = |block: &str, shape: &str| {
            placements
                .iter()
                .filter(|p| {
                    p.block == block && p.properties.iter().any(|(k, v)| k == "shape" && v == shape)
                })
                .count()
        };
        assert_eq!(
            count("minecraft:rail", "ascending_east"),
            21,
            "twenty-one j cells"
        );
        assert_eq!(
            count("minecraft:powered_rail", "ascending_east"),
            15,
            "fifteen i cells"
        );
        assert_eq!(
            count("minecraft:powered_rail", "east_west"),
            12,
            "twelve f cells"
        );

        // Differential: none of the first five decoded parts writes either
        // ascending_east shape, so an in-game ascending_east rail count can
        // only come from this part.
        let has_ascending_east = |ps: &[Placement]| {
            ps.iter().any(|p| {
                p.properties
                    .iter()
                    .any(|(k, v)| k == "shape" && v == "ascending_east")
            })
        };
        let street = decode_city(STREET_PART_JSON, STREET_PALETTE_JSON)
            .expect("the embedded street part decodes");
        let building =
            decode_city_with_variants(BUILDING_PART_JSON, BUILDING_PALETTE_JSON, &variants)
                .expect("the embedded building part decodes");
        let horizontal = decode_city_with_variants(RAILS_PART_JSON, RAILS_PALETTE_JSON, &variants)
            .expect("the embedded horizontal rail part decodes");
        let bend = decode_city_with_variants(RAILS_BEND_PART_JSON, RAILS_PALETTE_JSON, &variants)
            .expect("the embedded rail bend part decodes");
        let split =
            decode_city_with_variants(RAILS_3SPLIT_PART_JSON, RAILS_PALETTE_JSON, &variants)
                .expect("the embedded rail split part decodes");
        for (name, ps) in [
            ("street", &street),
            ("building", &building),
            ("horizontal line", &horizontal),
            ("bend", &bend),
            ("split", &split),
        ] {
            assert!(
                !has_ascending_east(ps),
                "the {name} must not write an ascending_east rail"
            );
        }
    }
}

/// Slice 8: the seventh part, the twelve-layer first rail descent. These tests
/// are written before the part data exists so the gap is proved red first:
/// `RAILS_DOWN1_PART_JSON` and `RAILS_DOWN1_PALETTE_JSON` are unresolved until
/// the part and its two extra palette entries are carried verbatim from the
/// cached jar. Every id and count below is read from the artefact — in
/// particular `:` = `minecraft:iron_bars` and `_` =
/// `minecraft:smooth_stone_slab[type=bottom]` from `palettes/default.json`, two
/// entries none of the earlier palettes place.
#[cfg(test)]
mod down1_tests {
    use super::*;

    /// The jar's `parts/rails_down1.json` is 16x16 with twelve slices, all rows
    /// the declared width — the same bound check as the other parts, but a
    /// different layer count from both the six-layer parts and the 18-layer
    /// descent laid below it.
    #[test]
    fn embedded_down1_part_has_twelve_16x16_layers() {
        let part =
            Part::from_json(&parse_json(RAILS_DOWN1_PART_JSON).expect("jar down1 part parses"))
                .expect("jar down1 part decodes");
        assert_eq!(part.xsize, 16, "jar xsize");
        assert_eq!(part.zsize, 16, "jar zsize");
        assert_eq!(part.slices.len(), 12, "the first descent has twelve slices");
        for (y, layer) in part.slices.iter().enumerate() {
            assert_eq!(layer.len(), 16, "slice {y} has 16 rows");
            for (z, row) in layer.iter().enumerate() {
                assert_eq!(row.chars().count(), 16, "slice {y} row {z} is 16 chars");
            }
        }
    }

    /// The descent's `:` and `_` chars resolve to the exact ids of
    /// `palettes/default.json` — the two entries absent from the carried rail
    /// palette. `iron_bars` is placed by no other decoded part (its palettes
    /// only name it as a `damaged` field, which the decoder does not place), so
    /// it is the clean in-game differential. Every count is the artefact's
    /// per-part census, not the whole filled window.
    #[test]
    fn down1_chars_resolve_to_the_jar_iron_bars_and_bottom_slab() {
        let variants = VariantTable::from_files(&[("stonebrick", STONEBRICK_VARIANT_JSON)])
            .expect("the jar's stonebrick variant decodes");
        let placements =
            decode_city_with_variants(RAILS_DOWN1_PART_JSON, RAILS_DOWN1_PALETTE_JSON, &variants)
                .expect("the embedded jar down1 part and merged palette decode");

        let cell = |dx: usize, dy: usize, dz: usize| -> (String, Vec<(String, String)>) {
            let p = placements
                .iter()
                .find(|p| p.dx == dx && p.dy == dy && p.dz == dz)
                .expect("a down1 cell at the expected coordinate");
            (p.block.clone(), p.properties.clone())
        };
        let (id, props) = cell(1, 8, 4);
        assert_eq!(id, "minecraft:iron_bars", "the : char is iron bars");
        assert!(
            props.is_empty(),
            "the : char has no state properties in palettes/default.json"
        );
        let (id, props) = cell(1, 11, 3);
        assert_eq!(
            id, "minecraft:smooth_stone_slab",
            "the _ char is a smooth stone slab"
        );
        assert_eq!(
            props,
            vec![("type".to_string(), "bottom".to_string())],
            "the _ char is the bottom slab of palettes/default.json"
        );

        // Exact per-part census of the artefact's 911 non-space cells: 30 `:`,
        // 26 `_`, 27 `e`, 3 `f`, 2 `g`, 18 `j`, 279 `y`, 526 `~`.
        let count = |id: &str| placements.iter().filter(|p| p.block == id).count();
        assert_eq!(count("minecraft:iron_bars"), 30, "thirty : cells");
        assert_eq!(
            count("minecraft:smooth_stone_slab"),
            26,
            "twenty-six _ cells"
        );

        // Differential: no other decoded part writes iron bars, so an in-game
        // iron-bar count at the seventh Y can only come from this part.
        let has_bars = |ps: &[Placement]| ps.iter().any(|p| p.block == "minecraft:iron_bars");
        let street = decode_city(STREET_PART_JSON, STREET_PALETTE_JSON)
            .expect("the embedded street part decodes");
        let building =
            decode_city_with_variants(BUILDING_PART_JSON, BUILDING_PALETTE_JSON, &variants)
                .expect("the embedded building part decodes");
        let horizontal = decode_city_with_variants(RAILS_PART_JSON, RAILS_PALETTE_JSON, &variants)
            .expect("the embedded horizontal rail part decodes");
        let bend = decode_city_with_variants(RAILS_BEND_PART_JSON, RAILS_PALETTE_JSON, &variants)
            .expect("the embedded rail bend part decodes");
        let split =
            decode_city_with_variants(RAILS_3SPLIT_PART_JSON, RAILS_PALETTE_JSON, &variants)
                .expect("the embedded rail split part decodes");
        let down2 = decode_city_with_variants(RAILS_DOWN2_PART_JSON, RAILS_PALETTE_JSON, &variants)
            .expect("the embedded rail descent part decodes");
        for (name, ps) in [
            ("street", &street),
            ("building", &building),
            ("horizontal line", &horizontal),
            ("bend", &bend),
            ("split", &split),
            ("eighteen-layer descent", &down2),
        ] {
            assert!(!has_bars(ps), "the {name} must not write an iron bar");
        }
    }
}

/// Slice 9: the eighth part, the second six-layer building. These tests are
/// written before the part data exists so the gap is proved red first:
/// `BUILDING_1_2_PART_JSON` and `BUILDING_1_2_PALETTE_JSON` are unresolved until
/// the part and its two extra palette entries are carried verbatim from the jar.
/// Every id and count below is read from the artefact, not from the brief — in
/// particular `Q` = `minecraft:quartz_block` from `palettes/default.json`, which
/// no other decoded part writes.
#[cfg(test)]
mod building1_2_tests {
    use super::*;

    /// The jar's `parts/building1_2.json` is 16x16 with six slices, all rows the
    /// declared width — the same bound check as the first building part.
    #[test]
    fn embedded_building12_part_has_six_16x16_layers() {
        let part = Part::from_json(
            &parse_json(BUILDING_1_2_PART_JSON).expect("jar building1_2 part parses"),
        )
        .expect("jar building1_2 part decodes");
        assert_eq!(part.xsize, 16, "jar xsize");
        assert_eq!(part.zsize, 16, "jar zsize");
        assert_eq!(part.slices.len(), 6, "the second building has six slices");
        for (y, layer) in part.slices.iter().enumerate() {
            assert_eq!(layer.len(), 16, "slice {y} has 16 rows");
            for (z, row) in layer.iter().enumerate() {
                assert_eq!(row.chars().count(), 16, "slice {y} row {z} is 16 chars");
            }
        }
    }

    /// The second building's `Q` and `:` chars resolve to the exact ids of
    /// `palettes/default.json` — the two entries absent from the carried
    /// building palette. Quartz is placed by no other decoded part, so it is
    /// the clean in-game differential. Every count is the artefact's per-part
    /// census (998 non-space cells), not the whole filled window.
    #[test]
    fn building12_chars_resolve_to_the_jar_quartz_block() {
        let variants = VariantTable::from_files(&[("stonebrick", STONEBRICK_VARIANT_JSON)])
            .expect("the jar's stonebrick variant decodes");
        let placements =
            decode_city_with_variants(BUILDING_1_2_PART_JSON, BUILDING_1_2_PALETTE_JSON, &variants)
                .expect("the embedded jar building1_2 part and merged palette decode");

        let cell = |dx: usize, dy: usize, dz: usize| -> (String, Vec<(String, String)>) {
            let p = placements
                .iter()
                .find(|p| p.dx == dx && p.dy == dy && p.dz == dz)
                .expect("a building1_2 cell at the expected coordinate");
            (p.block.clone(), p.properties.clone())
        };
        let (id, props) = cell(10, 1, 1);
        assert_eq!(id, "minecraft:quartz_block", "the Q char is a quartz block");
        assert!(
            props.is_empty(),
            "the Q char has no state properties in palettes/default.json"
        );
        let (id, props) = cell(2, 2, 10);
        assert_eq!(id, "minecraft:iron_bars", "the : char is iron bars");
        assert!(
            props.is_empty(),
            "the : char has no state properties in palettes/default.json"
        );

        let count = |id: &str| placements.iter().filter(|p| p.block == id).count();
        assert_eq!(count("minecraft:quartz_block"), 3, "three Q cells");
        assert_eq!(count("minecraft:iron_bars"), 2, "two : cells");

        // Differential: none of the first seven decoded parts writes a quartz
        // block, so an in-game quartz count at the eighth Y can only come from
        // this part.
        let street = decode_city(STREET_PART_JSON, STREET_PALETTE_JSON)
            .expect("the embedded street part decodes");
        let building =
            decode_city_with_variants(BUILDING_PART_JSON, BUILDING_PALETTE_JSON, &variants)
                .expect("the embedded first building part decodes");
        let horizontal = decode_city_with_variants(RAILS_PART_JSON, RAILS_PALETTE_JSON, &variants)
            .expect("the embedded horizontal rail part decodes");
        let bend = decode_city_with_variants(RAILS_BEND_PART_JSON, RAILS_PALETTE_JSON, &variants)
            .expect("the embedded rail bend part decodes");
        let split =
            decode_city_with_variants(RAILS_3SPLIT_PART_JSON, RAILS_PALETTE_JSON, &variants)
                .expect("the embedded rail split part decodes");
        let down2 = decode_city_with_variants(RAILS_DOWN2_PART_JSON, RAILS_PALETTE_JSON, &variants)
            .expect("the embedded rail descent part decodes");
        let down1 =
            decode_city_with_variants(RAILS_DOWN1_PART_JSON, RAILS_DOWN1_PALETTE_JSON, &variants)
                .expect("the embedded first descent part decodes");
        for (name, ps) in [
            ("street", &street),
            ("first building", &building),
            ("horizontal line", &horizontal),
            ("bend", &bend),
            ("split", &split),
            ("eighteen-layer descent", &down2),
            ("first descent", &down1),
        ] {
            assert!(
                !ps.iter().any(|p| p.block == "minecraft:quartz_block"),
                "the {name} must not write a quartz block"
            );
        }
    }
}

/// Slice 10: the ninth part, the six-layer city centre. These tests are
/// written before the part data exists so the gap is proved red first:
/// `CENTER00_1_PART_JSON` and `CENTER00_1_PALETTE_JSON` are unresolved until
/// the part and its extra `palettes/default.json` entries are carried verbatim
/// from the cached jar. Every id and count below is read from the artefact, not
/// from the brief — in particular `)` = `minecraft:quartz_stairs[facing=south,
/// half=bottom,shape=straight]`, which no other decoded part writes.
#[cfg(test)]
mod center00_1_tests {
    use super::*;

    /// The jar's `parts/center00_1.json` is 16x16 with six slices, all rows the
    /// declared width — the same bound check as the other six-layer parts.
    #[test]
    fn embedded_center001_part_has_six_16x16_layers() {
        let part =
            Part::from_json(&parse_json(CENTER00_1_PART_JSON).expect("jar center00_1 part parses"))
                .expect("jar center00_1 part decodes");
        assert_eq!(part.xsize, 16, "jar xsize");
        assert_eq!(part.zsize, 16, "jar zsize");
        assert_eq!(part.slices.len(), 6, "the city centre has six slices");
        for (y, layer) in part.slices.iter().enumerate() {
            assert_eq!(layer.len(), 16, "slice {y} has 16 rows");
            for (z, row) in layer.iter().enumerate() {
                assert_eq!(row.chars().count(), 16, "slice {y} row {z} is 16 chars");
            }
        }
    }

    /// The centre's `)` char resolves to the exact quartz-stairs id of
    /// `palettes/default.json`, a block written by no other decoded part, so an
    /// in-game quartz-stairs count at the ninth Y can only come from this part.
    /// Every count is the artefact's per-part census (680 non-space cells).
    #[test]
    fn center001_chars_resolve_to_the_jar_quartz_stairs() {
        let variants = VariantTable::from_files(&[("stonebrick", STONEBRICK_VARIANT_JSON)])
            .expect("the jar's stonebrick variant decodes");
        let placements =
            decode_city_with_variants(CENTER00_1_PART_JSON, CENTER00_1_PALETTE_JSON, &variants)
                .expect("the embedded jar center00_1 part and merged palette decode");

        let cell = |dx: usize, dy: usize, dz: usize| -> (String, Vec<(String, String)>) {
            let p = placements
                .iter()
                .find(|p| p.dx == dx && p.dy == dy && p.dz == dz)
                .expect("a center00_1 cell at the expected coordinate");
            (p.block.clone(), p.properties.clone())
        };
        let (id, props) = cell(13, 0, 14);
        assert_eq!(id, "minecraft:quartz_stairs", "the ) char is quartz stairs");
        assert_eq!(
            props,
            vec![
                ("facing".to_string(), "south".to_string()),
                ("half".to_string(), "bottom".to_string()),
                ("shape".to_string(), "straight".to_string()),
            ],
            "the ) char is palettes/default.json's south-facing quartz stairs"
        );

        let count = |id: &str| placements.iter().filter(|p| p.block == id).count();
        assert_eq!(count("minecraft:quartz_stairs"), 12, "twelve ) cells");
        assert_eq!(count("minecraft:quartz_block"), 107, "107 Q cells");
        assert_eq!(count("minecraft:smooth_stone_slab"), 97, "97 S cells");

        // Differential: none of the first eight decoded parts writes a quartz
        // stair, so the in-game fill at the ninth Y cannot be confused by them.
        let street = decode_city(STREET_PART_JSON, STREET_PALETTE_JSON)
            .expect("the embedded street part decodes");
        let building =
            decode_city_with_variants(BUILDING_PART_JSON, BUILDING_PALETTE_JSON, &variants)
                .expect("the embedded first building part decodes");
        let horizontal = decode_city_with_variants(RAILS_PART_JSON, RAILS_PALETTE_JSON, &variants)
            .expect("the embedded horizontal rail part decodes");
        let bend = decode_city_with_variants(RAILS_BEND_PART_JSON, RAILS_PALETTE_JSON, &variants)
            .expect("the embedded rail bend part decodes");
        let split =
            decode_city_with_variants(RAILS_3SPLIT_PART_JSON, RAILS_PALETTE_JSON, &variants)
                .expect("the embedded rail split part decodes");
        let down2 = decode_city_with_variants(RAILS_DOWN2_PART_JSON, RAILS_PALETTE_JSON, &variants)
            .expect("the embedded rail descent part decodes");
        let down1 =
            decode_city_with_variants(RAILS_DOWN1_PART_JSON, RAILS_DOWN1_PALETTE_JSON, &variants)
                .expect("the embedded first descent part decodes");
        let building12 =
            decode_city_with_variants(BUILDING_1_2_PART_JSON, BUILDING_1_2_PALETTE_JSON, &variants)
                .expect("the embedded second building part decodes");
        for (name, ps) in [
            ("street", &street),
            ("first building", &building),
            ("horizontal line", &horizontal),
            ("bend", &bend),
            ("split", &split),
            ("eighteen-layer descent", &down2),
            ("first descent", &down1),
            ("second building", &building12),
        ] {
            assert!(
                !ps.iter().any(|p| p.block == "minecraft:quartz_stairs"),
                "the {name} must not write a quartz stair"
            );
        }
    }
}

/// Slice 11: the tenth part, the six-layer shopping-open front. These tests are
/// written before the part data exists so the gap is proved red first:
/// `SHOPPING_OPEN01_1_PART_JSON` and `SHOPPING_OPEN01_1_PALETTE_JSON` are
/// unresolved until the part and its verbatim `palettes/default.json` entry are
/// carried from the jar. Every id and count below is read from the artefact — in
/// particular `w` = `minecraft:cobblestone_wall`, which no other decoded part
/// writes (its palettes never name a wall).
#[cfg(test)]
mod shopping_open01_1_tests {
    use super::*;

    /// The jar's `parts/shopping_open01_1.json` is 16x16 with six slices, all
    /// rows the declared width — the same bound check as the other six-layer
    /// parts. It carries a private `meta` key the decoder ignores.
    #[test]
    fn embedded_shopping1_part_has_six_16x16_layers() {
        let part = Part::from_json(
            &parse_json(SHOPPING_OPEN01_1_PART_JSON).expect("jar shopping_open01_1 part parses"),
        )
        .expect("jar shopping_open01_1 part decodes");
        assert_eq!(part.xsize, 16, "jar xsize");
        assert_eq!(part.zsize, 16, "jar zsize");
        assert_eq!(part.slices.len(), 6, "the shopping part has six slices");
        for (y, layer) in part.slices.iter().enumerate() {
            assert_eq!(layer.len(), 16, "slice {y} has 16 rows");
            for (z, row) in layer.iter().enumerate() {
                assert_eq!(row.chars().count(), 16, "slice {y} row {z} is 16 chars");
            }
        }
    }

    /// The part's twelve `w` cells (x=1, z=0 or 14, y=0..5) resolve to the
    /// exact `minecraft:cobblestone_wall` id of `palettes/default.json`, a block
    /// no other decoded part writes, so an in-game cobblestone-wall count at
    /// the tenth Y can only come from this part. Every count is the artefact's
    /// per-part census, not the whole filled window.
    #[test]
    fn shopping1_chars_resolve_to_the_jar_cobblestone_wall() {
        let variants = VariantTable::from_files(&[("stonebrick", STONEBRICK_VARIANT_JSON)])
            .expect("the jar's stonebrick variant decodes");
        let placements = decode_city_with_variants(
            SHOPPING_OPEN01_1_PART_JSON,
            SHOPPING_OPEN01_1_PALETTE_JSON,
            &variants,
        )
        .expect("the embedded jar shopping part and merged palette decode");

        let cell = placements
            .iter()
            .find(|p| p.dx == 1 && p.dy == 0 && p.dz == 0)
            .expect("a w cell at (1,0,0)");
        assert_eq!(
            cell.block, "minecraft:cobblestone_wall",
            "the w char is a cobblestone wall"
        );
        assert!(
            cell.properties.is_empty(),
            "the w char has no state properties in palettes/default.json"
        );

        let count = |id: &str| placements.iter().filter(|p| p.block == id).count();
        assert_eq!(count("minecraft:cobblestone_wall"), 12, "twelve w cells");
        assert_eq!(placements.len(), 12, "only the twelve w cells are placed");

        // Differential: none of the first nine decoded parts writes a cobblestone
        // wall, so the in-game fill at the tenth Y cannot be confused by them.
        let street = decode_city(STREET_PART_JSON, STREET_PALETTE_JSON)
            .expect("the embedded street part decodes");
        let building =
            decode_city_with_variants(BUILDING_PART_JSON, BUILDING_PALETTE_JSON, &variants)
                .expect("the embedded first building part decodes");
        let horizontal = decode_city_with_variants(RAILS_PART_JSON, RAILS_PALETTE_JSON, &variants)
            .expect("the embedded horizontal rail part decodes");
        let bend = decode_city_with_variants(RAILS_BEND_PART_JSON, RAILS_PALETTE_JSON, &variants)
            .expect("the embedded rail bend part decodes");
        let split =
            decode_city_with_variants(RAILS_3SPLIT_PART_JSON, RAILS_PALETTE_JSON, &variants)
                .expect("the embedded rail split part decodes");
        let down2 = decode_city_with_variants(RAILS_DOWN2_PART_JSON, RAILS_PALETTE_JSON, &variants)
            .expect("the embedded rail descent part decodes");
        let down1 =
            decode_city_with_variants(RAILS_DOWN1_PART_JSON, RAILS_DOWN1_PALETTE_JSON, &variants)
                .expect("the embedded first descent part decodes");
        let building12 =
            decode_city_with_variants(BUILDING_1_2_PART_JSON, BUILDING_1_2_PALETTE_JSON, &variants)
                .expect("the embedded second building part decodes");
        let center001 =
            decode_city_with_variants(CENTER00_1_PART_JSON, CENTER00_1_PALETTE_JSON, &variants)
                .expect("the embedded city-centre part decodes");
        for (name, ps) in [
            ("street", &street),
            ("first building", &building),
            ("horizontal line", &horizontal),
            ("bend", &bend),
            ("split", &split),
            ("eighteen-layer descent", &down2),
            ("first descent", &down1),
            ("second building", &building12),
            ("city centre", &center001),
        ] {
            assert!(
                !ps.iter().any(|p| p.block == "minecraft:cobblestone_wall"),
                "the {name} must not write a cobblestone wall"
            );
        }
    }
}

/// Slice 12: the eleventh part, the seven-layer stone stair. These tests are
/// written before the part data exists so the gap is proved red first:
/// `STAIRSNORMAL_PART_JSON` and `STAIRSNORMAL_PALETTE_JSON` are unresolved until
/// the part and its three verbatim `palettes/default.json` entries are carried
/// from the jar. Every id and count below is read from the artefact — in
/// particular `/` = `minecraft:stone_stairs[facing=west,half=bottom,shape=straight]`,
/// which no other decoded part writes.
#[cfg(test)]
mod stairsnormal_tests {
    use super::*;

    /// The jar's `parts/stairsnormal.json` is 6x11 with seven slices, every row
    /// six chars and every slice eleven rows — a non-16 footprint the generic
    /// bound check must still accept. Its private `meta` key is ignored.
    #[test]
    fn embedded_stairsnormal_part_has_seven_6x11_layers() {
        let part = Part::from_json(
            &parse_json(STAIRSNORMAL_PART_JSON).expect("jar stairsnormal part parses"),
        )
        .expect("jar stairsnormal part decodes");
        assert_eq!(part.xsize, 6, "jar xsize");
        assert_eq!(part.zsize, 11, "jar zsize");
        assert_eq!(part.slices.len(), 7, "the stair part has seven slices");
        for (y, layer) in part.slices.iter().enumerate() {
            assert_eq!(layer.len(), 11, "slice {y} has 11 rows");
            for (z, row) in layer.iter().enumerate() {
                assert_eq!(row.chars().count(), 6, "slice {y} row {z} is 6 chars");
            }
        }
    }

    /// The `/` char resolves to the exact stone-stairs id of
    /// `palettes/default.json`, a block no other decoded part writes, so an
    /// in-game stone-stairs count at the eleventh Y can only come from this
    /// part. Every count is the artefact's per-part census (138 non-space
    /// cells), not the whole filled window.
    #[test]
    fn stairsnormal_chars_resolve_to_the_jar_stone_stairs() {
        let variants = VariantTable::from_files(&[("stonebrick", STONEBRICK_VARIANT_JSON)])
            .expect("the jar's stonebrick variant decodes");
        let placements =
            decode_city_with_variants(STAIRSNORMAL_PART_JSON, STAIRSNORMAL_PALETTE_JSON, &variants)
                .expect("the embedded jar stairsnormal part and merged palette decode");

        let cell = placements
            .iter()
            .find(|p| p.dx == 5 && p.dy == 0 && p.dz == 6)
            .expect("a / cell at (5,0,6)");
        assert_eq!(
            cell.block, "minecraft:stone_stairs",
            "the / char is a stone stair"
        );
        assert_eq!(
            cell.properties,
            vec![
                ("facing".to_string(), "west".to_string()),
                ("half".to_string(), "bottom".to_string()),
                ("shape".to_string(), "straight".to_string()),
            ],
            "the / char is palettes/default.json's west-facing stone stairs"
        );

        let count = |id: &str| placements.iter().filter(|p| p.block == id).count();
        assert_eq!(count("minecraft:stone_stairs"), 24, "twenty-four / cells");
        assert_eq!(count("minecraft:smooth_stone_slab"), 12, "twelve _ cells");
        assert_eq!(placements.len(), 138, "102 x + 24 / + 12 _ cells");

        // Differential: none of the first ten decoded parts writes a stone
        // stair, so the in-game fill at the eleventh Y cannot be confused by
        // them.
        let street = decode_city(STREET_PART_JSON, STREET_PALETTE_JSON)
            .expect("the embedded street part decodes");
        let building =
            decode_city_with_variants(BUILDING_PART_JSON, BUILDING_PALETTE_JSON, &variants)
                .expect("the embedded first building part decodes");
        let horizontal = decode_city_with_variants(RAILS_PART_JSON, RAILS_PALETTE_JSON, &variants)
            .expect("the embedded horizontal rail part decodes");
        let bend = decode_city_with_variants(RAILS_BEND_PART_JSON, RAILS_PALETTE_JSON, &variants)
            .expect("the embedded rail bend part decodes");
        let split =
            decode_city_with_variants(RAILS_3SPLIT_PART_JSON, RAILS_PALETTE_JSON, &variants)
                .expect("the embedded rail split part decodes");
        let down2 = decode_city_with_variants(RAILS_DOWN2_PART_JSON, RAILS_PALETTE_JSON, &variants)
            .expect("the embedded rail descent part decodes");
        let down1 =
            decode_city_with_variants(RAILS_DOWN1_PART_JSON, RAILS_DOWN1_PALETTE_JSON, &variants)
                .expect("the embedded first descent part decodes");
        let building12 =
            decode_city_with_variants(BUILDING_1_2_PART_JSON, BUILDING_1_2_PALETTE_JSON, &variants)
                .expect("the embedded second building part decodes");
        let center001 =
            decode_city_with_variants(CENTER00_1_PART_JSON, CENTER00_1_PALETTE_JSON, &variants)
                .expect("the embedded city-centre part decodes");
        let shopping1 = decode_city_with_variants(
            SHOPPING_OPEN01_1_PART_JSON,
            SHOPPING_OPEN01_1_PALETTE_JSON,
            &variants,
        )
        .expect("the embedded shopping part decodes");
        for (name, ps) in [
            ("street", &street),
            ("first building", &building),
            ("horizontal line", &horizontal),
            ("bend", &bend),
            ("split", &split),
            ("eighteen-layer descent", &down2),
            ("first descent", &down1),
            ("second building", &building12),
            ("city centre", &center001),
            ("shopping part", &shopping1),
        ] {
            assert!(
                !ps.iter().any(|p| p.block == "minecraft:stone_stairs"),
                "the {name} must not write a stone stair"
            );
        }
    }
}

/// Slice 13: the twelfth part, the seven-layer large stone stair. These tests
/// are written before the part data exists so the gap is proved red first:
/// `STAIRSBIG_PART_JSON` is unresolved until the part bytes are carried from
/// the jar. Its three chars (`x`, `/`, `_`) are already declared verbatim in
/// `STAIRSNORMAL_PALETTE_JSON`, so no new palette entry or decoder form is
/// needed — only the part bytes are new.
#[cfg(test)]
mod stairsbig_tests {
    use super::*;

    /// The jar's `parts/stairsbig.json` is 6x12 with seven slices, every row six
    /// chars and every slice twelve rows — a non-16 footprint the generic bound
    /// check must still accept. Its private `meta` key is ignored.
    #[test]
    fn embedded_stairsbig_part_has_seven_6x12_layers() {
        let part =
            Part::from_json(&parse_json(STAIRSBIG_PART_JSON).expect("jar stairsbig part parses"))
                .expect("jar stairsbig part decodes");
        assert_eq!(part.xsize, 6, "jar xsize");
        assert_eq!(part.zsize, 12, "jar zsize");
        assert_eq!(
            part.slices.len(),
            7,
            "the large stair part has seven slices"
        );
        for (y, layer) in part.slices.iter().enumerate() {
            assert_eq!(layer.len(), 12, "slice {y} has 12 rows");
            for (z, row) in layer.iter().enumerate() {
                assert_eq!(row.chars().count(), 6, "slice {y} row {z} is 6 chars");
            }
        }
    }

    /// The `/` char resolves to the exact stone-stairs id of
    /// `palettes/default.json`, which this part shares with `stairsnormal`. The
    /// in-game differential is therefore a disjoint height band (y=164..170),
    /// not a new block id. Every count below is the artefact's per-part census
    /// (180 non-space cells), not the whole filled window.
    #[test]
    fn stairsbig_chars_resolve_to_the_jar_stone_stairs() {
        let variants = VariantTable::from_files(&[("stonebrick", STONEBRICK_VARIANT_JSON)])
            .expect("the jar's stonebrick variant decodes");
        let placements =
            decode_city_with_variants(STAIRSBIG_PART_JSON, STAIRSNORMAL_PALETTE_JSON, &variants)
                .expect("the embedded jar stairsbig part and verbatim stair palette decode");

        let cell = placements
            .iter()
            .find(|p| p.dx == 5 && p.dy == 0 && p.dz == 5)
            .expect("a / cell at (5,0,5)");
        assert_eq!(
            cell.block, "minecraft:stone_stairs",
            "the / char is a stone stair"
        );
        assert_eq!(
            cell.properties,
            vec![
                ("facing".to_string(), "west".to_string()),
                ("half".to_string(), "bottom".to_string()),
                ("shape".to_string(), "straight".to_string()),
            ],
            "the / char is palettes/default.json's west-facing stone stairs"
        );

        let count = |id: &str| placements.iter().filter(|p| p.block == id).count();
        assert_eq!(count("minecraft:stone_stairs"), 36, "thirty-six / cells");
        assert_eq!(count("minecraft:smooth_stone_slab"), 12, "twelve _ cells");
        assert_eq!(placements.len(), 180, "132 x + 36 / + 12 _ cells");

        // Differential: none of the first ten decoded parts writes a stone
        // stair. The eleventh (stairsnormal) writes only 24, so a count above
        // that in the disjoint y=164..170 band can only come from this part.
        let street = decode_city(STREET_PART_JSON, STREET_PALETTE_JSON)
            .expect("the embedded street part decodes");
        let building =
            decode_city_with_variants(BUILDING_PART_JSON, BUILDING_PALETTE_JSON, &variants)
                .expect("the embedded first building part decodes");
        let horizontal = decode_city_with_variants(RAILS_PART_JSON, RAILS_PALETTE_JSON, &variants)
            .expect("the embedded horizontal rail part decodes");
        let bend = decode_city_with_variants(RAILS_BEND_PART_JSON, RAILS_PALETTE_JSON, &variants)
            .expect("the embedded rail bend part decodes");
        let split =
            decode_city_with_variants(RAILS_3SPLIT_PART_JSON, RAILS_PALETTE_JSON, &variants)
                .expect("the embedded rail split part decodes");
        let down2 = decode_city_with_variants(RAILS_DOWN2_PART_JSON, RAILS_PALETTE_JSON, &variants)
            .expect("the embedded rail descent part decodes");
        let down1 =
            decode_city_with_variants(RAILS_DOWN1_PART_JSON, RAILS_DOWN1_PALETTE_JSON, &variants)
                .expect("the embedded first descent part decodes");
        let building12 =
            decode_city_with_variants(BUILDING_1_2_PART_JSON, BUILDING_1_2_PALETTE_JSON, &variants)
                .expect("the embedded second building part decodes");
        let center001 =
            decode_city_with_variants(CENTER00_1_PART_JSON, CENTER00_1_PALETTE_JSON, &variants)
                .expect("the embedded city-centre part decodes");
        let shopping1 = decode_city_with_variants(
            SHOPPING_OPEN01_1_PART_JSON,
            SHOPPING_OPEN01_1_PALETTE_JSON,
            &variants,
        )
        .expect("the embedded shopping part decodes");
        let stairsnormal =
            decode_city_with_variants(STAIRSNORMAL_PART_JSON, STAIRSNORMAL_PALETTE_JSON, &variants)
                .expect("the embedded stairsnormal part decodes");
        assert_eq!(
            stairsnormal
                .iter()
                .filter(|p| p.block == "minecraft:stone_stairs")
                .count(),
            24,
            "the earlier stair part writes only 24 stone stairs"
        );
        for (name, ps) in [
            ("street", &street),
            ("first building", &building),
            ("horizontal line", &horizontal),
            ("bend", &bend),
            ("split", &split),
            ("eighteen-layer descent", &down2),
            ("first descent", &down1),
            ("second building", &building12),
            ("city centre", &center001),
            ("shopping part", &shopping1),
        ] {
            assert!(
                !ps.iter().any(|p| p.block == "minecraft:stone_stairs"),
                "the {name} must not write a stone stair"
            );
        }
    }
}
