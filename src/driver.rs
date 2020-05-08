use std::{future::Future, time::Instant};

use tokio::stream::{Stream, StreamExt};

use crate::{
    config::{Config, EffectData},
    effects::{Effect, MeterEffect},
    property,
    state::{ChromaState, Tick},
};

pub struct Driver {
    effects: Vec<Effect>,
}

impl Driver {
    pub fn from_config(config: &Config) -> Self {
        let mut driver = Self { effects: vec![] };

        for effect in &config.effect {
            match &effect.data {
                EffectData::Custom { input, output } => {
                    let property = property::query_rate_property(input);

                    driver.add_effect(Effect::new(
                        effect.altitude,
                        Box::new(MeterEffect::new(property, output, &config.colors)),
                    ));
                }
                EffectData::Predefined { .. } => todo!(),
            }
        }

        driver
    }

    fn add_effect(&mut self, effect: Effect) {
        let insert_at = match self
            .effects
            .binary_search_by_key(&effect.altitude(), |e| e.altitude())
        {
            Ok(i) => {
                let effect = self
                    .effects
                    .iter()
                    .skip(i)
                    .enumerate()
                    .skip_while(|(_, e)| effect.altitude() == e.altitude())
                    .next();
                match effect {
                    Some((i, _)) => i,
                    None => self.effects.len(),
                }
            }
            Err(i) => i,
        };

        self.effects.insert(insert_at, effect);
    }

    pub async fn run(
        &self,
        mut stream: impl Stream<Item = std::io::Result<forza::Horizon4Datagram>> + Unpin,
        mut cancel: impl Future<Output = ()> + Unpin,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut instances: Vec<_> = self.effects.iter().map(|e| e.start()).collect();

        let mut last = None;
        while let Some(Ok(datagram)) = tokio::select! {
            datagram = stream.next() => datagram,
            _ = &mut cancel => None
        } {
            let mut state = ChromaState::new();

            let now = Instant::now();
            let tick = Tick {
                now,
                // elapsed: last.map(|last| now.duration_since(last)),
                elapsed: last.map(|last| now - last),
            };
            for i in &mut instances {
                i.update(&datagram);
                i.tick(&tick, &mut state);
            }
            last = Some(now);

            state.apply()?;
        }

        Ok(())
    }
}
