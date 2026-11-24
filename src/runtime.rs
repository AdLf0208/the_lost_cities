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
