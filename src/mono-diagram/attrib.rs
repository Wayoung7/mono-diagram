use std::str::FromStr;

use anyhow::Result;
use pest_derive::Parser;

#[derive(Debug, Default)]
pub struct Attrib {
    pub style: Style,
}

macro_rules! parse_attrib {
    ($input:expr, $($field:ident), *) => {{
        let mut result = Attrib::default();

        let input_trimmed = $input.trim().trim_end_matches("}").trim_start_matches("{");
        let pairs: Vec<&str> = input_trimmed.split(',').map(|s| s.trim()).collect();
        for pair in pairs {
            if let Some((key, value)) = pair.split_once(':') {
                let key = key.trim().to_ascii_lowercase();
                let value = value.trim();

                match key.as_str() {
                    $(
                        stringify!($field) => {
                            if let Ok(val) = value.parse() {
                                result.$field = val;
                            }
                        }
                    ),*
                    _ => {}
                }
            }
        }
        result
    }};
}

impl Attrib {
    pub fn parse_from_str(input: &str) -> Result<Self> {
        let attribs = parse_attrib!(input, style);
        Ok(attribs)
    }
}

#[derive(Default, Debug, PartialEq, Eq)]
pub enum Style {
    #[default]
    Ascii,
    Unicode,
}

impl FromStr for Style {
    type Err = ParseStyleError;

    fn from_str(s: &str) -> std::prelude::v1::Result<Self, Self::Err> {
        if s.to_ascii_lowercase() == "ascii" {
            Ok(Self::Ascii)
        } else if s.to_ascii_lowercase() == "unicode" {
            Ok(Self::Unicode)
        } else {
            Err(ParseStyleError)
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct ParseStyleError;

#[derive(Parser)]
#[grammar = "mono-diagram/grammar/attrib.pest"]
struct AttribParser;
