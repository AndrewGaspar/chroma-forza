#![allow(dead_code)]

use crate::config::{NumKeysSelector, PositionEffectData};

use chroma::Key;

const NUMPAD: [Key; 9] = [
    Key::Numpad1,
    Key::Numpad2,
    Key::Numpad3,
    Key::Numpad4,
    Key::Numpad5,
    Key::Numpad6,
    Key::Numpad7,
    Key::Numpad8,
    Key::Numpad9,
];

const NUMROW: [Key; 10] = [
    Key::Row1,
    Key::Row2,
    Key::Row3,
    Key::Row4,
    Key::Row5,
    Key::Row6,
    Key::Row7,
    Key::Row8,
    Key::Row9,
    Key::Row0,
];

pub struct PositionEffect {
    numkeys: &'static [Key],
}

impl PositionEffect {
    pub fn new(config: &PositionEffectData) -> Self {
        let numkeys = match config.numkeys {
            NumKeysSelector::Pad => &NUMPAD[..],
            NumKeysSelector::Row => &NUMROW[..],
        };

        Self { numkeys }
    }
}
