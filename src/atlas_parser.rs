use std::str::FromStr;

use derive_builder::Builder;
use regex::Regex;

#[derive(Builder, Debug, Clone)]
pub struct Part {
    #[builder(setter(into))]
    pub name: String,
    // pub parameters: Vec<PartParameter>,
    pub xy: (u32, u32),
    #[builder(setter(strip_option))]
    pub rotate: Option<u32>,
    pub size: (u32, u32),
    #[builder(setter(strip_option))]
    pub origin: Option<(u32, u32)>,
    #[builder(setter(strip_option))]
    pub offset: Option<(i32, i32)>,
    #[builder(setter(strip_option))]
    pub index: Option<i32>
}

#[derive(Builder, Debug)]
pub struct Atlas {
    #[builder(setter(into))]
    pub image_name: String,
    // pub parameters: Vec<AtlasParameter>,
    pub size: (u32, u32),
    #[builder(default)]
    pub parts: Vec<Part>
}

struct ParserRegexes {
    part_parameter: Regex,
    atlas_parameter: Regex
}

impl ParserRegexes {
    pub fn new() -> Self {
        let part_parameter = 
            Regex::new(r"^\s+(\w+):\s*(.+)$").expect("the regex is valid");

        let atlas_parameter = 
            Regex::new(r"^(\w+):\s*(.+)$").expect("the regex is valid");

        Self {
            part_parameter,
            atlas_parameter
        }
    }
}

pub struct AtlasParser {
    image_name_found: bool,
    atlas_header_finished: bool,
    regexes: ParserRegexes,
    // current_atlas: Option<Atlas>,
    // current_part: Option<Part>
    current_atlas: AtlasBuilder,
    current_part: PartBuilder
}

fn split_str<T: FromStr + Default>(input: &str) -> Vec<T> {
    input
        .split(',')
        .map(|str| {
            str
                .trim()
                .parse::<T>()
                .unwrap_or_default()
        })
        .collect()
}

impl AtlasParser {
    pub fn new() -> Self {
        Self {
            image_name_found: false,
            atlas_header_finished: false,
            regexes: ParserRegexes::new(),
            current_atlas: AtlasBuilder::default(),
            current_part: PartBuilder::default()
        }
    }

    fn parse_atlas_parameter(&mut self, id: &str, val: &str) {
        match id {
            "size" => {
                let uints: Vec<u32> = split_str(val);
                self.current_atlas.size((uints[0], uints[1]));
            }
            _ => ()
        };
    }

    fn parse_part_parameter(&mut self, id: &str, val: &str) {
        match id {
            "size" | "xy" | "orig" => {
                let uints: Vec<u32> = split_str(val);
                let vec2u = (uints[0], uints[1]);
                match id {
                    "size" => self.current_part.size(vec2u),
                    "xy" => self.current_part.xy(vec2u),
                    "orig" => self.current_part.origin(vec2u),
                    _ => unreachable!()
                };
            },
            "offset" => {
                let ints: Vec<i32> = split_str(val);
                self.current_part.offset((ints[0], ints[1]));
            },
            "rotate" => {
                let angle: u32 = match val {
                    "true" => 90,
                    "false" => 0,
                    _ => val.parse().expect("rotate parameter value should've been a u32 if not a bool")
                };

                self.current_part.rotate(angle);
            },
            "index" => {
                self.current_part.index(
                    val.parse::<i32>().expect("index param should've been an i32")
                );
            },
            _ => ()
        };
    }

    fn finish_part(&mut self) -> Result<(), PartBuilderError> {
        let finished_part = self.current_part.build()?;

        if let Some(part_list) = self.current_atlas.parts.as_mut() {
            part_list.push(finished_part);
        } else {
            self.current_atlas.parts(vec![finished_part]);
        }

        // renew the builder
        self.current_part = PartBuilder::default();

        Ok(())
    }

    fn parse_line(&mut self, line: &str) -> Result<(), PartBuilderError> {
        if line.len() == 0 { return Ok(()); }

        if !self.image_name_found {
            self.current_atlas.image_name(line);
            self.image_name_found = true;
            return Ok(());
        }

        if !self.atlas_header_finished {
            let atlas_param_captures = self.regexes.atlas_parameter.captures(line);

            if let Some(captures) = atlas_param_captures {
                let (_, [id, val]) = captures.extract();
                self.parse_atlas_parameter(id, val);
            } else {
                self.atlas_header_finished = true;
                self.current_part.name(line);
            }

            return Ok(());
        }

        let part_param_captures = self.regexes.part_parameter.captures(line);
        if let Some(captures) = part_param_captures {
            let (_, [id, val]) = captures.extract();
            self.parse_part_parameter(id, val);
        } else {
            if self.current_part.name.is_some() {
                self.finish_part()?;
            }
            self.current_part.name(line);
        }

        Ok(())
    }

    pub fn parse_str(&mut self, atlas_str: &str) -> Result<Atlas, Box<dyn std::error::Error>> {
        for line in atlas_str.lines() {
            self.parse_line(line)?;
        }
        // because parsing is done line by line, 
        // the last part needs to be finished separately
        self.finish_part()?;

        Ok(self.current_atlas.build()?)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn regexes_work() {
        use crate::atlas_parser::ParserRegexes;
        let regexes = ParserRegexes::new();

        let haystacks_to_pass = vec![
            "size: 1024,2048",
            "another: 223",
            "third:something"
        ];

        let haystacks_to_fail = vec![
            " size: 1024,2048"
        ];

        for hay in haystacks_to_pass.into_iter() {
            assert!(regexes.atlas_parameter.is_match(hay));
            let caps = regexes.atlas_parameter.captures(hay);
            assert!(caps.is_some());
        }

        for hay in haystacks_to_fail.into_iter() {
            assert!(!regexes.atlas_parameter.is_match(hay));
            let caps = regexes.atlas_parameter.captures(hay);
            assert!(caps.is_none());
        }
    }
}