use prelude::*;

use forza::Horizon4Datagram;

pub mod prelude {
    pub use crate::state::{ChromaState, Tick};
}

mod meter;
mod position;

pub use meter::*;
pub use position::*;

pub struct Effect {
    altitude: i32,
    implementation: Box<dyn EffectImpl>,
}

impl Effect {
    pub fn new(altitude: i32, implementation: Box<dyn EffectImpl>) -> Self {
        Self {
            altitude,
            implementation,
        }
    }

    pub fn altitude(&self) -> i32 {
        self.altitude
    }

    pub fn start<'a>(&'a self) -> Box<dyn 'a + EffectInstance> {
        self.implementation.start()
    }
}

pub trait EffectImpl {
    fn start<'a>(&'a self) -> Box<dyn 'a + EffectInstance>;
}

pub trait EffectInstance {
    /// Called when a new data state is available
    fn update(&mut self, datagram: &Horizon4Datagram);

    /// Called when the chroma is being updated.
    fn tick(&mut self, tick: &Tick, state: &mut ChromaState);
}
