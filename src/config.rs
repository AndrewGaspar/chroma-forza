use std::{collections::HashMap, fmt, ops::RangeInclusive};

use chroma::MAX_COLUMN;
use rgb::RGB8;
use serde::{
    de::{self, Visitor},
    Deserialize, Deserializer,
};

#[derive(Debug, Deserialize)]
pub struct Config {
    #[serde(default)]
    pub colors: HashMap<String, Color>,
    #[serde(default)]
    pub effect: Vec<Effect>,
}

#[derive(Debug)]
pub struct Color(pub RGB8);

impl<'de> Deserialize<'de> for Color {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ColorVisitor;

        impl<'de> Visitor<'de> for ColorVisitor {
            type Value = Color;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a 6-digit hexadecimal value in the format 'rrggbb'")
            }

            fn visit_str<E: de::Error>(self, v: &str) -> Result<Self::Value, E> {
                if v.len() != 6 {
                    return Err(E::custom("Unexpected color value"));
                }

                let rgb = u32::from_str_radix(v, 16).map_err(de::Error::custom)?;
                let r = ((rgb >> 16) & 0xff) as u8;
                let g = ((rgb >> 8) & 0xff) as u8;
                let b = (rgb & 0xff) as u8;
                Ok(Color(RGB8 { r, g, b }))
            }
        }

        deserializer.deserialize_str(ColorVisitor)
    }
}

#[derive(Debug, Deserialize)]
pub struct Effect {
    layer: Option<f32>,
    #[serde(flatten)]
    data: EffectData,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum EffectData {
    Custom {
        input: Input,
        output: Output,
    },
    Predefined {
        #[serde(flatten)]
        data: PredefinedEffectData,
    },
}

#[derive(Debug, Deserialize)]
pub struct Input {
    property: String,
}

#[derive(Debug, Deserialize)]
pub struct Output {
    r#type: String,
    #[serde(default = "white")]
    color: String,
    canvas: Canvas,
}

#[derive(Debug, Deserialize)]
pub struct Canvas {
    column: GridRange,
    row: GridRange,
}

#[derive(Debug)]
pub enum GridRange {
    Scalar(u8),
    Range(RangeInclusive<u8>),
    All,
}

impl<'de> Deserialize<'de> for GridRange {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct GridRangeVisitor;

        impl<'de> Visitor<'de> for GridRangeVisitor {
            type Value = GridRange;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str(
                    "a non-negative integer or an integer range of the form 'x:y' or 'y:x'",
                )
            }

            fn visit_i64<E: de::Error>(self, v: i64) -> Result<Self::Value, E> {
                if !(0..(MAX_COLUMN as i64)).contains(&v) {
                    Err(E::custom(format!("value {} is out of range", v)))
                } else {
                    Ok(GridRange::Scalar(v as u8))
                }
            }

            fn visit_u64<E: de::Error>(self, v: u64) -> Result<Self::Value, E> {
                if !(0..(MAX_COLUMN as u64)).contains(&v) {
                    Err(E::custom(format!("value {} is out of range", v)))
                } else {
                    Ok(GridRange::Scalar(v as u8))
                }
            }

            fn visit_str<E: de::Error>(self, v: &str) -> Result<Self::Value, E> {
                if v == ":" {
                    return Ok(GridRange::All);
                }

                // parse from
                let result = (|| {
                    let mut chars = v.char_indices().skip_while(|(_, c)| c.is_numeric());

                    let (i, from) = match chars.next() {
                        Some((i, ':')) => (i + 1, u8::from_str_radix(&v[..i], 10).ok()?),
                        Some(_) => return None,
                        None => return Some((u8::from_str_radix(&v, 10).ok()?, None)),
                    };

                    let to = u8::from_str_radix(&v[i..], 10).ok()?;

                    Some((from, Some(to)))
                })();

                if let Some((from, to)) = result {
                    Ok(match to {
                        Some(to) => GridRange::Range(RangeInclusive::new(from, to)),
                        None => GridRange::Scalar(from),
                    })
                } else {
                    Err(E::custom(
                        "Expected an integer or a range in the form of \"x:y\"",
                    ))
                }
            }
        }

        let value = deserializer.deserialize_any(GridRangeVisitor)?;
        Ok(value)
    }
}

#[derive(Debug, Deserialize)]
#[serde(tag = "predefined")]
pub enum PredefinedEffectData {
    #[serde(rename = "position")]
    Position { numkeys: String },
}

fn white() -> String {
    "white".to_string()
}
