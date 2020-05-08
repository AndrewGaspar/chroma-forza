use std::{collections::HashMap, fmt, ops::RangeInclusive};

use chroma::MAX_COLUMN;
use rgb::RGB8;
use serde::{
    de::{self, Visitor},
    Deserialize, Deserializer,
};

#[derive(Clone, Debug, Deserialize)]
pub struct Config {
    #[serde(default)]
    pub colors: HashMap<String, Color>,
    #[serde(default)]
    pub effect: Vec<Effect>,
}

#[derive(Clone, Debug)]
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

#[derive(Clone, Debug, Deserialize)]
pub struct Effect {
    #[serde(default)]
    pub altitude: i32,
    #[serde(flatten)]
    pub data: EffectData,
}

#[derive(Clone, Debug, Deserialize)]
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

#[derive(Clone, Debug, Deserialize)]
pub struct Input {
    pub property: String,
    pub max_value: Option<f32>,
    #[serde(default)]
    pub auto_raise: bool,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Output {
    #[serde(default = "white")]
    pub color: String,
    pub keyboard: Option<Keyboard>,
    #[serde(flatten)]
    #[serde(rename = "type")]
    pub effect_type: EffectType,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(tag = "type")]
pub enum EffectType {
    #[serde(rename = "meter")]
    Meter {
        #[serde(default)]
        fill: bool,
    },
}

#[derive(Clone, Debug, Deserialize)]
pub struct Keyboard {
    pub column: GridRange,
    pub row: GridRange,
}

#[derive(Clone, Debug)]
pub enum GridRange {
    Range(RangeInclusive<u8>),
    Direction(RangeInclusive<u8>),
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
                    let v = v as u8;
                    Ok(GridRange::Range(v..=v))
                }
            }

            fn visit_u64<E: de::Error>(self, v: u64) -> Result<Self::Value, E> {
                if !(0..(MAX_COLUMN as u64)).contains(&v) {
                    Err(E::custom(format!("value {} is out of range", v)))
                } else {
                    let v = v as u8;
                    Ok(GridRange::Range(v..=v))
                }
            }

            fn visit_str<E: de::Error>(self, v: &str) -> Result<Self::Value, E> {
                if v == ":" {
                    return Ok(GridRange::All);
                }

                // parse from
                let result = (|| {
                    let mut chars = v.char_indices().skip_while(|(_, c)| c.is_numeric());

                    enum Separator {
                        Colon,
                        LeftArrow,
                        RightArrow,
                    }

                    let (i, j, separator) = match chars.next() {
                        Some((i, ':')) => (i, i + 1, Separator::Colon),
                        Some((i, '-')) => {
                            if let Some((_, '>')) = chars.next() {
                                (i, i + 2, Separator::RightArrow)
                            } else {
                                return None;
                            }
                        }
                        Some((i, '<')) => {
                            if let Some((_, '-')) = chars.next() {
                                (i, i + 2, Separator::LeftArrow)
                            } else {
                                return None;
                            }
                        }
                        Some(_) => return None,
                        None => {
                            let v = u8::from_str_radix(&v, 10).ok()?;
                            return Some(GridRange::Range(v..=v));
                        }
                    };

                    let left = u8::from_str_radix(&v[..i], 10).ok()?;
                    let right = u8::from_str_radix(&v[j..], 10).ok()?;

                    Some(match separator {
                        Separator::Colon => GridRange::Range(if left < right {
                            left..=right
                        } else {
                            right..=left
                        }),
                        Separator::LeftArrow => GridRange::Direction(right..=left),
                        Separator::RightArrow => GridRange::Direction(left..=right),
                    })
                })();

                if let Some(result) = result {
                    Ok(result)
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

#[derive(Clone, Debug, Deserialize)]
#[serde(tag = "predefined")]
pub enum PredefinedEffectData {
    #[serde(rename = "position")]
    Position(PositionEffectData),
}

#[derive(Clone, Debug, Deserialize)]
pub struct PositionEffectData {
    pub numkeys: NumKeysSelector,
}

#[derive(Clone, Debug, Deserialize)]
pub enum NumKeysSelector {
    #[serde(rename = "pad")]
    Pad,
    #[serde(rename = "row")]
    Row,
}

fn white() -> String {
    "white".to_string()
}
