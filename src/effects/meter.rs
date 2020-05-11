use std::{collections::HashMap, ops::RangeInclusive};

use rgb::RGB8;

use crate::{
    config::{self, Color, GridRange},
    effects::{EffectImpl, EffectInstance},
    property::RateProperty,
};

#[derive(Copy, Clone)]
enum MeterOrientation {
    ColumnBase,
    RowBase,
}

pub struct MeterEffect {
    property: RateProperty,
    color: RGB8,
    orientation: MeterOrientation,
    base: RangeInclusive<u8>,
    meter: RangeInclusive<u8>,
    fill: bool,
}

impl MeterEffect {
    pub fn new(
        property: RateProperty,
        output: &config::Output,
        config: &config::MeterEffect,
        colors: &HashMap<String, Color>,
    ) -> Self {
        let keyboard = config
            .keyboard
            .as_ref()
            .expect("TODO: We shouldn't require a keyboard output for MeterEffect.");

        let color = colors[&output.color].0;

        let column_range = match &keyboard.column {
            GridRange::All => GridRange::Range(0..=chroma::MAX_COLUMN - 1),
            x => x.clone(),
        };

        let row_range = match &keyboard.row {
            GridRange::All => GridRange::Range(0..=chroma::MAX_ROW - 1),
            x => x.clone(),
        };

        let (orientation, base, meter) = match (column_range, row_range) {
            (GridRange::Range(base), GridRange::Direction(meter)) => {
                (MeterOrientation::ColumnBase, base, meter)
            }
            (GridRange::Direction(meter), GridRange::Range(base)) => {
                (MeterOrientation::RowBase, base, meter)
            }
            _ => {
                panic!(
                    "One of the ranges for keyboard output must be direction (e.g. x->y) and \
                     the other must be non-direction (e.g. x or x:y)"
                );
            }
        };

        Self {
            property,
            color,
            orientation,
            base,
            meter,
            fill: config.fill,
        }
    }
}

impl EffectImpl for MeterEffect {
    fn start<'a>(&'a self) -> Box<dyn 'a + EffectInstance> {
        Box::new(MeterEffectInstance {
            effect: &self,
            current: None,
        })
    }
}

pub struct MeterEffectInstance<'a> {
    effect: &'a MeterEffect,
    current: Option<f32>,
}

impl<'a> EffectInstance for MeterEffectInstance<'a> {
    fn update(&mut self, datagram: &forza::Horizon4Datagram) {
        self.current = if datagram.sled.is_race_on != 0 {
            Some(self.effect.property.query(datagram).max(0.0).min(1.0))
        } else {
            None
        };
    }

    fn tick(&mut self, _tick: &super::prelude::Tick, state: &mut super::prelude::ChromaState) {
        let pct_rpm = if let Some(current) = self.current {
            current
        } else {
            return;
        };

        let orientation = self.effect.orientation;
        let color = self.effect.color;
        let meter = self.effect.meter.clone();
        let base = self.effect.base.clone();

        let meter_height = if meter.start() > meter.end() {
            meter.start() + 1 - meter.end()
        } else {
            meter.end() + 1 - meter.start()
        } as f32;

        let shade = meter_height * pct_rpm;
        // round down
        let num_filled = shade as u8;
        for base in base {
            let extent = |position: u8| {
                if meter.start() > meter.end() {
                    meter.start() - position
                } else {
                    meter.start() + position
                }
            };

            if self.effect.fill {
                for position in 0..num_filled {
                    let (row, column) = match orientation {
                        MeterOrientation::RowBase => (base, extent(position)),
                        MeterOrientation::ColumnBase => (extent(position), base),
                    };

                    state.set_position(row, column, color);
                }
            }

            // In case we manage to hit 100%...
            if num_filled < chroma::MAX_COLUMN {
                let (row, column) = match orientation {
                    MeterOrientation::RowBase => (base, extent(num_filled)),
                    MeterOrientation::ColumnBase => (extent(num_filled), base),
                };

                let color = if self.effect.fill {
                    let color: rgb::RGB<f32> = color.into();
                    let color: rgb::RGB<f32> = color * (shade % 1.0);
                    RGB8 {
                        r: color.r as u8,
                        g: color.g as u8,
                        b: color.b as u8,
                    }
                } else {
                    color
                };

                state.set_position(row, column, color);
            }
        }
    }
}
