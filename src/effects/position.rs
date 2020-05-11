use std::collections::HashMap;

use chroma::Key;
use rgb::RGB8;

use crate::{
    config::{self, Color, NumKeys},
    effects::{EffectImpl, EffectInstance},
    property::ScoreProperty,
};

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
    property: ScoreProperty,
    color: RGB8,
    numkeys: config::NumKeys,
}

impl PositionEffect {
    pub fn new(
        property: ScoreProperty,
        output: &config::Output,
        config: &config::ScoreEffect,
        colors: &HashMap<String, Color>,
    ) -> Self {
        let keyboard = config
            .keyboard
            .as_ref()
            .expect("TODO: We shouldn't require a keyboard output for MeterEffect.");

        let color = colors[&output.color].0;

        Self {
            property,
            color,
            numkeys: keyboard.numkeys,
        }
    }
}

impl EffectImpl for PositionEffect {
    fn start<'a>(&'a self) -> Box<dyn 'a + EffectInstance> {
        Box::new(PositionEffectInstance {
            effect: &self,
            current: None,
        })
    }
}

pub struct PositionEffectInstance<'a> {
    effect: &'a PositionEffect,
    current: Option<i32>,
}

impl<'a> EffectInstance for PositionEffectInstance<'a> {
    fn update(&mut self, datagram: &forza::Horizon4Datagram) {
        self.current = if datagram.sled.is_race_on != 0 {
            Some(self.effect.property.query(datagram))
        } else {
            None
        };
    }

    fn tick(&mut self, _tick: &super::prelude::Tick, state: &mut super::prelude::ChromaState) {
        let current = if let Some(v) = self.current {
            v
        } else {
            return;
        };

        let keys = match self.effect.numkeys {
            NumKeys::Row => &NUMROW[..],
            NumKeys::Pad => &NUMPAD[..],
        };

        if current < 0 {
            return;
        }

        let current = current as usize;

        if current >= keys.len() {
            return;
        }

        state.set_key(keys[current], self.effect.color);
    }
}
