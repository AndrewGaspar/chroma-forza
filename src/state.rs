use std::time::{Duration, Instant};

use chroma::KeyboardCustomKeyEffectBuilder;

pub struct Tick {
    pub now: Instant,
    pub elapsed: Option<Duration>,
}

pub struct ChromaState {
    keyboard: KeyboardCustomKeyEffectBuilder,
}

impl ChromaState {
    pub fn new() -> Self {
        Self {
            keyboard: KeyboardCustomKeyEffectBuilder::new(),
        }
    }

    pub fn set_position(&mut self, row: u8, column: u8, color: rgb::RGB8) {
        self.keyboard.set_position(row, column, color);
    }

    #[allow(dead_code)]
    pub fn set_key(&mut self, key: chroma::Key, color: rgb::RGB8) {
        self.keyboard.set_key(key, color);
    }

    pub fn apply(self) -> chroma::Result<()> {
        self.keyboard.build()?.set()?;
        Ok(())
    }
}
